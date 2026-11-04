//! Independent dispatcher and web-flow checks using saved fixtures and loopback HTTP only.
use super::*;
use super::authentication::{Route, WebPlan};
use super::auth_tests::{Fixture, MockProvider, clean_response, done};
use axum::{Json, Router, body::Bytes, extract::State, http::{Method, StatusCode}};
use tokio::sync::mpsc;

fn remote_record(id: &str) -> Value {
    json!({"server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.fixture.example/mcp","transport":"streamable_http","headers":[]}},
        "credentials":[]})
}

fn plan(provider: Provider, metadata: Option<remote_auth::Metadata>) -> WebPlan {
    WebPlan { provider, authorization:"https://issuer.fixture.example/authorize".into(),
        token:"https://issuer.fixture.example/token".into(), registration:"https://issuer.fixture.example/register".into(),
        resource:if provider == Provider::Railway { None } else { Some("https://mcp.fixture.example/mcp".into()) },
        remote_metadata:metadata }
}

fn metadata(token: &str) -> remote_auth::Metadata {
    remote_auth::Metadata { issuer:"https://issuer.fixture.example/".into(), authorization:"https://issuer.fixture.example/authorize".into(),
        token:token.into(), registration:"https://issuer.fixture.example/register".into(), resource:"https://mcp.fixture.example/mcp".into(),
        scope:Some("read tools".into()), authorization_response_iss_parameter_supported:true }
}

#[test]
fn imported_remote_defaults_to_web_while_explicit_api_credentials_keep_their_handler() {
    let mut record = remote_record("remote");
    record["config"]["connection"]["headers"] = json!([{"name":"X-Custom-Key","credential_ref":"custom-key"}]);
    assert_eq!(authentication::dispatch(&record, Provider::Remote).unwrap(), Route::Web);
    assert_eq!(authentication::dispatch(&record, Provider::Manual).unwrap(), Route::Credentials);
    assert_eq!(authentication::dispatch(&record, Provider::Linear).unwrap(), Route::Web);
    assert_eq!(authentication::dispatch(&record, Provider::Railway).unwrap(), Route::Web);
}

#[test]
fn local_requirements_choose_credentials_without_guessing_a_cli_login_command() {
    let record = json!({"config":{"kind":"local_script","executable":"/private/fixture/server","args":["--token-env","SERVICE_KEY"],
        "environment":[{"name":"SERVICE_KEY","credential_ref":"saved-key"}]}});
    assert_eq!(authentication::dispatch(&record, Provider::Unknown).unwrap(), Route::Credentials);
    let no_requirements = json!({"config":{"kind":"local_script","executable":"/private/fixture/server","args":["login"],"environment":[]}});
    assert_eq!(authentication::dispatch(&no_requirements, Provider::Unknown).unwrap(), Route::Unsupported);
    assert_eq!(authentication::dispatch(&no_requirements, Provider::None).unwrap(), Route::NotRequired);
}

#[tokio::test]
async fn google_accounts_without_assigned_app_inputs_require_settings_without_running_helper_or_rewriting_entries() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    for id in ["google-personal", "google-work"] {
        fixture.local(id, "@a-bonus/google-docs-mcp", &[("GOOGLE_CLIENT_ID", "client-id"), ("GOOGLE_CLIENT_SECRET", "client-secret")],
            json!({"client-id":format!("fixture-{id}-id")}));
        let mut record = fixture.manager.record(id).unwrap();
        record["config"]["package"]["installed_version"] = json!("1.11.3");
        record["config"]["executable"] = json!("/private/disposable/not-executed/node");
        record["config"]["args"] = json!(["/private/disposable/not-executed/google-docs-mcp/dist/index.js"]);
        fixture.manager.repository.save(id, &record).unwrap();
        assert_eq!(authentication::dispatch(&record, Provider::Unknown).unwrap(), Route::LocalHelper);
        let before = fixture.manager.credentials.read(id).unwrap().unwrap();
        let (status, result) = done(fixture.post(id)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(result["status"], "required");
        assert_eq!(fixture.manager.credentials.read(id).unwrap().unwrap().expose(), before.expose());
        clean_response(&result, &[&format!("fixture-{id}-id"), &format!("fixture-{id}-secret")]);
    }
    assert!(fixture.manager.flows.lock().await.is_empty());
    provider.no_more_requests();
}

