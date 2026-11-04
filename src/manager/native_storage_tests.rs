use super::*;
use tempfile::TempDir;
fn local(id:&str)->Value{json!({"server_id":id,"name":"native fixture","config":{"kind":"local_script","executable":"/usr/bin/true","args":[],"working_directory":null,"environment":[]}})}
#[test]
fn unmodified_instances_have_distinct_home_and_xdg_paths(){
    let root=TempDir::new().unwrap();let first=native_state::transport(root.path(),&local("first"),&json!({"manual":{}})).unwrap();let second=native_state::transport(root.path(),&local("second"),&json!({"manual":{}})).unwrap();
    let (TransportConfig::Stdio{executable,args,environment:first,..},TransportConfig::Stdio{environment:second,..})=(first,second)else{panic!("stdio")};
    assert_eq!(executable,std::path::PathBuf::from("/usr/bin/true"));assert!(args.is_empty());assert_ne!(first["HOME"],second["HOME"]);assert_ne!(first["XDG_CONFIG_HOME"],second["XDG_CONFIG_HOME"]);assert_ne!(first["XDG_STATE_HOME"],second["XDG_STATE_HOME"]);
}
#[test]
fn native_setup_never_guesses_package_commands(){
    let mut record=local("unknown");record["config"]["package"]=json!({"name":"@a-bonus/google-docs-mcp"});
    assert_eq!(native_state::setup(&record).err().unwrap().code,"SERVER_SETUP_UNSUPPORTED");
}
#[test]
fn fixed_keys_only_reach_their_assigned_environment_names(){
    let root=TempDir::new().unwrap();let mut record=local("manual");record["config"]["environment"]=json!([{"name":"EXAMPLE_KEY","credential_ref":"assigned"}]);
    let TransportConfig::Stdio{environment,..}=native_state::transport(root.path(),&record,&json!({"manual":{"assigned":"fixture","unassigned":"private"}})).unwrap()else{panic!("stdio")};
    assert_eq!(environment["EXAMPLE_KEY"],"fixture");assert!(!environment.values().any(|value|value=="private"));
}
#[test]
fn missing_fixed_key_is_explicit_and_does_not_fall_back_to_process_environment(){
    let root=TempDir::new().unwrap();let mut record=local("missing");record["config"]["environment"]=json!([{"name":"PATH","credential_ref":"missing"}]);
    assert_eq!(native_state::transport(root.path(),&record,&json!({"manual":{}})).err().unwrap().code,"SERVER_AUTH_REQUIRED");
}
#[test]
fn public_timestamps_are_utc_strings_or_null(){assert_eq!(timestamp(Some(0)),json!("1970-01-01T00:00:00Z"));assert_eq!(timestamp(None),Value::Null);}

