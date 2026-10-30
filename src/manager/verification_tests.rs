//! Independent checks of trusted descriptions, bindings and real mock HTTP replies.
use super::{bindings::InputBindings, verification::{Profile, builtin_for}};
use super::auth_tests::MockProvider;
use axum::{Router, http::{StatusCode, header}};
use serde_json::{Value, json};
use std::time::Duration;

fn single_profile(success: Value) -> Profile {
    serde_json::from_value(json!({
        "id":"fixture-single","targets":{"packages":["fixture"],"hosts":[]},
        "manual":[{"mode":"each","selectors":[{"source":"environment","names":["ARBITRARY_INPUT"]}],
            "input":"access","request":{
                "method":"GET","url":"https://verification.fixture.example/check","headers":{},
                "credentials":[{"input":"access","placement":"header","name":"Authorization","prefix":"Bearer "}],
                "success":success
            }}],"oauth":null
    })).unwrap()
}

fn one_binding() -> (InputBindings, Value) {
    let bindings = InputBindings::from_record(&json!({"config":{"environment":[
        {"name":"ARBITRARY_INPUT","credential_ref":"saved-arbitrary"}
    ]}})).unwrap();
    (bindings, json!({"manual":{"saved-arbitrary":"fixture-token"},"oauth":null}))
}

struct ReplyServer {
    endpoint: String,
    task: tokio::task::JoinHandle<()>,
}
impl ReplyServer {
    async fn new(status: StatusCode, body: &str) -> Self {
        let body = body.to_owned();
        let app = Router::new().fallback(move || {
            let body = body.clone();
            async move { (status, [(header::CONTENT_TYPE, "application/json")], body) }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });
        Self { endpoint, task }
    }
}
impl Drop for ReplyServer { fn drop(&mut self) { self.task.abort(); } }

fn client() -> reqwest::Client {
    reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(3)).build().unwrap()
}

