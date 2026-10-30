//! Independent auth acceptance using encrypted disposable storage and a fake service.
//! These tests never run a managed process, native prompt, Keychain or real provider.
use super::*;
use crate::{
    api::{AppState, router},
    authority::{AuthorizationError, Authorizer},
    credentials::{CredentialError, KeyProvider},
    discovery::DiscoveryService,
};
use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::State,
    http::{HeaderMap, Method, Request, StatusCode, header},
    response::Response,
};
use http_body_util::BodyExt;
use tempfile::TempDir;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tower::ServiceExt;
use zeroize::Zeroizing;

struct FixtureKeys;
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([71; 32]))
    }
}

struct FixtureAuthority;
impl Authorizer for FixtureAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        if headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            != Some("Bearer fixture-caller")
        {
            return Err(AuthorizationError::Unauthenticated);
        }
        Ok(Caller::from_verified_scopes(["fetch:manage".to_owned()]))
    }
}

pub(super) struct Observed {
    pub(super) authorization: String,
    pub(super) method: Method,
    pub(super) headers: HeaderMap,
    pub(super) body: Vec<u8>,
    pub(super) respond: oneshot::Sender<StatusCode>,
}
pub(super) struct MockProvider {
    pub(super) endpoint: String,
    pub(super) requests: mpsc::UnboundedReceiver<Observed>,
    task: JoinHandle<()>,
}
async fn fake_service(
    State(requests): State<mpsc::UnboundedSender<Observed>>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, Json<Value>) {
    let (respond, response) = oneshot::channel();
    requests
        .send(Observed {
            authorization: headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_owned(),
            method,
            headers,
            body: body.to_vec(),
            respond,
        })
        .unwrap();
    let status = response.await.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (
        status,
        Json(json!({"id":"disposable-account","data":{"viewer":{"id":"disposable-account"}}})),
    )
}
impl MockProvider {
    pub(super) async fn new() -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let (requests, receiver) = mpsc::unbounded_channel();
        let app = Router::new()
            .route("/", axum::routing::get(fake_service).post(fake_service))
            .with_state(requests);
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            endpoint,
            requests: receiver,
            task,
        }
    }
    pub(super) async fn next(&mut self) -> Observed {
        tokio::time::timeout(Duration::from_secs(3), self.requests.recv())
            .await
            .expect("expected a request to the disposable authenticated service")
            .expect("mock service closed")
    }
    pub(super) fn no_more_requests(&mut self) {
        assert!(self.requests.try_recv().is_err());
    }
}
impl Drop for MockProvider {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub(super) struct Fixture {
    _directory: TempDir,
    pub(super) manager: Arc<Manager>,
    pub(super) app: Router,
}
impl Fixture {
    pub(super) fn new(endpoint: &str) -> Self {
        Self::with_profile(endpoint, None)
    }
    pub(super) fn with_profile(endpoint: &str, profile: Option<super::verification::Profile>) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("state");
        let repository = ServerRepository::new(root.clone());
        let store = Arc::new(CredentialStore::new(root.clone(), Arc::new(FixtureKeys)));
        let installer = Installer::new(
            root,
            DiscoveryService::new(directory.path().join("empty-home"), None),
        );
        let mut manager = Manager::new(repository.clone(), store, Runner::new(), installer);
        let mutable = Arc::get_mut(&mut manager).unwrap();
        mutable.verification_test_endpoint = Some(endpoint.to_owned());
        mutable.verification_test_profile = profile;
        mutable.http = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        let mut state = AppState::new(
            "https://localhost:53672".into(),
            repository,
            DiscoveryService::new(directory.path().join("empty-home"), None),
            Arc::new(FixtureAuthority),
        )
        .unwrap();
        state.manager = Some(manager.clone());
        let app = router(Arc::new(state));
        Self {
            _directory: directory,
            manager,
            app,
        }
    }
    pub(super) fn local(&self, id: &str, package: &str, bindings: &[(&str, &str)], manual: Value) {
        let record = json!({
            "server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
            "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
            "config":{"kind":"npm","package":{"name":package,"requested_version":null,"installed_version":"1.0.0"},
                "executable":"/disposable/not-executed","args":[],"working_directory":null,
                "environment":bindings.iter().map(|(name,reference)|json!({"name":name,"credential_ref":reference})).collect::<Vec<_>>()},
            "credentials":bindings.iter().map(|(_,reference)|json!({"name":reference,"status":"stored"})).collect::<Vec<_>>()
        });
        self.manager.repository.save(id, &record).unwrap();
        self.store(id, manual, Value::Null);
    }
    pub(super) fn remote_github(&self, id: &str, manual: Value, oauth: &str) {
        let record = json!({
            "server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
            "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
            "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
                "connection":{"url":"https://api.githubcopilot.com/mcp/","transport":"streamable_http",
                    "headers":[{"name":"Authorization","credential_ref":"manual-key"}]}},
            "credentials":[{"name":"manual-key","status":"stored"}]
        });
        self.manager.repository.save(id, &record).unwrap();
        self.store(
            id,
            manual,
            json!({"provider":"github","access_token":oauth}),
        );
    }
    pub(super) fn store(&self, id: &str, manual: Value, oauth: Value) {
        self.manager
            .credentials
            .save(
                id,
                &SecretDocument::new(
                    serde_json::to_vec(&json!({"manual":manual,"oauth":oauth})).unwrap(),
                ),
            )
            .unwrap();
    }
    pub(super) fn post(&self, id: &str) -> JoinHandle<(StatusCode, Value)> {
        let app = self.app.clone();
        let request = Request::builder()
            .method("POST")
            .uri(format!("/api/v1/servers/{id}/authenticate"))
            .header(header::HOST, "localhost:53672")
            .header(header::AUTHORIZATION, "Bearer fixture-caller")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{}"))
            .unwrap();
        tokio::spawn(async move { decode(app.oneshot(request).await.unwrap()).await })
    }
    pub(super) fn save(&self, id: &str, patch: Value) -> JoinHandle<Result<Value, ManagementError>> {
        let manager = self.manager.clone();
        let id = id.to_owned();
        tokio::spawn(async move {
            // This is the shared post-confirmation implementation; native authority
            // remains required by the public save method and HTTP adapter.
            let gate = manager.lock(&id).await;
            let _guard = gate.lock().await;
            manager.save_credentials_locked(&id, patch).await
        })
    }
}
pub(super) async fn decode(response: Response) -> (StatusCode, Value) {
    let status = response.status();
    assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    (
        status,
        serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap(),
    )
}
pub(super) async fn done<T>(task: JoinHandle<T>) -> T {
    tokio::time::timeout(Duration::from_secs(3), task)
        .await
        .unwrap()
        .unwrap()
}
pub(super) fn clean_response(value: &Value, secrets: &[&str]) {
    let serialized = value.to_string();
    for secret in secrets {
        assert!(
            !serialized.contains(secret),
            "API response exposed a fixture credential"
        );
    }
    assert!(value["next_steps"].is_array());
}

