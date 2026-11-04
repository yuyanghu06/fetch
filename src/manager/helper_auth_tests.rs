//! Synthetic Google-helper fixtures behind the real private bridge and supervisor.
//! Node runs in verified OS confinement; only disposable loopback account checks run.
use super::*;
use super::auth_tests::MockProvider;
use super::helper_auth::{self, GoogleLaunch};
use axum::http::StatusCode;
use crate::credentials::{KeyProvider, bridge::Policy};
use zeroize::Zeroizing;

#[path = "../../tests/helper_bridge/support.rs"]
mod sandbox;

const FAKE_OAUTH: &str = r#"
class OAuth2 {
  constructor(id, secret) { this.credentials = {}; this.secret = secret; }
  setCredentials(value) { this.credentials = {...value}; }
  generateAuthUrl() { return 'https://accounts.google.com/o/oauth2/auth?state=disposable'; }
  async refreshTokenNoCache(refresh) {
    if (refresh.includes('rejected') || this.secret === 'fixture-rejected-app-secret') throw new Error('Disposable rejected refresh grant');
    return {tokens:{access_token:'fixture-access-' + refresh,refresh_token:refresh + '-rotated',token_type:'Bearer',expiry_date:Date.now() + 3600000},res:{}};
  }
  async getAccessToken() {
    if (!this.credentials.access_token || this.credentials.expiry_date <= Date.now()) {
      const response = await this.refreshTokenNoCache(this.credentials.refresh_token);
      this.credentials = {...this.credentials,...response.tokens};
    }
    return {token:this.credentials.access_token};
  }
}
module.exports = {OAuth2};
"#;
const RAW_STDOUT: &[u8] = b"{ \"jsonrpc\":\"2.0\", \"result\":{\"number\":184467440737095516160000001,\"content\":[{\"type\":\"image\",\"mimeType\":\"image/png\",\"data\":\"fixture-base64\"}]} }\n";

struct FixtureKeys;
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> { Ok(Zeroizing::new([83; 32])) }
}

