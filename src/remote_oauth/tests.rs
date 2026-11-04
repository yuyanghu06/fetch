//! Independent fake-Keychain checks: never accesses the native OS store.
use super::*;
use oauth2::TokenResponse;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Default)]
struct FixtureBackend {
    values:Mutex<BTreeMap<(String,String),Vec<u8>>>,
    deny_read:AtomicBool, deny_write:AtomicBool, corrupt_write:AtomicBool,
    fail_readback:AtomicBool, readback_pending:AtomicBool, writes:AtomicUsize,
}
impl Backend for FixtureBackend {
    fn read(&self,service:&str,account:&str)->Result<Option<Zeroizing<Vec<u8>>>,GrantError>{
        if self.deny_read.load(Ordering::SeqCst) || (self.fail_readback.load(Ordering::SeqCst) && self.readback_pending.swap(false,Ordering::SeqCst)) {return Err(GrantError::StorageUnavailable);}
        Ok(self.values.lock().unwrap().get(&(service.into(),account.into())).map(|v|Zeroizing::new(v.clone())))
    }
    fn write(&self,service:&str,account:&str,value:&[u8])->Result<(),GrantError>{
        if self.deny_write.load(Ordering::SeqCst){return Err(GrantError::StorageUnavailable);}
        self.writes.fetch_add(1,Ordering::SeqCst);
        let value=if self.corrupt_write.load(Ordering::SeqCst){b"wrong readback".to_vec()}else{value.to_vec()};
        self.values.lock().unwrap().insert((service.into(),account.into()),value);
        self.readback_pending.store(true,Ordering::SeqCst);Ok(())
    }
}

#[tokio::test]
async fn storage_separates_roots_and_instances_without_writing_provider_files() {
    let temporary=tempfile::tempdir().unwrap();let root=temporary.path().join("first-root");let other=temporary.path().join("other-root");
    let backend=Arc::new(FixtureBackend::default());
    let first=GrantStore::with_backend(root.clone(),backend.clone());let second=GrantStore::with_backend(other.clone(),backend.clone());
    first.save_section("personal","manual",json!({"key":"personal-fixture"})).await.unwrap();
    first.save_section("work","manual",json!({"key":"work-fixture"})).await.unwrap();
    second.save_section("personal","manual",json!({"key":"other-root-fixture"})).await.unwrap();
    assert_eq!(first.read_json("personal").await.unwrap().unwrap()["manual"]["key"],"personal-fixture");
    assert_eq!(first.read_json("work").await.unwrap().unwrap()["manual"]["key"],"work-fixture");
    assert_eq!(second.read_json("personal").await.unwrap().unwrap()["manual"]["key"],"other-root-fixture");
    assert_eq!(backend.values.lock().unwrap().len(),3);assert!(!root.exists());assert!(!other.exists());
}

#[tokio::test]
async fn concurrent_manual_and_sdk_updates_preserve_both_sections() {
    let temporary=tempfile::tempdir().unwrap();let store=Arc::new(GrantStore::memory(temporary.path().into()));
    for generation in 0..12 {
        let (manual,sdk)=tokio::join!(store.save_section("server","manual",json!({"key":format!("fixed-{generation}")})),
            store.save_section("server","sdk",json!({"refresh":format!("rotated-{generation}")})));
        manual.unwrap();sdk.unwrap();let document=store.read_json("server").await.unwrap().unwrap();
        assert_eq!(document["manual"]["key"],format!("fixed-{generation}"));
        assert_eq!(document["sdk"]["refresh"],format!("rotated-{generation}"));
    }
}

#[tokio::test]
async fn denied_keychain_read_or_write_never_creates_plaintext_fallback() {
    let temporary=tempfile::tempdir().unwrap();let root=temporary.path().join("state");let backend=Arc::new(FixtureBackend::default());
    let store=GrantStore::with_backend(root.clone(),backend.clone());
    backend.deny_read.store(true,Ordering::SeqCst);
    assert_eq!(store.read_json("server").await.unwrap_err(),GrantError::StorageUnavailable);
    assert_eq!(store.save_section("server","manual",json!({"key":"fixture-secret"})).await.unwrap_err(),GrantError::StorageUnavailable);
    backend.deny_read.store(false,Ordering::SeqCst);backend.deny_write.store(true,Ordering::SeqCst);
    assert_eq!(store.save_section("server","manual",json!({"key":"fixture-secret"})).await.unwrap_err(),GrantError::StorageUnavailable);
    assert!(!root.exists());assert!(backend.values.lock().unwrap().is_empty());assert_eq!(backend.writes.load(Ordering::SeqCst),0);
}

