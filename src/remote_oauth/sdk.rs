//! One official-library OAuth integration for every standards-compatible remote.
//! Fetch owns only Keychain/network connections and instance binding here.
use super::{GrantError,GrantStore};
use async_trait::async_trait;
use oauth2::TokenResponse;
use rmcp::transport::auth::{AuthError,AuthorizationManager,AuthorizationMetadata,AuthorizationRequest,AuthorizationSession,CredentialRefreshGuard,CredentialStore,OAuthHttpClient,OAuthHttpClientFuture,OAuthHttpRequest,StoredCredentials};
use serde_json::{Value,json};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::runner::startup_diagnostics::StartupDiagnostics;

const CLIENT_METADATA_URL:&str="https://www.yuyang-hu.org/oauth-client.json";

#[cfg(test)]
#[path="service_log_tests.rs"]
mod service_log_tests;

#[derive(Clone)]
pub(crate) struct Store {
    pub store:Arc<GrantStore>,pub id:String,pub endpoint:String,pub metadata:AuthorizationMetadata,
    refresh:Option<Arc<Mutex<()>>>,
}
impl Store {
    pub fn new(store:Arc<GrantStore>,id:String,endpoint:String,metadata:AuthorizationMetadata)->Self{let refresh=store.refresh_gate(&id).ok();Self{store,id,endpoint,metadata,refresh}}
    fn binding(&self)->Value{json!({"endpoint":self.endpoint,"metadata":self.metadata})}
    pub async fn snapshot(&self)->Result<Option<StoredCredentials>,AuthError>{self.load().await}
}
fn storage_error(_:GrantError)->AuthError{AuthError::CredentialStoreError("macOS Keychain could not safely read or save the selected server login".into())}
#[async_trait]
impl CredentialStore for Store {
    async fn load(&self)->Result<Option<StoredCredentials>,AuthError>{
        let document=self.store.read_json(&self.id).await.map_err(storage_error)?.unwrap_or_else(||json!({}));
        let Some(value)=document.get("sdk").filter(|value|!value.is_null()) else{return Ok(None)};
        if value["binding"]["endpoint"]!=self.endpoint{return Err(AuthError::CredentialStoreError("Saved login belongs to a different remote service endpoint; existing state was preserved".into()));}
        let credentials:StoredCredentials=serde_json::from_value(value["credentials"].clone()).map_err(|_|AuthError::CredentialStoreError("Saved OAuth data is unreadable".into()))?;
        if credentials.issuer!=self.metadata.issuer || credentials.client_id.is_empty() || value["client_id"]!=credentials.client_id{return Err(AuthError::AuthorizationRequired);}
        Ok(Some(credentials))
    }
    async fn save(&self,credentials:StoredCredentials)->Result<(),AuthError>{
        if credentials.issuer!=self.metadata.issuer || credentials.client_id.is_empty(){return Err(AuthError::AuthorizationRequired);}
        self.store.save_section(&self.id,"sdk",json!({"binding":self.binding(),"client_id":credentials.client_id,"credentials":credentials})).await.map_err(storage_error)
    }
    async fn clear(&self)->Result<(),AuthError>{
        // A library mismatch must not silently delete account state.
        Err(AuthError::CredentialStoreError("Existing account state was preserved; complete an explicit new sign-in".into()))
    }
    async fn acquire_refresh_guard(&self)->Result<Option<CredentialRefreshGuard>,AuthError>{Ok(Some(CredentialRefreshGuard::new(self.refresh.as_ref().ok_or_else(||AuthError::CredentialStoreError("OAuth refresh coordination is unavailable".into()))?.clone().lock_owned().await)))}

}

