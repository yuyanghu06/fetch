//! Independent, window-free checks for concurrent authentication and late replies.
use super::{Desktop, Message, Page, client::NativeClient};
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

#[tokio::test]
async fn ordinary_failed_start_creates_console_from_native_startup_evidence() {
    let (_directory,mut desktop)=desktop_fixture().await;
    assert!(desktop.authentication.is_empty());
    let (revision,sequence)=desktop.auth_requests.begin_read();
    let window=desktop.window;
    drop(desktop.update(Message::Servers(revision,sequence,window,Ok(json!({"servers":[{
        "server_id":"github","runtime":{"status":"failed","error":null},
        "authentication":{"status":"unknown","error":null,"startup":{
            "generation":1,"lines":["FATAL: native configuration was not found"],"exit_code":3,"signal":null
        }},"config":{"environment":[]}
    }],"next_steps":[]})))));
    let flow=&desktop.authentication["github"];
    assert!(flow.log.iter().any(|line|line=="[server] FATAL: native configuration was not found"));
    assert!(flow.log.iter().any(|line|line=="[server] Process exited with status 3."));
    assert!(!flow.log.iter().any(|line|line.contains("Starting authentication")),"a status read does not begin a login");
    assert!(!desktop.auth_requests.contains("github"));
}

struct UiFixtureKeys;
impl KeyProvider for UiFixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([83; 32]))
    }
}

pub(super) async fn desktop_fixture() -> (tempfile::TempDir, Desktop) {
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
        discovery_started: false,
        picker_window: None,
        selected_server: String::new(),
        timeout: "120".into(),
        concurrency: "1".into(),
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
        terminals: BTreeMap::new(),
        auth_requests: Requests::default(),
        deletions: BTreeMap::new(),
        opening_folders: BTreeSet::new(),
    };
    (directory, desktop)
}

#[tokio::test]
async fn desktop_install_stays_empty_until_a_location_is_chosen() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Page(Page::Install)));
    assert!(!desktop.discovery_started);
    assert!(!desktop.busy);
    assert!(desktop.discovery.is_null());
    drop(desktop.update(Message::Discover));
    assert!(!desktop.discovery_started, "blank paths must not discover global defaults");
    let window = desktop.window.unwrap();
    desktop.picker_window = Some(window);
    drop(desktop.update(Message::ConfigFileChosen(window, Ok(None))));
    assert!(!desktop.discovery_started, "cancelling the picker keeps the list empty");
    desktop.picker_window = Some(window);
    drop(desktop.update(Message::ConfigFileChosen(window, Ok(Some("/fixture/chosen/.codex".into())))));
    assert_eq!(desktop.config_path, "/fixture/chosen/.codex");
    assert!(desktop.discovery_started);
    assert!(desktop.busy, "choosing a location immediately starts discovery");
    drop(desktop.update(Message::Discovery(Ok(json!({"configs":[],"next_steps":[]})))));
    drop(desktop.update(Message::Page(Page::Servers)));
    drop(desktop.update(Message::Page(Page::Install)));
    assert!(!desktop.busy, "revisiting Install must preserve the discovery result");
}

#[tokio::test]
async fn desktop_typed_location_discovers_explicitly() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::ConfigPath("/fixture/another-home/.codex".into())));
    assert!(!desktop.busy);
    drop(desktop.update(Message::Discover));
    assert!(desktop.discovery_started);
    assert!(desktop.busy);
    assert_eq!(desktop.config_path, "/fixture/another-home/.codex");
}

#[tokio::test]
async fn desktop_explicit_discovery_waits_for_incompatible_operations() {
    for blocked in ["busy", "picker", "installation"] {
        let (_directory, mut desktop) = desktop_fixture().await;
        desktop.config_path = "/fixture/chosen/.codex".into();
        match blocked {
            "busy" => desktop.busy = true,
            "picker" => desktop.picker_window = desktop.window,
            "installation" => desktop.job = Some("fixture-installation".into()),
            _ => unreachable!(),
        }
        drop(desktop.update(Message::Discover));
        assert!(!desktop.discovery_started, "must not start during {blocked}");
    }
}

