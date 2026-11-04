//! One official-library OAuth integration for every standards-compatible remote.
//! Fetch owns only Keychain/network connections and instance binding here.
use super::{GrantError,GrantStore};
use async_trait::async_trait;
use oauth2::TokenResponse;
use rmcp::transport::auth::{AuthError,AuthorizationManager,AuthorizationMetadata,AuthorizationRequest,AuthorizationSession,CredentialRefreshGuard,CredentialStore,OAuthHttpClient,OAuthHttpClientFuture,OAuthHttpRequest,StoredCredentials};
use serde_json::{Value,json};
use std::sync::Arc;
use tokio::sync::Mutex;

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
        if value["binding"]["endpoint"]!=self.endpoint||value["binding"]["metadata"]!=serde_json::to_value(&self.metadata).map_err(|_|AuthError::AuthorizationRequired)?{return Err(AuthError::CredentialStoreError("Saved login belongs to different remote service metadata; existing state was preserved".into()));}
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

pub(crate) struct Prepared {pub manager:AuthorizationManager,pub store:Store}
impl Prepared {
    pub async fn new(grants:Arc<GrantStore>,id:&str,endpoint:&str)->Result<Self,AuthError>{Self::with_http(grants,id,endpoint,Arc::new(Network)).await}
    pub async fn with_http(grants:Arc<GrantStore>,id:&str,endpoint:&str,http:Arc<dyn OAuthHttpClient>)->Result<Self,AuthError>{
        let mut manager=AuthorizationManager::new_with_oauth_http_client(endpoint,http).await?;
        let resolution=manager.resolve_metadata().await?;
        if !resolution.source.is_discovered(){return Err(AuthError::NoAuthorizationSupport);}
        let metadata=resolution.metadata;
        // Preserve the existing exact-issuer callback protection, with no downgrade.
        if metadata.issuer.is_none() || metadata.additional_fields.get("authorization_response_iss_parameter_supported")!=Some(&json!(true)){return Err(AuthError::NoAuthorizationSupport);}
        for address in [Some(metadata.authorization_endpoint.as_str()),Some(metadata.token_endpoint.as_str()),metadata.registration_endpoint.as_deref(),metadata.issuer.as_deref()].into_iter().flatten(){crate::manager::network::checked_url(address).map_err(|_|AuthError::NoAuthorizationSupport)?;}
        let store=Store::new(grants,id.to_owned(),endpoint.to_owned(),metadata.clone());
        manager.set_metadata(metadata);manager.set_credential_store(store.clone());
        Ok(Self{manager,store})
    }
    pub async fn begin(self,redirect:&str)->Result<AuthorizationSession,AuthError>{
        AuthorizationSession::new(self.manager,AuthorizationRequest::new(redirect).with_client_name("Fetch")).await.map_err(|(_,error)|error)
    }
    pub async fn reuse(&mut self)->Result<String,AuthError>{
        if !self.manager.initialize_from_store().await?{return Err(AuthError::AuthorizationRequired);}
        // This invocation is inside Fetch's authenticate route. Auto-refresh here
        // is allowed; the raw runner never calls the library's token accessor.
        self.manager.get_access_token().await
    }
}
pub(crate) fn expires_at(credentials:&StoredCredentials)->Option<u64>{credentials.token_response.as_ref()?.expires_in()?.as_secs().checked_add(credentials.token_received_at?)}
pub(crate) fn token(credentials:&StoredCredentials)->Option<String>{Some(credentials.token_response.as_ref()?.access_token().secret().to_owned())}
pub(crate) fn now()->u64{std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()}

pub(crate) struct Network;
impl OAuthHttpClient for Network {
    fn execute(&self,operation:OAuthHttpRequest)->OAuthHttpClientFuture<'_>{Box::pin(async move{
        let url=crate::manager::network::checked_url(&operation.request.uri().to_string()).map_err(|_|network_error())?;
        let client=crate::manager::network::client_for(&url).await.map_err(|_|network_error())?;
        // Refuse all redirects, including where the SDK would otherwise allow
        // discovery redirects. TLS verification and checked DNS remain enabled.
        let request=reqwest::Request::try_from(operation.request).map_err(|_|network_error())?;
        let mut response=client.execute(request).await.map_err(|_|network_error())?;
        if response.status().is_redirection() || response.content_length().is_some_and(|length|length>1024*1024){return Err(network_error());}
        let mut result=oauth2::http::Response::builder().status(response.status());
        for(name,value)in response.headers(){result=result.header(name,value);}
        let mut bytes=Vec::new();while let Some(chunk)=response.chunk().await.map_err(|_|network_error())?{if bytes.len()+chunk.len()>1024*1024{return Err(network_error());}bytes.extend_from_slice(&chunk);}
        result.body(bytes).map_err(|_|network_error())
    })}
}
fn network_error()->rmcp::transport::auth::OAuthHttpClientError{Box::new(std::io::Error::other("OAuth destination/request rejected or unavailable"))}
