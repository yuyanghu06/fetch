//! Shared managed-server execution. HTTP, native MCP and Settings use this runner.
pub mod raw;
pub mod catalog;
mod scheduler;
mod transport;

use scheduler::{Reservation,Scheduler};
use catalog::{Aliases,Tool};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json,value::RawValue};
use std::{collections::BTreeMap,path::PathBuf,sync::{Arc,atomic::{AtomicBool,AtomicU64,Ordering}},time::Duration};
use tokio::sync::{Mutex,RwLock,Notify,broadcast,mpsc,oneshot};
use tokio::time::Instant;
pub use transport::{LEGACY,MODERN};

#[derive(Clone,Serialize,Deserialize)]
pub struct ExecutionPolicy {
    pub max_concurrent_calls:usize,
    pub default_timeout_ms:u64,
    #[serde(default)]
    pub tool_timeouts_ms:BTreeMap<String,u64>,
}
impl Default for ExecutionPolicy {
    fn default()->Self { Self{max_concurrent_calls:1,default_timeout_ms:120_000,tool_timeouts_ms:BTreeMap::new()} }
}
impl ExecutionPolicy {
    pub fn validate(&self)->Result<(),RunError> {
        if !(1..=64).contains(&self.max_concurrent_calls)||!(1..=86_400_000).contains(&self.default_timeout_ms)||self.tool_timeouts_ms.len()>4096||self.tool_timeouts_ms.iter().any(|(name,limit)|name.is_empty()||name.contains('\0')||!(1..=86_400_000).contains(limit)) { Err(RunError::invalid_config()) } else { Ok(()) }
    }
}
#[derive(Clone)]
pub enum TransportConfig {
    Stdio{executable:PathBuf,args:Vec<String>,working_directory:Option<PathBuf>,environment:BTreeMap<String,String>},
    Http{endpoint:String,headers:BTreeMap<String,String>},
    Sse{endpoint:String,headers:BTreeMap<String,String>},
}
#[derive(Clone)]
pub struct ServerRoute { pub server_id:String,pub transport:TransportConfig,pub execution:ExecutionPolicy }

#[derive(Clone,Default)]
pub struct CancelToken(Arc<CancelState>);
#[derive(Default)]
struct CancelState { cancelled:AtomicBool,changed:Notify }
impl CancelToken {
    pub fn new()->Self {Self::default()}
    pub fn cancel(&self) {self.0.cancelled.store(true,Ordering::Release);self.0.changed.notify_waiters();}
    pub fn is_cancelled(&self)->bool {self.0.cancelled.load(Ordering::Acquire)}
    pub async fn cancelled(&self) {loop{let notice=self.0.changed.notified();if self.is_cancelled(){return;}notice.await;}}
}

pub enum RelayEvent {
    Notification(Box<RawValue>),
    Request {id:Box<RawValue>,method:String,params:Box<RawValue>,reply:oneshot::Sender<Result<Box<RawValue>,()>>},
}
#[derive(Clone)]
pub struct InvocationContext {
    pub owner_id:String,
    pub capabilities:Value,
    pub client_revision:Option<String>,
    pub progress_token:Option<Box<RawValue>>,
    pub events:Option<mpsc::Sender<RelayEvent>>,
    pub cancel:CancelToken,
    /// Must check the current grant, not merely its state when the call entered.
    pub still_authorized:Option<Arc<dyn Fn()->bool+Send+Sync>>,
}
impl InvocationContext {
    pub fn terminal(owner_id:String)->Self {Self{owner_id,capabilities:json!({}),client_revision:None,progress_token:None,events:None,cancel:CancelToken::new(),still_authorized:None}}
    fn authorized(&self)->bool {self.still_authorized.as_ref().is_none_or(|check|check())}
}
pub fn interaction_supported(capabilities:&Value,method:&str,params:&RawValue)->bool {
    match method {
        "roots/list"=>capabilities.get("roots").is_some(),
        "sampling/createMessage"=>capabilities.get("sampling").is_some(),
        "elicitation/create"=>{
            let mode=raw::text(params,"mode").unwrap_or_else(||"form".into());
            capabilities.get("elicitation").and_then(|v|v.get(&mode)).is_some()
        },
        _=>false,
    }
}

