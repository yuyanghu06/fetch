//! Client-held Fetch grants. No provider/client credential import or plaintext fallback.
use crate::remote_oauth::{Backend, NativeKeychain};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Grant {
    pub client_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
    pub scope: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub resource: Option<String>,
}

/// Private request ownership survives a coding client's short startup timeout.
/// The native Fetch app, not the connector, claims the eventual grant.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Pending {
    pub request_id: String,
    pub origin: String,
    pub profile: String,
    pub code: String,
    pub expires_at: u64,
    installation: String,
    claim_secret: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub resource: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Document {
    version: u8,
    profile: String,
    #[serde(default)]
    project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    grant: Option<Grant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pending: Option<Pending>,
}

#[derive(Clone)]
pub struct ProfileStore {
    root: PathBuf,
    profile: String,
    project_id: Option<String>,
    service: String,
    account: String,
    backend: Arc<dyn Backend>,
}

fn storage_error() -> String {
    "Fetch connector could not safely read or save this connection in macOS Keychain. Existing credentials were preserved.".into()
}
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl ProfileStore {
    pub fn new(root: PathBuf, client: String) -> Result<Self, String> {
        Self::with_backend(root, client, Arc::new(NativeKeychain))
    }
    pub fn for_context(root:PathBuf,context:super::ConnectionContext)->Result<Self,String> {
        Self::with_backend_for_context(root,context,Arc::new(NativeKeychain))
    }
    pub fn with_backend(
        root: PathBuf,
        client: String,
        backend: Arc<dyn Backend>,
    ) -> Result<Self, String> {
        Self::with_backend_for_context(root,super::ConnectionContext::global(client),backend)
    }
    pub fn with_backend_for_context(root:PathBuf,context:super::ConnectionContext,backend:Arc<dyn Backend>)->Result<Self,String> {
        context.validate()?;
        let client=context.client;
        if !root.is_absolute()
            || client.trim().is_empty()
            || client.len() > 128
            || client.chars().any(char::is_control)
        {
            return Err(
                "Choose an absolute Fetch data folder and a nonempty client connection name."
                    .into(),
            );
        }
        let root = std::fs::canonicalize(&root).unwrap_or(root);
        let namespace = format!("{:x}", Sha256::digest(root.as_os_str().as_encoded_bytes()));
        // Preserve legacy global keys; project profiles use an unambiguous tuple.
        let account = match &context.project_id {
            None=>format!("{:x}", Sha256::digest(client.as_bytes())),
            Some(project)=>format!("{:x}",Sha256::digest(serde_json::to_vec(&(client.as_str(),project.as_str())).map_err(|_|storage_error())?)),
        };
        Ok(Self {
            root,
            profile: client,
            project_id:context.project_id,
            service: format!("com.fetch.local.connector.{namespace}"),
            account,
            backend,
        })
    }
    pub async fn read(&self) -> Result<Option<Grant>, String> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || store.read_sync())
            .await
            .map_err(|_| storage_error())?
    }
    fn read_sync(&self) -> Result<Option<Grant>, String> {
        Ok(self.document_sync()?.grant.clone())
    }
    fn document_sync(&self) -> Result<Document, String> {
        let Some(bytes) = self
            .backend
            .read(&self.service, &self.account)
            .map_err(|_| storage_error())?
        else {
            return Ok(Document {
                version: 1,
                profile: self.profile.clone(),
                project_id:self.project_id.clone(),
                ..Default::default()
            });
        };
        let document: Document = serde_json::from_slice(&bytes).map_err(|_| storage_error())?;
        if document.version != 1 || document.profile != self.profile || document.project_id!=self.project_id {
            return Err(storage_error());
        }
        if let Some(grant) = &document.grant {
            self.validate_grant(grant,None)?;
            if grant.client_id.is_empty()
                || grant.access_token.is_empty()
                || grant.refresh_token.is_empty()
            {
                return Err(storage_error());
            }
        }
        if let Some(pending) = &document.pending {
            if pending.request_id.is_empty()
                || pending.claim_secret.is_empty()
                || pending.profile != self.profile
                || pending.installation != self.service
                || !valid_origin(&pending.origin)
                ||pending.project_id!=self.project_id
                ||pending.resource.as_deref().is_some_and(|resource|resource!=self.resource(&pending.origin))
                ||(self.project_id.is_some()&&pending.resource.as_deref()!=Some(self.resource(&pending.origin).as_str()))
            {
                return Err(storage_error());
            }
        }
        Ok(document)
    }
    fn write_sync(&self, grant: &Grant) -> Result<(), String> {
        self.validate_grant(grant,None)?;
        let mut document = self.document_sync()?;
        document.grant = Some(grant.clone());
        self.write_document(&document)
    }
    fn write_document(&self, document: &Document) -> Result<(), String> {
        let bytes = Zeroizing::new(serde_json::to_vec(document).map_err(|_| storage_error())?);
        self.backend
            .write(&self.service, &self.account, &bytes)
            .map_err(|_| storage_error())?;
        let saved = self
            .backend
            .read(&self.service, &self.account)
            .map_err(|_| storage_error())?
            .ok_or_else(storage_error)?;
        if saved.as_slice() != bytes.as_slice() {
            return Err(storage_error());
        }
        Ok(())
    }
    fn resource(&self,origin:&str)->String {
        super::ConnectionContext {client:self.profile.clone(),project_id:self.project_id.clone()}.resource(origin)
    }
    fn validate_grant(&self,grant:&Grant,origin:Option<&str>)->Result<(),String> {
        if grant.project_id!=self.project_id || (self.project_id.is_some()&&grant.resource.is_none())
            ||(self.project_id.is_none()&&grant.scope.split_ascii_whitespace().any(|scope|scope=="fetch:sessions:read")) {
            return Err("The returned Fetch grant belongs to a different project or resource. Existing saved access was preserved.".into());
        }
        if let Some(resource)=&grant.resource {
            let url=url::Url::parse(resource).map_err(|_|storage_error())?;
            let saved_origin=url.origin().ascii_serialization();
            if !valid_origin(&saved_origin)||resource!=&self.resource(&saved_origin)||origin.is_some_and(|origin|resource!=&self.resource(origin)) {
                return Err("The returned Fetch grant belongs to a different project or resource. Existing saved access was preserved.".into());
            }
        }
        Ok(())
    }
    pub async fn pending(&self, origin: &str) -> Result<Option<Pending>, String> {
        if !valid_origin(origin) {
            return Err("Fetch's connection approval origin is invalid.".into());
        }
        let store = self.clone();
        let origin = origin.to_owned();
        tokio::task::spawn_blocking(move || {
            let _lock = store.lock()?;
            let mut document = store.document_sync()?;
            let Some(pending) = document.pending.as_ref() else {
                return Ok(None);
            };
            if pending.origin != origin {
                return Err("This pending approval belongs to a different Fetch address.".into());
            }
            if pending.expires_at <= now() {
                document.pending = None;
                store.write_document(&document)?;
                return Ok(None);
            }
            Ok(Some(pending.clone()))
        })
        .await
        .map_err(|_| storage_error())?
    }
    /// Only creation and persistence hold this lock, never the human approval wait.
    pub async fn ensure_pending(
        &self,
        http: &reqwest::Client,
        origin: &str,
    ) -> Result<Option<Pending>, String> {
        if !valid_origin(origin) {
            return Err("Fetch's connection approval origin is invalid.".into());
        }
        let store = self.clone();
        let lock = tokio::task::spawn_blocking(move || store.lock())
            .await
            .map_err(|_| storage_error())??;
        let store = self.clone();
        let mut document = tokio::task::spawn_blocking(move || store.document_sync())
            .await
            .map_err(|_| storage_error())??;
        if document.grant.is_some() {
            return Ok(None);
        }
        if let Some(pending) = &document.pending {
            if pending.origin != origin {
                return Err("This pending approval belongs to a different Fetch address.".into());
            }
            if pending.expires_at > now() {
                return Ok(Some(pending.clone()));
            }
            document.pending = None;
            let store = self.clone();
            document = tokio::task::spawn_blocking(move || {
                store.write_document(&document)?;
                Ok::<_, String>(document)
            })
            .await
            .map_err(|_| storage_error())??;
        }
        let mut request=serde_json::json!({"client_name":self.profile});
        if let Some(project)=&self.project_id {
            request["project_id"]=serde_json::json!(project);
            request["scopes"]=serde_json::json!(["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:sessions:read"]);
        }
        let response = http
            .post(format!("{origin}/api/v1/connections/requests"))
            .json(&request)
            .send()
            .await
            .map_err(|_| {
                "Fetch became unavailable while requesting connection approval.".to_owned()
            })?;
        let status = response.status();
        let value: Value = response
            .json()
            .await
            .map_err(|_| "Fetch returned an unreadable connection request.".to_owned())?;
        if !status.is_success() {
            return Err(super::api_error(&value, status.as_u16()));
        }
        let expected_resource=self.resource(origin);
        if value.get("project_id").and_then(Value::as_str)!=self.project_id.as_deref()
            ||value.get("resource").and_then(Value::as_str).is_some_and(|resource|resource!=expected_resource)
            ||(self.project_id.is_some()&&value.get("resource").and_then(Value::as_str)!=Some(expected_resource.as_str())) {
            return Err("Fetch returned a connection request for a different project or resource.".into());
        }
        let text = |field: &str| {
            value[field]
                .as_str()
                .filter(|v| !v.is_empty())
                .map(str::to_owned)
                .ok_or_else(storage_error)
        };
        let pending = Pending {
            request_id: text("request_id")?,
            origin: origin.to_owned(),
            profile: self.profile.clone(),
            code: text("code")?,
            expires_at: value["expires_at"]
                .as_u64()
                .filter(|v| *v > now())
                .ok_or_else(storage_error)?,
            installation: self.service.clone(),
            claim_secret: text("claim_secret")?,
            project_id:self.project_id.clone(),
            resource:Some(expected_resource),
        };
        document.pending = Some(pending.clone());
        let store = self.clone();
        tokio::task::spawn_blocking(move || store.write_document(&document))
            .await
            .map_err(|_| storage_error())??;
        drop(lock);
        Ok(Some(pending))
    }
    /// App-side only: claim a matching approved request, then save before clearing proof.
    pub async fn complete_pending(
        &self,
        http: &reqwest::Client,
        origin: &str,
        request_id: &str,
    ) -> Result<bool, String> {
        let Some(pending) = self.pending(origin).await? else {
            return Ok(false);
        };
        if pending.request_id != request_id {
            return Ok(false);
        }
        let store = self.clone();
        let lock = tokio::task::spawn_blocking(move || store.lock())
            .await
            .map_err(|_| storage_error())??;
        let store = self.clone();
        let document = tokio::task::spawn_blocking(move || store.document_sync())
            .await
            .map_err(|_| storage_error())??;
        if document.pending.as_ref() != Some(&pending) {
            return Ok(false);
        }
        if pending.expires_at <= now() {
            drop(lock);
            self.clear_pending(origin, request_id).await?;
            return Ok(false);
        }
        let response=http.post(format!("{origin}/api/v1/connections/requests/{}/claim",crate::json::path_component(request_id))).json(&serde_json::json!({"claim_secret":pending.claim_secret})).send().await.map_err(|_|"Fetch became unavailable before saving connection approval. The pending request was preserved.".to_owned())?;
        let status = response.status();
        let value: Value = response.json().await.map_err(|_| {
            "Fetch returned an unreadable approval response. The pending request was preserved."
                .to_owned()
        })?;
        if status == reqwest::StatusCode::ACCEPTED {
            return Ok(false);
        }
        let store = self.clone();
        let expected = pending.clone();
        if status == reqwest::StatusCode::OK {
            let grant = Grant::from_response(&value, None)?;
            self.validate_grant(&grant,Some(origin))?;
            let stored = tokio::task::spawn_blocking(move || {
                let mut latest = store.document_sync()?;
                if latest.pending.as_ref() != Some(&expected) {
                    return Ok(false);
                }
                latest.grant = Some(grant);
                latest.pending = None;
                store.write_document(&latest)?;
                Ok(true)
            })
            .await
            .map_err(|_| storage_error())?;
            drop(lock);
            return stored;
        }
        if matches!(status.as_u16(), 403 | 404 | 410) {
            tokio::task::spawn_blocking(move || {
                let mut latest = store.document_sync()?;
                if latest.pending.as_ref() == Some(&expected) {
                    latest.pending = None;
                    store.write_document(&latest)?;
                }
                Ok::<(), String>(())
            })
            .await
            .map_err(|_| storage_error())??;
        }
        drop(lock);
        Err(super::api_error(&value, status.as_u16()))
    }
    pub async fn clear_pending(&self, origin: &str, request_id: &str) -> Result<bool, String> {
        let store = self.clone();
        let origin = origin.to_owned();
        let id = request_id.to_owned();
        tokio::task::spawn_blocking(move || {
            let _lock = store.lock()?;
            let mut document = store.document_sync()?;
            if !document
                .pending
                .as_ref()
                .is_some_and(|p| p.request_id == id && p.origin == origin)
            {
                return Ok(false);
            }
            document.pending = None;
            store.write_document(&document)?;
            Ok(true)
        })
        .await
        .map_err(|_| storage_error())?
    }
    pub async fn save_response(&self, response: &Value) -> Result<(), String> {
        let grant = Grant::from_response(response, None)?;
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let _lock = store.lock()?;
            let _ = store.read_sync()?;
            store.write_sync(&grant)
        })
        .await
        .map_err(|_| storage_error())?
    }
    /// Serialize rotating refresh across connector processes, then reread the winner.
    pub async fn access_token(
        &self,
        http: &reqwest::Client,
        origin: &str,
        rejected: Option<&str>,
    ) -> Result<String, String> {
        let store = self.clone();
        let lock = tokio::task::spawn_blocking(move || store.lock())
            .await
            .map_err(|_| storage_error())??;
        let mut grant = self.read().await?.ok_or_else(|| {
            format!(
                "This Fetch connection is not approved. Open Fetch → Connect apps and set up {}.",
                self.profile
            )
        })?;
        self.validate_grant(&grant,Some(origin))?;
        let must_refresh = grant.expires_at <= now().saturating_add(30)
            || rejected == Some(grant.access_token.as_str());
        if must_refresh {
            let form = url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs([
                    ("grant_type", "refresh_token"),
                    ("client_id", grant.client_id.as_str()),
                    ("refresh_token", grant.refresh_token.as_str()),
                    ("resource", self.resource(origin).as_str()),
                ])
                .finish();
            let response=http.post(format!("{origin}/oauth/token")).header("content-type","application/x-www-form-urlencoded").body(form).send().await.map_err(|_|"Fetch connector could not renew this connection. Fetch may be unavailable; no tool call was retried.".to_owned())?;
            let status = response.status();
            let bytes=response.bytes().await.map_err(|_|"Fetch connector lost the renewal response. Set up this connection again before retrying.".to_owned())?;
            if !status.is_success() {
                let details = crate::runner::startup_diagnostics::sanitize_text(
                    &String::from_utf8_lossy(&bytes),
                    &[grant.access_token.clone(), grant.refresh_token.clone()],
                );
                return Err(format!(
                    "Fetch connector renewal failed (HTTP {}): {}. Set up this connection again in Fetch.",
                    status.as_u16(),
                    details
                ));
            }
            let value:Value=serde_json::from_slice(&bytes).map_err(|_|"Fetch connector received an unreadable renewal response; set up this connection again.".to_owned())?;
            grant = Grant::from_response(&value, Some(&grant))?;
            self.validate_grant(&grant,Some(origin))?;
            let store = self.clone();
            let saved = grant.clone();
            tokio::task::spawn_blocking(move || store.write_sync(&saved))
                .await
                .map_err(|_| storage_error())??;
        }
        let token = grant.access_token.clone();
        drop(lock);
        Ok(token)
    }
    fn lock(&self) -> Result<File, String> {
        private_directory(&self.root)?;
        let directory = self.root.join("connector-locks");
        private_directory(&directory)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(directory.join(&self.account))
            .map_err(|_| storage_error())?;
        let metadata = file.metadata().map_err(|_| storage_error())?;
        if !metadata.is_file()
            || metadata.uid() != unsafe { libc::geteuid() }
            || metadata.nlink() != 1
            || metadata.permissions().mode() & 0o077 != 0
        {
            return Err(storage_error());
        }
        file.lock_exclusive().map_err(|_| storage_error())?;
        Ok(file)
    }
}