#[tokio::test]
async fn desktop_folder_results_keep_installed_rows_out_of_select_all() {
    let (_directory, mut desktop) = desktop_fixture().await;
    let path = "/fixture/chosen-home/.codex/config.toml";
    drop(desktop.update(Message::Discovery(Ok(json!({"configs":[{"path":path,"servers":[
        {"name":"existing","status":"supported","installed":true,"installed_server_id":"saved-id","selected_by_default":false},
        {"name":"available","status":"supported","installed":false,"selected_by_default":true}
    ]}],"next_steps":[]})))));
    assert_eq!(desktop.selected, BTreeSet::from([(path.to_owned(), "available".to_owned())]));
    drop(desktop.update(Message::SelectAllServers));
    assert!(desktop.selected.is_empty(), "Unselect all clears the already selected available row");
    drop(desktop.update(Message::SelectAllServers));
    assert_eq!(desktop.selected, BTreeSet::from([(path.to_owned(), "available".to_owned())]));
    drop(desktop.update(Message::Select(path.into(), "existing".into(), true)));
    assert_eq!(desktop.selected_for_install(), vec![(path.to_owned(), "available".to_owned())]);
    assert!(desktop.job.is_none());
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
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    assert!(desktop.authentication["tavily"].phase == Phase::Authenticated);
    assert!(desktop.auth_requests.contains("github"));
    drop(desktop.update(Message::AuthFinished(
        github,
        "github".into(),
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
        Ok(json!({"status":"authenticated","next_steps":[]})),
    )));
    assert!(desktop.authentication["github"].phase == Phase::Starting);
    assert!(desktop.auth_requests.contains("github"));
}

#[tokio::test]
async fn desktop_folder_opening_leaves_other_authentication_responsive_and_reports_errors() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    drop(desktop.update(Message::OpenFolder("tavily".into())));
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.opening_folders.contains("tavily"));
    assert!(!desktop.busy);
    drop(desktop.update(Message::Authenticate("tavily".into())));
    assert!(desktop.auth_requests.contains("tavily"));
    let sequence=desktop.auth_requests.latest_sequence();
    drop(desktop.update(Message::OpenFolder("tavily".into())));
    assert_eq!(desktop.auth_requests.latest_sequence(),sequence);
    let window=desktop.window;
    drop(desktop.update(Message::FolderOpened("tavily".into(),window,Err("Native folder location is unsupported".into()))));
    assert!(!desktop.opening_folders.contains("tavily"));
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.auth_requests.contains("tavily"));
    assert_eq!(desktop.error.as_deref(),Some("Native folder location is unsupported"));
}

#[tokio::test]
async fn desktop_folder_opening_rejects_unknown_targets_and_replies_from_a_closed_window() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::OpenFolder("unknown".into())));
    assert!(desktop.opening_folders.is_empty());
    drop(desktop.update(Message::OpenFolder("github".into())));
    let old_window=desktop.window;
    desktop.close_authentication();
    desktop.window=Some(iced::window::Id::unique());
    drop(desktop.update(Message::OpenFolder("github".into())));
    drop(desktop.update(Message::FolderOpened("github".into(),old_window,Ok(json!({"server_id":"github","status":"opened"})))));
    assert!(desktop.opening_folders.contains("github"));
    assert!(desktop.notice.is_empty());
    let window=desktop.window;
    drop(desktop.update(Message::FolderOpened("github".into(),window,Ok(json!({"server_id":"github","status":"opened"})))));
    assert!(!desktop.opening_folders.contains("github"));
    assert_eq!(desktop.notice,"Server folder open request completed.");
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
    github.result(&json!({"status":"authenticated","next_steps":[]}));
    tavily.result(&json!({"status":"authenticating","next_steps":[]}));
    tavily.observe(&json!({"status":"failed","error":{"message":"Fixture key rejected"}}));
    assert!(github.phase == Phase::Authenticated);
    assert!(!github.reconcile);
    assert!(tavily.phase == Phase::Failed);
    assert_eq!(tavily.error.as_deref(), Some("Fixture key rejected"));
}

