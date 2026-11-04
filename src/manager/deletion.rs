//! Remove one idle Fetch-owned installation without touching imported sources.
use super::{Manager,ManagementError,busy};
use crate::authority::Caller;
use serde_json::{Value,json};
use std::sync::Arc;

impl Manager {
    pub async fn delete_server(self:&Arc<Self>,id:&str,caller:&Caller)->Result<Value,ManagementError>{
        let manager=self.clone();let id=id.to_owned();let caller=caller.clone();
        // Once accepted by this worker, cleanup and its admission/installation
        // guards outlive a disconnected HTTP/MCP caller. No work is replayed.
        tokio::spawn(async move{manager.delete_server_inner(&id,&caller).await}).await.map_err(|_|delete_error())?
    }
    async fn delete_server_inner(&self,id:&str,caller:&Caller)->Result<Value,ManagementError>{
        if !caller.can_manage(){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        if id.trim().is_empty()||id.contains('\0'){return Err(ManagementError::new(400,"INVALID_REQUEST","Choose one installed server ID."));}
        if !caller.permits("fetch:manage",id,None){return Err(ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found."));}
        self.record(id)?;
        // Serialize against submission as well as accepted preparation work.
        let _installation=self.installer.idle_for_delete().await.map_err(|_|busy("Finish the active installation before deleting a server."))?;
        let gate=self.lock(id).await;
        let _guard=gate.try_lock().map_err(|_|busy("This server is changing state. Finish that operation before deleting it."))?;
        self.record(id)?;
        if id.starts_with("fetch-internal:"){return Err(delete_error());}
        let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if live.authenticating||self.active_restart(id,&live).await{return Err(busy("Finish authentication or restart before deleting this server."));}
        let mut deletion=self.runner.idle_for_delete(id).await.map_err(|error|{
            if error.code=="SERVER_BUSY"{busy("This server has active, queued or uncertain work. Wait for it to finish before deleting.")}
            else{ManagementError::new(503,"SERVER_DELETE_UNAVAILABLE","Fetch cannot safely close this server right now.")}
        })?;
        deletion.stop().await.map_err(|_|ManagementError::new(503,"SERVER_STOP_FAILED","The idle upstream could not be confirmed closed within five seconds. Saved server data was preserved; review its state before retrying deletion."))?;
        let repository=self.repository.clone();let server_id=id.to_owned();
        // Remove Fetch's registration only. Native files, packages, legacy keys
        // and grants are deliberately preserved even after successful removal.
        let cleaned=tokio::task::spawn_blocking(move||repository.remove_registration(&server_id)).await;
        if !matches!(cleaned,Ok(Ok(()))){return Err(delete_error());}
        deletion.finish().await;
        self.live.lock().await.remove(id);
        self.attempts.lock().await.retain(|(server,_),_|server!=id);
        self.flows.lock().await.retain(|_,flow|flow.server!=id);
        // Keep the per-server lock entry: stale waiters must share this same gate.
        Ok(json!({"server_id":id,"status":"deleted","next_steps":[]}))
    }
}
fn delete_error()->ManagementError{
    let mut error=ManagementError::new(500,"SERVER_DELETE_FAILED","The server was not completely removed. Cleanup may be partial; review its saved status before retrying. The saved association is retained when possible.");
    error.next_steps=vec![json!({"actor":"user","action":"review_server_deletion","instructions":"Read this server's saved status and resolve registration storage access before explicitly retrying DELETE. Do not reinstall or repeat provider actions to repair cleanup."})];
    error
}