#[tokio::test]
async fn committed_write_with_failed_or_different_readback_is_not_success() {
    for mismatch in [false,true] {
        let temporary=tempfile::tempdir().unwrap();let backend=Arc::new(FixtureBackend::default());
        let store=GrantStore::with_backend(temporary.path().into(),backend.clone());
        backend.fail_readback.store(!mismatch,Ordering::SeqCst);backend.corrupt_write.store(mismatch,Ordering::SeqCst);
        let error=store.save_section("server","manual",json!({"key":"fixture-secret"})).await.unwrap_err();
        assert_eq!(error,if mismatch{GrantError::IntegrityFailed}else{GrantError::StorageUnavailable});
        assert_eq!(backend.writes.load(Ordering::SeqCst),1);
        assert_eq!(backend.values.lock().unwrap().len(),1,"uncertain committed state is preserved, not deleted");
    }
}

#[tokio::test]
async fn malformed_or_cross_instance_existing_values_are_preserved_without_overwrite() {
    let temporary=tempfile::tempdir().unwrap();let backend=Arc::new(FixtureBackend::default());
    let store=GrantStore::with_backend(temporary.path().into(),backend.clone());
    store.save_section("one","manual",json!({"key":"one-fixture"})).await.unwrap();
    let original=backend.values.lock().unwrap().values().next().unwrap().clone();
    let key=(store.inner.service.clone(),account("two").unwrap());
    for value in [b"not json".to_vec(),original] {
        backend.values.lock().unwrap().insert(key.clone(),value.clone());
        let before=backend.writes.load(Ordering::SeqCst);
        assert!(store.read_json("two").await.is_err());
        assert!(store.save_section("two","manual",json!({"key":"must-not-overwrite"})).await.is_err());
        assert_eq!(backend.writes.load(Ordering::SeqCst),before);
        assert_eq!(backend.values.lock().unwrap()[&key],value);
    }
}

#[tokio::test]
async fn invalid_ids_documents_and_sections_fail_before_os_storage() {
    let temporary=tempfile::tempdir().unwrap();let backend=Arc::new(FixtureBackend::default());
    let store=GrantStore::with_backend(temporary.path().into(),backend.clone());
    for id in ["", "bad\0id", "fetch-internal:app-authorization"] {
        assert!(store.read_json(id).await.is_err());assert!(store.save_json(id,&json!({})).await.is_err());
    }
    assert!(store.save_json("server",&json!([])).await.is_err());
    assert!(store.save_section("server","provider",json!({})).await.is_err());
    assert!(store.save_json("server",&json!({"too_large":"a".repeat(MAX_GRANT_BYTES)})).await.is_err());
    assert_eq!(backend.writes.load(Ordering::SeqCst),0);
}

fn metadata()->rmcp::transport::auth::AuthorizationMetadata {
    serde_json::from_value(json!({"issuer":"https://issuer.fixture.example/","authorization_endpoint":"https://issuer.fixture.example/authorize",
        "token_endpoint":"https://issuer.fixture.example/token","registration_endpoint":"https://issuer.fixture.example/register",
        "authorization_response_iss_parameter_supported":true,"code_challenge_methods_supported":["S256"]})).unwrap()
}
fn credentials(access:&str,refresh:&str)->rmcp::transport::auth::StoredCredentials {
    serde_json::from_value(json!({"client_id":"fixture-client","token_response":{"access_token":access,"token_type":"Bearer","refresh_token":refresh,"expires_in":3600},
        "granted_scopes":[],"token_received_at":sdk::now(),"issuer":"https://issuer.fixture.example/"})).unwrap()
}

