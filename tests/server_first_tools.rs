//! Server-first discovery through real gateway/API paths and disposable peers.
//! No installed client, provider, native prompt, Keychain or personal state is used.
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
};
use fetch_local::{
    api::{self, AppState, runtime::ManagementAdapter},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    installation::Installer,
    manager::Manager,
    mcp::McpGateway,
    remote_oauth::GrantStore,
    runner::{ExecutionPolicy, LEGACY, MODERN, Runner, ServerRoute, TransportConfig},
    storage::ServerRepository,
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    os::unix::fs::{MetadataExt, PermissionsExt},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::Barrier;
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53672";
const RAW_RESULT: &str = r#"{ "resultType":"complete", "content":[{"type":"text","text":"upstream result","future":true},{"type":"image","mimeType":"image/png","data":"fixture-base64"}], "structuredContent":{"number":184467440737095516160000001}, "futureResult":{"keep":true} }"#;
struct Authority;
impl Authorizer for Authority {
    fn authorize(&self, headers: &axum::http::HeaderMap) -> Result<Caller, AuthorizationError> {
        let scopes = match headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
        {
            Some("Bearer full") => vec!["fetch:catalog", "fetch:tools:call"],
            Some("Bearer catalog") => vec!["fetch:catalog"],
            Some("Bearer calls") => vec!["fetch:tools:call"],
            _ => return Err(AuthorizationError::Unauthenticated),
        };
        Ok(Caller::from_verified_scopes(
            scopes.into_iter().map(str::to_owned),
        ))
    }
}
struct Peer {
    modern: bool,
    catalogs: AtomicUsize,
    calls: AtomicUsize,
    gate: Mutex<Option<Arc<Barrier>>>,
    requests: Mutex<Vec<String>>,
}
impl Peer {
    fn new(modern: bool) -> Arc<Self> {
        Arc::new(Self {
            modern,
            catalogs: AtomicUsize::new(0),
            calls: AtomicUsize::new(0),
            gate: Mutex::new(None),
            requests: Mutex::new(vec![]),
        })
    }
}
fn reply(id: &Value, result: &str) -> Response {
    (
        [(header::CONTENT_TYPE, "application/json")],
        format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{result}}}"),
    )
        .into_response()
}
async fn peer(State(peer): State<Arc<Peer>>, body: String) -> Response {
    let request: Value = serde_json::from_str(&body).unwrap();
    let id = &request["id"];
    match request["method"].as_str().unwrap_or("") {
        "initialize" if peer.modern=>(StatusCode::BAD_REQUEST,[(header::CONTENT_TYPE,"application/json")],format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"error\":{{\"code\":-32022,\"message\":\"Unsupported protocol version.\"}}}}")).into_response(),
        "initialize"=>reply(id,&json!({"protocolVersion":LEGACY,"capabilities":{"tools":{}},"serverInfo":{"name":"owned peer","version":"1"}}).to_string()),
        "server/discover"=>reply(id,&json!({"resultType":"complete","supportedVersions":[MODERN],"capabilities":{"tools":{}},"ttlMs":0,"cacheScope":"private"}).to_string()),
        "tools/list"=>{
            peer.catalogs.fetch_add(1,Ordering::SeqCst);
            let gate=peer.gate.lock().unwrap().clone();if let Some(gate)=gate{gate.wait().await;}
            reply(id,r#"{"resultType":"complete","tools":[{"name":"echo","description":"Exact upstream description","inputSchema":{"type":"object","properties":{"mode":{"type":"string"}}},"outputSchema":{"type":"object"},"future":{"number":184467440737095516160000007}}]}"#)
        }
        "tools/call"=>{
            peer.calls.fetch_add(1,Ordering::SeqCst);peer.requests.lock().unwrap().push(body.clone());
            if request["params"]["arguments"]["mode"]=="ask" {
                if request["params"].get("requestState").is_none(){
                    return reply(id,r#"{"resultType":"input_required","requestState":"upstream-state","inputRequests":{"answer":{"method":"elicitation/create","params":{"mode":"form","message":"Owned fixture question","requestedSchema":{"type":"object","properties":{"answer":{"type":"string"}}}}}}}"#);
                }
                assert_eq!(request["params"]["requestState"],"upstream-state");
                assert_eq!(request["params"]["inputResponses"]["answer"]["content"]["answer"],"approved answer");
            }
            reply(id,RAW_RESULT)
        }
        _=>StatusCode::ACCEPTED.into_response(),
    }
}
struct Fixture {
    _directory: tempfile::TempDir,
    manager: Arc<Manager>,
    app: Router,
    peers: BTreeMap<String, Arc<Peer>>,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}
impl Fixture {
    async fn new(modern: bool) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("state");
        let repository = ServerRepository::new(root.clone());
        let runner = Runner::new();
        let discovery = DiscoveryService::new(directory.path().join("empty-home"), None);
        let manager = Manager::with_grant_store(
            repository.clone(),
            Arc::new(GrantStore::memory(root.clone())),
            runner.clone(),
            Installer::new(root, discovery.clone()),
        );
        let mut peers = BTreeMap::new();
        let mut tasks = vec![];
        for id in ["alpha", "beta", "unselected"] {
            let state = Peer::new(modern);
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
            let app = Router::new()
                .route("/mcp", post(peer))
                .with_state(state.clone());
            tasks.push(tokio::spawn(async move {
                let _ = axum::serve(listener, app).await;
            }));
            repository.save(id,&json!({"server_id":id,"name":format!("Owned {id}"),"runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":endpoint,"transport":"streamable_http","headers":[]}}})).unwrap();
            runner
                .configure(ServerRoute {
                    server_id: id.into(),
                    transport: TransportConfig::Http {
                        endpoint,
                        headers: BTreeMap::new(),
                    },
                    execution: ExecutionPolicy::default(),
                })
                .await
                .unwrap();
            runner.connect(id).await.unwrap();
            peers.insert(id.into(), state);
        }
        repository.save("off",&json!({"server_id":"off","name":"Owned disabled server","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"enabled":false,"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":"https://never-contacted.fixture.invalid/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
        let backend = Arc::new(ManagementAdapter {
            manager: manager.clone(),
            discovery: discovery.clone(),
            workspace: None,
        });
        let authority = Arc::new(Authority);
        let gateway = McpGateway::new(runner, backend, authority.clone(), ORIGIN.into());
        let mut state = AppState::new(ORIGIN.into(), repository, discovery, authority).unwrap();
        state.manager = Some(manager.clone());
        state.gateway = Some(gateway);
        Self {
            _directory: directory,
            manager,
            app: api::router(Arc::new(state)),
            peers,
            tasks,
        }
    }
    fn request(&self, method: &str, params: Value, token: &str) -> Request<Body> {
        let mut params = params;
        params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{"elicitation":{"form":{}}}});
        let mut builder = Request::builder()
            .method("POST")
            .uri("/mcp")
            .header(header::HOST, "localhost:53672")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .header("mcp-protocol-version", MODERN)
            .header("mcp-method", method);
        if method == "tools/call" {
            builder = builder.header("mcp-name", params["name"].as_str().unwrap());
        }
        builder
            .body(Body::from(
                json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}).to_string(),
            ))
            .unwrap()
    }
    async fn call(&self, method: &str, params: Value, token: &str) -> (StatusCode, String) {
        let response = self
            .app
            .clone()
            .oneshot(self.request(method, params, token))
            .await
            .unwrap();
        let status = response.status();
        let bytes = tokio::time::timeout(Duration::from_secs(4), response.into_body().collect())
            .await
            .expect("bounded owned fixture response")
            .unwrap()
            .to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }
    async fn tool(&self, name: &str, args: Value, token: &str) -> (StatusCode, Value) {
        let (status, body) = self
            .call("tools/call", json!({"name":name,"arguments":args}), token)
            .await;
        (status, last_message(&body))
    }
    async fn close(self) {
        self.manager.shutdown().await;
        for task in &self.tasks {
            task.abort();
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}
fn last_message(body: &str) -> Value {
    // HTTP permission refusals can have no MCP body; callers assert the status.
    if body.trim().is_empty() {
        return Value::Null;
    }
    if body.trim_start().starts_with('{') {
        return serde_json::from_str(body).unwrap();
    }
    body.lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .map(|line| serde_json::from_str(line).unwrap())
        .last()
        .expect("MCP result event")
}
fn tool_data(message: &Value) -> Value {
    serde_json::from_str(message["result"]["content"][0]["text"].as_str().unwrap()).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn startup_lists_every_permitted_server_without_fetching_upstream_tools_then_selection_runs_in_parallel()
 {
    let fixture = Fixture::new(false).await;
    let baseline: BTreeMap<_, _> = fixture
        .peers
        .iter()
        .map(|(id, peer)| (id.clone(), peer.catalogs.load(Ordering::SeqCst)))
        .collect();
    let (status, discover) = fixture.call("server/discover", json!({}), "full").await;
    assert_eq!(status, StatusCode::OK);
    let (status, native) = fixture.call("tools/list", json!({}), "full").await;
    assert_eq!(status, StatusCode::OK);
    let native: Value = serde_json::from_str(&native).unwrap();
    let tools = native["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 3);
    let description = tools
        .iter()
        .find(|tool| tool["name"] == "fetch__list_servers")
        .unwrap()["description"]
        .as_str()
        .unwrap();
    for id in ["alpha", "beta", "unselected", "off"] {
        assert!(description.contains(id));
        assert!(discover.contains(id));
    }
    for (id, peer) in &fixture.peers {
        assert_eq!(peer.catalogs.load(Ordering::SeqCst), baseline[id]);
    }
    let (status, servers) = fixture
        .tool("fetch__list_servers", json!({}), "catalog")
        .await;
    assert_eq!(status, StatusCode::OK);
    let servers = tool_data(&servers);
    assert_eq!(servers["servers"].as_array().unwrap().len(), 4);
    assert!(
        servers["servers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["server_id"] == "off" && row["status"] == "stopped")
    );
    assert!(
        !servers.to_string().contains("never-contacted"),
        "safe discovery omits connection configuration"
    );
    let gate = Arc::new(Barrier::new(2));
    for id in ["alpha", "beta"] {
        *fixture.peers[id].gate.lock().unwrap() = Some(gate.clone());
    }
    let (_, selected) = tokio::time::timeout(
        Duration::from_secs(3),
        fixture.tool(
            "fetch__list_tools",
            json!({"server_id":["beta","alpha","off"]}),
            "catalog",
        ),
    )
    .await
    .expect("both selected requests must start before either completes");
    let selected = tool_data(&selected);
    assert_eq!(selected["origins"][0]["source"]["server_id"], "beta");
    assert_eq!(selected["origins"][1]["source"]["server_id"], "alpha");
    assert_eq!(selected["origins"][0]["tools"][0]["name"], "echo");
    assert_eq!(
        selected["origins"][0]["tools"][0]["future"]["number"].to_string(),
        "184467440737095516160000007"
    );
    assert_eq!(selected["origins"][2]["status"], "unavailable");
    assert_eq!(selected["complete"], false);
    assert_eq!(
        fixture.peers["unselected"].catalogs.load(Ordering::SeqCst),
        baseline["unselected"]
    );
    for id in ["alpha", "beta"] {
        *fixture.peers[id].gate.lock().unwrap() = None;
    }
    let (status, missing_selection) = fixture
        .tool("fetch__list_tools", json!({}), "catalog")
        .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        missing_selection["error"].is_object() || missing_selection["result"]["isError"] == true
    );
    let request = Request::builder()
        .uri("/api/v1/tools")
        .header(header::HOST, "localhost:53672")
        .header(header::AUTHORIZATION, "Bearer catalog")
        .body(Body::empty())
        .unwrap();
    let response = fixture.app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value =
        serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert_eq!(
        body["origins"].as_array().unwrap().len(),
        4,
        "direct HTTP default still lists all origins"
    );
    fixture.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn generic_calls_keep_raw_results_and_catalog_only_connections_cannot_invoke() {
    let fixture = Fixture::new(false).await;
    let before = fixture.peers["alpha"].calls.load(Ordering::SeqCst);
    let (status, _) = fixture
        .tool(
            "fetch__call_tool",
            json!({"server_id":"alpha","tool_name":"echo","arguments":{}}),
            "catalog",
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(fixture.peers["alpha"].calls.load(Ordering::SeqCst), before);
    let inner = r#"{ "number":184467440737095516160000009, "future":{"keep":true} }"#;
    let params:Value=serde_json::from_str(&format!(r#"{{"name":"fetch__call_tool","arguments":{{"server_id":"alpha","tool_name":"echo","arguments":{inner}}}}}"#)).unwrap();
    let (status, body) = fixture.call("tools/call", params, "full").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains(RAW_RESULT),
        "native upstream result must not become a JSON-to-text wrapper: {body}"
    );
    let request: Value = serde_json::from_str(
        fixture.peers["alpha"]
            .requests
            .lock()
            .unwrap()
            .last()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(request["params"]["name"], "echo");
    assert!(request["params"]["arguments"].get("server_id").is_none());
    assert_eq!(
        request["params"]["arguments"]["number"].to_string(),
        "184467440737095516160000009"
    );
    let (status, hidden) = fixture.call("server/discover", json!({}), "calls").await;
    assert_eq!(status, StatusCode::OK);
    for id in ["alpha", "beta", "unselected", "off"] {
        assert!(
            !hidden.contains(id),
            "no catalog grant must reveal no server inventory"
        );
    }
    let (_, catalog_after) = fixture.call("tools/list", json!({}), "full").await;
    assert_eq!(
        serde_json::from_str::<Value>(&catalog_after).unwrap()["result"]["tools"]
            .as_array()
            .unwrap()
            .len(),
        3,
        "calls do not expand the fixed catalog"
    );
    fixture.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn modern_input_required_retains_server_tool_arguments_and_continuation_fields() {
    let fixture = Fixture::new(true).await;
    let routing = json!({"server_id":"alpha","tool_name":"echo","arguments":{"mode":"ask","future":{"keep":true}}});
    let (_, first) = fixture
        .tool("fetch__call_tool", routing.clone(), "full")
        .await;
    assert_eq!(first["result"]["resultType"], "input_required");
    let state = first["result"]["requestState"].clone();
    assert!(state.as_str().unwrap().starts_with("fetch-continuation-"));
    let before = fixture.peers["alpha"].calls.load(Ordering::SeqCst);
    let mut changed = routing.clone();
    changed["arguments"]["future"]["keep"] = json!(false);
    let(_,rejected)=fixture.call("tools/call",json!({"name":"fetch__call_tool","arguments":changed,"requestState":state,"inputResponses":{"answer":{"action":"accept","content":{"answer":"approved answer"}}}}),"full").await;
    let rejected = last_message(&rejected);
    assert!(rejected["error"].is_object() || rejected["result"]["isError"] == true);
    assert_eq!(fixture.peers["alpha"].calls.load(Ordering::SeqCst), before);
    let(status,resumed)=fixture.call("tools/call",json!({"name":"fetch__call_tool","arguments":routing,"requestState":state,"inputResponses":{"answer":{"action":"accept","content":{"answer":"approved answer"}}}}),"full").await;
    assert_eq!(status, StatusCode::OK);
    assert!(resumed.contains(RAW_RESULT));
    assert_eq!(
        fixture.peers["alpha"].calls.load(Ordering::SeqCst),
        before + 1
    );
    fixture.close().await;
}

#[tokio::test]
async fn alias_batches_are_durable_skip_unchanged_writes_and_do_not_commit_failed_saves() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let runner = Runner::persistent(directory.path().to_owned()).unwrap();
    let targets = vec![
        ("alpha".into(), "search".into()),
        ("beta".into(), "search".into()),
        ("alpha".into(), "search".into()),
    ];
    let names = runner.aliases(&targets).await.unwrap();
    assert_eq!(names[0], names[2]);
    assert_ne!(names[0], names[1]);
    let path = directory.path().join("aliases.json");
    let first = std::fs::metadata(&path).unwrap();
    assert_eq!(runner.aliases(&targets).await.unwrap(), names);
    let second = std::fs::metadata(&path).unwrap();
    assert_eq!(
        (first.ino(), first.mtime(), first.mtime_nsec()),
        (second.ino(), second.mtime(), second.mtime_nsec()),
        "an unchanged catalog must not rewrite the mapping file"
    );
    let reloaded = Runner::persistent(directory.path().to_owned()).unwrap();
    assert_eq!(reloaded.aliases(&targets).await.unwrap(), names);
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    assert!(
        runner
            .aliases(&[("alpha".into(), "new-tool".into())])
            .await
            .is_err()
    );
    std::fs::remove_dir(&path).unwrap();
    let added = runner
        .aliases(&[("alpha".into(), "new-tool".into())])
        .await
        .unwrap();
    assert!(
        path.is_file(),
        "failed save must not leave an in-memory mapping that skips persistence on retry"
    );
    let reloaded = Runner::persistent(directory.path().to_owned()).unwrap();
    assert_eq!(
        reloaded
            .aliases(&[("alpha".into(), "new-tool".into())])
            .await
            .unwrap(),
        added
    );
}
