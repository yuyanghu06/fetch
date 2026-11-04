//! Run the server's own local login without rebuilding its authentication.
use super::{Live, Manager, ManagementError, auth_response, busy, native_state, now, opaque, server_step, settings_step};
use serde_json::{Value, json};
use std::{sync::{Arc, atomic::Ordering}, time::Duration};
use tokio::sync::mpsc;

#[derive(Clone)]
pub(super) struct SignIn {
    // A pending OAuth URL is attempt-local. Never include it in GET or disk records.
    url: String,
    opened: bool,
}
impl SignIn {
    pub(super) fn public(&self) -> Value {
        let host=url::Url::parse(&self.url).ok().and_then(|url|url.host_str().map(str::to_owned));
        host.map(|host|json!({"host":host,"opened":self.opened})).unwrap_or(Value::Null)
    }
}

impl Manager {
    // The existing Authenticate caller owns this instance's gate until return.
    pub(super) async fn begin_local_authentication(self:&Arc<Self>,id:&str,record:Value,live:Live,interactive:bool)->Result<Value,ManagementError>{
        if !interactive {
            return Err(ManagementError::new(403,"FORBIDDEN","Background authentication cannot start a local login or open its browser."));
        }
        if live.authenticating {
            if let Some(sign_in)=live.native_sign_in.filter(|sign_in|!sign_in.opened) {
                // A second explicit click happens after the UI has displayed the host.
                self.present_authentication_browser(&sign_in.url).await?;
                let mut states=self.live.lock().await;
                if let Some(state)=states.get_mut(id).filter(|state|state.auth_attempt==live.auth_attempt&&state.authenticating) {
                    if let Some(current)=state.native_sign_in.as_mut().filter(|current|current.url==sign_in.url) { current.opened=true; }
                }
            }
            return Ok(auth_response(id,"authenticating",vec![server_step(id)]));
        }

        let descriptor=if record["_installation"].get("native_setup").is_some() {
            Some(self.native_setup_for(&record).await?)
        }else{None};
        let snapshot=self.runner.snapshot(id).await;
        if descriptor.is_none() && snapshot["runtime"]["status"]=="ready" {
            let error=json!({"code":"AUTHENTICATION_CHECK_UNSUPPORTED","message":"The server is already connected. It has not supplied a sign-in request, and connection alone does not prove account access. Follow its publisher's setup instructions if account login is still needed."});
            self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"unknown","error":error});
            self.remember_live_authentication(id).await?;
            return Ok(auth_response(id,"unknown",vec![settings_step("The server is connected. Use its publisher's documented setup if account login is still needed; Fetch will not guess a login command or restart a working connection.")]));
        }
        if ["active_calls","queued_calls","uncertain_calls"].iter().any(|name|snapshot["tool_execution"][*name].as_u64().unwrap_or(0)>0) {
            return Err(busy("This server has active, queued or uncertain work. Finish it before starting authentication."));
        }
        let manual=self.manual_for_record(&record).await?;
        let route=match self.route(&record).await {
            Ok(route)=>route,
            Err(error)=>{self.mark_failure(id,&error).await;return Err(error);}
        };
        let helper_transport=route.transport.clone();
        let registered=self.runner.server_ids().await.iter().any(|known|known==id);
        if !registered {self.runner.configure(route.clone()).await?;}
        // Atomic scheduler admission checks idleness and fences new calls for both
        // ordinary startup and a separately recorded helper, without draining work.
        self.runner.begin_authentication(id).await.map_err(|_|busy("This server has active, queued or uncertain work. Finish it before starting authentication."))?;
        if descriptor.is_none() && registered {
            // Settings may have supplied new process values since the previous
            // failed/stopped connection. Apply the freshly resolved route only
            // after the idle fence, never to an already-ready connection.
            if let Err(error)=self.runner.configure(route).await {
                self.runner.reopen_admission(id).await;
                return Err(error.into());
            }
        }
        let mut owned_tasks=self.local_tasks.lock().await;
        if self.stopping.load(Ordering::SeqCst){self.runner.reopen_admission(id).await;return Err(local_stopped());}
        let diagnostics=if descriptor.is_some(){Some(self.runner.begin_startup_diagnostics(id,&helper_transport).await)}else{None};
        let attempt=opaque("local-auth");
        {
            let mut states=self.live.lock().await;
            let state=states.entry(id.to_owned()).or_default();
            state.authenticating=true;state.auth_attempt=Some(attempt.clone());state.auth_deadline=now()+600;
            state.native_sign_in=None;state.runtime_error=None;
            state.auth=json!({"status":"authenticating","error":null});
        }
        let manager=self.clone();let server=id.to_owned();
        owned_tasks.retain(|task|!task.is_finished());
        owned_tasks.push(tokio::spawn(async move{manager.perform_local_authentication(server,attempt,record,manual,descriptor,diagnostics).await;}));
        drop(owned_tasks);
        Ok(auth_response(id,"authenticating",vec![settings_step("Fetch is starting the server's existing login process. If it supplies a sign-in page, review the host shown in Fetch and choose Open sign-in. No provider credentials are read or copied."),server_step(id)]))
    }

    async fn perform_local_authentication(self:Arc<Self>,id:String,attempt:String,record:Value,manual:Value,descriptor:Option<native_state::Setup>,diagnostics:Option<Arc<crate::runner::startup_diagnostics::StartupDiagnostics>>){
        let _diagnostic_guard=diagnostics.as_ref().map(|capture|super::DiagnosticGuard::new(capture.clone()));
        let helper=descriptor.is_some();
        let (urls,mut links)=mpsc::channel(8);
        let work=async {
            if self.stopping.load(Ordering::SeqCst) {return Err(local_stopped());}
            if let Some(setup)=descriptor {
                native_state::run_setup(self.repository.root(),&record,&setup,&manual,self.stop.clone(),urls,diagnostics.clone()).await
            }else{
                let quit=self.stop.notified();tokio::pin!(quit);quit.as_mut().enable();
                if self.stopping.load(Ordering::SeqCst) {return Err(local_stopped());}
                tokio::select! {
                    result=self.runner.connect_for_authentication(&id,urls)=>result.map_err(ManagementError::from),
                    _=tokio::time::sleep(Duration::from_secs(600))=>Err(ManagementError::new(409,"AUTHENTICATION_TIMEOUT","The local server login exceeded ten minutes. Choose Authenticate explicitly to try again.")),
                    _=&mut quit=>Err(local_stopped()),
                }
            }
        };
        tokio::pin!(work);
        let mut listening=true;
        let result=loop {
            tokio::select! {
                result=&mut work=>break result,
                value=links.recv(),if listening=>match value {
                    Some(url)=>self.observe_native_sign_in(&id,&attempt,url).await,
                    None=>listening=false,
                },
            }
        };
        if let (Some(capture),Err(error))=(&diagnostics,&result){if error.code!="NATIVE_SETUP_FAILED"{capture.text("setup",&format!("{}: {}",error.code,error.message));}}
        if !helper && result.is_err() {let _=self.runner.disconnect(&id,true).await;}
        let gate=self.lock(&id).await;let guard=gate.lock().await;
        let is_current=self.live.lock().await.get(&id).is_some_and(|state|state.auth_attempt.as_deref()==Some(&attempt));
        if !is_current {self.runner.reopen_admission(&id).await;return;}
        // Once status says setup finished, a ready server must already accept
        // calls. Release the idle-auth fence before publishing terminal status.
        self.runner.reopen_admission(&id).await;
        {
            let mut states=self.live.lock().await;let state=states.entry(id.clone()).or_default();
            state.authenticating=false;state.auth_attempt=None;state.native_sign_in=None;
            state.auth=match &result {
                Ok(())=>json!({"status":"unknown","error":{"code":"AUTHENTICATION_CHECK_UNSUPPORTED","message":"The server's setup finished. Fetch has not verified account access; use the server's tools to check the account."}}),
                Err(error)=>json!({"status":"failed","error":{"code":error.code,"message":error.message}}),
            };
            if result.is_ok() {
                if helper {state.oauth_restart_pending=state.applied.is_some();}
                else {state.applied=Some(record["config"].clone());state.oauth_restart_pending=false;state.runtime_error=None;}
            }else if !helper {
                state.runtime_error=result.as_ref().err().map(|error|json!({"code":error.code,"message":error.message}));
            }
        }
        if let Err(error)=self.remember_live_authentication(&id).await {self.status_save_failed(&id,&error).await;}
        drop(guard);
        if result.is_ok()&&!helper&&!self.stopping.load(Ordering::SeqCst){self.observe_features(&id).await;}
    }

    pub(super) async fn observe_native_sign_in(&self,id:&str,attempt:&str,url:String){
        if super::network::checked_url(&url).is_err(){return;}
        let gate=self.lock(id).await;let _guard=gate.lock().await;
        if self.stopping.load(Ordering::SeqCst){return;}
        let mut states=self.live.lock().await;
        if let Some(state)=states.get_mut(id).filter(|state|state.authenticating&&state.auth_attempt.as_deref()==Some(attempt)&&state.native_sign_in.is_none()){
            state.native_sign_in=Some(SignIn{url,opened:false});
        }
    }

    // Call under the per-instance manager gate. Only a non-secret terminal result
    // is saved; a pending flow or its browser URL can never enter config.json.
    pub(super) async fn remember_live_authentication(&self,id:&str)->Result<(),ManagementError>{
        let auth=self.live.lock().await.get(id).map(|state|state.auth.clone()).unwrap_or_else(||json!({"status":"unknown","error":null}));
        if auth["status"]=="authenticating" {return Ok(());}
        let mut record=self.record(id)?;
        record["authentication"]=json!({"status":auth["status"],"error":auth["error"]});
        let result=self.repository.save(id,&record).map_err(|_|ManagementError::new(500,"AUTHENTICATION_STATUS_SAVE_FAILED","Fetch could not save the last authentication result. The server's own credentials were not changed."));
        if let Err(error)=&result{self.status_save_failed(id,error).await;}
        result
    }

    pub(super) async fn status_save_failed(&self,id:&str,error:&ManagementError){
        let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();
        state.auth=json!({"status":"failed","error":{"code":error.code,"message":error.message}});
    }
}

fn local_stopped()->ManagementError{ManagementError::new(409,"AUTHENTICATION_STOPPED","Fetch stopped this local authentication attempt.")}
