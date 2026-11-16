//! Shared project installation and idle-only native scope assignment.
use super::{Caller, Live, ManagementError, Manager, busy, invalid};
use crate::{projects::{Project, ProjectError, ProjectRegistry, server_project_id}, runner::IdleStop};
use serde_json::{Value, json};
use std::{path::Path, sync::Arc};

impl From<ProjectError> for ManagementError {
    fn from(error:ProjectError)->Self {
        let mut failure=Self::new(error.http_status,error.code,"Fetch could not read or save the selected project.");
        failure.extra=json!({"error":{"code":error.code,"message":error.message}});
        failure
    }
}

impl Manager {
    pub fn projects(&self)->ProjectRegistry { ProjectRegistry::new(self.repository.root().to_path_buf()) }

    pub(super) async fn scope_lock(&self,id:&str)->Arc<tokio::sync::RwLock<()>> {
        self.scope_locks.lock().await.entry(id.to_owned()).or_insert_with(||Arc::new(tokio::sync::RwLock::new(()))).clone()
    }
    pub fn permitted_record(&self,caller:&Caller,permission:&str,record:&Value,item:Option<&str>)->bool {
        let Some(id)=record["server_id"].as_str() else{return false;};
        if !caller.permits(permission,id,item){return false;}
        // Legacy trusted test callers intentionally have no resource binding.
        // Every real OAuth grant now carries its exact global/project resource.
        caller.is_native_settings() || (caller.resource().is_none()&&caller.project_id().is_none())
            || server_project_id(record).is_none() || server_project_id(record)==caller.project_id()
    }
    pub(super) fn require_current_management(&self,caller:&Caller,record:&Value)->Result<(),ManagementError> {
        if self.permitted_record(caller,"fetch:manage",record,None){Ok(())}
        else{Err(ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found."))}
    }
    pub(super) async fn scope_access_for(&self,caller:&Caller,id:&str)->Result<tokio::sync::OwnedRwLockReadGuard<()>,ManagementError> {
        let scope=self.scope_lock(id).await.read_owned().await;
        {
            let gate=self.lock(id).await;let _guard=gate.lock().await;
            self.require_current_management(caller,&self.record(id)?)?;
        }
        Ok(scope)
    }

    pub async fn restart_for(self:&Arc<Self>,caller:&Caller,id:&str,body:Value)->Result<Value,ManagementError> {
        let manager=self.clone();let caller=caller.clone();let id=id.to_owned();
        tokio::spawn(async move{
            let _scope=manager.scope_access_for(&caller,&id).await?;
            manager.restart(&id,body).await
        }).await.map_err(|_|ManagementError::new(503,"RESTART_ACCEPTANCE_FAILED","The restart request could not finish."))?
    }

    pub async fn authenticate_for(self:&Arc<Self>,caller:&Caller,id:&str,interactive:bool)->Result<Value,ManagementError> {
        let manager=self.clone();let caller=caller.clone();let id=id.to_owned();
        tokio::spawn(async move{
            let _scope=manager.scope_access_for(&caller,&id).await?;
            manager.authenticate_mode(&id,interactive).await
        }).await.map_err(|_|ManagementError::new(503,"AUTHENTICATION_FAILED","The authentication request could not finish."))?
    }

    pub async fn install_for(self:&Arc<Self>,caller:&Caller,mut body:Value)->Result<Value,ManagementError> {
        if !caller.can_manage(){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        let object=body.as_object_mut().ok_or_else(invalid)?;
        let supplied=object.remove("project_id");
        let selected=match supplied.as_ref() {
            None=>if caller.is_native_settings(){None}else{caller.project_id()},
            Some(Value::Null)=>None,
            Some(Value::String(id))=>Some(id.as_str()),
            Some(_)=>return Err(invalid()),
        };
        if !caller.is_native_settings() && selected!=caller.project_id(){
            return Err(ManagementError::new(403,"FORBIDDEN","This connection cannot install servers for another project."));
        }
        let project=selected.map(|id|self.required_project(id)).transpose()?;
        if !caller.is_native_settings() {
            if let Some(project)=&project {
                for row in object.get("servers").and_then(Value::as_array).into_iter().flatten() {
                    if let Some(source)=row.get("config_path").and_then(Value::as_str) {
                        let source=Path::new(source).canonicalize().map_err(|_|ManagementError::new(400,"INVALID_REQUEST","The selected project configuration file could not be read."))?;
                        if !source.starts_with(&project.folder_path){return Err(ManagementError::new(403,"FORBIDDEN","This connection can import configuration only from its registered project folder."));}
                    }
                }
            }
        }
        self.install_project(body,project).await
    }

    pub async fn installation_for(&self,caller:&Caller,id:&str)->Result<Value,ManagementError> {
        if !caller.can_manage(){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        if caller.is_native_settings(){return self.installation(id).await;}
        Ok(self.installer.progress_scoped(id,caller.project_id()).await?)
    }

    pub(super) fn required_project(&self,id:&str)->Result<Project,ManagementError> {
        self.projects().get(id)?.ok_or_else(||ManagementError::new(404,"PROJECT_NOT_FOUND","The selected project was not found."))
    }

    pub(super) fn normalized_scope(&self,caller:&Caller,value:&Value)->Result<Value,ManagementError> {
        if !caller.is_native_settings(){return Err(ManagementError::new(403,"FORBIDDEN","Project assignment can be changed only in Fetch's native Settings window."));}
        let object=value.as_object().filter(|object|object.len()==1&&object.contains_key("project_id")).ok_or_else(invalid)?;
        match &object["project_id"] {
            Value::Null=>Ok(json!({"project_id":null})),
            Value::String(id)=>{self.required_project(id)?;Ok(json!({"project_id":id}))},
            _=>Err(invalid()),
        }
    }

    pub(super) async fn scope_admission(&self,id:&str,live:&Live)->Result<IdleStop,ManagementError> {
        if live.starting||live.authenticating||self.active_restart(id,live).await {
            return Err(busy("Finish startup, authentication or restart before changing this server's project."));
        }
        // Holding this existing guard blocks new admission and lifecycle changes;
        // dropping it reopens admission without stopping/restarting the server.
        self.runner.idle_for_stop(id).await.map_err(|error|if error.code=="SERVER_BUSY"{
            busy("This server has active, queued or uncertain work. Wait for it to finish before changing its project.")
        }else{error.into()})
    }
}
