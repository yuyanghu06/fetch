//! Independent acceptance checks using in-process requests and disposable files.
//! No native approval, Keychain, upstream connection or managed process is used.
use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode, header},
    response::Response,
};
use fetch_local::{
    api::{AppState, router},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    mcp::{ManagementBackend, McpGateway},
    native_auth::NativeAuthority,
    runner::{ExecutionPolicy, InvocationContext, RunError, Runner, ServerRoute, TransportConfig,
        LEGACY, MODERN, catalog, raw},
    storage::ServerRepository,
};
use http_body_util::BodyExt;
use serde_json::{Value, json, value::RawValue};
use std::{
    collections::BTreeMap,
    future::Future,
    path::PathBuf,
    pin::Pin,
    sync::{Arc, atomic::{AtomicUsize, Ordering}},
};
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53672";

struct FixtureAuthority {
    scopes: Vec<String>,
}
impl Authorizer for FixtureAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        if headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok())
            != Some("Bearer disposable-fixture")
        {
            return Err(AuthorizationError::Unauthenticated);
        }
        Ok(Caller::from_verified_scopes(self.scopes.clone()))
    }
}

#[derive(Default)]
struct FixtureManagement {
    calls: AtomicUsize,
}
impl ManagementBackend for FixtureManagement {
    fn invoke<'a>(
        &'a self,
        _name: &'a str,
        _arguments: Box<RawValue>,
        _caller: Caller,
        _view: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Value, RunError>> + Send + 'a>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(json!({"status":"ok","next_steps":[]})) })
    }
}

fn fixture(scopes: &[&str]) -> (Arc<McpGateway>, Arc<FixtureManagement>, Router) {
    let management = Arc::new(FixtureManagement::default());
    let authority = Arc::new(FixtureAuthority {
        scopes: scopes.iter().map(|scope| (*scope).to_owned()).collect(),
    });
    let gateway = McpGateway::new(Runner::new(), management.clone(), authority, ORIGIN.into());
    (gateway.clone(), management, gateway.router())
}

fn request(body: impl Into<Body>, revision: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/mcp")
        .header(header::HOST, "localhost:53672")
        .header(header::AUTHORIZATION, "Bearer disposable-fixture")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json, text/event-stream")
        .header("mcp-protocol-version", revision)
        .body(body.into())
        .unwrap()
}

fn modern(method: &str, mut params: Value) -> Request<Body> {
    params["_meta"] = json!({
        "io.modelcontextprotocol/protocolVersion": MODERN,
        "io.modelcontextprotocol/clientCapabilities": {}
    });
    let mut input = request(json!({"jsonrpc":"2.0","id":7,"method":method,"params":params}).to_string(), MODERN);
    input.headers_mut().insert("mcp-method", method.parse().unwrap());
    input
}

async fn json_body(response: Response) -> Value {
    serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()
}

