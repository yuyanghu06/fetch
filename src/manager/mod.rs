//! Shared server state and approved operations. UI and MCP adapters use these same handlers.
mod railway;
mod provider_callback;
mod remote_auth;
#[cfg(test)]
mod auth_tests;
use crate::diagnostics::{send as provider_send, UpstreamOperation};
use crate::{authority::Caller, credentials::{CredentialStore, SecretDocument}, installation::{Installer, opaque, private_directory}, native_auth::{NativeAuthority, SettingsCapability}, runner::{Runner, ServerRoute, TransportConfig, ExecutionPolicy}, storage::ServerRepository};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap,BTreeSet}, path::{Path,PathBuf}, sync::{Arc,RwLock,atomic::{AtomicBool,Ordering}}, time::{Duration,SystemTime,UNIX_EPOCH}};
use tokio::sync::{Mutex,Notify};

#[derive(Clone,Debug)]
pub struct ManagementError { pub http_status:u16, pub code:&'static str, pub message:&'static str, pub extra:Value, pub next_steps:Vec<Value> }
impl ManagementError {
    pub fn new(status:u16,code:&'static str,message:&'static str)->Self {Self{http_status:status,code,message,extra:json!({}),next_steps:vec![]}}
    pub fn body(&self)->Value {let mut value=json!({"status":"failed","error":{"code":self.code,"message":self.message},"next_steps":self.next_steps});if let Some(extra)=self.extra.as_object(){for(k,v)in extra{value[k]=v.clone();}}value}
}
impl From<crate::installation::InstallError> for ManagementError {fn from(e:crate::installation::InstallError)->Self{let mut error=Self::new(e.http_status,e.code,e.message);if let Some(id)=e.installation_id{error.extra=json!({"installation_id":id});error.next_steps=vec![json!({"actor":"agent","action":"check_installation_status","instructions":format!("After one second GET /api/v1/setup/installations/{id}.")})];}error}}
impl From<crate::runner::RunError> for ManagementError {fn from(e:crate::runner::RunError)->Self{Self::new(e.http_status,e.code,e.message)}}
#[derive(Clone)]
struct Live { auth:Value, applied:Option<Value>, current_manual_hashes:BTreeMap<String,String>, applied_manual_hashes:BTreeMap<String,String>, oauth_restart_pending:bool, oauth_present:bool, revision:u64, authenticating:bool, auth_deadline:u64, auth_attempt:Option<String>, restart:Option<String> }
impl Default for Live {fn default()->Self{Self{auth:json!({"status":"unknown","error":null}),applied:None,current_manual_hashes:BTreeMap::new(),applied_manual_hashes:BTreeMap::new(),oauth_restart_pending:false,oauth_present:false,revision:0,authenticating:false,auth_deadline:0,auth_attempt:None,restart:None}}}
struct ProviderFlow {server:String, state:String, provider:Provider, verifier:String, client_id:String, token_endpoint:String, redirect:String, resource:Option<String>, remote_metadata:Option<remote_auth::Metadata>, revision:u64, expires:u64, _callback:Option<provider_callback::Lease>}
pub struct Manager {
    pub repository:ServerRepository, pub credentials:Arc<CredentialStore>, pub runner:Arc<Runner>, pub installer:Arc<Installer>,
    locks:Mutex<BTreeMap<String,Arc<Mutex<()>>>>, live:Mutex<BTreeMap<String,Live>>, attempts:Mutex<BTreeMap<(String,String),Value>>, flows:Mutex<BTreeMap<String,ProviderFlow>>, origin:RwLock<String>, http:reqwest::Client, stopping:AtomicBool, stop:Arc<Notify>,
    #[cfg(test)]
    verification_test_endpoint:Option<String>,
}
impl Manager {
    pub fn new(repository:ServerRepository,credentials:Arc<CredentialStore>,runner:Arc<Runner>,installer:Arc<Installer>)->Arc<Self>{Arc::new(Self{repository,credentials,runner,installer,locks:Mutex::new(BTreeMap::new()),live:Mutex::new(BTreeMap::new()),attempts:Mutex::new(BTreeMap::new()),flows:Mutex::new(BTreeMap::new()),origin:RwLock::new(String::new()),stopping:AtomicBool::new(false),stop:Arc::new(Notify::new()),http:reqwest::Client::builder().connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(20)).redirect(reqwest::redirect::Policy::none()).build().expect("static provider HTTP configuration"),#[cfg(test)] verification_test_endpoint:None})}
    pub fn set_origin(&self,origin:String){if let Ok(mut value)=self.origin.write(){*value=origin;}}
    async fn lock(&self,id:&str)->Arc<Mutex<()>>{self.locks.lock().await.entry(id.to_owned()).or_insert_with(||Arc::new(Mutex::new(()))).clone()}
    fn record(&self,id:&str)->Result<Value,ManagementError>{self.repository.get_saved(id).map_err(|_|ManagementError::new(500,"SERVER_INFO_UNAVAILABLE","Saved server information is unavailable."))?.ok_or_else(||ManagementError::new(404,"SERVER_NOT_FOUND","The selected server was not found."))}
    pub async fn install(self:&Arc<Self>,body:Value)->Result<Value,ManagementError>{Ok(self.installer.submit(body).await?)}
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
            let gate=self.lock(&id).await;let _guard=gate.lock().await;
            record=self.record(&id)?;record.as_object_mut().map(|o|o.remove("_installation"));
            let snapshot=self.runner.snapshot(&id).await;
            if snapshot.get("runtime").is_some(){record["runtime"]=snapshot["runtime"].clone();}else if snapshot.get("status").is_some(){record["runtime"]=json!({"status":snapshot["status"],"error":snapshot.get("error").cloned().unwrap_or(Value::Null)});}
            record["tool_execution"]=snapshot.get("tool_execution").cloned().unwrap_or_else(||json!({"active_calls":0,"queued_calls":0,"uncertain_calls":0,"max_concurrent_calls":record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1),"max_queued_calls":64,"max_queued_bytes":16777216,"queue_wait_timeout_ms":300000}));
            if !self.runner.server_ids().await.iter().any(|known|known==&id){record["tool_execution"]["max_concurrent_calls"]=json!(record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1));}
            if record["config"]["kind"]=="remote" {record["unresolved_prior_calls"]=snapshot.get("unresolved_prior_calls").cloned().unwrap_or(json!(0));}
            let live=self.live.lock().await.get(&id).cloned().unwrap_or_default();record["authentication"]=live.auth.clone();
            if let Some(applied)=&live.applied{record["restart_required"]=json!(launch_config(applied)!=launch_config(&record["config"]) || pending_manual(&live,&record["config"]));}
            let attempt_id=restart.as_ref().or(live.restart.as_ref());record["restart"]=if let Some(attempt)=attempt_id{self.attempts.lock().await.get(&(id.clone(),attempt.clone())).cloned().unwrap_or(Value::Null)}else{Value::Null};
            if record["restart"]["status"]=="restarting"{next.push(restart_step(&id,attempt_id.unwrap()));}
            else if live.auth["status"]=="authenticating" || record["runtime"]["status"]=="starting"{next.push(server_step(&id));}
            else if live.auth["status"]=="required"{next.push(json!({"actor":"agent","action":"authenticate_server","server_id":id,"instructions":"Use this server's existing authenticate POST for supported login or precise human Settings instructions. This read has not started authentication."}));}
            else if live.oauth_restart_pending&&record["config"]["kind"]!="remote"&&live.auth["status"]=="authenticated"{
                let mut step=settings_step("The refreshed provider credential is verified and saved. Restart this local server to give its process the new credential; Fetch has not interrupted or replayed any calls.");step["server_id"]=json!(id);next.push(step);
            }
            else if record["restart_required"]==true || matches!(live.auth["status"].as_str(),Some("failed"|"unknown")) || matches!(record["runtime"]["status"].as_str(),Some("stopped"|"failed")) || missing_binding(&record,live.oauth_present){let mut step=settings_step("Review missing entries, authentication and saved settings. Use Connect or Restart explicitly after required setup; a failed or stopped server is not ready.");step["server_id"]=json!(id);next.push(step);}
            servers.push(record);
        }
        Ok(json!({"status":"ok","servers":servers,"not_found_ids":missing,"next_steps":next}))
    }
    pub async fn config(&self,id:&str,patch:Value,caller:&Caller)->Result<Value,ManagementError>{
        if !caller.permits("fetch:manage",id,None){return Err(ManagementError::new(403,"FORBIDDEN","Server-management permission is required."));}
        let object=patch.as_object().ok_or_else(invalid)?;if object.keys().any(|k|!["args","working_directory","environment","connection","execution"].contains(&k.as_str())){return Err(invalid());}
        let gate=self.lock(id).await;let _guard=gate.lock().await;let mut record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if let Some(applied)=&live.applied{record["restart_required"]=json!(launch_config(applied)!=launch_config(&record["config"]) || pending_manual(&live,&record["config"]));}
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
        let remote=next["kind"]=="remote";
        if remote && ["args","working_directory","environment"].iter().any(|key|object.contains_key(*key)){return Err(invalid_config());}
        if let Some(connection)=object.get("connection"){
            if !remote{return Err(invalid_config());}let c=connection.as_object().ok_or_else(invalid)?;if c.keys().any(|k|k!="headers"){if c.keys().any(|k|k=="url"||k=="transport"){return Err(invalid_config());}return Err(invalid());}
            if let Some(headers)=c.get("headers"){validate_bindings(headers,true,&record)?;next["connection"]["headers"]=headers.clone();}
        }
        if let Some(args)=object.get("args"){
            let list=string_array(args)?;let prior=string_array(&old["args"])?;let prefix=record["_installation"]["protected_args"].as_u64().unwrap_or(0) as usize;
            if list!=prior && (list.len()<prefix || prior.len()<prefix || list[..prefix]!=prior[..prefix] || !safe_argument_edit(&record,&list[prefix..])){return Err(invalid_config());}next["args"]=args.clone();
        }
        if let Some(cwd)=object.get("working_directory"){if !cwd.is_null() && !cwd.as_str().is_some_and(|s|Path::new(s).is_absolute() && Path::new(s).is_dir() && !s.contains('\0')){return Err(invalid_config());}next["working_directory"]=cwd.clone();}
        if let Some(environment)=object.get("environment"){validate_bindings(environment,false,&record)?;next["environment"]=environment.clone();}
        policy(&next)?.validate().map_err(ManagementError::from)?;
        let changed=old!=next;let launch_changed=launch_config(&old)!=launch_config(&next);
        if launch_changed && next.get("tool_features").is_some(){next["tool_features"]["status"]=json!("stale");}
        if changed {
            record["config"]=next.clone();
            if launch_changed {record["restart_required"]=json!(live.applied.as_ref().is_some_and(|applied|launch_config(applied)!=launch_config(&next)) || pending_manual(&live,&next) || (remote && live.applied.is_none()));}
            self.repository.save(id,&record).map_err(|_|ManagementError::new(500,"CONFIG_WRITE_FAILED","The complete configuration could not be safely saved. Read server state before retrying."))?;
            let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.revision=state.revision.wrapping_add(1);if launch_changed{state.auth=json!({"status":"unknown","error":null});}drop(states);
            let _=self.runner.update_execution(id,policy(&next)?).await;
        }
        let steps=if let Some(restart)=live.restart{let attempt=self.attempts.lock().await.get(&(id.to_owned(),restart.clone())).cloned();if attempt.is_some_and(|v|v["status"]=="restarting"){vec![restart_step(id,&restart)]}else if launch_changed||record["restart_required"]==true{vec![settings_step("Settings were saved. Authentication evidence must be established for changed launch settings; Restart applies pending changes.")]}else{vec![]}}else if launch_changed||record["restart_required"]==true{vec![settings_step("Settings were saved. Review authentication and use Connect or Restart to apply them.")]}else{vec![]};
        Ok(json!({"server_id":id,"status":"ok","changed":changed,"restart_required":record["restart_required"],"next_steps":steps}))
    }
    pub async fn restart(self:&Arc<Self>,id:&str,body:Value)->Result<Value,ManagementError>{
        let object=body.as_object().ok_or_else(invalid)?;if object.keys().any(|k|k!="force_restart"){return Err(invalid());}let force=match object.get("force_restart"){None=>false,Some(Value::Bool(b))=>*b,_=>return Err(invalid())};
        let gate=self.lock(id).await;let _guard=gate.lock().await;let captured=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if live.authenticating{return Err(busy("Authentication is active. Monitor server status first."));}
        if let Some(existing)=live.restart{if self.attempts.lock().await.get(&(id.to_owned(),existing.clone())).is_some_and(|v|v["status"]=="restarting"){let mut error=ManagementError::new(409,"RESTART_IN_PROGRESS","A restart or reconnect is already active.");error.extra=json!({"restart_id":existing});error.next_steps=vec![restart_step(id,&existing)];return Err(error);}}
        if provider_kind(&captured)==Provider::Railway&&captured["config"]["kind"]!="remote"{
            let installation=railway::installation_id(&captured).ok_or_else(invalid_config)?;
            self.installer.ensure_railway_cli(installation).await.map_err(|_|ManagementError::new(503,"SERVER_RUNTIME_UNAVAILABLE","Railway's private CLI could not be prepared. Retry Connect after checking network access; no server was restarted."))?;
        }
        if self.refresh_owned(&captured).await.is_err(){self.verification_failed(id).await;return Err(auth_required());}
        let route=match self.route(&captured).await{Ok(route)=>Some(route),Err(error) if error.code=="CREDENTIALS_WRITE_FAILED"=>{let mut failure=ManagementError::new(500,"RESTART_ACCEPTANCE_FAILED","The private credential context could not be safely captured. Review Settings before retrying.");failure.next_steps=vec![settings_step("Resolve protected-store access before requesting another restart or reconnect.")];return Err(failure);},Err(_)=>None};
        let attempt=opaque("restart");let remote=captured["config"]["kind"]=="remote";
        let mut registered=self.runner.server_ids().await.iter().any(|known|known==id);
        if !registered {if let Some(route)=&route{self.runner.configure(route.clone()).await?;registered=true;}}
        // Registration and admission gating precede the accepted response.
        if registered {self.runner.begin_restart(id).await?;}
        self.live.lock().await.entry(id.to_owned()).or_default().restart=Some(attempt.clone());
        self.phase(id,&attempt,"waiting_for_calls",remote).await;
        let captured_hashes=self.live.lock().await.get(id).map(|state|state.current_manual_hashes.clone()).unwrap_or_default();
        let manager=self.clone();let server=id.to_owned();let restart=attempt.clone();
        tokio::spawn(async move{if let Some(route)=route{manager.perform_restart(server,restart,captured,route,captured_hashes,force,remote).await;}else{manager.failed_restart(&server,&restart,"SERVER_AUTH_REQUIRED","The saved connection needs valid assigned credentials or supported settings. The old context was not stopped and no call was replayed.",remote).await;manager.runner.reopen_admission(&server).await;}});
        let mut response=json!({"server_id":id,"restart_id":attempt,"status":"restarting","next_steps":[restart_step(id,&attempt)]});if remote{response["operation"]=json!("reconnect");}Ok(response)
    }
    async fn perform_restart(self:Arc<Self>,id:String,attempt:String,captured:Value,route:ServerRoute,captured_hashes:BTreeMap<String,String>,force:bool,remote:bool){
        if self.stopping.load(Ordering::SeqCst){self.failed_restart(&id,&attempt,"SERVER_START_FAILED","Fetch is shutting down; no replacement was started.",remote).await;return;}
        if !force && !self.runner.drain(&id,Duration::from_secs(60)).await{self.failed_restart(&id,&attempt,"RESTART_WAIT_TIMEOUT","Existing calls did not finish within the drain deadline.",remote).await;self.runner.reopen_admission(&id).await;return;}
        self.phase(&id,&attempt,if remote{"disconnecting"}else{"stopping"},remote).await;
        match tokio::time::timeout(Duration::from_secs(5),self.runner.disconnect(&id,force)).await{Ok(Ok(()))=>{},_=>{self.failed_restart(&id,&attempt,if remote{"CONNECTION_CLOSE_FAILED"}else{"SERVER_STOP_FAILED"},"The previous context could not be confirmed closed. No replacement was started.",remote).await;return;}}
        self.phase(&id,&attempt,if remote{"connecting"}else{"starting"},remote).await;
        if self.stopping.load(Ordering::SeqCst){self.failed_restart(&id,&attempt,"SERVER_START_FAILED","Fetch is shutting down; no replacement was started.",remote).await;return;}
        if self.runner.configure(route).await.is_err(){self.failed_restart(&id,&attempt,if remote{"CONNECTION_FAILED"}else{"SERVER_START_FAILED"},"The replacement configuration could not be applied.",remote).await;return;}
        self.phase(&id,&attempt,"checking_readiness",remote).await;
        let result=tokio::time::timeout(Duration::from_secs(30),self.runner.connect(&id)).await;
        match result{Ok(Ok(()))=>{
            let gate=self.lock(&id).await;let _guard=gate.lock().await;
            if let Ok(mut latest)=self.record(&id){latest["restart_required"]=json!(launch_config(&latest["config"])!=launch_config(&captured["config"]));let _=self.repository.save(&id,&latest);}
            {let mut states=self.live.lock().await;let state=states.entry(id.clone()).or_default();state.applied=Some(captured["config"].clone());state.applied_manual_hashes=captured_hashes;state.oauth_restart_pending=false;}
            let mut value=json!({"restart_id":attempt,"status":"succeeded","code":if remote{"CONNECTION_READY"}else{"SERVICE_STARTED"}});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.clone(),attempt),value);self.runner.reopen_admission(&id).await;drop(_guard);self.observe_features(&id).await;
        },other=>{let(code,message)=match other{Err(_)=>(if remote{"CONNECTION_READY_TIMEOUT"}else{"SERVER_READY_TIMEOUT"},"Replacement readiness did not finish within thirty seconds."),Ok(Err(ref e))if matches!(e.code,"SERVER_AUTH_REQUIRED"|"PROVIDER_AUTH_REQUIRED")=>("SERVER_AUTH_REQUIRED","Provider authentication is required. Complete supported setup before reconnecting."),_=>(if remote{"CONNECTION_FAILED"}else{"SERVER_START_FAILED"},"Compatible communication and a complete tool catalog could not be established.")};let _=self.runner.disconnect(&id,true).await;self.failed_restart(&id,&attempt,code,message,remote).await;}}
    }
    async fn phase(&self,id:&str,attempt:&str,phase:&str,remote:bool){let mut value=json!({"restart_id":attempt,"status":"restarting","phase":phase});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.to_owned(),attempt.to_owned()),value);}
    async fn failed_restart(&self,id:&str,attempt:&str,code:&str,message:&str,remote:bool){let mut value=json!({"restart_id":attempt,"status":"failed","error":{"code":code,"message":message}});if remote{value["operation"]=json!("reconnect");}self.attempts.lock().await.insert((id.to_owned(),attempt.to_owned()),value);}
    async fn route(&self,record:&Value)->Result<ServerRoute,ManagementError>{
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let config=&record["config"];let document=self.secrets(id)?;
        {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.current_manual_hashes=manual_hashes(&document);state.oauth_present=document["oauth"]["access_token"].is_string();}
        let railway=provider_kind(record)==Provider::Railway;
        let railway_token=if railway{
            if document["oauth"]["provider"]!="railway"||document["oauth"]["verified"]!=true
                ||document["oauth"]["expires_at"].as_u64().is_none_or(|expiry|expiry<=now()){return Err(auth_required());}
            Some(document["oauth"]["access_token"].as_str().ok_or_else(auth_required)?)
        }else{None};
        let transport=if config["kind"]=="remote"{
            let headers=remote_headers(config,&document)?;
            let endpoint=config["connection"]["url"].as_str().ok_or_else(invalid)?.to_owned();if config["connection"]["transport"]=="sse"{TransportConfig::Sse{endpoint,headers}}else{TransportConfig::Http{endpoint,headers}}
        }else{
            let mut environment=BTreeMap::new();for binding in config["environment"].as_array().ok_or_else(invalid)?{
                let name=binding["name"].as_str().ok_or_else(invalid)?;
                if railway&&["RAILWAY_API_TOKEN","RAILWAY_TOKEN","HOME","PATH","RAILWAY_NO_AUTO_UPDATE","RAILWAY_NO_TELEMETRY"].contains(&name){continue;}
                let reference=binding["credential_ref"].as_str().ok_or_else(invalid)?;let value=document["manual"][reference].as_str().ok_or_else(auth_required)?;
                environment.insert(name.to_owned(),value.to_owned());
            }
            let home=self.repository.root().join("servers").join(format!("{:x}",Sha256::digest(id.as_bytes()))).join("home");private_directory(&home).map_err(|_|ManagementError::new(500,"SERVER_START_FAILED","The private server home could not be prepared."))?;
            let executable:PathBuf=config["executable"].as_str().ok_or_else(invalid)?.into();let path=executable.parent().map(|p|format!("{}:/usr/bin:/bin",p.display())).unwrap_or_else(||"/usr/bin:/bin".into());environment.entry("PATH".into()).or_insert(path);environment.entry("HOME".into()).or_insert_with(||home.to_string_lossy().into_owned());environment.entry("LANG".into()).or_insert_with(||"en_US.UTF-8".into());
            if let Some(token)=railway_token{
                let cli=railway::cli_bin(self.repository.root(),record).ok_or_else(||ManagementError::new(503,"SERVER_RUNTIME_UNAVAILABLE","Railway needs its private CLI. Use Connect to prepare it before starting this server."))?;
                let node=executable.parent().ok_or_else(invalid)?;
                environment.insert("PATH".into(),format!("{}:{}:/usr/bin:/bin",cli.display(),node.display()));
                environment.insert("HOME".into(),home.to_string_lossy().into_owned());
                environment.insert("RAILWAY_API_TOKEN".into(),token.to_owned());
                environment.insert("RAILWAY_NO_AUTO_UPDATE".into(),"1".into());
                environment.insert("RAILWAY_NO_TELEMETRY".into(),"1".into());
            }
            // Relative server writes belong in its private home unless the import selects a directory.
            TransportConfig::Stdio{executable,args:string_array(&config["args"] )?,working_directory:Some(config["working_directory"].as_str().map(PathBuf::from).unwrap_or(home)),environment}
        };
        Ok(ServerRoute{server_id:id.to_owned(),transport,execution:policy(config)?})
    }
    pub async fn startup(self:&Arc<Self>){
        if let Ok(records)=self.repository.list_saved(){for candidate in records{
            if self.stopping.load(Ordering::SeqCst){break;}let Some(id)=candidate["server_id"].as_str()else{continue};
            let gate=self.lock(id).await;let guard=gate.lock().await;
            let record=match self.record(id){Ok(record)=>record,Err(_)=>continue};
            if self.refresh_owned(&record).await.is_err(){self.verification_failed(id).await;continue;}
            let mut connected=false;
            match self.route(&record).await{Ok(route)=>{if self.runner.configure(route).await.is_ok() && matches!(tokio::time::timeout(Duration::from_secs(30),self.runner.connect(id)).await,Ok(Ok(()))){let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.applied=Some(record["config"].clone());state.applied_manual_hashes=state.current_manual_hashes.clone();if matches!(provider_kind(&record),Provider::None){state.auth=json!({"status":"not_required","error":null});}connected=true;}},Err(_)=>{self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"required","error":null});}}
            drop(guard);
            if connected {self.observe_features(id).await;}
            self.schedule_refresh(id.to_owned()).await;
        }}
    }
    pub async fn shutdown(&self){self.stopping.store(true,Ordering::SeqCst);self.stop.notify_waiters();self.installer.shutdown();self.flows.lock().await.clear();self.runner.shutdown().await;}
    fn secrets(&self,id:&str)->Result<Value,ManagementError>{match self.credentials.read(id){Ok(Some(document))=>{let value:Value=serde_json::from_slice(document.expose()).map_err(|_|store_error())?;if !value.is_object() || !value["manual"].is_object(){return Err(store_error());}Ok(value)},Ok(None)=>Ok(json!({"manual":{},"oauth":null})),Err(_)=>Err(store_error())}}
    pub async fn credential_fingerprint(&self,id:&str,patch:&Value)->Result<String,ManagementError>{let gate=self.lock(id).await;let _guard=gate.lock().await;self.fingerprint(id,patch)}
    fn fingerprint(&self,id:&str,patch:&Value)->Result<String,ManagementError>{let record=self.record(id)?;let secrets=self.secrets(id)?;let bytes=serde_json::to_vec(&json!({"patch":patch,"config":record["config"],"secrets":secrets})).map_err(|_|invalid())?;Ok(format!("{:x}",Sha256::digest(bytes)))}
    pub async fn save_credentials(&self,id:&str,patch:Value,native:&Arc<NativeAuthority>,cap:&SettingsCapability)->Result<Value,ManagementError>{
        let gate=self.lock(id).await;let _guard=gate.lock().await;native.validate_for(cap,id,&self.fingerprint(id,&patch)?).map_err(|_|ManagementError::new(403,"FORBIDDEN","A current Mac confirmation for this exact Settings change is required."))?;
        self.save_credentials_locked(id,patch).await
    }
    // The caller holds this server's lock and has validated its native Settings capability.
    async fn save_credentials_locked(&self,id:&str,patch:Value)->Result<Value,ManagementError>{
        let mut record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();if live.authenticating || self.active_restart(id,&live).await{return Err(busy("Finish the active authentication or restart before changing credentials."));}
        let object=patch.as_object().ok_or_else(invalid)?;if object.keys().any(|k|k!="set"&&k!="remove"){return Err(invalid());}let empty=Vec::new();let set=match object.get("set"){None=>&empty,Some(v)=>v.as_array().ok_or_else(invalid)?};let remove=match object.get("remove"){None=>&empty,Some(v)=>v.as_array().ok_or_else(invalid)?};
        let mut names=BTreeSet::new();let mut document=self.secrets(id)?;let original=document.clone();let mut affected=Vec::new();
        for item in set{let o=item.as_object().filter(|o|o.len()==2).ok_or_else(invalid)?;let name=o.get("name").and_then(Value::as_str).filter(|n|credential_name(n)).ok_or_else(invalid_credential)?;let value=o.get("value").and_then(Value::as_str).filter(|v|!v.contains('\0')&&v.len()<=256*1024).ok_or_else(invalid_credential)?;if !names.insert(name.to_owned()){return Err(invalid());}document["manual"][name]=json!(value);affected.push(json!({"name":name,"status":"stored"}));}
        for item in remove{let name=item.as_str().filter(|n|credential_name(n)).ok_or_else(invalid_credential)?;if !names.insert(name.to_owned()){return Err(invalid());}document["manual"].as_object_mut().ok_or_else(store_error)?.remove(name);affected.push(json!({"name":name,"status":"missing"}));}
        let changed=original!=document;
        if changed{let bytes=serde_json::to_vec(&document).map_err(|_|store_error())?;self.credentials.save(id,&SecretDocument::new(bytes)).map_err(|_|store_error())?;let mut credentials=record["credentials"].as_array().cloned().unwrap_or_default();for item in &affected{if let Some(old)=credentials.iter_mut().find(|v|v["name"]==item["name"]){*old=item.clone();}else{credentials.push(item.clone());}}record["credentials"]=json!(credentials);let mut updated_live=live.clone();updated_live.current_manual_hashes=manual_hashes(&document);record["restart_required"]=json!(updated_live.applied.as_ref().is_some_and(|applied|launch_config(applied)!=launch_config(&record["config"])) || pending_manual(&updated_live,&record["config"]) || (live.applied.is_none() && record["config"]["kind"]=="remote"));{let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.auth=json!({"status":"unknown","error":null});state.current_manual_hashes=manual_hashes(&document);state.revision=state.revision.wrapping_add(1);}self.repository.save(id,&record).map_err(|_|store_error())?;}
        // Every submitted authentication value must participate in verification of
        // the just-saved document. An existing OAuth token cannot prove a new API key.
        let submitted:Vec<String>=set.iter().filter_map(|item|item["name"].as_str().map(str::to_owned)).collect();
        if submitted.is_empty() || !remove.is_empty() || self.verify_stored_with_manual(&record,&submitted).await.is_err(){self.verification_failed(id).await;return Err(verification_error());}
        self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"authenticated","error":null});
        Ok(json!({"server_id":id,"status":"ok","changed":changed,"credentials":affected,"restart_required":record["restart_required"],"next_steps":if record["restart_required"]==true{vec![settings_step("The stored credential passed its authenticated provider check. Restart or Reconnect applies pending changes.")]}else{vec![]}}))
    }
    async fn active_restart(&self,id:&str,live:&Live)->bool{if let Some(attempt)=&live.restart{self.attempts.lock().await.get(&(id.to_owned(),attempt.clone())).is_some_and(|v|v["status"]=="restarting")}else{false}}
    async fn verification_failed(&self,id:&str){self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"failed","error":{"code":"CREDENTIAL_VERIFICATION_FAILED","message":"The stored credential could not be verified with the provider. It may already be saved. Retry Authenticate for browser sign-in, or correct manual credentials in Settings."}});}
    async fn verify_stored(&self,record:&Value)->Result<(),ManagementError>{
        self.verify_stored_with_manual(record,&[]).await
    }
    async fn verify_stored_with_manual(&self,record:&Value,submitted:&[String])->Result<(),ManagementError>{
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let document=self.secrets(id)?;let provider=provider_kind(record);
        if provider==Provider::Railway&&(document["oauth"]["provider"]!="railway"
            ||document["oauth"]["expires_at"].as_u64().is_none_or(|expiry|expiry<=now())){return Err(verification_error());}
        let assigned=assigned_manual_references(record);
        if submitted.iter().any(|name|!assigned.contains(name)){return Err(verification_error());}
        let oauth=if record["config"]["kind"]=="remote"||provider==Provider::Railway{document["oauth"]["access_token"].as_str()}else{None};
        let mut required:BTreeSet<&str>=submitted.iter().map(String::as_str).collect();
        if oauth.is_none(){required.extend(assigned.iter().map(String::as_str));}
        if required.is_empty()&&oauth.is_none(){return Err(verification_error());}
        // Resolve every required value before probing so a missing member cannot
        // be silently replaced by a working sibling or by an old OAuth credential.
        let tokens:Vec<&str>=required.iter().map(|name|document["manual"][*name].as_str().ok_or_else(verification_error)).collect::<Result<_,_>>()?;
        for token in tokens{self.verify_credential(provider,token).await?;}
        if let Some(token)=oauth{self.verify_credential(provider,token).await?;}
        Ok(())
    }
    async fn verify_credential(&self,provider:Provider,token:&str)->Result<(),ManagementError>{
        let endpoint=match provider{
            Provider::Tavily=>"https://api.tavily.com/usage",
            Provider::Github=>"https://api.github.com/user",
            Provider::Linear=>"https://api.linear.app/graphql",
            Provider::Railway=>railway::ME,
            _=>return Err(verification_error()),
        };
        #[cfg(test)]
        let endpoint=self.verification_test_endpoint.as_deref().unwrap_or(endpoint);
        let token=token.strip_prefix("Bearer ").unwrap_or(&token);
        let response=match provider{
            Provider::Tavily=>provider_send(UpstreamOperation::ProviderVerification,self.http.get(endpoint).bearer_auth(token)).await,
            Provider::Github=>provider_send(UpstreamOperation::ProviderVerification,self.http.get(endpoint).bearer_auth(token).header("User-Agent","Fetch/0.1").header("Accept","application/vnd.github+json")).await,
            Provider::Linear=>provider_send(UpstreamOperation::ProviderVerification,self.http.post(endpoint).bearer_auth(token).json(&json!({"query":"query FetchCredentialVerification { viewer { id } }"}))).await,
            Provider::Railway=>provider_send(UpstreamOperation::ProviderVerification,self.http.get(endpoint).bearer_auth(token)).await,
            _=>return Err(verification_error())
        }.map_err(|_|verification_error())?;
        if !response.status().is_success(){return Err(verification_error());}
        match provider{
            Provider::Linear=>{let value=bounded_json(response).await.map_err(|_|verification_error())?;if !value["data"]["viewer"]["id"].is_string()||value.get("errors").is_some(){return Err(verification_error());}},
            Provider::Railway=>{let value=bounded_json(response).await.map_err(|_|verification_error())?;if !railway::subject_is_present(&value){return Err(verification_error());}},
            _=>{}
        }
        Ok(())
    }
    pub async fn authenticate(self:&Arc<Self>,id:&str)->Result<Value,ManagementError>{
        use crate::diagnostics::{authentication_outcome,AuthenticationOutcome as Outcome};
        let result=self.authenticate_inner(id).await;
        let outcome=match result.as_ref().ok().and_then(|value|value["status"].as_str()){
            Some("authenticating")=>Outcome::BrowserPending,Some("required")=>Outcome::CredentialRequired,
            Some("authenticated")=>Outcome::Verified,Some("not_required")=>Outcome::NotRequired,
            Some("unknown")=>Outcome::UnsupportedProvider,_=>Outcome::Failed,
        };
        authentication_outcome(None,outcome);result
    }
    async fn authenticate_inner(self:&Arc<Self>,id:&str)->Result<Value,ManagementError>{
        let gate=self.lock(id).await;let _guard=gate.lock().await;let record=self.record(id)?;let live=self.live.lock().await.get(id).cloned().unwrap_or_default();
        if self.active_restart(id,&live).await{return Err(busy("Monitor the active restart before authenticating."));}
        if live.authenticating{return Ok(auth_response(id,"authenticating",vec![server_step(id)]));}
        match provider_kind(&record){
            Provider::None=>{self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"not_required","error":null});return Ok(auth_response(id,"not_required",vec![]));},
            Provider::Unknown=>{self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"unknown","error":null});return Ok(auth_response(id,"unknown",vec![settings_step("Fetch has no documented credential ownership and verification adapter for this server. Review its setup requirements; readiness alone cannot prove provider login.")]));},
            _=>{}
        }
        let document=self.secrets(id)?;
        let has_values=document["oauth"]["access_token"].is_string() || assigned_manual_references(&record).iter().any(|entry|document["manual"][entry].is_string());
        let browser_supported=provider_kind(&record)==Provider::Railway || (matches!(provider_kind(&record),Provider::Linear|Provider::Remote) && record["config"]["kind"]=="remote");
        let retry_browser=browser_supported&&(live.auth["status"]=="failed"||(provider_kind(&record)==Provider::Railway
            &&(document["oauth"]["verified"]!=true||document["oauth"]["refresh_in_progress"]==true)));
        if has_values && !retry_browser {
            if self.refresh_owned(&record).await.is_err() || self.verify_stored(&record).await.is_err(){self.verification_failed(id).await;return Err(verification_error());}
            self.mark_oauth_verified(&record)?;
            {let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.auth=json!({"status":"authenticated","error":null});state.oauth_present=self.secrets(id)?["oauth"]["access_token"].is_string();}let steps=if record["restart_required"]==true||live.oauth_restart_pending{vec![settings_step("The stored credential passed its authenticated provider check. Connect or Restart explicitly to apply pending settings or a new provider login.")]}else{vec![]};return Ok(auth_response(id,"authenticated",steps));
        }
        if browser_supported {
            let revision=live.revision;let attempt=opaque("provider");let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();state.authenticating=true;state.auth_deadline=now()+600;state.auth_attempt=Some(attempt.clone());state.auth=json!({"status":"authenticating","error":null});drop(states);
            let instructions=if provider_kind(&record)==Provider::Railway{"Complete Railway sign-in in your browser and choose workspace/project access. Fetch requests normal member access, privately saves the new credential and verifies your Railway identity."}else{"Complete the provider sign-in in your browser. Fetch will privately save and verify the new credential."};
            let manager=self.clone();let server=id.to_owned();let request_id=crate::diagnostics::current_request_id();
            tokio::spawn(crate::diagnostics::with_request_context(request_id,async move{
                if let Err(error)=manager.begin_provider_login(&record,revision,&attempt).await{
                    manager.end_auth_failure(&server,&attempt,error.code,"Provider browser login could not begin. Check network access and retry Authenticate; no provider credential was verified.").await;
                }
            }));
            return Ok(auth_response(id,"authenticating",vec![json!({"actor":"user","action":"complete_browser_login","instructions":instructions}),server_step(id)]));
        }
        self.live.lock().await.entry(id.to_owned()).or_default().auth=json!({"status":"required","error":null});
        Ok(auth_response(id,"required",vec![settings_step("Enter this provider's API key or token in native Fetch Settings. Saving succeeds only after encrypted readback and an authenticated provider check. Do not send the value through an agent.")]))
    }
    async fn begin_provider_login(self:&Arc<Self>,record:&Value,revision:u64,attempt:&str)->Result<(),ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let provider=provider_kind(record);
        let callback=if provider==Provider::Railway{Some(provider_callback::Listener::bind().await?)}else{None};
        let redirect=if let Some(callback)=&callback{callback.redirect().to_owned()}else{
            let origin=self.origin.read().map_err(|_|auth_required())?.clone();if origin.is_empty(){return Err(auth_required());}
            format!("{origin}/oauth/provider/callback")
        };
        let remote_metadata=if provider==Provider::Remote{Some(remote_auth::discover(record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?).await?)}else{None};
        let (authorization,token,registration,resource)=if provider==Provider::Railway{
            (railway::AUTHORIZE.to_owned(),railway::TOKEN.to_owned(),railway::REGISTER.to_owned(),None)
        }else if provider==Provider::Linear{
            let endpoint=record["config"]["connection"]["url"].as_str().ok_or_else(invalid)?;
            let endpoint_url=url::Url::parse(endpoint).map_err(|_|invalid())?;
            if endpoint_url.scheme()!="https"||endpoint_url.host_str()!=Some("mcp.linear.app"){return Err(auth_required());}
            let metadata=self.provider_json("https://mcp.linear.app/.well-known/oauth-authorization-server").await?;
            let issuer=url::Url::parse(metadata["issuer"].as_str().ok_or_else(auth_required)?).map_err(|_|auth_required())?;
            if issuer.scheme()!="https"||issuer.host_str()!=Some("mcp.linear.app")||!issuer.username().is_empty()||issuer.password().is_some(){return Err(auth_required());}
            if !metadata["code_challenge_methods_supported"].as_array().is_some_and(|a|a.iter().any(|v|v=="S256")){return Err(auth_required());}
            (checked_linear_endpoint(metadata["authorization_endpoint"].as_str().ok_or_else(auth_required)?)?,
                checked_linear_endpoint(metadata["token_endpoint"].as_str().ok_or_else(auth_required)?)?,
                checked_linear_endpoint(metadata["registration_endpoint"].as_str().ok_or_else(auth_required)?)?,Some(endpoint.to_owned()))
        }else if let Some(metadata)=&remote_metadata{
            (metadata.authorization.clone(),metadata.token.clone(),metadata.registration.clone(),Some(metadata.resource.clone()))
        }else{return Err(auth_required());};
        // Registration is owned by this Fetch installation and this managed server.
        // Keep all returned registration material encrypted, never in saved config/API results.
        let mut document=self.secrets(id)?;
        let registration_record=&document["oauth_client"];
        let reusable=registration_record["provider"]==provider.name()&&registration_record["redirect"]==redirect
            &&registration_record["registration_endpoint"]==registration
            &&(provider!=Provider::Remote||registration_record["remote_metadata"]==serde_json::to_value(&remote_metadata).map_err(|_|store_error())?)
            &&registration_record["registration"]["client_id"].as_str().is_some_and(|s|!s.is_empty());
        let client_registration=if reusable{registration_record["registration"].clone()}else if let Some(metadata)=&remote_metadata{
            remote_auth::register(metadata,&redirect).await?
        }else{
            let body=if provider==Provider::Railway{railway::registration(&redirect)}else{
                json!({"client_name":"Fetch","redirect_uris":[redirect],"token_endpoint_auth_method":"none","grant_types":["authorization_code","refresh_token"],"response_types":["code"]})
            };
            let response=provider_send(UpstreamOperation::ProviderRegistration,self.http.post(&registration).json(&body)).await.map_err(|_|auth_required())?;
            bounded_json(response).await?
        };
        let client_id=client_registration["client_id"].as_str().filter(|s|!s.is_empty()&&s.len()<=8192).ok_or_else(auth_required)?.to_owned();
        if client_registration["token_endpoint_auth_method"].as_str().is_some_and(|method|method!="none"){
            return Err(auth_required());
        }
        if provider==Provider::Linear&&client_registration.get("client_secret").is_some_and(|v|!v.is_null()){return Err(auth_required());}
        let gate=self.lock(id).await;let guard=gate.lock().await;
        let live=self.live.lock().await.get(id).cloned().ok_or_else(auth_required)?;
        if live.revision!=revision||live.auth_attempt.as_deref()!=Some(attempt)||!live.authenticating||live.auth_deadline<=now()||self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        document=self.secrets(id)?;
        document["oauth_client"]=json!({"provider":provider.name(),"redirect":redirect,"registration_endpoint":registration,"registration":client_registration,"remote_metadata":remote_metadata});
        self.credentials.save(id,&SecretDocument::new(serde_json::to_vec(&document).map_err(|_|store_error())?)).map_err(|_|store_error())?;
        let stored=self.secrets(id)?;
        if stored["oauth_client"]["registration"]["client_id"]!=client_id{return Err(store_error());}
        let verifier=URL_SAFE_NO_PAD.encode(rand::random::<[u8;32]>());let challenge=URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        let flow=ProviderFlow{server:id.to_owned(),state:attempt.to_owned(),provider,verifier,client_id:client_id.clone(),token_endpoint:token,
            redirect:redirect.clone(),resource:resource.clone(),remote_metadata:remote_metadata.clone(),revision,expires:live.auth_deadline,_callback:None};
        {let mut flows=self.flows.lock().await;if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}flows.insert(attempt.to_owned(),flow);}
        if let Some(callback)=callback{
            let lease=callback.start(Arc::downgrade(self),self.stop.clone(),id.to_owned(),attempt.to_owned(),live.auth_deadline);
            let mut flows=self.flows.lock().await;
            if self.stopping.load(Ordering::SeqCst){flows.remove(attempt);return Err(auth_required());}
            let Some(flow)=flows.get_mut(attempt)else{return Err(auth_required());};flow._callback=Some(lease);
        }
        let mut target=url::Url::parse(&authorization).map_err(|_|auth_required())?;
        target.query_pairs_mut().append_pair("response_type","code").append_pair("client_id",&client_id).append_pair("redirect_uri",&redirect)
            .append_pair("code_challenge",&challenge).append_pair("code_challenge_method","S256").append_pair("state",attempt);
        if let Some(resource)=resource{target.query_pairs_mut().append_pair("resource",&resource);}
        if provider==Provider::Railway{target.query_pairs_mut().append_pair("scope",railway::SCOPES).append_pair("prompt","consent");}
        if let Some(metadata)=&remote_metadata{
            if let Some(scope)=&metadata.scope{target.query_pairs_mut().append_pair("scope",scope);}
            remote_auth::validate_browser_destination(target.as_str()).await?;
        }
        drop(guard);
        if self.stopping.load(Ordering::SeqCst){self.flows.lock().await.remove(attempt);return Err(auth_required());}
        let mut browser=tokio::process::Command::new("/usr/bin/open");browser.arg(target.as_str()).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).kill_on_drop(true);
        let opened=tokio::time::timeout(Duration::from_secs(5),browser.status()).await;
        if !matches!(opened,Ok(Ok(s)) if s.success()){self.flows.lock().await.remove(attempt);return Err(auth_required());}
        let manager=Arc::downgrade(self);let server=id.to_owned();let state=attempt.to_owned();let expires=live.auth_deadline;
        let request_id=crate::diagnostics::current_request_id();
        tokio::spawn(crate::diagnostics::with_request_context(request_id,async move{tokio::time::sleep(Duration::from_secs(expires.saturating_sub(now()))).await;
            if let Some(manager)=manager.upgrade(){manager.flows.lock().await.remove(&state);manager.end_auth_failure(&server,&state,"AUTHENTICATION_TIMEOUT","The browser login expired after ten minutes. Retry Authenticate when ready.").await;}}));
        Ok(())
    }
    pub async fn provider_callback(self:&Arc<Self>,state:&str,code:Option<&str>,error:Option<&str>,issuer:Option<&str>)->Result<(),ManagementError>{
        let flow=self.flows.lock().await.remove(state).ok_or_else(||ManagementError::new(400,"INVALID_REQUEST","This provider login is no longer pending."))?;
        let server=flow.server.clone();let attempt=flow.state.clone();
        let mut interrupted=provider_callback::AttemptGuard::new(Arc::downgrade(self),server.clone(),attempt.clone());
        let remaining=Duration::from_secs(flow.expires.saturating_sub(now()));
        let result=tokio::time::timeout(remaining,self.finish_provider_callback(flow,code,error,issuer)).await
            .unwrap_or_else(|_|Err(ManagementError::new(400,"AUTHENTICATION_TIMEOUT","Provider sign-in exceeded its original deadline. Start a fresh sign-in.")));
        if let Err(ref failure)=result {self.end_auth_failure(&server,&attempt,failure.code,"Provider sign-in was denied, expired or could not be verified. Retry Authenticate to start fresh; review provider access and workspace/project consent.").await;}
        interrupted.disarm();
        result
    }
    async fn finish_provider_callback(self:&Arc<Self>,flow:ProviderFlow,code:Option<&str>,error:Option<&str>,issuer:Option<&str>)->Result<(),ManagementError>{
        let gate=self.lock(&flow.server).await;let _guard=gate.lock().await;
        let live=self.live.lock().await.get(&flow.server).cloned().unwrap_or_default();
        let record=self.record(&flow.server)?;
        if flow.expires<=now(){return Err(ManagementError::new(400,"AUTHENTICATION_TIMEOUT","This provider login expired. Start a fresh sign-in."));}
        if live.revision!=flow.revision||live.auth_attempt.as_deref()!=Some(flow.state.as_str())
            ||!live.authenticating||error.is_some()||provider_kind(&record)!=flow.provider||self.stopping.load(Ordering::SeqCst){
            return Err(ManagementError::new(400,"AUTHENTICATION_FAILED","Provider login was denied, cancelled or its saved settings changed."));
        }
        // Generic metadata may deliberately use distinct public authorization
        // and token origins. Bind the callback issuer before transmitting either
        // the authorization code or its PKCE verifier to that token endpoint.
        if let Some(metadata)=&flow.remote_metadata{metadata.validate_response_issuer(issuer)?;}
        let code=code.filter(|c|!c.is_empty()&&c.len()<=8192).ok_or_else(auth_required)?;
        let mut fields=vec![("grant_type","authorization_code"),("code",code),("client_id",flow.client_id.as_str()),
            ("redirect_uri",flow.redirect.as_str()),("code_verifier",flow.verifier.as_str())];
        if let Some(resource)=flow.resource.as_deref(){fields.push(("resource",resource));}
        let token=if let Some(metadata)=&flow.remote_metadata{remote_auth::token(metadata,&fields,false).await?}else{
            let response=provider_send(UpstreamOperation::ProviderTokenExchange,self.http.post(&flow.token_endpoint).form(&fields)).await.map_err(|_|auth_required())?;
            bounded_json(response).await?
        };
        if flow.expires<=now()||self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let mut document=self.secrets(&flow.server)?;
        document["oauth"]=oauth_document(&token,flow.provider,&flow.client_id,&flow.token_endpoint,flow.resource.as_deref(),None)?;
        if let Some(metadata)=&flow.remote_metadata{document["oauth"]["remote_metadata"]=serde_json::to_value(metadata).map_err(|_|store_error())?;}
        self.credentials.save(&flow.server,&SecretDocument::new(serde_json::to_vec(&document).map_err(|_|store_error())?)).map_err(|_|store_error())?;
        if self.verify_stored(&record).await.is_err(){self.verification_failed(&flow.server).await;return Err(verification_error());}
        if flow.expires<=now()||self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        self.mark_oauth_verified(&record)?;
        if flow.expires<=now()||self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let mut states=self.live.lock().await;let live=states.entry(flow.server.clone()).or_default();
        if live.auth_attempt.as_deref()!=Some(flow.state.as_str()){return Err(auth_required());}
        live.oauth_present=true;if live.applied.is_some(){live.oauth_restart_pending=true;}
        live.authenticating=false;live.auth_attempt=None;live.auth=json!({"status":"authenticated","error":null});drop(states);
        crate::diagnostics::authentication_outcome(None,crate::diagnostics::AuthenticationOutcome::Verified);
        self.schedule_refresh(flow.server).await;Ok(())
    }
    fn mark_oauth_verified(&self,record:&Value)->Result<(),ManagementError>{
        if !matches!(provider_kind(record),Provider::Railway|Provider::Remote){return Ok(());}
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let mut document=self.secrets(id)?;
        if !document["oauth"]["access_token"].is_string()||document["oauth"]["provider"]!=provider_kind(record).name(){return Err(auth_required());}
        document["oauth"]["verified"]=json!(true);
        self.credentials.save(id,&SecretDocument::new(serde_json::to_vec(&document).map_err(|_|store_error())?)).map_err(|_|store_error())?;
        if self.secrets(id)?["oauth"]["verified"]!=true{return Err(store_error());}Ok(())
    }
    async fn provider_json(&self,url:&str)->Result<Value,ManagementError>{
        let response=provider_send(UpstreamOperation::ProviderMetadata,self.http.get(url)).await.map_err(|_|auth_required())?;bounded_json(response).await
    }
    async fn end_auth_failure(&self,id:&str,attempt:&str,code:&str,message:&str){
        let mut states=self.live.lock().await;let live=states.entry(id.to_owned()).or_default();
        if live.auth_attempt.as_deref()!=Some(attempt){return;}
        live.authenticating=false;live.auth_attempt=None;live.auth=json!({"status":"failed","error":{"code":code,"message":message}});drop(states);
        crate::diagnostics::authentication_outcome(None,crate::diagnostics::AuthenticationOutcome::Failed);
    }
    async fn refresh_owned(&self,record:&Value)->Result<(),ManagementError>{
        if self.stopping.load(Ordering::SeqCst){return Err(auth_required());}
        let id=record["server_id"].as_str().ok_or_else(invalid)?;let mut document=self.secrets(id)?;let oauth=document["oauth"].clone();
        let provider=provider_kind(record);
        // Railway rotates refresh tokens. An interrupted exchange must never
        // retry a possibly consumed token, including after an application restart.
        if matches!(provider,Provider::Railway|Provider::Remote)&&oauth["refresh_in_progress"]==true{return Err(auth_required());}
        if !oauth.is_object()||oauth["expires_at"].as_u64().is_none_or(|t|t>now()+60){return Ok(());}
        let refresh=oauth["refresh_token"].as_str().ok_or_else(auth_required)?;
        if oauth["provider"].as_str().is_some_and(|stored|stored!=provider.name()){return Err(auth_required());}
        let saved_endpoint=oauth["token_endpoint"].as_str().ok_or_else(auth_required)?;
        let remote_metadata=if provider==Provider::Remote{
            let metadata=remote_auth::Metadata::from_saved(&oauth["remote_metadata"],record["config"]["connection"]["url"].as_str().ok_or_else(auth_required)?)?;
            let client_metadata=&document["oauth_client"];
            if saved_endpoint!=metadata.token||oauth["resource"]!=metadata.resource||client_metadata["remote_metadata"]!=oauth["remote_metadata"]
                ||client_metadata["registration"]["client_id"]!=oauth["client_id"]{return Err(auth_required());}
            Some(metadata)
        }else{None};
        let endpoint=if provider==Provider::Railway&&saved_endpoint==railway::TOKEN{railway::TOKEN.to_owned()}
            else if provider==Provider::Linear{checked_linear_endpoint(saved_endpoint)?}
            else if let Some(metadata)=&remote_metadata{metadata.token.clone()}else{return Err(auth_required());};
        let client=oauth["client_id"].as_str().ok_or_else(auth_required)?;let resource=oauth["resource"].as_str();
        if provider==Provider::Linear&&resource!=record["config"]["connection"]["url"].as_str(){return Err(auth_required());}
        let mut fields=vec![("grant_type","refresh_token"),("refresh_token",refresh),("client_id",client)];
        if matches!(provider,Provider::Linear|Provider::Remote){fields.push(("resource",resource.ok_or_else(auth_required)?));}
        if matches!(provider,Provider::Railway|Provider::Remote){
            document["oauth"]["refresh_in_progress"]=json!(true);
            self.credentials.save(id,&SecretDocument::new(serde_json::to_vec(&document).map_err(|_|store_error())?)).map_err(|_|store_error())?;
        }
        let value=if let Some(metadata)=&remote_metadata{remote_auth::token(metadata,&fields,true).await?}else{
            let response=provider_send(UpstreamOperation::ProviderRefresh,self.http.post(&endpoint).form(&fields)).await.map_err(|_|auth_required())?;
            bounded_json(response).await?
        };
        document["oauth"]=oauth_document(&value,provider,client,&endpoint,resource,Some(refresh))?;
        if let Some(metadata)=&remote_metadata{document["oauth"]["remote_metadata"]=serde_json::to_value(metadata).map_err(|_|store_error())?;}
        self.credentials.save(id,&SecretDocument::new(serde_json::to_vec(&document).map_err(|_|store_error())?)).map_err(|_|store_error())?;
        self.verify_stored(record).await?;self.mark_oauth_verified(record)?;
        if record["config"]["kind"]!="remote"{
            let mut states=self.live.lock().await;let state=states.entry(id.to_owned()).or_default();if state.applied.is_some(){state.oauth_restart_pending=true;}return Ok(());
        }
        let applied=self.live.lock().await.get(id).and_then(|live|live.applied.clone()).unwrap_or_else(||record["config"].clone());
        let headers=remote_headers(&applied,&document)?;
        let pending=self.live.lock().await.get(id).is_some_and(|live|live.oauth_restart_pending);
        if !pending&&self.runner.update_remote_headers(id,headers).await.is_err(){let mut states=self.live.lock().await;let live=states.entry(id.to_owned()).or_default();if live.applied.is_some(){live.oauth_restart_pending=true;}}
        Ok(())
    }
    async fn schedule_refresh(self:&Arc<Self>,id:String){let manager=self.clone();let request_id=crate::diagnostics::current_request_id();tokio::spawn(crate::diagnostics::with_request_context(request_id,async move{
        loop {
            if manager.stopping.load(Ordering::SeqCst){break;}
            let document=match manager.secrets(&id){Ok(v)=>v,Err(_)=>break};
            if !document["oauth"]["refresh_token"].is_string(){break;}let Some(expiry)=document["oauth"]["expires_at"].as_u64()else{break};
            tokio::select!{_=tokio::time::sleep(Duration::from_secs(expiry.saturating_sub(now()+60).max(1)))=>{},_=manager.stop.notified()=>break}
            let gate=manager.lock(&id).await;let guard=gate.lock().await;let record=match manager.record(&id){Ok(v)=>v,Err(_)=>break};
            let live=manager.live.lock().await.get(&id).cloned().unwrap_or_default();if manager.active_restart(&id,&live).await || live.authenticating{drop(guard);tokio::select!{_=tokio::time::sleep(Duration::from_secs(1))=>{},_=manager.stop.notified()=>break}continue;}
            if manager.refresh_owned(&record).await.is_err(){manager.verification_failed(&id).await;break;}
        }
    }));}
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
fn invalid_config()->ManagementError{ManagementError::new(422,"INVALID_CONFIG","The complete configuration cannot be safely applied. Keep the imported entrypoint and use supported non-secret arguments and server-local references.")}
fn invalid_credential()->ManagementError{ManagementError::new(422,"INVALID_CREDENTIAL_ENTRY","Use valid manual entry names and string values. Backend-owned OAuth entries cannot be edited manually.")}
fn busy(message:&'static str)->ManagementError{let mut e=ManagementError::new(409,"SERVER_BUSY",message);e.next_steps=vec![settings_step("Monitor the existing server or named restart status once per second. Do not submit an automatic retry.")];e}
fn auth_required()->ManagementError{let mut e=ManagementError::new(409,"SERVER_AUTH_REQUIRED","Supported provider setup or credential access is required.");e.next_steps=vec![settings_step("Complete supported authentication or review credentials in native Fetch Settings.")];e}
fn store_error()->ManagementError{ManagementError::new(500,"CREDENTIALS_WRITE_FAILED","The encrypted credential document could not be safely read, saved and read back. Existing data has not been deliberately reset.")}
fn verification_error()->ManagementError{let mut e=ManagementError::new(422,"CREDENTIAL_VERIFICATION_FAILED","The stored credential could not be verified with a supported authenticated service endpoint. A save may already have committed; no unverified success is reported.");e.next_steps=vec![settings_step("Review the saved credential and the supported provider verification requirements in native Fetch Settings. Retry explicitly after correcting the problem.")];e}
fn settings_step(instructions:&str)->Value{json!({"actor":"user","action":"review_server_settings","instructions":instructions})}
fn server_step(id:&str)->Value{json!({"actor":"agent","action":"check_server_status","server_id":id,"instructions":format!("After one second GET /api/v1/servers?id={id}; continue only after verified authentication or confirmed no-auth state.")})}
fn restart_step(id:&str,attempt:&str)->Value{json!({"actor":"agent","action":"check_restart_status","server_id":id,"instructions":format!("After one second GET /api/v1/servers?id={id}&restart_id={attempt}; acceptance is not completion.")})}
fn auth_response(id:&str,status:&str,steps:Vec<Value>)->Value{json!({"server_id":id,"status":status,"next_steps":steps})}
fn string_array(v:&Value)->Result<Vec<String>,ManagementError>{v.as_array().ok_or_else(invalid)?.iter().map(|v|v.as_str().filter(|s|!s.contains(['\0','\r','\n'])).map(str::to_owned).ok_or_else(invalid)).collect()}
fn launch_config(config:&Value)->Value{let mut result=config.clone();if let Some(o)=result.as_object_mut(){o.remove("execution");o.remove("tool_features");}result}
fn policy(config:&Value)->Result<ExecutionPolicy,ManagementError>{if config.get("execution").is_none(){return Ok(ExecutionPolicy::default());}serde_json::from_value(config["execution"].clone()).map_err(|_|invalid_config())}
fn credential_name(name:&str)->bool{!name.trim().is_empty()&&!name.contains('\0')&&name.len()<=256&&!name.starts_with("_fetch")&&!name.starts_with("oauth:")}
fn env_name(name:&str)->bool{let mut b=name.bytes();b.next().is_some_and(|b|b.is_ascii_alphabetic()||b==b'_')&&b.all(|b|b.is_ascii_alphanumeric()||b==b'_')}
fn header_name(name:&str)->bool{!name.is_empty()&&name.bytes().all(|b|b.is_ascii_alphanumeric()||b"!#$%&'*+-.^_`|~".contains(&b))}
fn validate_bindings(value:&Value,header:bool,record:&Value)->Result<(),ManagementError>{let rows=value.as_array().ok_or_else(invalid)?;let mut names=BTreeSet::new();for row in rows{let o=row.as_object().filter(|o|o.len()==2&&o.contains_key("name")&&o.contains_key("credential_ref")).ok_or_else(invalid)?;let name=o["name"].as_str().ok_or_else(invalid)?;let reference=o["credential_ref"].as_str().filter(|s|credential_name(s)).ok_or_else(invalid_config)?;
    if !(if header{header_name(name)}else{env_name(name)})||!names.insert(if header{name.to_ascii_lowercase()}else{name.to_owned()}){return Err(invalid_config());}
    if header && reserved_header(name){return Err(invalid_config());}
    if !record["credentials"].as_array().is_some_and(|a|a.iter().any(|v|v["name"]==reference)){return Err(invalid_config());}
}Ok(())}
fn safe_argument_edit(record:&Value,args:&[String])->bool{let name=record["config"]["package"]["name"].as_str().unwrap_or("");match name{"@modelcontextprotocol/server-filesystem"=>!args.is_empty()&&args.iter().all(|s|Path::new(s).is_absolute()&&!s.contains(['\0','\r','\n'])) ,"mcp-server-time"=>args.iter().all(|s|s.strip_prefix("--local-timezone=").is_some_and(|v|!v.is_empty()&&v.chars().all(|c|c.is_ascii_alphanumeric()||"/_+-".contains(c)))),_=>args.is_empty()}}
#[derive(Clone,Copy,PartialEq,Eq)]enum Provider{None,Tavily,Linear,Github,Railway,Remote,Unknown}
impl Provider {fn name(self)->&'static str{match self{Self::None=>"none",Self::Tavily=>"tavily",Self::Linear=>"linear",Self::Github=>"github",Self::Railway=>"railway",Self::Remote=>"remote",Self::Unknown=>"unknown"}}}
fn provider_kind(record:&Value)->Provider{if railway::matches_record(record){return Provider::Railway;}match record["config"]["package"]["name"].as_str(){Some("@modelcontextprotocol/server-everything"|"@modelcontextprotocol/server-filesystem"|"mcp-server-time")=>Provider::None,Some("tavily-mcp")=>Provider::Tavily,Some("@modelcontextprotocol/server-github")=>Provider::Github,_=>{let url=record["config"]["connection"]["url"].as_str().and_then(|s|url::Url::parse(s).ok());match url.as_ref().and_then(url::Url::host_str){Some("mcp.linear.app")=>Provider::Linear,Some("mcp.tavily.com")=>Provider::Tavily,Some("api.githubcopilot.com")=>Provider::Github,_ if record["config"]["kind"]=="remote"=>Provider::Remote,_=>Provider::Unknown}}}}
fn checked_linear_endpoint(value:&str)->Result<String,ManagementError>{let u=url::Url::parse(value).map_err(|_|auth_required())?;if u.scheme()!="https"||!matches!(u.host_str(),Some("mcp.linear.app"|"linear.app"|"api.linear.app"))||!u.username().is_empty()||u.password().is_some()||u.fragment().is_some(){return Err(auth_required());}Ok(value.to_owned())}
async fn bounded_json(mut response:reqwest::Response)->Result<Value,ManagementError>{if !response.status().is_success()||response.content_length().is_some_and(|n|n>1024*1024){return Err(auth_required());}let mut bytes=vec![];while let Some(chunk)=response.chunk().await.map_err(|_|auth_required())?{if bytes.len()+chunk.len()>1024*1024{return Err(auth_required());}bytes.extend_from_slice(&chunk);}serde_json::from_slice(&bytes).map_err(|_|auth_required())}
fn oauth_document(token:&Value,provider:Provider,client:&str,endpoint:&str,resource:Option<&str>,prior_refresh:Option<&str>)->Result<Value,ManagementError>{
    let access=token["access_token"].as_str().filter(|s|!s.is_empty()&&s.len()<=65536&&!s.contains(['\r','\n','\0'])).ok_or_else(auth_required)?;
    if !token["token_type"].as_str().is_some_and(|s|s.eq_ignore_ascii_case("bearer")){return Err(auth_required());}
    let returned_refresh=token["refresh_token"].as_str().filter(|s|!s.is_empty()&&s.len()<=65536);
    let refresh=if provider==Provider::Railway{Some(returned_refresh.ok_or_else(auth_required)?)}else{returned_refresh.or(prior_refresh)};
    let expiry=token["expires_in"].as_u64().filter(|n|*n>0).and_then(|n|now().checked_add(n));
    if provider==Provider::Railway&&expiry.is_none(){return Err(auth_required());}
    Ok(json!({"provider":provider.name(),"access_token":access,"refresh_token":refresh,"expires_at":expiry,
        "client_id":client,"token_endpoint":endpoint,"resource":resource,"verified":false}))
}
fn now()->u64{SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0)}
fn manual_hashes(document:&Value)->BTreeMap<String,String>{document["manual"].as_object().map(|o|o.iter().filter_map(|(name,v)|v.as_str().map(|s|(name.clone(),format!("{:x}",Sha256::digest(s.as_bytes()))))).collect()).unwrap_or_default()}
fn pending_manual(live:&Live,config:&Value)->bool{if live.applied.is_none(){return false;}live.oauth_restart_pending || config["environment"].as_array().into_iter().flatten().chain(config["connection"]["headers"].as_array().into_iter().flatten()).filter_map(|v|v["credential_ref"].as_str()).any(|name|live.current_manual_hashes.get(name)!=live.applied_manual_hashes.get(name))}
fn assigned_manual_references(record:&Value)->Vec<String>{
    // These are existing named service adapters, not inferred authentication for
    // arbitrary environment values or unknown provider headers.
    if !matches!(provider_kind(record),Provider::Tavily|Provider::Linear|Provider::Github){return vec![];}
    if record["config"]["kind"]=="remote"{return record["config"]["connection"]["headers"].as_array().into_iter().flatten().filter(|v|v["name"].as_str().is_some_and(|s|s.eq_ignore_ascii_case("Authorization"))).filter_map(|v|v["credential_ref"].as_str().map(str::to_owned)).collect::<BTreeSet<_>>().into_iter().collect();}
    let variables:&[&str]=match provider_kind(record){Provider::Tavily=>&["TAVILY_API_KEY"],Provider::Linear=>&["LINEAR_API_KEY"],Provider::Github=>&["GITHUB_PERSONAL_ACCESS_TOKEN","GITHUB_TOKEN"],_=>&[]};
    record["config"]["environment"].as_array().into_iter().flatten().filter(|v|v["name"].as_str().is_some_and(|s|variables.contains(&s))).filter_map(|v|v["credential_ref"].as_str().map(str::to_owned)).collect::<BTreeSet<_>>().into_iter().collect()
}
fn remote_headers(config:&Value,document:&Value)->Result<BTreeMap<String,String>,ManagementError>{
    let oauth=document["oauth"]["access_token"].as_str();let mut headers=BTreeMap::new();
    for binding in config["connection"]["headers"].as_array().ok_or_else(invalid)?{let name=binding["name"].as_str().ok_or_else(invalid)?;if oauth.is_some()&&name.eq_ignore_ascii_case("Authorization"){continue;}if reserved_header(name){return Err(invalid_config());}let reference=binding["credential_ref"].as_str().ok_or_else(invalid)?;let value=document["manual"][reference].as_str().ok_or_else(auth_required)?;if value.contains(['\r','\n','\0']){return Err(invalid_credential());}headers.insert(name.to_owned(),value.to_owned());}
    if let Some(token)=oauth{if document["oauth"]["expires_at"].as_u64().is_some_and(|t|t<=now()){return Err(auth_required());}headers.insert("Authorization".into(),format!("Bearer {token}"));}Ok(headers)
}
fn reserved_header(name:&str)->bool{let lower=name.to_ascii_lowercase();["host","content-type","content-length","accept","connection","origin","cookie","transfer-encoding","te","trailer","upgrade"].contains(&lower.as_str())||lower.starts_with("mcp-")||lower.starts_with("proxy-")}
fn missing_binding(record:&Value,oauth:bool)->bool{
    record["config"]["environment"].as_array().into_iter().flatten()
        .filter(|v|!(oauth&&provider_kind(record)==Provider::Railway&&v["name"].as_str().is_some_and(|name|["RAILWAY_API_TOKEN","RAILWAY_TOKEN","HOME","PATH","RAILWAY_NO_AUTO_UPDATE","RAILWAY_NO_TELEMETRY"].contains(&name))))
        .chain(record["config"]["connection"]["headers"].as_array().into_iter().flatten().filter(|v|!(oauth&&v["name"].as_str().is_some_and(|s|s.eq_ignore_ascii_case("Authorization")))))
        .filter_map(|v|v["credential_ref"].as_str()).any(|name|!record["credentials"].as_array().is_some_and(|a|a.iter().any(|entry|entry["name"]==name&&entry["status"]=="stored")))
}
