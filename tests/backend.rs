use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode, header},
    response::Response,
};
use fetch_local::{
    api::{AppState, router},
    authority::{AuthorizationError, Authorizer, Caller, PendingNativeAuthority},
    discovery::DiscoveryService,
    storage::ServerRepository,
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::{path::Path, sync::Arc};
use tempfile::TempDir;
use tower::ServiceExt;

const ORIGIN: &str = "https://localhost:53671";

/// A fixture grant injected in-process. It is not a developer bearer-token path.
struct FixtureAuthority {
    manage: bool,
}

impl Authorizer for FixtureAuthority {
    fn authorize(&self, _: &HeaderMap) -> Result<Caller, AuthorizationError> {
        Ok(Caller::from_verified_scopes(
            self.manage.then(|| "fetch:manage".to_owned()),
        ))
    }
}

fn state(root: &Path, authority: Arc<dyn Authorizer>) -> Arc<AppState> {
    Arc::new(
        AppState::new(
            ORIGIN.into(),
            ServerRepository::new(root.join("state")),
            DiscoveryService::new(root.join("fixture-home"), None),
            authority,
        )
        .unwrap(),
    )
}

fn fixture() -> (TempDir, Arc<AppState>, Router) {
    let directory = tempfile::tempdir().unwrap();
    let state = state(
        directory.path(),
        Arc::new(FixtureAuthority { manage: true }),
    );
    let app = router(state.clone());
    (directory, state, app)
}

fn request(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header(header::HOST, "localhost:53671")
        .body(Body::empty())
        .unwrap()
}

async fn value(response: Response) -> Value {
    assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()
}

fn record(id: &str) -> Value {
    json!({
        "server_id": id, "name": "Fixture server",
        "runtime": {"status":"ready","error":null},
        "authentication": {"status":"authenticated","error":null},
        "restart_required":false, "restart":null,
        "config": {
            "kind":"local_script", "package":null,
            "executable":"/fixture/node", "args":["/fixture/server.js"],
            "working_directory":null, "environment":[]
        },
        "credentials":[]
    })
}

fn corrupt_saved_record(root: &Path, id: &str) {
    let path = std::fs::read_dir(root.join("state/servers"))
        .unwrap()
        .map(|entry| entry.unwrap().path().join("config.json"))
        .find(|path| {
            let body: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            body["server_id"] == id
        })
        .expect("fixture record exists");
    std::fs::write(path, "corrupt-private-fixture").unwrap();
}

#[tokio::test]
async fn public_resource_metadata_has_no_application_wrapper_or_private_inventory() {
    let directory = tempfile::tempdir().unwrap();
    let app = router(state(directory.path(), Arc::new(PendingNativeAuthority)));
    let response = app
        .oneshot(request("/.well-known/oauth-protected-resource/mcp"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    assert_eq!(body["resource"], format!("{ORIGIN}/mcp"));
    assert_eq!(body["authorization_servers"], json!([ORIGIN]));
    assert_eq!(body["bearer_methods_supported"], json!(["header"]));
    assert!(body.get("servers").is_none());
    assert!(body.get("status").is_none());
    assert!(body.get("next_steps").is_none());
}

#[tokio::test]
async fn incomplete_oauth_does_not_advertise_working_grant_endpoints() {
    let (_, _, app) = fixture();
    let response = app
        .oneshot(request("/.well-known/oauth-authorization-server"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = value(response).await;
    assert_eq!(body["error"], "temporarily_unavailable");
    assert!(body.get("token_endpoint").is_none());
    assert!(body.get("next_steps").is_none());
}

#[tokio::test]
async fn no_authority_means_no_source_existence_or_saved_record_disclosure() {
    let directory = tempfile::tempdir().unwrap();
    let state = state(directory.path(), Arc::new(PendingNativeAuthority));
    state
        .repository
        .save("private-fixture", &record("private-fixture"))
        .unwrap();
    let app = router(state);
    for uri in [
        "/api/v1/servers?id=private-fixture",
        "/api/v1/setup/discovery?config_path=%2Fnot-a-real-fixture.json",
        "/api/v1/setup/discovery?unknown=invalid",
    ] {
        let mut input = request(uri);
        input.headers_mut().insert(
            header::AUTHORIZATION,
            "Bearer pretend-dev-token".parse().unwrap(),
        );
        let response = app.clone().oneshot(input).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(
            response.headers()[header::WWW_AUTHENTICATE]
                .to_str()
                .unwrap()
                .contains("oauth-protected-resource/mcp")
        );
        let body = value(response).await;
        assert_eq!(body["error"]["code"], "UNAUTHENTICATED");
        assert_eq!(body["next_steps"], json!([]));
        let serialized = body.to_string();
        assert!(!serialized.contains("private-fixture"));
        assert!(!serialized.contains("not-a-real-fixture"));
    }
}

#[tokio::test]
async fn catalog_permission_does_not_allow_server_management() {
    let directory = tempfile::tempdir().unwrap();
    let app = router(state(
        directory.path(),
        Arc::new(FixtureAuthority { manage: false }),
    ));
    for uri in ["/api/v1/servers", "/api/v1/setup/discovery"] {
        let response = app.clone().oneshot(request(uri)).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(value(response).await["error"]["code"], "FORBIDDEN");
    }
}

#[tokio::test]
async fn missing_host_wrong_host_and_foreign_origin_are_rejected_without_cors() {
    let (_, _, app) = fixture();
    let mut missing_host = request("/api/v1/servers");
    missing_host.headers_mut().remove(header::HOST);
    let mut wrong_host = request("/api/v1/servers");
    wrong_host
        .headers_mut()
        .insert(header::HOST, "attacker.example".parse().unwrap());
    wrong_host
        .headers_mut()
        .insert("x-forwarded-host", "localhost:53671".parse().unwrap());
    let mut foreign_origin = request("/api/v1/servers");
    foreign_origin
        .headers_mut()
        .insert(header::ORIGIN, "https://attacker.example".parse().unwrap());
    let mut duplicate_host = request("/api/v1/servers");
    duplicate_host
        .headers_mut()
        .append(header::HOST, "localhost:53671".parse().unwrap());
    for (input, expected) in [
        (missing_host, StatusCode::BAD_REQUEST),
        (wrong_host, StatusCode::BAD_REQUEST),
        (foreign_origin, StatusCode::FORBIDDEN),
        (duplicate_host, StatusCode::BAD_REQUEST),
    ] {
        let response = app.clone().oneshot(input).await.unwrap();
        assert_eq!(response.status(), expected);
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .is_none()
        );
        assert!(value(response).await["next_steps"].is_array());
    }
}

#[tokio::test]
async fn absolute_request_authority_cannot_override_the_listener_address() {
    let (_, _, app) = fixture();
    let response = app
        .oneshot(request("https://attacker.example/api/v1/servers"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn server_listing_preserves_exact_ids_order_and_truthful_unknown_auth() {
    let (_, state, app) = fixture();
    for id in ["z", "A", "a", " padded "] {
        state.repository.save(id, &record(id)).unwrap();
    }
    let response = app
        .clone()
        .oneshot(request("/api/v1/servers"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    let names: Vec<_> = body["servers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["server_id"].as_str().unwrap())
        .collect();
    assert_eq!(names, [" padded ", "A", "a", "z"]);
    assert!(
        body["servers"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["runtime"]["status"] == "stopped"
                && v["authentication"]["status"] == "unknown")
    );

    let response = app
        .clone()
        .oneshot(request(
            "/api/v1/servers?id=z&id=missing&id=A&id=z&id=missing",
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    assert_eq!(body["servers"][0]["server_id"], "z");
    assert_eq!(body["servers"][1]["server_id"], "A");
    assert_eq!(body["servers"].as_array().unwrap().len(), 2);
    assert_eq!(body["not_found_ids"], json!(["missing"]));

    let response = app
        .oneshot(request("/api/v1/servers?id=%20padded%20"))
        .await
        .unwrap();
    assert_eq!(value(response).await["servers"][0]["server_id"], " padded ");
}

#[tokio::test]
async fn all_unknown_ids_are_successful_with_no_fabricated_records() {
    let (_, _, app) = fixture();
    let response = app
        .oneshot(request("/api/v1/servers?id=unknown&id=unknown"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["servers"], json!([]));
    assert_eq!(body["not_found_ids"], json!(["unknown"]));
}

#[tokio::test]
async fn invalid_get_inputs_are_rejected_without_echoing_input() {
    let (_, _, app) = fixture();
    for uri in [
        "/api/v1/servers?id=",
        "/api/v1/servers?id=%20",
        "/api/v1/servers?unexpected=never-echo-this",
        "/api/v1/servers?restart_id=attempt",
        "/api/v1/servers?id=A&id=B&restart_id=attempt",
        "/api/v1/setup/discovery?config_path=relative.json",
        "/api/v1/setup/discovery?config_path=%2Fa&config_path=%2Fb",
        "/api/v1/setup/discovery?config_path=%2Fa%00b",
    ] {
        let response = app.clone().oneshot(request(uri)).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{uri}");
        let body = value(response).await;
        assert_eq!(body["error"]["code"], "INVALID_REQUEST");
        assert!(!body.to_string().contains("never-echo-this"));
    }
    let input = Request::builder()
        .uri("/api/v1/servers")
        .header(header::HOST, "localhost:53671")
        .body(Body::from("secret-body"))
        .unwrap();
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(!value(response).await.to_string().contains("secret-body"));
}

#[tokio::test]
async fn malformed_percent_encoding_is_not_silently_reinterpreted() {
    let (_, _, app) = fixture();
    for uri in [
        "/api/v1/servers?id=%FF",
        "/api/v1/servers?id=%ZZ",
        "/api/v1/setup/discovery?config_path=%2Ftmp%FF",
    ] {
        let response = app.clone().oneshot(request(uri)).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{uri}");
        assert_eq!(value(response).await["error"]["code"], "INVALID_REQUEST");
    }
}

#[tokio::test]
async fn corrupt_saved_record_is_a_read_failure_not_an_empty_installation() {
    let (directory, state, app) = fixture();
    state
        .repository
        .save("fixture", &record("fixture"))
        .unwrap();
    let folder = std::fs::read_dir(directory.path().join("state/servers"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    std::fs::write(folder.join("config.json"), b"private-corrupt-payload").unwrap();
    let response = app.oneshot(request("/api/v1/servers")).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = value(response).await;
    assert_eq!(body["error"]["code"], "SERVER_INFO_UNAVAILABLE");
    assert_eq!(body["servers"], json!([]));
    assert!(!body.to_string().contains("private-corrupt-payload"));
}

#[tokio::test]
async fn selected_healthy_servers_do_not_read_unrelated_corrupt_records() {
    let (directory, state, app) = fixture();
    for id in ["healthy", "damaged"] {
        state.repository.save(id, &record(id)).unwrap();
    }
    corrupt_saved_record(directory.path(), "damaged");
    let response = app
        .clone()
        .oneshot(request("/api/v1/servers?id=healthy&id=unknown"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    assert_eq!(body["servers"].as_array().unwrap().len(), 1);
    assert_eq!(body["servers"][0]["server_id"], "healthy");
    assert_eq!(body["not_found_ids"], json!(["unknown"]));

    let response = app.oneshot(request("/api/v1/servers")).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        value(response).await["error"]["code"],
        "SERVER_INFO_UNAVAILABLE"
    );
}

#[tokio::test]
async fn restart_lookup_distinguishes_unknown_server_missing_attempt_and_read_failure() {
    let (directory, state, app) = fixture();
    for id in ["known", "damaged"] {
        state.repository.save(id, &record(id)).unwrap();
    }
    corrupt_saved_record(directory.path(), "damaged");
    for (uri, expected, code, missing) in [
        (
            "/api/v1/servers?id=missing&restart_id=attempt",
            StatusCode::NOT_FOUND,
            "SERVER_NOT_FOUND",
            json!(["missing"]),
        ),
        (
            "/api/v1/servers?id=known&restart_id=attempt",
            StatusCode::NOT_FOUND,
            "RESTART_NOT_FOUND",
            json!([]),
        ),
        (
            "/api/v1/servers?id=known&id=known&restart_id=attempt",
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            json!([]),
        ),
        (
            "/api/v1/servers?id=known&restart_id=attempt&restart_id=attempt",
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            json!([]),
        ),
        (
            "/api/v1/servers?id=damaged&restart_id=attempt",
            StatusCode::INTERNAL_SERVER_ERROR,
            "RESTART_STATUS_UNAVAILABLE",
            json!([]),
        ),
    ] {
        let response = app.clone().oneshot(request(uri)).await.unwrap();
        assert_eq!(response.status(), expected, "{uri}");
        let body = value(response).await;
        assert_eq!(body["error"]["code"], code, "{uri}");
        assert_eq!(body["servers"], json!([]));
        assert_eq!(body["not_found_ids"], missing);
        assert!(!body.to_string().contains("corrupt-private-fixture"));
    }
}

#[tokio::test]
async fn discovery_returns_safe_rows_and_keeps_source_disabled_entries_unselected() {
    let (directory, _, app) = fixture();
    let path = directory.path().join("source.json");
    std::fs::write(
        &path,
        r#"{"mcpServers":{"active":{"command":"npx","args":["-y","@fixture/example@1.2.3"],"env":{"FIXTURE_API_KEY":"do-not-return-source-value"}},"disabled":{"command":"uvx","args":["fixture-server==2.0.0"],"disabled":true},"blocked":{"command":"sh","args":["-c","do-not-return-shell-command"]}}}"#,
    ).unwrap();
    let query: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("config_path", path.to_str().unwrap())
        .finish();
    let response = app
        .oneshot(request(&format!("/api/v1/setup/discovery?{query}")))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    let rows = body["configs"][0]["servers"].as_array().unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r["name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["active", "blocked", "disabled"]
    );
    assert_eq!(rows[0]["status"], "supported");
    assert_eq!(rows[0]["package"], "@fixture/example");
    assert_eq!(rows[0]["version_spec"], "1.2.3");
    assert_eq!(rows[0]["credential_variables"], json!(["FIXTURE_API_KEY"]));
    assert_eq!(rows[0]["selected_by_default"], true);
    assert_eq!(rows[1]["status"], "unsupported");
    assert_eq!(rows[1]["selected_by_default"], false);
    assert_eq!(rows[2]["source_enabled"], false);
    assert_eq!(rows[2]["selected_by_default"], false);
    assert!(body["next_steps"].is_array());
    assert!(!body.to_string().contains("do-not-return"));
}

#[tokio::test]
async fn explicit_discovery_errors_differ_from_failed_default_sources() {
    let (directory, _, app) = fixture();
    let home = directory.path().join("fixture-home");
    std::fs::create_dir(&home).unwrap();
    std::fs::write(home.join(".claude.json"), "invalid-private-fixture").unwrap();
    let response = app
        .clone()
        .oneshot(request("/api/v1/setup/discovery"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = value(response).await;
    assert_eq!(body["configs"].as_array().unwrap().len(), 1);
    assert_eq!(body["configs"][0]["status"], "failed");
    assert_eq!(body["configs"][0]["servers"], json!([]));
    assert!(!body.to_string().contains("invalid-private-fixture"));

    for (file, expected, code) in [
        ("missing.json", StatusCode::NOT_FOUND, "CONFIG_NOT_FOUND"),
        (
            "duplicate.json",
            StatusCode::UNPROCESSABLE_ENTITY,
            "INVALID_CONFIG_FILE",
        ),
        (
            "ambiguous.json",
            StatusCode::UNPROCESSABLE_ENTITY,
            "AMBIGUOUS_CONFIG_FILE",
        ),
    ] {
        let path = directory.path().join(file);
        match file {
            "duplicate.json" => std::fs::write(
                &path,
                r#"{"mcpServers":{"a":{"command":"npx","command":"uvx","args":["fixture"]}}}"#,
            )
            .unwrap(),
            "ambiguous.json" => std::fs::write(&path, r#"{"mcpServers":{},"mcp":{}}"#).unwrap(),
            _ => {}
        }
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("config_path", path.to_str().unwrap())
            .finish();
        let response = app
            .clone()
            .oneshot(request(&format!("/api/v1/setup/discovery?{query}")))
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{file}");
        let body = value(response).await;
        assert_eq!(body["error"]["code"], code);
        assert_eq!(body["configs"], json!([]));
    }
}

#[tokio::test]
async fn application_method_errors_keep_the_common_json_guidance_field() {
    let (_, _, app) = fixture();
    let input = Request::builder()
        .method("POST")
        .uri("/api/v1/servers")
        .header(header::HOST, "localhost:53671")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(input).await.unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert!(value(response).await["next_steps"].is_array());
}
