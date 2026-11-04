//! Independent authority checks. No native Keychain, window, browser or provider runs.
use super::*;
use crate::{authority::PendingNativeAuthority, installation::Installer, manager::Manager,
    native_auth::NativeAuthority, remote_oauth::GrantStore, runner::Runner};
use http_body_util::BodyExt;
use serde_json::{Value,json};
use std::path::PathBuf;
use tower::ServiceExt;

struct Fixture {
    _directory:tempfile::TempDir,
    root:PathBuf,
    manager:Arc<Manager>,
    native:Arc<NativeAuthority>,
    grants:Arc<GrantStore>,
    app:Router,
}
impl Fixture {
    fn new()->Self {
        Self::with_native(NativeAuthority::new())
    }
    fn with_native(native:Arc<NativeAuthority>)->Self {
        Self::with_backend(native,None)
    }
    fn with_backend(native:Arc<NativeAuthority>,backend:Option<Arc<dyn crate::remote_oauth::Backend>>)->Self {
        let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");
        let repository=ServerRepository::new(root.clone());let discovery=DiscoveryService::new(directory.path().join("empty-home"),None);
        let grants=Arc::new(match backend{Some(backend)=>GrantStore::with_backend(root.clone(),backend),None=>GrantStore::memory(root.clone())});
        let manager=Manager::with_grant_store(repository.clone(),grants.clone(),Runner::new(),Installer::new(root.clone(),discovery.clone()));
        let mut state=AppState::new("https://localhost:53672".into(),repository,discovery,Arc::new(PendingNativeAuthority)).unwrap();
        state.manager=Some(manager.clone());state.native=Some(native.clone());
        Self{_directory:directory,root,manager,native,grants,app:router(Arc::new(state))}
    }
    fn request(&self,path:&str,method:&str,body:Value,token:&str)->axum::http::Request<Body> {
        axum::http::Request::builder().uri(path).method(method).header(header::HOST,"localhost:53672")
            .header(header::CONTENT_TYPE,"application/json").header("x-fetch-background-auth",token)
            .body(Body::from(body.to_string())).unwrap()
    }
}
async fn body(response:Response)->Value {serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()}

#[tokio::test]
async fn process_owned_background_token_cannot_open_interactive_login_or_gain_other_permissions() {
    let fixture=Fixture::new();let token=fixture.manager.background_token();
    for input in [json!({}),json!({"interactive":true})] {
        let response=fixture.app.clone().oneshot(fixture.request("/api/v1/servers/missing/authenticate","POST",input,&token)).await.unwrap();
        assert_eq!(response.status(),StatusCode::FORBIDDEN);assert_eq!(body(response).await["error"]["code"],"FORBIDDEN");
    }
    for (method,path,input) in [
        ("GET","/api/v1/servers",json!({})),
        ("PATCH","/api/v1/servers/missing/config",json!({"execution":{"default_timeout_ms":1}})),
        ("PATCH","/api/v1/servers/missing/credentials",json!({"set":[{"name":"key","value":"fixture-manual"}]})),
        ("POST","/api/v1/servers/missing/restart",json!({})),
        ("POST","/api/v1/servers/missing/open-folder",json!({})),
        ("POST","/api/v1/servers/missing/tools/echo/call",json!({})),
        ("POST","/api/v1/setup/installations",json!({"servers":[{"server_name":"fixture","command":"npx fixture"}]})),
    ] {
        let response=fixture.app.clone().oneshot(fixture.request(path,method,input,&token)).await.unwrap();
        assert!(matches!(response.status(),StatusCode::UNAUTHORIZED|StatusCode::FORBIDDEN),"{method} {path} must not accept background authority");
    }
    assert!(!fixture.root.exists());
}

