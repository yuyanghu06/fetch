//! Explicit real-package diagnostic, never part of the normal fixture suite.
//! Run only inside verified OS confinement; this file does not establish it.
//! The key provider below is a fixture, so this is not native/Keychain acceptance.
//! Select exactly one ignored test per process and give it a fresh demo root.
use fetch_local::{
    authority::Caller,
    credentials::{CredentialError, CredentialStore, KeyProvider},
    discovery::DiscoveryService,
    installation::{Installer, NODE_VERSION},
    manager::Manager,
    runner::{InvocationContext, Runner},
    storage::ServerRepository,
};
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}, sync::{Arc, OnceLock}, time::{Duration, Instant}};
use zeroize::Zeroizing;

const MARKER: &str = "isolated Fetch delete latency demo\n";
static DELETE_START: OnceLock<Instant> = OnceLock::new();

struct DemoKeys;
impl KeyProvider for DemoKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([69; 32]))
    }
    fn delete_key_for_server(&self, _: &str) -> Result<(), CredentialError> {
        event("package_cleanup_finished_before_fixture_key_delete",json!({
            "elapsed_ms":DELETE_START.get().map(|start|start.elapsed().as_millis()),"keychain_used":false
        }));
        Ok(())
    }
}

fn event(stage: &str, details: Value) {
    println!("DELETE_DEMO {}", json!({"stage":stage,"details":details}));
}

fn demo_root() -> Result<PathBuf, String> {
    let supplied = std::env::var_os("FETCH_DELETE_DEMO_ROOT")
        .ok_or("Set FETCH_DELETE_DEMO_ROOT only for an isolated explicit run")?;
    let path = PathBuf::from(supplied);
    if !path.is_absolute() { return Err("Demo root must be absolute".into()); }
    let root = fs::canonicalize(&path).map_err(|e|e.to_string())?;
    if root.parent() != Some(Path::new("/private/tmp")) && root.parent() != Some(Path::new("/tmp")) {
        return Err("Demo root must be directly under /private/tmp or /tmp".into());
    }
    if !root.file_name().and_then(|n|n.to_str()).is_some_and(|n|n.starts_with("fetch-delete-demo-")) {
        return Err("Demo root must be named fetch-delete-demo-*".into());
    }
    if fs::read_to_string(root.join(".fetch-delete-demo")).map_err(|e|e.to_string())? != MARKER {
        return Err("Explicit demo marker is missing or incorrect".into());
    }
    if root.join("state/servers").exists() || root.join("demo-config.toml").exists() {
        return Err("Demo root already contains a prior run; use a fresh root".into());
    }
    Ok(root)
}

// Do not follow package symlinks. During sampling, disappearance is expected.
fn entries(path: &Path) -> (u64, u64) {
    let Ok(items) = fs::read_dir(path) else { return (0,0); };
    let (mut files, mut folders) = (0,0);
    for item in items.flatten() {
        if item.file_type().is_ok_and(|kind|kind.is_dir()) {
            folders += 1;
            let (f,d) = entries(&item.path()); files += f; folders += d;
        } else { files += 1; }
    }
    (files, folders)
}

