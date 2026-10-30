//! Independent API/storage/route checks for config-driven authentication inputs.
//! All providers and keys are disposable; no managed process or native prompt runs.
use super::*;
use super::auth_tests::{Fixture, MockProvider, clean_response, decode, done};
use super::verification::Profile;
use axum::{body::Body, http::{Request, StatusCode, header}};
use tower::ServiceExt;

fn joint_profile() -> Profile {
    serde_json::from_value(json!({
        "id":"fixture-joint",
        "targets":{"packages":["fixture-joint-mcp"],"hosts":[]},
        "manual":[{
            "mode":"together",
            "inputs":{
                "access":{"source":"environment","names":["UNUSUAL_ACCESS_NAME"]},
                "project":{"source":"environment","names":["CUSTOM_PROJECT_INPUT"]}
            },
            "request":{
                "method":"POST","url":"https://verification.fixture.example/check",
                "headers":{"X-Fetch-Fixture":"joint"},
                "body":{"context":{"project":null}},
                "credentials":[
                    {"input":"access","placement":"header","name":"Authorization","prefix":"Bearer ","strip_prefix":"Bearer "},
                    {"input":"project","placement":"json","pointer":"/context/project"}
                ],
                "success":[{"rule":"string","pointer":"/id","nonempty":true}]
            }
        }],
        "oauth":null
    })).unwrap()
}

fn standard_remote_profile() -> Profile {
    serde_json::from_value(json!({
        "id":"fixture-standard-remote",
        "targets":{"packages":[],"hosts":["mcp.miro.com"]},
        "manual":[{"mode":"each","selectors":[{"source":"header","names":["X-Arbitrary-Service-Token"]}],
            "input":"access","request":{
                "method":"GET","url":"https://verification.fixture.example/check","headers":{},
                "credentials":[{"input":"access","placement":"header","name":"Authorization","prefix":"Bearer "}],
                "success":[{"rule":"string","pointer":"/id","nonempty":true}]
            }}],"oauth":null
    })).unwrap()
}

fn remote_record(id: &str, headers: Value) -> Value {
    json!({
        "server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.miro.com/","transport":"streamable_http","headers":headers}},
        "credentials":[{"name":"saved-token","status":"stored"},{"name":"workspace","status":"stored"}]
    })
}

#[test]
fn production_profile_selection_keeps_standard_remote_oauth_independent_of_verification() {
    let profile = standard_remote_profile();
    let record = remote_record("remote", json!([]));
    assert!(profile.matches_record(&record));
    assert!(matches!(provider_for_profile(&record, Some(&profile)), Provider::Remote));
    assert!(matches!(provider_for_profile(&record, None), Provider::Remote));
}

#[tokio::test]
async fn standard_remote_can_verify_saved_manual_header_without_requiring_an_oauth_record() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(standard_remote_profile()));
    let record = remote_record("remote", json!([
        {"name":"X-Arbitrary-Service-Token","credential_ref":"saved-token"}
    ]));
    fixture.manager.repository.save("remote", &record).unwrap();
    fixture.store("remote", json!({"saved-token":"fixture-remote-manual"}), Value::Null);
    let pending = fixture.post("remote");
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-remote-manual");
    check.respond.send(StatusCode::OK).unwrap();
    let (status, result) = done(pending).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["status"], "authenticated");
    assert!(fixture.manager.secrets("remote").unwrap()["oauth"].is_null());
    clean_response(&result, &["fixture-remote-manual"]);
    provider.no_more_requests();
}

#[tokio::test]
async fn ordinary_remote_header_does_not_count_as_a_saved_authentication_credential() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(standard_remote_profile()));
    let record = remote_record("remote", json!([
        {"name":"X-Workspace","credential_ref":"workspace"}
    ]));
    let document = json!({"manual":{"workspace":"fixture-workspace"},"oauth":null});
    assert!(!fixture.manager.has_stored_authentication(&record, &document).unwrap());
    provider.no_more_requests();
}

