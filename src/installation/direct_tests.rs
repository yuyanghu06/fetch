//! Pure direct-input mapping and supervised preparation-hook fixtures.
//! No runtime, executable, provider, credentials or managed connection runs.
use super::*;
use tokio::sync::{Semaphore, mpsc};

#[test]
fn npx_and_uvx_commands_share_imported_package_capture_without_installing_packages() {
    let npm = direct::command("npm", "npx -y @example/mcp-server@1.2.3").unwrap();
    let source = crate::discovery::capture_definition("npm", &json!({"command":"npx","args":["-y","@example/mcp-server@1.2.3"]}), "claude_code").unwrap();
    assert_eq!(npm.capture.row, source.row);
    assert_eq!(npm.capture.config, source.config);
    assert_eq!(npm.capture.row["type"], "npm");
    assert_eq!(npm.capture.row["package"], "@example/mcp-server");
    assert_eq!(npm.capture.row["version_spec"], "1.2.3");
    let python = direct::command("time", "uvx --from mcp-server-time==2026.10.19 mcp-server-time --local-timezone=America/New_York").unwrap();
    assert_eq!(python.capture.row["type"], "pypi");
    assert_eq!(python.capture.row["package"], "mcp-server-time");
    assert_eq!(python.capture.row["version_spec"], "2026.10.19");
    assert_eq!(python.capture.package_arguments, vec!["--local-timezone=America/New_York"]);
}

#[test]
fn quoted_script_arguments_and_equivalent_command_spellings_keep_exact_words() {
    let first = direct::command("script", "node '/private/fixture folder/server.js'").unwrap();
    let second = direct::command("script", "  node  \"/private/fixture folder/server.js\"  ").unwrap();
    assert_eq!(first.identity, second.identity);
    assert_eq!(first.capture.config, second.capture.config);
    assert_eq!(first.capture.row["type"], "local_script");
    assert_eq!(first.capture.config["args"], json!(["/private/fixture folder/server.js"]));
    assert_ne!(first.identity, direct::command("another-name", "node '/private/fixture folder/server.js'").unwrap().identity);
    assert_ne!(first.identity, direct::command("script", "node '/private/another folder/server.js'").unwrap().identity);
}

#[test]
fn direct_bridges_use_native_remote_capture_and_reject_embedded_credentials() {
    let bridge = direct::command("bridge", "npx -y mcp-remote@0.1.38 https://v0.app/api/mcp").unwrap();
    assert_eq!(bridge.capture.config["kind"], "remote");
    assert_eq!(bridge.capture.config["connection"]["url"], "https://v0.app/api/mcp");
    assert_eq!(bridge.capture.config["connection"]["transport"], "streamable_http");
    assert!(bridge.capture.config["executable"].is_null());
    assert_eq!(bridge.capture.config["args"], json!([]));
    assert!(bridge.capture.package_arguments.is_empty());
    assert!(direct::command("bridge", "npx mcp-remote https://v0.app/api/mcp --header 'Authorization:Bearer fixture-secret'").is_err());
    assert!(direct::command("key", "API_KEY=fixture-secret npx @example/mcp-server").is_err());
    assert!(direct::command("key", "npx @example/mcp-server --token fixture-secret").is_err());
}

#[test]
fn shell_operators_comments_expansions_and_unsupported_launchers_are_not_executed_or_reinterpreted() {
    for command in [
        "npx pkg ; echo marker", "npx pkg && echo marker", "npx pkg | cat", "npx pkg > /private/output",
        "npx pkg $(echo marker)", "npx pkg `echo marker`", "npx pkg $API_KEY", "npx pkg # ignored",
        "npx pkg\necho marker", "npx pkg 'unclosed", "npx --package pkg server", "sh -c 'npx pkg'",
    ] { assert!(direct::command("unsupported", command).is_err(), "{command}"); }
}

