//! Authentication dispatch and shared web OAuth requests.
//!
//! Saved launch/configuration facts choose a handler. Compatibility profiles add
//! only the provider requirements that differ from the common OAuth sequence.
use super::{ManagementError, Provider, auth_required, bindings, bounded_json, invalid, railway, remote_auth};
use crate::diagnostics::{UpstreamOperation, send};
use serde_json::{Value, json};
use url::Url;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Route { NotRequired, Web, Credentials, LocalHelper, Unsupported }

pub(super) fn dispatch(record: &Value, provider: Provider) -> Result<Route, ManagementError> {
    if provider == Provider::None { return Ok(Route::NotRequired); }
    if provider == Provider::Railway { return Ok(Route::Web); }
    let config = &record["config"];
    if config["kind"] == "remote" {
        return Ok(if provider == Provider::Manual { Route::Credentials } else { Route::Web });
    }
    // Compatibility selects a helper interface; the server ID owns its account.
    // Exact installed version/launch checks happen before any process is run.
    if config["package"]["name"] == "@a-bonus/google-docs-mcp"
        && config["executable"].is_string() && config["args"].is_array() {
        return Ok(Route::LocalHelper);
    }
    let inputs = bindings::InputBindings::from_record(record)?;
    Ok(if provider == Provider::Manual || !inputs.references().is_empty() {
        Route::Credentials
    } else { Route::Unsupported })
}

pub(super) struct WebPlan {
    pub provider: Provider,
    pub authorization: String,
    pub token: String,
    pub registration: String,
    pub resource: Option<String>,
    pub remote_metadata: Option<remote_auth::Metadata>,
}

impl WebPlan {
    pub async fn discover(record: &Value, provider: Provider, http: &reqwest::Client) -> Result<Self, ManagementError> {
        let (authorization, token, registration, resource, remote_metadata) = match provider {
            Provider::Railway => (railway::AUTHORIZE.to_owned(), railway::TOKEN.to_owned(), railway::REGISTER.to_owned(), None, None),
            Provider::Linear => {
                let endpoint = record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?;
                let endpoint_url = Url::parse(endpoint).map_err(|_| invalid())?;
                if endpoint_url.scheme() != "https" || endpoint_url.host_str() != Some("mcp.linear.app") { return Err(auth_required()); }
                let response = send(UpstreamOperation::ProviderMetadata, http.get("https://mcp.linear.app/.well-known/oauth-authorization-server"))
                    .await.map_err(|_| auth_required())?;
                let metadata = bounded_json(response).await?;
                let issuer = Url::parse(metadata["issuer"].as_str().ok_or_else(auth_required)?).map_err(|_| auth_required())?;
                if issuer.scheme() != "https" || issuer.host_str() != Some("mcp.linear.app")
                    || !issuer.username().is_empty() || issuer.password().is_some() { return Err(auth_required()); }
                if !metadata["code_challenge_methods_supported"].as_array().is_some_and(|values| values.iter().any(|v| v == "S256")) { return Err(auth_required()); }
                (checked_linear_endpoint(metadata["authorization_endpoint"].as_str().ok_or_else(auth_required)?)?,
                    checked_linear_endpoint(metadata["token_endpoint"].as_str().ok_or_else(auth_required)?)?,
                    checked_linear_endpoint(metadata["registration_endpoint"].as_str().ok_or_else(auth_required)?)?, Some(endpoint.to_owned()), None)
            },
            Provider::Remote => {
                let metadata = remote_auth::discover(record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?).await?;
                (metadata.authorization.clone(), metadata.token.clone(), metadata.registration.clone(), Some(metadata.resource.clone()), Some(metadata))
            },
            _ => return Err(auth_required()),
        };
        Ok(Self { provider, authorization, token, registration, resource, remote_metadata })
    }

    pub fn registration_body(&self, redirect: &str) -> Value {
        let mut body = json!({"client_name":"Fetch", "redirect_uris":[redirect],
            "token_endpoint_auth_method":"none", "grant_types":["authorization_code","refresh_token"], "response_types":["code"]});
        if self.provider == Provider::Railway { body["application_type"] = json!("native"); }
        body
    }