#[test]
fn generic_ok_is_not_proof_of_authentication() {
    let mut flow = Flow::starting();
    flow.result(&json!({"status":"ok","next_steps":[]}));
    assert!(flow.phase == Phase::Failed);
    assert!(flow.error.is_some());
}

#[test]
fn server_snapshot_cannot_approve_a_request_still_starting() {
    for phase in [Phase::Starting] {
        let mut flow = Flow::starting();
        flow.phase = phase;
        flow.observe(&json!({"status":"authenticated","error":null}));
        assert!(flow.phase == phase);
    }
}

#[test]
fn pending_browser_flow_reports_success_only_after_verified_server_status() {
    let mut flow = Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    assert!(flow.phase == Phase::Pending);
    assert!(flow.reconcile);
    flow.observe(&json!({"status":"authenticated","error":null}));
    assert!(flow.phase == Phase::Authenticated);
    assert!(!flow.reconcile);
}

#[test]
fn native_login_host_is_updated_while_pending_and_removed_when_finished() {
    let mut flow=Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    assert!(flow.sign_in.is_none());
    flow.observe(&json!({"status":"authenticating","error":null,
        "sign_in":{"host":"accounts.example.com","opened":false}}));
    assert_eq!(flow.sign_in.as_ref().unwrap().host,"accounts.example.com");
    assert!(!flow.sign_in.as_ref().unwrap().opened);
    assert!(flow.reconcile);
    flow.observe(&json!({"status":"authenticating","error":null,
        "sign_in":{"host":"accounts.example.com","opened":true}}));
    assert!(flow.sign_in.as_ref().unwrap().opened);
    flow.observe(&json!({"status":"unknown","error":null,"sign_in":null,
        "last_known":{"status":"authenticated","error":null}}));
    assert!(flow.sign_in.is_none());
    assert!(flow.phase==Phase::NeedsVerification);
    assert!(!flow.reconcile,"a remembered result is not current account verification");
}

#[tokio::test]
async fn pending_local_login_can_open_only_its_offered_host_once_per_request() {
    let (_directory,mut desktop)=desktop_fixture().await;
    let mut flow=Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    flow.observe(&json!({"status":"authenticating","sign_in":{"host":"accounts.example.com","opened":false}}));
    desktop.authentication.insert("github".into(),flow);
    assert!(desktop.auth_active("github"),"normal server changes stay blocked during login");
    assert!(desktop.can_open_sign_in("github"),"host consent remains usable during login");
    assert!(!desktop.can_open_sign_in("tavily"),"another server cannot use this consent action");
    drop(desktop.update(Message::AuthOpenSignIn("github".into())));
    assert!(desktop.auth_requests.contains("github"));
    assert!(!desktop.can_open_sign_in("github"),"do not submit duplicate open requests");
    desktop.auth_requests.clear();
    desktop.authentication.get_mut("github").unwrap().observe(&json!({"status":"authenticating",
        "sign_in":{"host":"accounts.example.com","opened":true}}));
    assert!(!desktop.can_open_sign_in("github"),"an opened page is not opened again");
}

#[tokio::test]
async fn reopening_sign_in_panel_restores_backend_owned_host_consent() {
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.close_authentication();
    assert!(desktop.authentication.is_empty());
    let (revision,sequence)=desktop.auth_requests.begin_read();
    let window=desktop.window;
    drop(desktop.update(Message::Servers(revision,sequence,window,Ok(json!({"servers":[{
        "server_id":"github","authentication":{"status":"authenticating","error":null,
            "sign_in":{"host":"accounts.example.com","opened":false}},"config":{"environment":[]}
    }],"next_steps":[]})))));
    assert!(desktop.can_open_sign_in("github"));
    assert_eq!(desktop.authentication["github"].sign_in.as_ref().unwrap().host,"accounts.example.com");
    assert!(desktop.authentication["github"].reconcile);
    assert!(!desktop.auth_requests.contains("github"),"restoring status must not start a request");
}