#[tokio::test]
async fn background_token_is_per_process_rejected_on_quit_and_duplicate_headers_fail_closed() {
    let first=Fixture::new();let second=Fixture::new();let token=first.manager.background_token();
    assert_ne!(token,second.manager.background_token());
    for candidate in ["bad-fixture-token".to_owned(),second.manager.background_token()] {
        let response=first.app.clone().oneshot(first.request("/api/v1/servers/missing/authenticate","POST",json!({"interactive":false}),&candidate)).await.unwrap();
        assert_eq!(response.status(),StatusCode::FORBIDDEN);
    }
    let mut duplicate=first.request("/api/v1/servers/missing/authenticate","POST",json!({"interactive":false}),&token);
    duplicate.headers_mut().append("x-fetch-background-auth",token.parse().unwrap());
    assert_eq!(first.app.clone().oneshot(duplicate).await.unwrap().status(),StatusCode::FORBIDDEN);
    let response=first.app.clone().oneshot(first.request("/api/v1/servers/missing/authenticate","POST",json!({"interactive":false}),&token)).await.unwrap();
    assert_eq!(response.status(),StatusCode::NOT_FOUND,"valid token enters only the authenticate route");
    first.manager.shutdown().await;
    assert_eq!(first.app.clone().oneshot(first.request("/api/v1/servers/missing/authenticate","POST",json!({"interactive":false}),&token)).await.unwrap().status(),StatusCode::FORBIDDEN);
    assert!(!first.root.exists());assert!(!second.root.exists());
}

#[tokio::test]
async fn quiet_authentication_never_executes_a_recorded_native_command() {
    let fixture=Fixture::new();
    let record=json!({"server_id":"native","name":"native","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"credentials":[],
        "config":{"kind":"local_script","package":null,"executable":"/disposable/not-executed","args":[],"working_directory":null,"environment":[]},
        "_installation":{"source_path":"/disposable/source.json","source_name":"native","installation_id":"fixture-installation","protected_args":0,
            "native_setup":{"executable":"/disposable/not-executed","args":[],"working_directory":fixture.root,"state_directory":fixture.root}}});
    fixture.manager.repository.save("native",&record).unwrap();
    let before=fixture.manager.repository.get_saved("native").unwrap().unwrap();
    let response=fixture.app.clone().oneshot(fixture.request("/api/v1/servers/native/authenticate","POST",json!({"interactive":false}),&fixture.manager.background_token())).await.unwrap();
    assert_eq!(response.status(),StatusCode::FORBIDDEN);
    assert_eq!(fixture.manager.repository.get_saved("native").unwrap().unwrap(),before);
    assert!(fixture.manager.runner.server_ids().await.is_empty());
}

#[tokio::test]
async fn native_folder_route_rejects_path_overrides_and_unknown_locations_without_opening_finder() {
    let fixture=Fixture::new();let session=fixture.native.open_settings();
    for (uri,body,expected) in [
        ("/api/v1/servers/unknown/open-folder?path=/disposable",Some(json!({})),StatusCode::BAD_REQUEST),
        ("/api/v1/servers/unknown/open-folder",Some(json!({"path":"/disposable"})),StatusCode::BAD_REQUEST),
        ("/api/v1/servers/unknown/open-folder",None,StatusCode::NOT_FOUND),
    ] {
        let body=body.map(|value|Body::from(value.to_string())).unwrap_or_else(Body::empty);
        let request=axum::http::Request::builder().method("POST").uri(uri).header(header::HOST,"localhost:53672")
            .header("x-fetch-settings-capability",session.header_value()).body(body).unwrap();
        let response=fixture.app.clone().oneshot(request).await.unwrap();assert_eq!(response.status(),expected);
    }
    fixture.manager.repository.save("native",&json!({"server_id":"native","name":"native","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"credentials":[],
        "config":{"kind":"local_script","package":null,"executable":"/disposable/not-executed","args":[],"working_directory":null,"environment":[]}})).unwrap();
    let request=axum::http::Request::builder().method("POST").uri("/api/v1/servers/native/open-folder").header(header::HOST,"localhost:53672")
        .header("x-fetch-settings-capability",session.header_value()).body(Body::empty()).unwrap();
    let response=fixture.app.clone().oneshot(request).await.unwrap();assert_eq!(response.status(),StatusCode::CONFLICT);
    assert_eq!(body(response).await["error"]["code"],"SERVER_FOLDER_UNAVAILABLE");
    assert!(fixture.grants.read_json("native").await.unwrap().is_none());
}

