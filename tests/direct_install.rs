//! Direct-input installation fixtures. No package, process, provider, Keychain or MCP runs.
use axum::{Router, body::Body, http::{HeaderMap, Request, StatusCode, header}, response::Response};
use fetch_local::{
    api::{AppState, router},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    installation::Installer,
    manager::Manager,
    remote_oauth::GrantStore,
    runner::Runner,
    storage::ServerRepository,
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{path::{Path, PathBuf}, sync::Arc, time::Duration};
use tempfile::TempDir;
use tower::ServiceExt;

struct FixtureAuthority;
impl Authorizer for FixtureAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        match headers.get(header::AUTHORIZATION).and_then(|value| value.to_str().ok()) {
            Some("Bearer fixture-manage") => Ok(Caller::from_verified_scopes(["fetch:manage".into()])),
            Some("Bearer fixture-catalog") => Ok(Caller::from_verified_scopes(["fetch:catalog".into()])),
            _ => Err(AuthorizationError::Unauthenticated),
        }
    }
}

struct Fixture {
    _directory: TempDir,
    root: PathBuf,
    source: PathBuf,
    installer: Arc<Installer>,
    repository: ServerRepository,
    app: Router,
}
impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("state");
        let source = directory.path().join("source.json");
        let discovery = DiscoveryService::new(directory.path().join("empty-home"), None);
        let repository = ServerRepository::new(root.clone());
        let installer = Installer::new(root.clone(), discovery.clone());
        let manager = Manager::with_grant_store(repository.clone(), Arc::new(GrantStore::memory(root.clone())), Runner::new(), installer.clone());
        let mut state = AppState::new("https://localhost:53672".into(), repository.clone(), discovery, Arc::new(FixtureAuthority)).unwrap();
        state.manager = Some(manager);
        Self { _directory:directory, root, source, installer, repository, app:router(Arc::new(state)) }
    }

    fn write_source(&self, endpoint: &str) {
        std::fs::write(&self.source, serde_json::to_vec(&json!({"mcpServers":{"same":{"url":endpoint}}})).unwrap()).unwrap();
    }

    async fn prepare(&self, selections: Value) -> Value {
        let accepted = self.installer.submit(json!({"servers":selections})).await.unwrap();
        assert_eq!(accepted["status"], "installing");
        assert!(accepted["next_steps"].is_array());
        completed(&self.installer, accepted["installation_id"].as_str().unwrap()).await
    }

    async fn request(&self, method: &str, uri: &str, token: Option<&str>, body: Value) -> (StatusCode, HeaderMap, Value) {
        let mut request = Request::builder().method(method).uri(uri).header(header::HOST, "localhost:53672")
            .header(header::CONTENT_TYPE, "application/json");
        if let Some(token) = token { request = request.header(header::AUTHORIZATION, token); }
        let body = if method == "GET" { Body::empty() } else { Body::from(serde_json::to_vec(&body).unwrap()) };
        decode(self.app.clone().oneshot(request.body(body).unwrap()).await.unwrap()).await
    }
}

async fn completed(installer: &Arc<Installer>, id: &str) -> Value {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let value = installer.progress(id).await.unwrap();
            if value["status"] != "installing" { return value; }
            tokio::task::yield_now().await;
        }
    }).await.expect("disposable preparation must finish without a download or process")
}

async fn decode(response: Response) -> (StatusCode, HeaderMap, Value) {
    let status = response.status();
    let headers = response.headers().clone();
    assert_eq!(headers[header::CACHE_CONTROL], "no-store");
    let value: Value = serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert!(value["next_steps"].is_array());
    (status, headers, value)
}

fn id(progress: &Value, index: usize) -> &str { progress["servers"][index]["server_id"].as_str().unwrap() }

