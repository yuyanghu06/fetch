//! Shared server operations. Native servers own their configuration and account state.
mod authentication;
pub(crate) mod network;
mod bindings;
mod deletion;
mod native_state;
mod process_environment;
mod local_auth;
mod remote_callback;
#[cfg(test)] mod process_environment_tests;
#[cfg(test)] mod local_auth_tests;
#[cfg(test)] mod native_storage_tests;
#[cfg(test)] mod service_log_tests;
#[cfg(test)] mod remote_callback_tests;
use crate::{authority::Caller, credentials::CredentialStore, remote_oauth::{GrantStore,sdk}, installation::{Installer,opaque}, runner::{Runner,ServerRoute,TransportConfig,ExecutionPolicy}, native_auth::{NativeAuthority,SettingsCapability}, storage::ServerRepository};
use base64::{Engine,engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},future::Future,pin::Pin,sync::{Arc,RwLock,atomic::{AtomicBool,Ordering}},time::Duration};
use tokio::sync::{Mutex,Notify};
use crate::runner::startup_diagnostics::StartupDiagnostics;
#[derive(Clone,Debug)]
pub struct ManagementError { pub http_status:u16, pub code:&'static str, pub message:&'static str, pub extra:Value, pub next_steps:Vec<Value> }
impl ManagementError {
    pub fn new(status:u16,code:&'static str,message:&'static str)->Self {Self{http_status:status,code,message,extra:json!({}),next_steps:vec![]}}
    pub fn body(&self)->Value {let mut value=json!({"status":"failed","error":{"code":self.code,"message":self.message},"next_steps":self.next_steps});if let Some(extra)=self.extra.as_object(){for(k,v)in extra{value[k]=v.clone();}}value}
}
impl From<crate::installation::InstallError> for ManagementError {fn from(e:crate::installation::InstallError)->Self{let mut error=Self::new(e.http_status,e.code,e.message);if let Some(id)=e.installation_id{error.extra=json!({"installation_id":id});error.next_steps=vec![json!({"actor":"agent","action":"check_installation_status","instructions":format!("After one second GET /api/v1/setup/installations/{id}.")})];}error}}
impl From<crate::runner::RunError> for ManagementError {fn from(e:crate::runner::RunError)->Self{Self::new(e.http_status,e.code,e.message)}}
#[derive(Clone)]
struct Live { access_expires_at:Option<u64>,last_verified_at:Option<u64>,auth:Value, runtime_error:Option<Value>, applied:Option<Value>, oauth_restart_pending:bool, revision:u64, authenticating:bool, auth_deadline:u64, auth_attempt:Option<String>, restart:Option<String>, native_sign_in:Option<local_auth::SignIn> }
impl Default for Live {fn default()->Self{Self{access_expires_at:None,last_verified_at:None,auth:json!({"status":"unknown","error":null}),runtime_error:None,applied:None,oauth_restart_pending:false,revision:0,authenticating:false,auth_deadline:0,auth_attempt:None,restart:None,native_sign_in:None}}}
struct CallbackGuard{manager:std::sync::Weak<Manager>,server:String,attempt:String,armed:bool}
impl Drop for CallbackGuard{fn drop(&mut self){if !self.armed{return;}let manager=self.manager.clone();let server=self.server.clone();let attempt=self.attempt.clone();if let Ok(runtime)=tokio::runtime::Handle::try_current(){runtime.spawn(async move{if let Some(manager)=manager.upgrade(){manager.end_auth_failure(&server,&attempt,"AUTHENTICATION_FAILED","The sign-in callback was interrupted. Use Authenticate explicitly for a fresh login.").await;}});}}}
struct ProviderFlow {server:String,attempt:String,session:rmcp::transport::auth::AuthorizationSession,store:sdk::Store,revision:u64,expires:u64,diagnostics:Arc<StartupDiagnostics>,callback_close:remote_callback::Close}
impl Drop for ProviderFlow{fn drop(&mut self){self.callback_close.close();self.diagnostics.freeze();}}
struct DiagnosticGuard {capture:Arc<StartupDiagnostics>,armed:bool}
impl DiagnosticGuard {fn new(capture:Arc<StartupDiagnostics>)->Self{Self{capture,armed:true}}}
impl Drop for DiagnosticGuard{fn drop(&mut self){if self.armed{self.capture.freeze();}}}
pub struct Manager {
    pub repository:ServerRepository,pub runner:Arc<Runner>,pub installer:Arc<Installer>, grants:Arc<GrantStore>,
    terminals:Arc<crate::terminal::TerminalService>,
    #[cfg(test)] browser_opener:RwLock<Option<AuthenticateSender>>,
    #[cfg(test)] oauth_http:RwLock<Option<Arc<dyn rmcp::transport::auth::OAuthHttpClient>>>,
    #[cfg(test)] authentication_verifier:RwLock<Option<AuthenticationVerifier>>,
    locks:Mutex<BTreeMap<String,Arc<Mutex<()>>>>,live:Arc<Mutex<BTreeMap<String,Live>>>,attempts:Mutex<BTreeMap<(String,String),Value>>,flows:Mutex<BTreeMap<String,ProviderFlow>>,origin:RwLock<String>,sender:RwLock<Option<AuthenticateSender>>,background_token:String,scheduled:Mutex<BTreeSet<String>>,restart_auth:Mutex<BTreeMap<String,String>>,stopping:AtomicBool,stop:Arc<Notify>,local_tasks:Mutex<Vec<tokio::task::JoinHandle<()>>>,
}
#[cfg(test)]
pub(crate) type AuthenticationVerifier=Arc<dyn Fn(TransportConfig)->Pin<Box<dyn Future<Output=Result<(),ManagementError>>+Send>>+Send+Sync>;
pub type AuthenticateSender=Arc<dyn Fn(String)->Pin<Box<dyn Future<Output=Result<Value,ManagementError>>+Send>>+Send+Sync>;
impl Manager {
    pub fn new(repository:ServerRepository,_application_store:Arc<CredentialStore>,runner:Arc<Runner>,installer:Arc<Installer>)->Arc<Self>{let grants=Arc::new(GrantStore::new(repository.root().to_owned()));Self::with_grant_store(repository,grants,runner,installer)}
    pub fn with_grant_store(repository:ServerRepository,grants:Arc<GrantStore>,runner:Arc<Runner>,installer:Arc<Installer>)->Arc<Self>{Arc::new(Self{repository,grants,runner,installer,terminals:crate::terminal::TerminalService::new(),#[cfg(test)]browser_opener:RwLock::new(None),#[cfg(test)]oauth_http:RwLock::new(None),#[cfg(test)]authentication_verifier:RwLock::new(None),locks:Mutex::new(BTreeMap::new()),live:Arc::new(Mutex::new(BTreeMap::new())),attempts:Mutex::new(BTreeMap::new()),flows:Mutex::new(BTreeMap::new()),origin:RwLock::new(String::new()),sender:RwLock::new(None),background_token:URL_SAFE_NO_PAD.encode(rand::random::<[u8;32]>()),scheduled:Mutex::new(BTreeSet::new()),restart_auth:Mutex::new(BTreeMap::new()),stopping:AtomicBool::new(false),stop:Arc::new(Notify::new()),local_tasks:Mutex::new(vec![])})}
    pub fn set_origin(&self,origin:String){if let Ok(mut value)=self.origin.write(){*value=origin;}}
    pub fn set_authenticate_sender(&self,sender:AuthenticateSender){if let Ok(mut value)=self.sender.write(){*value=Some(sender);}}
    pub(crate) fn background_token(&self)->String{self.background_token.clone()}
    pub(crate) fn accepts_background_token(&self,token:&str)->bool{!self.stopping.load(Ordering::SeqCst)&&token.len()==self.background_token.len()&&token.bytes().zip(self.background_token.bytes()).fold(0u8,|acc,(left,right)|acc|(left^right))==0}
    async fn request_background_authenticate(&self,id:&str)->Result<Value,ManagementError>{let sender=self.sender.read().map_err(|_|auth_required())?.clone().ok_or_else(auth_required)?;sender(id.to_owned()).await}
    pub async fn before_remote_use(&self,id:&str)->Result<(),ManagementError>{
        let record=self.record(id)?;if record["config"]["kind"]!="remote"{return Ok(());}
        let due=self.live.lock().await.get(id).and_then(|value|value.access_expires_at).is_some_and(|expiry|expiry<=now()+29);
        if due{self.request_background_authenticate(id).await?;}
        Ok(())
    }
    #[cfg(test)] fn set_browser_opener(&self,opener:AuthenticateSender){*self.browser_opener.write().unwrap()=Some(opener);}
    async fn present_authentication_browser(&self,target:&str)->Result<(),ManagementError>{
        #[cfg(test)]{let opener=self.browser_opener.read().unwrap().clone();if let Some(opener)=opener{return opener(target.to_owned()).await.map(|_|());}}
        network::validate_browser_destination(target).await?;authentication::open_browser(target).await
    }
    #[cfg(test)] pub(crate) fn set_oauth_http(&self,http:Arc<dyn rmcp::transport::auth::OAuthHttpClient>){*self.oauth_http.write().unwrap()=Some(http);}
    #[cfg(test)] pub(crate) fn set_authentication_verifier(&self,verifier:AuthenticationVerifier){*self.authentication_verifier.write().unwrap()=Some(verifier);}
    #[cfg(test)] pub(crate) async fn set_authentication_expiry(&self,id:&str,expiry:Option<u64>){self.live.lock().await.entry(id.to_owned()).or_default().access_expires_at=expiry;}
    async fn prepare_oauth(&self,id:&str,endpoint:&str,diagnostics:Arc<StartupDiagnostics>)->Result<sdk::Prepared,rmcp::transport::auth::AuthError>{
        #[cfg(test)] {let http=self.oauth_http.read().unwrap().clone();if let Some(http)=http{return sdk::Prepared::with_http_and_diagnostics(self.grants.clone(),id,endpoint,http,diagnostics).await;}}
        sdk::Prepared::new_with_diagnostics(self.grants.clone(),id,endpoint,diagnostics).await
    }
    async fn verify_remote(&self,transport:&TransportConfig,diagnostics:Arc<StartupDiagnostics>)->Result<(),ManagementError>{
        if let TransportConfig::Http{headers,..}|TransportConfig::Sse{headers,..}=transport{
            let mut values:Vec<String>=headers.values().cloned().collect();
            values.extend(headers.values().filter_map(|value|value.strip_prefix("Bearer ").map(str::to_owned)));
            diagnostics.add_redactions(values);
        }
        let result=self.verify_remote_inner(transport,diagnostics.clone()).await;
        if let Err(error)=&result{if error.code!="TOOL_RPC_ERROR"{diagnostics.text("protected access check",&format!("{}: {}",error.code,error.message));}}
        result
    }
    async fn verify_remote_inner(&self,transport:&TransportConfig,diagnostics:Arc<StartupDiagnostics>)->Result<(),ManagementError>{
        #[cfg(test)] {let verifier=self.authentication_verifier.read().unwrap().clone();if let Some(verifier)=verifier{return verifier(transport.clone()).await;}}
        self.runner.verify_remote_transport_with_diagnostics(transport,diagnostics).await.map_err(ManagementError::from)
    }
    async fn lock(&self,id:&str)->Arc<Mutex<()>>{self.locks.lock().await.entry(id.to_owned()).or_insert_with(||Arc::new(Mutex::new(()))).clone()}
    fn record(&self,id:&str)->Result<Value,ManagementError>{self.repository.get_saved(id).map_err(|_|ManagementError::new(500,"SERVER_INFO_UNAVAILABLE","Saved server information is unavailable."))?.ok_or_else(||ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found."))}
    pub async fn install(self:&Arc<Self>,body:Value)->Result<Value,ManagementError>{
        let weak=Arc::downgrade(self);
        let hook:crate::installation::PreparedHook=Arc::new(move|id|{
            let weak=weak.clone();Box::pin(async move{if let Some(manager)=weak.upgrade(){manager.connect_installed(id).await;}})
        });
        Ok(self.installer.submit_with_connect(body,hook).await?)
    }
    async fn connect_installed(self:Arc<Self>,id:String){
        if self.stopping.load(Ordering::SeqCst){return;}
        let gate=self.lock(&id).await;let guard=gate.lock().await;
        if self.stopping.load(Ordering::SeqCst)||self.record(&id).is_err(){return;}
        // Reused ready imports must not interrupt existing calls or repeat login.
        if self.runner.snapshot(&id).await["runtime"]["status"]=="ready"{return;}
        let live=self.live.lock().await.get(&id).cloned().unwrap_or_default();
        if live.authenticating{return;}
        let active=if self.active_restart(&id,&live).await{live.restart.clone()}else{None};
        drop(guard);
        let attempt=if let Some(attempt)=active{Some(attempt)}else{
            match self.restart(&id,json!({})).await{
                Ok(value)=>value["restart_id"].as_str().map(str::to_owned),
                Err(error) if error.code=="RESTART_IN_PROGRESS"=>error.extra["restart_id"].as_str().map(str::to_owned),
                Err(error) if error.code=="SERVER_BUSY"=>None,
                Err(error)=>{self.connection_failed(&id,error.code,error.message).await;None},
            }
        };
        if let Some(attempt)=attempt{loop{
            if self.stopping.load(Ordering::SeqCst){break;}
            let status=self.attempts.lock().await.get(&(id.clone(),attempt.clone())).map(|value|value["status"].clone());
            if status.as_ref().is_none_or(|status|matches!(status.as_str(),Some("succeeded"|"failed"))){break;}
            tokio::select!{_=tokio::time::sleep(Duration::from_millis(100))=>{},_=self.stop.notified()=>break}
        }}
    }
    async fn connection_failed(&self,id:&str,code:&str,message:&str){
        let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();
        state.runtime_error=Some(json!({"code":code,"message":message}));
        if !self.stopping.load(Ordering::SeqCst)&&matches!(code,"SERVER_AUTH_REQUIRED"|"PROVIDER_AUTH_REQUIRED"){
            state.auth=json!({"status":"required","error":{"code":code,"message":message}});
        }
    }
    async fn connection_succeeded(&self,id:&str,record:&Value)->Result<(),ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(ManagementError::new(503,"SERVER_NOT_READY","Fetch is shutting down."));}
        let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();
        state.applied=Some(record["config"].clone());state.oauth_restart_pending=false;state.runtime_error=None;
        let local=!state.authenticating && record["config"]["kind"]!="remote";
        if local {state.auth=json!({"status":"unknown","error":null});}
        drop(states);
        if local {self.remember_live_authentication(id).await?;}
        Ok(())
    }
    pub async fn installation(&self,id:&str)->Result<Value,ManagementError>{Ok(self.installer.progress(id).await?)}
    pub async fn servers_for(&self,caller:&Caller,ids:Vec<String>,restart:Option<String>)->Result<Value,ManagementError>{
        if !caller.can_manage(){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        let requested=if ids.is_empty(){self.repository.list_saved().map_err(|_|ManagementError::new(500,"SERVER_INFO_UNAVAILABLE","Saved server information is unavailable."))?.iter().filter_map(|v|v["server_id"].as_str().map(str::to_owned)).collect()}else{ids};
        let visible:Vec<String>=requested.iter().filter(|id|caller.permits("fetch:manage",id,None)).cloned().collect();
        let hidden:Vec<String>=requested.iter().filter(|id|!caller.permits("fetch:manage",id,None)).cloned().collect();
        if restart.is_some() && visible.len()!=1{return Err(ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found."));}
        let mut result=if visible.is_empty(){json!({"status":"ok","servers":[],"not_found_ids":[],"next_steps":[]})}else{self.servers(visible,restart).await?};
        if let Some(missing)=result["not_found_ids"].as_array_mut(){missing.extend(hidden.into_iter().map(Value::String));}
        if let Some(servers)=result["servers"].as_array_mut(){for record in servers{
            let id=record["server_id"].as_str().unwrap_or("").to_owned();
            // Startup output is for the person in Fetch, never an agent/MCP
            // response. The runner sanitizes it before keeping it in memory.
            if caller.is_native_settings(){
                if let Some(startup)=self.runner.startup_diagnostics(&id).await{
                    record["authentication"]["startup"]=startup;
                }
            }
            if let Some(timeouts)=record["config"]["execution"]["tool_timeouts_ms"].as_object_mut(){timeouts.retain(|name,_|caller.permits("fetch:catalog",&id,Some(name)));}
            if !caller.permits("fetch:catalog",&id,None){record["config"]["tool_features"]=json!({"status":"unavailable","tools":[]});}
            else if let Some(tools)=record["config"]["tool_features"]["tools"].as_array_mut(){tools.retain(|tool|tool["name"].as_str().is_some_and(|name|caller.permits("fetch:catalog",&id,Some(name))));}
        }}
        Ok(result)
    }
    pub async fn servers(&self,ids:Vec<String>,restart:Option<String>)->Result<Value,ManagementError>{
        let records=if ids.is_empty(){self.repository.list_saved().map_err(|_|ManagementError::new(500,"SERVER_INFO_UNAVAILABLE","Saved server information is unavailable."))?}else{let mut records=Vec::new();for id in &ids{if let Some(record)=self.repository.get_saved(id).map_err(|_|ManagementError::new(500,"SERVER_INFO_UNAVAILABLE","Saved server information is unavailable."))?{records.push(record);}}records};
        let missing:Vec<&String>=ids.iter().filter(|id|!records.iter().any(|v|v["server_id"].as_str()==Some(id.as_str()))).collect();
        if let Some(restart_id)=restart.as_deref(){if records.is_empty(){let mut error=ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found.");error.extra=json!({"servers":[],"not_found_ids":missing});return Err(error);}if ids.len()!=1{return Err(invalid());}if !self.attempts.lock().await.contains_key(&(ids[0].clone(),restart_id.to_owned())){let mut error=ManagementError::new(404,"RESTART_NOT_FOUND","No matching restart attempt exists in this Fetch run.");error.extra=json!({"servers":[],"not_found_ids":[]});return Err(error);}}
        let mut servers=Vec::new();let mut next=Vec::new();
        for mut record in records {
            let id=record["server_id"].as_str().ok_or_else(invalid)?.to_owned();
            // Saved records are atomic; a status read must not queue behind
            // this instance's potentially slow login or refresh request.
            record=self.record(&id)?;record.as_object_mut().map(|o|{o.remove("_installation");o.remove("credentials");});
            let snapshot=self.runner.snapshot(&id).await;
            if snapshot.get("runtime").is_some(){record["runtime"]=snapshot["runtime"].clone();}else if snapshot.get("status").is_some(){record["runtime"]=json!({"status":snapshot["status"],"error":snapshot.get("error").cloned().unwrap_or(Value::Null)});}
            record["tool_execution"]=snapshot.get("tool_execution").cloned().unwrap_or_else(||json!({"active_calls":0,"queued_calls":0,"uncertain_calls":0,"max_concurrent_calls":record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1),"max_queued_calls":64,"max_queued_bytes":16777216,"queue_wait_timeout_ms":300000}));
            if !self.runner.server_ids().await.iter().any(|known|known==&id){record["tool_execution"]["max_concurrent_calls"]=json!(record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1));}
            if record["config"]["kind"]=="remote" {record["unresolved_prior_calls"]=snapshot.get("unresolved_prior_calls").cloned().unwrap_or(json!(0));}
            let mut live=self.live.lock().await.get(&id).cloned().unwrap_or_default();
            if snapshot["provider_auth_required"]==true&&!live.authenticating&&!(live.auth["status"]=="authenticated"&&live.oauth_restart_pending){
                live.auth=json!({"status":"required","error":{"code":"PROVIDER_AUTH_REQUIRED","message":"The upstream requires provider authentication. Use this server's supported native setup, or Authenticate for supported remote OAuth."}});
            }
            if live.auth["status"]=="authenticated"&&live.access_expires_at.is_some_and(|expiry|expiry<=now()){live.auth["status"]=json!("unknown");}
            let last_known=record["authentication"].clone();
            record["authentication"]=live.auth.clone();record["authentication"]["access_expires_at"]=timestamp(live.access_expires_at);record["authentication"]["last_verified_at"]=timestamp(live.last_verified_at);
            record["authentication"]["last_known"]=last_known;
            record["authentication"]["sign_in"]=live.native_sign_in.as_ref().filter(|_|live.authenticating).map(local_auth::SignIn::public).unwrap_or(Value::Null);
            if record["runtime"]["status"]!="ready"{if let Some(error)=&live.runtime_error{record["runtime"]=json!({"status":"failed","error":error});}}
            if let Some(applied)=&live.applied{record["restart_required"]=json!(launch_config(applied)!=launch_config(&record["config"])||live.oauth_restart_pending);}
            let attempt_id=restart.as_ref().or(live.restart.as_ref());record["restart"]=if let Some(attempt)=attempt_id{self.attempts.lock().await.get(&(id.clone(),attempt.clone())).cloned().unwrap_or(Value::Null)}else{Value::Null};
            if record["restart"]["status"]=="restarting"{next.push(restart_step(&id,attempt_id.unwrap()));}
            else if live.auth["status"]=="authenticating" || record["runtime"]["status"]=="starting"{next.push(server_step(&id));}
            else if live.auth["status"]=="required"{next.push(json!({"actor":"agent","action":"restart_server","server_id":id,"instructions":"For an explicit user sign-in, POST this server's restart route with interactive:true, or use fetch__restart_server with the same option. Relay its setup instructions and ask the user before opening an offered sign-in host. This read has not started authentication."}));}
            else if record["restart_required"]==true || matches!(live.auth["status"].as_str(),Some("failed"|"unknown")) || matches!(record["runtime"]["status"].as_str(),Some("stopped"|"failed")){let mut step=settings_step("Review the server's native configuration and login requirements. Use Connect or Restart explicitly after changing its files. Readiness alone does not prove provider account access.");step["server_id"]=json!(id);next.push(step);}
            servers.push(record);
        }
        Ok(json!({"status":"ok","servers":servers,"not_found_ids":missing,"next_steps":next}))
    }
    pub async fn config(&self,id:&str,patch:Value,caller:&Caller)->Result<Value,ManagementError>{
        if !caller.permits("fetch:manage",id,None){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        let object=patch.as_object().ok_or_else(invalid)?;if object.keys().any(|k|!["execution"].contains(&k.as_str())){return Err(invalid());}
        let gate=self.lock(id).await;let _guard=gate.lock().await;let mut record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if let Some(applied)=&live.applied{record["restart_required"]=json!(launch_config(applied)!=launch_config(&record["config"])||live.oauth_restart_pending);}
        if live.authenticating{return Err(busy("Authentication is active. Monitor server status before editing."));}
        let mut next=record["config"].clone();let old=next.clone();
        if let Some(execution)=object.get("execution"){
            let changes=execution.as_object().ok_or_else(invalid)?;if changes.keys().any(|k|!["max_concurrent_calls","default_timeout_ms","tool_timeouts_ms"].contains(&k.as_str())){return Err(invalid());}
            if next.get("execution").is_none(){next["execution"]=json!({"max_concurrent_calls":1,"default_timeout_ms":120000,"tool_timeouts_ms":{}});}
            for key in ["max_concurrent_calls","default_timeout_ms"]{if let Some(value)=changes.get(key){let n=value.as_u64().ok_or_else(||if value.is_number(){invalid_config()}else{invalid()})?;if n==0 || (key=="max_concurrent_calls" && n>64) || (key=="default_timeout_ms" && n>86_400_000){return Err(invalid_config());}next["execution"][key]=value.clone();}}
            if let Some(timeouts)=changes.get("tool_timeouts_ms"){
                let timeouts=timeouts.as_object().ok_or_else(invalid)?;for(name,value)in timeouts{if name.is_empty() || !caller.permits("fetch:catalog",id,Some(name)){return Err(ManagementError::new(403,"FORBIDDEN","Permission to edit the selected tool settings is required."));}if value.is_null(){next["execution"]["tool_timeouts_ms"].as_object_mut().ok_or_else(invalid)?.remove(name);}else{let n=value.as_u64().ok_or_else(||if value.is_number(){invalid_config()}else{invalid()})?;if n==0 || n>86_400_000{return Err(invalid_config());}next["execution"]["tool_timeouts_ms"][name]=value.clone();}}
            }
        }
        policy(&next)?.validate().map_err(ManagementError::from)?;
        let changed=old!=next;
        if changed{record["config"]=next.clone();self.repository.save(id,&record).map_err(|_|ManagementError::new(500,"CONFIG_WRITE_FAILED","Fetch execution settings could not be saved."))?;let _=self.runner.update_execution(id,policy(&next)?).await;}
        Ok(json!({"server_id":id,"status":"ok","changed":changed,"restart_required":record["restart_required"],"next_steps":[]}))
    }
    pub async fn restart(self:&Arc<Self>,id:&str,body:Value)->Result<Value,ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(ManagementError::new(503,"SERVER_NOT_READY","Fetch is shutting down."));}
        let object=body.as_object().ok_or_else(invalid)?;if object.keys().any(|k|!matches!(k.as_str(),"force_restart"|"interactive")){return Err(invalid());}let interactive=match object.get("interactive"){None=>false,Some(Value::Bool(value))=>*value,_=>return Err(invalid())};let force=match object.get("force_restart"){None=>false,Some(Value::Bool(b))=>*b,_=>return Err(invalid())};
        if interactive {if force{return Err(invalid());}return self.authenticate_inner(id,true,false).await;}
        let initial=self.record(id)?;let _=self.renew_saved_remote(&initial).await;
        let gate=self.lock(id).await;let _guard=gate.lock().await;let captured=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if live.authenticating{return Err(busy("Authentication is active. Monitor server status first."));}
        if let Some(existing)=live.restart{if self.attempts.lock().await.get(&(id.to_owned(),existing.clone())).is_some_and(|v|v["status"]=="restarting"){let mut error=ManagementError::new(409,"RESTART_IN_PROGRESS","A restart or reconnect is already active.");error.extra=json!({"restart_id":existing});error.next_steps=vec![restart_step(id,&existing)];return Err(error);}}
        let (route,route_error)=match self.route(&captured).await{
            Ok(route)=>(Some(route),None),
            Err(error) if error.code=="REMOTE_OAUTH_STORAGE_UNAVAILABLE"=>{self.connection_failed(id,error.code,error.message).await;let mut failure=ManagementError::new(500,"RESTART_ACCEPTANCE_FAILED","The remote OAuth context could not be safely captured. Review Settings before retrying.");failure.next_steps=vec![settings_step("Resolve remote OAuth storage access before requesting another restart or reconnect.")];return Err(failure);},
            Err(error)=>(None,Some(error)),
        };
        let attempt=opaque("restart");let remote=captured["config"]["kind"]=="remote";
        let mut registered=self.runner.server_ids().await.iter().any(|known|known==id);
        if !registered {if let Some(route)=&route{self.runner.configure(route.clone()).await?;registered=true;}}
        // Registration and admission gating precede the accepted response.
        if registered {self.runner.begin_restart(id).await?;}
        {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.restart=Some(attempt.clone());state.runtime_error=None;}
        self.phase(id,&attempt,"waiting_for_calls",remote).await;
        let manager=self.clone();let server=id.to_owned();let restart=attempt.clone();
        tokio::spawn(async move{if let Some(route)=route{manager.perform_restart(server,restart,captured,route,force,remote).await;}else if let Some(error)=route_error{manager.failed_restart(&server,&restart,error.code,error.message,remote).await;manager.runner.reopen_admission(&server).await;}});
        let mut response=json!({"server_id":id,"restart_id":attempt,"status":"restarting","next_steps":[restart_step(id,&attempt)]});if remote{response["operation"]=json!("reconnect");}Ok(response)
    }
    async fn perform_restart(self:Arc<Self>,id:String,attempt:String,mut captured:Value,mut route:ServerRoute,force:bool,remote:bool){
        if self.stopping.load(Ordering::SeqCst){self.failed_restart(&id,&attempt,"SERVER_START_FAILED","Fetch is shutting down; no replacement was started.",remote).await;return;}
        if !force && !self.runner.drain(&id,Duration::from_secs(60)).await{self.failed_restart(&id,&attempt,"RESTART_WAIT_TIMEOUT","Existing calls did not finish within the drain deadline.",remote).await;self.runner.reopen_admission(&id).await;return;}
        self.phase(&id,&attempt,if remote{"disconnecting"}else{"stopping"},remote).await;
        match tokio::time::timeout(Duration::from_secs(5),self.runner.disconnect(&id,force)).await{Ok(Ok(()))=>{},_=>{self.failed_restart(&id,&attempt,if remote{"CONNECTION_CLOSE_FAILED"}else{"SERVER_STOP_FAILED"},"The previous context could not be confirmed closed. No replacement was started.",remote).await;return;}}
        self.phase(&id,&attempt,if remote{"connecting"}else{"starting"},remote).await;
        if self.stopping.load(Ordering::SeqCst){self.failed_restart(&id,&attempt,"SERVER_START_FAILED","Fetch is shutting down; no replacement was started.",remote).await;return;}
        if remote{
            // The previous calls may have occupied the entire drain interval.
            // Quietly re-enter the actual route now, then capture the current
            // saved grant instead of reconnecting with pre-drain headers.
            self.restart_auth.lock().await.insert(id.clone(),attempt.clone());
            let renewed=self.renew_saved_remote(&captured).await;
            self.restart_auth.lock().await.remove(&id);
            if let Err(error)=renewed{self.failed_restart(&id,&attempt,error.code,error.message,true).await;self.runner.reopen_admission(&id).await;return;}
            let gate=self.lock(&id).await;let _guard=gate.lock().await;
            captured=match self.record(&id){Ok(value)=>value,Err(error)=>{drop(_guard);self.failed_restart(&id,&attempt,error.code,error.message,true).await;return;}};
            route=match self.route(&captured).await{Ok(value)=>value,Err(error)=>{drop(_guard);self.failed_restart(&id,&attempt,error.code,error.message,true).await;self.runner.reopen_admission(&id).await;return;}};
        }
        if self.runner.configure(route).await.is_err(){self.failed_restart(&id,&attempt,if remote{"CONNECTION_FAILED"}else{"SERVER_START_FAILED"},"The replacement configuration could not be applied.",remote).await;return;}
        self.phase(&id,&attempt,"checking_readiness",remote).await;
        let result=tokio::time::timeout(Duration::from_secs(30),self.runner.connect(&id)).await;
        match result{Ok(Ok(()))=>{
            let gate=self.lock(&id).await;let _guard=gate.lock().await;
            if let Err(error)=self.connection_succeeded(&id,&captured).await{
                drop(_guard);let _=self.runner.disconnect(&id,true).await;self.failed_restart(&id,&attempt,error.code,error.message,remote).await;return;
            }
            if let Ok(mut latest)=self.record(&id){latest["restart_required"]=json!(launch_config(&latest["config"])!=launch_config(&captured["config"]));let _=self.repository.save(&id,&latest);}
            let mut value=json!({"restart_id":attempt,"status":"succeeded","code":if remote{"CONNECTION_READY"}else{"SERVICE_STARTED"}});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.clone(),attempt),value);self.runner.reopen_admission(&id).await;drop(_guard);self.observe_features(&id).await;
        },other=>{let(code,message)=match other{Err(_)=>(if remote{"CONNECTION_READY_TIMEOUT"}else{"SERVER_READY_TIMEOUT"},"Replacement readiness did not finish within thirty seconds."),Ok(Err(ref error))=>(error.code,error.message),Ok(Ok(()))=>unreachable!()};let _=self.runner.disconnect(&id,true).await;self.failed_restart(&id,&attempt,code,message,remote).await;}}
    }
    async fn phase(&self,id:&str,attempt:&str,phase:&str,remote:bool){let mut value=json!({"restart_id":attempt,"status":"restarting","phase":phase});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.to_owned(),attempt.to_owned()),value);}
    async fn failed_restart(&self,id:&str,attempt:&str,code:&str,message:&str,remote:bool){let gate=self.lock(id).await;let _guard=gate.lock().await;self.connection_failed(id,code,message).await;if self.live.lock().await.get(id).is_some_and(|state|state.auth["status"]=="required"){let _=self.remember_live_authentication(id).await;}let mut value=json!({"restart_id":attempt,"status":"failed","error":{"code":code,"message":message}});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.to_owned(),attempt.to_owned()),value);}
    async fn route(&self,record:&Value)->Result<ServerRoute,ManagementError>{
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let manual=self.manual_for_record(record).await?;
        let transport=if record["config"]["kind"]=="remote"{
            let document=self.grants.read_json(id).await.map_err(|_|store_error())?.unwrap_or_else(||json!({}));
            let mut access=None;if let Some(saved)=document.get("sdk").filter(|value|!value.is_null()){
                if saved["binding"]["endpoint"]!=record["config"]["connection"]["url"]{return Err(auth_required());}
                let metadata:rmcp::transport::auth::AuthorizationMetadata=serde_json::from_value(saved["binding"]["metadata"].clone()).map_err(|_|store_error())?;
                let endpoint=record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?;
                let store=sdk::Store::new(self.grants.clone(),id.to_owned(),endpoint.to_owned(),metadata);
                let credentials=store.snapshot().await.map_err(sdk_error)?.ok_or_else(auth_required)?;
                if sdk::expires_at(&credentials).is_some_and(|expiry|expiry<=now()){return Err(auth_required());}access=sdk::token(&credentials);
            }
            remote_transport(record,access,&manual)?
        }else{
            let root=self.repository.root().to_owned();let record=record.clone();tokio::task::spawn_blocking(move||native_state::transport(&root,&record,&manual)).await.map_err(|_|invalid_config())??
        };
        Ok(ServerRoute{server_id:id.to_owned(),transport,execution:policy(&record["config"])?})
    }
    pub async fn startup(self:&Arc<Self>){
        if let Ok(records)=self.repository.list_saved(){
            // A slow connection must not hold up startup of the other saved servers.
            futures_util::future::join_all(records.into_iter().map(|candidate|async move{
            if self.stopping.load(Ordering::SeqCst){return;}let Some(id)=candidate["server_id"].as_str()else{return};
            let _=self.renew_saved_remote(&candidate).await;
            let gate=self.lock(id).await;let guard=gate.lock().await;
            let record=match self.record(id){Ok(record)=>record,Err(_)=>return};
            let mut connected=false;
            match self.route(&record).await{
                Ok(route)=>match self.runner.configure(route).await{
                    Ok(())=>match tokio::time::timeout(Duration::from_secs(30),self.runner.connect(id)).await{
                        Ok(Ok(()))=>{
                            match self.connection_succeeded(id,&record).await{Ok(())=>connected=true,Err(error)=>{let _=self.runner.disconnect(id,true).await;self.connection_failed(id,error.code,error.message).await;}}
                        },
                        Ok(Err(error))=>{let _=self.runner.disconnect(id,true).await;self.connection_failed(id,error.code,error.message).await;},
                        Err(_)=>{let _=self.runner.disconnect(id,true).await;self.connection_failed(id,"SERVER_READY_TIMEOUT","Server readiness did not finish within thirty seconds.").await;},
                    },
                    Err(error)=>self.connection_failed(id,error.code,error.message).await,
                },
                Err(error)=>self.connection_failed(id,error.code,error.message).await,
            }
            if self.live.lock().await.get(id).is_some_and(|state|state.auth["status"]=="required"){let _=self.remember_live_authentication(id).await;}
            drop(guard);
            if connected {self.observe_features(id).await;}
            self.schedule_refresh(id.to_owned()).await;
            })).await;
        }
    }
    pub async fn shutdown(&self){self.stopping.store(true,Ordering::SeqCst);self.stop.notify_waiters();self.installer.shutdown();let tasks={let mut owned_tasks=self.local_tasks.lock().await;self.flows.lock().await.clear();for state in self.live.lock().await.values_mut(){state.native_sign_in=None;}std::mem::take(&mut *owned_tasks)};for task in tasks{let _=task.await;}let terminals=self.terminals.clone();if let Ok(Err(error))=tokio::task::spawn_blocking(move||terminals.shutdown()).await{crate::diagnostics::application_failure(error.code);}self.runner.shutdown().await;}
    async fn renew_saved_remote(&self,record:&Value)->Result<(),ManagementError>{
        if record["config"]["kind"]!="remote"{return Ok(());}
        let id=record["server_id"].as_str().ok_or_else(invalid)?;
        let document=self.grants.read_json(id).await.map_err(|_|store_error())?.unwrap_or_else(||json!({}));
        if document["sdk"].is_object(){self.request_background_authenticate(id).await?;}
        Ok(())
    }
    async fn manual_document(&self,id:&str)->Result<Value,ManagementError>{let document=self.grants.read_json(id).await.map_err(|_|store_error())?.unwrap_or_else(||json!({}));if document.get("manual").is_some_and(|value|!value.is_object()){return Err(store_error());}Ok(document)}
    async fn apply_remote_renewal(&self,id:&str,transport:&TransportConfig){
        if self.live.lock().await.get(id).is_some_and(|value|value.oauth_restart_pending){return;}
        if let TransportConfig::Http{headers,..}|TransportConfig::Sse{headers,..}=transport{if self.runner.update_remote_headers(id,headers.clone()).await.is_err(){let mut states=self.live.lock().await;let value=states.entry(id.to_owned()).or_default();value.oauth_restart_pending=value.applied.is_some();}}
    }
    async fn active_restart(&self,id:&str,live:&Live)->bool{if let Some(attempt)=&live.restart{self.attempts.lock().await.get(&(id.to_owned(),attempt.clone())).is_some_and(|value|value["status"]=="restarting")}else{false}}
    pub async fn authenticate(self:&Arc<Self>,id:&str)->Result<Value,ManagementError>{self.authenticate_mode(id,true).await}
    pub async fn authenticate_mode(self:&Arc<Self>,id:&str,interactive:bool)->Result<Value,ManagementError>{self.authenticate_inner(id,interactive,false).await}
    pub(crate) async fn authenticate_background(self:&Arc<Self>,id:&str)->Result<Value,ManagementError>{self.authenticate_inner(id,false,true).await}
    async fn authenticate_inner(self:&Arc<Self>,id:&str,interactive:bool,background:bool)->Result<Value,ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let gate=self.lock(id).await;let guard=gate.lock().await;let record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if self.active_restart(id,&live).await{
            let permitted=background&&self.restart_auth.lock().await.get(id).is_some_and(|attempt|live.restart.as_deref()==Some(attempt.as_str()));
            if !permitted{return Err(busy("Monitor the active restart before authenticating."));}
        }
        if record["config"]["kind"]!="remote"{
            return self.begin_local_authentication(id,record,live,interactive).await;
        }
        if live.authenticating{return Ok(auth_response(id,"authenticating",vec![server_step(id)]));}
        let endpoint=record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?;
        // A manually configured header is supplied as-is. It is not an OAuth
        // refresh token and must never be interpreted as one.
        let manual=self.manual_for_record(&record).await?;
        let diagnostic_transport=remote_transport(&record,None,&manual)?;
        let diagnostics=self.runner.begin_startup_diagnostics(id,&diagnostic_transport).await;
        let mut diagnostic_guard=DiagnosticGuard::new(diagnostics.clone());
        let has_manual_headers=record["config"]["connection"]["headers"].as_array().into_iter().flatten().any(|binding|binding["credential_ref"].as_str().is_some_and(|name|manual["manual"][name].is_string()));
        if has_manual_headers{
            let transport=remote_transport(&record,None,&manual)?;
            if let Err(error)=self.verify_remote(&transport,diagnostics.clone()).await{self.mark_failure(id,&error).await;return Err(error);}
            {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.auth=json!({"status":"authenticated","error":null});state.access_expires_at=None;state.last_verified_at=Some(now());}
            self.remember_live_authentication(id).await?;
            self.apply_remote_renewal(id,&transport).await;
            return Ok(auth_response(id,"authenticated",if live.oauth_restart_pending{vec![settings_step("Use Connect or Reconnect explicitly to apply the newly saved login or fixed headers to the existing connection.")]}else{vec![]}));
        }
        let mut prepared=match self.prepare_oauth(id,endpoint,diagnostics.clone()).await{Ok(value)=>value,Err(error)=>{self.mark_auth_error(id,&error).await;return Err(sdk_error(error));}};
        let has_saved=match prepared.store.snapshot().await{Ok(value)=>value.is_some(),Err(rmcp::transport::auth::AuthError::AuthorizationRequired)if interactive=>false,Err(error)=>{sdk::log_auth_error(&diagnostics,"oauth credential storage",&error);return Err(sdk_error(error));}};
        if has_saved{
            match prepared.reuse().await{
                Ok(token)=>{
                    let transport=remote_transport(&record,Some(token),&manual)?;
                    let accepted=match self.verify_remote(&transport,diagnostics.clone()).await{
                        Ok(())=>true,
                        Err(error) if matches!(error.code,"PROVIDER_AUTH_REQUIRED"|"SERVER_AUTH_REQUIRED")=>{
                            let required=auth_required();self.mark_failure(id,&required).await;
                            if !interactive{return Err(required);}false
                        },
                        Err(error)=>{self.mark_failure(id,&error).await;return Err(error);}
                    };
                    if accepted{
                        let credentials=prepared.store.snapshot().await.map_err(sdk_error)?.ok_or_else(auth_required)?;
                        {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.auth=json!({"status":"authenticated","error":null});state.access_expires_at=sdk::expires_at(&credentials);state.last_verified_at=Some(now());}
                        self.remember_live_authentication(id).await?;
                        self.apply_remote_renewal(id,&transport).await;
                        drop(guard);self.schedule_refresh(id.to_owned()).await;
                        return Ok(auth_response(id,"authenticated",if live.oauth_restart_pending{vec![settings_step("Use Connect or Reconnect explicitly to apply the newly saved login or fixed headers to the existing connection.")]}else{vec![]}));
                    }
                },
                Err(error)=>{let needs_login=matches!(error,rmcp::transport::auth::AuthError::AuthorizationRequired|rmcp::transport::auth::AuthError::TokenRefreshRejected(_));self.mark_auth_error(id,&error).await;if !interactive||!needs_login{return Err(sdk_error(error));}}
            }
        }
        if !interactive{let error=auth_required();self.mark_failure(id,&error).await;return Err(error);}
        let callback=remote_callback::Pending::bind().await?;
        let redirect=callback.redirect_uri();
        let store=prepared.store.clone();let session=prepared.begin(&redirect).await.map_err(sdk_error)?;
        let target=session.get_authorization_url().to_owned();
        let state=url::Url::parse(&target).map_err(|_|auth_required())?.query_pairs().find(|(key,_)|key=="state").map(|(_,value)|value.into_owned()).ok_or_else(auth_required)?;
        diagnostics.add_redactions([state.clone()]);
        let attempt=opaque("remote-auth");let expires=now()+600;
        let mut owned_tasks=self.local_tasks.lock().await;
        if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let(callback_close,callback_task)=callback.start(Arc::downgrade(self),state.clone(),expires,self.stop.clone());
        owned_tasks.retain(|task|!task.is_finished());owned_tasks.push(callback_task);
        {let mut states=self.live.lock().await;let value=states.entry(id.to_owned()).or_default();value.authenticating=true;value.auth_deadline=expires;value.auth_attempt=Some(attempt.clone());value.auth=json!({"status":"authenticating","error":null});}
        self.flows.lock().await.insert(state.clone(),ProviderFlow{server:id.to_owned(),attempt:attempt.clone(),session,store,revision:live.revision,expires,diagnostics:diagnostics.clone(),callback_close});
        drop(owned_tasks);
        diagnostic_guard.armed=false;
        let server=id.to_owned();drop(guard);
        if let Err(error)=self.present_authentication_browser(&target).await{diagnostics.text("oauth browser",&format!("{}: {}",error.code,error.message));self.flows.lock().await.remove(&state);self.end_auth_failure(&server,&attempt,error.code,error.message).await;return Err(error);}
        Ok(auth_response(id,"authenticating",vec![settings_step("Complete the remote service's browser login. Fetch uses the official MCP library and saves only its client grant in macOS Keychain."),server_step(id)]))
    }
    async fn mark_auth_error(&self,id:&str,error:&rmcp::transport::auth::AuthError){let safe=sdk_error_ref(error);self.mark_failure(id,&safe).await;}
    async fn mark_failure(&self,id:&str,error:&ManagementError){
        {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.auth=json!({"status":if matches!(error.code,"SERVER_AUTH_REQUIRED"|"PROVIDER_AUTH_REQUIRED"){"required"}else if error.code=="AUTHENTICATION_CHECK_UNSUPPORTED"{"unknown"}else{"failed"},"error":{"code":error.code,"message":error.message}});}
        if let Err(error)=self.remember_live_authentication(id).await{self.status_save_failed(id,&error).await;}
    }
    async fn end_auth_failure(&self,id:&str,attempt:&str,code:&str,message:&str){
        let gate=self.lock(id).await;let _guard=gate.lock().await;
        {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();if state.auth_attempt.as_deref()!=Some(attempt){return;}state.authenticating=false;state.auth_attempt=None;state.native_sign_in=None;state.auth=json!({"status":if code=="AUTHENTICATION_CHECK_UNSUPPORTED"{"unknown"}else{"failed"},"error":{"code":code,"message":message}});}
        if let Err(error)=self.remember_live_authentication(id).await{self.status_save_failed(id,&error).await;}
    }
    pub async fn provider_callback(self:&Arc<Self>,state:&str,code:Option<&str>,error:Option<&str>,issuer:Option<&str>)->Result<(),ManagementError>{
        let quit=self.stop.notified();tokio::pin!(quit);quit.as_mut().enable();
        let flow=self.flows.lock().await.remove(state).ok_or_else(invalid)?;flow.callback_close.close();let server=flow.server.clone();let attempt=flow.attempt.clone();let mut callback_guard=CallbackGuard{manager:Arc::downgrade(self),server:server.clone(),attempt:attempt.clone(),armed:true};let deadline=flow.expires;let gate=self.lock(&server).await;let guard=gate.lock().await;
        let record=self.record(&server)?;let live=self.live.lock().await.get(&server).cloned().unwrap_or_default();
        flow.diagnostics.add_redactions(code.into_iter().chain(std::iter::once(state)).map(str::to_owned));
        if let Some(error)=error{flow.diagnostics.text("oauth callback",&format!("The service rejected authorization: {error}"));}
        if flow.expires<=now()||error.is_some()||live.revision!=flow.revision||live.auth_attempt.as_deref()!=Some(&attempt)||self.stopping.load(Ordering::SeqCst){drop(guard);self.end_auth_failure(&server,&attempt,"AUTHENTICATION_FAILED","The login expired, was denied, or no longer owns this instance.").await;return Err(auth_required());}
        let work=async{
            flow.session.handle_callback_with_issuer(code.filter(|value|!value.is_empty()&&value.len()<=8192).ok_or_else(invalid)?,state,issuer).await.map_err(|error|{sdk::log_auth_error(&flow.diagnostics,"oauth callback",&error);sdk_error(error)})?;
            let credentials=flow.store.snapshot().await.map_err(|error|{sdk::log_auth_error(&flow.diagnostics,"oauth credential storage",&error);sdk_error(error)})?.ok_or_else(auth_required)?;
            let manual=self.manual_for_record(&record).await?;let transport=remote_transport(&record,sdk::token(&credentials),&manual)?;
            self.verify_remote(&transport,flow.diagnostics.clone()).await?;
            if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
            let mut states=self.live.lock().await;let value=states.entry(server.clone()).or_default();value.authenticating=false;value.auth_attempt=None;value.auth=json!({"status":"authenticated","error":null});value.access_expires_at=sdk::expires_at(&credentials);value.last_verified_at=Some(now());value.oauth_restart_pending=value.applied.is_some();
            drop(states);self.remember_live_authentication(&server).await?;
            Ok(())
        };
        let result=tokio::select!{result=tokio::time::timeout(Duration::from_secs(deadline.saturating_sub(now())),work)=>result.unwrap_or_else(|_|Err(ManagementError::new(409,"AUTHENTICATION_TIMEOUT","The login callback exceeded its original deadline."))),_=&mut quit=>Err(auth_required())};
        callback_guard.armed=false;
        drop(guard);
        if let Err(failure)=&result{flow.diagnostics.text("oauth callback",&format!("{}: {}",failure.code,failure.message));self.end_auth_failure(&server,&attempt,failure.code,failure.message).await;}
        if result.is_ok(){self.schedule_refresh(server).await;}result
    }
    async fn schedule_refresh(self:&Arc<Self>,id:String){
        if !self.scheduled.lock().await.insert(id.clone()){return;}
        let manager=self.clone();tokio::spawn(async move{
            loop{
                if manager.stopping.load(Ordering::SeqCst)||manager.record(&id).is_err(){break;}
                let expiry=manager.live.lock().await.get(&id).and_then(|value|value.access_expires_at);
                let Some(expiry)=expiry else{break;};
                tokio::select!{_=tokio::time::sleep(Duration::from_secs(expiry.saturating_sub(now()+29).max(1)))=>{},_=manager.stop.notified()=>break};
                if manager.request_background_authenticate(&id).await.is_err(){break;}
            }
            manager.scheduled.lock().await.remove(&id);
        });
    }
    async fn manual_for_record(&self,record:&Value)->Result<Value,ManagementError>{let inputs=bindings::InputBindings::from_record(record)?;if inputs.references().is_empty(){Ok(json!({"manual":{}}))}else{self.manual_document(record["server_id"].as_str().ok_or_else(invalid)?).await}}
    pub async fn credential_fingerprint(&self,id:&str,patch:&Value)->Result<String,ManagementError>{let gate=self.lock(id).await;let _guard=gate.lock().await;self.fingerprint(id,patch).await}
    async fn fingerprint(&self,id:&str,patch:&Value)->Result<String,ManagementError>{let record=self.record(id)?;let document=self.manual_document(id).await?;let bytes=serde_json::to_vec(&json!({"patch":patch,"config":record["config"],"manual":document["manual"]})).map_err(|_|invalid())?;Ok(format!("{:x}",Sha256::digest(bytes)))}
    pub async fn save_credentials(&self,id:&str,patch:Value,native:&Arc<NativeAuthority>,cap:&SettingsCapability)->Result<Value,ManagementError>{
        let gate=self.lock(id).await;let _guard=gate.lock().await;let record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if live.authenticating||self.active_restart(id,&live).await{return Err(busy("Finish the active authentication or restart before changing keys."));}
        native.validate_for(cap,id,&self.fingerprint(id,&patch).await?).map_err(|_|ManagementError::new(403,"FORBIDDEN","A current Mac confirmation for these exact Settings changes is required."))?;
        let inputs=bindings::InputBindings::from_record(&record)?;let assigned=inputs.references();
        let reserved: BTreeSet<String>=inputs.matching(bindings::Source::Environment,&["HOME".into(),"XDG_CONFIG_HOME".into(),"XDG_STATE_HOME".into()]).iter().map(|entry|entry.reference.clone()).collect();let document=self.manual_document(id).await?;let mut manual=document["manual"].as_object().cloned().unwrap_or_default();
        let object=patch.as_object().filter(|object|object.keys().all(|key|key=="set"||key=="remove")).ok_or_else(invalid)?;let mut names=BTreeSet::new();let mut affected=vec![];
        for item in object.get("set").unwrap_or(&json!([])).as_array().ok_or_else(invalid)?{let item=item.as_object().filter(|item|item.len()==2).ok_or_else(invalid)?;let name=item.get("name").and_then(Value::as_str).filter(|name|assigned.contains(*name)&&!reserved.contains(*name)).ok_or_else(invalid)?;let value=item.get("value").and_then(Value::as_str).filter(|value|!value.contains('\0')&&value.len()<=256*1024).ok_or_else(invalid)?;if !names.insert(name.to_owned()){return Err(invalid());}manual.insert(name.to_owned(),json!(value));affected.push(json!({"name":name,"status":"stored"}));}
        for item in object.get("remove").unwrap_or(&json!([])).as_array().ok_or_else(invalid)?{let name=item.as_str().filter(|name|assigned.contains(*name)&&!reserved.contains(*name)).ok_or_else(invalid)?;if !names.insert(name.to_owned()){return Err(invalid());}manual.remove(name);affected.push(json!({"name":name,"status":"missing"}));}
        let changed=document["manual"].as_object().cloned().unwrap_or_default()!=manual;if changed{self.grants.save_section(id,"manual",json!(manual)).await.map_err(|_|store_error())?;let mut states=self.live.lock().await;let value=states.entry(id.to_owned()).or_default();value.auth=json!({"status":"unknown","error":null});value.last_verified_at=None;value.access_expires_at=None;value.oauth_restart_pending=true;}
        Ok(json!({"server_id":id,"status":"stored","changed":changed,"verified":false,"credentials":affected,"restart_required":changed||live.oauth_restart_pending,"next_steps":[settings_step("The fixed keys were saved in macOS Keychain. Storage is not proof of account access. Connect or Restart explicitly to apply changed process/header inputs.")]}))
    }
    async fn observe_features(&self,id:&str){
        let catalog=match self.runner.cached_catalog(id,"raw").await{Ok(value)=>value,Err(_)=>return};
        let definitions=match self.runner.known_tools(id).await{Ok(v)=>v,Err(_)=>return};
        let gate=self.lock(id).await;let _guard=gate.lock().await;let Ok(mut record)=self.record(id)else{return};
        let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        let current=live.applied.as_ref().is_some_and(|applied|launch_config(applied)==launch_config(&record["config"]));
        let connection=if record["config"]["kind"]=="remote"{record["config"]["connection"]["transport"].as_str().unwrap_or("unknown")}else{"stdio"}.to_owned();
        let tools:Vec<Value>=definitions.iter().filter_map(|raw|crate::runner::raw::text(raw,"name")).map(|name|json!({"name":name,"timeout_ms":null,"provenance":"unknown","server_id":id,"connection_type":connection})).collect();
        record["config"]["tool_features"]=json!({"status":if !current{"stale"}else if catalog["status"]=="available"{"observed"}else{"partial"},"source_complete":catalog["status"]=="available","tools":tools});
        // Saving observations cannot replace explicit user execution overrides.
        let _=self.repository.save(id,&record);
    }
}
fn invalid()->ManagementError{ManagementError::new(400,"INVALID_REQUEST","The request does not match the supported operation fields.")}
fn invalid_config()->ManagementError{ManagementError::new(422,"INVALID_CONFIG","Use supported Fetch execution settings; server configuration is managed through its native files.")}
fn busy(message:&'static str)->ManagementError{let mut error=ManagementError::new(409,"SERVER_BUSY",message);error.next_steps=vec![settings_step("Monitor the existing operation. Do not submit an automatic retry.")];error}
fn auth_required()->ManagementError{let mut error=ManagementError::new(409,"SERVER_AUTH_REQUIRED","The remote MCP service requires a fresh supported login.");error.next_steps=vec![settings_step("Use Authenticate explicitly for supported remote OAuth, or complete the server's native setup.")];error}
fn store_error()->ManagementError{ManagementError::new(503,"REMOTE_OAUTH_STORAGE_UNAVAILABLE","Fetch could not read or safely save its remote MCP client grant in macOS Keychain. Existing server credential files were not read or changed.")}
fn settings_step(instructions:&str)->Value{json!({"actor":"user","action":"review_server_settings","instructions":instructions})}
fn server_step(id:&str)->Value{json!({"actor":"agent","action":"check_server_status","server_id":id,"instructions":format!("After one second GET /api/v1/servers?id={id}; inspect authentication and runtime separately.")})}
fn restart_step(id:&str,attempt:&str)->Value{json!({"actor":"agent","action":"check_restart_status","server_id":id,"instructions":format!("After one second GET /api/v1/servers?id={id}&restart_id={attempt}; acceptance is not completion.")})}
fn auth_response(id:&str,status:&str,steps:Vec<Value>)->Value{json!({"server_id":id,"status":status,"next_steps":steps})}
fn string_array(value:&Value)->Result<Vec<String>,ManagementError>{value.as_array().ok_or_else(invalid)?.iter().map(|value|value.as_str().filter(|value|!value.contains(['\0','\r','\n'])).map(str::to_owned).ok_or_else(invalid)).collect()}
fn launch_config(config:&Value)->Value{let mut value=config.clone();if let Some(object)=value.as_object_mut(){object.remove("execution");object.remove("tool_features");}value}
fn policy(config:&Value)->Result<ExecutionPolicy,ManagementError>{if config.get("execution").is_none(){return Ok(ExecutionPolicy::default());}serde_json::from_value(config["execution"].clone()).map_err(|_|invalid_config())}
fn now()->u64{sdk::now()}
fn sdk_error(error:rmcp::transport::auth::AuthError)->ManagementError{sdk_error_ref(&error)}
fn sdk_error_ref(error:&rmcp::transport::auth::AuthError)->ManagementError{use rmcp::transport::auth::AuthError;match error{AuthError::CredentialStoreError(_)=>store_error(),AuthError::NoAuthorizationSupport|AuthError::PkceUnsupported|AuthError::RegistrationFailed(_)=>ManagementError::new(422,"SERVER_AUTH_UNSUPPORTED","The service does not publish compatible standard OAuth setup or does not accept this Fetch client. No provider-specific login is guessed."),AuthError::AuthorizationRequired|AuthError::TokenExpired|AuthError::TokenRefreshRejected(_)=>auth_required(),AuthError::TokenRefreshFailed(_)=>ManagementError::new(503,"AUTHENTICATION_RENEWAL_FAILED","The existing login could not be renewed. Review server status before explicitly retrying; no browser was opened."),_=>ManagementError::new(409,"AUTHENTICATION_FAILED","The standard remote MCP login did not complete. Review service requirements before trying again.")}}
fn remote_transport(record:&Value,token:Option<String>,manual:&Value)->Result<TransportConfig,ManagementError>{
    let config=&record["config"];let mut headers=bindings::InputBindings::from_record(record)?.resolve_source(bindings::Source::Header,manual,if token.is_some(){&["Authorization"]}else{&[]})?;
    if let Some(token)=token{headers.insert("Authorization".into(),format!("Bearer {token}"));}
    for(name,value)in &headers{if reserved_header(name)||value.contains(['\r','\n','\0']){return Err(invalid_config());}}
    let endpoint=config["connection"]["url"].as_str().ok_or_else(invalid)?.to_owned();Ok(if config["connection"]["transport"]=="sse"{TransportConfig::Sse{endpoint,headers}}else{TransportConfig::Http{endpoint,headers}})
}
fn reserved_header(name:&str)->bool{let name=name.to_ascii_lowercase();["host","content-type","content-length","accept","connection","origin","cookie","transfer-encoding","te","trailer","upgrade"].contains(&name.as_str())||name.starts_with("mcp-")||name.starts_with("proxy-")}

fn timestamp(value:Option<u64>)->Value{value.and_then(|value|i64::try_from(value).ok()).and_then(|value|chrono::DateTime::<chrono::Utc>::from_timestamp(value,0)).map(|value|json!(value.to_rfc3339_opts(chrono::SecondsFormat::Secs,true))).unwrap_or(Value::Null)}