    pub async fn register(&self, http: &reqwest::Client, redirect: &str) -> Result<Value, ManagementError> {
        let http = request_client(http, &self.registration, self.remote_metadata.as_ref()).await?;
        let response = send(UpstreamOperation::ProviderRegistration, http.post(&self.registration).json(&self.registration_body(redirect)))
            .await.map_err(|_| auth_required())?;
        let registration = bounded_json(response).await?;
        // Keep the generic remote registration contract; the named adapters have
        // their established, separately checked public-client compatibility.
        if self.remote_metadata.is_some() && (registration.get("client_secret").is_some_and(|v| !v.is_null() && v != "")
            || registration.get("token_endpoint_auth_method").is_some_and(|v| v != "none")
            || registration.get("redirect_uris").is_some_and(|v| v.as_array().is_none_or(|values| !values.iter().any(|v| v == redirect)))) {
            return Err(auth_required());
        }
        Ok(registration)
    }

    pub fn validate_registration(&self, registration: &Value) -> Result<String, ManagementError> {
        let client = registration["client_id"].as_str().filter(|v| !v.is_empty() && v.len() <= 8192).ok_or_else(auth_required)?;
        if registration["token_endpoint_auth_method"].as_str().is_some_and(|method| method != "none")
            || (self.provider == Provider::Linear && registration.get("client_secret").is_some_and(|v| !v.is_null())) {
            return Err(auth_required());
        }
        Ok(client.to_owned())
    }

    pub fn authorization_url(&self, client: &str, redirect: &str, challenge: &str, state: &str) -> Result<Url, ManagementError> {
        let mut target = Url::parse(&self.authorization).map_err(|_| auth_required())?;
        target.query_pairs_mut().append_pair("response_type", "code").append_pair("client_id", client).append_pair("redirect_uri", redirect)
            .append_pair("code_challenge", challenge).append_pair("code_challenge_method", "S256").append_pair("state", state);
        if let Some(resource) = &self.resource { target.query_pairs_mut().append_pair("resource", resource); }
        if self.provider == Provider::Railway { target.query_pairs_mut().append_pair("scope", railway::SCOPES).append_pair("prompt", "consent"); }
        if let Some(scope) = self.remote_metadata.as_ref().and_then(|metadata| metadata.scope.as_deref()) { target.query_pairs_mut().append_pair("scope", scope); }
        Ok(target)
    }
}

pub(super) fn checked_linear_endpoint(value: &str) -> Result<String, ManagementError> {
    let endpoint = Url::parse(value).map_err(|_| auth_required())?;
    if endpoint.scheme() != "https" || !matches!(endpoint.host_str(), Some("mcp.linear.app" | "linear.app" | "api.linear.app"))
        || !endpoint.username().is_empty() || endpoint.password().is_some() || endpoint.fragment().is_some() { return Err(auth_required()); }
    Ok(value.to_owned())
}

async fn request_client(http: &reqwest::Client, endpoint: &str, metadata: Option<&remote_auth::Metadata>) -> Result<reqwest::Client, ManagementError> {
    if metadata.is_some() { remote_auth::client_for(&remote_auth::checked_url(endpoint)?).await } else { Ok(http.clone()) }
}

pub(super) async fn token(http: &reqwest::Client, endpoint: &str, metadata: Option<&remote_auth::Metadata>, fields: &[(&str, &str)], refresh: bool) -> Result<Value, ManagementError> {
    let http = request_client(http, endpoint, metadata).await?;
    let response = send(if refresh { UpstreamOperation::ProviderRefresh } else { UpstreamOperation::ProviderTokenExchange }, http.post(endpoint).form(fields))
        .await.map_err(|_| auth_required())?;
    bounded_json(response).await
}

/// Shared browser presentation for web OAuth and supervised helper URLs.
pub(super) async fn open_browser(target:&str)->Result<(),ManagementError>{
    let mut browser=tokio::process::Command::new("/usr/bin/open");
    browser.arg(target).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).kill_on_drop(true);
    match tokio::time::timeout(std::time::Duration::from_secs(5),browser.status()).await{
        Ok(Ok(status)) if status.success()=>Ok(()),
        _=>Err(ManagementError::new(409,"SERVER_AUTH_REQUIRED","The provider sign-in browser could not be opened. Retry Authenticate explicitly.")),
    }
}