#[tokio::test]
async fn post_waits_for_stored_key_check_and_rejects_a_newly_replaced_bad_key() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[("GITHUB_TOKEN", "key")],
        json!({"key":"fixture-secret-old"}),
    );
    let pending = fixture.post("github");
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-secret-old");
    assert!(
        !pending.is_finished(),
        "POST must wait for the authenticated service check"
    );
    check.respond.send(StatusCode::OK).unwrap();
    let (status, result) = done(pending).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["status"], "authenticated");
    clean_response(&result, &["fixture-secret-old"]);

    fixture.store(
        "github",
        json!({"key":"fixture-secret-replaced"}),
        Value::Null,
    );
    let pending = fixture.post("github");
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-secret-replaced");
    check.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    let (status, result) = done(pending).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(result["error"]["code"], "CREDENTIAL_VERIFICATION_FAILED");
    assert_eq!(
        fixture.manager.live.lock().await["github"].auth["status"],
        "failed"
    );
    clean_response(&result, &["fixture-secret-old", "fixture-secret-replaced"]);
}

#[tokio::test]
async fn saving_two_assigned_keys_checks_both_new_saved_values_before_success() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[
            ("GITHUB_TOKEN", "first"),
            ("GITHUB_PERSONAL_ACCESS_TOKEN", "second"),
        ],
        json!({}),
    );
    let pending = fixture.save("github", json!({"set":[{"name":"first","value":"fixture-secret-first"},{"name":"second","value":"fixture-secret-second"}],"remove":[]}));
    let first = provider.next().await;
    assert_eq!(first.authorization, "Bearer fixture-secret-first");
    let stored = fixture.manager.secrets("github").unwrap();
    assert_eq!(stored["manual"]["first"], "fixture-secret-first");
    assert_eq!(stored["manual"]["second"], "fixture-secret-second");
    assert!(!pending.is_finished());
    first.respond.send(StatusCode::OK).unwrap();
    let second = provider.next().await;
    assert_eq!(second.authorization, "Bearer fixture-secret-second");
    assert!(!pending.is_finished());
    second.respond.send(StatusCode::OK).unwrap();
    let result = done(pending).await.unwrap();
    assert_eq!(result["status"], "ok");
    assert_eq!(result["credentials"].as_array().unwrap().len(), 2);
    clean_response(&result, &["fixture-secret-first", "fixture-secret-second"]);
}