async fn run_demo(root: &Path, manager: &Arc<Manager>, package_spec: &str, connect_and_echo: bool) -> Result<(), String> {
    let source = root.join("demo-config.toml");
    let config = format!("[mcp_servers.delete_demo]\ncommand = \"npx\"\nargs = [\"-y\", \"{package_spec}\"{}]\n",if connect_and_echo{", \"stdio\""}else{""});
    fs::write(&source, &config).map_err(|e|e.to_string())?;
    let sentinel = root.join("unrelated-preserve.txt");
    fs::write(&sentinel, "preserve this disposable canary\n").map_err(|e|e.to_string())?;
    let runtime = root.join("state/packages/runtimes/node").join(NODE_VERSION).join("bin/node");
    if !runtime.is_file() { return Err("Stage the public pinned Node/npm runtime before this run".into()); }
    let begin = Instant::now();
    let input = json!({"servers":[{"config_path":source,"server_name":"delete_demo"}]});
    let accepted = if connect_and_echo {
        manager.install(input).await.map_err(|e|format!("install {}: {}", e.code, e.message))?
    } else {
        // Google preparation deliberately does not use Manager::install: its
        // automatic Connect hook would start the provider's MCP process.
        manager.installer.submit(input).await.map_err(|e|format!("prepare {}: {}", e.code, e.message))?
    };
    let installation = accepted["installation_id"].as_str().ok_or("No installation ID")?;
    event("installation_accepted", json!({"installation_id":installation,"package_spec":package_spec,"connect_requested":connect_and_echo}));
    let progress = tokio::time::timeout(Duration::from_secs(180), async {
        loop {
            let progress = manager.installation(installation).await.map_err(|e|format!("progress {}",e.code))?;
            if progress["status"] != "installing" { return Ok::<_,String>(progress); }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    }).await.map_err(|_|"Installation/connect exceeded 180 seconds")??;
    if progress["status"] != "succeeded" { return Err(format!("Installation failed: {progress}")); }
    let id = progress["servers"][0]["server_id"].as_str().ok_or("No server ID")?;
    let tools_count = if connect_and_echo {
        let snapshot = manager.runner.snapshot(id).await;
        if snapshot["runtime"]["status"] != "ready" { return Err(format!("Installed but not ready: {snapshot}")); }
        let catalog = manager.runner.catalog(id,"raw").await.map_err(|e|format!("catalog {}",e.code))?;
        let tools = catalog["tools"].as_array().filter(|v|!v.is_empty()).ok_or("Empty tool catalog")?;
        if !tools.iter().any(|t|t["name"] == "echo") { return Err("Expected Everything echo tool".into()); }
        let reply = manager.runner.call(id,"echo",serde_json::value::to_raw_value(&json!({"message":"disposable-delete-demo"})).unwrap(),
            InvocationContext::terminal("isolated-delete-demo".into())).await.map_err(|e|format!("echo {}",e.code))?;
        let reply: Value = serde_json::from_str(reply.get()).map_err(|e|e.to_string())?;
        if reply["isError"] == true || !reply.to_string().contains("disposable-delete-demo") { return Err("Echo did not return the disposable marker".into()); }
        Some(tools.len())
    } else {
        if !manager.runner.server_ids().await.is_empty() { return Err("Preparation unexpectedly configured a running server".into()); }
        None
    };
    let record = manager.repository.get_saved(id).map_err(|e|e.to_string())?.ok_or("No installed record")?;
    let package_id = record["_installation"]["installation_id"].as_str().ok_or("No package ID")?;
    let package = root.join("state/packages/servers").join(package_id);
    let (files, folders) = entries(&package);
    event(if connect_and_echo{"installed_and_echo_verified"}else{"installation_prepared"},json!({"elapsed_ms":begin.elapsed().as_millis(),"server_id":id,
        "package_version":record["config"]["package"]["installed_version"],"tools":tools_count,"files":files,"folders":folders,
        "connect_verified":connect_and_echo,"echo_verified":connect_and_echo}));
    let started = Instant::now();
    DELETE_START.set(started).map_err(|_|"Only one delete demo may run per process")?;
    event("delete_started",json!({"files":files,"folders":folders}));
    let sampled_package = package.clone();
    let sampler = tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let sample_path = sampled_package.clone();
            let (files,folders) = tokio::task::spawn_blocking(move||entries(&sample_path)).await.unwrap_or((0,0));
            event("deleting",json!({"elapsed_ms":started.elapsed().as_millis(),"remaining_files":files,"remaining_folders":folders,
                "package_exists":sampled_package.exists()}));
        }
    });
    let caller = Caller::from_verified_scopes(["fetch:manage".to_owned()]);
    let deleted = tokio::time::timeout(Duration::from_secs(240),manager.delete_server(id,&caller)).await;
    sampler.abort();
    let deleted = deleted.map_err(|_|"Deletion exceeded 240 seconds; its owned worker may still be cleaning up")?
        .map_err(|e|format!("delete {}: {}",e.code,e.message))?;
    event("delete_completed",json!({"elapsed_ms":started.elapsed().as_millis(),"response":deleted}));
    if deleted["status"] != "deleted" || package.exists() || manager.repository.get_saved(id).map_err(|e|e.to_string())?.is_some() {
        return Err("Deletion did not remove package and saved record".into());
    }
    if fs::read_to_string(&source).map_err(|e|e.to_string())? != config
        || fs::read_to_string(&sentinel).map_err(|e|e.to_string())? != "preserve this disposable canary\n"
        || !runtime.is_file() || !manager.runner.server_ids().await.is_empty() {
        return Err("Preservation or runner removal check failed".into());
    }
    event("preservation_verified",json!({"source_unchanged":true,"unrelated_file_unchanged":true,"runtime_preserved":true,
        "package_removed":true,"saved_record_removed":true,"runner_removed":true,"keychain_used":false}));
    Ok(())
}

async fn isolated_case(package_spec: &str, connect_and_echo: bool) {
    let root = demo_root().expect("safe explicit demo root");
    let state = root.join("state");
    let discovery = DiscoveryService::new(root.join("empty-home"),None);
    let manager = Manager::new(ServerRepository::new(state.clone()),
        Arc::new(CredentialStore::new(state.clone(),Arc::new(DemoKeys))),Runner::new(),Installer::new(state,discovery));
    let result = run_demo(&root,&manager,package_spec,connect_and_echo).await;
    let shutdown = tokio::time::timeout(Duration::from_secs(12),manager.shutdown()).await;
    assert!(shutdown.is_ok(),"Demo shutdown exceeded its bound; stop only this test's owned process group");
    result.expect("isolated real MCP delete demo");
}

#[tokio::test(flavor="multi_thread", worker_threads=4)]
#[ignore = "Real npm install and MCP execution: requires explicit disposable root and verified OS confinement"]
async fn real_install_echo_and_delete_latency_demo() {
    isolated_case("@modelcontextprotocol/server-everything@2026.8.31",true).await;
}

#[tokio::test(flavor="multi_thread", worker_threads=4)]
#[ignore = "Real Google package preparation/deletion only: requires explicit disposable root and verified OS confinement"]
async fn google_package_prepare_and_delete_latency_demo() {
    isolated_case("@a-bonus/google-docs-mcp@1.11.3",false).await;
}