#[tokio::test]
async fn sdk_store_preserves_manual_values_and_rejects_service_or_issuer_rebinding() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    grants.save_section("server","manual",json!({"fixed":"manual-fixture"})).await.unwrap();
    let store=sdk::Store::new(grants.clone(),"server".into(),"https://mcp.fixture.example/mcp".into(),metadata());
    store.save(credentials("old-access","old-refresh")).await.unwrap();
    store.save(credentials("new-access","new-refresh")).await.unwrap();
    let loaded=store.load().await.unwrap().unwrap();assert_eq!(sdk::token(&loaded).unwrap(),"new-access");
    assert_eq!(loaded.token_response.unwrap().refresh_token().unwrap().secret(),"new-refresh");
    assert_eq!(grants.read_json("server").await.unwrap().unwrap()["manual"]["fixed"],"manual-fixture");
    let before=grants.read_json("server").await.unwrap().unwrap();
    let other=sdk::Store::new(grants.clone(),"server".into(),"https://different.fixture.example/mcp".into(),metadata());
    assert!(other.load().await.is_err());
    let mut wrong=credentials("wrong-access","wrong-refresh");wrong.issuer=Some("https://other-issuer.fixture.example/".into());
    assert!(store.save(wrong).await.is_err());assert!(store.clear().await.is_err());
    assert_eq!(grants.read_json("server").await.unwrap().unwrap(),before);
}

struct ScriptHttp {requests:Mutex<Vec<(String,String)>>, token_status:u16, iss_supported:bool}
impl ScriptHttp {fn new(token_status:u16)->Self{Self{requests:Mutex::new(vec![]),token_status,iss_supported:true}}}
impl rmcp::transport::auth::OAuthHttpClient for ScriptHttp {
    fn execute(&self,operation:rmcp::transport::auth::OAuthHttpRequest)->rmcp::transport::auth::OAuthHttpClientFuture<'_>{
        Box::pin(async move {
            let uri=operation.request.uri().to_string();let form=String::from_utf8_lossy(operation.request.body()).into_owned();
            self.requests.lock().unwrap().push((uri.clone(),form));
            let path=operation.request.uri().path();
            let (status,value)=if path.contains("oauth-protected-resource") {
                (200,json!({"resource":"https://mcp.fixture.example/mcp","authorization_servers":["https://issuer.fixture.example/"]}))
            }else if path.contains("oauth-authorization-server") || path.contains("openid-configuration") {
                let mut value=serde_json::to_value(metadata()).unwrap();value["authorization_response_iss_parameter_supported"]=json!(self.iss_supported);(200,value)
            }else if path=="/register" {
                let mut registration:Value=serde_json::from_slice(operation.request.body()).unwrap();
                registration["client_id"]=json!("fixture-client");
                (201,registration)
            }else if path=="/token" {
                if self.token_status==200{(200,json!({"access_token":"renewed-access-fixture","token_type":"Bearer","refresh_token":"rotated-refresh-fixture","expires_in":3600}))}
                else{(self.token_status,json!({"error":if self.token_status==400{"invalid_grant"}else{"temporarily_unavailable"}}))}
            }else{(404,json!({"error":"fixture-not-found"}))};
            Ok(oauth2::http::Response::builder().status(status).header("content-type","application/json").body(value.to_string().into_bytes()).unwrap())
        })
    }
}

#[tokio::test]
async fn official_sdk_refreshes_expired_saved_grant_and_saves_rotation_without_losing_manual_values() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    grants.save_section("server","manual",json!({"fixed":"manual-fixture"})).await.unwrap();
    let http=Arc::new(ScriptHttp::new(200));
    let mut prepared=sdk::Prepared::with_http(grants.clone(),"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    let mut old=credentials("old-access","old-refresh");old.token_received_at=Some(sdk::now()-7200);
    prepared.store.save(old).await.unwrap();
    assert_eq!(prepared.reuse().await.unwrap(),"renewed-access-fixture");
    let saved=prepared.store.snapshot().await.unwrap().unwrap();
    assert_eq!(saved.token_response.unwrap().refresh_token().unwrap().secret(),"rotated-refresh-fixture");
    let requests=http.requests.lock().unwrap();let token_calls:Vec<_>=requests.iter().filter(|(uri,_)|uri.ends_with("/token")).collect();
    assert_eq!(token_calls.len(),1);
    let fields:url::form_urlencoded::Parse<'_>=url::form_urlencoded::parse(token_calls[0].1.as_bytes());
    let fields:BTreeMap<_,_>=fields.into_owned().collect();assert_eq!(fields["grant_type"],"refresh_token");assert_eq!(fields["refresh_token"],"old-refresh");
    drop(requests);
    assert_eq!(grants.read_json("server").await.unwrap().unwrap()["manual"]["fixed"],"manual-fixture");
}

