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
/// Local HTTP status metadata, kept outside the API JSON body.
#[derive(Clone)]
pub(super) struct TerminalOpened {
    pub(super) created: bool,
    pub(super) value: Value,
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
        self.request_with_status(method, path, body, false).await.map(|(_, value)| value)
    }
    pub(super) async fn save_new_workspace_file(&self, body: Value) -> Result<Value, String> {
        self.request_with_status(Method::PUT, "/api/v1/workspace/file", Some(body), true)
            .await
            .map(|(_, value)| value)
    }
    async fn request_with_status(&self, method: Method, path: &str, body: Option<Value>, create_only: bool) -> Result<(reqwest::StatusCode, Value), String> {
        if !path.starts_with("/api/v1/") || path.contains('#') { return Err("This operation is unavailable.".into()); }
        let mut request = self.client.request(method, format!("{}{}", self.origin, path))
            .header("X-Fetch-Settings-Capability", self.session.header_value());
        if create_only { request = request.header(reqwest::header::IF_NONE_MATCH, "*"); }
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
            ) { return Ok((status, value)); }
            return Err(value["error"]["message"].as_str().unwrap_or("Fetch could not complete the operation.").to_owned());
        }
        Ok((status, value))
    }
    pub(super) async fn terminal_open(&self, id: &str) -> Result<TerminalOpened, String> {
        let (status, value) = self.request_with_status(Method::POST,
            &format!("/api/v1/servers/{}/terminal", component(id)),
            Some(json!({"action":"open","columns":80,"rows":24})), false).await?;
        Ok(TerminalOpened { created: status == reqwest::StatusCode::CREATED, value })
    }
    pub async fn terminal_action(&self, id: &str, body: Value) -> Result<Value, String> {
        self.request(Method::POST, &format!("/api/v1/servers/{}/terminal", component(id)), Some(body)).await
    }
    pub async fn terminal_output(&self, id: &str, session: &str, after: u64) -> Result<Value, String> {
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("session_id", session).append_pair("after", &after.to_string()).finish();
        self.request(Method::GET, &format!("/api/v1/servers/{}/terminal?{query}", component(id)), None).await
    }
    pub async fn open_server_folder(&self, id: &str) -> Result<Value, String> {
        self.request(Method::POST, &format!("/api/v1/servers/{}/open-folder", component(id)), None).await
    }
    pub async fn projects(&self) -> Result<Value,String> { self.request(Method::GET,"/api/v1/projects",None).await }
    pub async fn register_project(&self, folder: &str) -> Result<Value,String> { self.request(Method::POST,"/api/v1/projects",Some(json!({"folder_path":folder}))).await }
    pub async fn servers(&self) -> Result<Value, String> { self.servers_for(None).await }
    pub async fn servers_for(&self, project: Option<&str>) -> Result<Value,String> {
        let suffix=project.map(|id|format!("?{}",url::form_urlencoded::Serializer::new(String::new()).append_pair("project_id",id).finish())).unwrap_or_default();
        self.request(Method::GET,&format!("/api/v1/servers{suffix}"),None).await
    }
    pub async fn discovery(&self,path:&str)->Result<Value,String>{self.discovery_for(path,None).await}
    pub async fn discovery_for(&self,path:&str,project:Option<&str>)->Result<Value,String>{
        let query={let mut query=url::form_urlencoded::Serializer::new(String::new());
            if !path.is_empty(){query.append_pair("config_path",path);}
            if let Some(id)=project{query.append_pair("project_id",id);}
            query.finish()};let suffix=if query.is_empty(){String::new()}else{format!("?{query}")};
        self.request(Method::GET,&format!("/api/v1/setup/discovery{suffix}"),None).await
    }
    pub async fn install(&self,selections:Vec<(String,String)>)->Result<Value,String>{self.install_for(selections,None).await}
    pub async fn install_for(&self,selections:Vec<(String,String)>,project:Option<&str>)->Result<Value,String>{
        let servers:Vec<Value>=selections.into_iter().map(|(config_path,server_name)|json!({"config_path":config_path,"server_name":server_name})).collect();
        self.request(Method::POST,"/api/v1/setup/installations",Some(json!({"servers":servers,"project_id":project}))).await
    }

}
pub fn component(value: &str) -> String { crate::json::path_component(value) }