#[test]
fn url_capture_normalizes_identity_and_keeps_command_url_and_account_names_separate() {
    let first = direct::url("remote", "https://MCP.MIRO.COM").unwrap();
    let normalized = direct::url("remote", "https://mcp.miro.com/").unwrap();
    assert_eq!(first.identity, normalized.identity);
    assert_eq!(first.capture.config["connection"]["url"], "https://mcp.miro.com/");
    assert_ne!(first.identity, direct::url("another-account", "https://mcp.miro.com/").unwrap().identity);
    assert_ne!(first.identity, direct::command("remote", "npx mcp-remote https://mcp.miro.com/").unwrap().identity);
    assert_eq!(first.identity.len(), 64);
    assert!(first.identity.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    for endpoint in ["https://user:fixture-secret@mcp.miro.com/", "https://mcp.miro.com/?token=fixture-secret", "https://example.com/private/fixture-secret"] {
        assert!(direct::url("remote", endpoint).is_err());
    }
}

async fn finished(installer: &Arc<Installer>, id: &str) -> Value {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let status = installer.progress(id).await.unwrap();
            if status["status"] != "installing" { return status; }
            tokio::task::yield_now().await;
        }
    }).await.unwrap()
}

#[tokio::test]
async fn connection_hooks_are_sequential_and_failure_state_does_not_undo_preparation() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("state");
    let installer = Installer::new(root.clone(), DiscoveryService::new(directory.path().join("empty-home"), None));
    let repository = ServerRepository::new(root.clone());
    let gate = Arc::new(Semaphore::new(0));
    let (started, mut events) = mpsc::unbounded_channel();
    let hook_gate = gate.clone();
    let hook_repository = repository.clone();
    let hook: PreparedHook = Arc::new(move |id| {
        let gate = hook_gate.clone();
        let repository = hook_repository.clone();
        let started = started.clone();
        Box::pin(async move {
            let mut record = repository.get_saved(&id).unwrap().unwrap();
            // Simulate Connect reporting its distinct runtime/authentication outcome.
            record["runtime"] = json!({"status":"failed","error":{"code":"SERVER_AUTH_REQUIRED","message":"Disposable fixture needs login."}});
            record["authentication"] = json!({"status":"required","error":null});
            repository.save(&id, &record).unwrap();
            started.send(id).unwrap();
            gate.acquire().await.unwrap().forget();
        })
    });
    let accepted = installer.submit_with_connect(json!({"servers":[
        {"server_name":"first","url":"https://mcp.miro.com/"},
        {"server_name":"second","command":"npx mcp-remote https://v0.app/api/mcp"}
    ]}), hook).await.unwrap();
    let installation = accepted["installation_id"].as_str().unwrap();
    let first = tokio::time::timeout(Duration::from_secs(3), events.recv()).await.unwrap().unwrap();
    let progress = installer.progress(installation).await.unwrap();
    assert_eq!(progress["status"], "installing");
    assert_eq!(progress["servers"][0]["server_id"], first);
    assert_eq!(progress["servers"][0]["status"], "installing");
    assert_eq!(progress["servers"][1]["status"], "pending");
    assert_eq!(repository.list_saved().unwrap().len(), 1);
    let conflict = installer.submit(json!({"servers":[{"server_name":"third","url":"https://mcp.miro.com/"}]})).await.unwrap_err();
    assert_eq!((conflict.http_status, conflict.code), (409, "INSTALLATION_IN_PROGRESS"));
    assert_eq!(conflict.installation_id.as_deref(), Some(installation));
    gate.add_permits(1);
    let second = tokio::time::timeout(Duration::from_secs(3), events.recv()).await.unwrap().unwrap();
    assert_ne!(first, second);
    let progress = installer.progress(installation).await.unwrap();
    assert_eq!(progress["servers"][0]["status"], "succeeded");
    assert_eq!(progress["servers"][1]["status"], "installing");
    gate.add_permits(1);
    let progress = finished(&installer, installation).await;
    assert_eq!(progress["status"], "succeeded");
    for server in [&first, &second] {
        let saved = repository.get_saved(server).unwrap().unwrap();
        assert_eq!(saved["runtime"]["status"], "failed");
        assert_eq!(saved["authentication"]["status"], "required");
    }
    assert!(!root.join("packages").exists());
    installer.shutdown();
}