#[test]
fn authentication_console_keeps_order_without_repeating_unchanged_polls() {
    let mut flow=Flow::starting();
    assert!(flow.log.is_empty(),"starting alone must not invent service output");
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    let pending_log=flow.log.clone();
    for _ in 0..3 {flow.observe(&json!({"status":"authenticating","error":null,"sign_in":null}));}
    assert_eq!(flow.log,pending_log,"polling the same state must not flood the console");
    let offered=json!({"status":"authenticating","error":null,
        "sign_in":{"host":"accounts.example.com","opened":false}});
    flow.observe(&offered);
    let offered_log=flow.log.clone();
    flow.observe(&offered);
    assert_eq!(flow.log,offered_log,"an unchanged sign-in host is one event");
    assert!(flow.log.is_empty(),"the sign-in control must not create a service log line");
    assert_eq!(flow.sign_in.as_ref().unwrap().host,"accounts.example.com");
    flow.observe(&json!({"status":"authenticating","error":null,
        "sign_in":{"host":"accounts.example.com","opened":true}}));
    assert!(flow.log.is_empty());assert!(flow.sign_in.as_ref().unwrap().opened);
    flow.observe(&json!({"status":"failed","error":{"message":"Generic Fetch failure"},"sign_in":null,
        "startup":{"generation":1,"lines":["Original service error: login rejected"],"exit_code":null,"signal":null}}));
    assert_eq!(flow.log,vec!["[server] Original service error: login rejected"]);
    assert!(!flow.log.join("\n").contains("Generic Fetch failure"));
    flow.read_error=Some("Fetch status read failed after service output".into());flow.record_state();
    assert_eq!(flow.log,vec!["[server] Original service error: login rejected"],"Fetch read failures are fallback only when no service output exists");
}

#[test]
fn authentication_console_preserves_errors_and_keeps_attempts_separate() {
    let mut first=Flow::starting();
    let second=Flow::starting();
    let second_before=second.log.clone();
    first.result(&json!({"status":"authenticating","next_steps":[]}));
    first.read_error=Some("Fixture status read interrupted".into());
    first.record_state();
    let read_failure_log=first.log.clone();
    first.record_state();
    assert_eq!(first.log,read_failure_log,"the same read error is not appended every frame");
    first.observe(&json!({"status":"authenticating","error":null,"sign_in":null}));
    first.phase=Phase::Failed;
    first.error=Some("Fixture request failed".into());
    first.record_state();
    let output=first.log.join("\n");
    assert!(output.contains("Fixture status read interrupted"),"a successful poll must not erase read history");
    assert!(output.contains("Fixture request failed"));
    assert_eq!(second.log,second_before,"another server's console is unchanged");
    first.observe(&json!({"status":"failed","error":{"message":"Fixture request failed"},
        "startup":{"generation":3,"lines":["Original delayed service error"],"exit_code":null,"signal":null}}));
    assert_eq!(first.log,vec!["[server] Original delayed service error"],"delayed service output replaces earlier temporary Fetch fallbacks too");
    let retry=Flow::starting();
    assert!(!retry.log.join("\n").contains("Fixture"),"a new attempt must start a fresh log");
}

#[test]
fn authentication_console_shows_missing_input_names_without_private_values_or_urls() {
    let mut flow=Flow::starting();
    flow.entry="fixture-config-entry".into();
    flow.secret="private-fixture-input-value".into();
    flow.result(&json!({"status":"authenticating","next_steps":[],
        "raw_output":"private-fixture-raw-line"}));
    flow.observe(&json!({"status":"authenticating","error":null,
        "sign_in":{"host":"accounts.example.com","opened":false,
            "url":"https://accounts.example.com/authorize?state=private-fixture-state&code=private-fixture-code"}}));
    flow.observe(&json!({"status":"required","sign_in":null,"error":{
        "message":"Provide GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET in Settings"}}));
    let output=flow.log.join("\n");
    assert!(output.contains("GOOGLE_CLIENT_ID"));
    assert!(output.contains("GOOGLE_CLIENT_SECRET"));
    assert!(!output.contains("accounts.example.com"),"host consent remains in its own control, not a fabricated service log");
    for private in ["private-fixture-input-value","private-fixture-raw-line","private-fixture-state","private-fixture-code","https://","fixture-config-entry"] {
        assert!(!output.contains(private),"console must not collect unrelated private fields: {private}");
    }
}

