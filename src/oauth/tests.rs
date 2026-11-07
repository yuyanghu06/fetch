//! Disposable app-approval flow; no real Keychain, native dialog or client state.
use super::*;
use crate::{api::AppState, credentials::{CredentialError, KeyProvider}, discovery::DiscoveryService, storage::ServerRepository};
use axum::body::{Body, to_bytes};
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53672";

struct FixtureKeys;
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([73; 32]))
    }
}

fn request(method: &str, path: &str) -> axum::http::request::Builder {
    Request::builder().method(method).uri(path).header(header::HOST, "localhost:53672")
}

async fn document(response: Response) -> String {
    String::from_utf8(to_bytes(response.into_body(), BODY_LIMIT).await.unwrap().to_vec()).unwrap()
}

fn hidden(html: &str, name: &str) -> String {
    html.split(&format!("name=\"{name}\" value=\"")).nth(1).unwrap().split('"').next().unwrap().to_owned()
}

#[tokio::test]
async fn browser_approval_preserves_its_origin_and_exchanges_the_approved_code() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().to_owned();
    let credentials = Arc::new(CredentialStore::new(root.clone(), Arc::new(FixtureKeys)));
    let service = OAuthService::open(ORIGIN.into(), root.clone(), credentials, NativeAuthority::fixture()).unwrap();
    let mut state = AppState::new(ORIGIN.into(), ServerRepository::new(root.clone()), DiscoveryService::new(root.join("unused-home"), None), service.clone()).unwrap();
    state.oauth = Some(service.clone());
    let app = crate::api::router(Arc::new(state));
    let redirect = "http://localhost:45678/callback";
    let response = app.clone().oneshot(request("POST", "/oauth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json!({"client_name":"Disposable Codex", "redirect_uris":[redirect], "token_endpoint_auth_method":"none", "grant_types":["authorization_code","refresh_token"]}).to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let registered: Value = serde_json::from_str(&document(response).await).unwrap();
    let client = registered["client_id"].as_str().unwrap();
    let verifier = "v".repeat(64);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let resource = format!("{ORIGIN}/mcp");
    let query = url::form_urlencoded::Serializer::new(String::new()).extend_pairs([
        ("response_type", "code"), ("client_id", client), ("redirect_uri", redirect),
        ("code_challenge", &challenge), ("code_challenge_method", "S256"),
        ("resource", &resource), ("scope", "fetch:catalog"), ("state", "fixture-state"),
    ]).finish();
    let response = app.clone().oneshot(request("GET", &format!("/oauth/authorize?{query}")).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // A form POST under no-referrer sends Origin:null and is rejected by Fetch.
    // same-origin preserves proof for this form while disclosing nothing externally.
    assert_eq!(response.headers()[header::REFERRER_POLICY], "same-origin");
    let policy = response.headers()[header::CONTENT_SECURITY_POLICY].to_str().unwrap();
    let form_action = policy.split(';').map(str::trim).find(|part| part.starts_with("form-action ")).unwrap();
    assert_eq!(form_action, "form-action 'self' http://localhost:45678");
    let cookie = response.headers()[header::SET_COOKIE].to_str().unwrap().split(';').next().unwrap().to_owned();
    let html = document(response).await;
    let form = url::form_urlencoded::Serializer::new(String::new()).extend_pairs([
        ("request_id", hidden(&html, "request_id")), ("csrf", hidden(&html, "csrf")),
        ("decision", "allow".into()), ("scope", "fetch:catalog".into()),
    ]).finish();
    for origin in ["null", "https://other.example"] {
        let response = app.clone().oneshot(request("POST", "/oauth/authorize")
            .header(header::ORIGIN, origin).header(header::COOKIE, &cookie)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(form.clone())).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let response = app.clone().oneshot(request("POST", "/oauth/authorize")
        .header(header::ORIGIN, ORIGIN).header(header::COOKIE, &cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(form)).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers()[header::REFERRER_POLICY], "no-referrer");
    let callback = Url::parse(response.headers()[header::LOCATION].to_str().unwrap()).unwrap();
    let params: BTreeMap<_,_> = callback.query_pairs().into_owned().collect();
    assert_eq!(params["state"], "fixture-state");
    let form = url::form_urlencoded::Serializer::new(String::new()).extend_pairs([
        ("grant_type", "authorization_code"), ("client_id", client), ("code", &params["code"]),
        ("redirect_uri", redirect), ("code_verifier", &verifier), ("resource", &resource),
    ]).finish();
    let response = app.oneshot(request("POST", "/oauth/token")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(form)).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let token: Value = serde_json::from_str(&document(response).await).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, format!("Bearer {}", token["access_token"].as_str().unwrap()).parse().unwrap());
    assert!(service.authorize(&headers).unwrap().has_scope("fetch:catalog"));
}