#[tokio::test]
async fn unknown_connector_imports_and_delivers_arbitrary_names_without_claiming_authentication() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("source.json");
    std::fs::write(&path, json!({"mcpServers":{"custom":{
        "command":"npx","args":["-y","fixture-unrecognized-mcp"],
        "env":{"CUSTOM_TOKEN_NAME":"source-must-not-be-copied","CUSTOM_TEAM_SETTING":"source-team"}
    }}}).to_string()).unwrap();
    let capture = crate::discovery::DiscoveryService::new(directory.path().into(), None)
        .capture(&path, "custom").unwrap();
    assert_eq!(capture.config["environment"], json!([
        {"name":"CUSTOM_TEAM_SETTING","credential_ref":"CUSTOM_TEAM_SETTING"},
        {"name":"CUSTOM_TOKEN_NAME","credential_ref":"CUSTOM_TOKEN_NAME"}
    ]));
    assert!(!capture.config.to_string().contains("source-must-not-be-copied"));
    fixture.local("custom", "fixture-unrecognized-mcp", &[
        ("CUSTOM_TOKEN_NAME", "custom-token"), ("CUSTOM_TEAM_SETTING", "custom-team")
    ], json!({}));
    let error = done(fixture.save("custom", json!({"set":[
        {"name":"custom-token","value":"fixture-custom-token"},
        {"name":"custom-team","value":"fixture-custom-team"}
    ]}))).await.unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    let stored = fixture.manager.secrets("custom").unwrap();
    assert_eq!(stored["manual"]["custom-token"], "fixture-custom-token");
    let record = fixture.manager.record("custom").unwrap();
    match fixture.manager.route(&record).await.unwrap().transport {
        TransportConfig::Stdio { environment, .. } => {
            assert_eq!(environment["CUSTOM_TOKEN_NAME"], "fixture-custom-token");
            assert_eq!(environment["CUSTOM_TEAM_SETTING"], "fixture-custom-team");
        }
        _ => panic!("expected only a prepared local route"),
    }
    let (status, result) = done(fixture.post("custom")).await;
    assert_eq!(status, StatusCode::OK);
    assert_ne!(result["status"], "authenticated");
    assert_eq!(result["status"], "unknown");
    clean_response(&result, &["fixture-custom-token", "fixture-custom-team"]);
    clean_response(&error.body(), &["fixture-custom-token", "fixture-custom-team"]);
    provider.no_more_requests();
}

#[tokio::test]
async fn arbitrary_imported_header_names_use_the_same_encrypted_route_resolution() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("remote-source.json");
    std::fs::write(&path, json!({"mcpServers":{"remote":{
        "url":"https://mcp.miro.com/",
        "headers":{"X-Custom-Service-Key":"source-header-must-not-be-copied"}
    }}}).to_string()).unwrap();
    let capture = crate::discovery::DiscoveryService::new(directory.path().into(), None)
        .capture(&path, "remote").unwrap();
    assert_eq!(capture.config["connection"]["headers"], json!([
        {"name":"X-Custom-Service-Key","credential_ref":"X-Custom-Service-Key"}
    ]));
    assert!(!capture.config.to_string().contains("source-header-must-not-be-copied"));
    fixture.manager.repository.save("remote", &json!({
        "server_id":"remote","name":"remote","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.miro.com/","transport":"streamable_http","headers":[
                {"name":"X-Custom-Service-Key","credential_ref":"header-input"}
            ]}},
        "credentials":[{"name":"header-input","status":"missing"}]
    })).unwrap();
    let error = done(fixture.save("remote", json!({"set":[{"name":"header-input","value":"fixture-custom-header"}]}))).await.unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    let record = fixture.manager.record("remote").unwrap();
    match fixture.manager.route(&record).await.unwrap().transport {
        TransportConfig::Http { headers, .. } => assert_eq!(headers["X-Custom-Service-Key"], "fixture-custom-header"),
        _ => panic!("expected only a prepared HTTP route"),
    }
    clean_response(&error.body(), &["fixture-custom-header"]);
    provider.no_more_requests();
}