#[tokio::test]
async fn semantic_success_uses_the_description_instead_of_accepting_any_2xx() {
    let profile = single_profile(json!([{"rule":"equals","pointer":"/ok","value":true}]));
    let (bindings, document) = one_binding();
    for (status, body, expected) in [
        (StatusCode::OK, r#"{"ok":true}"#, true),
        (StatusCode::OK, r#"{"ok":false}"#, false),
        (StatusCode::OK, r#"{"ok":"true"}"#, false),
        (StatusCode::OK, r#"{}"#, false),
        (StatusCode::OK, "not-json", false),
        (StatusCode::UNAUTHORIZED, r#"{"ok":true}"#, false),
    ] {
        let server = ReplyServer::new(status, body).await;
        let plan = profile.bind(&bindings, &document, &[], None).unwrap().with_endpoint(&server.endpoint);
        assert_eq!(plan.execute(&client()).await.is_ok(), expected, "status={status}, body={body}");
    }
}

#[tokio::test]
async fn existing_linear_description_preserves_viewer_and_error_member_semantics() {
    let record = json!({"config":{"kind":"remote","environment":[],"connection":{
        "url":"https://mcp.linear.app/mcp/readonly","headers":[{"name":"Authorization","credential_ref":"linear-saved"}]
    }}});
    let profile = builtin_for(&record).unwrap();
    let bindings = InputBindings::from_record(&record).unwrap();
    let document = json!({"manual":{"linear-saved":"fixture-linear"},"oauth":null});
    for (body, expected) in [
        (r#"{"data":{"viewer":{"id":"fixture-user"}}}"#, true),
        (r#"{"data":{"viewer":{"id":""}}}"#, true),
        (r#"{"data":{"viewer":{"id":"fixture-user"}},"errors":null}"#, false),
        (r#"{"data":{"viewer":{"id":"fixture-user"}},"errors":[]}"#, false),
        (r#"{"data":{"viewer":{"id":12}}}"#, false),
        (r#"{"data":{"viewer":{}}}"#, false),
    ] {
        let server = ReplyServer::new(StatusCode::OK, body).await;
        let plan = profile.bind(&bindings, &document, &[], None).unwrap().with_endpoint(&server.endpoint);
        assert_eq!(plan.execute(&client()).await.is_ok(), expected, "body={body}");
    }
}

#[tokio::test]
async fn existing_railway_description_requires_a_nonblank_subject() {
    let record = json!({"config":{"kind":"remote","environment":[],"connection":{
        "url":"https://mcp.railway.com/","headers":[]
    }}});
    let profile = builtin_for(&record).unwrap();
    let bindings = InputBindings::from_record(&record).unwrap();
    let document = json!({"manual":{},"oauth":{"access_token":"fixture-railway"}});
    for (body, expected) in [
        (r#"{"sub":"fixture-user"}"#, true),
        (r#"{"sub":"  fixture-user  "}"#, true),
        (r#"{"sub":"   "}"#, false),
        (r#"{"sub":""}"#, false),
        (r#"{"sub":15}"#, false),
        (r#"{}"#, false),
    ] {
        let server = ReplyServer::new(StatusCode::OK, body).await;
        let plan = profile.bind(&bindings, &document, &[], Some("fixture-railway")).unwrap().with_endpoint(&server.endpoint);
        assert_eq!(plan.execute(&client()).await.is_ok(), expected, "body={body}");
    }
}

#[test]
fn binding_names_are_generic_but_ambiguous_duplicates_are_rejected() {
    let record = json!({"config":{"environment":[
        {"name":"NOT_A_KNOWN_PROVIDER_KEY","credential_ref":"one"},
        {"name":"CUSTOM_PATH_INPUT","credential_ref":"two"}
    ],"connection":{"headers":[{"name":"X-Custom-Auth","credential_ref":"three"}]}}});
    let bindings = InputBindings::from_record(&record).unwrap();
    assert_eq!(bindings.references(), ["one", "two", "three"].into_iter().map(str::to_owned).collect());
    for config in [
        json!({"environment":[{"name":"VALUE","credential_ref":"one"},{"name":"VALUE","credential_ref":"two"}]}),
        json!({"environment":[],"connection":{"headers":[{"name":"X-Key","credential_ref":"one"},{"name":"x-key","credential_ref":"two"}]}}),
    ] {
        assert!(InputBindings::from_config(&config).is_err(), "duplicate source names cannot pick an arbitrary saved value");
    }
}

fn pair_profile(inputs: Value, credentials: Value) -> Profile {
    serde_json::from_value(json!({
        "id":"pair","targets":{"packages":["fixture"],"hosts":[]},
        "manual":[{"mode":"together","inputs":inputs,"request":{
            "method":"POST","url":"https://verification.fixture.example/check","headers":{},
            "body":{"project":null,"context":{"project":null}},"credentials":credentials,"success":[]
        }}],"oauth":null
    })).unwrap()
}

#[test]
fn joint_proof_rejects_missing_ambiguous_or_reused_required_inputs() {
    let inputs = json!({
        "access":{"source":"environment","names":["ACCESS"]},
        "project":{"source":"environment","names":["PROJECT","ALTERNATE_PROJECT"]}
    });
    let placements = json!([
        {"input":"access","placement":"header","name":"Authorization","prefix":"Bearer "},
        {"input":"project","placement":"json","pointer":"/project"}
    ]);
    let profile = pair_profile(inputs, placements);
    for environment in [
        json!([{"name":"ACCESS","credential_ref":"first"}]),
        json!([{"name":"ACCESS","credential_ref":"first"},{"name":"PROJECT","credential_ref":"second"},{"name":"ALTERNATE_PROJECT","credential_ref":"third"}]),
        json!([{"name":"ACCESS","credential_ref":"first"},{"name":"PROJECT","credential_ref":"first"}]),
    ] {
        let bindings = InputBindings::from_config(&json!({"environment":environment})).unwrap();
        assert!(profile.bind(&bindings, &json!({"manual":{"first":"fixture-a","second":"fixture-b","third":"fixture-c"}}), &[], None).is_err());
    }
}

#[test]
fn all_joint_inputs_must_have_distinct_explicit_request_destinations() {
    let inputs = json!({
        "access":{"source":"environment","names":["ACCESS"]},
        "project":{"source":"environment","names":["PROJECT"]}
    });
    let bindings = InputBindings::from_config(&json!({"environment":[
        {"name":"ACCESS","credential_ref":"first"},{"name":"PROJECT","credential_ref":"second"}
    ]})).unwrap();
    let document = json!({"manual":{"first":"fixture-a","second":"fixture-b"}});
    for placements in [
        json!([{"input":"access","placement":"header","name":"Authorization"}]),
        json!([{"input":"access","placement":"header","name":"Authorization"},{"input":"project","placement":"header","name":"authorization"}]),
        json!([{"input":"access","placement":"json","pointer":"/project"},{"input":"project","placement":"json","pointer":"/project"}]),
        json!([{"input":"access","placement":"json","pointer":"/context/project"},{"input":"project","placement":"json","pointer":"/context"}]),
        json!([{"input":"access","placement":"json","pointer":"/context"},{"input":"project","placement":"json","pointer":"/context/project"}]),
    ] {
        assert!(pair_profile(inputs.clone(), placements).bind(&bindings, &document, &[], None).is_err());
    }
}

#[tokio::test]
async fn malformed_later_header_cannot_dispatch_an_earlier_verification_request() {
    let profile: Profile = serde_json::from_value(json!({
        "id":"preflight","targets":{"packages":["fixture"],"hosts":[]},
        "manual":[{"mode":"each","selectors":[{"source":"environment","names":["ONE","TWO"]}],
            "input":"token","request":{
                "method":"GET","url":"https://verification.fixture.example/check","headers":{},
                "credentials":[{"input":"token","placement":"header","name":"Authorization","prefix":"Bearer "}],
                "success":[]
            }}],"oauth":null
    })).unwrap();
    let bindings = InputBindings::from_config(&json!({"environment":[
        {"name":"ONE","credential_ref":"a-first"},{"name":"TWO","credential_ref":"z-last"}
    ]})).unwrap();
    let document = json!({"manual":{"a-first":"fixture-valid","z-last":"fixture-invalid\r\nextra-header"}});
    let mut provider = MockProvider::new().await;
    let plan = match profile.bind(&bindings, &document, &[], None) {
        Ok(plan) => plan.with_endpoint(&provider.endpoint),
        Err(error) => {
            assert_eq!(error.code, "CREDENTIAL_VERIFICATION_FAILED");
            provider.no_more_requests();
            return;
        }
    };
    let mut task = tokio::spawn(async move { plan.execute(&client()).await });
    tokio::select! {
        result = &mut task => assert_eq!(result.unwrap().unwrap_err().code, "CREDENTIAL_VERIFICATION_FAILED"),
        request = provider.requests.recv() => {
            assert!(request.is_some());
            task.abort();
            panic!("a malformed later header allowed an earlier verification request to leave Fetch");
        }
        _ = tokio::time::sleep(Duration::from_secs(3)) => {
            task.abort();
            panic!("malformed-request validation did not finish");
        }
    }
    provider.no_more_requests();
}

#[test]
fn profile_target_matching_rejects_lookalike_hosts_ports_and_credential_urls() {
    let profile = builtin_for(&json!({"config":{"kind":"remote","connection":{"url":"https://api.githubcopilot.com/mcp/"}}})).unwrap();
    for endpoint in ["https://api.githubcopilot.com.evil.example/mcp/", "https://api.githubcopilot.com:8443/mcp/", "https://user@api.githubcopilot.com/mcp/", "http://api.githubcopilot.com/mcp/"] {
        assert!(!profile.matches_record(&json!({"config":{"kind":"remote","connection":{"url":endpoint}}})), "{endpoint}");
    }
}