pub(crate) struct Prepared {pub manager:AuthorizationManager,pub store:Store,diagnostics:Option<Arc<StartupDiagnostics>>}
impl Prepared {
    pub async fn new(grants:Arc<GrantStore>,id:&str,endpoint:&str)->Result<Self,AuthError>{Self::with_http(grants,id,endpoint,Arc::new(Network)).await}
    pub async fn with_http(grants:Arc<GrantStore>,id:&str,endpoint:&str,http:Arc<dyn OAuthHttpClient>)->Result<Self,AuthError>{
        Self::prepare(grants,id,endpoint,http,None).await
    }
    pub async fn new_with_diagnostics(grants:Arc<GrantStore>,id:&str,endpoint:&str,diagnostics:Arc<StartupDiagnostics>)->Result<Self,AuthError>{
        Self::with_http_and_diagnostics(grants,id,endpoint,Arc::new(DiagnosticNetwork{diagnostics:diagnostics.clone()}),diagnostics).await
    }
    pub async fn with_http_and_diagnostics(grants:Arc<GrantStore>,id:&str,endpoint:&str,http:Arc<dyn OAuthHttpClient>,diagnostics:Arc<StartupDiagnostics>)->Result<Self,AuthError>{
        Self::prepare(grants,id,endpoint,http,Some(diagnostics)).await
    }
    async fn prepare(grants:Arc<GrantStore>,id:&str,endpoint:&str,http:Arc<dyn OAuthHttpClient>,diagnostics:Option<Arc<StartupDiagnostics>>)->Result<Self,AuthError>{
        let http:Arc<dyn OAuthHttpClient>=match &diagnostics {
            Some(capture)=>Arc::new(DiagnosticHttp{inner:http,diagnostics:capture.clone()}),
            None=>http,
        };
        let mut manager=AuthorizationManager::new_with_oauth_http_client(endpoint,http).await.map_err(|error|{if let Some(capture)=&diagnostics{log_auth_error(capture,"oauth discovery",&error);}error})?;
        let resolution=manager.resolve_metadata().await.map_err(|error|{if let Some(capture)=&diagnostics{log_auth_error(capture,"oauth discovery",&error);}error})?;
        let metadata=resolution.metadata;
        let store=Store::new(grants,id.to_owned(),endpoint.to_owned(),metadata.clone());
        manager.set_metadata(metadata);manager.set_credential_store(store.clone());
        Ok(Self{manager,store,diagnostics})
    }
    pub async fn begin(self,redirect:&str)->Result<AuthorizationSession,AuthError>{
        AuthorizationSession::new(self.manager,AuthorizationRequest::new(redirect).with_client_name("Fetch").with_client_metadata_url(CLIENT_METADATA_URL)).await.map_err(|(_,error)|{if let Some(capture)=&self.diagnostics{log_auth_error(capture,"oauth client registration",&error);}error})
    }
    pub async fn reuse(&mut self)->Result<String,AuthError>{
        if !self.manager.initialize_from_store().await.map_err(|error|{if let Some(capture)=&self.diagnostics{log_auth_error(capture,"oauth saved login",&error);}error})?{return Err(AuthError::AuthorizationRequired);}
        // This invocation is inside Fetch's authenticate route. Auto-refresh here
        // is allowed; the raw runner never calls the library's token accessor.
        self.manager.get_access_token().await.map_err(|error|{if let Some(capture)=&self.diagnostics{log_auth_error(capture,"oauth saved login",&error);}error})
    }
}

