//! Native UI uses the same HTTPS application APIs as other clients.
use crate::native_auth::NativeSettingsSession;
use reqwest::{Client, Method};
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Clone)]
pub struct NativeClient {
    client: Client,
    origin: String,
    session: Arc<NativeSettingsSession>,
}
impl NativeClient {
    pub fn new(origin: String, certificate: &str, session: Arc<NativeSettingsSession>) -> Result<Self, String> {
        let certificate = reqwest::Certificate::from_pem(certificate.as_bytes()).map_err(|_| "Fetch's local certificate could not be read.")?;
        let client = Client::builder().https_only(true).redirect(reqwest::redirect::Policy::none())
            .tls_built_in_root_certs(false).add_root_certificate(certificate).timeout(std::time::Duration::from_secs(150))
            .build().map_err(|_| "Fetch's local API connection could not be prepared.")?;
        Ok(Self { client, origin, session })
    }
    pub async fn request(&self, method: Method, path: &str, body: Option<Value>) -> Result<Value, String> {
        if !path.starts_with("/api/v1/") || path.contains('#') { return Err("This operation is unavailable.".into()); }
        let mut request = self.client.request(method, format!("{}{}", self.origin, path))
            .header("X-Fetch-Settings-Capability", self.session.header_value());
        if let Some(body) = body { request = request.json(&body); }
        let response = crate::diagnostics::send(crate::diagnostics::UpstreamOperation::LocalApi, request).await.map_err(|_| "Fetch could not read its local API response. A submitted change may still be running; refresh its status before trying again.")?;
        let status = response.status();
        const LIMIT: usize = 32 * 1024 * 1024;
        if response.content_length().is_some_and(|size|size > LIMIT as u64) { return Err("The local API response is too large to display.".into()); }
        let mut response = response;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| "The local API response ended before it was complete.")? {
            if chunk.len() > LIMIT.saturating_sub(bytes.len()) { return Err("The local API response is too large to display.".into()); }
            bytes.extend_from_slice(&chunk);
        }
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| "The local API returned an unreadable response.")?;
        if !status.is_success() {
            if status == reqwest::StatusCode::CONFLICT && (
                (value["error"]["code"] == "INSTALLATION_IN_PROGRESS" && value["installation_id"].is_string()) ||
                (value["error"]["code"] == "RESTART_IN_PROGRESS" && value["restart_id"].is_string())
            ) { return Ok(value); }
            return Err(value["error"]["message"].as_str().unwrap_or("Fetch could not complete the operation.").to_owned());
        }
        Ok(value)
    }
    pub async fn servers(&self) -> Result<Value, String> { self.request(Method::GET, "/api/v1/servers", None).await }
    pub async fn discovery(&self, path: &str) -> Result<Value, String> {
        let suffix = if path.is_empty() { String::new() } else { format!("?{}", url::form_urlencoded::Serializer::new(String::new()).append_pair("config_path", path).finish()) };
        self.request(Method::GET, &format!("/api/v1/setup/discovery{suffix}"), None).await
    }
    pub async fn install(&self, selections: Vec<(String,String)>) -> Result<Value, String> {
        let servers: Vec<Value> = selections.into_iter().map(|(config_path,server_name)| json!({"config_path":config_path,"server_name":server_name})).collect();
        self.request(Method::POST, "/api/v1/setup/installations", Some(json!({"servers":servers}))).await
    }
}
pub fn component(value: &str) -> String { crate::json::path_component(value) }
