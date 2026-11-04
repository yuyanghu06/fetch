//! One complete remote sign-in through the temporary local callback, using fake provider/storage.
use super::*;
use futures_util::FutureExt;
use rmcp::transport::auth::{OAuthHttpClient,OAuthHttpClientFuture,OAuthHttpRequest};

const CLIENT_ID:&str="https://www.yuyang-hu.org/oauth-client.json";
const ISSUER:&str="https://issuer.fixture.example/";

#[derive(Default)]
struct PublicIdentityProvider {token_forms:std::sync::Mutex<Vec<String>>}
impl OAuthHttpClient for PublicIdentityProvider {
    fn execute(&self,request:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(async move{
        let path=request.request.uri().path();
        let(status,value)=if path.contains("oauth-protected-resource") {
            (200,json!({"resource":"https://mcp.fixture.example/mcp","authorization_servers":[ISSUER]}))
        }else if path.contains("oauth-authorization-server")||path.contains("openid-configuration") {
            (200,json!({"issuer":ISSUER,"authorization_endpoint":"https://issuer.fixture.example/authorize",
                "token_endpoint":"https://issuer.fixture.example/token","registration_endpoint":"https://issuer.fixture.example/register",
                "authorization_response_iss_parameter_supported":true,"client_id_metadata_document_supported":true,
                "code_challenge_methods_supported":["S256"]}))
        }else if path=="/token" {
            self.token_forms.lock().unwrap().push(String::from_utf8(request.request.body().clone()).unwrap());
            (200,json!({"access_token":"callback-access-fixture","refresh_token":"callback-refresh-fixture","token_type":"Bearer","expires_in":3600}))
        }else if path=="/register" {
            (400,json!({"error":"registration_is_not_available_use_public_client_identity"}))
        }else{(404,json!({"error":"fixture-not-found"}))};
        Ok(oauth2::http::Response::builder().status(status).header("content-type","application/json").body(value.to_string().into_bytes()).unwrap())
    })}
}

#[tokio::test]
async fn remote_login_uses_public_identity_and_temporary_loopback_callback() {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");
    let repository=crate::storage::ServerRepository::new(root.clone());
    repository.save("remote",&json!({"server_id":"remote","name":"remote callback fixture",
        "runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},
        "restart_required":false,"restart":null,"credentials":[],
        "config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://mcp.fixture.example/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
    let grants=Arc::new(GrantStore::memory(root.clone()));
    let discovery=crate::discovery::DiscoveryService::new(directory.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository,grants.clone(),Runner::new(),Installer::new(root,discovery));
    manager.set_origin("https://localhost:53672".into());
    let provider=Arc::new(PublicIdentityProvider::default());manager.set_oauth_http(provider.clone());
    manager.set_authentication_verifier(Arc::new(|transport|Box::pin(async move{
        let TransportConfig::Http{headers,..}=transport else{panic!("remote HTTP login")};
        assert_eq!(headers.get("Authorization").map(String::as_str),Some("Bearer callback-access-fixture"));Ok(())
    })));
    let opened=Arc::new(std::sync::Mutex::new(Vec::new()));let browser=opened.clone();
    manager.set_browser_opener(Arc::new(move|url|{browser.lock().unwrap().push(url);Box::pin(async{Ok(json!({}))})}));
    let result=std::panic::AssertUnwindSafe(async {
        assert_eq!(manager.authenticate("remote").await.unwrap()["status"],"authenticating");
        let browser_url=url::Url::parse(&opened.lock().unwrap()[0]).unwrap();
        let fields:BTreeMap<String,String>=browser_url.query_pairs().into_owned().collect();
        assert_eq!(fields["client_id"],CLIENT_ID);
        let mut callback=url::Url::parse(&fields["redirect_uri"]).unwrap();
        assert_eq!(callback.scheme(),"http");assert_eq!(callback.host_str(),Some("127.0.0.1"));
        assert_eq!(callback.path(),"/oauth/provider/callback");assert_ne!(callback.port().unwrap(),0);
        let address=format!("127.0.0.1:{}",callback.port().unwrap());
        callback.query_pairs_mut().append_pair("state",&fields["state"]).append_pair("code","callback-code-fixture").append_pair("iss",ISSUER);
        let client=reqwest::Client::builder().no_proxy().timeout(Duration::from_secs(5)).build().unwrap();
        let response=client.get(callback).send().await.unwrap();assert!(response.status().is_success());
        assert!(response.text().await.unwrap().contains("Provider setup finished"));
        assert_eq!(manager.servers(vec!["remote".into()],None).await.unwrap()["servers"][0]["authentication"]["status"],"authenticated");
        let saved=grants.read_json("remote").await.unwrap().unwrap();
        assert_eq!(saved["sdk"]["credentials"]["client_id"],CLIENT_ID);
        assert_eq!(saved["sdk"]["credentials"]["token_response"]["access_token"],"callback-access-fixture");
        let forms=provider.token_forms.lock().unwrap().clone();assert_eq!(forms.len(),1);
        let token_fields:BTreeMap<String,String>=url::form_urlencoded::parse(forms[0].as_bytes()).into_owned().collect();
        assert_eq!(token_fields["client_id"],CLIENT_ID);assert_eq!(token_fields["redirect_uri"],fields["redirect_uri"]);
        tokio::time::timeout(Duration::from_secs(2),async {
            loop {match tokio::net::TcpStream::connect(&address).await{Err(_)=>break,Ok(stream)=>drop(stream)}
                tokio::time::sleep(Duration::from_millis(10)).await;}
        }).await.expect("the completed sign-in must close its temporary listener");
    }).catch_unwind().await;
    manager.shutdown().await;
    if let Err(error)=result{std::panic::resume_unwind(error);}
}
