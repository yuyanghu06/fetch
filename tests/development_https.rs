use fetch_local::development::{DevelopmentOptions, DevelopmentServer};
use fetch_local::{
    api::AppState,
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    storage::ServerRepository,
};
use reqwest::{Certificate, Client, StatusCode};
use serde_json::Value;
use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};

struct Running {
    origin: String,
    address: SocketAddr,
    certificate_pem: String,
    handle: axum_server::Handle<SocketAddr>,
    task: Option<tokio::task::JoinHandle<std::io::Result<()>>>,
}

impl Running {
    async fn start(path: &Path) -> Self {
        let server = DevelopmentServer::prepare(DevelopmentOptions {
            data_dir: path.to_owned(),
            port: 0,
        })
        .await
        .unwrap();
        Self::from_prepared(server).await
    }

    async fn from_prepared(server: DevelopmentServer) -> Self {
        let origin = server.origin.clone();
        let address = server.address().unwrap();
        let certificate_pem = server.certificate_pem.clone();
        let handle = axum_server::Handle::new();
        let task = tokio::spawn(server.run(handle.clone()));
        let listening = tokio::time::timeout(Duration::from_secs(5), handle.listening())
            .await
            .unwrap();
        assert_eq!(listening, Some(address));
        Self {
            origin,
            address,
            certificate_pem,
            handle,
            task: Some(task),
        }
    }

    fn trusted_client(&self) -> Client {
        Client::builder()
            .no_proxy()
            .http1_only()
            .timeout(Duration::from_secs(5))
            .resolve("localhost", self.address)
            .add_root_certificate(Certificate::from_pem(self.certificate_pem.as_bytes()).unwrap())
            .build()
            .unwrap()
    }

    async fn stop(mut self) {
        self.handle.graceful_shutdown(Some(Duration::from_secs(1)));
        tokio::time::timeout(Duration::from_secs(5), self.task.take().unwrap())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        self.handle.shutdown();
    }
}

#[tokio::test]
async fn http2_public_metadata_accepts_the_valid_authority_without_a_host_header() {
    let directory = tempfile::tempdir().unwrap();
    let running = Running::start(&directory.path().join("developer-state")).await;
    let client = Client::builder()
        .no_proxy()
        .http2_prior_knowledge()
        .timeout(Duration::from_secs(5))
        .resolve("localhost", running.address)
        .add_root_certificate(Certificate::from_pem(running.certificate_pem.as_bytes()).unwrap())
        .build()
        .unwrap();
    let response = client
        .get(format!(
            "{}/.well-known/oauth-protected-resource/mcp",
            running.origin
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.version(), reqwest::Version::HTTP_2);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["resource"], format!("{}/mcp", running.origin));
    running.stop().await;
}

struct FixtureGrant;
impl Authorizer for FixtureGrant {
    fn authorize(&self, _: &axum::http::HeaderMap) -> Result<Caller, AuthorizationError> {
        Ok(Caller::from_verified_scopes(["fetch:manage".to_owned()]))
    }
}

#[tokio::test]
async fn approved_read_apis_work_over_https_with_an_injected_fixture_grant() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("developer-state");
    let home = directory.path().join("fixture-home");
    std::fs::create_dir(&home).unwrap();
    std::fs::write(home.join(".claude.json"), r#"{"mcpServers":{"example":{"command":"npx","args":["-y","@fixture/example@1.2.3"],"env":{"EXAMPLE_KEY":"never-return-this-fixture-secret"}}}}"#).unwrap();
    let mut server = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: root.clone(),
        port: 0,
    })
    .await
    .unwrap();
    server.state = Arc::new(
        AppState::new(
            server.origin.clone(),
            ServerRepository::new(root),
            DiscoveryService::new(home, None),
            Arc::new(FixtureGrant),
        )
        .unwrap(),
    );
    let running = Running::from_prepared(server).await;
    let client = running.trusted_client();
    let response = client
        .get(format!("{}/api/v1/servers?id=absent", running.origin))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["servers"], serde_json::json!([]));
    assert_eq!(body["not_found_ids"], serde_json::json!(["absent"]));
    assert!(body["next_steps"].is_array());

    let response = client
        .get(format!("{}/api/v1/setup/discovery", running.origin))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = response.json().await.unwrap();
    assert_eq!(
        body["configs"][0]["servers"][0]["package"],
        "@fixture/example"
    );
    assert_eq!(
        body["configs"][0]["servers"][0]["credential_variables"],
        serde_json::json!(["EXAMPLE_KEY"])
    );
    assert!(
        !body
            .to_string()
            .contains("never-return-this-fixture-secret")
    );
    running.stop().await;
}

