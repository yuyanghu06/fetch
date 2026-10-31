use super::*;
use super::auth_tests::{Fixture, MockProvider, clean_response, done};
use super::verification::Profile;
use axum::{Json, Router, http::StatusCode};

fn pair_profile() -> Profile {
    serde_json::from_value(json!({
        "id":"arbitrary-pair","targets":{"packages":["fixture-pair-mcp"],"hosts":[]},
        "manual":[{"mode":"together","inputs":{
            "session":{"source":"environment","names":["CUSTOM_SESSION_INPUT"]},
            "workspace":{"source":"environment","names":["CUSTOM_WORKSPACE_INPUT"]}
        },"request":{"method":"POST","url":"https://verification.fixture.example/check",
            "body":{"context":{"workspace":null}},"credentials":[
                {"input":"session","placement":"header","name":"Authorization","prefix":"Bearer "},
                {"input":"workspace","placement":"json","pointer":"/context/workspace"}
            ],"success":[{"rule":"string","pointer":"/id","nonempty":true}]
        }}],"oauth":null
    })).unwrap()
}

fn set_document(fixture: &Fixture, id: &str, document: &Value) {
    fixture.manager.credentials.save(id, &SecretDocument::new(serde_json::to_vec(document).unwrap())).unwrap();
}

#[tokio::test]
async fn saving_an_arbitrary_pair_keeps_other_saved_values_and_checks_encrypted_readback() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(pair_profile()));
    fixture.local("custom", "fixture-pair-mcp", &[("CUSTOM_SESSION_INPUT", "saved-session"), ("CUSTOM_WORKSPACE_INPUT", "saved-workspace")], json!({}));
    set_document(&fixture, "custom", &json!({"manual":{"unrelated":"fixture-keep"},"oauth":null,
        "oauth_client":{"registration":{"client_id":"fixture-client"}},"future":{"setting":17}}));
    let pending = fixture.save("custom", json!({"set":[
        {"name":"saved-session","value":"fixture-new-session"},
        {"name":"saved-workspace","value":"fixture-new-workspace"}
    ]}));
    let request = provider.next().await;
    assert_eq!(request.authorization, "Bearer fixture-new-session");
    assert_eq!(serde_json::from_slice::<Value>(&request.body).unwrap()["context"]["workspace"], "fixture-new-workspace");
    let saved = fixture.manager.credentials.read_json("custom").unwrap().unwrap();
    assert_eq!(saved["manual"]["saved-session"], "fixture-new-session");
    assert_eq!(saved["manual"]["saved-workspace"], "fixture-new-workspace");
    assert_eq!(saved["manual"]["unrelated"], "fixture-keep");
    assert_eq!(saved["oauth_client"]["registration"]["client_id"], "fixture-client");
    assert_eq!(saved["future"]["setting"], 17);
    assert!(!pending.is_finished());
    request.respond.send(StatusCode::OK).unwrap();
    let result = done(pending).await.unwrap();
    assert_eq!(result["status"], "ok");
    clean_response(&result, &["fixture-new-session", "fixture-new-workspace", "fixture-keep"]);
    provider.no_more_requests();
}

#[tokio::test]
async fn a_bad_later_manual_value_does_not_write_the_earlier_valid_value() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(pair_profile()));
    fixture.local("custom", "fixture-pair-mcp", &[("CUSTOM_SESSION_INPUT", "saved-session"), ("CUSTOM_WORKSPACE_INPUT", "saved-workspace")],
        json!({"saved-session":"fixture-before"}));
    let before = fixture.manager.credentials.read("custom").unwrap().unwrap();
    let error = done(fixture.save("custom", json!({"set":[
        {"name":"saved-session","value":"fixture-must-not-save"},
        {"name":"saved-workspace","value":17}
    ]}))).await.unwrap_err();
    assert_eq!(error.code, "INVALID_CREDENTIAL_ENTRY");
    assert_eq!(fixture.manager.credentials.read("custom").unwrap().unwrap().expose(), before.expose());
    provider.no_more_requests();
}

#[test]
fn existing_oauth_flows_share_normalization_without_losing_their_saved_ownership() {
    let response = json!({"access_token":"fixture-new-access","token_type":"Bearer","expires_in":3600});
    for provider in [Provider::Linear, Provider::Remote] {
        let result = oauth_document(&response, provider, "fixture-client", "https://issuer.fixture.example/token", Some("https://resource.fixture.example/mcp"), Some("fixture-prior-refresh")).unwrap();
        assert_eq!(result["provider"], provider.name());
        assert_eq!(result["client_id"], "fixture-client");
        assert_eq!(result["token_endpoint"], "https://issuer.fixture.example/token");
        assert_eq!(result["resource"], "https://resource.fixture.example/mcp");
        assert_eq!(result["refresh_token"], "fixture-prior-refresh");
        assert_eq!(result["verified"], false);
    }
    assert!(oauth_document(&response, Provider::Railway, "fixture-client", "https://issuer.fixture.example/token", None, Some("fixture-prior-refresh")).is_err());
    let mut rotating = response;
    rotating["refresh_token"] = json!("fixture-new-refresh");
    assert_eq!(oauth_document(&rotating, Provider::Railway, "fixture-client", "https://issuer.fixture.example/token", None, Some("fixture-prior-refresh")).unwrap()["refresh_token"], "fixture-new-refresh");
}