/// Selected error fields are sanitized by the same boundary as service output.
/// Never format whole SDK objects with Debug: they can contain token responses.
pub(crate) fn log_auth_error(capture:&StartupDiagnostics,stage:&str,error:&AuthError){
    let (kind,message)=match error{
        AuthError::AuthorizationRequired=>("AuthorizationRequired","No usable saved authorization is available."),
        AuthError::NoAuthorizationSupport=>("NoAuthorizationSupport","No compatible authorization setup was discovered."),
        AuthError::PkceUnsupported=>("PkceUnsupported","The service does not support the required S256 sign-in protection."),
        AuthError::TokenExpired=>("TokenExpired","The saved access token has expired."),
        AuthError::AuthorizationFailed(message)=>("AuthorizationFailed",message.as_str()),
        AuthError::RegistrationFailed(message)=>("RegistrationFailed",message.as_str()),
        AuthError::TokenExchangeFailed(message)=>("TokenExchangeFailed",message.as_str()),
        AuthError::TokenRefreshFailed(message)=>("TokenRefreshFailed",message.as_str()),
        AuthError::TokenRefreshRejected(message)=>("TokenRefreshRejected",message.as_str()),
        AuthError::CredentialStoreError(message)=>("CredentialStoreError",message.as_str()),
        AuthError::MetadataError(message)=>("MetadataError",message.as_str()),
        AuthError::OAuthError(message)=>("OAuthError",message.as_str()),
        AuthError::InternalError(message)=>("InternalError",message.as_str()),
        AuthError::InvalidTokenType(message)=>("InvalidTokenType",message.as_str()),
        AuthError::InvalidScope(message)=>("InvalidScope",message.as_str()),
        AuthError::ClientCredentialsError(message)=>("ClientCredentialsError",message.as_str()),
        AuthError::HttpError(_)=>("HttpError","The authorization HTTP request failed."),
        AuthError::UrlError(_)=>("UrlError","A returned authorization URL is invalid."),
        AuthError::InsufficientScope{..}=>("InsufficientScope","The service requires additional permissions."),
        AuthError::AuthorizationServerMismatch{..}=>("AuthorizationServerMismatch","The returned authorization issuer differs from the selected issuer."),
        AuthError::AuthorizationServerMissingIssuer{..}=>("AuthorizationServerMissingIssuer","The authorization callback omitted its required issuer."),
        _=>("AuthError","The authorization operation failed."),
    };
    capture.value(stage,&json!({"kind":kind,"message":message}));
}

struct DiagnosticHttp {inner:Arc<dyn OAuthHttpClient>,diagnostics:Arc<StartupDiagnostics>}
impl OAuthHttpClient for DiagnosticHttp {
    fn execute(&self,operation:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(async move{
        let mut stage="oauth discovery";
        let body=operation.request.body();
        let content_type=operation.request.headers().get("content-type").and_then(|value|value.to_str().ok()).unwrap_or("");
        let mut redactions=Vec::new();
        for name in ["authorization","cookie"]{
            if let Some(value)=operation.request.headers().get(name).and_then(|value|value.to_str().ok()){
                redactions.push(value.to_owned());
                if let Some(token)=value.strip_prefix("Bearer "){redactions.push(token.to_owned());}
            }
        }
        if content_type.starts_with("application/x-www-form-urlencoded"){
            stage="oauth token exchange";
            for (name,value) in url::form_urlencoded::parse(body){
                if name=="grant_type"&&value=="refresh_token"{stage="oauth token refresh";}
                if sensitive_request_field(&name)&&!value.is_empty(){redactions.push(value.into_owned());}
            }
        }else if operation.request.method()==oauth2::http::Method::POST{
            stage="oauth client registration";
            if let Ok(value)=serde_json::from_slice::<Value>(body){request_redactions(&value,&mut redactions);}
        }
        if let Some(query)=operation.request.uri().query(){for(name,value)in url::form_urlencoded::parse(query.as_bytes()){if sensitive_request_field(&name)&&!value.is_empty(){redactions.push(value.into_owned());}}}
        self.diagnostics.add_redactions(redactions);
        let response=self.inner.execute(operation).await.map_err(|error|{self.diagnostics.text(stage,&format!("Request failed: {error}"));error})?;
        self.diagnostics.text(stage,&format!("Service returned HTTP {}.",response.status().as_u16()));
        if !response.status().is_success(){
            if let Ok(value)=serde_json::from_slice::<Value>(response.body()){
                self.diagnostics.value(stage,&value);
            }else{
                self.diagnostics.text(stage,&String::from_utf8_lossy(response.body()));
            }
        }
        Ok(response)
    })}
}
fn sensitive_request_field(name:&str)->bool{
    let name=name.to_ascii_lowercase();
    name.contains("token")||name.contains("secret")||name.contains("password")||matches!(name.as_str(),"code"|"code_verifier"|"state"|"authorization"|"cookie"|"assertion"|"client_assertion")
}
fn request_redactions(value:&Value,redactions:&mut Vec<String>){
    match value{
        Value::Object(object)=>for(name,value)in object{if sensitive_request_field(name){if let Some(value)=value.as_str().filter(|value|!value.is_empty()){redactions.push(value.into());}}else{request_redactions(value,redactions);}},
        Value::Array(values)=>for value in values{request_redactions(value,redactions);},
        _=>{},
    }
}