#[derive(Clone,Debug)]
pub struct RunError {pub http_status:u16,pub code:&'static str,pub message:&'static str,pub execution:&'static str}
impl std::fmt::Display for RunError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str(self.message)}}
impl std::error::Error for RunError {}
impl RunError {
    pub fn invalid()->Self{Self{http_status:400,code:"INVALID_REQUEST",message:"The request does not have the required shape.",execution:"not_started"}}
    pub fn invalid_config()->Self{Self{http_status:422,code:"INVALID_CONFIG",message:"Execution settings must use finite supported positive limits.",execution:"not_started"}}
    pub fn missing()->Self{Self{http_status:404,code:"SERVER_NOT_FOUND",message:"The selected server was not found.",execution:"not_started"}}
    pub fn busy()->Self{Self{http_status:409,code:"SERVER_BUSY",message:"The server is restarting or its waiting queue is full.",execution:"not_started"}}
    pub fn not_ready()->Self{Self{http_status:503,code:"SERVER_NOT_READY",message:"The MCP server is not ready. Inspect its saved status before choosing a remedy.",execution:"not_started"}}
    pub fn unavailable()->Self{Self{http_status:502,code:"TOOL_CALL_UNAVAILABLE",message:"The upstream connection ended without a usable terminal result. Review whether the action happened before retrying.",execution:"unknown"}}
    pub fn rpc()->Self{Self{http_status:502,code:"TOOL_RPC_ERROR",message:"The upstream returned a protocol error. Review the action before retrying.",execution:"unknown"}}
    pub fn timeout()->Self{Self{http_status:504,code:"TOOL_CALL_TIMEOUT",message:"The execution deadline ended. The action may still be running; review its outcome before retrying.",execution:"unknown"}}
    pub fn queue_timeout()->Self{Self{http_status:504,code:"TOOL_CALL_TIMEOUT",message:"The queue waiting deadline ended before the call was sent.",execution:"not_started"}}
    pub fn cancelled(sent:bool)->Self{Self{http_status:409,code:"TOOL_CALL_UNAVAILABLE",message:"The owning request ended. A sent action may still be running.",execution:if sent{"unknown"}else{"not_started"}}}
    pub fn unsupported(sent:bool)->Self{Self{http_status:if sent{502}else{409},code:"TOOL_INTERACTION_UNSUPPORTED",message:"This operation needs a protocol feature or owning client context unavailable on this connection.",execution:if sent{"unknown"}else{"not_started"}}}
    pub fn version()->Self{Self{http_status:502,code:"MCP_VERSION_UNSUPPORTED",message:"The upstream requested a different supported protocol setup.",execution:"not_started"}}
    pub fn provider_auth()->Self{Self{http_status:502,code:"PROVIDER_AUTH_REQUIRED",message:"The upstream rejected its assigned provider access. Complete provider setup in Fetch Settings.",execution:"unknown"}}
    pub fn catalog()->Self{Self{http_status:502,code:"TOOLS_UNAVAILABLE",message:"A complete unambiguous upstream catalog could not be collected.",execution:"not_started"}}
    pub fn next_steps(&self,server_id:&str)->Value {
        json!([{"actor":if self.execution=="unknown"{"user"}else{"agent"},"action":if self.execution=="unknown"{"review_tool_outcome"}else{"check_server_status"},"server_id":server_id,"instructions":self.message}])
    }
}

