//! File-backed Workspace operations. The native API owns caller authorization.
mod agents;
mod files;
mod sessions;
mod conversations;
pub mod models;
pub mod project_sessions;

use serde_json::{Value, json};
use std::{fmt, path::{Path, PathBuf}};

pub const DEFAULT_CHUNK_BYTES: usize = 32 * 1024;
pub const MAX_CHUNK_BYTES: usize = 64 * 1024;
pub(crate) const MAX_FILE_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Debug)]
pub struct WorkspaceService {
    pub home: PathBuf,
    pub codex_root: PathBuf,
    pub claude_root: PathBuf,
    pub readers_root: PathBuf,
}

#[derive(Debug)]
pub struct WorkspaceError {
    pub http_status: u16,
    pub code: &'static str,
    pub message: String,
    pub details: Option<Value>,
}

impl WorkspaceError {
    pub(crate) fn new(http_status: u16, code: &'static str, message: impl Into<String>) -> Self {
        Self { http_status, code, message: message.into(), details: None }
    }
    pub fn body(&self) -> Value {
        let mut body=json!({"status":"failed", "error":{"code":self.code,"message":self.message}, "next_steps":[]});
        if let Some(details)=&self.details {body["error"]["details"]=details.clone();}
        body
    }
    pub(crate) fn io(error: std::io::Error) -> Self {
        let (status, code) = match error.kind() {
            std::io::ErrorKind::NotFound => (404, "WORKSPACE_FILE_NOT_FOUND"),
            std::io::ErrorKind::PermissionDenied => (403, "WORKSPACE_ACCESS_DENIED"),
            _ => (500, "WORKSPACE_IO_FAILED"),
        };
        Self::new(status, code, error.to_string())
    }
}
impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.message) }
}
impl std::error::Error for WorkspaceError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum App { Claude, Codex }
impl App {
    pub(crate) fn parse(app: &str) -> Result<Self, WorkspaceError> {
        match app {
            "claude" | "claude_code" => Ok(Self::Claude),
            "codex" => Ok(Self::Codex),
            _ => Err(WorkspaceError::new(400, "INVALID_WORKSPACE_APP", "Choose claude or codex.")),
        }
    }
    pub(crate) fn as_str(self) -> &'static str { match self { Self::Claude => "claude", Self::Codex => "codex" } }
}

