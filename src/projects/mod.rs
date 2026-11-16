//! Explicit project folders and shared/project server membership.
use crate::storage::{Directory, ServerRepository, StorageError};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{path::{Path, PathBuf}, sync::Mutex};

const MAX_PROJECT_BYTES: usize = 64 * 1024;
static REGISTRATION: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub folder_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct ProjectRegistry { root: PathBuf }

#[derive(Debug)]
pub struct ProjectError {
    pub http_status: u16,
    pub code: &'static str,
    pub message: String,
}
impl ProjectError {
    pub fn new(http_status: u16, code: &'static str, message: impl Into<String>) -> Self {
        Self { http_status, code, message:message.into() }
    }
    pub fn body(&self) -> Value {
        json!({"status":"failed","error":{"code":self.code,"message":self.message},"next_steps":[]})
    }
    fn storage(error:StorageError) -> Self {
        Self::new(503,"PROJECT_STORAGE_UNAVAILABLE",error.to_string())
    }
}
impl std::fmt::Display for ProjectError {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str(&self.message) }
}
impl std::error::Error for ProjectError {}

pub fn valid_project_id(id:&str)->bool {
    id.strip_prefix("project-").is_some_and(|suffix| suffix.len()==32 && suffix.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)))
}
pub fn server_project_id(record:&Value)->Option<&str> { record["config"]["scope"]["project_id"].as_str() }
pub fn visible_server_ids(repository:&ServerRepository,project_id:Option<&str>)->Result<Vec<String>,ProjectError> {
    let records=repository.list_saved().map_err(ProjectError::storage)?;
    Ok(records.iter().filter(|record|server_project_id(record).is_none()||server_project_id(record)==project_id)
        .filter_map(|record|record["server_id"].as_str().map(str::to_owned)).collect())
}

impl ProjectRegistry {
    pub fn new(root:PathBuf)->Self { Self { root } }
    fn directory(&self,create:bool)->Result<Option<Directory>,ProjectError> {
        if !create && !self.root.exists() {return Ok(None);}
        let root=Directory::open(&self.root,create).map_err(ProjectError::storage)?;
        root.child("projects",create).map_err(ProjectError::storage)
    }
    pub fn list(&self)->Result<Vec<Project>,ProjectError> {
        let _guard=REGISTRATION.lock().map_err(|_|ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE","The project registry is unavailable."))?;
        self.list_unlocked()
    }
    fn list_unlocked(&self)->Result<Vec<Project>,ProjectError> {
        let Some(directory)=self.directory(false)? else {return Ok(vec![]);};
        let mut projects=Vec::new();
        for name in directory.names().map_err(ProjectError::storage)? {
            let Some(id)=name.strip_suffix(".json").filter(|id|valid_project_id(id)) else {
                return Err(ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE","A saved project record has an invalid name."));
            };
            if let Some(project)=read_project(&directory,id)? {projects.push(project);}
        }
        projects.sort_by(|left,right|left.name.cmp(&right.name).then(left.id.cmp(&right.id)));
        Ok(projects)
    }
    pub fn get(&self,id:&str)->Result<Option<Project>,ProjectError> {
        if !valid_project_id(id) {return Err(ProjectError::new(400,"INVALID_PROJECT_ID","Use a registered Fetch project ID."));}
        let Some(directory)=self.directory(false)? else{return Ok(None);};
        read_project(&directory,id)
    }
    pub fn register(&self,folder_path:&str,name:Option<&str>)->Result<(Project,bool),ProjectError> {
        let path=Path::new(folder_path);
        if !path.is_absolute() || path.components().any(|part|matches!(part,std::path::Component::ParentDir)) {
            return Err(ProjectError::new(400,"INVALID_PROJECT_FOLDER","Choose an absolute existing project folder."));
        }
        let folder_path=path.canonicalize().map_err(|error|ProjectError::new(400,"INVALID_PROJECT_FOLDER",error.to_string()))?;
        if !folder_path.is_dir() {return Err(ProjectError::new(400,"INVALID_PROJECT_FOLDER","Choose a project folder, rather than a file."));}
        if name.is_some_and(|name|name.trim().is_empty()||name.chars().any(char::is_control)) {
            return Err(ProjectError::new(400,"INVALID_PROJECT_NAME","Use a nonempty project name without control characters."));
        }
        let _guard=REGISTRATION.lock().map_err(|_|ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE","The project registry is unavailable."))?;
        if let Some(project)=self.list_unlocked()?.into_iter().find(|project|project.folder_path==folder_path) {return Ok((project,false));}
        let name=name.map(str::trim).map(str::to_owned).unwrap_or_else(||folder_path.file_name().unwrap_or_else(||folder_path.as_os_str()).to_string_lossy().into_owned());
        let project=Project {id:format!("project-{:032x}",rand::random::<u128>()),name,folder_path};
        let directory=self.directory(true)?.ok_or_else(||ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE","The project registry could not be opened."))?;
        let bytes=serde_json::to_vec(&project).map_err(|error|ProjectError::new(500,"PROJECT_WRITE_FAILED",error.to_string()))?;
        directory.write(&format!("{}.json",project.id),&bytes).map_err(ProjectError::storage)?;
        Ok((project,true))
    }
}

fn read_project(directory:&Directory,id:&str)->Result<Option<Project>,ProjectError> {
    let Some(bytes)=directory.read(&format!("{id}.json"),MAX_PROJECT_BYTES).map_err(ProjectError::storage)? else{return Ok(None);};
    let project:Project=serde_json::from_slice(&bytes).map_err(|error|ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE",error.to_string()))?;
    if project.id!=id || !valid_project_id(&project.id) || project.name.trim().is_empty() || !project.folder_path.is_absolute()
        || project.folder_path.components().any(|part|matches!(part,std::path::Component::ParentDir)) {
        return Err(ProjectError::new(503,"PROJECT_STORAGE_UNAVAILABLE","The saved project record is invalid."));
    }
    Ok(Some(project))
}