#[test]
fn railway_uses_common_web_request_shapes_with_only_native_consent_differences() {
    let railway = plan(Provider::Railway, None);
    let redirect = "http://127.0.0.1:12345/oauth/provider/callback";
    let body = railway.registration_body(redirect);
    assert_eq!(body["application_type"], "native");
    assert_eq!(body["redirect_uris"], json!([redirect]));
    assert_eq!(body["token_endpoint_auth_method"], "none");
    assert_eq!(body["grant_types"], json!(["authorization_code", "refresh_token"]));
    let url = railway.authorization_url("fixture-client", redirect, "fixture-challenge", "fixture-state").unwrap();
    let query: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
    assert_eq!(query["response_type"], "code");
    assert_eq!(query["client_id"], "fixture-client");
    assert_eq!(query["redirect_uri"], redirect);
    assert_eq!(query["code_challenge"], "fixture-challenge");
    assert_eq!(query["code_challenge_method"], "S256");
    assert_eq!(query["state"], "fixture-state");
    assert_eq!(query["scope"], railway::SCOPES);
    assert_eq!(query["prompt"], "consent");
    assert!(!query.contains_key("resource"));
}

#[test]
fn linear_and_generic_share_web_requests_while_preserving_resource_and_discovered_scope() {
    let redirect = "https://localhost:53672/oauth/provider/callback";
    for (provider, metadata) in [(Provider::Linear, None), (Provider::Remote, Some(metadata("https://issuer.fixture.example/token")))] {
        let plan = plan(provider, metadata);
        let body = plan.registration_body(redirect);
        assert!(body.get("application_type").is_none());
        assert_eq!(body["redirect_uris"], json!([redirect]));
        let url = plan.authorization_url("fixture-client", redirect, "fixture-challenge", "fixture-state").unwrap();
        let query: BTreeMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(query["resource"], "https://mcp.fixture.example/mcp");
        assert_eq!(query["code_challenge_method"], "S256");
        assert!(!query.contains_key("prompt"));
        if provider == Provider::Remote { assert_eq!(query["scope"], "read tools"); }
        else { assert!(!query.contains_key("scope")); }
    }
}

#[test]
fn common_registration_validation_preserves_named_public_client_rules() {
    for provider in [Provider::Railway, Provider::Linear, Provider::Remote] {
        let plan = plan(provider, None);
        assert_eq!(plan.validate_registration(&json!({"client_id":"fixture-client","token_endpoint_auth_method":"none"})).unwrap(), "fixture-client");
        assert!(plan.validate_registration(&json!({"client_id":"fixture-client","token_endpoint_auth_method":"client_secret_basic"})).is_err());
    }
    assert!(plan(Provider::Linear, None).validate_registration(&json!({"client_id":"fixture-client","client_secret":"fixture-private"})).is_err());
}

#[tokio::test]
async fn two_instances_of_one_server_verify_distinct_saved_keys_and_finish_independently() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    for (id, secret) in [("github-personal", "fixture-personal-key"), ("github-work", "fixture-work-key")] {
        fixture.local(id, "@modelcontextprotocol/server-github", &[("GITHUB_TOKEN", "key")], json!({"key":secret}));
    }
    let personal = fixture.post("github-personal");
    let personal_check = provider.next().await;
    assert_eq!(personal_check.authorization, "Bearer fixture-personal-key");
    let work = fixture.post("github-work");
    let work_check = provider.next().await;
    assert_eq!(work_check.authorization, "Bearer fixture-work-key");
    work_check.respond.send(StatusCode::OK).unwrap();
    assert_eq!(done(work).await.1["status"], "authenticated");
    assert!(!personal.is_finished());
    personal_check.respond.send(StatusCode::UNAUTHORIZED).unwrap();
    assert_eq!(done(personal).await.1["error"]["code"], "CREDENTIAL_VERIFICATION_FAILED");
    let live = fixture.manager.live.lock().await;
    assert_eq!(live["github-work"].auth["status"], "authenticated");
    assert_eq!(live["github-personal"].auth["status"], "failed");
    drop(live);
    assert_eq!(fixture.manager.secrets("github-personal").unwrap()["manual"]["key"], "fixture-personal-key");
    assert_eq!(fixture.manager.secrets("github-work").unwrap()["manual"]["key"], "fixture-work-key");
    provider.no_more_requests();
}

#[tokio::test]
async fn background_web_start_failure_is_visible_without_switching_to_credentials() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.manager.repository.save("web", &remote_record("web")).unwrap();
    fixture.store("web", json!({}), Value::Null);
    // No origin is set: the background start must fail before network/browser work.
    let (status, result) = done(fixture.post("web")).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(result["status"], "authenticating");
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            if fixture.manager.live.lock().await.get("web").is_some_and(|live| live.auth["status"] == "failed") { break; }
            tokio::task::yield_now().await;
        }
    }).await.unwrap();
    let caller = Caller::from_verified_scopes(["fetch:manage".to_owned()]);
    let servers = fixture.manager.servers_for(&caller, vec!["web".into()], None).await.unwrap();
    let failure = &servers["servers"][0]["authentication"]["error"];
    assert_eq!(failure["code"], "SERVER_AUTH_REQUIRED");
    assert_eq!(failure["message"], auth_required().message);
    assert!(fixture.manager.flows.lock().await.is_empty());
    assert_eq!(fixture.manager.secrets("web").unwrap()["oauth"], Value::Null);
    provider.no_more_requests();
}