impl WorkspaceService {
    /// Inject paths in fixtures. Construction does not read any files.
    pub fn new(home: PathBuf, codex_root: PathBuf, claude_root: PathBuf) -> Self {
        let readers_root=home.join("Library/Application Support").join(if cfg!(debug_assertions){"Fetch-Development"}else{"Fetch"}).join("session-readers");
        Self { home, codex_root, claude_root, readers_root }
    }
    /// The host passes its actual data folder; fixtures can use a disposable root.
    pub fn with_readers_root(mut self, root: PathBuf) -> Self { self.readers_root=root; self }
    pub async fn read_conversation(&self, folder_path:Option<&str>, file_path:&str)->Result<Value,WorkspaceError>{
        conversations::read(self,folder_path,file_path).await
    }
    pub async fn list_conversations(&self,folder_path:Option<&str>,app:Option<&str>,cursor:Option<&str>,limit:Option<usize>)->Result<Value,WorkspaceError>{
        conversations::list(self,folder_path,app,cursor,limit).await
    }
    pub fn from_environment() -> Result<Self, WorkspaceError> {
        let home = std::env::var_os("HOME").map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .ok_or_else(|| WorkspaceError::new(503,"WORKSPACE_HOME_UNAVAILABLE","The current user's home folder is unavailable."))?;
        let codex = std::env::var_os("CODEX_HOME").filter(|value| !value.is_empty()).map(PathBuf::from).unwrap_or_else(||home.join(".codex"));
        let claude = std::env::var_os("CLAUDE_CONFIG_DIR").filter(|value| !value.is_empty()).map(PathBuf::from).unwrap_or_else(||home.join(".claude"));
        if !codex.is_absolute() || !claude.is_absolute() {
            return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","Coding-app configuration folders must use absolute paths."));
        }
        Ok(Self::new(home,codex,claude))
    }
    pub(crate) fn folder(&self, folder: Option<&str>) -> Result<Option<PathBuf>, WorkspaceError> {
        folder.map(|folder| {
            let path = absolute_path(folder)?;
            let path = path.canonicalize().map_err(WorkspaceError::io)?;
            if !path.is_dir() { return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_FOLDER","Choose a project folder.")); }
            Ok(path)
        }).transpose()
    }
    pub fn discover(&self, folder_path: Option<&str>) -> Result<Value, WorkspaceError> { files::discover(self,folder_path) }
    pub fn read_file(&self, folder_path: Option<&str>, file_path: &str) -> Result<Value, WorkspaceError> { files::read(self,folder_path,file_path) }
    pub fn save_file(&self, folder_path: Option<&str>, file_path: &str, content: &str) -> Result<Value, WorkspaceError> { files::save(self,folder_path,file_path,content) }
    pub fn convert_agent(&self, source_app: &str, target_app: &str, content: &str) -> Result<Value, WorkspaceError> {
        agents::convert(App::parse(source_app)?,App::parse(target_app)?,content)
    }
    pub async fn reveal_file(&self,folder_path:Option<&str>,file_path:&str)->Result<Value,WorkspaceError>{
        files::reveal(self,folder_path,file_path).await
    }
    pub fn set_agent_model(&self, app: &str, content: &str, model: &str) -> Result<Value, WorkspaceError> {
        agents::set_model(App::parse(app)?,content,model)
    }
    pub fn list_sessions(&self, folder_path: Option<&str>, app: Option<&str>, cursor: Option<&str>, limit: Option<usize>) -> Result<Value, WorkspaceError> {
        sessions::list(self,folder_path,app,cursor,limit)
    }
    pub fn read_session(&self, folder_path: Option<&str>, file_path: &str, offset: u64, limit: Option<usize>) -> Result<Value, WorkspaceError> {
        sessions::read(self,folder_path,file_path,offset,limit)
    }
    pub fn read_session_by_id(&self, folder_path: Option<&str>, session_id: &str, offset: u64, limit: Option<usize>) -> Result<Value, WorkspaceError> {
        sessions::read_by_id(self,folder_path,session_id,offset,limit)
    }
}

pub(crate) fn agent_definition_preview(app:&str,content:&str)->Result<Value,WorkspaceError> {
    agents::preview(App::parse(app)?,content)
}

pub(crate) fn absolute_path(value: &str) -> Result<PathBuf, WorkspaceError> {
    let path = Path::new(value);
    if !path.is_absolute() || path.components().any(|part| matches!(part,std::path::Component::ParentDir)) {
        return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","Use an absolute path without parent-directory components."));
    }
    Ok(path.to_path_buf())
}

/// Resolve existing ancestors too, so a new file cannot escape through a link.
pub(crate) fn resolved_path(path: &Path) -> Result<PathBuf, WorkspaceError> {
    if path.exists() { return path.canonicalize().map_err(WorkspaceError::io); }
    let mut ancestor = path;
    let mut missing = Vec::new();
    while !ancestor.exists() {
        missing.push(ancestor.file_name().ok_or_else(||WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","This path has no existing parent."))?.to_owned());
        ancestor = ancestor.parent().ok_or_else(||WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","This path has no existing parent."))?;
    }
    let mut resolved = ancestor.canonicalize().map_err(WorkspaceError::io)?;
    for name in missing.into_iter().rev() { resolved.push(name); }
    Ok(resolved)
}

pub(crate) fn success(mut fields: Value) -> Value {
    fields["status"] = json!("ok"); fields["next_steps"] = json!([]); fields
}
