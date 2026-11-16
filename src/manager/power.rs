//! Persistent on/off preference and admission-safe Stop; provider behavior stays upstream.
use super::{Manager,ManagementError,busy};
use serde_json::{Value,json};
use std::sync::{Arc,atomic::Ordering};

pub(super) fn enabled(record:&Value)->bool{record["config"]["enabled"].as_bool().unwrap_or(true)}
pub(super) fn disabled()->ManagementError{ManagementError::new(409,"SERVER_DISABLED","This MCP server is switched off. Turn it on before using it.")}
impl Manager {
    pub(super) async fn require_enabled(&self,id:&str)->Result<(),ManagementError>{
        let gate=self.lock(id).await;let _guard=gate.lock().await;
        if enabled(&self.record(id)?){Ok(())}else{Err(disabled())}
    }
    pub(super) fn save_enabled(&self,id:&str,record:&mut Value,value:bool)->Result<bool,ManagementError>{
        if enabled(record)==value{return Ok(false);}
        record["config"]["enabled"]=json!(value);
        self.repository.save(id,record).map_err(|_|ManagementError::new(500,"CONFIG_WRITE_FAILED","Fetch could not save the server's on/off setting."))?;
        Ok(true)
    }
    pub async fn stop_server(self:&Arc<Self>,id:&str)->Result<Value,ManagementError>{
        let manager=self.clone();let id=id.to_owned();
        // Like deletion, an accepted worker retains its guards if its caller leaves.
        tokio::spawn(async move{manager.stop_server_inner(&id).await}).await.map_err(|_|stop_failed())?
    }
    async fn stop_server_inner(&self,id:&str)->Result<Value,ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(ManagementError::new(503,"SERVER_NOT_READY","Fetch is shutting down."));}
        self.record(id)?;
        let _installation=self.installer.idle_for_delete().await.map_err(|_|busy("Finish the active installation before switching a server off."))?;
        let gate=self.lock(id).await;let _guard=gate.try_lock().map_err(|_|busy("This server is changing state. Finish that operation before switching it off."))?;
        let mut record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if live.starting||live.authenticating||self.active_restart(id,&live).await{return Err(busy("Finish startup, authentication or restart before switching this server off."));}
        let mut stopped=self.runner.idle_for_stop(id).await.map_err(|error|if error.code=="SERVER_BUSY"{busy("This server has active, queued or uncertain work. Wait for it to finish before switching it off.")}else{stop_failed()})?;
        let changed=self.save_enabled(id,&mut record,false)?;
        match stopped.stop().await{
            Ok(closed)=>{
                let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.starting=false;state.runtime_error=None;state.oauth_restart_pending=false;state.restart=None;
                Ok(json!({"server_id":id,"status":"stopped","enabled":false,"changed":changed||closed,"runtime":{"status":"stopped","error":null},"next_steps":[]}))
            }
            Err(_)=>{let error=stop_failed();self.connection_failed(id,error.code,error.message).await;Err(error)}
        }
    }
}
fn stop_failed()->ManagementError{ManagementError::new(503,"SERVER_STOP_FAILED","The MCP server could not be confirmed stopped within five seconds. Its Off setting and saved data are preserved; review its status before retrying.")}