fn callback_profile(login: &str) -> Profile {
    serde_json::from_value(json!({
        "id":"callback-fixture","targets":{"packages":[],"hosts":["mcp.fixture.example"]},"login":login,
        "manual":[],"oauth":{"method":"GET","url":"https://verification.fixture.example/check",
            "credentials":[{"input":"token","placement":"header","name":"Authorization","prefix":"Bearer "}],
            "success":[{"rule":"string","pointer":"/id","nonempty":true}]
        }
    })).unwrap()
}

fn remote_record() -> Value {
    json!({"server_id":"callback","name":"callback","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":"https://mcp.fixture.example/mcp","transport":"streamable_http","headers":[]}},
        "credentials":[]})
}

#[tokio::test]
async fn verified_marker_update_preserves_manual_values_registration_and_extra_oauth_fields() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(callback_profile("standard")));
    let record = remote_record();
    fixture.manager.repository.save("callback", &record).unwrap();
    let before = json!({"manual":{"custom":"fixture-private"},"oauth":{"provider":"remote","access_token":"fixture-access","refresh_token":"fixture-refresh","custom":{"keep":true},"verified":false},
        "oauth_client":{"registration":{"client_id":"fixture-client"}},"extra":"preserve"});
    set_document(&fixture, "callback", &before);
    fixture.manager.mark_oauth_verified(&record).unwrap();
    let mut expected = before;
    expected["oauth"]["verified"] = json!(true);
    assert_eq!(fixture.manager.credentials.read_json("callback").unwrap().unwrap(), expected);
    provider.no_more_requests();
}

#[tokio::test]
async fn callback_saves_through_shared_store_and_waits_for_saved_token_check() {
    for status in [StatusCode::OK, StatusCode::UNAUTHORIZED] {
        let mut provider = MockProvider::new().await;
        let fixture = Fixture::with_profile(&provider.endpoint, Some(callback_profile("linear")));
        let record = remote_record();
        fixture.manager.repository.save("callback", &record).unwrap();
        set_document(&fixture, "callback", &json!({"manual":{"other":"fixture-keep"},"oauth":null,
            "oauth_client":{"registration":{"client_id":"fixture-client"}},"future":{"preserve":true}}));
        let token_app = Router::new().fallback(|| async { Json(json!({"access_token":"fixture-callback-access","token_type":"Bearer"})) });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let token_endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let token_task = tokio::spawn(async move { axum::serve(listener, token_app).await.unwrap(); });
        let attempt = "fixture-attempt";
        let expiry = now() + 60;
        fixture.manager.live.lock().await.insert("callback".into(), Live { revision:7,authenticating:true,auth_attempt:Some(attempt.into()),auth_deadline:expiry,..Live::default() });
        fixture.manager.flows.lock().await.insert(attempt.into(), ProviderFlow {
            server:"callback".into(),state:attempt.into(),provider:Provider::Linear,verifier:"fixture-verifier".into(),
            client_id:"fixture-client".into(),token_endpoint,redirect:"https://localhost:53672/oauth/provider/callback".into(),
            resource:Some("https://mcp.fixture.example/mcp".into()),remote_metadata:None,revision:7,expires:expiry,_callback:None,
        });
        let manager = fixture.manager.clone();
        let pending = tokio::spawn(async move { manager.provider_callback(attempt, Some("fixture-code"), None, None).await });
        let check = provider.next().await;
        assert_eq!(check.authorization, "Bearer fixture-callback-access");
        let saved = fixture.manager.credentials.read_json("callback").unwrap().unwrap();
        assert_eq!(saved["oauth"]["access_token"], "fixture-callback-access");
        assert_eq!(saved["manual"]["other"], "fixture-keep");
        assert_eq!(saved["oauth_client"]["registration"]["client_id"], "fixture-client");
        assert_eq!(saved["future"]["preserve"], true);
        assert!(!pending.is_finished());
        check.respond.send(status).unwrap();
        let result = done(pending).await;
        if status == StatusCode::OK {
            result.unwrap();
            assert_eq!(fixture.manager.live.lock().await["callback"].auth["status"], "authenticated");
        } else {
            assert_eq!(result.unwrap_err().code, "CREDENTIAL_VERIFICATION_FAILED");
            assert_eq!(fixture.manager.live.lock().await["callback"].auth["status"], "failed");
        }
        token_task.abort();
        fixture.manager.shutdown().await;
        provider.no_more_requests();
    }
}