#[tokio::test]
async fn https_serves_public_metadata_with_normal_certificate_verification() {
    let directory = tempfile::tempdir().unwrap();
    let running = Running::start(&directory.path().join("developer-state")).await;
    assert!(running.address.ip().is_loopback());
    assert!(running.address.is_ipv4());
    let client = running.trusted_client();
    let response = client
        .get(format!(
            "{}/.well-known/oauth-protected-resource/mcp",
            running.origin
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");
    let metadata: Value = response.json().await.unwrap();
    assert_eq!(metadata["resource"], format!("{}/mcp", running.origin));

    let protected = client
        .get(format!("{}/api/v1/servers", running.origin))
        .bearer_auth("not-a-real-grant")
        .send()
        .await
        .unwrap();
    assert_eq!(protected.status(), StatusCode::UNAUTHORIZED);
    let body: Value = protected.json().await.unwrap();
    assert_eq!(body["error"]["code"], "UNAUTHENTICATED");
    assert_eq!(body["servers"], serde_json::json!([]));
    assert!(body["next_steps"].is_array());

    let hostile = client
        .get(format!("{}/api/v1/servers", running.origin))
        .header("Origin", "https://foreign.example")
        .send()
        .await
        .unwrap();
    assert_eq!(hostile.status(), StatusCode::FORBIDDEN);
    assert!(
        hostile
            .headers()
            .get("access-control-allow-origin")
            .is_none()
    );
    running.stop().await;
}

#[tokio::test]
async fn untrusted_certificate_and_wrong_hostname_are_tls_failures() {
    let directory = tempfile::tempdir().unwrap();
    let running = Running::start(&directory.path().join("developer-state")).await;
    let untrusted = Client::builder()
        .no_proxy()
        .http1_only()
        .timeout(Duration::from_secs(5))
        .resolve("localhost", running.address)
        .build()
        .unwrap();
    let response = untrusted
        .get(format!(
            "{}/.well-known/oauth-protected-resource/mcp",
            running.origin
        ))
        .send()
        .await;
    assert!(
        response.is_err(),
        "An untrusted self-signed identity was accepted"
    );
    assert!(response.unwrap_err().is_connect());

    let response = running
        .trusted_client()
        .get(format!(
            "https://127.0.0.1:{}/.well-known/oauth-protected-resource/mcp",
            running.address.port()
        ))
        .send()
        .await;
    assert!(
        response.is_err(),
        "The localhost certificate was accepted for an IP address"
    );
    assert!(response.unwrap_err().is_connect());
    running.stop().await;
}

#[tokio::test]
async fn same_developer_directory_cannot_start_a_duplicate_instance() {
    let directory = tempfile::tempdir().unwrap();
    let state = directory.path().join("developer-state");
    let first = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: state.clone(),
        port: 0,
    })
    .await
    .unwrap();
    let certificate = std::fs::read(state.join("certificate.pem")).unwrap();
    let second = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: state.clone(),
        port: 0,
    })
    .await;
    assert!(second.is_err());
    assert_eq!(
        std::fs::read(state.join("certificate.pem")).unwrap(),
        certificate
    );
    drop(first);
    let next = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: state.clone(),
        port: 0,
    })
    .await
    .unwrap();
    assert_ne!(next.certificate_pem.as_bytes(), certificate);
    let files: Vec<_> = std::fs::read_dir(&state)
        .unwrap()
        .map(|item| item.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(files.len(), 3);
    for expected in ["certificate.pem", "instance.lock", "config-fixtures"] {
        assert!(files.iter().any(|name| name == expected));
    }
    assert!(
        next.certificate_pem
            .starts_with("-----BEGIN CERTIFICATE-----")
    );
    assert!(!next.certificate_pem.contains("PRIVATE KEY"));
}

#[tokio::test]
async fn an_occupied_port_does_not_replace_an_existing_public_certificate() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let first = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: directory.path().join("first"),
        port: 0,
    })
    .await
    .unwrap();
    let second_root = directory.path().join("second");
    std::fs::create_dir(&second_root).unwrap();
    std::fs::set_permissions(&second_root, std::fs::Permissions::from_mode(0o700)).unwrap();
    let existing = b"existing-public-certificate-fixture";
    std::fs::write(second_root.join("certificate.pem"), existing).unwrap();
    let second = DevelopmentServer::prepare(DevelopmentOptions {
        data_dir: second_root.clone(),
        port: first.address().unwrap().port(),
    })
    .await;
    assert!(second.is_err());
    assert_eq!(
        std::fs::read(second_root.join("certificate.pem")).unwrap(),
        existing
    );
}

#[tokio::test]
async fn developer_state_rejects_a_symlink_or_world_readable_directory() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let directory = tempfile::tempdir().unwrap();
    let real = directory.path().join("real");
    std::fs::create_dir(&real).unwrap();
    std::fs::set_permissions(&real, std::fs::Permissions::from_mode(0o700)).unwrap();
    let link = directory.path().join("linked");
    symlink(&real, &link).unwrap();
    assert!(
        DevelopmentServer::prepare(DevelopmentOptions {
            data_dir: link,
            port: 0
        })
        .await
        .is_err()
    );
    std::fs::set_permissions(&real, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(
        DevelopmentServer::prepare(DevelopmentOptions {
            data_dir: real.clone(),
            port: 0
        })
        .await
        .is_err()
    );
    assert!(std::fs::read_dir(real).unwrap().next().is_none());
}