fn manual_record()->Value {json!({"server_id":"manual","name":"manual","runtime":{"status":"stopped","error":null},
    "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"credentials":[],
    "config":{"kind":"local_script","package":null,"executable":"/disposable/not-executed","args":[],"working_directory":null,
        "environment":[{"name":"API_KEY","credential_ref":"assigned-key"}]}})}
fn manual_request(session:&crate::native_auth::NativeSettingsSession,input:Value)->axum::http::Request<Body>{
    axum::http::Request::builder().method("PATCH").uri("/api/v1/servers/manual/credentials").header(header::HOST,"localhost:53672")
        .header(header::CONTENT_TYPE,"application/json").header("x-fetch-settings-capability",session.header_value()).body(Body::from(input.to_string())).unwrap()
}
#[tokio::test]
async fn manual_patch_reports_stored_without_claiming_account_verification_and_rejects_unassigned_values() {
    let fixture=Fixture::with_native(NativeAuthority::fixture());let session=fixture.native.open_settings();
    fixture.manager.repository.save("manual",&manual_record()).unwrap();
    let response=fixture.app.clone().oneshot(manual_request(&session,json!({"set":[{"name":"assigned-key","value":"manual-fixture-secret"}]}))).await.unwrap();
    assert_eq!(response.status(),StatusCode::OK);let value=body(response).await;
    assert_eq!(value["status"],"stored");assert_eq!(value["verified"],false);assert_eq!(value["changed"],true);
    assert!(!value.to_string().contains("manual-fixture-secret"));
    assert_eq!(fixture.grants.read_json("manual").await.unwrap().unwrap()["manual"]["assigned-key"],"manual-fixture-secret");
    let state=fixture.manager.servers(vec!["manual".into()],None).await.unwrap();
    assert_eq!(state["servers"][0]["authentication"]["status"],"unknown");assert!(state["servers"][0]["authentication"]["last_verified_at"].is_null());
    assert!(fixture.manager.runner.server_ids().await.is_empty(),"saving a manual key never starts its server");
    let before=fixture.grants.read_json("manual").await.unwrap();
    let response=fixture.app.clone().oneshot(manual_request(&session,json!({"set":[{"name":"unassigned","value":"must-not-save"}]}))).await.unwrap();
    assert_eq!(response.status(),StatusCode::BAD_REQUEST);assert_eq!(fixture.grants.read_json("manual").await.unwrap(),before);
    let response=fixture.app.clone().oneshot(manual_request(&session,json!({"remove":["assigned-key"]}))).await.unwrap();
    assert_eq!(response.status(),StatusCode::OK);let removed=body(response).await;assert_eq!(removed["status"],"stored");assert_eq!(removed["verified"],false);
    assert!(fixture.grants.read_json("manual").await.unwrap().unwrap()["manual"].get("assigned-key").is_none());
}
struct MissingReadback;
impl crate::remote_oauth::Backend for MissingReadback {
    fn read(&self,_:&str,_:&str)->Result<Option<zeroize::Zeroizing<Vec<u8>>>,crate::remote_oauth::GrantError>{Ok(None)}
    fn write(&self,_:&str,_:&str,_:&[u8])->Result<(),crate::remote_oauth::GrantError>{Ok(())}
}
#[tokio::test]
async fn manual_patch_cannot_report_stored_when_keychain_readback_fails() {
    let fixture=Fixture::with_backend(NativeAuthority::fixture(),Some(Arc::new(MissingReadback)));let session=fixture.native.open_settings();
    fixture.manager.repository.save("manual",&manual_record()).unwrap();
    let response=fixture.app.clone().oneshot(manual_request(&session,json!({"set":[{"name":"assigned-key","value":"manual-fixture-secret"}]}))).await.unwrap();
    assert_eq!(response.status(),StatusCode::SERVICE_UNAVAILABLE);let value=body(response).await;
    assert_eq!(value["status"],"failed");assert!(!value.to_string().contains("manual-fixture-secret"));assert_ne!(value["verified"],true);
    let state=fixture.manager.servers(vec!["manual".into()],None).await.unwrap();assert_ne!(state["servers"][0]["authentication"]["status"],"authenticated");
    assert!(fixture.manager.runner.server_ids().await.is_empty());
}