#[tokio::test]
async fn one_rejected_member_fails_the_whole_saved_authentication_batch() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[
            ("GITHUB_TOKEN", "first"),
            ("GITHUB_PERSONAL_ACCESS_TOKEN", "second"),
        ],
        json!({}),
    );
    let pending = fixture.save("github", json!({"set":[{"name":"first","value":"fixture-secret-first"},{"name":"second","value":"fixture-secret-bad"}]}));
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    let bad = provider.next().await;
    assert_eq!(bad.authorization, "Bearer fixture-secret-bad");
    bad.respond.send(StatusCode::FORBIDDEN).unwrap();
    let error = done(pending).await.unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    assert_eq!(
        fixture.manager.live.lock().await["github"].auth["status"],
        "failed"
    );
    clean_response(
        &error.body(),
        &["fixture-secret-first", "fixture-secret-bad"],
    );
}

#[tokio::test]
async fn one_credential_referenced_by_two_supported_bindings_is_checked_once() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[
            ("GITHUB_TOKEN", "shared"),
            ("GITHUB_PERSONAL_ACCESS_TOKEN", "shared"),
        ],
        json!({"shared":"fixture-secret-shared"}),
    );
    let pending = fixture.post("github");
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-secret-shared");
    check.respond.send(StatusCode::OK).unwrap();
    assert_eq!(done(pending).await.0, StatusCode::OK);
    provider.no_more_requests();
}

#[tokio::test]
async fn missing_required_credential_cannot_be_approved_by_a_working_sibling() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[
            ("GITHUB_TOKEN", "first"),
            ("GITHUB_PERSONAL_ACCESS_TOKEN", "missing"),
        ],
        json!({"first":"fixture-secret-first"}),
    );
    let (status, result) = done(fixture.post("github")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(result["error"]["code"], "CREDENTIAL_VERIFICATION_FAILED");
    provider.no_more_requests();
}

#[tokio::test]
async fn new_manual_key_is_checked_even_when_an_existing_oauth_token_is_present() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.remote_github(
        "github",
        json!({"manual-key":"fixture-secret-prior"}),
        "fixture-secret-oauth",
    );
    let pending = fixture.save(
        "github",
        json!({"set":[{"name":"manual-key","value":"fixture-secret-new"}]}),
    );
    let manual = provider.next().await;
    assert_eq!(manual.authorization, "Bearer fixture-secret-new");
    manual.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    let error = done(pending).await.unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    provider.no_more_requests();
    clean_response(
        &error.body(),
        &["fixture-secret-new", "fixture-secret-oauth"],
    );
}