struct Server {
    route:RwLock<ServerRoute>,
    connection:RwLock<Option<Arc<transport::Connection>>>,
    generation:AtomicU64,
    lifecycle:Mutex<()>,
    scheduler:Arc<Scheduler>,
    tools:RwLock<Vec<Tool>>,
    catalog_observed:AtomicBool,
    catalog_observed_at:RwLock<Option<String>>,
    runtime:RwLock<String>,
    unresolved_prior:AtomicU64,
}
struct Continuation {
    owner_id:String,server_id:String,generation:u64,method:String,
    original_params:Box<RawValue>,upstream_state:Option<Box<RawValue>>,
    expected_keys:Vec<String>,deadline:Instant,reservation:Reservation,
}
#[derive(Clone)]
pub struct Runner {
    servers:Arc<RwLock<BTreeMap<String,Arc<Server>>>>,
    aliases:Arc<Mutex<Aliases>>,
    alias_path:Option<PathBuf>,
    continuations:Arc<Mutex<BTreeMap<String,Continuation>>>,
    stopping:Arc<AtomicBool>,
    shutdown_lock:Arc<Mutex<()>>,
    invocation_sequence:Arc<AtomicU64>,
    invocations:Arc<Mutex<BTreeMap<u64,(String,CancelToken)>>>,
    pub changes:broadcast::Sender<(String,String)>,
}
impl Runner {
    pub fn new()->Arc<Self>{Self::construct(None,Aliases::default())}
    /// Call only during explicit application startup, with Fetch's private root.
    pub fn persistent(root:PathBuf)->Result<Arc<Self>,RunError>{
        let path=root.join("aliases.json");
        let aliases=match std::fs::symlink_metadata(&path){
            Ok(metadata)=>{
                #[cfg(unix)]
                {use std::os::unix::fs::{MetadataExt,PermissionsExt};if !metadata.is_file()||metadata.file_type().is_symlink()||metadata.nlink()!=1||metadata.permissions().mode()&0o077!=0{return Err(RunError::catalog());}}
                if metadata.len()>8*1024*1024{return Err(RunError::catalog());}
                let bytes=std::fs::read(&path).map_err(|_|RunError::catalog())?;
                serde_json::from_slice(&bytes).map_err(|_|RunError::catalog())?
            },
            Err(error) if error.kind()==std::io::ErrorKind::NotFound=>Aliases::default(),
            Err(_)=>return Err(RunError::catalog()),
        };
        Ok(Self::construct(Some(path),aliases))
    }
    fn construct(alias_path:Option<PathBuf>,aliases:Aliases)->Arc<Self>{let(changes,_)=broadcast::channel(256);Arc::new(Self{servers:Arc::new(RwLock::new(BTreeMap::new())),aliases:Arc::new(Mutex::new(aliases)),alias_path,continuations:Arc::new(Mutex::new(BTreeMap::new())),stopping:Arc::new(AtomicBool::new(false)),shutdown_lock:Arc::new(Mutex::new(())),invocation_sequence:Arc::new(AtomicU64::new(0)),invocations:Arc::new(Mutex::new(BTreeMap::new())),changes})}
    fn ensure_running(&self)->Result<(),RunError>{if self.stopping.load(Ordering::Acquire){Err(RunError::not_ready())}else{Ok(())}}
    pub async fn configure(&self,route:ServerRoute)->Result<(),RunError>{
        self.ensure_running()?;
        route.execution.validate()?;
        if route.server_id.is_empty()||route.server_id.contains('\0'){return Err(RunError::invalid());}
        let mut servers=self.servers.write().await;
        self.ensure_running()?;
        if let Some(server)=servers.get(&route.server_id){server.scheduler.update(route.execution.clone());*server.route.write().await=route;}
        else{servers.insert(route.server_id.clone(),Arc::new(Server{scheduler:Scheduler::new(route.execution.clone()),route:RwLock::new(route),connection:RwLock::new(None),generation:AtomicU64::new(0),lifecycle:Mutex::new(()),tools:RwLock::new(vec![]),catalog_observed:AtomicBool::new(false),catalog_observed_at:RwLock::new(None),runtime:RwLock::new("stopped".into()),unresolved_prior:AtomicU64::new(0)}));}
        Ok(())
    }
    async fn server(&self,id:&str)->Result<Arc<Server>,RunError>{self.servers.read().await.get(id).cloned().ok_or_else(RunError::missing)}
    pub async fn server_ids(&self)->Vec<String>{self.servers.read().await.keys().cloned().collect()}
    pub async fn connect(&self,id:&str)->Result<(),RunError>{
        self.ensure_running()?;
        let server=self.server(id).await?;let _lock=server.lifecycle.lock().await;
        self.ensure_running()?;
        if server.connection.read().await.as_ref().is_some_and(|c|!c.closed.load(Ordering::Acquire)){return Ok(());}
        let generation=server.generation.fetch_add(1,Ordering::AcqRel)+1;
        server.catalog_observed.store(false,Ordering::Release);
        if let Some(previous)=server.connection.write().await.take(){let local=previous.is_local();let confirmed=previous.close().await;if local&&confirmed{server.scheduler.confirmed_reset();}else if local{return Err(RunError::unavailable());}}
        *server.runtime.write().await="starting".into();
        let route=server.route.read().await.clone();
        let connection=match transport::Connection::open(&route.transport).await{Ok(c)=>c,Err(e)=>{*server.runtime.write().await="failed".into();return Err(e);}};
        let mut preparing=transport::CloseOnDrop::new(connection.clone());
        if let Err(error)=connection.setup().await{let _=connection.close().await;*server.runtime.write().await="failed".into();return Err(error);}
        let capabilities=connection.capabilities.read().await.clone();
        let modern_http=connection.modern_http()&&connection.revision.read().await.as_str()==MODERN;
        let tools=if capabilities.get("tools").is_some(){
            match collect(&connection,"tools/list","tools").await.and_then(|definitions|catalog::validate_tools(definitions,modern_http)){
                Ok(tools)=>tools,Err(error)=>{let _=connection.close().await;*server.runtime.write().await="failed".into();return Err(error);}
            }
        }else{vec![]};
        if self.stopping.load(Ordering::Acquire){let _=connection.close().await;*server.runtime.write().await="stopped".into();return Err(RunError::not_ready());}
        *server.tools.write().await=tools;
        *server.catalog_observed_at.write().await=Some(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis,true));
        server.catalog_observed.store(true,Ordering::Release);
        *server.connection.write().await=Some(connection.clone());
        *server.runtime.write().await="ready".into();server.scheduler.ready(true);
        let mut events=connection.changes.subscribe();let changes=self.changes.clone();let id=id.to_owned();
        tokio::spawn(async move{loop{match events.recv().await{Ok(method)=>{let _=changes.send((id.clone(),method));},Err(broadcast::error::RecvError::Lagged(_))=>continue,Err(_)=>break}}});
        let monitored=server.clone();let c=connection.clone();
        tokio::spawn(async move{
            while monitored.generation.load(Ordering::Acquire)==generation{
                if c.closed.load(Ordering::Acquire){
                    monitored.scheduler.ready(false);*monitored.runtime.write().await="failed".into();
                    if c.is_local()&&c.confirmed_child_exit().await{monitored.scheduler.confirmed_reset();break;}
                    if !c.is_local(){break;}
                }
                tokio::time::sleep(Duration::from_millis(250)).await;
            }
        });
        let _=self.changes.send((route.server_id,"notifications/tools/list_changed".into()));
        preparing.disarm();
        Ok(())
    }
    pub async fn disconnect(&self,id:&str,force:bool)->Result<(),RunError>{
        let server=self.server(id).await?;let _lock=server.lifecycle.lock().await;
        server.generation.fetch_add(1,Ordering::AcqRel);
        server.catalog_observed.store(false,Ordering::Release);
        server.scheduler.ready(false);
        if let Some(connection)=server.connection.write().await.take(){
            let local=connection.is_local();let confirmed=connection.close().await;
            if confirmed{server.scheduler.confirmed_reset();}
            else if !local&&force{let count=server.scheduler.confirmed_reset();server.unresolved_prior.fetch_add(count as u64,Ordering::AcqRel);}
            else if local{return Err(RunError::unavailable());}
        }
        *server.runtime.write().await="stopped".into();
        self.continuations.lock().await.retain(|_,flow|flow.server_id!=id);
        Ok(())
    }
    pub async fn update_execution(&self,id:&str,policy:ExecutionPolicy)->Result<(),RunError>{self.ensure_running()?;policy.validate()?;let server=self.server(id).await?;self.ensure_running()?;server.scheduler.update(policy.clone());server.route.write().await.execution=policy;Ok(())}
    pub async fn update_remote_headers(&self,id:&str,headers:BTreeMap<String,String>)->Result<(),RunError>{
        self.ensure_running()?;
        let server=self.server(id).await?;
        if let Some(connection)=server.connection.read().await.as_ref(){connection.update_headers(headers.clone()).await?;}
        match &mut server.route.write().await.transport{TransportConfig::Http{headers:stored,..}|TransportConfig::Sse{headers:stored,..}=>*stored=headers,_=>return Err(RunError::invalid())}Ok(())
    }
    pub async fn begin_restart(&self,id:&str)->Result<(),RunError>{self.ensure_running()?;self.server(id).await?.scheduler.begin_restart()}
    pub async fn reopen_admission(&self,id:&str){if self.ensure_running().is_err(){return;}if let Ok(server)=self.server(id).await{if self.ensure_running().is_ok(){server.scheduler.reopen();}}}
    pub async fn drain(&self,id:&str,budget:Duration)->bool{match self.server(id).await{Ok(server)=>server.scheduler.drain(budget).await,Err(_)=>true}}
    pub async fn snapshot(&self,id:&str)->Value{
        match self.server(id).await{Ok(server)=>{
            let closed=server.connection.read().await.as_ref().is_some_and(|c|c.closed.load(Ordering::Acquire));
            if closed{server.scheduler.ready(false);}
            json!({"runtime":{"status":if closed{"failed".into()}else{server.runtime.read().await.clone()},"error":if closed{json!({"code":"SERVER_CONNECTION_LOST","message":"The upstream connection ended."})}else{Value::Null}},"tool_execution":server.scheduler.snapshot(),"unresolved_prior_calls":server.unresolved_prior.load(Ordering::Acquire)})
        },Err(_)=>json!({"runtime":{"status":"stopped","error":null},"tool_execution":{"active_calls":0,"queued_calls":0,"uncertain_calls":0,"max_concurrent_calls":1,"max_queued_calls":64,"max_queued_bytes":16777216,"queue_wait_timeout_ms":300000},"unresolved_prior_calls":0})}
    }
    pub async fn source_revision(&self,id:&str)->Result<String,RunError>{let connection=self.connection(id).await?;let revision=connection.revision.read().await.clone();Ok(revision)}
    pub async fn known_tools(&self,id:&str)->Result<Vec<Box<RawValue>>,RunError>{let server=self.server(id).await?;let tools=server.tools.read().await.iter().filter(|tool|!tool.source_rejected).map(|tool|tool.definition.clone()).collect();Ok(tools)}
    pub async fn native_tool_usable(&self,id:&str,name:&str,view:&str)->Result<bool,RunError>{let server=self.server(id).await?;let usable=server.tools.read().await.iter().any(|tool|tool.name==name&&!tool.source_rejected&&!(view==MODERN&&tool.header_error));Ok(usable)}
    async fn connection(&self,id:&str)->Result<Arc<transport::Connection>,RunError>{self.ensure_running()?;let server=self.server(id).await?;let connection=server.connection.read().await.clone().ok_or_else(RunError::not_ready)?;if connection.closed.load(Ordering::Acquire){server.scheduler.ready(false);return Err(RunError::not_ready());}Ok(connection)}
    /// Pure projection of the last successfully completed acquisition. This never
    /// contacts an upstream, changes observation time or persists a feature profile.
    pub async fn cached_catalog(&self,id:&str,view:&str)->Result<Value,RunError>{
        if !matches!(view,"raw"|LEGACY|MODERN){return Err(RunError::invalid());}
        let server=self.server(id).await?;
        if !server.catalog_observed.load(Ordering::Acquire){return Err(RunError::catalog());}
        let tools=server.tools.read().await;if !server.catalog_observed.load(Ordering::Acquire){return Err(RunError::catalog());}let mut origin=catalog::origin(id,&tools,view);
        origin["observed_at"]=server.catalog_observed_at.read().await.clone().map(Value::String).unwrap_or(Value::Null);
        Ok(origin)
    }
    pub async fn catalog(&self,id:&str,view:&str)->Result<Value,RunError>{
        if !matches!(view,"raw"|LEGACY|MODERN){return Err(RunError::invalid());}
        let server=self.server(id).await?;let connection=self.connection(id).await?;
        let generation=server.generation.load(Ordering::Acquire);
        let tools=if connection.capabilities.read().await.get("tools").is_some(){catalog::validate_tools(collect(&connection,"tools/list","tools").await?,connection.modern_http()&&connection.revision.read().await.as_str()==MODERN)?}else{vec![]};
        if self.stopping.load(Ordering::Acquire)||server.generation.load(Ordering::Acquire)!=generation||connection.closed.load(Ordering::Acquire){return Err(RunError::catalog());}
        let origin=catalog::origin(id,&tools,view);let mut stored=server.tools.write().await;
        if self.stopping.load(Ordering::Acquire)||server.generation.load(Ordering::Acquire)!=generation||connection.closed.load(Ordering::Acquire){return Err(RunError::catalog());}
        *stored=tools;*server.catalog_observed_at.write().await=origin["observed_at"].as_str().map(str::to_owned);server.catalog_observed.store(true,Ordering::Release);Ok(origin)
    }
    pub async fn tool_definitions(&self,id:&str,view:&str)->Result<Vec<Box<RawValue>>,RunError>{
        let origin=self.catalog(id,view).await?;
        let tools=origin["tools"].as_array().ok_or_else(RunError::catalog)?;
        let mut aliases=self.aliases.lock().await;let mut result=vec![];
        for tool in tools{let name=tool["name"].as_str().ok_or_else(RunError::catalog)?;let alias=aliases.assign(id,name)?;let definition=raw::change(&raw::raw(tool),"name",raw::raw(&json!(alias))).map_err(|_|RunError::catalog())?;result.push(definition);}
        self.persist_aliases(&aliases)?;Ok(result)
    }
    pub async fn alias(&self,id:&str,name:&str)->Result<String,RunError>{let mut aliases=self.aliases.lock().await;let alias=aliases.assign(id,name)?;self.persist_aliases(&aliases)?;Ok(alias)}
    pub async fn resolve_alias(&self,alias:&str)->Option<(String,String)>{self.aliases.lock().await.resolve(alias)}
    fn persist_aliases(&self,aliases:&Aliases)->Result<(),RunError>{
        let Some(path)=&self.alias_path else{return Ok(());};
        use std::io::Write;
        #[cfg(unix)]use std::os::unix::fs::OpenOptionsExt;
        let temporary=path.with_file_name(format!(".aliases-{:032x}",rand::random::<u128>()));
        let mut options=std::fs::OpenOptions::new();options.write(true).create_new(true);
        #[cfg(unix)]{options.mode(0o600).custom_flags(libc::O_NOFOLLOW|libc::O_CLOEXEC);}
        let result=(||{let mut file=options.open(&temporary).map_err(|_|RunError::catalog())?;file.write_all(&serde_json::to_vec(aliases).map_err(|_|RunError::catalog())?).map_err(|_|RunError::catalog())?;file.sync_all().map_err(|_|RunError::catalog())?;std::fs::rename(&temporary,path).map_err(|_|RunError::catalog())?;if let Some(parent)=path.parent(){std::fs::File::open(parent).and_then(|f|f.sync_all()).map_err(|_|RunError::catalog())?;}Ok(())})();
        if result.is_err(){let _=std::fs::remove_file(temporary);}result
    }
    pub async fn call(&self,id:&str,name:&str,args:Box<RawValue>,context:InvocationContext)->Result<Box<RawValue>,RunError>{
        if raw::object(&args).is_err(){return Err(RunError::invalid());}
        let params=RawValue::from_string(format!("{{\"name\":{},\"arguments\":{}}}",serde_json::to_string(name).expect("name"),args.get())).map_err(|_|RunError::invalid())?;
        self.invoke(id,"tools/call",params,context).await
    }
    pub async fn invoke(&self,id:&str,method:&str,params:Box<RawValue>,context:InvocationContext)->Result<Box<RawValue>,RunError>{
        self.ensure_running()?;
        let mut guard=ActiveGuard{cancel:context.cancel.clone(),armed:true};
        let runner=self.clone();let id=id.to_owned();let method=method.to_owned();
        let invocation=self.invocation_sequence.fetch_add(1,Ordering::AcqRel);
        {let mut invocations=self.invocations.lock().await;self.ensure_running()?;if invocations.len()>=4096{return Err(RunError::busy());}invocations.insert(invocation,(context.owner_id.clone(),context.cancel.clone()));}
        let check=context.still_authorized.clone();let cancelled=context.cancel.clone();
        let authority_monitor=tokio::spawn(async move{if check.is_none(){return;}let mut timer=tokio::time::interval(Duration::from_secs(1));loop{timer.tick().await;if check.as_ref().is_some_and(|check|!check()){cancelled.cancel();break;}}});
        let (reply,result)=oneshot::channel();
        tokio::spawn(async move{let value=runner.invoke_inner(&id,&method,params,context).await;authority_monitor.abort();runner.invocations.lock().await.remove(&invocation);let _=reply.send(value);});
        let value=result.await.unwrap_or_else(|_|Err(RunError::unavailable()));
        guard.armed=false;value
    }
    async fn invoke_inner(&self,id:&str,method:&str,params:Box<RawValue>,context:InvocationContext)->Result<Box<RawValue>,RunError>{
        self.ensure_running()?;
        let server=self.server(id).await?;let connection=self.connection(id).await?;
        let required_capability=match method{"completion/complete"=>Some("completions"),"resources/read"=>Some("resources"),"prompts/get"=>Some("prompts"),_=>None};
        if let Some(capability)=required_capability{if connection.capabilities.read().await.get(capability).is_none(){return Err(RunError::unsupported(false));}}
        if !context.authorized(){return Err(RunError{http_status:403,code:"FORBIDDEN",message:"This connection no longer permits the operation.",execution:"not_started"});}
        let generation=server.generation.load(Ordering::Acquire);
        let mut fields=raw::object(&params).map_err(|_|RunError::invalid())?;
        let client_state=fields.remove("requestState");let responses=fields.remove("inputResponses");fields.remove("_meta");
        let original=serde_json::value::to_raw_value(&fields).map_err(|_|RunError::invalid())?;
        let (reservation,deadline,upstream_params)=if let Some(state)=client_state{
            let handle=serde_json::from_str::<String>(state.get()).map_err(|_|RunError::invalid())?;
            let mut continuations=self.continuations.lock().await;
            let flow=continuations.get(&handle).ok_or_else(||RunError::unsupported(false))?;
            if flow.owner_id!=context.owner_id||flow.server_id!=id||flow.generation!=generation||flow.method!=method||flow.deadline<=Instant::now()||raw::scalar(&flow.original_params)!=raw::scalar(&original){return Err(RunError::unsupported(false));}
            let supplied=responses.as_ref().map(|r|raw::object(r)).transpose().map_err(|_|RunError::invalid())?.unwrap_or_default();
            if supplied.len()!=flow.expected_keys.len()||flow.expected_keys.iter().any(|key|!supplied.contains_key(key)){return Err(RunError::invalid());}
            let flow=continuations.remove(&handle).ok_or_else(||RunError::unsupported(false))?;
            let mut forwarded=raw::object(&flow.original_params).map_err(|_|RunError::invalid())?;
            if let Some(state)=flow.upstream_state{forwarded.insert("requestState".into(),state);}
            if let Some(responses)=responses{forwarded.insert("inputResponses".into(),responses);}
            (flow.reservation,flow.deadline,serde_json::value::to_raw_value(&forwarded).map_err(|_|RunError::invalid())?)
        }else{
            if responses.is_some(){return Err(RunError::invalid());}
            let name=if method=="tools/call"{raw::text(&original,"name")}else{None};
            let timeout=server.scheduler.timeout(name.as_deref()); // captured before queue wait
            let reservation=server.scheduler.admit(params.get().len(),context.cancel.clone()).await?;
            (reservation,Instant::now()+timeout,original.clone())
        };
        if self.stopping.load(Ordering::Acquire)||context.cancel.is_cancelled()||!context.authorized()||server.generation.load(Ordering::Acquire)!=generation{reservation.complete();return Err(RunError::cancelled(false));}
        let mut current_params=upstream_params;
        let reservation=reservation;
        loop{
            self.ensure_running()?;
            let mirrored=if method=="tools/call"&&connection.modern_http()&&connection.revision.read().await.as_str()==MODERN{
                let name=raw::text(&current_params,"name").ok_or_else(RunError::invalid)?;
                let tools=server.tools.read().await;
                let definition=tools.iter().find(|tool|tool.name==name).ok_or_else(||RunError::unsupported(false))?;
                catalog::mirrored_headers(&definition.definition,&raw::field(&current_params,"arguments").unwrap_or_else(||raw::raw(&json!({}))))?
            }else{BTreeMap::new()};
            let request=match connection.start(method,current_params,&context,mirrored).await{Ok(request)=>request,Err(error)=>{reservation.complete();return Err(error);}};
            reservation.mark_forwarded();
            let request_id=request.id.clone();let mut response=request.response;
            let mut guard=ActiveGuard{cancel:context.cancel.clone(),armed:true};
            let result=tokio::select!{
                result=&mut response=>result.unwrap_or_else(|_|Err(RunError::unavailable())),
                _=tokio::time::sleep_until(deadline)=>{
                    reservation.mark_uncertain();connection.cancel(&request_id).await;
                    let local=connection.is_local();let server=server.clone();
                    tokio::spawn(async move{if let Ok(result)=response.await{if terminal_response(&result){reservation.complete();}}else if local&&server.connection.read().await.as_ref().is_some_and(|c|c.closed.load(Ordering::Acquire)){/* exit must be confirmed by lifecycle recovery */}});
                    guard.armed=false;return Err(RunError::timeout());
                },
                _=context.cancel.cancelled()=>{
                    reservation.mark_uncertain();connection.cancel(&request_id).await;
                    tokio::spawn(async move{if let Ok(result)=response.await{if terminal_response(&result){reservation.complete();}}});
                    guard.armed=false;return Err(RunError::cancelled(true));
                },
            };
            guard.armed=false;
            let result=match result{
                Ok(result)=>result,
                Err(error)=>{if error.code=="TOOL_RPC_ERROR"||error.code=="MCP_VERSION_UNSUPPORTED"{reservation.complete();}else{reservation.mark_uncertain();}return Err(error);}
            };
            if raw::text(&result,"resultType").as_deref()==Some("input_required"){
                if context.client_revision.is_none(){reservation.mark_uncertain();return Err(RunError::unsupported(true));}
                let requests=raw::field(&result,"inputRequests").map(|r|raw::object(&r)).transpose().map_err(|_|RunError::unsupported(true))?.unwrap_or_default();
                if requests.is_empty()&&raw::field(&result,"requestState").is_none(){reservation.mark_uncertain();return Err(RunError::unsupported(true));}
                for request in requests.values(){let method=raw::text(request,"method").ok_or_else(||RunError::unsupported(true))?;let params=raw::field(request,"params").unwrap_or_else(||raw::raw(&json!({})));if !interaction_supported(&context.capabilities,&method,&params){reservation.mark_uncertain();return Err(RunError::unsupported(true));}}
                if context.client_revision.as_deref()==Some(LEGACY){
                    let Some(events)=&context.events else{reservation.mark_uncertain();return Err(RunError::unsupported(true));};
                    let mut answers=raw::RawObject::new();
                    for (key,request) in requests{
                        let (reply,receiver)=oneshot::channel();let method=raw::text(&request,"method").ok_or_else(||RunError::unsupported(true))?;let params=raw::field(&request,"params").unwrap_or_else(||raw::raw(&json!({})));
                        events.send(RelayEvent::Request{id:raw::raw(&json!(key)),method,params,reply}).await.map_err(|_|RunError::unsupported(true))?;
                        let answer=tokio::time::timeout_at(deadline,receiver).await.map_err(|_|RunError::timeout())?.map_err(|_|RunError::unsupported(true))?.map_err(|_|RunError::unsupported(true))?;
                        answers.insert(key,answer);
                    }
                    let mut params=raw::object(&original).map_err(|_|RunError::invalid())?;
                    if let Some(state)=raw::field(&result,"requestState"){params.insert("requestState".into(),state);}
                    params.insert("inputResponses".into(),serde_json::value::to_raw_value(&answers).map_err(|_|RunError::invalid())?);
                    current_params=serde_json::value::to_raw_value(&params).map_err(|_|RunError::invalid())?;
                    continue;
                }
                let handle=format!("fetch-continuation-{:032x}{:032x}",rand::random::<u128>(),rand::random::<u128>());
                let state=Continuation{owner_id:context.owner_id.clone(),server_id:id.to_owned(),generation,method:method.to_owned(),original_params:original.clone(),upstream_state:raw::field(&result,"requestState"),expected_keys:requests.keys().cloned().collect(),deadline,reservation};
                let mut flows=self.continuations.lock().await;
                // Drop expired relay state without treating uncertain reservations as completed.
                flows.retain(|_,flow|{if flow.deadline<=Instant::now(){flow.reservation.mark_uncertain();false}else{true}});
                if flows.len()>=256{state.reservation.mark_uncertain();return Err(RunError::unsupported(true));}
                flows.insert(handle.clone(),state);
                let expiry_flows=self.continuations.clone();let expiry_handle=handle.clone();
                tokio::spawn(async move{tokio::time::sleep_until(deadline).await;if let Some(flow)=expiry_flows.lock().await.remove(&expiry_handle){flow.reservation.mark_uncertain();}});
                return raw::change(&result,"requestState",raw::raw(&json!(handle))).map_err(|_|RunError::unsupported(true));
            }
            if connection.revision.read().await.as_str()==MODERN&&raw::text(&result,"resultType").is_none(){reservation.mark_uncertain();return Err(RunError::unsupported(true));}
            if !terminal(&result){reservation.mark_uncertain();return Err(RunError::unsupported(true));}
            reservation.complete();if !context.authorized()||context.cancel.is_cancelled(){return Err(RunError::cancelled(true));}return Ok(result);
        }
    }
    pub async fn list_native(&self,id:&str,method:&str,key:&str)->Result<Vec<Box<RawValue>>,RunError>{
        let connection=self.connection(id).await?;let capability=if method.starts_with("resources/"){"resources"}else{"prompts"};
        if connection.capabilities.read().await.get(capability).is_none(){return Ok(vec![]);}
        collect(&connection,method,key).await
    }
    pub async fn revoke_owner(&self,owner:&str){
        for (_, (caller,cancel)) in self.invocations.lock().await.iter(){if caller==owner{cancel.cancel();}}
        let mut flows=self.continuations.lock().await;flows.retain(|_,flow|{if flow.owner_id==owner{flow.reservation.mark_uncertain();false}else{true}});
    }
    pub async fn shutdown(&self){
        let _shutdown=self.shutdown_lock.lock().await;
        self.stopping.store(true,Ordering::Release);
        for (_,(_,cancel)) in self.invocations.lock().await.iter(){cancel.cancel();}
        let ids=self.server_ids().await;
        for id in &ids{if let Ok(server)=self.server(id).await{server.scheduler.ready(false);}}
        for id in ids{let _=self.disconnect(&id,true).await;}
    }
}
struct ActiveGuard{cancel:CancelToken,armed:bool}
impl Drop for ActiveGuard{fn drop(&mut self){if self.armed{self.cancel.cancel();}}}
fn terminal(result:&RawValue)->bool{matches!(raw::text(result,"resultType").as_deref(),None|Some("complete"))}
fn terminal_response(result:&Result<Box<RawValue>,RunError>)->bool{match result{Ok(result)=>terminal(result),Err(error)=>matches!(error.code,"TOOL_RPC_ERROR"|"MCP_VERSION_UNSUPPORTED")}}
async fn collect(connection:&Arc<transport::Connection>,method:&str,key:&str)->Result<Vec<Box<RawValue>>,RunError>{
    let deadline=Instant::now()+Duration::from_secs(10);let mut cursor:Option<Box<RawValue>>=None;let mut seen=std::collections::BTreeSet::new();let mut all=vec![];let mut bytes=0usize;
    for _ in 0..1024{
        let params=if let Some(cursor)=&cursor{RawValue::from_string(format!("{{\"cursor\":{}}}",cursor.get())).map_err(|_|RunError::catalog())?}else{raw::raw(&json!({}))};
        let request=connection.start(method,params,&InvocationContext::terminal("fetch-catalog".into()),BTreeMap::new()).await?;
        let result=tokio::time::timeout_at(deadline,request.response).await.map_err(|_|RunError::catalog())?.map_err(|_|RunError::catalog())??;
        if raw::text(&result,"resultType").is_some_and(|v|v!="complete"){return Err(RunError::catalog());}
        let definitions=raw::field(&result,key).ok_or_else(RunError::catalog)?;
        let definitions:Vec<Box<RawValue>>=serde_json::from_str(definitions.get()).map_err(|_|RunError::catalog())?;
        for definition in definitions{bytes=bytes.saturating_add(definition.get().len());if bytes>64*1024*1024||all.len()>=4096{return Err(RunError::catalog());}all.push(definition);}
        match raw::field(&result,"nextCursor"){
            None=>return Ok(all),
            Some(next)=>{let value=serde_json::from_str::<String>(next.get()).map_err(|_|RunError::catalog())?;if !seen.insert(value){return Err(RunError::catalog());}cursor=Some(next);}
        }
    }
    Err(RunError::catalog())
}
