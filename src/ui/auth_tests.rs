//! Independent, window-free checks for concurrent authentication and late replies.
use super::{AuthAction, Desktop, Message, Page, client::NativeClient};
use super::{
    auth_requests::Requests,
    authentication::{Flow, Phase},
};
use crate::{
    application::ApplicationHost,
    credentials::{CredentialError, CredentialStore, KeyProvider},
};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use zeroize::Zeroizing;

struct UiFixtureKeys;
impl KeyProvider for UiFixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([83; 32]))
    }
}

async fn desktop_fixture() -> (tempfile::TempDir, Desktop) {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("ui-state");
    let store = Arc::new(CredentialStore::new(root.clone(), Arc::new(UiFixtureKeys)));
    let host = ApplicationHost::fixture(root, store).await.unwrap();
    let session = Arc::new(host.native.open_settings());
    let client = NativeClient::new(
        host.state.origin.clone(),
        &host.certificate_pem,
        session.clone(),
    )
    .unwrap();
    // No Desktop::boot, window::open, application start or task execution.
    let desktop = Desktop {
        host,
        client: Some(client),
        session: Some(session),
        window: Some(iced::window::Id::unique()),
        page: Page::Servers,
        servers: json!({"servers":[
            {"server_id":"github","authentication":{"status":"required","error":null},"config":{"environment":[]}},
            {"server_id":"tavily","authentication":{"status":"required","error":null},"config":{"environment":[]}}
        ]}),
        discovery: serde_json::Value::Null,
        tools: serde_json::Value::Null,
        selected: BTreeSet::new(),
        config_path: String::new(),
        picker_window: None,
        selected_server: String::new(),
        config: "{}".into(),
        timeout: "120".into(),
        concurrency: "1".into(),
        advanced: false,
        secret_name: String::new(),
        secret_value: String::new(),
        job: None,
        progress: serde_json::Value::Null,
        restarts: BTreeMap::new(),
        error: None,
        notice: String::new(),
        busy: false,
        polling: None,
        installation_polling: false,
        restart_polling: BTreeSet::new(),
        cleanup_confirmed: false,
        expanded_servers: BTreeSet::new(),
        expanded_tools: BTreeSet::new(),
        authentication: BTreeMap::new(),
        auth_requests: Requests::default(),
    };
    (directory, desktop)
}

