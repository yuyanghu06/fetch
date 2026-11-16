//! Two disposable projects through the real native APIs and MCP gateway.
//! Peers are in-process loopback fixtures: no personal files, credentials,
//! accounts, clients or executable MCP packages are used.
use axum::{Router, body::Body, extract::State, http::{HeaderMap, Request, StatusCode, header}, response::{IntoResponse, Response}, routing::post};
use fetch_local::{
    api::{self, AppState, runtime::ManagementAdapter},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    installation::Installer,
    manager::Manager,
    mcp::McpGateway,
    native_auth::{NativeAuthority, NativeSettingsSession},
    projects::{Project, ProjectRegistry, visible_server_ids},
    remote_oauth::GrantStore,
    runner::{ExecutionPolicy, InvocationContext, LEGACY, MODERN, Runner, ServerRoute, TransportConfig, raw},
    storage::ServerRepository,
    workspace::WorkspaceService,
};
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicUsize, Ordering}}, time::Duration};
use tempfile::TempDir;
use tokio::sync::Notify;
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53672";

struct ScopeAuthority {
    native: Arc<NativeAuthority>,
    repository: ServerRepository,
    a: Project,
    b: Project,
}
impl ScopeAuthority {
    fn caller(&self, token: &str) -> Result<Caller, AuthorizationError> {
        let (project, history_only) = match token {
            "global" => (None, false),
            "a" => (Some(self.a.clone()), false),
            "b" => (Some(self.b.clone()), false),
            "history-a" => (Some(self.a.clone()), true),
            "history-b" => (Some(self.b.clone()), true),
            _ => return Err(AuthorizationError::Unauthenticated),
        };
        let scopes = if history_only { vec!["fetch:sessions:read"] } else {
            vec!["fetch:manage", "fetch:catalog", "fetch:tools:call", "fetch:content:read", "fetch:sessions:read"]
        };
        let resource = resource(project.as_ref().map(|project|project.id.as_str()));
        let visible = visible_server_ids(&self.repository, project.as_ref().map(|project|project.id.as_str()))
            .map_err(|_|AuthorizationError::Unavailable)?;
        Ok(Caller::from_verified_scopes(scopes.into_iter().map(str::to_owned))
            .bind_project(resource, project).restrict_visibility(visible))
    }
}
impl Authorizer for ScopeAuthority {
    fn authorize(&self, headers:&HeaderMap) -> Result<Caller,AuthorizationError> {
        if let Some(caller) = self.native.authorize_settings_headers(headers) { return Ok(caller); }
        let token = headers.get(header::AUTHORIZATION).and_then(|value|value.to_str().ok())
            .and_then(|value|value.strip_prefix("Bearer ")).ok_or(AuthorizationError::Unauthenticated)?;
        self.caller(token)
    }
}
fn resource(project:Option<&str>) -> String {
    project.map_or_else(||format!("{ORIGIN}/mcp"),|id|format!("{ORIGIN}/mcp/projects/{id}"))
}
fn mcp_path(project:Option<&str>) -> String {
    project.map_or_else(||"/mcp".into(),|id|format!("/mcp/projects/{id}"))
}