#[test]
fn startup_console_renders_cumulative_history_without_poll_or_generation_duplicates() {
    let mut flow=Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    let initial=json!({"status":"authenticating","error":null,"startup":{
        "generation":3,"lines":["Reading native settings","Loading account"],
        "exit_code":null,"signal":null}});
    flow.observe(&initial);
    let first_log=flow.log.clone();
    flow.observe(&initial);
    assert_eq!(flow.log,first_log,"the same server-output snapshot must not repeat on each poll");
    flow.observe(&json!({"status":"authenticating","error":null,"startup":{
        "generation":3,"lines":["Reading native settings","Loading account","Loading account","Connecting service"],
        "exit_code":null,"signal":null}}));
    flow.observe(&json!({"status":"authenticating","error":null,"startup":{
        "generation":4,"lines":["Reading native settings","Loading account","Loading account","Connecting service","Reading native settings","Loading account"],
        "exit_code":null,"signal":null}}));
    let server_lines:Vec<_>=flow.log.iter().filter_map(|line|line.strip_prefix("[server] ")).collect();
    assert_eq!(server_lines,vec!["Reading native settings","Loading account","Loading account",
        "Connecting service","Reading native settings","Loading account"],
        "cumulative snapshots preserve intentional repeated lines without duplicating inherited history");
}

#[test]
fn startup_console_retains_actual_exit_status_and_signal_without_poll_duplicates() {
    let mut flow=Flow::starting();
    flow.result(&json!({"status":"authenticating","next_steps":[]}));
    let failed=json!({"status":"failed","error":{"message":"Fixture startup failed"},
        "startup":{"generation":7,"lines":["Required server setting is missing"],
            "exit_code":1,"signal":null}});
    flow.observe(&failed);
    let failure_log=flow.log.clone();
    flow.observe(&failed);
    assert_eq!(flow.log,failure_log,"terminal output and exit status should remain one event each");
    assert!(flow.log.iter().any(|line|line=="[server] Required server setting is missing"));
    assert!(flow.log.iter().any(|line|line.contains("exited with status 1")));
    flow.observe(&json!({"status":"failed","error":{"message":"Fixture process stopped"},
        "startup":{"generation":8,"lines":["Required server setting is missing","Process exited with status 1."],"exit_code":null,"signal":15}}));
    assert!(flow.log.iter().any(|line|line.contains("ended with signal 15")));
    flow.observe(&json!({"status":"unknown","error":null,
        "startup":{"generation":9,"lines":["Required server setting is missing","Process exited with status 1.","Process ended with signal 15.","Server setup finished"],"exit_code":0,"signal":null}}));
    assert!(flow.log.iter().any(|line|line.contains("exited with status 0")),"a successful process exit is still useful console output");
    assert!(flow.log.starts_with(&failure_log),"later runs must not erase the earlier failure evidence");
}

#[tokio::test]
async fn manual_key_save_uses_its_own_request_and_never_claims_authenticated_access() {
    let (_directory, mut desktop) = desktop_fixture().await;
    drop(desktop.update(Message::Authenticate("github".into())));
    drop(desktop.update(Message::Choose("tavily".into())));
    drop(desktop.update(Message::SecretName("api-key".into())));
    drop(desktop.update(Message::SecretValue("disposable-ui-value".into())));
    drop(desktop.update(Message::SaveSecret));
    let token=desktop.auth_requests.latest_sequence();
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.auth_requests.contains("tavily"));
    assert!(desktop.secret_value.is_empty());
    assert!(desktop.authentication["tavily"].phase==Phase::Saving);
    assert!(!desktop.busy);
    drop(desktop.update(Message::CredentialFinished(token,"tavily".into(),true,Ok(json!({"status":"stored","verified":false,"next_steps":[]})))));
    assert!(desktop.authentication["tavily"].phase==Phase::Stored);
    assert!(desktop.auth_requests.contains("github"));
    assert!(desktop.notice.contains("has not been verified"));
    desktop.authentication.get_mut("tavily").unwrap().observe(&json!({"status":"authenticated","error":null}));
    assert!(desktop.authentication["tavily"].phase==Phase::Stored,"a prior account login must not approve this storage operation");
}