#[tokio::test]
async fn refresh_rejection_and_transient_failures_preserve_existing_state_and_do_not_start_login() {
    use rmcp::transport::auth::CredentialStore;
    for status in [400,503] {
        let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
        let http=Arc::new(ScriptHttp::new(status));let mut prepared=sdk::Prepared::with_http(grants.clone(),"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
        let mut old=credentials("old-access","old-refresh");old.token_received_at=Some(sdk::now()-7200);
        prepared.store.save(old).await.unwrap();let before=grants.read_json("server").await.unwrap().unwrap();
        assert!(prepared.reuse().await.is_err());
        assert_eq!(grants.read_json("server").await.unwrap().unwrap(),before);
        let requests=http.requests.lock().unwrap();
        assert_eq!(requests.iter().filter(|(uri,_)|uri.ends_with("/token")).count(),1);
        assert!(!requests.iter().any(|(uri,_)|uri.ends_with("/authorize")||uri.ends_with("/register")));
    }
}

/// All requests stay inside this fake client, including legacy discovery probes.
struct MetadataHttp {inner:ScriptHttp, document:Option<Value>}
impl rmcp::transport::auth::OAuthHttpClient for MetadataHttp {
    fn execute(&self,operation:rmcp::transport::auth::OAuthHttpRequest)->rmcp::transport::auth::OAuthHttpClientFuture<'_>{Box::pin(async move{
        let path=operation.request.uri().path();
        let discovery=path.contains("oauth-authorization-server")||path.contains("openid-configuration");
        if discovery || (self.document.is_none() && path!="/register" && path!="/token") {
            self.inner.requests.lock().unwrap().push((operation.request.uri().to_string(),String::new()));
            let(status,value)=if discovery {self.document.as_ref().map(|value|(200,value.clone())).unwrap_or((404,json!({"error":"fixture-not-found"})))}else{(404,json!({"error":"fixture-not-found"}))};
            return Ok(oauth2::http::Response::builder().status(status).header("content-type","application/json").body(value.to_string().into_bytes()).unwrap());
        }
        self.inner.execute(operation).await
    })}
}
fn callback_state(session:&rmcp::transport::auth::AuthorizationSession)->String {
    url::Url::parse(session.get_authorization_url()).unwrap().query_pairs().find(|(name,_)|name=="state").unwrap().1.into_owned()
}