impl Grant {
    fn from_response(value: &Value, previous: Option<&Grant>) -> Result<Self, String> {
        let text = |name: &str| {
            value[name]
                .as_str()
                .filter(|v| !v.is_empty())
                .map(str::to_owned)
        };
        Ok(Self {
            client_id: text("client_id")
                .or_else(|| previous.map(|g| g.client_id.clone()))
                .ok_or_else(storage_error)?,
            access_token: text("access_token").ok_or_else(storage_error)?,
            refresh_token: text("refresh_token").ok_or_else(storage_error)?,
            expires_at: now()
                .saturating_add(value["expires_in"].as_u64().ok_or_else(storage_error)?),
            scope: text("scope")
                .or_else(|| previous.map(|g| g.scope.clone()))
                .unwrap_or_default(),
            project_id:match value.get("project_id") {
                Some(Value::Null)=>None,
                Some(Value::String(id))=>Some(id.clone()),
                Some(_)=>return Err(storage_error()),
                None=>previous.and_then(|grant|grant.project_id.clone()),
            },
            resource:text("resource").or_else(||previous.and_then(|grant|grant.resource.clone())),
        })
    }
}

pub async fn save_from_response(
    root: PathBuf,
    client: String,
    response: Value,
) -> Result<(), String> {
    save_from_response_for_context(root,super::ConnectionContext::global(client),response).await
}
pub async fn save_from_response_for_context(root:PathBuf,context:super::ConnectionContext,response:Value)->Result<(),String> {
    ProfileStore::for_context(root, context)?
        .save_response(&response)
        .await
}

