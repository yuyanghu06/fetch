//! Read-only project history shared by HTTP and MCP, with no model-chosen paths.
use super::{WorkspaceError, WorkspaceService};
use crate::{authority::Caller, projects::ProjectRegistry};
use serde_json::Value;

fn folder(registry:&ProjectRegistry,caller:&Caller)->Result<String,WorkspaceError>{
    if !caller.has_scope("fetch:sessions:read"){return Err(WorkspaceError::new(403,"FORBIDDEN","Project session-history permission is required."));}
    let id=caller.project_id().ok_or_else(||WorkspaceError::new(403,"FORBIDDEN","Session history requires a project-bound connection."))?;
    let project=registry.get(id).map_err(|error|WorkspaceError::new(error.http_status,error.code,error.message))?
        .ok_or_else(||WorkspaceError::new(404,"PROJECT_NOT_FOUND","The connected project was not found."))?;
    project.folder_path.into_os_string().into_string().map_err(|_|WorkspaceError::new(503,"PROJECT_FOLDER_UNAVAILABLE","The project folder cannot be represented as text."))
}
fn safe_metadata(value:&mut Value){
    if let Some(object)=value.as_object_mut(){for key in ["file_path","project_path","folder_path"]{object.remove(key);}}
}
pub fn list(service:&WorkspaceService,registry:&ProjectRegistry,caller:&Caller,app:Option<&str>,cursor:Option<&str>,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let folder=folder(registry,caller)?;
    if app.is_some_and(|app|!matches!(app,"claude"|"codex")){return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_APP","Choose claude or codex."));}
    let mut result=service.list_sessions(Some(&folder),app,cursor,limit)?;
    safe_metadata(&mut result);
    if let Some(rows)=result["sessions"].as_array_mut(){for row in rows{safe_metadata(row);}}
    Ok(result)
}
pub fn read(service:&WorkspaceService,registry:&ProjectRegistry,caller:&Caller,id:&str,offset:u64,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let folder=folder(registry,caller)?;
    let mut result=service.read_session_by_id(Some(&folder),id,offset,limit)?;
    safe_metadata(&mut result);safe_metadata(&mut result["session"]);
    Ok(result)
}