#[tokio::test]
async fn inline_key_submission_releases_the_ui_value_before_the_request_finishes() {
    let (_directory, mut desktop) = desktop_fixture().await;
    let mut flow=Flow::starting();flow.phase=Phase::Required;flow.entry="api-key".into();flow.secret="disposable-inline-value".into();
    desktop.authentication.insert("tavily".into(),flow);
    drop(desktop.update(Message::AuthSave("tavily".into())));
    assert!(desktop.authentication["tavily"].secret.is_empty());
    assert!(desktop.authentication["tavily"].phase==Phase::Saving);
    let token=desktop.auth_requests.latest_sequence();
    drop(desktop.update(Message::CredentialFinished(token,"tavily".into(),false,Err("Fixture Keychain write failed".into()))));
    desktop.authentication.get_mut("tavily").unwrap().observe(&json!({"status":"authenticated","error":null}));
    assert!(desktop.authentication["tavily"].phase==Phase::Failed);
    assert_eq!(desktop.authentication["tavily"].error.as_deref(),Some("Fixture Keychain write failed"));
}

#[test]
fn stored_response_without_explicit_unverified_result_does_not_approve_an_edit() {
    let mut flow=Flow::starting();
    flow.stored(&json!({"status":"stored","verified":true}));
    assert!(flow.phase==Phase::Failed);
    flow.observe(&json!({"status":"authenticated","error":null}));
    assert!(flow.phase==Phase::Failed);
}

fn continuous_server_read(desktop:&mut Desktop,servers:serde_json::Value) {
    let (revision,sequence)=desktop.auth_requests.begin_read();let window=desktop.window;
    drop(desktop.update(Message::Servers(revision,sequence,window,Ok(json!({"servers":servers,"next_steps":[]})))));
}

#[tokio::test]
async fn expanded_idle_server_consoles_are_passive_and_keep_reading_without_authentication() {
    let (_directory,mut desktop)=desktop_fixture().await;
    // Constructing an expanded row before its first status response must also
    // construct the shared empty console; it cannot require an auth flow.
    desktop.expanded_servers.insert("github".into());drop(desktop.servers_view());
    assert!(desktop.authentication.is_empty());assert!(!desktop.auth_requests.contains("github"));
    continuous_server_read(&mut desktop,json!([
        {"server_id":"github","runtime":{"status":"ready"},"authentication":{"status":"not_required","error":null},"config":{"environment":[]}},
        {"server_id":"tavily","runtime":{"status":"stopped"},"authentication":{"status":"unknown","error":null},"config":{"environment":[]}}
    ]));
    for id in ["github","tavily"] {
        let flow=&desktop.authentication[id];assert!(!flow.active());assert!(!flow.reconcile);assert!(flow.log.is_empty());
        assert!(!desktop.auth_active(id));assert!(!desktop.auth_requests.contains(id));
        desktop.expanded_servers.insert(id.into());
    }
    drop(desktop.servers_view());assert!(!desktop.busy);
    drop(desktop.update(Message::Tick));
    assert!(desktop.polling.is_some(),"visible passive consoles need ordinary status reads to show startup output");
    assert!(!desktop.auth_requests.contains("github"));assert!(!desktop.auth_requests.contains("tavily"));assert!(!desktop.busy);
}

#[tokio::test]
async fn authenticate_and_collapse_do_not_discard_initial_startup_history() {
    let (_directory,mut desktop)=desktop_fixture().await;
    continuous_server_read(&mut desktop,json!([{"server_id":"github","runtime":{"status":"failed"},
        "authentication":{"status":"failed","error":null,"startup":{"generation":1,"lines":["Initial startup reason"],"exit_code":null,"signal":null}},
        "config":{"environment":[]}}]));
    let before=desktop.authentication["github"].log.clone();
    drop(desktop.update(Message::Authenticate("github".into())));
    assert_eq!(desktop.authentication["github"].log,before);assert!(desktop.auth_requests.contains("github"));
    drop(desktop.update(Message::ToggleServer("github".into())));drop(desktop.update(Message::ToggleServer("github".into())));
    assert_eq!(desktop.authentication["github"].log,before);
    let token=desktop.auth_requests.latest_sequence();
    drop(desktop.update(Message::AuthFinished(token,"github".into(),Ok(json!({"status":"authenticating","next_steps":[]})))));
    let state=json!([{"server_id":"github","runtime":{"status":"starting"},"authentication":{"status":"authenticating","error":null,
        "startup":{"generation":2,"lines":["Initial startup reason","Actual authentication reason","Actual authentication reason"],"exit_code":null,"signal":null}},"config":{"environment":[]}}]);
    continuous_server_read(&mut desktop,state.clone());let complete=desktop.authentication["github"].log.clone();
    continuous_server_read(&mut desktop,state);
    assert_eq!(desktop.authentication["github"].log,complete);
    assert_eq!(complete,vec!["[server] Initial startup reason","[server] Actual authentication reason","[server] Actual authentication reason"]);
}