#[tokio::test]
async fn desktop_authenticate_messages_overlap_and_report_independent_results() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    let github = desktop.auth_requests.latest_sequence();
    drop(desktop.update(Message::Authenticate("tavily".into())));
    let tavily = desktop.auth_requests.latest_sequence();
    assert!(!desktop.busy);
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.auth_requests.contains("tavily"));
    assert!(desktop.authentication["github"].phase == Phase::Starting);
    assert!(desktop.authentication["tavily"].phase == Phase::Starting);
    drop(desktop.update(Message::AuthFinished(
        tavily,
        "tavily".into(),
        AuthAction::Authenticate,
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    assert!(desktop.authentication["tavily"].phase == Phase::Authenticated);
    assert!(desktop.auth_requests.contains("github"));
    drop(desktop.update(Message::AuthFinished(
        github,
        "github".into(),
        AuthAction::Authenticate,
        Err("Fixture authenticated service check failed".into()),
    )));
    assert!(desktop.authentication["github"].phase == Phase::Failed);
    assert!(desktop.authentication["tavily"].phase == Phase::Authenticated);
    assert!(!desktop.busy);
}

#[tokio::test]
async fn desktop_repeated_same_target_and_restart_do_not_replace_pending_authentication() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    let token = desktop.auth_requests.latest_sequence();
    drop(desktop.update(Message::Authenticate("github".into())));
    drop(desktop.update(Message::Restart("github".into(), false)));
    assert_eq!(desktop.auth_requests.latest_sequence(), token);
    assert!(!desktop.busy);
    assert!(desktop.auth_requests.contains("github"));
    drop(desktop.update(Message::AuthFinished(
        token + 100,
        "github".into(),
        AuthAction::Authenticate,
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    assert!(desktop.authentication["github"].phase == Phase::Starting);
    assert!(desktop.auth_requests.contains("github"));
}

#[tokio::test]
async fn desktop_settings_save_has_its_own_request_and_clears_the_entered_secret() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    drop(desktop.update(Message::Choose("tavily".into())));
    drop(desktop.update(Message::SecretName("api-key".into())));
    drop(desktop.update(Message::SecretValue("disposable-ui-secret".into())));
    drop(desktop.update(Message::SaveSecret));
    let token = desktop.auth_requests.latest_sequence();
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.auth_requests.contains("tavily"));
    assert!(desktop.secret_value.is_empty());
    assert!(desktop.authentication["tavily"].phase == Phase::Saving);
    assert!(!desktop.busy);
    drop(desktop.update(Message::AuthFinished(
        token,
        "tavily".into(),
        AuthAction::SettingsCredential,
        Ok(json!({"status":"ok","next_steps":[]})),
    )));
    assert!(desktop.authentication["tavily"].phase == Phase::Authenticated);
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.notice.starts_with("Saved."));
}

#[tokio::test]
async fn desktop_rejects_status_reads_started_before_authentication_completed() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    let token = desktop.auth_requests.latest_sequence();
    let (revision, sequence) = desktop.auth_requests.begin_read();
    let window = desktop.window;
    drop(desktop.update(Message::AuthFinished(
        token,
        "github".into(),
        AuthAction::Authenticate,
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    drop(desktop.update(Message::Servers(
        revision,
        sequence,
        window,
        Ok(json!({"servers":[
            {"server_id":"github","authentication":{"status":"unknown","error":null}}
        ]})),
    )));
    assert!(desktop.authentication["github"].phase == Phase::Authenticated);
    assert_eq!(desktop.servers["servers"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn desktop_old_window_and_old_auth_replies_cannot_overwrite_reopened_state() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    let old_auth = desktop.auth_requests.latest_sequence();
    let (revision, sequence) = desktop.auth_requests.begin_read();
    let old_window = desktop.window;
    // Exercise the close-authentication cleanup without calling native picker
    // cancellation or opening/closing any real operating-system window.
    desktop.close_authentication();
    desktop.window = Some(iced::window::Id::unique());
    drop(desktop.update(Message::Authenticate("github".into())));
    drop(desktop.update(Message::AuthFinished(
        old_auth,
        "github".into(),
        AuthAction::Authenticate,
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    drop(desktop.update(Message::Servers(
        revision,
        sequence,
        old_window,
        Ok(json!({"servers":[]})),
    )));
    assert!(desktop.authentication["github"].phase == Phase::Starting);
    assert!(desktop.auth_requests.contains("github"));
    assert_eq!(desktop.servers["servers"].as_array().unwrap().len(), 2);
}

#[test]
fn independent_services_finish_in_any_order_without_losing_other_requests() {
    let mut requests = Requests::default();
    let github = requests.begin("github").unwrap();
    let tavily = requests.begin("tavily").unwrap();
    assert_ne!(github, tavily);
    assert!(requests.finish("tavily", tavily));
    assert!(requests.contains("github"));
    assert!(!requests.contains("tavily"));
    assert!(requests.finish("github", github));
    assert!(!requests.contains("github"));
}

#[test]
fn duplicate_same_service_and_wrong_reply_cannot_displace_active_request() {
    let mut requests = Requests::default();
    let token = requests.begin("github").unwrap();
    let revision = requests.revision();
    assert!(requests.begin("github").is_none());
    assert_eq!(requests.revision(), revision);
    assert!(!requests.finish("github", token + 1));
    assert!(!requests.finish("tavily", token));
    assert!(requests.contains("github"));
    assert!(requests.finish("github", token));
    assert!(!requests.finish("github", token));
}

#[test]
fn closing_and_reopening_rejects_old_reply_for_the_same_service() {
    let mut requests = Requests::default();
    let old = requests.begin("github").unwrap();
    let other = requests.begin("tavily").unwrap();
    requests.clear();
    let current = requests.begin("github").unwrap();
    assert_ne!(old, current);
    assert!(!requests.finish("github", old));
    assert!(!requests.finish("tavily", other));
    assert!(requests.contains("github"));
    assert!(requests.finish("github", current));
}

#[test]
fn status_read_started_before_authentication_finishes_cannot_replace_the_result() {
    let mut requests = Requests::default();
    let token = requests.begin("github").unwrap();
    let (revision, sequence) = requests.begin_read();
    assert!(requests.finish("github", token));
    assert!(!requests.accept_read(revision, sequence));
    let (revision, sequence) = requests.begin_read();
    assert!(requests.accept_read(revision, sequence));
}

#[test]
fn status_reads_that_finish_out_of_order_preserve_the_newest_snapshot() {
    let mut requests = Requests::default();
    let (old_revision, old_sequence) = requests.begin_read();
    let (new_revision, new_sequence) = requests.begin_read();
    assert!(requests.accept_read(new_revision, new_sequence));
    assert!(!requests.accept_read(old_revision, old_sequence));
    let (revision, sequence) = requests.begin_read();
    requests.clear();
    assert!(!requests.accept_read(revision, sequence));
}

#[test]
fn failed_service_does_not_change_another_services_verified_result() {
    let mut github = Flow::starting();
    let mut tavily = Flow::starting();
    github.result(&json!({"status":"authenticated","next_steps":[]}), false);
    tavily.result(&json!({"status":"authenticating","next_steps":[]}), false);
    tavily.observe(&json!({"status":"failed","error":{"message":"Fixture key rejected"}}));
    assert!(github.phase == Phase::Authenticated);
    assert!(!github.reconcile);
    assert!(tavily.phase == Phase::Failed);
    assert_eq!(tavily.error.as_deref(), Some("Fixture key rejected"));
}

#[test]
fn rejected_manual_save_is_not_approved_by_an_older_valid_server_credential() {
    let mut flow = Flow::starting();
    flow.phase = Phase::Failed;
    flow.manual = true;
    flow.reconcile = true;
    flow.error = Some("The submitted credential failed verification".into());
    flow.observe(&json!({"status":"authenticated","error":null}));
    assert!(flow.phase == Phase::Failed);
    assert!(flow.manual);
    assert!(!flow.reconcile);
    assert_eq!(
        flow.error.as_deref(),
        Some("The submitted credential failed verification")
    );
}

#[test]
fn server_snapshot_cannot_approve_a_request_still_starting_or_saving() {
    for phase in [Phase::Starting, Phase::Saving] {
        let mut flow = Flow::starting();
        flow.phase = phase;
        flow.observe(&json!({"status":"authenticated","error":null}));
        assert!(flow.phase == phase);
    }
}

#[test]
fn pending_browser_flow_reports_success_only_after_verified_server_status() {
    let mut flow = Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}), false);
    assert!(flow.phase == Phase::Pending);
    assert!(flow.reconcile);
    flow.observe(&json!({"status":"authenticated","error":null}));
    assert!(flow.phase == Phase::Authenticated);
    assert!(!flow.reconcile);
}