pub(crate) fn expires_at(credentials:&StoredCredentials)->Option<u64>{credentials.token_response.as_ref()?.expires_in()?.as_secs().checked_add(credentials.token_received_at?)}
pub(crate) fn token(credentials:&StoredCredentials)->Option<String>{Some(credentials.token_response.as_ref()?.access_token().secret().to_owned())}
pub(crate) fn now()->u64{std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()}

pub(crate) struct Network;
impl OAuthHttpClient for Network {
    fn execute(&self,operation:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(network_request(operation,None))}
}
struct DiagnosticNetwork {diagnostics:Arc<StartupDiagnostics>}
impl OAuthHttpClient for DiagnosticNetwork {
    fn execute(&self,operation:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(network_request(operation,Some(&self.diagnostics)))}
}
async fn network_request(operation:OAuthHttpRequest,diagnostics:Option<&StartupDiagnostics>)->Result<oauth2::http::Response<Vec<u8>>,rmcp::transport::auth::OAuthHttpClientError>{
    let url=crate::manager::network::checked_url(&operation.request.uri().to_string()).map_err(|_|network_error())?;
    let client=crate::manager::network::client_for(&url).await.map_err(|_|network_error())?;
    // Refuse all redirects, including where the SDK would otherwise allow
    // discovery redirects. TLS verification and checked DNS remain enabled.
    let request=reqwest::Request::try_from(operation.request).map_err(|_|network_error())?;
    let response=client.execute(request).await.map_err(|_|network_error())?;
    collect_network_response(response,diagnostics).await
}
/// Keep the existing transport rejection rules, but preserve the returned status
/// before refusing a redirect or oversized response. No successful body is logged.
async fn collect_network_response(mut response:reqwest::Response,diagnostics:Option<&StartupDiagnostics>)->Result<oauth2::http::Response<Vec<u8>>,rmcp::transport::auth::OAuthHttpClientError>{
    let status=response.status();
    let reject=if status.is_redirection(){Some("OAuth HTTP redirect rejected; redirects are not followed.")}
        else if response.content_length().is_some_and(|length|length>1024*1024){Some("OAuth response rejected because Content-Length exceeds 1 MiB.")}else{None};
    if let Some(reason)=reject{
        if let Some(capture)=diagnostics{capture.value("oauth HTTP",&json!({"http_status":status.as_u16(),"error":reason}));}
        return Err(network_error());
    }
    let mut result=oauth2::http::Response::builder().status(status);
    for(name,value)in response.headers(){result=result.header(name,value);}
    let mut bytes=Vec::new();
    while let Some(chunk)=response.chunk().await.map_err(|_|network_error())?{
        if bytes.len()+chunk.len()>1024*1024{
            if let Some(capture)=diagnostics{capture.value("oauth HTTP",&json!({"http_status":status.as_u16(),"error":"OAuth response rejected because the body exceeds 1 MiB."}));}
            return Err(network_error());
        }
        bytes.extend_from_slice(&chunk);
    }
    result.body(bytes).map_err(|_|network_error())
}
fn network_error()->rmcp::transport::auth::OAuthHttpClientError{Box::new(std::io::Error::other("OAuth destination/request rejected or unavailable"))}