#[tokio::test]
async fn malformed_or_duplicate_selections_fail_before_job_or_state_creation() {
    let fixture = Fixture::new();
    for body in [
        json!({}), json!({"servers":[]}), json!({"servers":[],"extra":true}),
        json!({"servers":[{"server_name":"same","command":"npx pkg","url":"https://mcp.miro.com/"}]}),
        json!({"servers":[{"server_name":"same","config_path":fixture.source,"command":"npx pkg"}]}),
        json!({"servers":[{"server_name":"same"}]}),
        json!({"servers":[{"command":"npx pkg"}]}),
        json!({"servers":[{"server_name":"same","command":false}]}),
        json!({"servers":[{"server_name":"same","url":null}]}),
        json!({"servers":[{"server_name":" ","command":"npx pkg"}]}),
        json!({"servers":[{"server_name":"same","command":" \t "}]}),
        json!({"servers":[{"server_name":"same","command":"npx\u{0000}pkg"}]}),
        json!({"servers":[{"server_name":"same","config_path":"relative.json"}]}),
        json!({"servers":[{"server_name":"same","command":"npx pkg"},{"server_name":"same","command":"npx pkg"}]}),
        json!({"servers":[{"server_name":"same","url":"https://mcp.miro.com/"},{"server_name":"same","url":"https://mcp.miro.com/"}]}),
    ] {
        let error = fixture.installer.submit(body).await.unwrap_err();
        assert_eq!((error.http_status, error.code), (400, "INVALID_REQUEST"));
        assert!(error.installation_id.is_none());
        assert!(!fixture.root.exists());
    }
}

#[tokio::test]
async fn figma_remote_direct_url_does_not_create_an_installed_server() {
    let fixture = Fixture::new();
    // Submit directly to preparation: this uses the product URL parser but has
    // no connection hook, so even the pre-fix behavior cannot contact Figma.
    let result = fixture.prepare(json!([
        {"server_name":"figma-remote","url":"https://mcp.figma.com/mcp"}
    ])).await;
    assert_eq!(result["status"], "failed");
    assert_eq!(result["servers"][0]["status"], "failed");
    assert_eq!(result["servers"][0]["error"]["code"], "INSTALLATION_FAILED");
    assert!(result["servers"][0]["server_id"].is_null());
    assert!(fixture.repository.list_saved().unwrap().is_empty());
    assert!(!fixture.root.join("packages").exists());
}

#[tokio::test]
async fn unsupported_items_fail_safely_and_later_url_preparation_continues() {
    let fixture = Fixture::new();
    let result = fixture.prepare(json!([
        {"server_name":"bad-command","command":"npx pkg ; echo fixture-command-marker"},
        {"server_name":"bad-url","url":"https://user:fixture-secret-marker@mcp.miro.com/"},
        {"server_name":"good","url":"https://mcp.miro.com/"}
    ])).await;
    assert_eq!(result["status"], "failed");
    assert_eq!(result["servers"][0]["error"]["code"], "INSTALLATION_FAILED");
    assert_eq!(result["servers"][1]["error"]["code"], "INSTALLATION_FAILED");
    assert_eq!(result["servers"][2]["status"], "succeeded");
    assert_eq!(fixture.repository.list_saved().unwrap().len(), 1);
    for row in result["servers"].as_array().unwrap() { assert!(row["config_path"].is_null()); }
    for hidden in ["fixture-command-marker", "fixture-secret-marker", "https://mcp.miro.com/", "npx pkg"] {
        assert!(!result.to_string().contains(hidden));
    }
    assert!(!fixture.root.join("packages").exists());
}

#[tokio::test]
async fn url_identity_reuse_preserves_saved_settings_and_untouched_legacy_bytes() {
    let fixture = Fixture::new();
    let first = fixture.prepare(json!([{"server_name":"remote","url":"https://mcp.miro.com/"}])).await;
    let server = id(&first, 0).to_owned();
    let mut saved = fixture.repository.get_saved(&server).unwrap().unwrap();
    saved["config"]["execution"]["default_timeout_ms"] = json!(240_000);
    saved["config"]["connection"]["headers"] = json!([{"name":"Authorization","credential_ref":"my-key"}]);
    saved["credentials"] = json!([{"name":"my-key","status":"stored"}]);
    fixture.repository.save(&server, &saved).unwrap();
    // Old ciphertext is deliberately opaque. New code must not read, decrypt,
    // migrate or remove it while reusing an existing installation.
    let legacy = fixture.root.join("servers").join(format!("{:x}", Sha256::digest(server.as_bytes()))).join("credentials.enc");
    let secret_before = b"opaque-disposable-legacy-state\0\xff";
    std::fs::write(&legacy, secret_before).unwrap();
    let second = fixture.prepare(json!([{"server_name":"remote","url":"https://mcp.miro.com/"}])).await;
    assert_eq!(id(&second, 0), server);
    assert_eq!(fixture.repository.get_saved(&server).unwrap().unwrap(), saved);
    assert_eq!(std::fs::read(&legacy).unwrap(), secret_before);
    assert!(!second.to_string().contains("opaque-disposable-legacy-state"));
    assert!(!fixture.root.join("packages").exists());
}

