use super::*;
use crate::runner::{Runner,TransportConfig};
use std::collections::BTreeMap;

struct DiagnosticFixture {issuer_supported:bool,registration_status:u16,token_status:u16}
impl DiagnosticFixture {
    fn metadata(&self)->Value {json!({"issuer":"https://issuer.fixture.example/","authorization_endpoint":"https://issuer.fixture.example/authorize",
        "token_endpoint":"https://issuer.fixture.example/token","registration_endpoint":"https://issuer.fixture.example/register",
        "authorization_response_iss_parameter_supported":self.issuer_supported,"code_challenge_methods_supported":["S256"]})}
}
impl OAuthHttpClient for DiagnosticFixture {
    fn execute(&self,operation:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(async move{
        let path=operation.request.uri().path();
        let (status,value)=if path.contains("oauth-protected-resource") {(200,json!({"resource":"https://mcp.fixture.example/mcp","authorization_servers":["https://issuer.fixture.example/"]}))}
        else if path.contains("oauth-authorization-server")||path.contains("openid-configuration") {(200,self.metadata())}
        else if path=="/register" {
            if self.registration_status==201 {let mut value:Value=serde_json::from_slice(operation.request.body()).unwrap();value["client_id"]=json!("fixture-client");(201,value)}
            else {(self.registration_status,json!({"error":"invalid_client_metadata","error_description":"the original registration rejection","client_secret":"registration-private-secret"}))}
        }else if path=="/token" {
            if self.token_status==200 {(200,json!({"access_token":"successful-private-access","refresh_token":"successful-private-refresh","token_type":"Bearer","expires_in":3600}))}
            else {(self.token_status,json!({"error":"invalid_grant","error_description":"the original token rejection","access_token":"error-private-access"}))}
        }else{(404,json!({"error":"fixture-not-found"}))};
        Ok(oauth2::http::Response::builder().status(status).header("content-type","application/json").body(value.to_string().into_bytes()).unwrap())
    })}
}
async fn capture()->(Arc<Runner>,Arc<StartupDiagnostics>){
    let runner=Runner::new();let capture=runner.begin_startup_diagnostics("fixture",&TransportConfig::Http{endpoint:"https://mcp.fixture.example/mcp".into(),headers:BTreeMap::new()}).await;(runner,capture)
}

#[tokio::test]
async fn unadvertised_issuer_support_reaches_sdk_login_without_custom_guard_log() {
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let(runner,capture)=capture().await;
    let prepared=Prepared::with_http_and_diagnostics(grants.clone(),"fixture","https://mcp.fixture.example/mcp",Arc::new(DiagnosticFixture{issuer_supported:false,registration_status:201,token_status:200}),capture.clone()).await.unwrap();
    let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
    let target=url::Url::parse(session.get_authorization_url()).unwrap();let state=target.query_pairs().find(|(name,_)|name=="state").unwrap().1.into_owned();
    capture.add_redactions([state.clone(),"callback-private-code".into()]);
    session.handle_callback_with_issuer("callback-private-code",&state,None).await.unwrap();
    let value=capture.snapshot().to_string();assert!(!value.contains("oauth issuer validation"));assert!(!value.contains("authorization_response_iss_parameter_supported"));
    assert!(value.contains("oauth client registration"));assert!(value.contains("oauth token exchange"));
    for secret in ["callback-private-code","successful-private-access","successful-private-refresh"]{assert!(!value.contains(secret),"leaked {secret}");}
    runner.shutdown().await;
}

#[tokio::test]
async fn registration_rejection_preserves_http_status_and_service_reason_without_returned_secret() {
    let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let(runner,capture)=capture().await;
    let prepared=Prepared::with_http_and_diagnostics(grants,"fixture","https://mcp.fixture.example/mcp",Arc::new(DiagnosticFixture{issuer_supported:true,registration_status:403,token_status:200}),capture.clone()).await.unwrap();
    assert!(matches!(prepared.begin("https://localhost:53672/oauth/provider/callback").await,Err(AuthError::RegistrationFailed(_))));
    let value=capture.snapshot().to_string();assert!(value.contains("oauth client registration"));assert!(value.contains("403"));assert!(value.contains("the original registration rejection"));
    assert!(!value.contains("registration-private-secret"));runner.shutdown().await;
}

#[tokio::test]
async fn rejected_production_redirect_retains_status_without_following_or_dumping_body() {
    let(runner,capture)=capture().await;
    let response=reqwest::Response::from(oauth2::http::Response::builder().status(302)
        .header("location","https://other.example/callback?code=private-redirect-code")
        .body(b"redirect-body-private-token".to_vec()).unwrap());
    assert!(collect_network_response(response,Some(&capture)).await.is_err());
    let value=capture.snapshot().to_string();assert!(value.contains("302"));
    assert!(!value.contains("private-redirect-code"));assert!(!value.contains("redirect-body-private-token"));
    runner.shutdown().await;
}

#[tokio::test]
async fn embedded_json_in_sdk_error_keeps_reason_without_echoing_secret_fields() {
    let(runner,capture)=capture().await;
    log_auth_error(&capture,"oauth token exchange",&AuthError::TokenExchangeFailed(
        "The service rejected token exchange: {\"reason\":\"embedded useful rejection\",\"access_token\":\"embedded-private-token\"}".into()));
    let value=capture.snapshot().to_string();assert!(value.contains("embedded useful rejection"));
    assert!(!value.contains("embedded-private-token"));runner.shutdown().await;
}

#[tokio::test]
async fn token_rejection_has_its_original_reason_and_successful_token_payload_is_never_logged() {
    for status in [400,200] {
        let temporary=tempfile::tempdir().unwrap();let grants=Arc::new(GrantStore::memory(temporary.path().into()));let(runner,capture)=capture().await;
        let prepared=Prepared::with_http_and_diagnostics(grants,"fixture","https://mcp.fixture.example/mcp",Arc::new(DiagnosticFixture{issuer_supported:true,registration_status:201,token_status:status}),capture.clone()).await.unwrap();
        let session=prepared.begin("https://localhost:53672/oauth/provider/callback").await.unwrap();
        let target=url::Url::parse(session.get_authorization_url()).unwrap();let state=target.query_pairs().find(|(name,_)|name=="state").unwrap().1.into_owned();
        capture.add_redactions([state.clone(),"callback-private-code".into()]);
        let result=session.handle_callback_with_issuer("callback-private-code",&state,Some("https://issuer.fixture.example/")).await;
        let value=capture.snapshot().to_string();
        if status==400 {assert!(result.is_err());assert!(value.contains("oauth token exchange"));assert!(value.contains("the original token rejection"));}else{assert!(result.is_ok());}
        for secret in ["callback-private-code","successful-private-access","successful-private-refresh","error-private-access"]{assert!(!value.contains(secret),"leaked {secret}");}
        runner.shutdown().await;
    }
}
