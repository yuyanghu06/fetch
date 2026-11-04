//! Independent encrypted-store and private-channel fixtures; no real provider keys.
use super::*;
use super::bridge::{BridgeError, Factory, Policy, Verifier};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::UnixStream, sync::{mpsc, oneshot}};

#[path = "../../tests/helper_bridge/support.rs"]
mod sandbox;

struct FixtureKeys { unavailable: AtomicBool }
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        if self.unavailable.load(Ordering::SeqCst) { Err(CredentialError::KeyUnavailable) }
        else { Ok(Zeroizing::new([47; 32])) }
    }
}

struct Fixture {
    directory: tempfile::TempDir,
    keys: Arc<FixtureKeys>,
    store: Arc<CredentialStore>,
}
impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let keys = Arc::new(FixtureKeys { unavailable:AtomicBool::new(false) });
        let store = Arc::new(CredentialStore::new(directory.path().join("state"), keys.clone()));
        Self { directory, keys, store }
    }
    fn factory(&self, server: &str, verifier: Verifier) -> Factory {
        Factory::new(self.store.clone(), server.into(), "fixture-helper-v1".into(), verifier)
    }
}

fn accepts() -> Verifier { Arc::new(|_| Box::pin(async { Ok(()) })) }

async fn request(environment: &std::collections::BTreeMap<String, String>, mut value: Value) -> Value {
    value["capability"] = json!(environment["FETCH_HELPER_CAPABILITY"]);
    send_request(&environment["FETCH_HELPER_SOCKET"], value).await
}

async fn send_request(socket: &str, value: Value) -> Value {
    let stream = UnixStream::connect(socket).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut bytes = serde_json::to_vec(&value).unwrap(); bytes.push(b'\n');
    writer.write_all(&bytes).await.unwrap();
    let mut line = String::new();
    tokio::time::timeout(std::time::Duration::from_secs(3), BufReader::new(reader).read_line(&mut line)).await.unwrap().unwrap();
    if line.is_empty() { Value::Null } else { serde_json::from_str(&line).unwrap() }
}