#[tokio::test]
async fn valid_new_manual_key_also_requires_the_effective_oauth_context_to_pass() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.remote_github("github", json!({}), "fixture-secret-oauth");
    let pending = fixture.save(
        "github",
        json!({"set":[{"name":"manual-key","value":"fixture-secret-new"}]}),
    );
    let manual = provider.next().await;
    assert_eq!(manual.authorization, "Bearer fixture-secret-new");
    manual.respond.send(StatusCode::OK).unwrap();
    let oauth = provider.next().await;
    assert_eq!(oauth.authorization, "Bearer fixture-secret-oauth");
    oauth.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert_eq!(
        done(pending).await.unwrap_err().code,
        "CREDENTIAL_VERIFICATION_FAILED"
    );
}

#[tokio::test]
async fn unbound_values_and_removal_only_changes_never_report_unchecked_success() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[("GITHUB_TOKEN", "key")],
        json!({"key":"fixture-secret-existing"}),
    );
    let error = done(fixture.save(
        "github",
        json!({"set":[{"name":"unbound","value":"fixture-secret-unbound"}]}),
    ))
    .await
    .unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    clean_response(
        &error.body(),
        &["fixture-secret-existing", "fixture-secret-unbound"],
    );
    let error = done(fixture.save("github", json!({"remove":["key"]})))
        .await
        .unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    assert!(
        fixture.manager.secrets("github").unwrap()["manual"]
            .get("key")
            .is_none()
    );
    provider.no_more_requests();
}

#[tokio::test]
async fn separate_services_finish_independently_while_one_check_is_still_pending() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[("GITHUB_TOKEN", "key")],
        json!({"key":"fixture-secret-github"}),
    );
    fixture.local(
        "tavily",
        "tavily-mcp",
        &[("TAVILY_API_KEY", "key")],
        json!({"key":"fixture-secret-tavily"}),
    );
    let github = fixture.post("github");
    let github_check = provider.next().await;
    assert_eq!(github_check.authorization, "Bearer fixture-secret-github");
    let tavily = fixture.post("tavily");
    let tavily_check = provider.next().await;
    assert_eq!(tavily_check.authorization, "Bearer fixture-secret-tavily");
    tavily_check.respond.send(StatusCode::OK).unwrap();
    let (status, body) = done(tavily).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "authenticated");
    assert!(
        !github.is_finished(),
        "another service must finish without waiting for GitHub"
    );
    github_check.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert_eq!(done(github).await.0, StatusCode::UNPROCESSABLE_ENTITY);
    let live = fixture.manager.live.lock().await;
    assert_eq!(live["tavily"].auth["status"], "authenticated");
    assert_eq!(live["github"].auth["status"], "failed");
}

#[tokio::test]
async fn same_service_requests_never_run_simultaneous_checks() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[("GITHUB_TOKEN", "key")],
        json!({"key":"fixture-secret-github"}),
    );
    let first = fixture.post("github");
    let first_check = provider.next().await;
    let second = fixture.post("github");
    assert!(
        tokio::time::timeout(Duration::from_millis(100), provider.requests.recv())
            .await
            .is_err(),
        "same server started a second concurrent provider check"
    );
    assert!(!second.is_finished());
    first_check.respond.send(StatusCode::OK).unwrap();
    assert_eq!(done(first).await.0, StatusCode::OK);
    provider.next().await.respond.send(StatusCode::OK).unwrap();
    assert_eq!(done(second).await.0, StatusCode::OK);
}

#[tokio::test]
async fn repeated_post_joins_an_existing_pending_browser_flow_without_new_checks() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local(
        "github",
        "@modelcontextprotocol/server-github",
        &[("GITHUB_TOKEN", "key")],
        json!({}),
    );
    let mut live = Live::default();
    live.authenticating = true;
    live.auth_deadline = now() + 600;
    live.auth_attempt = Some("fixture-attempt".into());
    live.auth = json!({"status":"authenticating","error":null});
    fixture
        .manager
        .live
        .lock()
        .await
        .insert("github".into(), live);
    let first = fixture.post("github");
    let second = fixture.post("github");
    for pending in [first, second] {
        let (status, body) = done(pending).await;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body["status"], "authenticating");
    }
    assert_eq!(
        fixture.manager.live.lock().await["github"]
            .auth_attempt
            .as_deref(),
        Some("fixture-attempt")
    );
    provider.no_more_requests();
}