/// The Fetch window uses this wrapper after native approval. The proof remains
/// in Keychain even if the original coding-client process has already exited.
pub async fn complete_pending(
    root: PathBuf,
    client: String,
    origin: String,
    request_id: String,
) -> Result<bool, String> {
    complete_pending_for_context(root,super::ConnectionContext::global(client),origin,request_id).await
}
pub async fn complete_pending_for_context(root:PathBuf,context:super::ConnectionContext,origin:String,request_id:String)->Result<bool,String> {
    let url = url::Url::parse(&origin)
        .map_err(|_| "Fetch's connection approval origin is invalid.".to_owned())?;
    if !valid_origin(&origin) {
        return Err("Fetch's connection approval origin is invalid.".into());
    }
    let options = super::Options {
        root: root.clone(),
        client: context.client.clone(),
        project_id:context.project_id.clone(),
        port: url
            .port()
            .ok_or("Fetch's connection approval origin requires a port.")?,
    };
    ProfileStore::for_context(root, context)?
        .complete_pending(&super::pinned_client(&options)?, &origin, &request_id)
        .await
}

fn valid_origin(origin: &str) -> bool {
    url::Url::parse(origin).is_ok_and(|url| {
        url.scheme() == "https"
            && url.host_str() == Some("localhost")
            && url.port().is_some_and(|p| p >= 1024)
            && url.path() == "/"
            && url.query().is_none()
            && url.fragment().is_none()
            && url.username().is_empty()
            && url.password().is_none()
            && origin == url.origin().ascii_serialization()
    })
}