#[tokio::test]
async fn direct_name_or_definition_changes_create_separate_instances() {
    let fixture = Fixture::new();
    let result = fixture.prepare(json!([
        {"server_name":"first","url":"https://mcp.miro.com/"},
        {"server_name":"second","url":"https://mcp.miro.com/"},
        {"server_name":"first","url":"https://v0.app/api/mcp"}
    ])).await;
    assert_eq!(result["status"], "succeeded");
    assert_ne!(id(&result, 0), id(&result, 1));
    assert_ne!(id(&result, 0), id(&result, 2));
    assert_eq!(fixture.repository.list_saved().unwrap().len(), 3);
}

#[tokio::test]
async fn equivalent_quoted_commands_reuse_without_running_the_fixture_executable() {
    let fixture = Fixture::new();
    let script = fixture.source.with_file_name("fixture program");
    std::fs::write(&script, "this fixture is data and must never execute").unwrap();
    let result = fixture.prepare(json!([
        {"server_name":"same","command":format!("'{}'",script.display())},
        {"server_name":"same","command":format!("  \"{}\"  ",script.display())}
    ])).await;
    assert_eq!(result["status"], "succeeded");
    assert_eq!(id(&result, 0), id(&result, 1));
    let saved = fixture.repository.get_saved(id(&result, 0)).unwrap().unwrap();
    assert_eq!(saved["config"]["kind"], "local_script");
    assert_eq!(saved["config"]["executable"], script.to_string_lossy().as_ref());
    assert_eq!(saved["config"]["args"], json!([]));
    assert_eq!(std::fs::read_to_string(&script).unwrap(), "this fixture is data and must never execute");
    assert_eq!(fixture.repository.list_saved().unwrap().len(), 1);
}

#[tokio::test]
async fn config_url_and_bridge_command_have_distinct_installation_identities() {
    let fixture = Fixture::new();
    fixture.write_source("https://v0.app/api/mcp");
    let source_before = std::fs::read(&fixture.source).unwrap();
    let result = fixture.prepare(json!([
        {"server_name":"same","config_path":fixture.source},
        {"server_name":"same","url":"https://v0.app/api/mcp"},
        {"server_name":"same","command":"npx mcp-remote https://v0.app/api/mcp"}
    ])).await;
    assert_eq!(result["status"], "succeeded");
    let ids: std::collections::BTreeSet<_> = (0..3).map(|index| id(&result, index)).collect();
    assert_eq!(ids.len(), 3);
    assert_eq!(result["servers"][0]["config_path"], fixture.source.to_string_lossy().as_ref());
    assert!(result["servers"][1]["config_path"].is_null());
    assert!(result["servers"][2]["config_path"].is_null());
    assert_eq!(std::fs::read(&fixture.source).unwrap(), source_before);
    for index in 0..3 {
        let saved = fixture.repository.get_saved(id(&result, index)).unwrap().unwrap();
        assert_eq!(saved["config"]["kind"], "remote");
        assert_eq!(saved["config"]["connection"]["url"], "https://v0.app/api/mcp");
    }
    assert!(!fixture.root.join("packages").exists());
}

#[tokio::test]
async fn public_route_preserves_authority_and_invalid_body_contract() {
    let fixture = Fixture::new();
    let valid = json!({"servers":[{"server_name":"direct","command":"unsupported-fixture"}]});
    for (token, expected) in [(None, StatusCode::UNAUTHORIZED), (Some("Bearer fixture-catalog"), StatusCode::FORBIDDEN)] {
        let (status, _, _) = fixture.request("POST", "/api/v1/setup/installations", token, valid.clone()).await;
        assert_eq!(status, expected);
        assert!(!fixture.root.exists());
    }
    for uri in ["/api/v1/setup/installations", "/api/v1/setup/installations?unexpected=true"] {
        let (status, _, value) = fixture.request("POST", uri, Some("Bearer fixture-manage"), json!({"servers":[{
            "server_name":"direct","command":"unsupported-fixture","config_path":fixture.source
        }]})).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(value["error"]["code"], "INVALID_REQUEST");
        assert!(!fixture.root.exists());
    }
}