#[tokio::test]
async fn unadvertised_issuer_support_completes_sdk_login_without_callback_issuer() {
    for advertised in [Some(false),None] {
        let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
        let mut document=serde_json::to_value(metadata()).unwrap();
        if let Some(value)=advertised{document["authorization_response_iss_parameter_supported"]=json!(value);}else{document.as_object_mut().unwrap().remove("authorization_response_iss_parameter_supported");}
        let http=Arc::new(MetadataHttp{inner:ScriptHttp::new(200),document:Some(document)});
        let prepared=sdk::Prepared::with_http(grants.clone(),"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
        let store=prepared.store.clone();let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
        assert!(session.get_authorization_url().starts_with("https://issuer.fixture.example/authorize?"));
        assert!(store.snapshot().await.unwrap().is_none(),"no grant is saved before the callback");
        session.handle_callback_with_issuer("code-fixture",&callback_state(&session),None).await.unwrap();
        assert_eq!(sdk::token(&store.snapshot().await.unwrap().unwrap()).as_deref(),Some("renewed-access-fixture"));
        assert_eq!(http.inner.requests.lock().unwrap().iter().filter(|(uri,_)|uri.ends_with("/token")).count(),1);
    }
}

#[tokio::test]
async fn sdk_legacy_endpoint_fallback_can_finish_without_issuer_metadata() {
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    let http=Arc::new(MetadataHttp{inner:ScriptHttp::new(200),document:None});
    let prepared=sdk::Prepared::with_http(grants,"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    assert!(prepared.store.metadata.issuer.is_none());
    let store=prepared.store.clone();let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
    assert!(session.get_authorization_url().starts_with("https://mcp.fixture.example/authorize?"));
    session.handle_callback_with_issuer("legacy-code-fixture",&callback_state(&session),None).await.unwrap();
    let saved=store.snapshot().await.unwrap().unwrap();assert!(saved.issuer.is_none());assert_eq!(sdk::token(&saved).as_deref(),Some("renewed-access-fixture"));
    assert!(http.inner.requests.lock().unwrap().iter().any(|(uri,_)|uri=="https://mcp.fixture.example/register"));
}

#[tokio::test]
async fn unused_registration_endpoint_does_not_block_saved_login_reuse() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    let mut document=serde_json::to_value(metadata()).unwrap();document["registration_endpoint"]=json!("http://127.0.0.1/unused-register");
    let http=Arc::new(MetadataHttp{inner:ScriptHttp::new(200),document:Some(document)});
    let mut prepared=sdk::Prepared::with_http(grants,"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    prepared.store.save(credentials("saved-access-fixture","saved-refresh-fixture")).await.unwrap();
    assert_eq!(prepared.reuse().await.unwrap(),"saved-access-fixture");
    assert!(!http.inner.requests.lock().unwrap().iter().any(|(uri,_)|uri.contains("unused-register")));
}

#[tokio::test]
async fn harmless_metadata_changes_reuse_saved_login_but_changed_issuer_does_not() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    let original=sdk::Store::new(grants.clone(),"server".into(),"https://mcp.fixture.example/mcp".into(),metadata());
    original.save(credentials("saved-access-fixture","saved-refresh-fixture")).await.unwrap();
    let before=grants.read_json("server").await.unwrap().unwrap();
    let mut document=serde_json::to_value(metadata()).unwrap();document["service_documentation"]=json!("https://issuer.fixture.example/updated-docs");document["scopes_supported"]=json!(["new_optional_scope"]);
    let http=Arc::new(MetadataHttp{inner:ScriptHttp::new(200),document:Some(document)});
    let mut prepared=sdk::Prepared::with_http(grants.clone(),"server","https://mcp.fixture.example/mcp",http).await.unwrap();
    assert_eq!(prepared.reuse().await.unwrap(),"saved-access-fixture");
    assert_eq!(grants.read_json("server").await.unwrap().unwrap(),before,"read-only reuse must not rewrite the account");
    let mut changed=metadata();changed.issuer=Some("https://different-issuer.fixture.example/".into());
    let other=sdk::Store::new(grants.clone(),"server".into(),"https://mcp.fixture.example/mcp".into(),changed);
    assert!(other.load().await.is_err());assert_eq!(grants.read_json("server").await.unwrap().unwrap(),before);
}

#[tokio::test]
async fn sdk_still_requires_callback_issuer_when_service_advertises_support() {
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let http=Arc::new(ScriptHttp::new(200));
    let prepared=sdk::Prepared::with_http(grants,"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
    assert!(matches!(session.handle_callback_with_issuer("code-fixture",&callback_state(&session),None).await,Err(rmcp::transport::auth::AuthError::AuthorizationServerMissingIssuer{..})));
    assert!(!http.requests.lock().unwrap().iter().any(|(uri,_)|uri.ends_with("/token")));
}

#[tokio::test]
async fn a_saved_grant_cannot_be_rebound_to_a_different_oauth_client() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));
    let store=sdk::Store::new(grants.clone(),"server".into(),"https://mcp.fixture.example/mcp".into(),metadata());
    store.save(credentials("access-fixture","refresh-fixture")).await.unwrap();
    let mut value=grants.read_json("server").await.unwrap().unwrap();value["sdk"]["credentials"]["client_id"]=json!("different-client");
    grants.save_json("server",&value).await.unwrap();
    assert!(store.load().await.is_err());assert_eq!(grants.read_json("server").await.unwrap().unwrap(),value);
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn renewal_uses_the_actual_same_https_authenticate_route_and_get_is_only_a_projection() {
    use crate::{api::{AppState,router},authority::PendingNativeAuthority,discovery::DiscoveryService,
        installation::Installer,manager::Manager,runner::{Runner,TransportConfig},storage::ServerRepository};
    use axum::{body::Body,extract::Request,middleware::Next};
    use futures_util::FutureExt;
    use rmcp::transport::auth::CredentialStore;
    use std::time::Duration;
    let temporary=tempfile::tempdir().unwrap();let root=temporary.path().join("state");
    let repository=ServerRepository::new(root.clone());let grants=Arc::new(GrantStore::memory(root.clone()));
    let discovery=DiscoveryService::new(temporary.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),grants.clone(),Runner::new(),Installer::new(root.clone(),discovery.clone()));
    let record=json!({"server_id":"remote","name":"remote","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},
        "restart_required":false,"restart":null,"credentials":[],"config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.fixture.example/mcp","transport":"streamable_http","headers":[]}}});
    repository.save("remote",&record).unwrap();
    let http=Arc::new(ScriptHttp::new(200));
    let prepared=sdk::Prepared::with_http(grants.clone(),"remote","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    prepared.store.save(credentials("initial-access-fixture","initial-refresh-fixture")).await.unwrap();
    manager.set_oauth_http(http.clone());let verifications=Arc::new(AtomicUsize::new(0));let verified=verifications.clone();
    manager.set_authentication_verifier(Arc::new(move|transport|{let verified=verified.clone();Box::pin(async move{
        let TransportConfig::Http{headers,..}=transport else{panic!("expected HTTPS MCP transport")};
        assert!(matches!(headers.get("Authorization").map(String::as_str),Some("Bearer initial-access-fixture"|"Bearer renewed-access-fixture")));
        verified.fetch_add(1,Ordering::SeqCst);Ok(())
    })}));
    let listener=std::net::TcpListener::bind("127.0.0.1:0").unwrap();listener.set_nonblocking(true).unwrap();
    let origin=format!("https://localhost:{}",listener.local_addr().unwrap().port());manager.set_origin(origin.clone());
    let rcgen::CertifiedKey{cert,signing_key}=rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let tls=axum_server::tls_rustls::RustlsConfig::from_der(vec![cert.der().to_vec()],signing_key.serialize_der()).await.unwrap();
    let observed=Arc::new(AtomicUsize::new(0));let request_count=observed.clone();let expected=manager.background_token();
    let mut state=AppState::new(origin.clone(),repository.clone(),discovery,Arc::new(PendingNativeAuthority)).unwrap();state.manager=Some(manager.clone());
    let routes=router(Arc::new(state)).layer(axum::middleware::from_fn(move|request:Request,next:Next|{
        let observed=request_count.clone();let expected=expected.clone();async move{
            if request.uri().path().ends_with("/authenticate") {
                assert_eq!(request.method(),axum::http::Method::POST);
                assert_eq!(request.headers().get("x-fetch-background-auth").unwrap(),expected.as_str());
                assert!(!request.headers().contains_key("authorization"));assert!(!request.headers().contains_key("x-fetch-settings-capability"));
                let (parts,body)=request.into_parts();let bytes=axum::body::to_bytes(body,1024).await.unwrap();
                assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(),json!({"interactive":false}));
                observed.fetch_add(1,Ordering::SeqCst);next.run(Request::from_parts(parts,Body::from(bytes))).await
            }else{next.run(request).await}
        }
    }));
    let handle=axum_server::Handle::new();let server_handle=handle.clone();
    let server=tokio::spawn(async move{axum_server::from_tcp_rustls(listener,tls).unwrap().handle(server_handle).serve(routes.into_make_service()).await});
    let result=std::panic::AssertUnwindSafe(async {
        assert_eq!(manager.authenticate_mode("remote",false).await.unwrap()["status"],"authenticated");
        assert_eq!(verifications.load(Ordering::SeqCst),1);
        // Expire the SDK token and cached clock independently. Neither act proves
        // authentication; only the injected accepted-request check above does.
        let mut expired=credentials("initial-access-fixture","initial-refresh-fixture");expired.token_received_at=Some(sdk::now()-7200);
        prepared.store.save(expired).await.unwrap();manager.set_authentication_expiry("remote",Some(sdk::now()-1)).await;
        let before=grants.read_json("remote").await.unwrap().unwrap();let requests_before=http.requests.lock().unwrap().len();
        for _ in 0..3 {
            let read=manager.servers(vec!["remote".into()],None).await.unwrap();let auth=&read["servers"][0]["authentication"];
            assert_eq!(auth["status"],"unknown");
            assert!(chrono::DateTime::parse_from_rfc3339(auth["access_expires_at"].as_str().unwrap()).is_ok());
            assert!(chrono::DateTime::parse_from_rfc3339(auth["last_verified_at"].as_str().unwrap()).is_ok());
        }
        assert_eq!(http.requests.lock().unwrap().len(),requests_before);assert_eq!(grants.read_json("remote").await.unwrap().unwrap(),before);
        crate::application::configure_authenticate_sender(&manager,&origin,&cert.pem()).unwrap();
        tokio::time::timeout(Duration::from_secs(5),manager.before_remote_use("remote")).await.unwrap().unwrap();
        assert_eq!(observed.load(Ordering::SeqCst),1,"production sender must enter the actual HTTPS route");
        assert_eq!(verifications.load(Ordering::SeqCst),2);
        let saved=prepared.store.snapshot().await.unwrap().unwrap();assert_eq!(sdk::token(&saved).unwrap(),"renewed-access-fixture");
        assert_eq!(saved.token_response.unwrap().refresh_token().unwrap().secret(),"rotated-refresh-fixture");
        let read=manager.servers(vec!["remote".into()],None).await.unwrap();assert_eq!(read["servers"][0]["authentication"]["status"],"authenticated");
        assert_eq!(http.requests.lock().unwrap().iter().filter(|(uri,_)|uri.ends_with("/token")).count(),1);
    }).catch_unwind().await;
    manager.shutdown().await;handle.graceful_shutdown(Some(Duration::from_secs(1)));
    tokio::time::timeout(Duration::from_secs(3),server).await.unwrap().unwrap().unwrap();
    if let Err(error)=result{std::panic::resume_unwind(error);}
}