fn private_directory(path: &Path) -> Result<(), String> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match std::fs::DirBuilder::new().mode(0o700).create(path) {
                Ok(()) => {}
                // Two first-time connectors can create the shared lock folder
                // simultaneously. Never change the winner's permissions; check
                // exactly the same private owner/no-symlink constraints below.
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(_) => return Err(storage_error()),
            }
            std::fs::symlink_metadata(path).map_err(|_| storage_error())?
        }
        Err(_) => return Err(storage_error()),
    };
    if metadata.is_dir()
        && !metadata.file_type().is_symlink()
        && metadata.uid() == unsafe { libc::geteuid() }
        && metadata.permissions().mode() & 0o077 == 0
    {
        Ok(())
    } else {
        Err(storage_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::remote_oauth::GrantError;
    use std::sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    #[derive(Default)]
    struct Memory(Mutex<std::collections::BTreeMap<(String, String), Vec<u8>>>);
    impl Backend for Memory {
        fn read(
            &self,
            service: &str,
            account: &str,
        ) -> Result<Option<Zeroizing<Vec<u8>>>, GrantError> {
            Ok(self
                .0
                .lock()
                .unwrap()
                .get(&(service.into(), account.into()))
                .cloned()
                .map(Zeroizing::new))
        }
        fn write(&self, service: &str, account: &str, value: &[u8]) -> Result<(), GrantError> {
            self.0
                .lock()
                .unwrap()
                .insert((service.into(), account.into()), value.to_vec());
            Ok(())
        }
    }
    fn approved(access: &str, refresh: &str, expires: u64) -> Value {
        serde_json::json!({"client_id":"test-client","access_token":access,"refresh_token":refresh,"expires_in":expires,"scope":"fetch:catalog"})
    }

    #[tokio::test]
    async fn project_profiles_keep_grants_separate_and_reject_a_different_binding() {
        let root=tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(),std::fs::Permissions::from_mode(0o700)).unwrap();
        let backend=Arc::new(Memory::default());
        let global=ProfileStore::with_backend(root.path().into(),"codex".into(),backend.clone()).unwrap();
        let first_context=super::super::ConnectionContext {client:"codex".into(),project_id:Some("project-11111111111111111111111111111111".into())};
        let second_context=super::super::ConnectionContext {client:"codex".into(),project_id:Some("project-22222222222222222222222222222222".into())};
        let first=ProfileStore::with_backend_for_context(root.path().into(),first_context.clone(),backend.clone()).unwrap();
        let second=ProfileStore::with_backend_for_context(root.path().into(),second_context.clone(),backend.clone()).unwrap();
        global.save_response(&approved("global-access","global-refresh",3600)).await.unwrap();
        let mut value=approved("first-access","first-refresh",3600);
        value["project_id"]=serde_json::json!(first_context.project_id);
        value["resource"]=serde_json::json!(first_context.resource("https://localhost:53672"));
        value["scope"]=serde_json::json!("fetch:sessions:read");
        first.save_response(&value).await.unwrap();
        let pending=Pending {request_id:"project-request".into(),origin:"https://localhost:53672".into(),profile:"codex".into(),code:"123456".into(),expires_at:now()+600,installation:first.service.clone(),claim_secret:"task-proof".into(),project_id:first_context.project_id.clone(),resource:Some(first_context.resource("https://localhost:53672"))};
        let mut document=first.document_sync().unwrap();document.pending=Some(pending.clone());first.write_document(&document).unwrap();
        assert!(second.read().await.unwrap().is_none());
        assert!(second.pending("https://localhost:53672").await.unwrap().is_none());
        assert_eq!(global.read().await.unwrap().unwrap().access_token,"global-access");
        value["project_id"]=serde_json::json!(second_context.project_id);
        value["resource"]=serde_json::json!(second_context.resource("https://localhost:53672"));
        assert!(first.save_response(&value).await.is_err());
        let reopened=ProfileStore::with_backend_for_context(root.path().into(),first_context,backend).unwrap();
        assert_eq!(reopened.read().await.unwrap().unwrap().access_token,"first-access");
        assert!(reopened.pending("https://localhost:53672").await.unwrap().as_ref()==Some(&pending));
    }

    #[tokio::test]
    async fn client_profiles_and_installations_have_separate_saved_grants() {
        let root = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::set_permissions(other.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let backend = Arc::new(Memory::default());
        let claude =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend.clone())
                .unwrap();
        let codex = ProfileStore::with_backend(root.path().into(), "codex".into(), backend.clone())
            .unwrap();
        let separate =
            ProfileStore::with_backend(other.path().into(), "claude-code".into(), backend).unwrap();
        claude
            .save_response(&approved("claude-access", "claude-refresh", 3600))
            .await
            .unwrap();
        assert!(codex.read().await.unwrap().is_none());
        assert!(separate.read().await.unwrap().is_none());
        codex
            .save_response(&approved("codex-access", "codex-refresh", 3600))
            .await
            .unwrap();
        assert_eq!(
            claude.read().await.unwrap().unwrap().access_token,
            "claude-access"
        );
        assert_eq!(
            codex.read().await.unwrap().unwrap().access_token,
            "codex-access"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_profile_refresh_rereads_the_rotated_winner() {
        use axum::{Json, Router, extract::State, routing::post};
        let root = tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let backend = Arc::new(Memory::default());
        let first =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend.clone())
                .unwrap();
        let second =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend).unwrap();
        first
            .save_response(&approved("old-access", "old-refresh", 0))
            .await
            .unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let app=Router::new().route("/oauth/token",post(|State(calls):State<Arc<AtomicUsize>>,body:String|async move{
            assert!(body.contains("refresh_token=old-refresh"));calls.fetch_add(1,Ordering::SeqCst);
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            Json(serde_json::json!({"access_token":"new-access","refresh_token":"new-refresh","expires_in":3600,"scope":"fetch:catalog"}))
        })).with_state(calls.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, app).await });
        let http = reqwest::Client::builder().no_proxy().build().unwrap();
        let (a, b) = tokio::join!(
            first.access_token(&http, &origin, None),
            second.access_token(&http, &origin, None)
        );
        assert_eq!(a.unwrap(), "new-access");
        assert_eq!(b.unwrap(), "new-access");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            first.read().await.unwrap().unwrap().refresh_token,
            "new-refresh"
        );
        server.abort();
        let _ = server.await;
    }

    #[tokio::test]
    async fn pending_proof_survives_store_reopen_and_only_matching_clear_preserves_grant() {
        let root = tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let backend = Arc::new(Memory::default());
        let store =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend.clone())
                .unwrap();
        store
            .save_response(&approved("saved-access", "saved-refresh", 3600))
            .await
            .unwrap();
        let pending = Pending {
            request_id: "request-one".into(),
            origin: "https://localhost:53672".into(),
            profile: "claude-code".into(),
            code: "123456".into(),
            expires_at: now() + 600,
            installation: store.service.clone(),
            claim_secret: "private-proof".into(),
            project_id:None,
            resource:None,
        };
        let mut document = store.document_sync().unwrap();
        document.pending = Some(pending.clone());
        store.write_document(&document).unwrap();
        drop(store);
        let reopened =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend).unwrap();
        assert!(reopened.pending(&pending.origin).await.unwrap().as_ref() == Some(&pending));
        assert!(
            !reopened
                .clear_pending(&pending.origin, "different-request")
                .await
                .unwrap()
        );
        assert!(reopened.pending(&pending.origin).await.unwrap().is_some());
        assert!(
            reopened
                .clear_pending(&pending.origin, &pending.request_id)
                .await
                .unwrap()
        );
        assert!(reopened.pending(&pending.origin).await.unwrap().is_none());
        assert_eq!(
            reopened.read().await.unwrap().unwrap().access_token,
            "saved-access"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn simultaneous_startups_create_one_durable_pending_request() {
        use axum::{Json, Router, extract::State, routing::post};
        let root = tempfile::tempdir().unwrap();
        std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        std::fs::write(root.path().join("certificate.pem"), cert.pem()).unwrap();
        let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(cert.pem().as_bytes()))
            .map(|v| v.unwrap().to_vec())
            .collect();
        let tls =
            axum_server::tls_rustls::RustlsConfig::from_der(certs, signing_key.serialize_der())
                .await
                .unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let router=Router::new().route("/api/v1/connections/requests",post(|State(calls):State<Arc<AtomicUsize>>,Json(body):Json<Value>|async move {
            assert_eq!(body["client_name"],"claude-code");calls.fetch_add(1,Ordering::SeqCst);
            (axum::http::StatusCode::ACCEPTED,Json(serde_json::json!({"status":"pending","client_name":"claude-code","request_id":"one-request","claim_secret":"one-secret","code":"123456","expires_at":now()+600,"scopes":["fetch:catalog","fetch:tools:call","fetch:content:read"],"next_steps":[]})))
        })).with_state(calls.clone());
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let handle = axum_server::Handle::<std::net::SocketAddr>::new();
        let server_handle = handle.clone();
        let server = tokio::spawn(async move {
            axum_server::from_tcp_rustls(listener, tls)
                .unwrap()
                .handle(server_handle)
                .serve(router.into_make_service())
                .await
        });
        handle.listening().await.unwrap();
        let options = crate::connector::Options {
            root: root.path().into(),
            client: "claude-code".into(),
            port,
            project_id:None,
        };
        let http = crate::connector::pinned_client(&options).unwrap();
        let backend = Arc::new(Memory::default());
        let first =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend.clone())
                .unwrap();
        let second =
            ProfileStore::with_backend(root.path().into(), "claude-code".into(), backend).unwrap();
        let origin = options.origin();
        let (a, b) = tokio::join!(
            first.ensure_pending(&http, &origin),
            second.ensure_pending(&http, &origin)
        );
        let a = a.unwrap().unwrap();
        let b = b.unwrap().unwrap();
        assert!(a == b);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(first.read().await.unwrap().is_none());
        assert!(second.pending(&origin).await.unwrap().as_ref() == Some(&a));
        handle.shutdown();
        tokio::time::timeout(std::time::Duration::from_secs(3), server)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
}