#[derive(Clone)]
struct TokenRequests(mpsc::UnboundedSender<(Method, Vec<u8>)>);
async fn token_service(State(requests): State<TokenRequests>, method: Method, body: Bytes) -> Json<Value> {
    requests.0.send((method, body.to_vec())).unwrap();
    Json(json!({"access_token":"fixture-access","token_type":"Bearer"}))
}

#[tokio::test]
async fn shared_token_sender_preserves_exchange_and_refresh_form_values() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let (requests, mut received) = mpsc::unbounded_channel();
    let app = Router::new().fallback(token_service).with_state(TokenRequests(requests));
    let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    for refresh in [false, true] {
        let fields = if refresh { vec![("grant_type", "refresh_token"), ("refresh_token", "fixture-refresh"), ("client_id", "fixture-client"), ("resource", "https://mcp.fixture.example/mcp")] }
            else { vec![("grant_type", "authorization_code"), ("code", "fixture-code"), ("client_id", "fixture-client"), ("redirect_uri", "https://localhost:53672/oauth/provider/callback"), ("code_verifier", "fixture-verifier")] };
        let response = authentication::token(&http, &endpoint, None, &fields, refresh).await.unwrap();
        assert_eq!(response["access_token"], "fixture-access");
        let (method, body) = received.recv().await.unwrap();
        assert_eq!(method, Method::POST);
        assert_eq!(url::form_urlencoded::parse(&body).into_owned().collect::<BTreeMap<_, _>>(), fields.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect());
    }
    task.abort();
}

#[tokio::test]
async fn generic_callback_rejects_wrong_issuer_before_any_token_exchange() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.manager.repository.save("web", &remote_record("web")).unwrap();
    fixture.store("web", json!({}), Value::Null);
    let attempt = "fixture-owned-attempt";
    let expiry = now() + 60;
    fixture.manager.live.lock().await.insert("web".into(), Live { revision:2, authenticating:true, auth_attempt:Some(attempt.into()), auth_deadline:expiry, ..Live::default() });
    fixture.manager.flows.lock().await.insert(attempt.into(), ProviderFlow { server:"web".into(),state:attempt.into(),provider:Provider::Remote,
        verifier:"fixture-verifier".into(),client_id:"fixture-client".into(),token_endpoint:provider.endpoint.clone(),
        redirect:"https://localhost:53672/oauth/provider/callback".into(),resource:Some("https://mcp.fixture.example/mcp".into()),
        remote_metadata:Some(metadata(&provider.endpoint)),revision:2,expires:expiry,_callback:None });
    let error = fixture.manager.provider_callback(attempt, Some("fixture-code"), None, Some("https://wrong.fixture.example/")).await.unwrap_err();
    assert_eq!(error.code, "AUTHENTICATION_FAILED");
    assert_eq!(fixture.manager.live.lock().await["web"].auth["status"], "failed");
    assert_eq!(fixture.manager.secrets("web").unwrap()["oauth"], Value::Null);
    provider.no_more_requests();
}

#[tokio::test]
async fn stale_callback_cannot_change_the_newer_attempt_or_its_account() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.manager.repository.save("web", &remote_record("web")).unwrap();
    fixture.store("web", json!({"keep":"fixture-private"}), Value::Null);
    let before = fixture.manager.credentials.read("web").unwrap().unwrap();
    let expiry = now() + 60;
    fixture.manager.live.lock().await.insert("web".into(), Live { revision:3,authenticating:true,auth_attempt:Some("fixture-new-attempt".into()),auth_deadline:expiry,..Live::default() });
    fixture.manager.flows.lock().await.insert("fixture-old-attempt".into(), ProviderFlow { server:"web".into(),state:"fixture-old-attempt".into(),provider:Provider::Remote,
        verifier:"fixture-verifier".into(),client_id:"fixture-client".into(),token_endpoint:provider.endpoint.clone(),
        redirect:"https://localhost:53672/oauth/provider/callback".into(),resource:Some("https://mcp.fixture.example/mcp".into()),
        remote_metadata:Some(metadata(&provider.endpoint)),revision:2,expires:expiry,_callback:None });
    assert_eq!(fixture.manager.provider_callback("fixture-old-attempt", Some("fixture-code"), None, Some("https://issuer.fixture.example/")).await.unwrap_err().code, "AUTHENTICATION_FAILED");
    let live = fixture.manager.live.lock().await;
    assert!(live["web"].authenticating);
    assert_eq!(live["web"].auth_attempt.as_deref(), Some("fixture-new-attempt"));
    drop(live);
    assert_eq!(fixture.manager.credentials.read("web").unwrap().unwrap().expose(), before.expose());
    provider.no_more_requests();
}