struct BlockingHttp {inner:ScriptHttp,entered:tokio::sync::Notify,release:tokio::sync::Notify}
impl rmcp::transport::auth::OAuthHttpClient for BlockingHttp {
    fn execute(&self,operation:rmcp::transport::auth::OAuthHttpRequest)->rmcp::transport::auth::OAuthHttpClientFuture<'_>{Box::pin(async move{
        if operation.request.uri().path()=="/token"{self.entered.notify_one();self.release.notified().await;}
        self.inner.execute(operation).await
    })}
}
#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn status_read_stays_responsive_while_the_same_servers_renewal_is_waiting() {
    use crate::{discovery::DiscoveryService,installation::Installer,manager::Manager,runner::Runner,storage::ServerRepository};
    use rmcp::transport::auth::CredentialStore;
    use std::time::Duration;
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");let grants=Arc::new(GrantStore::memory(root.clone()));
    let repository=ServerRepository::new(root.clone());let discovery=DiscoveryService::new(directory.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),grants.clone(),Runner::new(),Installer::new(root,discovery));
    repository.save("remote",&json!({"server_id":"remote","name":"remote","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},
        "restart_required":false,"restart":null,"credentials":[],"config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.fixture.example/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
    let http=Arc::new(BlockingHttp{inner:ScriptHttp::new(200),entered:tokio::sync::Notify::new(),release:tokio::sync::Notify::new()});
    let prepared=sdk::Prepared::with_http(grants,"remote","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
    let mut expired=credentials("old-access","old-refresh");expired.token_received_at=Some(sdk::now()-7200);prepared.store.save(expired).await.unwrap();
    manager.set_oauth_http(http.clone());manager.set_authentication_verifier(Arc::new(|_|Box::pin(async{Ok(())})));
    let pending={let manager=manager.clone();tokio::spawn(async move{manager.authenticate_mode("remote",false).await})};
    tokio::time::timeout(Duration::from_secs(2),http.entered.notified()).await.unwrap();
    let observed=tokio::time::timeout(Duration::from_millis(200),manager.servers(vec!["remote".into()],None)).await;
    // Always release and stop the owned future, including when the read is blocked.
    http.release.notify_one();let authenticated=tokio::time::timeout(Duration::from_secs(3),pending).await.unwrap().unwrap();
    manager.shutdown().await;
    let state=observed.expect("GET must not wait on the authentication operation lock").unwrap();
    assert_eq!(state["servers"][0]["authentication"]["status"],"unknown");assert_eq!(authenticated.unwrap()["status"],"authenticated");
}

#[tokio::test]
async fn callback_state_and_issuer_mismatches_never_exchange_a_code() {
    for wrong_issuer in [false,true] {
        let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let http=Arc::new(ScriptHttp::new(200));
        let prepared=sdk::Prepared::with_http(grants,"server","https://mcp.fixture.example/mcp",http.clone()).await.unwrap();
        let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
        let address=url::Url::parse(session.get_authorization_url()).unwrap();
        let state=address.query_pairs().find(|(key,_)|key=="state").unwrap().1.into_owned();
        let state=if wrong_issuer{state}else{"wrong-state-fixture".into()};
        let issuer=if wrong_issuer{"https://wrong-issuer.fixture.example/"}else{"https://issuer.fixture.example/"};
        assert!(session.handle_callback_with_issuer("code-fixture",&state,Some(issuer)).await.is_err());
        assert!(!http.requests.lock().unwrap().iter().any(|(uri,_)|uri.ends_with("/token")));
    }
}

struct OmittedRefreshHttp(ScriptHttp);
impl rmcp::transport::auth::OAuthHttpClient for OmittedRefreshHttp {
    fn execute(&self,operation:rmcp::transport::auth::OAuthHttpRequest)->rmcp::transport::auth::OAuthHttpClientFuture<'_>{Box::pin(async move{
        if operation.request.uri().path()=="/token" {
            self.0.requests.lock().unwrap().push((operation.request.uri().to_string(),String::from_utf8_lossy(operation.request.body()).into_owned()));
            return Ok(oauth2::http::Response::builder().status(200).header("content-type","application/json")
                .body(json!({"access_token":"renewed-access-fixture","token_type":"Bearer","expires_in":3600}).to_string().into_bytes()).unwrap());
        }
        self.0.execute(operation).await
    })}
}
#[tokio::test]
async fn official_sdk_retains_prior_refresh_token_when_a_successful_renewal_omits_replacement() {
    use rmcp::transport::auth::CredentialStore;
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let http=Arc::new(OmittedRefreshHttp(ScriptHttp::new(200)));
    let mut prepared=sdk::Prepared::with_http(grants,"server","https://mcp.fixture.example/mcp",http).await.unwrap();
    let mut old=credentials("old-access","still-valid-refresh");old.token_received_at=Some(sdk::now()-7200);prepared.store.save(old).await.unwrap();
    assert_eq!(prepared.reuse().await.unwrap(),"renewed-access-fixture");
    let saved=prepared.store.snapshot().await.unwrap().unwrap();
    assert_eq!(saved.token_response.unwrap().refresh_token().unwrap().secret(),"still-valid-refresh");
}