struct Peer { id:String, calls:AtomicUsize, release:Notify }
fn peer_reply(id:&Value,result:Value) -> Response {
    ([(header::CONTENT_TYPE,"application/json")],json!({"jsonrpc":"2.0","id":id,"result":result}).to_string()).into_response()
}
async fn peer(State(peer):State<Arc<Peer>>,body:String) -> Response {
    let request:Value=serde_json::from_str(&body).unwrap(); let id=&request["id"];
    let result=match request["method"].as_str().unwrap_or("") {
        "initialize"=>json!({"protocolVersion":LEGACY,"capabilities":{"tools":{},"resources":{},"prompts":{}},"serverInfo":{"name":"Owned project peer","version":"1"}}),
        "tools/list"=>json!({"tools":[{"name":"echo","description":"Owned scoped echo","inputSchema":{"type":"object","properties":{"marker":{"type":"string"},"hold":{"type":"boolean"}}}}]}),
        "tools/call"=>{
            peer.calls.fetch_add(1,Ordering::SeqCst);
            if request["params"]["arguments"]["hold"]==true { peer.release.notified().await; }
            json!({"content":[{"type":"text","text":format!("fixture-server:{}",peer.id)}],"structuredContent":{"server":peer.id,"arguments":request["params"]["arguments"]}})
        },
        "resources/list"=>json!({"resources":[{"uri":format!("fixture://{}/data",peer.id),"name":format!("{} resource",peer.id)}]}),
        "resources/templates/list"=>json!({"resourceTemplates":[]}),
        "resources/read"=>json!({"contents":[{"uri":request["params"]["uri"],"text":format!("resource-from-{}",peer.id)}]}),
        "prompts/list"=>json!({"prompts":[{"name":"inspect","description":format!("{} prompt",peer.id)}]}),
        "prompts/get"=>json!({"messages":[{"role":"user","content":{"type":"text","text":format!("prompt-from-{}",peer.id)}}]}),
        _=>return StatusCode::ACCEPTED.into_response(),
    };
    peer_reply(id,result)
}