#[tokio::test]
async fn trusted_joint_description_checks_arbitrary_saved_inputs_in_one_request_for_save_and_post() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(joint_profile()));
    fixture.local("custom", "fixture-joint-mcp", &[
        ("UNUSUAL_ACCESS_NAME", "private-access"), ("CUSTOM_PROJECT_INPUT", "private-project")
    ], json!({}));
    let pending = fixture.save("custom", json!({"set":[
        {"name":"private-access","value":"fixture-joint-access"},
        {"name":"private-project","value":"fixture-joint-project"}
    ]}));
    let check = provider.next().await;
    assert_eq!(check.method, reqwest::Method::POST);
    assert_eq!(check.authorization, "Bearer fixture-joint-access");
    assert_eq!(check.headers["x-fetch-fixture"], "joint");
    assert_eq!(serde_json::from_slice::<Value>(&check.body).unwrap(), json!({"context":{"project":"fixture-joint-project"}}));
    assert_eq!(fixture.manager.secrets("custom").unwrap()["manual"]["private-access"], "fixture-joint-access");
    assert!(!pending.is_finished());
    check.respond.send(StatusCode::OK).unwrap();
    let saved = done(pending).await.unwrap();
    assert_eq!(saved["status"], "ok");
    clean_response(&saved, &["fixture-joint-access", "fixture-joint-project"]);
    let pending = fixture.post("custom");
    let check = provider.next().await;
    assert_eq!(check.authorization, "Bearer fixture-joint-access");
    assert_eq!(serde_json::from_slice::<Value>(&check.body).unwrap()["context"]["project"], "fixture-joint-project");
    check.respond.send(StatusCode::OK).unwrap();
    let (status, result) = done(pending).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["status"], "authenticated");
    clean_response(&result, &["fixture-joint-access", "fixture-joint-project"]);
    provider.no_more_requests();
}

#[tokio::test]
async fn unrelated_environment_value_cannot_borrow_the_joint_authentication_result() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(joint_profile()));
    fixture.local("custom", "fixture-joint-mcp", &[
        ("UNUSUAL_ACCESS_NAME", "private-access"), ("CUSTOM_PROJECT_INPUT", "private-project"),
        ("HOME", "private-home")
    ], json!({"private-access":"fixture-access","private-project":"fixture-project"}));
    let error = done(fixture.save("custom", json!({"set":[{"name":"private-home","value":"/disposable/custom-home"}]}))).await.unwrap_err();
    assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
    assert_eq!(fixture.manager.secrets("custom").unwrap()["manual"]["private-home"], "/disposable/custom-home");
    provider.no_more_requests();
}

#[tokio::test]
async fn trusted_description_for_another_server_does_not_enable_this_connector() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::with_profile(&provider.endpoint, Some(joint_profile()));
    fixture.local("other", "fixture-different-mcp", &[
        ("UNUSUAL_ACCESS_NAME", "private-access"), ("CUSTOM_PROJECT_INPUT", "private-project")
    ], json!({"private-access":"fixture-access","private-project":"fixture-project"}));
    let (status, result) = done(fixture.post("other")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["status"], "unknown");
    provider.no_more_requests();
}

#[tokio::test]
async fn ordinary_api_inputs_cannot_supply_a_verification_destination() {
    let mut provider = MockProvider::new().await;
    let fixture = Fixture::new(&provider.endpoint);
    fixture.local("custom", "fixture-unrecognized-mcp", &[("ANY_INPUT_NAME", "input")], json!({"input":"fixture-input"}));
    for (method, path, input) in [
        ("POST", "/api/v1/servers/custom/authenticate", json!({"verification":{"url":provider.endpoint}})),
        ("PATCH", "/api/v1/servers/custom/config", json!({"verification":{"url":provider.endpoint}})),
    ] {
        let request = Request::builder().method(method).uri(path)
            .header(header::HOST, "localhost:53672")
            .header(header::AUTHORIZATION, "Bearer fixture-caller")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(input.to_string())).unwrap();
        let (status, result) = decode(fixture.app.clone().oneshot(request).await.unwrap()).await;
        assert!(!status.is_success());
        assert!(result["error"]["code"].is_string());
        clean_response(&result, &["fixture-input"]);
    }
    provider.no_more_requests();
}
