//! Server-owned login tests use synthetic children inside verified OS confinement.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;

use confinement::Sandbox;
use axum::{body::Body, http::{HeaderMap, Request, StatusCode}, Router};
use fetch_local::{
    api::{AppState, router},
    authority::{Authorizer, AuthorizationError, Caller},
    discovery::DiscoveryService, installation::Installer, manager::Manager,
    native_auth::NativeAuthority,
    remote_oauth::GrantStore,
    runner::{raw, ExecutionPolicy, InvocationContext, Runner, ServerRoute, TransportConfig},
    storage::ServerRepository,
};
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::{path::Path, sync::Arc, time::Duration};
use tower::ServiceExt;

struct RemoteSetup {
    reject: bool,
    initialize_headers: std::sync::Mutex<Vec<HeaderMap>>,
}
async fn remote_setup(
    axum::extract::State(state): axum::extract::State<Arc<RemoteSetup>>,
    headers: HeaderMap,
    axum::Json(request): axum::Json<Value>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    if request["method"]=="notifications/initialized" {return StatusCode::ACCEPTED.into_response();}
    assert_eq!(request["method"],"initialize");
    state.initialize_headers.lock().unwrap().push(headers);
    if state.reject {return StatusCode::UNAUTHORIZED.into_response();}
    axum::Json(json!({"jsonrpc":"2.0","id":request["id"],"result":{
        "protocolVersion":"2025-11-25","capabilities":{},"serverInfo":{"name":"disposable-remote","version":"1"}
    }})).into_response()
}

const SECRET: &str = "never-display-this-secret";
const STATE: &str = "private-transient-fixture-state";
const HOST: &str = "accounts.fixture.example";

struct FixtureAuthority;
impl Authorizer for FixtureAuthority {
    fn authorize(&self, _: &HeaderMap) -> Result<Caller, AuthorizationError> {
        Ok(Caller::from_verified_scopes(["fetch:manage".into()]))
    }
}
struct DiagnosticAuthority(Arc<NativeAuthority>);
impl Authorizer for DiagnosticAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        Ok(self.0.authorize_settings_headers(headers)
            .unwrap_or_else(||Caller::from_verified_scopes(["fetch:manage".into()])))
    }
}
fn app(manager: Arc<Manager>, repository: ServerRepository, sandbox: &Sandbox) -> Router {
    let discovery=DiscoveryService::new(sandbox.root.join("unused-source-home"), None);
    let mut state=AppState::new("https://localhost:53672".into(), repository, discovery, Arc::new(FixtureAuthority)).unwrap();
    state.manager=Some(manager);
    router(Arc::new(state))
}
fn request(path: &str, method: &str, body: Option<Value>) -> Request<Body> {
    Request::builder().uri(path).method(method).header("host", "localhost:53672")
        .header("content-type", "application/json")
        .body(body.map(|value| Body::from(value.to_string())).unwrap_or_else(Body::empty)).unwrap()
}
async fn response_body(response: axum::response::Response) -> Value {
    assert_eq!(response.headers()["cache-control"], "no-store");
    serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()
}

fn fixture() -> Sandbox {
    let sandbox = Sandbox::new();
    std::fs::write(&sandbox.peer, include_str!("native_state/login_peer.py")).unwrap();
    sandbox
}

fn args(sandbox: &Sandbox, additional: &[&str]) -> Vec<String> {
    let mut args = vec![sandbox.peer.to_string_lossy().into_owned(), sandbox.home.to_string_lossy().into_owned()];
    args.extend(additional.iter().map(|value| (*value).to_owned()));
    args
}

fn route(id: &str, sandbox: &Sandbox) -> ServerRoute {
    ServerRoute { server_id: id.into(), transport: TransportConfig::Stdio {
        executable: sandbox.launcher.clone(), args: args(sandbox, &[]),
        working_directory: Some(sandbox.home.clone()), environment: sandbox.environment(),
    }, execution: ExecutionPolicy::default() }
}