#[tokio::test]
async fn catalog_grant_only_lists_the_catalog_management_tool() {
    let (_, management, app) = fixture(&["fetch:catalog"]);
    let response = app.oneshot(modern("tools/list", json!({}))).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    let tools = body["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0]["name"], "fetch__list_tools");
    assert_eq!(body["result"]["resultType"], "complete");
    assert_eq!(body["result"]["_meta"]["fetch/catalog"]["complete"], true);
    assert!(body.get("next_steps").is_none(), "Native MCP retains its protocol envelope");
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn catalog_and_tool_call_grants_cannot_dispatch_server_management() {
    let (_, management, app) = fixture(&["fetch:catalog", "fetch:tools:call"]);
    let mut input = modern("tools/call", json!({
        "name":"fetch__install_servers",
        "arguments":{"servers":[{"config_path":"/disposable/config.json","server_name":"fixture"}]}
    }));
    input.headers_mut().insert("mcp-name", "fetch__install_servers".parse().unwrap());
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn unauthenticated_protocol_input_is_denied_before_parsing() {
    let (_, management, app) = fixture(&["fetch:manage"]);
    let mut input = request("not JSON and not a real credential", MODERN);
    input.headers_mut().remove(header::AUTHORIZATION);
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert!(response.headers()[header::WWW_AUTHENTICATE].to_str().unwrap()
        .contains("/.well-known/oauth-protected-resource/mcp"));
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn modern_method_and_name_must_match_the_protocol_body() {
    let (_, management, app) = fixture(&["fetch:manage"]);
    let mut input = modern("tools/list", json!({}));
    input.headers_mut().insert("mcp-method", "tools/call".parse().unwrap());
    assert_eq!(app.clone().oneshot(input).await.unwrap().status(), StatusCode::BAD_REQUEST);

    let mut input = modern("tools/call", json!({"name":"fetch__get_servers","arguments":{}}));
    input.headers_mut().insert("mcp-name", "fetch__restart_server".parse().unwrap());
    assert_eq!(app.oneshot(input).await.unwrap().status(), StatusCode::BAD_REQUEST);
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn nested_duplicate_protocol_values_are_rejected_before_dispatch() {
    let (_, management, app) = fixture(&["fetch:manage"]);
    let mut input = request(r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"fetch__restart_server","arguments":{"server_id":"first","server_id":"second"}}}"#, MODERN);
    input.headers_mut().insert("mcp-method", "tools/call".parse().unwrap());
    input.headers_mut().insert("mcp-name", "fetch__restart_server".parse().unwrap());
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(json_body(response).await["error"]["code"], -32700);
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn legacy_session_requires_initialization_and_is_removed_on_revocation() {
    let (gateway, management, app) = fixture(&["fetch:catalog"]);
    let initialize = request(json!({"jsonrpc":"2.0","id":1,"method":"initialize",
        "params":{"protocolVersion":LEGACY,"capabilities":{},"clientInfo":{"name":"disposable-test","version":"1"}}}).to_string(), LEGACY);
    let response = app.clone().oneshot(initialize).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let session = response.headers()["mcp-session-id"].clone();
    assert_eq!(json_body(response).await["result"]["protocolVersion"], LEGACY);

    let list = || {
        let mut input = request(json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}).to_string(), LEGACY);
        input.headers_mut().insert("mcp-session-id", session.clone());
        input
    };
    assert_eq!(app.clone().oneshot(list()).await.unwrap().status(), StatusCode::BAD_REQUEST);
    let mut initialized = request(json!({"jsonrpc":"2.0","method":"notifications/initialized","params":{}}).to_string(), LEGACY);
    initialized.headers_mut().insert("mcp-session-id", session.clone());
    assert_eq!(app.clone().oneshot(initialized).await.unwrap().status(), StatusCode::ACCEPTED);
    let response = app.clone().oneshot(list()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["result"]["tools"][0]["name"], "fetch__list_tools");

    gateway.revoke_owner("trusted-injected-authority").await;
    assert_eq!(app.oneshot(list()).await.unwrap().status(), StatusCode::NOT_FOUND);
    assert_eq!(management.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn ordinary_agent_cannot_write_manual_secrets_even_with_management_scope() {
    let directory = tempfile::tempdir().unwrap();
    let authority = Arc::new(FixtureAuthority { scopes: vec!["fetch:manage".into()] });
    let mut state = AppState::new(ORIGIN.into(), ServerRepository::new(directory.path().join("state")),
        DiscoveryService::new(directory.path().join("fixture-home"), None), authority).unwrap();
    state.native = Some(NativeAuthority::new());
    let app = router(Arc::new(state));
    let input = Request::builder().method("PATCH")
        .uri("/api/v1/servers/disposable/credentials")
        .header(header::HOST, "localhost:53672")
        .header(header::AUTHORIZATION, "Bearer disposable-fixture")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"set":[{"name":"API_KEY","value":"disposable-canary-value"}]}"#)).unwrap();
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], "FORBIDDEN");
    assert_eq!(body["next_steps"], json!([]));
    assert!(!body.to_string().contains("disposable-canary-value"));
    assert!(!directory.path().join("state").exists(), "Denial must not create saved state");
}

#[test]
fn partial_modern_catalog_retains_raw_usable_definitions_and_legacy_view() {
    let valid = r#"{ "name":"valid", "future":{"number":184467440737095516160000000001}, "inputSchema":{"type":"object"}, "outputSchema":{"futureOutput":true} }"#;
    let rejected = r#"{"name":"bad-header","inputSchema":{"type":"object","properties":{"tenant":{"type":"object","x-mcp-header":"Tenant"}}},"unknown":"retain"}"#;
    let tools = catalog::validate_tools(vec![RawValue::from_string(valid.into()).unwrap(), RawValue::from_string(rejected.into()).unwrap()], false).unwrap();
    let canonical = tools.iter().find(|tool| tool.name == "valid").unwrap();
    assert_eq!(canonical.definition.get(), valid);
    assert_eq!(tools.iter().find(|tool| tool.name == "bad-header").unwrap().definition.get(), rejected);
    let modern = catalog::origin("disposable", &tools, MODERN);
    assert_eq!(modern["status"], "partial");
    assert_eq!(modern["tools"].as_array().unwrap().len(), 1);
    assert_eq!(modern["tools"][0]["name"], "valid");
    assert_eq!(modern["tools"][0]["future"]["number"].to_string(), "184467440737095516160000000001");
    assert_eq!(modern["rejected_tools"][0]["name"], "bad-header");
    let legacy = catalog::origin("disposable", &tools, LEGACY);
    assert_eq!(legacy["status"], "available");
    assert_eq!(legacy["tools"].as_array().unwrap().len(), 2);

    let duplicates = vec![raw::raw(&json!({"name":"same","future":1})), raw::raw(&json!({"name":"same","future":2}))];
    assert_eq!(catalog::validate_tools(duplicates, false).err().unwrap().code, "TOOLS_UNAVAILABLE");
}

#[tokio::test]
async fn aliases_survive_restart_and_never_mix_servers_or_reserved_names() {
    let directory = tempfile::tempdir().unwrap();
    let runner = Runner::persistent(directory.path().to_path_buf()).unwrap();
    let first = runner.alias("first-server", "fetch__get_servers").await.unwrap();
    let second = runner.alias("second-server", "fetch__get_servers").await.unwrap();
    let exceptional = runner.alias("first-server", "工具 / punctuation?{}").await.unwrap();
    assert_ne!(first, second);
    assert!(!first.starts_with("fetch__"));
    assert!(exceptional.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'));
    drop(runner);
    let restarted = Runner::persistent(directory.path().to_path_buf()).unwrap();
    assert_eq!(restarted.alias("first-server", "fetch__get_servers").await.unwrap(), first);
    assert_eq!(restarted.resolve_alias(&second).await, Some(("second-server".into(), "fetch__get_servers".into())));
    assert_eq!(restarted.resolve_alias(&exceptional).await, Some(("first-server".into(), "工具 / punctuation?{}".into())));
}

#[test]
fn qualified_resource_templates_preserve_original_bytes_and_origin() {
    let original = "custom://path/%2F/{identifier}{?query*}?literal=+%2520#section";
    let qualified = catalog::qualified_uri("first-server", original);
    assert_eq!(catalog::unqualify_uri("first-server", &qualified), Some(original));
    assert_eq!(catalog::unqualify_uri("second-server", &qualified), None);
    assert_ne!(qualified, catalog::qualified_uri("second-server", original));
}

#[tokio::test]
async fn reading_or_calling_a_stopped_server_never_starts_its_executable() {
    let runner = Runner::new();
    runner.configure(ServerRoute {
        server_id: "disposable".into(),
        transport: TransportConfig::Stdio {
            executable: PathBuf::from("/this-disposable-executable-does-not-exist"),
            args: vec![], working_directory: None, environment: BTreeMap::new(),
        },
        execution: ExecutionPolicy::default(),
    }).await.unwrap();
    assert_eq!(runner.snapshot("disposable").await["runtime"]["status"], "stopped");
    assert!(runner.cached_catalog("disposable", "raw").await.is_err());
    let error = runner.call("disposable", "echo", raw::raw(&json!({"message":"marker"})),
        InvocationContext::terminal("disposable-owner".into())).await.err().unwrap();
    assert_eq!(error.code, "SERVER_NOT_READY");
    assert_eq!(error.execution, "not_started");
    let state = runner.snapshot("disposable").await;
    assert_eq!(state["runtime"]["status"], "stopped");
    assert_eq!(state["tool_execution"]["active_calls"], 0);
    assert_eq!(state["tool_execution"]["queued_calls"], 0);
    runner.shutdown().await;
}