#[tokio::test]
async fn callback_route_ignores_provider_extensions_but_rejects_duplicate_routing_fields() {
    use crate::{api::{AppState,router},authority::PendingNativeAuthority,discovery::DiscoveryService,installation::Installer,manager::Manager,runner::Runner,storage::ServerRepository};
    use axum::{body::Body,http::{Request,StatusCode,header}};
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    let temporary=tempfile::tempdir().unwrap();let root=temporary.path().join("state");
    let repository=ServerRepository::new(root.clone());let grants=Arc::new(GrantStore::memory(root.clone()));
    let discovery=DiscoveryService::new(temporary.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),grants,Runner::new(),Installer::new(root,discovery.clone()));
    let mut state=AppState::new("https://localhost:53672".into(),repository,discovery,Arc::new(PendingNativeAuthority)).unwrap();state.manager=Some(manager.clone());let app=router(Arc::new(state));
    // An unknown attempt gives Manager's HTML failure page. Parser rejections are JSON.
    let base="state=unknown-fixture&code=code-fixture&iss=https%3A%2F%2Fissuer.fixture.example%2F&error=access_denied";
    for extra in ["scope=openid%20profile&session_state=provider-session-fixture","provider_extension=one&provider_extension=two"] {
        let response=app.clone().oneshot(Request::builder().uri(format!("/oauth/provider/callback?{base}&{extra}")).header(header::HOST,"localhost:53672").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(),StatusCode::BAD_REQUEST);assert!(response.headers()[header::CONTENT_TYPE].to_str().unwrap().starts_with("text/html"));
        let body=response.into_body().collect().await.unwrap().to_bytes();assert!(String::from_utf8_lossy(&body).contains("Provider setup did not finish successfully"));
    }
    for duplicate in ["state=second","code=second","iss=second","error=second"] {
        let response=app.clone().oneshot(Request::builder().uri(format!("/oauth/provider/callback?{base}&{duplicate}")).header(header::HOST,"localhost:53672").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(),StatusCode::BAD_REQUEST);assert!(response.headers()[header::CONTENT_TYPE].to_str().unwrap().starts_with("application/json"));
        let body=response.into_body().collect().await.unwrap().to_bytes();assert_eq!(serde_json::from_slice::<Value>(&body).unwrap()["error"]["code"],"INVALID_REQUEST");
    }
    manager.shutdown().await;
}


#[test]
fn removed_metadata_preflight_does_not_allow_private_or_plain_http_destinations() {
    for endpoint in ["http://127.0.0.1/unused-register","https://127.0.0.1/unused-register"] {
        assert!(crate::manager::network::checked_url(endpoint).is_err());
    }
    assert!(crate::manager::network::checked_url("https://issuer.fixture.example/token").is_ok());
}