fn record(id: &str, sandbox: &Sandbox, additional: &[&str]) -> Value {
    json!({"server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"local_script","package":null,"executable":sandbox.launcher,
            "args":args(sandbox,additional),"working_directory":sandbox.home,"environment":[]}})
}

fn manager(sandbox: &Sandbox, saved: &Value) -> (Arc<Manager>, ServerRepository) {
    let root = sandbox.root.join("fetch-state");
    let repository = ServerRepository::new(root.clone());
    repository.save(saved["server_id"].as_str().unwrap(), saved).unwrap();
    let discovery = DiscoveryService::new(root.join("unused-source-home"), None);
    (Manager::with_grant_store(repository.clone(), Arc::new(GrantStore::memory(root.clone())),
        Runner::new(), Installer::new(root, discovery)), repository)
}

async fn status(manager: &Manager, id: &str) -> Value {
    tokio::time::timeout(Duration::from_millis(500), manager.servers(vec![id.into()], None))
        .await.expect("status reads must stay responsive during login").unwrap()["servers"][0].clone()
}

async fn until(manager: &Manager, id: &str, predicate: impl Fn(&Value) -> bool) -> Value {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let current = status(manager, id).await;
            if predicate(&current) { break current; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("expected login state within five seconds")
}

fn alive(path: &Path) -> bool {
    let Ok(pid) = std::fs::read_to_string(path).and_then(|value| value.parse::<i32>()
        .map_err(|_| std::io::Error::other("invalid fixture PID"))) else { return false; };
    unsafe { libc::kill(pid, 0) == 0 }
}

fn assert_private(value: &Value) {
    let text = value.to_string();
    for private in [SECRET, STATE, "response_type=", "unrelated.fixture.example"] {
        assert!(!text.contains(private), "private setup output must not reach API: {private}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn configured_startup_authenticates_without_helper_metadata_and_remembers_only_terminal_status() {
    let sandbox = fixture(); let sibling = Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let (manager, repository) = manager(&sandbox, &record("login", &sandbox, &["--wait-for-link"]));
    let app=app(manager.clone(), repository.clone(), &sandbox);
    let result = std::panic::AssertUnwindSafe(async {
        let response = tokio::time::timeout(Duration::from_millis(500), app.clone().oneshot(request(
            "/api/v1/servers/login/restart", "POST", Some(json!({"interactive":true}))))
            ).await.expect("interactive restart starts asynchronously").unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        assert!(response.headers()["location"].to_str().unwrap().contains("/api/v1/servers?"));
        let response=response_body(response).await;
        assert_eq!(response["status"], "authenticating"); assert_private(&response);
        // Before a URL exists a repeated request must only monitor the same process.
        let compatibility=app.clone().oneshot(request("/api/v1/servers/login/authenticate", "POST", Some(json!({"interactive":true})))).await.unwrap();
        assert_eq!(compatibility.status(), StatusCode::ACCEPTED);
        assert_eq!(compatibility.headers()["deprecation"], "@1793664000");
        assert!(compatibility.headers()["link"].to_str().unwrap().contains("/api/v1/servers/login/restart"));
        assert_eq!(response_body(compatibility).await["status"], "authenticating");
        std::fs::write(sandbox.home.join("release-link"), "show sign-in link").unwrap();
        let pending = until(&manager, "login", |value| value["authentication"]["sign_in"]["host"] == HOST).await;
        assert_eq!(pending["authentication"]["status"], "authenticating");
        assert_eq!(pending["authentication"]["sign_in"]["opened"], false);
        assert_private(&pending);
        let read=app.clone().oneshot(request("/api/v1/servers?id=login", "GET", None)).await.unwrap();
        assert_eq!(read.status(), StatusCode::OK);
        let read=response_body(read).await;
        assert_eq!(read["servers"][0]["authentication"]["sign_in"]["host"], HOST); assert_private(&read);
        let saved = repository.get_saved("login").unwrap().unwrap();
        assert_eq!(saved["authentication"]["status"], "unknown"); assert_private(&saved);
        assert_eq!(std::fs::read_to_string(sandbox.home.join("starts.log")).unwrap().lines().count(), 1);
        assert_eq!(manager.authenticate_mode("login", false).await.unwrap_err().code, "FORBIDDEN");
        std::fs::write(sandbox.home.join("login-complete"), "disposable consent complete").unwrap();
        let ready = until(&manager, "login", |value| value["runtime"]["status"] == "ready" && value["authentication"]["status"] != "authenticating").await;
        assert_eq!(ready["authentication"]["status"], "unknown", "MCP readiness is not account proof");
        assert!(ready["authentication"]["sign_in"].is_null()); assert_private(&ready);
        assert_eq!(repository.get_saved("login").unwrap().unwrap()["authentication"]["status"], "unknown");
        let output = manager.runner.call("login", "echo", raw::raw(&json!({})),
            InvocationContext::terminal("disposable-owner".into())).await.unwrap();
        assert_eq!(output.get(), "{ \"content\":[{\"type\":\"text\",\"text\":\"server-owned login fixture\"}], \"number\":184467440737095516160000001 }");
        let already_ready = manager.restart("login", json!({"interactive":true})).await.unwrap();
        assert_eq!(already_ready["status"], "unknown");
        assert_eq!(std::fs::read_to_string(sandbox.home.join("starts.log")).unwrap().lines().count(), 1);
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8), manager.shutdown()).await.expect("owned login process shutdown");
    if let Err(error) = result { std::panic::resume_unwind(error); }
    assert!(!alive(&sandbox.home.join("peer.pid")));
    let root = sandbox.root.clone(); drop(manager); drop(repository); drop(sandbox); drop(sibling); assert!(!root.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn recorded_helper_emits_sign_in_without_leaking_output_or_claiming_account_access() {
    let sandbox = fixture(); let sibling = Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let mut saved = record("helper", &sandbox, &[]);
    saved["_installation"] = json!({"source_path":"/disposable/source.json","source_name":"helper",
        "installation_id":format!("package-{}", "a".repeat(32)),"protected_args":0,
        "native_setup":{"executable":sandbox.launcher,"args":args(&sandbox,&["--helper"]),
            "working_directory":sandbox.home,"state_directory":sandbox.home}});
    let (manager, repository) = manager(&sandbox, &saved);
    let result = std::panic::AssertUnwindSafe(async {
        assert_eq!(manager.authenticate("helper").await.unwrap()["status"], "authenticating");
        let pending = until(&manager, "helper", |value| value["authentication"]["sign_in"]["host"] == HOST).await;
        assert_eq!(pending["authentication"]["sign_in"]["opened"], false); assert_private(&pending);
        std::fs::write(sandbox.home.join("login-complete"), "disposable consent complete").unwrap();
        let completed = until(&manager, "helper", |value| value["authentication"]["status"] != "authenticating").await;
        assert_eq!(completed["authentication"]["status"], "unknown");
        assert!(completed["authentication"]["sign_in"].is_null()); assert_private(&completed);
        assert_eq!(repository.get_saved("helper").unwrap().unwrap()["authentication"]["status"], "unknown");
        assert!(sandbox.home.join("native-login.json").is_file());
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8), manager.shutdown()).await.expect("owned helper shutdown");
    if let Err(error) = result { std::panic::resume_unwind(error); }
    assert!(!alive(&sandbox.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn explicit_login_waits_beyond_normal_startup_deadline_and_other_server_remains_usable() {
    let sandbox = fixture(); let other = Sandbox::new(); other.account("other-account");
    sandbox.assert_confinement(&other.home.join("account.json")).await;
    other.assert_confinement(&sandbox.peer).await;
    let runner = Runner::new();
    let mut connection_attempt=None;
    let result = std::panic::AssertUnwindSafe(async {
        runner.configure(route("login", &sandbox)).await.unwrap();
        runner.configure(ServerRoute {server_id:"other".into(),transport:TransportConfig::Stdio {
            executable:other.launcher.clone(),args:vec![other.peer.to_string_lossy().into_owned()],
            working_directory:Some(other.home.clone()),environment:other.environment()},execution:ExecutionPolicy::default()}).await.unwrap();
        runner.connect("other").await.unwrap();
        let (urls, mut observed) = tokio::sync::mpsc::channel(4);
        connection_attempt=Some({ let runner=runner.clone(); tokio::spawn(async move { runner.connect_for_authentication("login", urls).await }) });
        let url = tokio::time::timeout(Duration::from_secs(3), observed.recv()).await.unwrap().unwrap();
        assert_eq!(url::Url::parse(&url).unwrap().host_str(), Some(HOST));
        assert!(url.contains(STATE)); assert!(!url.contains(SECRET));
        // A real wait is deliberate: both old readiness deadlines were 30 seconds.
        tokio::time::sleep(Duration::from_millis(31_250)).await;
        assert!(!connection_attempt.as_ref().unwrap().is_finished(), "interactive consent must outlive the ordinary startup deadline");
        assert!(alive(&sandbox.home.join("peer.pid")));
        let quick = tokio::time::timeout(Duration::from_millis(500), runner.call("other", "echo", raw::raw(&json!({})),
            InvocationContext::terminal("disposable-owner".into()))).await.expect("another server remains usable").unwrap();
        assert_eq!(serde_json::from_str::<Value>(quick.get()).unwrap()["account"], "other-account");
        std::fs::write(sandbox.home.join("login-complete"), "disposable consent complete").unwrap();
        tokio::time::timeout(Duration::from_secs(3), connection_attempt.as_mut().unwrap()).await.unwrap().unwrap().unwrap();
        connection_attempt=None;
        assert_eq!(runner.snapshot("login").await["runtime"]["status"], "ready");
    }).catch_unwind().await;
    if let Some(attempt)=connection_attempt { attempt.abort(); let _=attempt.await; }
    tokio::time::timeout(Duration::from_secs(8), runner.shutdown()).await.expect("owned peer shutdown");
    if let Err(error) = result { std::panic::resume_unwind(error); }
    assert!(!alive(&sandbox.home.join("peer.pid"))); assert!(!alive(&other.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn absent_environment_values_allow_native_file_fallback_and_actual_child_failure() {
    let fallback=fixture(); let missing=fixture();
    fallback.assert_confinement(&missing.peer).await;
    missing.assert_confinement(&fallback.peer).await;
    let config_bytes=b"{\"installed\":{\"client_id\":\"disposable-client\",\"client_secret\":\"disposable-secret\"}}";
    std::fs::write(fallback.home.join("credentials.json"),config_bytes).unwrap();
    std::fs::write(fallback.home.join("login-complete"),"disposable login complete").unwrap();
    let make_record=|id:&str,sandbox:&Sandbox| {
        let mut saved=record(id,sandbox,&["--requires-native-config"]);
        saved["config"]["environment"]=json!([
            {"name":"FIXTURE_CLIENT_SECRET","credential_ref":"absent-client"},
            {"name":"FIXTURE_PROFILE","credential_ref":"absent-optional-profile"}
        ]);
        saved
    };
    let (fallback_manager,fallback_repository)=manager(&fallback,&make_record("fallback",&fallback));
    let (missing_manager,missing_repository)=manager(&missing,&make_record("missing",&missing));
    let result=std::panic::AssertUnwindSafe(async {
        // Ordinary startup (also used after installation) must reach the child.
        assert_eq!(fallback_manager.restart("fallback",json!({})).await.unwrap()["status"],"restarting");
        let ready=until(&fallback_manager,"fallback",|value|value["runtime"]["status"]=="ready").await;
        assert_eq!(ready["authentication"]["status"],"unknown");
        assert_eq!(std::fs::read_to_string(fallback.home.join("configuration-source.txt")).unwrap(),"native-file");
        assert_eq!(std::fs::read_to_string(fallback.home.join("profile.txt")).unwrap(),"default");
        assert_eq!(std::fs::read(fallback.home.join("credentials.json")).unwrap(),config_bytes);
        assert_eq!(fallback_manager.restart("fallback",json!({"interactive":true})).await.unwrap()["status"],"unknown");
        assert_eq!(std::fs::read_to_string(fallback.home.join("starts.log")).unwrap().lines().count(),1);
        assert_private(&fallback_repository.get_saved("fallback").unwrap().unwrap());
        // A truly required setting is rejected by the server, after it really starts.
        assert_eq!(missing_manager.restart("missing",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        let failed=until(&missing_manager,"missing",|value|value["authentication"]["status"]=="failed"
            && value["authentication"]["last_known"]["status"]=="failed").await;
        assert_eq!(failed["runtime"]["status"],"failed");
        assert_eq!(std::fs::read_to_string(missing.home.join("starts.log")).unwrap().lines().count(),1);
        assert_eq!(std::fs::read_to_string(missing.home.join("self-reported-failure")).unwrap(),"the server itself rejected missing setup");
        assert!(!failed.to_string().contains("SERVER_INPUT_REQUIRED"));
        assert_eq!(missing_repository.get_saved("missing").unwrap().unwrap()["authentication"]["status"],"failed");
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),fallback_manager.shutdown()).await.unwrap();
    tokio::time::timeout(Duration::from_secs(8),missing_manager.shutdown()).await.unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    assert!(!alive(&fallback.home.join("peer.pid")));assert!(!alive(&missing.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn quit_awaits_pending_recorded_helper_and_its_owned_descendant() {
    let sandbox=fixture(); let sibling=Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let mut saved=record("helper-quit", &sandbox, &[]);
    saved["_installation"]=json!({"source_path":"/disposable/source.json","source_name":"helper-quit",
        "installation_id":format!("package-{}","b".repeat(32)),"protected_args":0,
        "native_setup":{"executable":sandbox.launcher,"args":args(&sandbox,&["--helper","--descendant"]),
            "working_directory":sandbox.home,"state_directory":sandbox.home}});
    let (manager, repository)=manager(&sandbox,&saved);
    let result=std::panic::AssertUnwindSafe(async {
        assert_eq!(manager.restart("helper-quit",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        until(&manager,"helper-quit",|value| value["authentication"]["sign_in"]["host"]==HOST).await;
        tokio::time::timeout(Duration::from_secs(2), async {
            while !sandbox.home.join("descendant.pid").is_file() { tokio::time::sleep(Duration::from_millis(10)).await; }
        }).await.unwrap();
        assert!(alive(&sandbox.home.join("peer.pid"))); assert!(alive(&sandbox.home.join("descendant.pid")));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("Quit must await recorded helper cleanup");
    if let Err(error)=result { std::panic::resume_unwind(error); }
    assert!(!alive(&sandbox.home.join("peer.pid")),"shutdown returned with its recorded helper still alive");
    assert!(!alive(&sandbox.home.join("descendant.pid")),"shutdown returned with an owned helper descendant still alive");
    assert!(!sandbox.home.join("native-login.json").exists());
    assert_private(&repository.get_saved("helper-quit").unwrap().unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn quit_during_pending_login_reaps_owned_process_and_saved_result_is_not_current_proof() {
    let sandbox = fixture(); let sibling = Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let mut saved = record("quit", &sandbox, &[]);
    saved["authentication"]["status"] = json!("authenticated");
    let (manager, repository) = manager(&sandbox, &saved);
    let result = std::panic::AssertUnwindSafe(async {
        let before = status(&manager, "quit").await;
        assert_eq!(before["authentication"]["status"], "unknown");
        assert_eq!(before["authentication"]["last_known"]["status"], "authenticated");
        assert_eq!(manager.authenticate("quit").await.unwrap()["status"], "authenticating");
        until(&manager, "quit", |value| value["authentication"]["sign_in"]["host"] == HOST).await;
        assert!(alive(&sandbox.home.join("peer.pid")));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8), manager.shutdown()).await.expect("Quit must cancel pending login before ten-minute deadline");
    if let Err(error) = result { std::panic::resume_unwind(error); }
    tokio::time::timeout(Duration::from_secs(3), async {
        while alive(&sandbox.home.join("peer.pid")) { tokio::time::sleep(Duration::from_millis(20)).await; }
    }).await.expect("owned process must be reaped after Quit");
    assert_private(&repository.get_saved("quit").unwrap().unwrap());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn recorded_login_helper_cannot_start_while_its_server_has_an_active_call() {
    let sandbox=fixture(); let sibling=Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    std::fs::write(sandbox.home.join("login-complete"),"already signed in fixture").unwrap();
    let mut saved=record("busy",&sandbox,&[]);
    saved["_installation"]=json!({"source_path":"/disposable/source.json","source_name":"busy",
        "installation_id":format!("package-{}","c".repeat(32)),"protected_args":0,
        "native_setup":{"executable":sandbox.launcher,"args":args(&sandbox,&["--helper"]),
            "working_directory":sandbox.home,"state_directory":sandbox.home}});
    let (manager,_repository)=manager(&sandbox,&saved);
    let mut held=None;
    let result=std::panic::AssertUnwindSafe(async {
        manager.runner.configure(route("busy",&sandbox)).await.unwrap(); manager.runner.connect("busy").await.unwrap();
        held=Some({let runner=manager.runner.clone();tokio::spawn(async move {
            runner.call("busy","echo",raw::raw(&json!({"hold":true})),InvocationContext::terminal("disposable-owner".into())).await
        })});
        tokio::time::timeout(Duration::from_secs(2),async {
            while !sandbox.home.join("call-entered").exists() {tokio::time::sleep(Duration::from_millis(10)).await;}
        }).await.unwrap();
        assert_eq!(manager.restart("busy",json!({"interactive":true})).await.unwrap_err().code,"SERVER_BUSY");
        assert_eq!(std::fs::read_to_string(sandbox.home.join("starts.log")).unwrap().lines().count(),1,"busy server must not spawn its login helper");
        std::fs::write(sandbox.home.join("release-call"),"finish existing call").unwrap();
        tokio::time::timeout(Duration::from_secs(3),held.as_mut().unwrap()).await.unwrap().unwrap().unwrap(); held=None;
    }).catch_unwind().await;
    if let Some(call)=held {call.abort();let _=call.await;}
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("owned busy peer shutdown");
    if let Err(error)=result {std::panic::resume_unwind(error);}
    assert!(!alive(&sandbox.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn registered_stopped_server_uses_newly_saved_inputs_when_login_is_retried() {
    let sandbox=fixture(); let sibling=Sandbox::new(); sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    std::fs::write(sandbox.home.join("login-complete"),"disposable login already complete").unwrap();
    let mut saved=record("repaired-input",&sandbox,&[]);
    saved["config"]["environment"]=json!([{"name":"FIXTURE_LOGIN_INPUT","credential_ref":"assigned-input"}]);
    let root=sandbox.root.join("fetch-state"); let repository=ServerRepository::new(root.clone());
    repository.save("repaired-input",&saved).unwrap();
    let grants=Arc::new(GrantStore::memory(root.clone()));
    grants.save_section("repaired-input","manual",json!({"assigned-input":"new-disposable-input"})).await.unwrap();
    let discovery=DiscoveryService::new(root.join("unused-source-home"),None);
    let manager=Manager::with_grant_store(repository,grants,Runner::new(),Installer::new(root,discovery));
    let result=std::panic::AssertUnwindSafe(async {
        let mut stale=route("repaired-input",&sandbox);
        let TransportConfig::Stdio {environment,..}=&mut stale.transport else {unreachable!()};
        environment.insert("FIXTURE_LOGIN_INPUT".into(),"old-disposable-input".into());
        manager.runner.configure(stale).await.unwrap();
        assert_eq!(manager.runner.snapshot("repaired-input").await["runtime"]["status"],"stopped");
        assert_eq!(manager.restart("repaired-input",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        until(&manager,"repaired-input",|value| value["runtime"]["status"]=="ready" && value["authentication"]["status"]!="authenticating").await;
        assert_eq!(std::fs::read_to_string(sandbox.home.join("login-input.txt")).unwrap(),"new-disposable-input", "registered stopped server must replace stale process inputs before spawning");
        assert_eq!(std::fs::read_to_string(sandbox.home.join("starts.log")).unwrap().lines().count(),1);
        let ready=status(&manager,"repaired-input").await;
        assert!(!ready.to_string().contains("new-disposable-input"));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("owned repaired peer shutdown");
    if let Err(error)=result {std::panic::resume_unwind(error);}
    assert!(!alive(&sandbox.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn absent_header_values_reach_remote_server_and_actual_response_decides_outcome() {
    for reject in [false,true] {
        let directory=tempfile::tempdir().unwrap(); let root=directory.path().join("state");
        let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint=format!("http://{}/mcp",listener.local_addr().unwrap());
        let received=Arc::new(RemoteSetup {reject,initialize_headers:std::sync::Mutex::new(Vec::new())});
        let service=Router::new().route("/mcp",axum::routing::post(remote_setup)).with_state(received.clone());
        let server=tokio::spawn(async move {axum::serve(listener,service).await.unwrap()});
        let repository=ServerRepository::new(root.clone());
        repository.save("remote",&json!({"server_id":"remote","name":"remote",
            "runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},
            "restart_required":false,"restart":null,"config":{"kind":"remote","package":null,"executable":null,
                "args":[],"working_directory":null,"environment":[],"connection":{"url":endpoint,"transport":"streamable_http",
                    "headers":[{"name":"X-Optional","credential_ref":"absent"},{"name":"X-Supplied","credential_ref":"present"}]}}})).unwrap();
        let grants=Arc::new(GrantStore::memory(root.clone()));
        grants.save_section("remote","manual",json!({"present":"fixture-exact-header"})).await.unwrap();
        let discovery=DiscoveryService::new(root.join("unused-source-home"),None);
        let manager=Manager::with_grant_store(repository,grants,Runner::new(),Installer::new(root,discovery));
        let result=std::panic::AssertUnwindSafe(async {
            assert_eq!(manager.restart("remote",json!({})).await.unwrap()["status"],"restarting");
            let observed=until(&manager,"remote",|value| value["runtime"]["status"]==if reject {"failed"} else {"ready"}).await;
            let attempts=received.initialize_headers.lock().unwrap();
            assert_eq!(attempts.len(),1,"the configured remote startup must actually send initialize");
            assert!(!attempts[0].contains_key("X-Optional"));
            assert_eq!(attempts[0]["X-Supplied"],"fixture-exact-header");
            if reject {
                assert_eq!(observed["authentication"]["status"],"required","real HTTP401 may require login after an actual attempt");
                assert_eq!(observed["runtime"]["error"]["code"],"PROVIDER_AUTH_REQUIRED");
            } else {
                assert_eq!(observed["runtime"]["status"],"ready");
            }
            assert!(!observed.to_string().contains("SERVER_INPUT_REQUIRED"));
            assert!(!observed.to_string().contains("fixture-exact-header"));
        }).catch_unwind().await;
        tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("disposable remote shutdown");
        server.abort();let _=server.await;
        if let Err(error)=result {std::panic::resume_unwind(error);}
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn actual_fast_startup_failure_is_visible_only_to_native_settings_with_exit_status_and_redacted_output() {
    let sandbox=fixture();let sibling=Sandbox::new();sibling.account("separate-account");
    sandbox.assert_confinement(&sibling.peer).await;
    let mut saved=record("diagnostic",&sandbox,&["--requires-native-config","--diagnostic-details"]);
    saved["config"]["environment"]=json!([{"name":"FIXTURE_STARTUP_SECRET","credential_ref":"assigned-diagnostic-key"}]);
    let root=sandbox.root.join("fetch-state");let repository=ServerRepository::new(root.clone());repository.save("diagnostic",&saved).unwrap();
    let grants=Arc::new(GrantStore::memory(root.clone()));
    grants.save_section("diagnostic","manual",json!({"assigned-diagnostic-key":"assigned-diagnostic-private-value"})).await.unwrap();
    let discovery=DiscoveryService::new(root.join("unused-source-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),grants,Runner::new(),Installer::new(root,discovery.clone()));
    let native=NativeAuthority::new();let session=native.open_settings();
    let mut state=AppState::new("https://localhost:53672".into(),repository.clone(),discovery,Arc::new(DiagnosticAuthority(native.clone()))).unwrap();
    state.manager=Some(manager.clone());state.native=Some(native);let app=router(Arc::new(state));
    let native_request=|| {
        let mut read=request("/api/v1/servers?id=diagnostic","GET",None);
        read.headers_mut().insert("x-fetch-settings-capability",session.header_value().parse().unwrap());read
    };
    let result=std::panic::AssertUnwindSafe(async {
        assert_eq!(manager.restart("diagnostic",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        let failed=until(&manager,"diagnostic",|value|value["authentication"]["status"]=="failed").await;
        assert!(failed["authentication"].get("startup").is_none(),"plain Manager status must not include startup stderr");
        let native_read=tokio::time::timeout(Duration::from_secs(5),async {
            loop {
                let response=app.clone().oneshot(native_request()).await.unwrap();assert_eq!(response.status(),StatusCode::OK);
                let value=response_body(response).await;
                if value["servers"][0]["authentication"]["startup"]["exit_code"]==3 {break value;}
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("the real child exit3 must reach native status");
        let startup=&native_read["servers"][0]["authentication"]["startup"];
        assert_eq!(startup["exit_code"],3);assert!(startup["signal"].is_null(),"cleanup SIGTERM must not replace an already observed natural exit");
        let lines=startup["lines"].as_array().unwrap().iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n");
        assert!(lines.contains("FATAL: this disposable server has no client setup"),"the fast final stderr line must not be lost");
        for private in ["assigned-diagnostic-private-value","https://","diagnostic-private-state","diagnostic-private-url-secret"] {
            assert!(!startup.to_string().contains(private),"startup diagnostics leaked {private}");
        }
        let frozen=startup.clone();
        let second=response_body(app.clone().oneshot(native_request()).await.unwrap()).await;
        assert_eq!(second["servers"][0]["authentication"]["startup"],frozen,"completed attempt diagnostics stay frozen");
        let grant_read=response_body(app.clone().oneshot(request("/api/v1/servers?id=diagnostic","GET",None)).await.unwrap()).await;
        assert!(grant_read["servers"][0]["authentication"].get("startup").is_none());
        assert!(!grant_read.to_string().contains("FATAL:"));
        let disk=repository.get_saved("diagnostic").unwrap().unwrap();
        assert!(disk["authentication"].get("startup").is_none());assert!(!disk.to_string().contains("FATAL:"));
        assert!(!disk.to_string().contains("assigned-diagnostic-private-value"));
        // A new attempt retains sanitized history and starts with fresh current exit status.
        let mut repaired=repository.get_saved("diagnostic").unwrap().unwrap();repaired["config"]["args"]=json!(args(&sandbox,&[]));
        repository.save("diagnostic",&repaired).unwrap();
        assert_eq!(manager.restart("diagnostic",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        until(&manager,"diagnostic",|value|value["authentication"]["sign_in"]["host"]==HOST).await;
        let next=response_body(app.clone().oneshot(native_request()).await.unwrap()).await;
        let next_startup=&next["servers"][0]["authentication"]["startup"];
        assert!(next_startup["generation"].as_u64().unwrap()>frozen["generation"].as_u64().unwrap());
        assert!(next_startup["exit_code"].is_null());assert!(next_startup["signal"].is_null());
        let next_lines=next_startup["lines"].as_array().unwrap().iter().filter_map(Value::as_str).collect::<Vec<_>>();
        assert_eq!(next_lines.iter().filter(|line|line.contains("FATAL: this disposable server has no client setup")).count(),1);
        assert_eq!(next_lines.iter().filter(|line|**line=="Process exited with status 3.").count(),1);
        for private in ["assigned-diagnostic-private-value","https://","diagnostic-private-state","diagnostic-private-url-secret"] {
            assert!(!next_startup.to_string().contains(private),"retained startup diagnostics leaked {private}");
        }
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("owned diagnostic process shutdown");
    if let Err(error)=result {std::panic::resume_unwind(error);}
    assert!(!alive(&sandbox.home.join("peer.pid")));
}
