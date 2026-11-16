//! Focused native-approval routes using disposable files, fake keys and fake Mac confirmation.
use super::*;
use crate::{
    credentials::{CredentialError, KeyProvider},
    native_auth::NativeSettingsSession,
};
use axum::body::Body;
use tower::ServiceExt;
use std::os::unix::fs::PermissionsExt;

const ORIGIN: &str = "https://localhost:53672";
const REQUESTS: &str = "/api/v1/connections/requests";

struct Keys;
impl KeyProvider for Keys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([84; 32]))
    }
}
struct Fixture {
    root: tempfile::TempDir,
    service: Arc<OAuthService>,
    session: NativeSettingsSession,
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(),std::fs::Permissions::from_mode(0o700)).unwrap();
        let native = NativeAuthority::fixture();
        let session = native.open_settings();
        let credentials = Arc::new(CredentialStore::new(root.path().to_owned(), Arc::new(Keys)));
        let service =
            OAuthService::open(ORIGIN.into(), root.path().to_owned(), credentials, native).unwrap();
        Self {
            root,
            service,
            session,
        }
    }
    async fn json(
        &self,
        method: &str,
        path: &str,
        value: Option<Value>,
        native: bool,
    ) -> (StatusCode, Value) {
        let mut request = Request::builder()
            .method(method)
            .uri(path)
            .header(header::HOST, "localhost:53672");
        if native {
            request = request.header("x-fetch-settings-capability", self.session.header_value());
        }
        let body = match value {
            Some(value) => {
                request = request.header(header::CONTENT_TYPE, "application/json");
                Body::from(value.to_string())
            }
            None => Body::empty(),
        };
        let response = super::super::router(self.service.clone())
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap();
        assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
        let status = response.status();
        let bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
        let value = serde_json::from_slice(&bytes).unwrap();
        (status, value)
    }
    async fn pending(&self, name: &str) -> Value {
        let (status, value) = self
            .json("POST", REQUESTS, Some(json!({"client_name":name})), false)
            .await;
        assert_eq!(status, StatusCode::ACCEPTED);
        value
    }
    async fn allow(&self, pending: &Value, scopes: &[&str]) -> Value {
        let id = pending["request_id"].as_str().unwrap();
        let (status, value) = self
            .json(
                "POST",
                &format!("{REQUESTS}/{id}/decision"),
                Some(json!({"decision":"allow","scopes":scopes})),
                true,
            )
            .await;
        assert_eq!(status, StatusCode::OK, "{value}");
        assert_eq!(value["status"], "approved");
        let (status, grant) = self
            .json(
                "POST",
                &format!("{REQUESTS}/{id}/claim"),
                Some(json!({"claim_secret":pending["claim_secret"]})),
                false,
            )
            .await;
        assert_eq!(status, StatusCode::OK, "{grant}");
        grant
    }
    async fn form(&self, path: &str, pairs: &[(&str, &str)]) -> (StatusCode, Value) {
        let form = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(pairs.iter().copied())
            .finish();
        let request = Request::builder()
            .method("POST")
            .uri(path)
            .header(header::HOST, "localhost:53672")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(form))
            .unwrap();
        let response = super::super::router(self.service.clone())
            .oneshot(request)
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
        (
            status,
            if bytes.is_empty() {
                Value::Null
            } else {
                serde_json::from_slice(&bytes).unwrap()
            },
        )
    }
}
fn access(grant: &Value) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", grant["access_token"].as_str().unwrap())
            .parse()
            .unwrap(),
    );
    headers
}