#[tokio::test]
async fn reopened_window_rehydrates_idle_console_history_from_the_backend_snapshot() {
    let (_directory,mut desktop)=desktop_fixture().await;
    let state=json!([{"server_id":"github","runtime":{"status":"ready"},"authentication":{"status":"not_required","error":null,
        "startup":{"generation":4,"lines":["Startup before the window was opened","Earlier auth output"],"exit_code":null,"signal":null}},"config":{"environment":[]}}]);
    continuous_server_read(&mut desktop,state.clone());let expected=desktop.authentication["github"].log.clone();
    desktop.close_authentication();assert!(desktop.authentication.is_empty());
    continuous_server_read(&mut desktop,state);
    assert_eq!(desktop.authentication["github"].log,expected);assert!(!desktop.auth_active("github"));assert!(!desktop.auth_requests.contains("github"));
}

#[test]
fn manual_storage_verdict_keeps_receiving_service_history_without_claiming_account_access() {
    let mut flow=Flow::starting();flow.result(&json!({"status":"unknown","next_steps":[]}));
    flow.observe(&json!({"status":"unknown","error":null,"startup":{"generation":1,"lines":["Initial service output"],"exit_code":null,"signal":null}}));
    flow.stored(&json!({"status":"stored","verified":false}));
    flow.observe(&json!({"status":"authenticated","error":null,"startup":{"generation":2,"lines":["Initial service output","Later service output"],"exit_code":null,"signal":null}}));
    assert!(flow.phase==Phase::Stored);assert!(flow.storage_result);
    assert_eq!(flow.log,vec!["[server] Initial service output","[server] Later service output"]);
}

#[test]
fn old_history_does_not_hide_a_new_authentication_request_failure() {
    let mut flow=Flow::passive(&json!({"status":"failed","error":{"message":"Older backend summary"},
        "startup":{"generation":1,"lines":["Earlier boot failure"],"exit_code":9,"signal":null}}));
    flow.begin_authentication();flow.phase=Phase::Failed;flow.error=Some("Current Fetch request failed before new service output".into());flow.record_state();
    assert!(flow.log.iter().any(|line|line=="[server] Earlier boot failure"));
    assert!(flow.log.iter().any(|line|line.contains("Current Fetch request failed")),"old logs are not the current request's returned error");
    flow.observe(&json!({"status":"failed","error":{"message":"Older backend summary"},
        "startup":{"generation":1,"lines":["Earlier boot failure"],"exit_code":9,"signal":null}}));
    assert!(flow.log.iter().any(|line|line.contains("Current Fetch request failed")),"an unchanged old backend snapshot must not hide the current request failure");
    flow.observe(&json!({"status":"failed","error":{"message":"Older backend summary"},
        "startup":{"generation":2,"lines":["Earlier boot failure","Process exited with status 9."],"exit_code":null,"signal":null}}));
    assert!(flow.log.iter().any(|line|line.contains("Current Fetch request failed")),"a new capture containing only inherited output/exit has no current service response yet");
    flow.observe(&json!({"status":"failed","error":{"message":"Generic current summary"},
        "startup":{"generation":2,"lines":["Earlier boot failure","Process exited with status 9.","Actual current service rejection"],"exit_code":null,"signal":null}}));
    assert_eq!(flow.log,vec!["[server] Earlier boot failure","[server] Process exited with status 9.","[server] Actual current service rejection"]);
}
