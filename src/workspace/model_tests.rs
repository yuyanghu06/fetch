//! Catalog checks use disposable fake programs, never an installed coding app.
use super::*;
use std::fs;
use tempfile::TempDir;

fn fake_command(script: &str) -> (TempDir, Command) {
    let temp = TempDir::new().unwrap();
    let script_path = temp.path().join("fake_catalog.py");
    fs::write(&script_path, script).unwrap();
    let mut command = Command::new("python3");
    command.arg(script_path).arg(temp.path().join("requests.jsonl"));
    command.current_dir(temp.path());
    // Fixtures do not inherit personal credentials, app settings or MCP setup.
    let path = std::env::var_os("PATH").unwrap_or_default();
    command.env_clear().env("PATH", path).env("HOME", temp.path());
    (temp, command)
}

#[tokio::test]
async fn codex_picker_reads_updated_saved_models_without_private_cache_metadata() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("models_cache.json");
    let cache = json!({
        "identity":"fixture-private-identity", "etag":"fixture-private-etag", "account":"fixture-private-account",
        "fetched_at":"2026-11-12T19:00:00Z", "client_version":"fixture-cli",
        "models":[
            {"slug":"fixture-one","display_name":"First","visibility":"list","supported_reasoning_levels":[{"effort":"fixture-effort","description":"native effort"}],"vendorMetadata":{"keep":true}},
            {"slug":"fixture-hidden","visibility":"hide"}
        ]
    });
    fs::write(&path, cache.to_string()).unwrap();
    let result = catalog_from_codex_cache(&path).await.unwrap();
    assert_eq!(result["status"], "ok");
    assert_eq!(result["source"], "codex_cache");
    assert_eq!(result["fetched_at"], "2026-11-12T19:00:00Z");
    assert_eq!(result["models"].as_array().unwrap().len(), 1);
    assert_eq!(result["models"][0]["vendorMetadata"]["keep"], true);
    assert_eq!(result["models"][0]["supported_reasoning_levels"][0]["effort"], "fixture-effort");
    assert!(!result.to_string().contains("fixture-private"));
    assert_eq!(serde_json::from_slice::<Value>(&fs::read(&path).unwrap()).unwrap(), cache);
    let updated = json!({"fetched_at":"2026-11-12T20:00:00Z","models":[{"slug":"app-added-model","display_name":"New model from app","visibility":"list"}]});
    fs::write(&path, updated.to_string()).unwrap();
    let result = catalog_from_codex_cache(&path).await.unwrap();
    assert_eq!(result["models"][0]["slug"], "app-added-model");
    assert_eq!(result["models"].as_array().unwrap().len(), 1);
    fs::remove_file(&path).unwrap();
    let error = catalog_from_codex_cache(&path).await.unwrap_err();
    assert_eq!(error.http_status, 503);
    assert_eq!(error.code, "WORKSPACE_MODEL_CATALOG_UNAVAILABLE");
    assert!(error.message.contains("models_cache.json"));
}

#[tokio::test]
async fn claude_picker_reads_initialization_models_without_sending_a_user_message() {
    let (temp, command) = fake_command(r#"
import json, sys, time
args = sys.argv[2:]
for flag in ['--print','--safe-mode','--strict-mcp-config','--no-session-persistence']:
    assert flag in args
assert args[args.index('--mcp-config')+1] == '{"mcpServers":{}}'
assert args[args.index('--tools')+1] == ''
assert args[args.index('--input-format')+1] == 'stream-json'
assert args[args.index('--output-format')+1] == 'stream-json'
request = json.loads(sys.stdin.readline())
with open(sys.argv[1], 'w') as trace: trace.write(json.dumps(request)+'\n')
assert request['type'] == 'control_request'
assert request['request'] == {'subtype':'initialize','hooks':None,'skills':[]}
print(json.dumps({'type':'control_response','response':{'subtype':'success','request_id':request['request_id'],'response':{'models':[{'value':'fixture-family','resolvedModel':'fixture-model','displayName':'Fixture model','supportedEffortLevels':['fixture-effort']}],'account':{'mustNotReturn':'fixture-private'}}}}), flush=True)
while True: time.sleep(10)
"#);
    let result = catalog_with_command("claude", command).await.unwrap();
    assert_eq!(result["models"][0]["resolvedModel"], "fixture-model");
    assert_eq!(result["models"][0]["supportedEffortLevels"][0], "fixture-effort");
    assert!(result.get("account").is_none());
    assert!(!result.to_string().contains("fixture-private"));
    let requests = fs::read_to_string(temp.path().join("requests.jsonl")).unwrap();
    assert_eq!(requests.lines().count(), 1);
    let request: Value = serde_json::from_str(requests.trim()).unwrap();
    assert_eq!(request["request"]["subtype"], "initialize");
    assert!(request.get("message").is_none());
}

#[tokio::test]
async fn catalog_failure_keeps_the_coding_apps_returned_error() {
    let (_temp, command) = fake_command(r#"
import json, sys
request = json.loads(sys.stdin.readline())
print('Fixture app diagnostic',file=sys.stderr,flush=True)
print(json.dumps({'type':'control_response','response':{'subtype':'error','request_id':request['request_id'],'error':{'code':403,'message':'Fixture policy forbids this catalog','data':{'reason':'fixture-policy'}}}}), flush=True)
"#);
    let error = catalog_with_command("claude", command).await.unwrap_err();
    assert_eq!(error.code, "WORKSPACE_MODEL_CATALOG_FAILED");
    assert!(error.message.contains("Fixture policy forbids this catalog"));
    assert!(error.message.contains("fixture-policy"));
    assert!(error.message.contains("403"));
    assert!(error.message.contains("Fixture app diagnostic"));
}

#[tokio::test]
async fn catalog_response_limit_also_bounds_one_large_line() {
    let (_temp, command) = fake_command(r#"
import sys
sys.stdin.readline()
sys.stdout.write('x' * (2*1024*1024+100))
sys.stdout.flush()
"#);
    let error = catalog_with_command("claude", command).await.unwrap_err();
    assert!(error.message.contains("exceeded 2 MiB"));
}