#[tokio::test]
async fn project_history_approval_and_refresh_keep_the_exact_project_resource() {
    let fixture=Fixture::new();
    let folder=fixture.root.path().join("source-project");
    let other_folder=fixture.root.path().join("other-project");
    std::fs::create_dir_all(&folder).unwrap();std::fs::create_dir_all(&other_folder).unwrap();
    let (project,_)=fixture.service.projects.register(folder.to_str().unwrap(),Some("Source project")).unwrap();
    let (other,_)=fixture.service.projects.register(other_folder.to_str().unwrap(),None).unwrap();
    let (status,pending)=fixture.json("POST",REQUESTS,Some(json!({"client_name":"Codex","project_id":project.id,"scopes":["fetch:sessions:read"]})),false).await;
    assert_eq!(status,StatusCode::ACCEPTED);
    assert_eq!(pending["project"]["folder_path"],project.folder_path.to_str().unwrap());
    let grant=fixture.allow(&pending,&["fetch:sessions:read"]).await;
    let resource=format!("{ORIGIN}/mcp/projects/{}",project.id);
    assert_eq!(grant["resource"],resource);
    let caller=fixture.service.authorize(&access(&grant)).unwrap();
    assert_eq!(caller.project_id(),Some(project.id.as_str()));
    assert_eq!(caller.project_folder(),Some(project.folder_path.as_path()));
    assert_eq!(caller.resource(),Some(resource.as_str()));
    assert!(caller.has_scope("fetch:sessions:read"));
    assert!(!caller.has_scope("fetch:tools:call"));
    let other_resource=format!("{ORIGIN}/mcp/projects/{}",other.id);
    let pairs=[("grant_type","refresh_token"),("client_id",grant["client_id"].as_str().unwrap()),("refresh_token",grant["refresh_token"].as_str().unwrap()),("resource",other_resource.as_str())];
    let (status,error)=fixture.form("/oauth/token",&pairs).await;
    assert_eq!(status,StatusCode::BAD_REQUEST);assert_eq!(error["error"],"invalid_grant");
    let (status,renewed)=fixture.form("/oauth/token",&[("grant_type","refresh_token"),("client_id",grant["client_id"].as_str().unwrap()),("refresh_token",grant["refresh_token"].as_str().unwrap()),("resource",resource.as_str())]).await;
    assert_eq!(status,StatusCode::OK);assert_eq!(renewed["project_id"],project.id);
    assert_eq!(fixture.service.authorize(&access(&renewed)).unwrap().project_id(),Some(project.id.as_str()));
    let (status,_)=fixture.json("POST",REQUESTS,Some(json!({"client_name":"Global client","scopes":["fetch:sessions:read"]})),false).await;
    assert_eq!(status,StatusCode::BAD_REQUEST);
    // Reopen the same task-owned encrypted store with the same fake keys, as a
    // Fetch restart would, then revoke through the actual product endpoint.
    let reopened=OAuthService::open(ORIGIN.into(),fixture.root.path().to_owned(),
        Arc::new(CredentialStore::new(fixture.root.path().to_owned(),Arc::new(Keys))),fixture.service.native.clone()).unwrap();
    let restored=reopened.authorize(&access(&renewed)).unwrap();
    assert_eq!(restored.project_id(),Some(project.id.as_str()));
    assert_eq!(restored.project_folder(),Some(project.folder_path.as_path()));
    assert_eq!(restored.resource(),Some(resource.as_str()));
    assert!(restored.has_scope("fetch:sessions:read"));
    let revoke=url::form_urlencoded::Serializer::new(String::new()).extend_pairs([
        ("client_id",grant["client_id"].as_str().unwrap()),("token",renewed["access_token"].as_str().unwrap()),
    ]).finish();
    let response=super::super::router(reopened.clone()).oneshot(Request::builder().method("POST").uri("/oauth/revoke")
        .header(header::HOST,"localhost:53672").header(header::CONTENT_TYPE,"application/x-www-form-urlencoded")
        .body(Body::from(revoke)).unwrap()).await.unwrap();
    assert_eq!(response.status(),StatusCode::OK);
    assert!(reopened.authorize(&access(&renewed)).is_err());
}