#[tokio::test]
async fn login_ack_uses_encrypted_readback_preserves_other_sections_and_does_not_prove_login() {
    let fixture = Fixture::new();
    fixture.store.save("account", &SecretDocument::new(serde_json::to_vec(&json!({
        "manual":{"client":"fixture-client-secret"},"oauth":{"unrelated":"preserve"},"future":{"keep":17}
    })).unwrap())).unwrap();
    let factory = fixture.factory("account", accepts());
    let lease = factory.start(Policy::Login).await.unwrap();
    let environment = lease.environment();
    let initial = request(&environment, json!({"op":"load"})).await;
    assert_eq!(initial, json!({"ok":true,"revision":0,"payload":null,"verified":false}));
    let ack = request(&environment, json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-refresh-only"}})).await;
    assert_eq!(ack["ok"], true);
    assert_eq!(ack["revision"], 1);
    assert_eq!(ack["verified"], false);
    let saved = fixture.store.read_json("account").unwrap().unwrap();
    assert_eq!(saved["helper"]["payload"], ack["payload"]);
    assert_eq!(saved["manual"]["client"], "fixture-client-secret");
    assert_eq!(saved["oauth"]["unrelated"], "preserve");
    assert_eq!(saved["future"]["keep"], 17);
    assert!(saved["helper"]["verified_revision"].is_null());
    let encrypted = fixture.directory.path().join("state/servers").join(crate::storage::server_directory_name("account")).join("credentials.enc");
    let encrypted = std::fs::read(encrypted).unwrap();
    assert!(!encrypted.windows(b"fixture-refresh-only".len()).any(|window| window == b"fixture-refresh-only"));
    assert!(!encrypted.windows(b"fixture-client-secret".len()).any(|window| window == b"fixture-client-secret"));
    assert_eq!(request(&environment, json!({"op":"load"})).await, ack);
}

#[tokio::test]
async fn stale_revision_and_wrong_capability_cannot_overwrite_or_read_account_access() {
    let fixture = Fixture::new();
    let (failures, mut reported) = mpsc::unbounded_channel();
    let factory = fixture.factory("account", accepts()).with_failure_handler(Arc::new(move |error, _| { failures.send(error).unwrap(); }));
    let first = factory.start(Policy::Login).await.unwrap();
    let second = factory.start(Policy::Login).await.unwrap();
    let current = request(&first.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-current"}})).await;
    assert_eq!(current["revision"], 1);
    let stale = request(&second.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-stale"}})).await;
    assert_eq!(stale, json!({"ok":false,"error":"stale"}));
    assert_eq!(reported.recv().await, Some(BridgeError::Stale));
    let invalid = send_request(&first.environment()["FETCH_HELPER_SOCKET"], json!({"capability":"wrong-disposable-capability","op":"load"})).await;
    assert_eq!(invalid, json!({"ok":false,"error":"invalid"}));
    assert!(reported.try_recv().is_err(), "an unauthenticated probe must not fail the owning server");
    assert_eq!(fixture.store.read_json("account").unwrap().unwrap()["helper"]["payload"]["refresh_token"], "fixture-current");
}

#[tokio::test]
async fn hung_runtime_verification_expires_and_reports_failure_without_verified_ack() {
    let fixture = Fixture::new();
    let verifier: Verifier = Arc::new(|_| Box::pin(std::future::pending()));
    let (failures, mut reported) = mpsc::unbounded_channel();
    let factory = fixture.factory("account", verifier).with_failure_handler(Arc::new(move |error, _| { failures.send(error).unwrap(); }));
    let runtime = factory.start(Policy::Runtime).await.unwrap();
    let environment = runtime.environment();
    let stream = UnixStream::connect(&environment["FETCH_HELPER_SOCKET"]).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut bytes = serde_json::to_vec(&json!({"capability":environment["FETCH_HELPER_CAPABILITY"],"op":"update","expected_revision":0,
        "payload":{"refresh_token":"fixture-timeout-refresh"}})).unwrap();
    bytes.push(b'\n');
    writer.write_all(&bytes).await.unwrap();
    let mut line = String::new();
    // Exercise the production30-second limit, rather than shortening it in test code.
    tokio::time::timeout(std::time::Duration::from_secs(35), BufReader::new(reader).read_line(&mut line)).await.unwrap().unwrap();
    assert_eq!(serde_json::from_str::<Value>(&line).unwrap(), json!({"ok":false,"error":"unavailable"}));
    assert_eq!(reported.recv().await, Some(BridgeError::Unavailable));
    let saved = fixture.store.read_json("account").unwrap().unwrap();
    assert!(saved["helper"]["verified_revision"].is_null());
    assert_eq!(saved["helper"]["payload"]["refresh_token"], "fixture-timeout-refresh");
}

#[tokio::test]
async fn two_instances_of_the_same_helper_keep_distinct_payloads_and_capabilities() {
    let fixture = Fixture::new();
    let personal = fixture.factory("personal", accepts());
    let work = fixture.factory("work", accepts());
    let personal = personal.start(Policy::Login).await.unwrap();
    let work = work.start(Policy::Login).await.unwrap();
    assert_ne!(personal.environment()["FETCH_HELPER_CAPABILITY"], work.environment()["FETCH_HELPER_CAPABILITY"]);
    for (lease, token) in [(&personal, "fixture-personal-refresh"), (&work, "fixture-work-refresh")] {
        assert_eq!(request(&lease.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":token}})).await["ok"], true);
    }
    let mut cross = work.environment();
    cross.insert("FETCH_HELPER_CAPABILITY".into(), personal.environment()["FETCH_HELPER_CAPABILITY"].clone());
    assert_eq!(request(&cross, json!({"op":"load"})).await, json!({"ok":false,"error":"invalid"}));
    assert_eq!(request(&personal.environment(), json!({"op":"load"})).await["payload"]["refresh_token"], "fixture-personal-refresh");
    assert_eq!(request(&work.environment(), json!({"op":"load"})).await["payload"]["refresh_token"], "fixture-work-refresh");
}

#[tokio::test]
async fn verification_reads_saved_payload_and_failure_never_marks_renewal_verified() {
    let fixture = Fixture::new();
    let store = fixture.store.clone();
    let verifier: Verifier = Arc::new(move |payload| {
        let store = store.clone();
        Box::pin(async move {
            assert_eq!(store.read_json("account").unwrap().unwrap()["helper"]["payload"], payload);
            if payload["refresh_token"] == "fixture-rejected" { Err(BridgeError::Verification) } else { Ok(()) }
        })
    });
    let factory = fixture.factory("account", verifier);
    let login = factory.start(Policy::Login).await.unwrap();
    let saved = request(&login.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-initial"}})).await;
    assert_eq!(saved["verified"], false);
    assert_eq!(request(&login.environment(), json!({"op":"verify","expected_revision":1})).await["verified"], true);
    let runtime = factory.start(Policy::Runtime).await.unwrap();
    let rejected = request(&runtime.environment(), json!({"op":"update","expected_revision":1,"payload":{"refresh_token":"fixture-rejected"}})).await;
    assert_eq!(rejected, json!({"ok":false,"error":"verification"}));
    let saved = request(&runtime.environment(), json!({"op":"load"})).await;
    assert_eq!(saved["revision"], 2);
    assert_eq!(saved["verified"], false);
    assert_eq!(saved["payload"]["refresh_token"], "fixture-rejected");
}

#[tokio::test]
async fn runtime_rotation_waits_for_saved_access_verification_before_acknowledgement() {
    let fixture = Fixture::new();
    let (started, mut observed) = mpsc::unbounded_channel();
    let verifier: Verifier = Arc::new(move |payload| {
        let (release, wait) = oneshot::channel();
        started.send((payload, release)).unwrap();
        Box::pin(async move { wait.await.unwrap(); Ok(()) })
    });
    let factory = fixture.factory("account", verifier);
    let login = factory.start(Policy::Login).await.unwrap();
    request(&login.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-before"}})).await;
    let runtime = factory.start(Policy::Runtime).await.unwrap();
    let environment = runtime.environment();
    let update = tokio::spawn(async move { request(&environment, json!({"op":"update","expected_revision":1,"payload":{"refresh_token":"fixture-rotated","access_token":"fixture-access"}})).await });
    let (payload, release) = tokio::time::timeout(std::time::Duration::from_secs(3), observed.recv()).await.unwrap().unwrap();
    assert_eq!(payload["refresh_token"], "fixture-rotated");
    assert!(!update.is_finished());
    assert_eq!(fixture.store.read_json("account").unwrap().unwrap()["helper"]["payload"], payload);
    release.send(()).unwrap();
    let ack = update.await.unwrap();
    assert_eq!(ack["ok"], true);
    assert_eq!(ack["verified"], true);
    assert_eq!(ack["revision"], 2);
}

#[tokio::test]
async fn revocation_cancels_pending_proof_and_old_lease_cannot_overwrite_a_new_session() {
    let fixture = Fixture::new();
    let (started, mut observed) = mpsc::unbounded_channel();
    let verifier: Verifier = Arc::new(move |_| {
        let (release, wait) = oneshot::channel::<()>();
        started.send(release).unwrap();
        Box::pin(async move { let _ = wait.await; Ok(()) })
    });
    let factory = fixture.factory("account", verifier);
    let login = factory.start(Policy::Login).await.unwrap();
    request(&login.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-before"}})).await;
    let runtime = factory.start(Policy::Runtime).await.unwrap();
    let old_socket = runtime.environment()["FETCH_HELPER_SOCKET"].clone();
    let environment = runtime.environment();
    let update = tokio::spawn(async move { request(&environment, json!({"op":"update","expected_revision":1,"payload":{"refresh_token":"fixture-unverified"}})).await });
    let release = tokio::time::timeout(std::time::Duration::from_secs(3), observed.recv()).await.unwrap().unwrap();
    factory.controller().revoke().await;
    let response = update.await.unwrap();
    assert!(response.is_null() || response["ok"] == false);
    assert!(!std::path::Path::new(&old_socket).exists());
    let _ = release.send(());
    assert!(matches!(factory.start(Policy::Login).await, Err(BridgeError::Revoked)));
    let replacement_factory = fixture.factory("account", accepts());
    let replacement = replacement_factory.start(Policy::Login).await.unwrap();
    let snapshot = request(&replacement.environment(), json!({"op":"load"})).await;
    assert_eq!(snapshot["verified"], false);
    let updated = request(&replacement.environment(), json!({"op":"update","expected_revision":snapshot["revision"],"payload":{"refresh_token":"fixture-replacement"}})).await;
    assert_eq!(updated["ok"], true);
    assert_eq!(fixture.store.read_json("account").unwrap().unwrap()["helper"]["payload"]["refresh_token"], "fixture-replacement");
}

#[tokio::test]
async fn storage_failure_has_no_success_ack_and_lease_drop_removes_private_socket() {
    let fixture = Fixture::new();
    let factory = fixture.factory("account", accepts());
    let lease = factory.start(Policy::Login).await.unwrap();
    let socket = lease.environment()["FETCH_HELPER_SOCKET"].clone();
    fixture.keys.unavailable.store(true, Ordering::SeqCst);
    assert_eq!(request(&lease.environment(), json!({"op":"update","expected_revision":0,"payload":{"refresh_token":"fixture-not-saved"}})).await,
        json!({"ok":false,"error":"storage"}));
    drop(lease);
    assert!(!std::path::Path::new(&socket).exists());
    assert!(!std::path::Path::new(&socket).parent().unwrap().exists());
}

#[tokio::test]
async fn confined_node_client_uses_private_channel_without_polluting_raw_stdout_or_token_files() {
    let fixture = Fixture::new();
    let factory = fixture.factory("account", accepts());
    let lease = factory.start(Policy::Login).await.unwrap();
    let mut sandbox = sandbox::Sandbox::new();
    sandbox.allow_bridge(std::path::Path::new(&lease.environment()["FETCH_HELPER_SOCKET"]));
    sandbox.assert_confinement().await;
    sandbox.write("bridge_client.mjs", bridge::BRIDGE_CLIENT);
    let script = sandbox.write("client-fixture.mjs", r#"
import assert from 'node:assert/strict';
import { createBridgeClient } from './bridge_client.mjs';
const bridge = createBridgeClient();
const first = await bridge.load();
assert.equal(first.revision, 0);
const saved = await bridge.update(first.revision, {refresh_token:'fixture-node-refresh'});
assert.equal(saved.verified, false);
const checked = await bridge.verify(saved.revision);
assert.equal(checked.verified, true);
await assert.rejects(bridge.update(0, {refresh_token:'fixture-stale'}), error => error.code === 'stale');
process.stdout.write('{ "jsonrpc":"2.0", "result":{"future":1844674407370955161600001} }\n');
"#);
    let mut command = sandbox.command();
    command.arg(script).envs(lease.environment());
    let output = tokio::time::timeout(std::time::Duration::from_secs(10), command.output()).await.unwrap().unwrap();
    assert!(output.status.success(), "confined bridge client failed: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, b"{ \"jsonrpc\":\"2.0\", \"result\":{\"future\":1844674407370955161600001} }\n");
    assert_eq!(fixture.store.read_json("account").unwrap().unwrap()["helper"]["payload"]["refresh_token"], "fixture-node-refresh");
    assert!(!sandbox.home.join(".config").exists());
    assert!(!sandbox.root.join("token.json").exists());
}