struct OAuthFixture {resource:String,tokens:std::sync::atomic::AtomicUsize}
impl OAuthFixture {fn metadata()->rmcp::transport::auth::AuthorizationMetadata{serde_json::from_value(json!({"issuer":"https://issuer.fixture.example/","authorization_endpoint":"https://issuer.fixture.example/authorize","token_endpoint":"https://issuer.fixture.example/token","registration_endpoint":"https://issuer.fixture.example/register","authorization_response_iss_parameter_supported":true,"code_challenge_methods_supported":["S256"]})).unwrap()}}
impl rmcp::transport::auth::OAuthHttpClient for OAuthFixture{
    fn execute(&self,request:rmcp::transport::auth::OAuthHttpRequest)->rmcp::transport::auth::OAuthHttpClientFuture<'_>{Box::pin(async move{
        let path=request.request.uri().path();let (status,value)=if path.contains("oauth-protected-resource"){(200,json!({"resource":self.resource,"authorization_servers":["https://issuer.fixture.example/"]}))}else if path.contains("oauth-authorization-server")||path.contains("openid-configuration"){(200,serde_json::to_value(Self::metadata()).unwrap())}else if path=="/register"{let mut registration:Value=serde_json::from_slice(request.request.body()).unwrap();registration["client_id"]=json!("fixture-client");(201,registration)}else if path=="/token"{self.tokens.fetch_add(1,Ordering::SeqCst);(200,json!({"access_token":"new-fixture","refresh_token":"new-refresh-fixture","token_type":"Bearer","expires_in":3600}))}else{(404,json!({}))};
        Ok(oauth2::http::Response::builder().status(status).header("content-type","application/json").body(value.to_string().into_bytes()).unwrap())
    })}
}
fn fixture_credentials(expired:bool)->rmcp::transport::auth::StoredCredentials{serde_json::from_value(json!({"client_id":"fixture-client","token_response":{"access_token":"old-fixture","refresh_token":"old-refresh-fixture","token_type":"Bearer","expires_in":3600},"granted_scopes":[],"token_received_at":if expired{sdk::now()-7200}else{sdk::now()},"issuer":"https://issuer.fixture.example/"})).unwrap()}
fn remote_record(endpoint:&str)->Value{json!({"server_id":"remote","name":"remote","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":endpoint,"transport":"streamable_http","headers":[]}}})}
fn manager_fixture(root:&std::path::Path,endpoint:&str)->(Arc<Manager>,Arc<GrantStore>,Arc<OAuthFixture>){
    let root=root.join("fetch-state");let repository=ServerRepository::new(root.clone());repository.save("remote",&remote_record(endpoint)).unwrap();let grants=Arc::new(GrantStore::memory(root.clone()));let discovery=crate::discovery::DiscoveryService::new(root.join("empty-home"),None);let manager=Manager::with_grant_store(repository,grants.clone(),Runner::new(),Installer::new(root,discovery));let http=Arc::new(OAuthFixture{resource:endpoint.into(),tokens:std::sync::atomic::AtomicUsize::new(0)});manager.set_oauth_http(http.clone());(manager,grants,http)
}
#[tokio::test]
async fn revoked_saved_access_requires_quiet_handoff_and_explicit_login_recovers(){
    use rmcp::transport::auth::CredentialStore;
    let root=TempDir::new().unwrap();let endpoint="https://mcp.fixture.example/mcp";let (manager,grants,http)=manager_fixture(root.path(),endpoint);manager.set_origin("https://localhost:53672".into());
    let store=sdk::Store::new(grants,"remote".into(),endpoint.into(),OAuthFixture::metadata());store.save(fixture_credentials(false)).await.unwrap();
    manager.set_authentication_verifier(Arc::new(|transport|Box::pin(async move{let TransportConfig::Http{headers,..}=transport else{panic!("http")};if headers["Authorization"]=="Bearer old-fixture"{Err(ManagementError::new(502,"PROVIDER_AUTH_REQUIRED","fixture rejected old token"))}else{Ok(())}})));
    let opened=Arc::new(std::sync::Mutex::new(Vec::new()));let urls=opened.clone();manager.set_browser_opener(Arc::new(move|url|{urls.lock().unwrap().push(url);Box::pin(async{Ok(json!({}))})}));
    assert_eq!(manager.authenticate_mode("remote",false).await.unwrap_err().code,"SERVER_AUTH_REQUIRED");assert!(opened.lock().unwrap().is_empty());assert_eq!(http.tokens.load(Ordering::SeqCst),0);
    assert_eq!(manager.authenticate("remote").await.unwrap()["status"],"authenticating");let url=url::Url::parse(&opened.lock().unwrap()[0]).unwrap();let state=url.query_pairs().find(|(name,_)|name=="state").unwrap().1.into_owned();
    manager.provider_callback(&state,Some("fixture-code"),None,Some("https://issuer.fixture.example/")).await.unwrap();
    assert_eq!(manager.servers(vec!["remote".into()],None).await.unwrap()["servers"][0]["authentication"]["status"],"authenticated");assert_eq!(sdk::token(&store.snapshot().await.unwrap().unwrap()).unwrap(),"new-fixture");assert_eq!(http.tokens.load(Ordering::SeqCst),1);manager.shutdown().await;
}
#[tokio::test]
async fn transient_and_unprovable_saved_access_never_open_a_login(){
    use rmcp::transport::auth::CredentialStore;
    for code in ["AUTHENTICATION_CHECK_UNSUPPORTED","SERVER_UNAVAILABLE"]{
        let root=TempDir::new().unwrap();let endpoint="https://mcp.fixture.example/mcp";let(manager,grants,_)=manager_fixture(root.path(),endpoint);manager.set_origin("https://localhost:53672".into());sdk::Store::new(grants,"remote".into(),endpoint.into(),OAuthFixture::metadata()).save(fixture_credentials(false)).await.unwrap();
        manager.set_authentication_verifier(Arc::new(move|_|Box::pin(async move{Err(ManagementError::new(503,code,"fixture failure"))})));
        let opened=Arc::new(std::sync::atomic::AtomicUsize::new(0));let count=opened.clone();manager.set_browser_opener(Arc::new(move|_|{count.fetch_add(1,Ordering::SeqCst);Box::pin(async{Ok(json!({}))})}));
        assert_eq!(manager.authenticate("remote").await.unwrap_err().code,code);assert_eq!(opened.load(Ordering::SeqCst),0);manager.shutdown().await;
    }
}