#[tokio::test]
async fn public_route_accepts_unsupported_item_then_reports_safe_current_run_failure() {
    let fixture = Fixture::new();
    let (status, headers, accepted) = fixture.request("POST", "/api/v1/setup/installations", Some("Bearer fixture-manage"), json!({"servers":[{
        "server_name":"direct","command":"unsupported-fixture fixture-private-command-marker"
    }]})).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let installation = accepted["installation_id"].as_str().unwrap();
    let location = format!("/api/v1/setup/installations/{installation}");
    assert_eq!(headers[header::LOCATION].to_str().unwrap(), location);
    completed(&fixture.installer, installation).await;
    let (status, _, progress) = fixture.request("GET", &location, Some("Bearer fixture-manage"), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(progress["status"], "failed");
    assert_eq!(progress["servers"][0]["error"]["code"], "INSTALLATION_FAILED");
    assert!(!progress.to_string().contains("fixture-private-command-marker"));
    let fresh = Installer::new(fixture.root.clone(), DiscoveryService::new(Path::new("/not-read-by-progress").into(), None));
    assert_eq!(fresh.progress(installation).await.unwrap_err().code, "INSTALLATION_NOT_FOUND");
}

#[tokio::test]
async fn public_configuration_edits_only_fetch_call_limits_and_never_native_launch_fields() {
    let fixture=Fixture::new();
    let prepared=fixture.prepare(json!([{"server_name":"remote","url":"https://mcp.miro.com/"}])).await;
    let server=id(&prepared,0);let uri=format!("/api/v1/servers/{server}/config");
    let before=fixture.repository.get_saved(server).unwrap().unwrap();
    for patch in [json!({"args":["changed"]}),json!({"working_directory":"/disposable/changed"}),
        json!({"environment":[]}),json!({"connection":{"headers":[]}}),
        json!({"execution":{"default_timeout_ms":45000},"args":[]})] {
        let (status,_,body)=fixture.request("PATCH",&uri,Some("Bearer fixture-manage"),patch).await;
        assert_eq!(status,StatusCode::BAD_REQUEST,"{body}");
        assert_eq!(fixture.repository.get_saved(server).unwrap().unwrap(),before,"rejected changes are atomic");
    }
    let (status,_,body)=fixture.request("PATCH",&uri,Some("Bearer fixture-manage"),json!({"execution":{"default_timeout_ms":45000,"max_concurrent_calls":3}})).await;
    assert_eq!(status,StatusCode::OK,"{body}");
    let after=fixture.repository.get_saved(server).unwrap().unwrap();
    assert_eq!(after["config"]["execution"]["default_timeout_ms"],45000);
    assert_eq!(after["config"]["execution"]["max_concurrent_calls"],3);
    for key in ["args","working_directory","environment","connection","executable"] {assert_eq!(after["config"][key],before["config"][key]);}
    let (status,_,_)=fixture.request("PATCH",&uri,Some("Bearer fixture-catalog"),json!({"execution":{"default_timeout_ms":90000}})).await;
    assert_eq!(status,StatusCode::FORBIDDEN);
    assert_eq!(fixture.repository.get_saved(server).unwrap().unwrap(),after);
}

#[tokio::test]
async fn server_status_reads_do_not_open_or_verify_native_login_or_expose_old_credential_fields() {
    let fixture=Fixture::new();
    let prepared=fixture.prepare(json!([{"server_name":"remote","url":"https://mcp.miro.com/"}])).await;
    let server=id(&prepared,0);let before=fixture.repository.get_saved(server).unwrap().unwrap();
    for _ in 0..3 {
        let (status,_,body)=fixture.request("GET",&format!("/api/v1/servers?id={server}"),Some("Bearer fixture-manage"),Value::Null).await;
        assert_eq!(status,StatusCode::OK);
        let row=&body["servers"][0];
        assert_eq!(row["runtime"]["status"],"stopped");
        assert_eq!(row["authentication"]["status"],"unknown");
        assert!(row["authentication"]["access_expires_at"].is_null());
        assert!(row["authentication"]["last_verified_at"].is_null());
        assert!(row.get("credentials").is_none());
        assert_eq!(fixture.repository.get_saved(server).unwrap().unwrap(),before);
    }
}