struct Fixture {
    _directory:TempDir,
    root:PathBuf,
    home:PathBuf,
    a:Project,
    b:Project,
    registry:Arc<ProjectRegistry>,
    repository:ServerRepository,
    installer:Arc<Installer>,
    manager:Arc<Manager>,
    authority:Arc<ScopeAuthority>,
    native_session:NativeSettingsSession,
    app:Router,
    peers:BTreeMap<String,Arc<Peer>>,
    tasks:Vec<tokio::task::JoinHandle<()>>,
}
impl Fixture {
    async fn new() -> Self {
        let directory=tempfile::tempdir().unwrap(); let root=directory.path().join("fetch-state");
        let home=directory.path().join("fixture-home"); let a_path=directory.path().join("project-a"); let b_path=directory.path().join("project-b");
        for folder in [&home,&a_path,&b_path] { std::fs::create_dir_all(folder).unwrap(); }
        let registry=Arc::new(ProjectRegistry::new(root.clone()));
        let a=registry.register(a_path.to_str().unwrap(),Some("Project A")).unwrap().0;
        let b=registry.register(b_path.to_str().unwrap(),Some("Project B")).unwrap().0;
        let repository=ServerRepository::new(root.clone());let runner=Runner::new();
        let discovery=DiscoveryService::new(home.clone(),None);let installer=Installer::new(root.clone(),discovery.clone());
        let manager=Manager::with_grant_store(repository.clone(),Arc::new(GrantStore::memory(root.clone())),runner.clone(),installer.clone());
        let mut peers=BTreeMap::new();let mut tasks=vec![];
        for (id,project) in [("shared",None),("project-a-server",Some(&a.id)),("project-b-server",Some(&b.id))] {
            let peer=Arc::new(Peer{id:id.into(),calls:AtomicUsize::new(0),release:Notify::new()});
            let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();let endpoint=format!("http://{}/mcp",listener.local_addr().unwrap());
            let app=Router::new().route("/mcp",post(self::peer)).with_state(peer.clone());
            tasks.push(tokio::spawn(async move{let _=axum::serve(listener,app).await;}));
            repository.save(id,&json!({"server_id":id,"name":"Database","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
                "config":{"scope":{"project_id":project},"enabled":true,"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":endpoint,"transport":"streamable_http","headers":[]}}})).unwrap();
            runner.configure(ServerRoute{server_id:id.into(),transport:TransportConfig::Http{endpoint,headers:BTreeMap::new()},execution:ExecutionPolicy::default()}).await.unwrap();
            runner.connect(id).await.unwrap();peers.insert(id.into(),peer);
        }
        let native=NativeAuthority::new();let native_session=native.open_settings();
        let authority=Arc::new(ScopeAuthority{native:native.clone(),repository:repository.clone(),a:a.clone(),b:b.clone()});
        let workspace=Arc::new(WorkspaceService::new(home.clone(),home.join(".codex"),home.join(".claude")));
        let backend=Arc::new(ManagementAdapter{manager:manager.clone(),discovery:discovery.clone(),workspace:Some(workspace.clone())});
        let gateway=McpGateway::new(runner,backend,authority.clone(),ORIGIN.into());
        let mut state=AppState::new(ORIGIN.into(),repository.clone(),discovery,authority.clone()).unwrap();
        state.projects=Some(registry.clone());state.manager=Some(manager.clone());state.native=Some(native);
        state.workspace=Some(workspace);
        state.gateway=Some(gateway);
        Self{_directory:directory,root,home,a,b,registry,repository,installer,manager,authority,native_session,app:api::router(Arc::new(state)),peers,tasks}
    }
    async fn http(&self,method:&str,path:&str,body:Value,token:Option<&str>,native:bool) -> (StatusCode,Value) {
        let mut request=Request::builder().method(method).uri(path).header(header::HOST,"localhost:53672").header(header::CONTENT_TYPE,"application/json");
        if native { request=request.header("x-fetch-settings-capability",self.native_session.header_value()); }
        if let Some(token)=token { request=request.header(header::AUTHORIZATION,format!("Bearer {token}")); }
        let response=self.app.clone().oneshot(request.body(if method=="GET"{Body::empty()}else{Body::from(body.to_string())}).unwrap()).await.unwrap();
        assert_eq!(response.headers()[header::CACHE_CONTROL],"no-store");let status=response.status();let value=decode(response).await;
        assert!(value["next_steps"].is_array(),"application response has guidance: {value}");(status,value)
    }
    async fn mcp(&self,project:Option<&str>,method:&str,mut params:Value,token:&str) -> (StatusCode,Value) {
        params["_meta"]=json!({"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{}});
        let mut request=Request::builder().method("POST").uri(mcp_path(project)).header(header::HOST,"localhost:53672")
            .header(header::AUTHORIZATION,format!("Bearer {token}")).header(header::CONTENT_TYPE,"application/json")
            .header(header::ACCEPT,"application/json, text/event-stream").header("mcp-protocol-version",MODERN).header("mcp-method",method);
        if matches!(method,"tools/call"|"prompts/get"|"resources/read") {
            let key=if method=="resources/read"{"uri"}else{"name"};
            request=request.header("mcp-name",params[key].as_str().unwrap());
        }
        let response=self.app.clone().oneshot(request.body(Body::from(json!({"jsonrpc":"2.0","id":7,"method":method,"params":params}).to_string())).unwrap()).await.unwrap();
        let status=response.status();(status,decode(response).await)
    }
    async fn tool(&self,project:Option<&str>,name:&str,args:Value,token:&str) -> (StatusCode,Value) {
        self.mcp(project,"tools/call",json!({"name":name,"arguments":args}),token).await
    }
    fn session(&self,name:&str,project:&Project,body:&str) -> PathBuf {
        let path=self.home.join(".codex/sessions/2026/11/12").join(format!("{name}.jsonl"));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path,format!("{}\n{}\n",json!({"type":"session_meta","payload":{"id":name,"cwd":project.folder_path,"title":name}}),json!({"type":"response_item","payload":{"role":"assistant","content":body}}))).unwrap();path
    }
    async fn close(&self) { for peer in self.peers.values(){peer.release.notify_one();}self.manager.shutdown().await;for task in &self.tasks {task.abort();} }
}
impl Drop for Fixture {fn drop(&mut self){for task in &self.tasks{task.abort();}}}
async fn decode(response:Response) -> Value {
    let bytes=tokio::time::timeout(Duration::from_secs(5),response.into_body().collect()).await.expect("bounded fixture response").unwrap().to_bytes();
    if bytes.is_empty(){return Value::Null;}
    if let Ok(value)=serde_json::from_slice(&bytes){return value;}
    let text=std::str::from_utf8(&bytes).unwrap();
    text.lines().filter_map(|line|line.strip_prefix("data:")).filter_map(|line|serde_json::from_str::<Value>(line.trim()).ok())
        .find(|value|value["id"]==7).unwrap_or_else(||panic!("missing fixture result: {text}"))
}
fn successful(value:&Value) -> bool {value.get("result").is_some()&&value.get("error").is_none()&&value["result"]["isError"]!=true}
fn contains_id(value:&Value,id:&str) -> bool {value.to_string().contains(id)}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn native_projects_are_idempotent_and_idle_reassignment_preserves_off_state() {
    let fixture=Fixture::new().await;
    let result=std::panic::AssertUnwindSafe(async {
        let (status,existing)=fixture.http("POST","/api/v1/projects",json!({"folder_path":fixture.a.folder_path,"name":"Must not rename existing"}),None,true).await;
        assert_eq!(status,StatusCode::OK);assert_eq!(existing["project"]["id"],fixture.a.id);assert_eq!(existing["project"]["name"],"Project A");
        let third=fixture._directory.path().join("project-new");std::fs::create_dir_all(&third).unwrap();
        let (status,created)=fixture.http("POST","/api/v1/projects",json!({"folder_path":third}),None,true).await;
        assert_eq!(status,StatusCode::CREATED);let id=created["project"]["id"].as_str().unwrap();
        assert!(ProjectRegistry::new(fixture.root.clone()).get(id).unwrap().is_some());
        assert_eq!(fixture.registry.list().unwrap().len(),3);
        let (status,_)=fixture.http("POST","/api/v1/projects",json!({"folder_path":third}),Some("a"),false).await;assert_eq!(status,StatusCode::FORBIDDEN);
        fixture.repository.save("idle-off",&json!({"server_id":"idle-off","name":"Off fixture","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart":null,"restart_required":false,
            "config":{"scope":{"project_id":fixture.a.id},"enabled":false,"kind":"remote","executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":"https://never-used.fixture.invalid/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
        let before=fixture.repository.get_saved("idle-off").unwrap().unwrap();
        let stale_a=fixture.authority.caller("a").unwrap();
        let (status,_)=fixture.http("PATCH","/api/v1/servers/idle-off/config",json!({"scope":{"project_id":fixture.b.id}}),None,true).await;assert_eq!(status,StatusCode::OK);
        let after=fixture.repository.get_saved("idle-off").unwrap().unwrap();
        assert_eq!(after["config"]["scope"]["project_id"],fixture.b.id);assert_eq!(after["config"]["enabled"],false);
        assert_eq!(after["config"]["connection"],before["config"]["connection"]);assert_eq!(after["config"]["working_directory"],before["config"]["working_directory"]);
        // A request can have acquired its caller snapshot before the reassignment.
        // It must lose reads and mutations once the saved project changes.
        let visible=fixture.manager.servers_for(&stale_a,vec![],None).await.unwrap();
        assert!(!contains_id(&visible,"idle-off"));assert_eq!(visible["not_found_ids"],json!([]));
        assert_eq!(fixture.manager.config("idle-off",json!({"execution":{"default_timeout_ms":6000}}),&stale_a).await.unwrap_err().http_status,404);
        assert_eq!(fixture.manager.restart_for(&stale_a,"idle-off",json!({})).await.unwrap_err().http_status,404);
        assert_eq!(fixture.manager.authenticate_for(&stale_a,"idle-off",false).await.unwrap_err().http_status,404);
        assert_eq!(fixture.manager.delete_server("idle-off",&stale_a).await.unwrap_err().http_status,404);
        assert_eq!(fixture.repository.get_saved("idle-off").unwrap().unwrap(),after);
        let (status,_)=fixture.http("PATCH","/api/v1/servers/idle-off/config",json!({"scope":{"project_id":fixture.a.id}}),Some("b"),false).await;assert_eq!(status,StatusCode::FORBIDDEN);
        let runner=fixture.manager.runner.clone();
        let held=tokio::spawn(async move{runner.call("project-a-server","echo",raw::raw(&json!({"hold":true})),InvocationContext::terminal("scope-held-fixture".into())).await});
        tokio::time::timeout(Duration::from_secs(3),async{while fixture.peers["project-a-server"].calls.load(Ordering::SeqCst)==0{tokio::task::yield_now().await;}}).await.unwrap();
        let runner=fixture.manager.runner.clone();
        let queued=tokio::spawn(async move{runner.call("project-a-server","echo",raw::raw(&json!({"marker":"queued"})),InvocationContext::terminal("scope-queued-fixture".into())).await});
        tokio::time::timeout(Duration::from_secs(3),async{
            while fixture.manager.runner.snapshot("project-a-server").await["tool_execution"]["queued_calls"]!=1{tokio::task::yield_now().await;}
        }).await.unwrap();
        let (status,_)=fixture.http("PATCH","/api/v1/servers/project-a-server/config",json!({"scope":{"project_id":fixture.b.id}}),None,true).await;
        // Always release the fixture before asserting, including a failed busy guard.
        fixture.peers["project-a-server"].release.notify_one();
        tokio::time::timeout(Duration::from_secs(3),held).await.unwrap().unwrap().unwrap();
        tokio::time::timeout(Duration::from_secs(3),queued).await.unwrap().unwrap().unwrap();
        assert_eq!(status,StatusCode::CONFLICT);
        assert_eq!(fixture.repository.get_saved("project-a-server").unwrap().unwrap()["config"]["scope"]["project_id"],fixture.a.id);
    }).catch_unwind().await;
    fixture.close().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

async fn prepared(installer:&Arc<Installer>,input:Value,project:Option<Project>) -> Value {
    let accepted=installer.submit_scoped(input,project).await.unwrap();let id=accepted["installation_id"].as_str().unwrap();
    tokio::time::timeout(Duration::from_secs(3),async{
        loop{let value=installer.progress(id).await.unwrap();if value["status"]!="installing"{return value;}tokio::task::yield_now().await;}
    }).await.expect("fixture local preparation needs no package/runtime execution")
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn scoped_installations_reuse_within_one_project_and_keep_working_directories_and_progress_private() {
    let fixture=Fixture::new().await;
    let result=std::panic::AssertUnwindSafe(async {
        // Preparation records this existing executable; there is deliberately no
        // Manager connection hook and the fixture program must never run.
        let program=fixture._directory.path().join("owned-fixture-program");
        std::fs::write(&program,"#!/bin/sh\nexit 91\n").unwrap();
        let explicit=fixture.a.folder_path.join("explicit-working-folder");std::fs::create_dir_all(&explicit).unwrap();
        let source=fixture.a.folder_path.join(".mcp.json");
        std::fs::write(&source,json!({"mcpServers":{
            "Database":{"command":program},
            "Explicit":{"command":program,"cwd":explicit},
        }}).to_string()).unwrap();
        let input=json!({"servers":[{"config_path":source,"server_name":"Database"}]});
        let first=prepared(&fixture.installer,input.clone(),Some(fixture.a.clone())).await;assert_eq!(first["status"],"succeeded");
        let a_id=first["servers"][0]["server_id"].as_str().unwrap();
        let second=prepared(&fixture.installer,input.clone(),Some(fixture.b.clone())).await;assert_eq!(second["status"],"succeeded");
        let b_id=second["servers"][0]["server_id"].as_str().unwrap();assert_ne!(a_id,b_id);
        let again=prepared(&fixture.installer,input.clone(),Some(fixture.a.clone())).await;assert_eq!(again["servers"][0]["server_id"],a_id);
        let shared=prepared(&fixture.installer,input,None).await;let shared_id=shared["servers"][0]["server_id"].as_str().unwrap();assert_ne!(shared_id,a_id);assert_ne!(shared_id,b_id);
        for (id,project) in [(a_id,&fixture.a),(b_id,&fixture.b)] {
            let saved=fixture.repository.get_saved(id).unwrap().unwrap();assert_eq!(saved["config"]["scope"]["project_id"],project.id);assert_eq!(Path::new(saved["config"]["working_directory"].as_str().unwrap()),project.folder_path);
        }
        assert!(fixture.repository.get_saved(shared_id).unwrap().unwrap()["config"]["working_directory"].is_null());
        let kept=prepared(&fixture.installer,json!({"servers":[{"config_path":source,"server_name":"Explicit"}]}),Some(fixture.b.clone())).await;
        let saved=fixture.repository.get_saved(kept["servers"][0]["server_id"].as_str().unwrap()).unwrap().unwrap();assert_eq!(saved["config"]["working_directory"],explicit.to_str().unwrap());
        let group=first["installation_id"].as_str().unwrap();
        assert_eq!(fixture.http("GET",&format!("/api/v1/setup/installations/{group}"),Value::Null,Some("a"),false).await.0,StatusCode::OK);
        assert_eq!(fixture.http("GET",&format!("/api/v1/setup/installations/{group}"),Value::Null,Some("b"),false).await.0,StatusCode::NOT_FOUND);
        let rejected=fixture.manager.install_for(&fixture.authority.caller("b").unwrap(),json!({"servers":[{"config_path":source,"server_name":"Database"}]})).await.unwrap_err();assert_eq!(rejected.http_status,403);
    }).catch_unwind().await;
    fixture.close().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn project_gateway_exposes_shared_and_own_servers_and_rejects_other_targets() {
    let fixture=Fixture::new().await;
    let result=std::panic::AssertUnwindSafe(async {
        for (project,token,own,other) in [(&fixture.a,"a","project-a-server","project-b-server"),(&fixture.b,"b","project-b-server","project-a-server")] {
            let (status,inventory)=fixture.mcp(Some(&project.id),"server/discover",json!({}),token).await;
            assert_eq!(status,StatusCode::OK);assert!(contains_id(&inventory,"shared")&&contains_id(&inventory,own));assert!(!contains_id(&inventory,other));
            let (status,listed)=fixture.http("GET","/api/v1/servers",Value::Null,Some(token),false).await;
            assert_eq!(status,StatusCode::OK);assert_eq!(listed["servers"].as_array().unwrap().len(),2);assert!(!contains_id(&listed,other));
            assert_eq!(listed["not_found_ids"],json!([]),"unfiltered inventory cannot enumerate hidden IDs");
            let (_,managed)=fixture.tool(Some(&project.id),"fetch__get_servers",json!({}),token).await;
            assert!(successful(&managed));
            let managed:Value=serde_json::from_str(managed["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
            assert_eq!(managed["not_found_ids"],json!([]));assert!(!contains_id(&managed,other));
            let (_,hidden)=fixture.http("GET",&format!("/api/v1/servers?id={other}"),Value::Null,Some(token),false).await;
            assert!(hidden["servers"].as_array().is_none_or(|servers|servers.is_empty()),"a guessed ID cannot reveal the other record: {hidden}");
            let before=fixture.peers[other].calls.load(Ordering::SeqCst);
            let (_,denied)=fixture.tool(Some(&project.id),"fetch__call_tool",json!({"server_id":other,"tool_name":"echo","arguments":{"marker":"forbidden"}}),token).await;assert!(!successful(&denied));assert_eq!(fixture.peers[other].calls.load(Ordering::SeqCst),before);
            let (status,called)=fixture.tool(Some(&project.id),"fetch__call_tool",json!({"server_id":own,"tool_name":"echo","arguments":{"marker":"allowed"}}),token).await;
            assert_eq!(status,StatusCode::OK);assert!(successful(&called));assert!(contains_id(&called,&format!("fixture-server:{own}")));
            for method in ["resources/list","prompts/list"] {
                let (status,listed)=fixture.mcp(Some(&project.id),method,json!({}),token).await;assert_eq!(status,StatusCode::OK);assert!(contains_id(&listed,"shared")&&contains_id(&listed,own));assert!(!contains_id(&listed,other));
            }
            for (server,allowed) in [(own,true),(other,false)] {
                let uri=fetch_local::runner::catalog::qualified_uri(server,&format!("fixture://{server}/data"));
                let (_,read)=fixture.mcp(Some(&project.id),"resources/read",json!({"uri":uri}),token).await;
                assert_eq!(successful(&read),allowed,"resource reads use the same project restriction: {read}");
                let name=fixture.manager.runner.alias(server,"inspect").await.unwrap();
                let (_,prompt)=fixture.mcp(Some(&project.id),"prompts/get",json!({"name":name}),token).await;
                assert_eq!(successful(&prompt),allowed,"prompt reads use the same project restriction: {prompt}");
            }
        }
        let (_,global)=fixture.mcp(None,"server/discover",json!({}),"global").await;assert!(contains_id(&global,"shared"));assert!(!contains_id(&global,"project-a-server")&&!contains_id(&global,"project-b-server"));
        let (status,_)=fixture.mcp(Some(&fixture.b.id),"tools/list",json!({}),"a").await;assert!(matches!(status,StatusCode::FORBIDDEN|StatusCode::UNAUTHORIZED));
        let (status,_)=fixture.mcp(Some(&fixture.a.id),"tools/list",json!({}),"global").await;assert!(matches!(status,StatusCode::FORBIDDEN|StatusCode::UNAUTHORIZED));

        // A legacy session ID carries its original project binding too, even
        // when another otherwise-valid grant is used at its own endpoint.
        let legacy_request=|project:Option<&str>,token:&str,session:Option<&str>,message:Value| {
            let mut request=Request::builder().method("POST").uri(mcp_path(project))
                .header(header::HOST,"localhost:53672")
                .header(header::AUTHORIZATION,format!("Bearer {token}"))
                .header(header::CONTENT_TYPE,"application/json")
                .header(header::ACCEPT,"application/json, text/event-stream")
                .header("mcp-protocol-version",LEGACY);
            if let Some(session)=session {request=request.header("mcp-session-id",session);}
            request.body(Body::from(message.to_string())).unwrap()
        };
        let initialized=fixture.app.clone().oneshot(legacy_request(Some(&fixture.a.id),"a",None,json!({
            "jsonrpc":"2.0","id":7,"method":"initialize","params":{
                "protocolVersion":LEGACY,"capabilities":{},"clientInfo":{"name":"Owned project fixture","version":"1"}
            }
        }))).await.unwrap();
        assert_eq!(initialized.status(),StatusCode::OK);
        let session_id=initialized.headers()["mcp-session-id"].to_str().unwrap().to_owned();
        assert_eq!(decode(initialized).await["result"]["protocolVersion"],LEGACY);
        let acknowledged=fixture.app.clone().oneshot(legacy_request(Some(&fixture.a.id),"a",Some(&session_id),json!({
            "jsonrpc":"2.0","method":"notifications/initialized","params":{}
        }))).await.unwrap();
        assert_eq!(acknowledged.status(),StatusCode::ACCEPTED);
        let list=json!({"jsonrpc":"2.0","id":7,"method":"tools/list","params":{}});
        let permitted=fixture.app.clone().oneshot(legacy_request(Some(&fixture.a.id),"a",Some(&session_id),list.clone())).await.unwrap();
        assert_eq!(permitted.status(),StatusCode::OK);
        assert!(successful(&decode(permitted).await));
        for (project,token,expected) in [
            (Some(fixture.b.id.as_str()),"b",StatusCode::NOT_FOUND),
            (None,"global",StatusCode::NOT_FOUND),
            (Some(fixture.a.id.as_str()),"b",StatusCode::FORBIDDEN),
        ] {
            let rejected=fixture.app.clone().oneshot(legacy_request(project,token,Some(&session_id),list.clone())).await.unwrap();
            assert_eq!(rejected.status(),expected,"legacy session IDs cannot move between project resources");
        }
    }).catch_unwind().await;
    fixture.close().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn project_history_permission_reads_only_its_chunks_without_upstream_permissions() {
    let fixture=Fixture::new().await;
    let result=std::panic::AssertUnwindSafe(async {
        let a_path=fixture.session("A history",&fixture.a,&"A-private-fixture字🌱".repeat(4_000));
        fixture.session("B history",&fixture.b,"B-private-fixture");let original=std::fs::read_to_string(&a_path).unwrap();
        let (status,names)=fixture.http("GET","/api/v1/sessions",Value::Null,Some("history-a"),false).await;assert_eq!(status,StatusCode::OK);
        let sessions=names["sessions"].as_array().unwrap();assert_eq!(sessions.len(),1);assert_eq!(sessions[0]["name"],"A history");assert!(sessions[0].get("content").is_none()&&sessions[0].get("file_path").is_none());
        let a_id=sessions[0]["id"].as_str().unwrap();let mut offset=0;let mut combined=String::new();
        loop {
            let (status,part)=fixture.http("GET",&format!("/api/v1/sessions/{a_id}?offset={offset}&limit=4097"),Value::Null,Some("history-a"),false).await;
            assert_eq!(status,StatusCode::OK);let content=part["content"].as_str().unwrap();assert!(content.len()<=4097);combined.push_str(content);
            assert!(part.get("file_path").is_none());assert!(part["session"].get("file_path").is_none());
            if part["eof"]==true{break;}let next=part["next_offset"].as_u64().unwrap();assert!(next>offset);offset=next;
        }
        assert_eq!(combined,original);assert_eq!(std::fs::read_to_string(&a_path).unwrap(),original);
        let (status,_)=fixture.http("GET",&format!("/api/v1/sessions/{a_id}"),Value::Null,Some("history-b"),false).await;assert_eq!(status,StatusCode::NOT_FOUND);
        let (status,_)=fixture.http("GET","/api/v1/sessions",Value::Null,Some("global"),false).await;assert_eq!(status,StatusCode::FORBIDDEN);
        let (_,tools)=fixture.mcp(Some(&fixture.a.id),"tools/list",json!({}),"history-a").await;
        assert!(contains_id(&tools,"fetch__list_sessions")&&contains_id(&tools,"fetch__read_session"));assert!(!contains_id(&tools,"fetch__call_tool"));
        let (status,read)=fixture.tool(Some(&fixture.a.id),"fetch__read_session",json!({"session_id":a_id,"limit":64}),"history-a").await;assert_eq!(status,StatusCode::OK);assert!(successful(&read));
        let body:Value=serde_json::from_str(read["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
        assert!(body["content"].as_str().unwrap().len()<=64);assert!(body.get("file_path").is_none());assert!(body["session"].get("file_path").is_none());
        let (_,global_tools)=fixture.mcp(None,"tools/list",json!({}),"global").await;assert!(!contains_id(&global_tools,"fetch__list_sessions")&&!contains_id(&global_tools,"fetch__read_session"));
        let (status,_)=fixture.http("GET","/api/v1/workspace",Value::Null,Some("history-a"),false).await;assert_eq!(status,StatusCode::FORBIDDEN);
    }).catch_unwind().await;
    fixture.close().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}