#[tokio::test]
async fn native_popup_approval_issues_only_selected_access_to_the_request_owner() {
    let fixture = Fixture::new();
    let pending = fixture.pending("Claude Code").await;
    assert!(fixture.service.has_pending_connections());
    assert!(fixture.service.take_connection_request());
    assert!(!fixture.service.take_connection_request());
    assert_eq!(pending["status"], "pending");
    assert!(
        !pending["scopes"]
            .as_array()
            .unwrap()
            .contains(&json!("fetch:manage"))
    );
    assert!(
        fixture
            .service
            .state
            .lock()
            .unwrap()
            .persistent
            .families
            .is_empty()
    );
    let id = pending["request_id"].as_str().unwrap();
    let claim = format!("{REQUESTS}/{id}/claim");
    assert_eq!(
        fixture
            .json(
                "POST",
                &claim,
                Some(json!({"claim_secret":pending["claim_secret"]})),
                false
            )
            .await
            .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(
        fixture.json("GET", REQUESTS, None, false).await.0,
        StatusCode::UNAUTHORIZED
    );
    let (status, safe) = fixture.json("GET", REQUESTS, None, true).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(safe["requests"][0]["code"], pending["code"]);
    assert!(
        !safe
            .to_string()
            .contains(pending["claim_secret"].as_str().unwrap())
    );
    assert_eq!(
        fixture
            .json(
                "POST",
                &format!("{REQUESTS}/{id}/decision"),
                Some(json!({"decision":"allow","scopes":["fetch:catalog"]})),
                false
            )
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        fixture
            .json(
                "POST",
                &claim,
                Some(json!({"claim_secret":random_token()})),
                false
            )
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let grant = fixture.allow(&pending, &["fetch:catalog"]).await;
    let caller = fixture.service.authorize(&access(&grant)).unwrap();
    assert!(caller.has_scope("fetch:catalog"));
    assert!(!caller.has_scope("fetch:tools:call"));
    assert!(!caller.is_native_settings());
    assert!(!fixture.service.has_pending_connections());
    assert_eq!(
        fixture
            .json(
                "POST",
                &claim,
                Some(json!({"claim_secret":pending["claim_secret"]})),
                false
            )
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let bearer_request = Request::builder()
        .uri(REQUESTS)
        .header(
            header::AUTHORIZATION,
            access(&grant)[header::AUTHORIZATION].clone(),
        )
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        super::super::router(fixture.service.clone())
            .oneshot(bearer_request)
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn separate_client_grants_renew_revoke_and_survive_fetch_restart() {
    let fixture = Fixture::new();
    let claude = fixture
        .allow(
            &fixture.pending("Claude Code").await,
            &["fetch:catalog", "fetch:tools:call"],
        )
        .await;
    let codex = fixture
        .allow(&fixture.pending("Codex").await, &["fetch:catalog"])
        .await;
    assert_ne!(claude["client_id"], codex["client_id"]);
    let claude_owner = fixture
        .service
        .authorize(&access(&claude))
        .unwrap()
        .owner_id()
        .to_owned();
    assert_ne!(
        claude_owner,
        fixture
            .service
            .authorize(&access(&codex))
            .unwrap()
            .owner_id()
    );
    let resource = format!("{ORIGIN}/mcp");
    let (status, refreshed) = fixture
        .form(
            "/oauth/token",
            &[
                ("grant_type", "refresh_token"),
                ("client_id", claude["client_id"].as_str().unwrap()),
                ("refresh_token", claude["refresh_token"].as_str().unwrap()),
                ("resource", &resource),
            ],
        )
        .await;
    assert_eq!(status, StatusCode::OK, "{refreshed}");
    assert_ne!(refreshed["refresh_token"], claude["refresh_token"]);
    assert_eq!(
        fixture
            .service
            .authorize(&access(&refreshed))
            .unwrap()
            .owner_id(),
        claude_owner
    );
    let pending = fixture.pending("Not approved before restart").await;
    fixture.service.shutdown();
    let credentials = Arc::new(CredentialStore::new(
        fixture.root.path().to_owned(),
        Arc::new(Keys),
    ));
    let restarted = OAuthService::open(
        ORIGIN.into(),
        fixture.root.path().to_owned(),
        credentials,
        NativeAuthority::fixture(),
    )
    .unwrap();
    assert!(restarted.authorize(&access(&refreshed)).is_ok());
    assert!(restarted.authorize(&access(&codex)).is_ok());
    assert!(!restarted.has_pending_connections());
    assert!(
        restarted
            .state
            .lock()
            .unwrap()
            .connections
            .0
            .get(pending["request_id"].as_str().unwrap())
            .is_none()
    );
    let revoke = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs([
            ("client_id", claude["client_id"].as_str().unwrap()),
            ("token", refreshed["refresh_token"].as_str().unwrap()),
        ])
        .finish();
    let response = super::super::router(restarted.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/oauth/revoke")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(revoke))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(restarted.authorize(&access(&refreshed)).is_err());
    assert!(restarted.authorize(&access(&codex)).is_ok());
}

#[tokio::test]
async fn denied_or_expired_popup_requests_never_create_credentials() {
    let fixture = Fixture::new();
    let pending = fixture.pending("Declined app").await;
    let id = pending["request_id"].as_str().unwrap();
    let (status, denied) = fixture
        .json(
            "POST",
            &format!("{REQUESTS}/{id}/decision"),
            Some(json!({"decision":"deny","scopes":[]})),
            true,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(denied["status"], "denied");
    let (status, result) = fixture
        .json(
            "POST",
            &format!("{REQUESTS}/{id}/claim"),
            Some(json!({"claim_secret":pending["claim_secret"]})),
            false,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(result["error"]["code"], "CONNECTION_DENIED");
    let expired = fixture.pending("Expired app").await;
    let id = expired["request_id"].as_str().unwrap();
    fixture
        .service
        .state
        .lock()
        .unwrap()
        .connections
        .0
        .get_mut(id)
        .unwrap()
        .expires = now().saturating_sub(1);
    let (status, result) = fixture
        .json(
            "POST",
            &format!("{REQUESTS}/{id}/claim"),
            Some(json!({"claim_secret":expired["claim_secret"]})),
            false,
        )
        .await;
    assert_eq!(status, StatusCode::GONE);
    assert_eq!(result["error"]["code"], "CONNECTION_REQUEST_EXPIRED");
    assert!(
        fixture
            .service
            .state
            .lock()
            .unwrap()
            .persistent
            .families
            .is_empty()
    );
    assert!(
        fixture
            .service
            .state
            .lock()
            .unwrap()
            .persistent
            .clients
            .is_empty()
    );
}