struct Fixture {
    sandbox: sandbox::Sandbox,
    manager: Arc<Manager>,
    launch: GoogleLaunch,
}
impl Fixture {
    fn new(endpoint: &str) -> Self {
        let sandbox = sandbox::Sandbox::new();
        let package = sandbox.root.join("package");
        sandbox.write("package/package.json", r#"{"name":"fixture-google-helper","version":"1.11.3","type":"module"}"#);
        sandbox.write("package/dist/auth.js", include_str!("../../tests/helper_bridge/google_auth_fixture.mjs"));
        let entry = sandbox.write("package/dist/index.js", include_str!("../../tests/helper_bridge/google_runtime_fixture.mjs"));
        sandbox.write("package/credentials.json", r#"{"must_not_read":"disposable-fallback-credential"}"#);
        sandbox.write("package/node_modules/googleapis/package.json", r#"{"main":"index.cjs"}"#);
        sandbox.write("package/node_modules/googleapis/index.cjs", "const {OAuth2}=require('googleapis-common/node_modules/google-auth-library/build/src/auth/oauth2client.js');module.exports={google:{auth:{OAuth2}}};");
        sandbox.write("package/node_modules/googleapis-common/package.json", r#"{"main":"index.cjs"}"#);
        sandbox.write("package/node_modules/googleapis-common/index.cjs", "module.exports={};");
        sandbox.write("package/node_modules/googleapis-common/node_modules/google-auth-library/build/src/auth/oauth2client.js", FAKE_OAUTH);
        let script = helper_auth::script().replace("af1aeb78dcd8ac899f157686f07c7b492131605fbf940909890fe9a4eeb90fe1", &format!("{:x}", Sha256::digest(FAKE_OAUTH.as_bytes())));
        let launch = GoogleLaunch { executable:sandbox.launcher(),entry,home:sandbox.home.clone(),script_override:Some(script) };
        let root = sandbox.root.join("state");
        let repository = ServerRepository::new(root.clone());
        let credentials = Arc::new(CredentialStore::new(root.clone(), Arc::new(FixtureKeys)));
        let installer = Installer::new(root, crate::discovery::DiscoveryService::new(sandbox.root.join("empty-home"), None));
        let mut manager = Manager::new(repository, credentials, Runner::new(), installer);
        let mutable = Arc::get_mut(&mut manager).unwrap();
        mutable.helper_test_launch = Some(launch.clone());
        mutable.verification_test_endpoint = Some(endpoint.into());
        mutable.verification_test_profile = Some(serde_json::from_value(json!({
            "id":"fixture-helper-account","targets":{"packages":["@a-bonus/google-docs-mcp"],"hosts":[]},"manual":[],
            "oauth":{"method":"GET","url":"https://verification.fixture.example/check","credentials":[
                {"input":"token","placement":"header","name":"Authorization","prefix":"Bearer "}
            ],"success":[{"rule":"string","pointer":"/id","nonempty":true}]}
        })).unwrap());
        mutable.http = reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(3)).build().unwrap();
        assert!(package.starts_with(&sandbox.root));
        Self { sandbox,manager,launch }
    }

    fn account(&self, id: &str, client: &str, helper: Value) -> Value {
        let record = json!({"server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
            "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
            "config":{"kind":"npm","package":{"name":"@a-bonus/google-docs-mcp","requested_version":null,"installed_version":"1.11.3"},
                "executable":self.launch.executable,"args":[self.launch.entry],"working_directory":null,
                "environment":[{"name":"GOOGLE_CLIENT_ID","credential_ref":"client-id"},{"name":"GOOGLE_CLIENT_SECRET","credential_ref":"client-secret"}]},
            "credentials":[{"name":"client-id","status":"stored"},{"name":"client-secret","status":"stored"}]});
        self.manager.repository.save(id, &record).unwrap();
        let mut document = json!({"manual":{"client-id":client,"client-secret":format!("fixture-{id}-app-secret")},"oauth":null,"other":{"preserve":id}});
        if !helper.is_null() { document["helper"] = helper; }
        self.manager.credentials.save(id, &SecretDocument::new(serde_json::to_vec(&document).unwrap())).unwrap();
        record
    }

    async fn terminal(&self, id: &str) -> Value {
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let live = self.manager.live.lock().await.get(id).cloned().unwrap_or_default();
                if !live.authenticating && matches!(live.auth["status"].as_str(), Some("failed" | "authenticated")) { return live.auth; }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("synthetic helper must reach a terminal authentication state")
    }

    fn assert_no_token_file(&self) {
        assert!(!self.sandbox.home.join(".config/google-docs-mcp/fetch/token.json").exists());
        assert!(!self.sandbox.root.join("package/token.json").exists());
        assert_eq!(std::fs::read_to_string(self.sandbox.root.join("package/credentials.json")).unwrap(), r#"{"must_not_read":"disposable-fallback-credential"}"#);
    }
}

fn saved(payload: Value, verified: bool) -> Value {
    json!({"adapter":helper_auth::ADAPTER,"revision":1,"payload":payload,"verified_revision":if verified { Some(1) } else { None }})
}

#[test]
fn helper_environment_uses_assigned_app_entries_and_overrides_user_storage_and_runtime_paths() {
    let launch = GoogleLaunch {executable:"/private/fixture/node".into(),entry:"/private/fixture/package/dist/index.js".into(),home:"/private/fixture/account-home".into(),script_override:None};
    let record = json!({"config":{"kind":"npm","environment":[
        {"name":"GOOGLE_CLIENT_ID","credential_ref":"assigned-id"},{"name":"GOOGLE_CLIENT_SECRET","credential_ref":"assigned-secret"},
        {"name":"HOME","credential_ref":"old-home"},{"name":"XDG_CONFIG_HOME","credential_ref":"old-xdg"},
        {"name":"GOOGLE_MCP_PROFILE","credential_ref":"old-profile"},{"name":"NODE_OPTIONS","credential_ref":"old-options"}
    ]}});
    let document = json!({"manual":{"assigned-id":"fixture-id","assigned-secret":"fixture-secret"}});
    let environment = helper_auth::environment(&record, &document, &launch).unwrap();
    assert_eq!(environment["GOOGLE_CLIENT_ID"], "fixture-id");
    assert_eq!(environment["GOOGLE_CLIENT_SECRET"], "fixture-secret");
    assert_eq!(environment["HOME"], "/private/fixture/account-home");
    assert_eq!(environment["XDG_CONFIG_HOME"], "/private/fixture/account-home/.config");
    assert_eq!(environment["GOOGLE_MCP_PROFILE"], "fetch");
    assert!(!environment.contains_key("NODE_OPTIONS"));
    let mut unsupported = record;
    unsupported["config"]["environment"].as_array_mut().unwrap().push(json!({"name":"SERVICE_ACCOUNT_PATH","credential_ref":"personal-file"}));
    assert_eq!(helper_auth::environment(&unsupported, &document, &launch).unwrap_err().code, "INVALID_CONFIG");
}

#[tokio::test]
async fn browser_page_and_helper_exit_without_saved_access_never_report_authentication_success() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("page", "page-only", Value::Null);
    let accepted = fixture.manager.authenticate("page").await.unwrap();
    assert_eq!(accepted["status"], "authenticating");
    assert_eq!(fixture.terminal("page").await["status"], "failed");
    assert!(fixture.manager.secrets("page").unwrap()["helper"].is_null());
    fixture.assert_no_token_file();
    provider.no_more_requests();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn refresh_only_login_waits_for_encrypted_saved_access_proof_and_joins_repeated_post() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("personal", "personal", Value::Null);
    assert_eq!(fixture.manager.authenticate("personal").await.unwrap()["status"], "authenticating");
    let attempt = fixture.manager.live.lock().await["personal"].auth_attempt.clone();
    assert_eq!(fixture.manager.authenticate("personal").await.unwrap()["status"], "authenticating");
    assert_eq!(fixture.manager.live.lock().await["personal"].auth_attempt, attempt);
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-access-personal-fresh-refresh");
    let document = fixture.manager.secrets("personal").unwrap();
    assert_eq!(document["helper"]["payload"]["access_token"], "fixture-access-personal-fresh-refresh");
    assert!(document["helper"]["verified_revision"].is_null());
    assert!(fixture.manager.live.lock().await["personal"].authenticating);
    check.respond.send(StatusCode::OK).unwrap();
    // The bootstrap verifies after refresh and before returning prepared access.
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    assert_eq!(fixture.terminal("personal").await["status"], "authenticated");
    let document = fixture.manager.secrets("personal").unwrap();
    assert_eq!(document["helper"]["verified_revision"], document["helper"]["revision"]);
    assert_eq!(document["manual"]["client-secret"], "fixture-personal-app-secret");
    assert_eq!(document["other"]["preserve"], "personal");
    fixture.assert_no_token_file();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn account_checks_and_other_server_reads_finish_while_another_google_account_is_pending() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("personal", "personal", Value::Null);
    fixture.account("work", "work", Value::Null);
    fixture.manager.authenticate("personal").await.unwrap();
    let personal = provider.next().await;
    fixture.manager.authenticate("work").await.unwrap();
    let work = provider.next().await;
    assert_eq!(work.authorization, "Bearer fixture-access-work-fresh-refresh");
    work.respond.send(StatusCode::OK).unwrap();
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    assert_eq!(fixture.terminal("work").await["status"], "authenticated");
    assert!(fixture.manager.live.lock().await["personal"].authenticating);
    let caller = Caller::from_verified_scopes(["fetch:manage".to_owned()]);
    let state = tokio::time::timeout(Duration::from_secs(1), fixture.manager.servers_for(&caller, vec!["work".into()], None)).await.unwrap().unwrap();
    assert_eq!(state["servers"][0]["authentication"]["status"], "authenticated");
    personal.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert_eq!(fixture.terminal("personal").await["status"], "failed");
    assert_ne!(fixture.manager.secrets("personal").unwrap()["helper"]["payload"], fixture.manager.secrets("work").unwrap()["helper"]["payload"]);
    fixture.assert_no_token_file();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn failed_reverification_clears_persisted_verified_marker() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    let record = fixture.account("saved", "saved", saved(json!({"refresh_token":"fixture-existing-refresh","access_token":"fixture-existing-access","expiry_date":4_000_000_000_000u64}), true));
    // Recheck within the existing owner. A newly created owner deliberately
    // forces refresh to prove the currently assigned app credentials.
    let _context = fixture.manager.helper_context(&record).await.unwrap();
    let manager = fixture.manager.clone();
    let pending = tokio::spawn(async move { manager.prepare_helper_access(&record, false).await });
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-existing-access");
    check.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert!(pending.await.unwrap().is_err());
    assert!(fixture.manager.secrets("saved").unwrap()["helper"]["verified_revision"].is_null());
    fixture.assert_no_token_file();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn explicit_retry_after_rejected_saved_grant_runs_fresh_helper_without_within_attempt_fallback() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("retry", "retry", saved(json!({"refresh_token":"fixture-rejected-refresh"}), false));
    fixture.manager.authenticate("retry").await.unwrap();
    assert_eq!(fixture.terminal("retry").await["status"], "failed");
    assert_eq!(fixture.manager.secrets("retry").unwrap()["helper"]["payload"]["refresh_token"], "fixture-rejected-refresh");
    provider.no_more_requests();
    fixture.manager.authenticate("retry").await.unwrap();
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-access-retry-fresh-refresh");
    check.respond.send(StatusCode::OK).unwrap();
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    assert_eq!(fixture.terminal("retry").await["status"], "authenticated");
    fixture.assert_no_token_file();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn runtime_refresh_waits_for_ack_preserves_raw_stdout_and_never_uses_plaintext_fallback() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    let record = fixture.account("runtime", "runtime", saved(json!({"refresh_token":"fixture-runtime-refresh","access_token":"fixture-expired-access","expiry_date":1}), true));
    let context = fixture.manager.helper_context(&record).await.unwrap();
    let lease = context.factory.start(Policy::Runtime).await.unwrap();
    let mut environment = helper_auth::environment(&record, &fixture.manager.secrets("runtime").unwrap(), &fixture.launch).unwrap();
    environment.extend(lease.environment());
    let mut command = tokio::process::Command::new(&fixture.launch.executable);
    command.args(fixture.launch.args("runtime")).env_clear().envs(environment).current_dir(&fixture.launch.home)
        .stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).kill_on_drop(true);
    let pending = tokio::spawn(async move { command.output().await.unwrap() });
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-access-fixture-runtime-refresh");
    assert!(!pending.is_finished(), "runtime cannot use a rotated token before storage/check acknowledgement");
    check.respond.send(StatusCode::OK).unwrap();
    let output = pending.await.unwrap();
    assert!(output.status.success(), "synthetic runtime failed: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, RAW_STDOUT);
    let stored = fixture.manager.secrets("runtime").unwrap();
    assert_eq!(stored["helper"]["payload"]["refresh_token"], "fixture-runtime-refresh-rotated");
    assert_eq!(stored["helper"]["verified_revision"], stored["helper"]["revision"]);
    let missing = fixture.account("missing", "missing", Value::Null);
    let missing_context = fixture.manager.helper_context(&missing).await.unwrap();
    let missing_lease = missing_context.factory.start(Policy::Runtime).await.unwrap();
    let mut environment = helper_auth::environment(&missing, &fixture.manager.secrets("missing").unwrap(), &fixture.launch).unwrap();
    environment.extend(missing_lease.environment());
    let output = tokio::process::Command::new(&fixture.launch.executable).args(fixture.launch.args("runtime"))
        .env_clear().envs(environment).current_dir(&fixture.launch.home).kill_on_drop(true).output().await.unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty(), "normal startup with missing stored access must not start browser or emit auth events");
    provider.no_more_requests();
    fixture.assert_no_token_file();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn execution_setting_edit_preserves_the_live_helper_renewal_context_without_restart() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    let record = fixture.account("settings", "settings", saved(json!({"refresh_token":"fixture-before","access_token":"fixture-before-access"}), true));
    fixture.manager.live.lock().await.insert("settings".into(), Live { auth:json!({"status":"authenticated","error":null}),applied:Some(record["config"].clone()),..Live::default() });
    let context = fixture.manager.helper_context(&record).await.unwrap();
    let lease = context.factory.start(Policy::Runtime).await.unwrap();
    let caller = Caller::from_verified_scopes(["fetch:manage".to_owned()]);
    let changed = fixture.manager.config("settings", json!({"execution":{"default_timeout_ms":240000}}), &caller).await.unwrap();
    assert_eq!(changed["changed"], true);
    assert_eq!(changed["restart_required"], false);
    assert!(context.active.load(Ordering::Acquire));
    let latest = fixture.manager.record("settings").unwrap();
    let after = fixture.manager.helper_context(&latest).await.unwrap();
    assert!(Arc::ptr_eq(&context.active, &after.active));
    assert_eq!(fixture.manager.live.lock().await["settings"].auth["status"], "authenticated");
    let environment = lease.environment();
    let pending = tokio::spawn(async move {
        let stream = tokio::net::UnixStream::connect(&environment["FETCH_HELPER_SOCKET"]).await.unwrap();
        let (reader, mut writer) = stream.into_split();
        let mut bytes = serde_json::to_vec(&json!({"capability":environment["FETCH_HELPER_CAPABILITY"],"op":"update","expected_revision":1,
            "payload":{"refresh_token":"fixture-after","access_token":"fixture-after-access"}})).unwrap();
        bytes.push(b'\n'); writer.write_all(&bytes).await.unwrap();
        let mut line = String::new(); BufReader::new(reader).read_line(&mut line).await.unwrap();
        serde_json::from_str::<Value>(&line).unwrap()
    });
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    assert_eq!(pending.await.unwrap()["verified"], true);
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn changed_app_secret_cannot_borrow_a_fresh_cached_access_token_as_save_proof() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("save", "save", saved(json!({"refresh_token":"fixture-good-refresh","access_token":"fixture-old-fresh-access","expiry_date":4_000_000_000_000u64}), true));
    let gate = fixture.manager.lock("save").await;
    let _guard = gate.lock().await;
    // The public native-confirmation boundary is unchanged. Exercise its shared
    // post-confirmation save with a new disposable app secret.
    let result = fixture.manager.save_credentials_locked("save", json!({"set":[{"name":"client-secret","value":"fixture-rejected-app-secret"}]})).await;
    assert_eq!(result.unwrap_err().code, "CREDENTIAL_VERIFICATION_FAILED");
    let document = fixture.manager.secrets("save").unwrap();
    assert_eq!(document["manual"]["client-secret"], "fixture-rejected-app-secret");
    assert!(document["helper"]["verified_revision"].is_null());
    assert_eq!(fixture.manager.live.lock().await["save"].auth["status"], "failed");
    provider.no_more_requests();
    fixture.assert_no_token_file();
    drop(_guard);
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn shutdown_stops_waiting_helper_and_never_claims_a_success_page_completed_login() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    fixture.account("waiting", "wait", Value::Null);
    fixture.manager.authenticate("waiting").await.unwrap();
    let pid_file = fixture.sandbox.home.join("waiting-pid");
    let pid: i32 = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let Ok(value) = std::fs::read_to_string(&pid_file) { break value.parse().unwrap(); }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.unwrap();
    fixture.manager.shutdown().await;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            // This PID belongs to the disposable helper just launched by this test.
            if unsafe { libc::kill(pid, 0) } == -1 { break; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("owned helper must exit on shutdown");
    assert_ne!(fixture.manager.live.lock().await["waiting"].auth["status"], "authenticated");
    assert!(fixture.manager.helpers.lock().await.is_empty());
    fixture.assert_no_token_file();
    provider.no_more_requests();
}

#[tokio::test]
async fn managed_stdio_attaches_bridge_preserves_raw_mcp_and_closes_child_and_socket() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.sandbox.assert_confinement().await;
    std::fs::write(&fixture.launch.entry, include_str!("../../tests/helper_bridge/mcp_runtime_fixture.mjs")).unwrap();
    let record = fixture.account("managed", "managed", saved(json!({"refresh_token":"fixture-managed-refresh","access_token":"fixture-managed-access","expiry_date":4_000_000_000_000u64}), true));
    let transport = fixture.manager.helper_transport(&record).await.unwrap();
    fixture.manager.runner.configure(ServerRoute {server_id:"managed".into(),transport,execution:ExecutionPolicy::default()}).await.unwrap();
    tokio::time::timeout(Duration::from_secs(5), fixture.manager.runner.connect("managed")).await.unwrap().unwrap();
    assert_eq!(fixture.manager.runner.snapshot("managed").await["runtime"]["status"], "ready");
    let document = fixture.manager.secrets("managed").unwrap();
    fixture.manager.connection_succeeded("managed", &record, manual_hashes(&document)).await.unwrap();
    assert_eq!(fixture.manager.live.lock().await["managed"].auth["status"], "authenticated");
    let reply = fixture.manager.runner.call("managed", "echo", crate::runner::raw::raw(&json!({"message":"disposable"})),
        crate::runner::InvocationContext::terminal("fixture-owned-client".into())).await.unwrap();
    assert_eq!(reply.get(), "{ \"number\":184467440737095516160000001,\"content\":[{\"type\":\"image\",\"mimeType\":\"image/png\",\"data\":\"fixture-base64\"}],\"future\":{\"keep\":true} }");
    let pid: i32 = std::fs::read_to_string(fixture.sandbox.home.join("managed-pid")).unwrap().parse().unwrap();
    let socket = std::fs::read_to_string(fixture.sandbox.home.join("managed-socket")).unwrap();
    assert!(std::path::Path::new(&socket).exists());
    fixture.manager.runner.disconnect("managed", false).await.unwrap();
    assert_eq!(fixture.manager.runner.snapshot("managed").await["runtime"]["status"], "stopped");
    assert!(!std::path::Path::new(&socket).exists());
    assert_eq!(unsafe { libc::kill(pid, 0) }, -1, "the disposable managed child must have exited");
    fixture.assert_no_token_file();
    provider.no_more_requests();
    fixture.manager.shutdown().await;
}

#[tokio::test]
async fn queued_failure_from_a_closed_process_lease_cannot_invalidate_the_current_owner() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    let record = fixture.account("generation", "generation", saved(json!({"refresh_token":"fixture-before","access_token":"fixture-before-access"}), true));
    fixture.manager.live.lock().await.insert("generation".into(), Live {auth:json!({"status":"authenticated","error":null}),..Live::default()});
    let context = fixture.manager.helper_context(&record).await.unwrap();
    let lease = context.factory.start(Policy::Runtime).await.unwrap();
    let environment = lease.environment();
    let pending = tokio::spawn(async move {
        let stream = tokio::net::UnixStream::connect(&environment["FETCH_HELPER_SOCKET"]).await.unwrap();
        let (reader, mut writer) = stream.into_split();
        let mut bytes = serde_json::to_vec(&json!({"capability":environment["FETCH_HELPER_CAPABILITY"],"op":"update","expected_revision":1,
            "payload":{"refresh_token":"fixture-rejected","access_token":"fixture-rejected-access"}})).unwrap();
        bytes.push(b'\n'); writer.write_all(&bytes).await.unwrap();
        let mut line = String::new(); BufReader::new(reader).read_line(&mut line).await.unwrap();
        serde_json::from_str::<Value>(&line).unwrap()
    });
    let check = provider.next().await;
    let states = fixture.manager.live.lock().await;
    check.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert_eq!(pending.await.unwrap()["error"], "verification");
    // Let the failure callback queue on the live-state lock before retiring its
    // process lease. The callback must check that lease again after acquiring it.
    tokio::task::yield_now().await;
    lease.close();
    let replacement = context.factory.start(Policy::Runtime).await.unwrap();
    drop(states);
    assert_eq!(fixture.manager.live.lock().await["generation"].auth["status"], "authenticated");
    assert!(context.active.load(Ordering::Acquire));
    drop(replacement);
    fixture.manager.shutdown().await;
}