#[derive(Default)]
struct DrainingMcp {entered:Notify,release:Notify,initializations:std::sync::Mutex<Vec<String>>}
async fn draining_mcp(axum::extract::State(state):axum::extract::State<Arc<DrainingMcp>>,headers:axum::http::HeaderMap,axum::Json(request):axum::Json<Value>)->axum::response::Response{
    use axum::response::IntoResponse;
    let method=request["method"].as_str().unwrap_or("");
    if method=="notifications/initialized"{return axum::http::StatusCode::ACCEPTED.into_response();}
    let result=match method{
        "initialize"=>{state.initializations.lock().unwrap().push(headers.get("authorization").unwrap().to_str().unwrap().into());json!({"protocolVersion":"2025-11-25","capabilities":{"tools":{}},"serverInfo":{"name":"drain-fixture","version":"1"}})},
        "tools/list"=>json!({"tools":[{"name":"hold","inputSchema":{"type":"object"}}]}),
        "tools/call"=>{state.entered.notify_one();state.release.notified().await;json!({"content":[{"type":"text","text":"finished original call"}]})},
        _=>json!({}),
    };
    axum::Json(json!({"jsonrpc":"2.0","id":request["id"],"result":result})).into_response()
}
#[tokio::test]
async fn reconnect_renews_through_actual_route_after_drain_and_uses_new_headers(){
    use rmcp::transport::auth::CredentialStore;
    let root=TempDir::new().unwrap();let mcp=Arc::new(DrainingMcp::default());let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();let endpoint=format!("http://{}/mcp",listener.local_addr().unwrap());
    let mock=axum::Router::new().route("/mcp",axum::routing::post(draining_mcp)).with_state(mcp.clone());let upstream=tokio::spawn(async move{axum::serve(listener,mock).await.unwrap()});
    let(manager,grants,http)=manager_fixture(root.path(),&endpoint);let store=sdk::Store::new(grants,"remote".into(),endpoint.clone(),OAuthFixture::metadata());store.save(fixture_credentials(false)).await.unwrap();
    manager.set_authentication_verifier(Arc::new(|transport|Box::pin(async move{let TransportConfig::Http{headers,..}=transport else{panic!("HTTP")};assert!(matches!(headers.get("Authorization").map(String::as_str),Some("Bearer old-fixture"|"Bearer new-fixture")));Ok(())})));
    let listener=std::net::TcpListener::bind("127.0.0.1:0").unwrap();listener.set_nonblocking(true).unwrap();let origin=format!("https://localhost:{}",listener.local_addr().unwrap().port());manager.set_origin(origin.clone());
    let rcgen::CertifiedKey{cert,signing_key}=rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();let tls=axum_server::tls_rustls::RustlsConfig::from_der(vec![cert.der().to_vec()],signing_key.serialize_der()).await.unwrap();
    let discovery=crate::discovery::DiscoveryService::new(root.path().join("empty-home"),None);let mut state=crate::api::AppState::new(origin.clone(),manager.repository.clone(),discovery,Arc::new(crate::authority::PendingNativeAuthority)).unwrap();state.manager=Some(manager.clone());
    let routes=crate::api::router(Arc::new(state));let handle=axum_server::Handle::new();let shutdown=handle.clone();let api=tokio::spawn(async move{axum_server::from_tcp_rustls(listener,tls).unwrap().handle(shutdown).serve(routes.into_make_service()).await.unwrap()});
    crate::application::configure_authenticate_sender(&manager,&origin,&cert.pem()).unwrap();
    assert_eq!(manager.authenticate_mode("remote",false).await.unwrap()["status"],"authenticated");let record=manager.record("remote").unwrap();manager.runner.configure(manager.route(&record).await.unwrap()).await.unwrap();manager.runner.connect("remote").await.unwrap();manager.connection_succeeded("remote",&record).await.unwrap();
    let runner=manager.runner.clone();let call=tokio::spawn(async move{runner.call("remote","hold",crate::runner::raw::raw(&json!({})),crate::runner::InvocationContext::terminal("fixture-owner".into())).await});tokio::time::timeout(Duration::from_secs(3),mcp.entered.notified()).await.unwrap();
    let restart=manager.restart("remote",json!({})).await.unwrap();let restart_id=restart["restart_id"].as_str().unwrap();
    assert_eq!(manager.authenticate_mode("remote",false).await.unwrap_err().code,"SERVER_BUSY");assert_eq!(manager.authenticate_background("remote").await.unwrap_err().code,"SERVER_BUSY");
    // The credential expires while a real raw MCP call still holds the drain.
    store.save(fixture_credentials(true)).await.unwrap();manager.set_authentication_expiry("remote",Some(sdk::now()-1)).await;
    assert_eq!(mcp.initializations.lock().unwrap().as_slice(),["Bearer old-fixture"]);mcp.release.notify_one();tokio::time::timeout(Duration::from_secs(3),call).await.unwrap().unwrap().unwrap();
    tokio::time::timeout(Duration::from_secs(6),async{loop{let status=manager.servers(vec!["remote".into()],Some(restart_id.into())).await.unwrap();match status["servers"][0]["restart"]["status"].as_str(){Some("succeeded")=>break,Some("failed")=>panic!("restart failed: {}",status["servers"][0]["restart"]),_=>tokio::time::sleep(Duration::from_millis(10)).await}}}).await.unwrap();
    assert_eq!(mcp.initializations.lock().unwrap().as_slice(),["Bearer old-fixture","Bearer new-fixture"]);assert_eq!(http.tokens.load(Ordering::SeqCst),1);
    manager.shutdown().await;handle.graceful_shutdown(Some(Duration::from_secs(1)));tokio::time::timeout(Duration::from_secs(3),api).await.unwrap().unwrap();upstream.abort();let _=upstream.await;
}
