//! Product-route power checks with synthetic children in the existing OS sandbox.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;

use axum::{Router, body::Body, http::{HeaderMap, Request, StatusCode}, response::Response};
use confinement::Sandbox;
use fetch_local::{
    api::{AppState, router, runtime::ManagementAdapter},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    installation::Installer,
    manager::Manager,
    mcp::McpGateway,
    remote_oauth::GrantStore,
    runner::{InvocationContext, MODERN, Runner, raw},
    storage::ServerRepository,
};
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{path::Path, sync::Arc, time::Duration};
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53672";
const ID: &str = "power-fixture";

struct FixtureAuthority;
impl Authorizer for FixtureAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        match headers.get("authorization").and_then(|value| value.to_str().ok()) {
            Some("Bearer fixture-manage") => Ok(Caller::from_verified_scopes([
                "fetch:manage".into(), "fetch:catalog".into(), "fetch:tools:call".into(),
            ])),
            Some("Bearer fixture-catalog") => Ok(Caller::from_verified_scopes(["fetch:catalog".into()])),
            _ => Err(AuthorizationError::Unauthenticated),
        }
    }
}

fn fixture() -> Sandbox {
    let sandbox = Sandbox::new();
    std::fs::write(&sandbox.peer, include_str!("native_state/login_peer.py")).unwrap();
    std::fs::write(sandbox.home.join("login-complete"), "fixture already signed in").unwrap();
    sandbox
}
fn record(sandbox: &Sandbox) -> Value {
    // No enabled field: existing records must retain their normal On default.
    json!({"server_id":ID,"name":"Power fixture","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"local_script","package":null,"executable":sandbox.launcher,
            "args":[sandbox.peer,sandbox.home],"working_directory":sandbox.home,"environment":[]}})
}
fn manager(sandbox: &Sandbox, repository: ServerRepository) -> Arc<Manager> {
    let root = repository.root().to_owned();
    let discovery = DiscoveryService::new(sandbox.root.join("unused-source-home"), None);
    Manager::with_grant_store(repository, Arc::new(GrantStore::memory(root.clone())),
        Runner::new(), Installer::new(root, discovery))
}
fn app(sandbox: &Sandbox, manager: Arc<Manager>, repository: ServerRepository) -> Router {
    let discovery = DiscoveryService::new(sandbox.root.join("unused-source-home"), None);
    let mut state = AppState::new(ORIGIN.into(), repository, discovery, Arc::new(FixtureAuthority)).unwrap();
    state.manager = Some(manager);
    router(Arc::new(state))
}
async fn body(response: Response) -> Value {
    let status=response.status();
    let bytes=response.into_body().collect().await.unwrap().to_bytes();
    if let Ok(value)=serde_json::from_slice(&bytes){return value;}
    // Management tool calls use the same native SSE result framing as all MCP calls.
    let text=std::str::from_utf8(&bytes).unwrap();
    text.lines().filter_map(|line|line.strip_prefix("data:")).filter_map(|line|serde_json::from_str::<Value>(line.trim()).ok())
        .find(|value|value["id"]==7).unwrap_or_else(||panic!("missing terminal JSON response ({status}): {text}"))
}
async fn http(app: &Router, method: &str, path: &str, input: Value, token: &str) -> (StatusCode, Value) {
    let request = Request::builder().method(method).uri(path)
        .header("host", "localhost:53672").header("authorization", token)
        .header("content-type", "application/json")
        .body(if method == "GET" { Body::empty() } else { Body::from(input.to_string()) }).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    let status = response.status();
    (status, body(response).await)
}
async fn post(app: &Router, input: Value) -> (StatusCode, Value) {
    http(app, "POST", &format!("/api/v1/servers/{ID}/restart"), input, "Bearer fixture-manage").await
}
async fn wait_ready(manager: &Manager) {
    tokio::time::timeout(Duration::from_secs(6), async {
        loop {
            let value = manager.servers(vec![ID.into()], None).await.unwrap();
            if value["servers"][0]["runtime"]["status"] == "ready"
                && value["servers"][0]["restart"]["status"] == "succeeded"
                && value["servers"][0]["config"]["tool_features"]["status"] == "observed" { break; }
            if value["servers"][0]["restart"]["status"] == "failed" {
                panic!("fixture start failed: {}", value["servers"][0]["restart"]);
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("fixture server becomes ready");
}
fn alive(path: &Path) -> bool {
    let Ok(pid) = std::fs::read_to_string(path).and_then(|text| text.parse::<i32>().map_err(std::io::Error::other)) else { return false; };
    // Signal zero only checks this fixture's saved PID; it sends no signal.
    unsafe { libc::kill(pid, 0) == 0 }
}
fn starts(sandbox: &Sandbox) -> usize {
    std::fs::read_to_string(sandbox.home.join("starts.log")).unwrap_or_default().lines().count()
}
async fn shutdown(manager: &Manager) {
    tokio::time::timeout(Duration::from_secs(8), manager.shutdown()).await.expect("fixture-owned shutdown");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_power_http_stop_preserves_files_survives_restart_and_existing_post_turns_on() {
    let sandbox = fixture(); let sibling = Sandbox::new(); sibling.account("confinement-canary");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let repository = ServerRepository::new(sandbox.root.join("fetch-state"));
    repository.save(ID, &record(&sandbox)).unwrap();
    let first = manager(&sandbox, repository.clone());
    let mut owned = vec![first.clone()];
    let result = std::panic::AssertUnwindSafe(async {
        let application = app(&sandbox, first.clone(), repository.clone());
        let denied = http(&application, "POST", &format!("/api/v1/servers/{ID}/restart"), json!({"action":"stop"}), "Bearer fixture-catalog").await;
        assert_eq!(denied.0, StatusCode::FORBIDDEN);
        assert_eq!(post(&application, json!({})).await.0, StatusCode::ACCEPTED);
        wait_ready(&first).await;
        assert!(alive(&sandbox.home.join("peer.pid")));
        let login_file = sandbox.home.join("native-login.json");
        let login_before = std::fs::read(&login_file).unwrap();
        let stopped = post(&application, json!({"action":"stop"})).await;
        assert_eq!(stopped.0, StatusCode::OK);
        assert_eq!(stopped.1["status"], "stopped"); assert_eq!(stopped.1["enabled"], false);
        assert_eq!(stopped.1["changed"], true); assert_eq!(stopped.1["next_steps"], json!([]));
        assert!(!alive(&sandbox.home.join("peer.pid")), "Off confirms the owned child exited");
        assert_eq!(std::fs::read(&login_file).unwrap(), login_before);
        assert!(sandbox.peer.exists(), "Off retains the installed executable/script");
        assert_eq!(repository.get_saved(ID).unwrap().unwrap()["config"]["enabled"], false);
        assert_eq!(post(&application, json!({"action":"stop"})).await.1["changed"], false);
        let listed = http(&application, "GET", &format!("/api/v1/servers?id={ID}"), Value::Null, "Bearer fixture-manage").await;
        assert_eq!(listed.1["servers"][0]["runtime"]["status"], "stopped");
        assert_eq!(listed.1["servers"][0]["config"]["enabled"], false);
        shutdown(&first).await;

        let second = manager(&sandbox, repository.clone()); owned.push(second.clone());
        second.startup().await;
        assert_eq!(starts(&sandbox), 1, "a fresh Fetch run must not start a saved Off server");
        assert!(!alive(&sandbox.home.join("peer.pid")));
        let application = app(&sandbox, second.clone(), repository.clone());
        assert_eq!(post(&application, json!({})).await.0, StatusCode::ACCEPTED, "the existing POST is also On");
        wait_ready(&second).await;
        assert_eq!(repository.get_saved(ID).unwrap().unwrap()["config"]["enabled"], true);
        assert_eq!(starts(&sandbox), 2);
        let output = second.runner.call(ID, "echo", raw::raw(&json!({"marker":"after-explicit-on"})),
            InvocationContext::terminal("power-test".into())).await.unwrap();
        assert!(output.get().contains("server-owned login fixture"), "the server is usable again after On");
    }).catch_unwind().await;
    for manager in owned { shutdown(&manager).await; }
    if let Err(error) = result { std::panic::resume_unwind(error); }
    assert!(!alive(&sandbox.home.join("peer.pid")));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn server_power_busy_stop_preserves_active_and_waiting_calls() {
    let sandbox = fixture(); let sibling = Sandbox::new(); sibling.account("confinement-canary");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    let repository = ServerRepository::new(sandbox.root.join("fetch-state"));
    repository.save(ID, &record(&sandbox)).unwrap();
    let manager = manager(&sandbox, repository.clone());
    let mut held = None; let mut queued = None;
    let result = std::panic::AssertUnwindSafe(async {
        let application = app(&sandbox, manager.clone(), repository.clone());
        assert_eq!(post(&application, json!({})).await.0, StatusCode::ACCEPTED);
        wait_ready(&manager).await;
        let runner = manager.runner.clone();
        held = Some(tokio::spawn(async move { runner.call(ID, "echo", raw::raw(&json!({"hold":true})),
            InvocationContext::terminal("power-held".into())).await }));
        tokio::time::timeout(Duration::from_secs(3), async {
            while !sandbox.home.join("call-entered").exists() { tokio::task::yield_now().await; }
        }).await.unwrap();
        let runner = manager.runner.clone();
        queued = Some(tokio::spawn(async move { runner.call(ID, "echo", raw::raw(&json!({"marker":"waiting"})),
            InvocationContext::terminal("power-waiting".into())).await }));
        tokio::time::timeout(Duration::from_secs(3), async {
            while manager.runner.snapshot(ID).await["tool_execution"]["queued_calls"] != 1 { tokio::task::yield_now().await; }
        }).await.unwrap();
        let rejected = post(&application, json!({"action":"stop"})).await;
        assert_eq!(rejected.0, StatusCode::CONFLICT); assert_eq!(rejected.1["error"]["code"], "SERVER_BUSY");
        assert_ne!(repository.get_saved(ID).unwrap().unwrap()["config"]["enabled"], false);
        let counts = manager.runner.snapshot(ID).await;
        assert_eq!(counts["tool_execution"]["active_calls"], 1); assert_eq!(counts["tool_execution"]["queued_calls"], 1);
        assert!(alive(&sandbox.home.join("peer.pid")), "busy Off cannot kill the server");
        std::fs::write(sandbox.home.join("release-call"), "finish the owned call").unwrap();
        for task in [&mut held, &mut queued] {
            let output = tokio::time::timeout(Duration::from_secs(3), task.as_mut().unwrap()).await.unwrap().unwrap().unwrap();
            assert!(output.get().contains("server-owned login fixture")); *task = None;
        }
        assert_eq!(post(&application, json!({"action":"stop"})).await.0, StatusCode::OK);
        assert!(!alive(&sandbox.home.join("peer.pid")));
    }).catch_unwind().await;
    for task in [held, queued].into_iter().flatten() { task.abort(); let _ = task.await; }
    shutdown(&manager).await;
    if let Err(error) = result { std::panic::resume_unwind(error); }
}

fn modern(method: &str, mut params: Value) -> Request<Body> {
    params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{}});
    let mut request = Request::builder().method("POST").uri("/mcp")
        .header("host", "localhost:53672").header("authorization", "Bearer fixture-manage")
        .header("content-type", "application/json").header("accept", "application/json, text/event-stream")
        .header("mcp-protocol-version", MODERN).header("mcp-method", method);
    if method == "tools/call" { request = request.header("mcp-name", "fetch__restart_server"); }
    request.body(Body::from(json!({"jsonrpc":"2.0","id":7,"method":method,"params":params}).to_string())).unwrap()
}

#[tokio::test]
async fn server_power_mcp_schema_and_dispatch_share_the_real_stop_backend() {
    let directory = tempfile::tempdir().unwrap(); let root = directory.path().join("state");
    let repository = ServerRepository::new(root.clone());
    repository.save(ID, &json!({"server_id":ID,"name":"Stopped fixture","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://uncontacted.fixture.invalid/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
    let discovery = DiscoveryService::new(directory.path().join("empty-source-home"), None);
    let manager = Manager::with_grant_store(repository.clone(), Arc::new(GrantStore::memory(root.clone())), Runner::new(), Installer::new(root, discovery.clone()));
    let adapter = Arc::new(ManagementAdapter { manager:manager.clone(), discovery, workspace:None });
    let gateway = McpGateway::new(manager.runner.clone(), adapter, Arc::new(FixtureAuthority), ORIGIN.into());
    let application = gateway.router();
    let listed = body(application.clone().oneshot(modern("tools/list", json!({}))).await.unwrap()).await;
    let definition = listed["result"]["tools"].as_array().unwrap().iter().find(|tool|tool["name"]=="fetch__restart_server").unwrap();
    assert!(definition["inputSchema"]["properties"]["action"]["enum"].as_array().unwrap().contains(&json!("stop")));
    let stopped = body(application.clone().oneshot(modern("tools/call", json!({"name":"fetch__restart_server","arguments":{"server_id":ID,"action":"stop"}}))).await.unwrap()).await;
    assert!(stopped.get("error").is_none(), "the advertised action dispatches: {stopped}");
    assert_eq!(repository.get_saved(ID).unwrap().unwrap()["config"]["enabled"], false);
    assert_eq!(manager.runner.snapshot(ID).await["runtime"]["status"], "stopped");
    shutdown(&manager).await;
}
