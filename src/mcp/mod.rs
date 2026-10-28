//! The single client-facing Streamable HTTP endpoint, with native MCP envelopes.
mod management;
use crate::{authority::{Authorizer,AuthorizationError,Caller},runner::{Runner,RunError,InvocationContext,CancelToken,RelayEvent,raw,catalog,LEGACY,MODERN}};
use axum::{Router,body::to_bytes,extract::{Request,State},http::{HeaderMap,HeaderValue,StatusCode,header},response::{IntoResponse,Response,Sse,sse::{Event,KeepAlive}},routing::post};
use serde::Deserialize;
use serde_json::{Value,json,value::RawValue};
use std::{collections::BTreeMap,convert::Infallible,future::Future,pin::Pin,sync::Arc,time::Duration};
use tokio::sync::{Mutex,mpsc,oneshot};
use tokio_stream::wrappers::ReceiverStream;

pub trait ManagementBackend:Send+Sync{
    fn invoke<'a>(&'a self,name:&'a str,arguments:Box<RawValue>,caller:Caller,view:&'a str)->Pin<Box<dyn Future<Output=Result<Value,RunError>>+Send+'a>>;
}
struct Session{owner:String,capabilities:Value,initialized:bool,cancel:CancelToken}
struct ControlReply{owner:String,reply:oneshot::Sender<Result<Box<RawValue>,()>>}
pub struct McpGateway{
    runner:Arc<Runner>,management:Arc<dyn ManagementBackend>,authority:Arc<dyn Authorizer>,origin:String,
    sessions:Mutex<BTreeMap<String,Session>>,
    controls:Mutex<BTreeMap<String,ControlReply>>,
    calls:Mutex<BTreeMap<(String,String),CancelToken>>,
    active_authority:Mutex<BTreeMap<(String,String),HeaderMap>>,
    known_resources:Mutex<BTreeMap<String,Vec<String>>>,
}
#[derive(Deserialize)]
struct Message{jsonrpc:String,#[serde(default)]id:Option<Box<RawValue>>,#[serde(default)]method:Option<String>,#[serde(default)]params:Option<Box<RawValue>>,#[serde(default)]result:Option<Box<RawValue>>,#[serde(default)]error:Option<Box<RawValue>>}

impl McpGateway{
    pub fn new(runner:Arc<Runner>,management:Arc<dyn ManagementBackend>,authority:Arc<dyn Authorizer>,origin:String)->Arc<Self>{Arc::new(Self{runner,management,authority,origin,sessions:Mutex::new(BTreeMap::new()),controls:Mutex::new(BTreeMap::new()),calls:Mutex::new(BTreeMap::new()),active_authority:Mutex::new(BTreeMap::new()),known_resources:Mutex::new(BTreeMap::new())})}
    pub fn router(self:Arc<Self>)->Router{Router::new().route("/mcp",post(endpoint).get(endpoint).delete(endpoint)).with_state(self)}
    pub async fn revoke_owner(&self,owner:&str){
        let mut calls=self.calls.lock().await;calls.retain(|(caller,_),cancel|{if caller==owner{cancel.cancel();false}else{true}});drop(calls);
        self.active_authority.lock().await.retain(|(caller,_),_|caller!=owner);
        self.sessions.lock().await.retain(|_,session|{if session.owner==owner{session.cancel.cancel();false}else{true}});
        self.controls.lock().await.retain(|_,control|control.owner!=owner);
        self.runner.revoke_owner(owner).await;
    }
    pub async fn recheck_authority(&self){
        let entries=self.active_authority.lock().await.clone();
        for ((owner,id),headers) in entries{
            if self.caller(&headers).ok().is_none_or(|caller|caller.owner_id()!=owner){
                if let Some(cancel)=self.calls.lock().await.get(&(owner.clone(),id)){cancel.cancel();}
                self.runner.revoke_owner(&owner).await;
            }
        }
    }
    fn caller(&self,headers:&HeaderMap)->Result<Caller,Response>{
        match self.authority.authorize(headers){
            Ok(caller)=>Ok(caller),
            Err(AuthorizationError::Unauthenticated)=>{
                let mut response=StatusCode::UNAUTHORIZED.into_response();
                let challenge=format!("Bearer resource_metadata=\"{}/.well-known/oauth-protected-resource/mcp\"",self.origin);
                if let Ok(value)=HeaderValue::from_str(&challenge){response.headers_mut().insert(header::WWW_AUTHENTICATE,value);}Err(response)
            },
            Err(AuthorizationError::Unavailable)=>Err(StatusCode::SERVICE_UNAVAILABLE.into_response()),
        }
    }
    async fn handle(self:&Arc<Self>,request:Request)->Response{
        let (parts,body)=request.into_parts();
        if parts.uri.query().is_some(){return protocol_error(StatusCode::BAD_REQUEST,None,-32600,"The MCP endpoint does not use query parameters.");}
        let authority=self.origin.strip_prefix("https://").unwrap_or("");
        let hosts=parts.headers.get_all(header::HOST).iter().count();
        let valid_host=if hosts==0&&parts.version==axum::http::Version::HTTP_2{parts.uri.authority().is_some_and(|value|value.as_str()==authority)}else{hosts==1&&parts.headers.get(header::HOST).and_then(|v|v.to_str().ok())==Some(authority)};
        if !valid_host||parts.uri.authority().is_some_and(|value|value.as_str()!=authority){return StatusCode::BAD_REQUEST.into_response();}
        if parts.headers.get_all(header::ORIGIN).iter().count()>1||parts.headers.get(header::ORIGIN).is_some_and(|v|v.to_str().ok()!=Some(self.origin.as_str())){return StatusCode::FORBIDDEN.into_response();}
        let caller=match self.caller(&parts.headers){Ok(caller)=>caller,Err(response)=>return response};
        let version=headers_text(&parts.headers,"mcp-protocol-version");
        if parts.method!=axum::http::Method::POST{
            if version.as_deref()==Some(MODERN){return StatusCode::METHOD_NOT_ALLOWED.into_response();}
            let Some(session)=headers_text(&parts.headers,"mcp-session-id")else{return StatusCode::BAD_REQUEST.into_response();};
            let sessions=self.sessions.lock().await;
            if !sessions.get(&session).is_some_and(|s|s.owner==caller.owner_id()&&s.initialized){return StatusCode::NOT_FOUND.into_response();}
            let session_cancel=sessions.get(&session).map(|session|session.cancel.clone());
            drop(sessions);
            if parts.method==axum::http::Method::DELETE{if let Some(session)=self.sessions.lock().await.remove(&session){session.cancel.cancel();}return StatusCode::NO_CONTENT.into_response();}
            if !accepts_sse(&parts.headers){return StatusCode::NOT_ACCEPTABLE.into_response();}
            return self.notifications(caller,parts.headers,None,json!({"toolsListChanged":true,"resourcesListChanged":true,"promptsListChanged":true}),false,session_cancel).await;
        }
        if parts.headers.get(header::CONTENT_TYPE).and_then(|v|v.to_str().ok()).is_none_or(|value|value.split(';').next().unwrap_or("").trim()!="application/json"){return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();}
        if !accepts_json(&parts.headers)||!accepts_sse(&parts.headers){return StatusCode::NOT_ACCEPTABLE.into_response();}
        let bytes=match to_bytes(body,16*1024*1024).await{Ok(bytes)=>bytes,Err(_)=>return StatusCode::PAYLOAD_TOO_LARGE.into_response()};
        let message=match raw::parse(&bytes).and_then(|v|serde_json::from_str::<Message>(v.get()).map_err(|_|())){Ok(message)if message.jsonrpc=="2.0"=>message,_=>return protocol_error(StatusCode::BAD_REQUEST,None,-32700,"Expected one JSON-RPC message without duplicate object keys.")};
        let params=message.params.unwrap_or_else(||raw::raw(&json!({})));
        if raw::object(&params).is_err(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32602,"Parameters must be an object.");}
        let method=message.method.as_deref().unwrap_or("");
        let revision=match version.as_deref(){Some(MODERN)=>MODERN,Some(LEGACY)=>LEGACY,None if method=="initialize"=>LEGACY,_=>return unsupported_version(message.id.as_deref())};
        if revision==MODERN{
            let meta=raw::field(&params,"_meta").and_then(|v|raw::scalar(&v).ok()).unwrap_or(Value::Null);
            if meta["io.modelcontextprotocol/protocolVersion"]!=MODERN||!meta["io.modelcontextprotocol/clientCapabilities"].is_object(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32020,"Required protocol metadata is missing or does not match the version header.");}
            if headers_text(&parts.headers,"mcp-method").as_deref()!=Some(method){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32020,"The method header does not match the request.");}
            if matches!(method,"tools/call"|"resources/read"|"prompts/get"){
                let key=if method=="resources/read"{"uri"}else{"name"};
                let name=raw::text(&params,key);
                if headers_text(&parts.headers,"mcp-name").and_then(|v|catalog::decode_header(&v))!=name||name.is_none(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32020,"The name header does not match the request.");}
            }
            if message.method.is_none(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32600,"Modern HTTP messages cannot be standalone responses.");}
        }
        if method=="initialize"{
            if revision!=LEGACY||message.id.is_none(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32600,"Initialize requires the legacy request flow.");}
            let requested=raw::text(&params,"protocolVersion");
            if requested.as_deref()!=Some(LEGACY){return unsupported_version(message.id.as_deref());}
            let caps=raw::field(&params,"capabilities").and_then(|v|raw::scalar(&v).ok()).unwrap_or_else(||json!({}));
            if !caps.is_object(){return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32602,"Capabilities must be an object.");}
            let mut sessions=self.sessions.lock().await;if sessions.len()>=256{return StatusCode::TOO_MANY_REQUESTS.into_response();}
            let session=format!("{:032x}{:032x}",rand::random::<u128>(),rand::random::<u128>());
            sessions.insert(session.clone(),Session{owner:caller.owner_id().to_owned(),capabilities:caps,initialized:false,cancel:CancelToken::new()});
            let mut response=json_response(message.id.as_deref().expect("id"),raw::raw(&json!({"protocolVersion":LEGACY,"capabilities":server_capabilities(),"serverInfo":{"name":"Fetch","version":env!("CARGO_PKG_VERSION")}})));
            response.headers_mut().insert("mcp-session-id",HeaderValue::from_str(&session).expect("portable ID"));return response;
        }
        let capabilities=if revision==LEGACY{
            let Some(session_id)=headers_text(&parts.headers,"mcp-session-id")else{return StatusCode::BAD_REQUEST.into_response();};
            let mut sessions=self.sessions.lock().await;let Some(session)=sessions.get_mut(&session_id).filter(|session|session.owner==caller.owner_id())else{return StatusCode::NOT_FOUND.into_response();};
            if method=="notifications/initialized"&&message.id.is_none(){session.initialized=true;return StatusCode::ACCEPTED.into_response();}
            if !session.initialized{return protocol_error(StatusCode::BAD_REQUEST,message.id.as_deref(),-32600,"Complete initialization before invoking methods.");}
            session.capabilities.clone()
        }else{raw::field(&params,"_meta").and_then(|v|raw::scalar(&v).ok()).map(|v|v["io.modelcontextprotocol/clientCapabilities"].clone()).unwrap_or_else(||json!({}))};
        if message.method.is_none(){
            if let Some(id)=message.id{
                let key=id.get().to_owned();let mut controls=self.controls.lock().await;
                if controls.get(&key).is_some_and(|control|control.owner==caller.owner_id()){
                    if let Some(control)=controls.remove(&key){let _=control.reply.send(message.result.ok_or(()));return StatusCode::ACCEPTED.into_response();}
                }
            }
            return protocol_error(StatusCode::BAD_REQUEST,None,-32600,"No matching owned client request exists.");
        }
        if message.id.is_none(){
            if method=="notifications/cancelled"&&revision==LEGACY{
                if let Some(id)=raw::field(&params,"requestId"){if let Some(cancel)=self.calls.lock().await.get(&(caller.owner_id().to_owned(),id.get().to_owned())){cancel.cancel();}}
                return StatusCode::ACCEPTED.into_response();
            }
            return protocol_error(StatusCode::BAD_REQUEST,None,-32601,"This notification is not supported.");
        }
        let id=message.id.expect("checked ID");
        if !(id.get().starts_with('"')||id.get().parse::<serde_json::Number>().is_ok()){return protocol_error(StatusCode::BAD_REQUEST,Some(&id),-32600,"Request ID must be a string or number.");}
        if method=="subscriptions/listen"&&revision==MODERN{
            if !caller.has_scope("fetch:catalog"){return StatusCode::FORBIDDEN.into_response();}
            let filter=raw::field(&params,"notifications").and_then(|v|raw::scalar(&v).ok()).unwrap_or_else(||json!({}));
            if filter.get("resourceSubscriptions").is_some(){return protocol_error(StatusCode::BAD_REQUEST,Some(&id),-32602,"Live resource content subscriptions are deferred.");}
            return self.notifications(caller,parts.headers,Some(id),filter,true,None).await;
        }
        if method=="tools/call"||matches!(method,"resources/read"|"prompts/get"|"completion/complete"){
            let required=if method=="tools/call"{raw::text(&params,"name").and_then(|name|management::scope(&name)).unwrap_or("fetch:tools:call")}else{"fetch:content:read"};
            if !caller.has_scope(required){return StatusCode::FORBIDDEN.into_response();}
            if revision==MODERN&&method=="tools/call"{
                if let Err(error)=self.validate_mirrored(&parts.headers,&params).await{return protocol_error(StatusCode::BAD_REQUEST,Some(&id),-32020,error.message);}
            }
            return self.operation_stream(caller,parts.headers,id,method.to_owned(),params,capabilities,revision.to_owned()).await;
        }
        let value=match method{
            "ping"=>Ok(raw::raw(&json!({}))),
            "server/discover" if revision==MODERN=>Ok(raw::raw(&json!({"resultType":"complete","ttlMs":0,"cacheScope":"private","supportedVersions":[LEGACY,MODERN],"capabilities":server_capabilities(),"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"Fetch","version":env!("CARGO_PKG_VERSION")}}}))),
            "tools/list"=>{if !caller.has_scope("fetch:catalog")&&!caller.has_scope("fetch:manage"){return StatusCode::FORBIDDEN.into_response();}self.list_tools(&caller,revision).await},
            "resources/list"|"resources/templates/list"|"prompts/list"=>{if !caller.has_scope("fetch:catalog"){return StatusCode::FORBIDDEN.into_response();}self.list_content(&caller,method,revision).await},
            _=>return protocol_error(StatusCode::NOT_FOUND,Some(&id),-32601,"This MCP method is not implemented."),
        };
        match value{Ok(value)=>match frame(value,revision,method,false){Ok(value)=>json_response(&id,value),Err(error)=>protocol_error(StatusCode::BAD_GATEWAY,Some(&id),-32603,error.message)},Err(error)=>protocol_error(StatusCode::BAD_GATEWAY,Some(&id),-32603,error.message)}
    }

    async fn list_tools(&self,caller:&Caller,revision:&str)->Result<Box<RawValue>,RunError>{
        let mut definitions=vec![];let mut issues=vec![];
        if caller.has_scope("fetch:manage"){definitions.extend(management::definitions().into_iter().filter(|d|management::scope(d["name"].as_str().unwrap_or("")).is_some_and(|scope|caller.has_scope(scope))).map(|d|raw::raw(&d)));}
        else if caller.has_scope("fetch:catalog"){definitions.extend(management::definitions().into_iter().filter(|d|d["name"]=="fetch__list_tools").map(|d|raw::raw(&d)));}
        if caller.has_scope("fetch:catalog"){
            for server in self.runner.server_ids().await{
                if !caller.permits("fetch:catalog",&server,None){continue;}
                match self.runner.catalog(&server,revision).await{
                    Ok(origin)=>{
                        if origin["status"]!="available"{issues.push(json!({"server_id":server,"status":origin["status"],"error":origin["error"],"rejected_tools":origin["rejected_tools"]}));}
                        for definition in origin["tools"].as_array().ok_or_else(RunError::catalog)?{
                            let original=definition["name"].as_str().ok_or_else(RunError::catalog)?;
                            if !caller.permits("fetch:catalog",&server,Some(original)){continue;}
                            let alias=self.runner.alias(&server,original).await?;
                            definitions.push(raw::change(&raw::raw(definition),"name",raw::raw(&json!(alias))).map_err(|_|RunError::catalog())?);
                        }
                    },
                    Err(error)=>issues.push(json!({"server_id":server,"status":"unavailable","error":{"code":error.code,"message":error.message},"rejected_tools":[]})),
                }
            }
        }
        Ok(raw::raw(&json!({"tools":definitions,"_meta":{"fetch/catalog":{"view":revision,"complete":issues.is_empty(),"issues":issues}}})))
    }
    async fn list_content(&self,caller:&Caller,method:&str,revision:&str)->Result<Box<RawValue>,RunError>{
        let key=match method{"resources/list"=>"resources","resources/templates/list"=>"resourceTemplates",_=>"prompts"};
        let mut definitions=vec![];let mut issues=vec![];let mut ids=std::collections::BTreeSet::new();
        for server in self.runner.server_ids().await{
            if !caller.permits("fetch:catalog",&server,None){continue;}
            match self.runner.list_native(&server,method,key).await{
                Ok(items)=>{
                    let mut own=vec![];let mut valid=true;let mut observed=vec![];
                    for item in items{
                        let selector=if method=="prompts/list"{"name"}else if method=="resources/templates/list"{"uriTemplate"}else{"uri"};
                        let Some(original)=raw::text(&item,selector)else{valid=false;break;};
                        if !caller.permits("fetch:catalog",&server,Some(&original)){continue;}
                        if !ids.insert((server.clone(),original.clone())){valid=false;break;}
                        let exported=if method=="prompts/list"{self.runner.alias(&server,&original).await?}else{catalog::qualified_uri(&server,&original)};
                        own.push(raw::change(&item,selector,raw::raw(&json!(exported))).map_err(|_|RunError::catalog())?);
                        if method=="resources/list"{observed.push(original);}
                    }
                    if valid{
                        definitions.extend(own);
                        for uri in observed{let mut known=self.known_resources.lock().await;let origins=known.entry(uri).or_default();if !origins.contains(&server){origins.push(server.clone());}}
                    }else{issues.push(json!({"server_id":server,"code":"CATALOG_UNAVAILABLE","message":"The source catalog contains conflicting or unusable selectors."}));}
                },
                Err(error)=>issues.push(json!({"server_id":server,"code":error.code,"message":error.message})),
            }
        }
        let meta_key=if method=="prompts/list"{"fetch/prompts"}else{"fetch/resources"};
        let mut result=json!({"_meta":{}});
        result[key]=serde_json::to_value(definitions).map_err(|_|RunError::catalog())?;
        result["_meta"][meta_key]=json!({"complete":issues.is_empty(),"issues":issues});
        if revision==MODERN{result["resultType"]=json!("complete");result["ttlMs"]=json!(0);result["cacheScope"]=json!("private");}
        Ok(raw::raw(&result))
    }
    async fn validate_mirrored(&self,headers:&HeaderMap,params:&RawValue)->Result<(),RunError>{
        let Some(name)=raw::text(params,"name")else{return Err(RunError::invalid());};
        if name.starts_with("fetch__"){return Ok(());}
        let Some((server,original))=self.runner.resolve_alias(&name).await else{return Ok(());};
        let args=raw::field(params,"arguments").unwrap_or_else(||raw::raw(&json!({})));
        // Uses already observed raw metadata; never performs tools/list on a call.
        for tool in self.runner.known_tools(&server).await?{
            if raw::text(&tool,"name").as_deref()==Some(original.as_str()){
                for (name,value) in catalog::mirrored_headers(&tool,&args)?{
                    if headers.get_all(name.as_str()).iter().count()!=1||headers.get(name.as_str()).and_then(|v|v.to_str().ok()).and_then(catalog::decode_header)!=catalog::decode_header(&value){return Err(RunError::invalid());}
                }
                return Ok(());
            }
        }
        Err(RunError::unsupported(false))
    }

    async fn operation_stream(self:&Arc<Self>,caller:Caller,headers:HeaderMap,id:Box<RawValue>,method:String,params:Box<RawValue>,capabilities:Value,revision:String)->Response{
        let (events,event_rx)=mpsc::channel(64);let (wire,wire_rx)=mpsc::channel::<Result<Event,Infallible>>(64);
        let cancel=CancelToken::new();let key=(caller.owner_id().to_owned(),id.get().to_owned());
        {let mut calls=self.calls.lock().await;if calls.contains_key(&key)||calls.len()>=256{return protocol_error(StatusCode::CONFLICT,Some(&id),-32600,"This request ID is already active or capacity is full.");}calls.insert(key.clone(),cancel.clone());}
        self.active_authority.lock().await.insert(key.clone(),headers.clone());
        let session_cancel=if revision==LEGACY{if let Some(session)=headers_text(&headers,"mcp-session-id"){self.sessions.lock().await.get(&session).map(|session|session.cancel.clone()).unwrap_or_default()}else{CancelToken::new()}}else{CancelToken::new()};
        let gateway=self.clone();let owner=caller.owner_id().to_owned();let authority=self.authority.clone();let owner_check=owner.clone();let auth_headers=headers.clone();
        let required=if method=="tools/call"{raw::text(&params,"name").and_then(|name|management::scope(&name)).unwrap_or("fetch:tools:call")}else{"fetch:content:read"};
        let context=InvocationContext{owner_id:owner.clone(),capabilities,client_revision:Some(revision.clone()),progress_token:raw::field(&params,"_meta").and_then(|meta|raw::field(&meta,"progressToken")),events:Some(events),cancel:cancel.clone(),still_authorized:Some(Arc::new(move||authority.authorize(&auth_headers).ok().is_some_and(|caller|caller.owner_id()==owner_check&&caller.has_scope(required))))};
        let authorization=context.still_authorized.clone();let authority_cancel=cancel.clone();let runner_for_revoke=self.runner.clone();let revoked_owner=context.owner_id.clone();
        let auth_monitor=tokio::spawn(async move{let mut interval=tokio::time::interval(Duration::from_secs(1));loop{tokio::select!{_=session_cancel.cancelled()=>{authority_cancel.cancel();break;},_=interval.tick()=>{if authorization.as_ref().is_some_and(|check|!check()){authority_cancel.cancel();runner_for_revoke.revoke_owner(&revoked_owner).await;break;}}}}});
        let relay_gateway=self.clone();let relay_wire=wire.clone();let relay_cancel=cancel.clone();let relay_revision=revision.clone();
        let relay=tokio::spawn(async move{
            let mut events=event_rx;
            while let Some(event)=events.recv().await{
                if relay_cancel.is_cancelled()||relay_gateway.caller(&headers).is_err(){break;}
                match event{
                    RelayEvent::Notification(message)=>{if relay_wire.send(Ok(Event::default().event("message").data(message.get()))).await.is_err(){break;}},
                    RelayEvent::Request{method,params,reply,..}=>{
                        if relay_revision!=LEGACY{let _=reply.send(Err(()));continue;}
                        let control_id=format!("fetch-client-{:032x}",rand::random::<u128>());let raw_id=raw::raw(&json!(control_id));
                        let mut controls=relay_gateway.controls.lock().await;if controls.len()>=256{let _=reply.send(Err(()));continue;}
                        controls.insert(raw_id.get().to_owned(),ControlReply{owner:owner.clone(),reply});drop(controls);
                        let message=format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"method\":{},\"params\":{}}}",raw_id.get(),serde_json::to_string(&method).expect("method"),params.get());
                        if relay_wire.send(Ok(Event::default().event("message").data(message))).await.is_err(){break;}
                    }
                }
            }
        });
        let cancel_for_stream=cancel.clone();let modern=revision==MODERN;
        tokio::spawn(async move{
            let result=gateway.operation(caller,&method,params,context,&revision).await;
            if !cancel.is_cancelled(){
                let message=match result{
                    Ok(result)=>match frame(result,&revision,&method,true){Ok(result)=>format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",id.get(),result.get()),Err(error)=>rpc_error_text(Some(&id),-32603,error.message)},
                    Err(error)=>{
                        if error.code=="SERVER_NOT_FOUND"||error.code=="INVALID_REQUEST"{rpc_error_text(Some(&id),-32602,error.message)}else if error.code=="TOOL_RPC_ERROR"||error.code=="MCP_VERSION_UNSUPPORTED"||method!="tools/call"{rpc_error_text(Some(&id),-32603,error.message)}else{
                            let result=raw::raw(&json!({"content":[{"type":"text","text":error.message}],"isError":true}));
                            let result=frame(result,&revision,&method,true).unwrap_or_else(|_|raw::raw(&json!({})));
                            format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",id.get(),result.get())
                        }
                    }
                };
                let _=wire.send(Ok(Event::default().event("message").data(message))).await;
            }
            relay.abort();auth_monitor.abort();gateway.calls.lock().await.remove(&key);
            gateway.active_authority.lock().await.remove(&key);
            gateway.controls.lock().await.retain(|_,control|control.owner!=key.0||!control.reply.is_closed());
        });
        let stream=OwnedStream{inner:ReceiverStream::new(wire_rx),cancel:cancel_for_stream,modern,completed:false};
        let mut response=Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15))).into_response();
        response.headers_mut().insert("x-accel-buffering",HeaderValue::from_static("no"));response
    }
    async fn operation(&self,caller:Caller,method:&str,params:Box<RawValue>,mut context:InvocationContext,revision:&str)->Result<Box<RawValue>,RunError>{
        if method=="tools/call"{
            let name=raw::text(&params,"name").ok_or_else(RunError::invalid)?;
            let arguments=raw::field(&params,"arguments").unwrap_or_else(||raw::raw(&json!({})));
            if let Some(scope)=management::scope(&name){
                if !caller.has_scope(scope){return Err(RunError::missing());}
                if raw::object(&arguments).is_err(){return Err(RunError::invalid());}
                let result=self.management.invoke(&name,arguments,caller,revision).await?;
                return Ok(raw::raw(&json!({"content":[{"type":"text","text":serde_json::to_string(&result).map_err(|_|RunError::invalid())?}],"isError":result["status"]=="failed"})));
            }
            let(server,original)=self.runner.resolve_alias(&name).await.ok_or_else(RunError::missing)?;
            if !caller.permits("fetch:tools:call",&server,Some(&original)){return Err(RunError::missing());}
            if !self.runner.native_tool_usable(&server,&original,revision).await?{return Err(RunError::missing());}
            let mut fields=raw::object(&params).map_err(|_|RunError::invalid())?;fields.insert("name".into(),raw::raw(&json!(original)));
            let result=self.runner.invoke(&server,method,serde_json::value::to_raw_value(&fields).map_err(|_|RunError::invalid())?,context).await?;
            return adapt_result(result,revision,&self.runner.source_revision(&server).await?,method);
        }
        let mut fields=raw::object(&params).map_err(|_|RunError::invalid())?;
        let server=match method{
            "resources/read"=>{
                let uri=raw::text(&params,"uri").ok_or_else(RunError::invalid)?;let(server,original)=self.resolve_resource(&caller,&uri).await?;
                fields.insert("uri".into(),raw::raw(&json!(original)));server
            },
            "prompts/get"=>{
                let name=raw::text(&params,"name").ok_or_else(RunError::invalid)?;
                let(server,original)=self.runner.resolve_alias(&name).await.ok_or_else(RunError::missing)?;
                if !caller.permits("fetch:content:read",&server,Some(&original)){return Err(RunError::missing());}
                if let Some(arguments)=fields.get("arguments"){if raw::object(arguments).map_err(|_|RunError::invalid())?.values().any(|v|serde_json::from_str::<String>(v.get()).is_err()){return Err(RunError::invalid());}}
                fields.insert("name".into(),raw::raw(&json!(original)));server
            },
            "completion/complete"=>{
                let reference=fields.get("ref").ok_or_else(RunError::invalid)?;let mut reference=raw::object(reference).map_err(|_|RunError::invalid())?;
                let kind=reference.get("type").and_then(|v|serde_json::from_str::<String>(v.get()).ok()).ok_or_else(RunError::invalid)?;
                let server=if kind=="ref/prompt"{let name=reference.get("name").and_then(|v|serde_json::from_str::<String>(v.get()).ok()).ok_or_else(RunError::invalid)?;let(server,original)=self.runner.resolve_alias(&name).await.ok_or_else(RunError::missing)?;if !caller.permits("fetch:content:read",&server,Some(&original)){return Err(RunError::missing());}reference.insert("name".into(),raw::raw(&json!(original)));server}
                    else if kind=="ref/resource"{let uri=reference.get("uri").and_then(|v|serde_json::from_str::<String>(v.get()).ok()).ok_or_else(RunError::invalid)?;let(server,original)=self.resolve_resource(&caller,&uri).await?;reference.insert("uri".into(),raw::raw(&json!(original)));server}else{return Err(RunError::invalid());};
                fields.insert("ref".into(),serde_json::value::to_raw_value(&reference).map_err(|_|RunError::invalid())?);server
            },
            _=>return Err(RunError::unsupported(false)),
        };
        // Content operations keep the server's configured duration. They share
        // the same finite admission controls and never borrow per-tool overrides.
        context.owner_id=caller.owner_id().to_owned();
        let result=self.runner.invoke(&server,method,serde_json::value::to_raw_value(&fields).map_err(|_|RunError::invalid())?,context).await?;
        if method=="resources/read"{
            if let Some(contents)=raw::field(&result,"contents").and_then(|v|serde_json::from_str::<Vec<Value>>(v.get()).ok()){
                for content in contents{if let Some(uri)=content["uri"].as_str(){let mut known=self.known_resources.lock().await;let origins=known.entry(uri.to_owned()).or_default();if !origins.contains(&server){origins.push(server.clone());}}}
            }
        }
        adapt_result(result,revision,&self.runner.source_revision(&server).await?,method)
    }
    async fn resolve_resource(&self,caller:&Caller,uri:&str)->Result<(String,String),RunError>{
        for server in self.runner.server_ids().await{if let Some(original)=catalog::unqualify_uri(&server,uri){if caller.permits("fetch:content:read",&server,Some(original)){return Ok((server,original.to_owned()));}return Err(RunError::missing());}}
        let known=self.known_resources.lock().await;
        if let Some(origins)=known.get(uri){let origins:Vec<_>=origins.iter().filter(|server|caller.permits("fetch:content:read",server,Some(uri))).collect();if origins.len()==1{return Ok((origins[0].clone(),uri.to_owned()));}}
        Err(RunError{http_status:400,code:"RESOURCE_SELECTION_REQUIRED",message:"The resource's source is unknown or ambiguous. Use a qualified URI from the authorized resource catalog.",execution:"not_started"})
    }

    async fn notifications(self:&Arc<Self>,caller:Caller,headers:HeaderMap,id:Option<Box<RawValue>>,filter:Value,modern:bool,session_cancel:Option<CancelToken>)->Response{
        let (wire,rx)=mpsc::channel::<Result<Event,Infallible>>(64);let mut changes=self.runner.changes.subscribe();let gateway=self.clone();
        tokio::spawn(async move{
            let mut accepted=json!({});for key in ["toolsListChanged","resourcesListChanged","promptsListChanged"]{if filter[key]==true{accepted[key]=json!(true);}}
            if modern{if let Some(id)=&id{let notice=json!({"jsonrpc":"2.0","method":"notifications/subscriptions/acknowledged","params":{"_meta":{"io.modelcontextprotocol/subscriptionId":raw::scalar(id).unwrap_or(Value::Null)},"notifications":accepted}});if wire.send(Ok(Event::default().event("message").data(notice.to_string()))).await.is_err(){return;}}}
            let mut authority_interval=tokio::time::interval(Duration::from_secs(2));
            let session_cancel=session_cancel.unwrap_or_default();
            loop{
                tokio::select!{
                    _=wire.closed()=>break,
                    _=session_cancel.cancelled()=>break,
                    _=authority_interval.tick()=>{if gateway.caller(&headers).ok().is_none_or(|current|current.owner_id()!=caller.owner_id()||!current.has_scope("fetch:catalog")){break;}},
                    event=changes.recv()=>{
                        let (_,method)=match event{Ok(event)=>event,Err(tokio::sync::broadcast::error::RecvError::Lagged(_))=>continue,Err(_)=>break};
                        if session_cancel.is_cancelled()||gateway.caller(&headers).ok().is_none_or(|current|current.owner_id()!=caller.owner_id()||!current.has_scope("fetch:catalog")){break;}
                        let key=match method.as_str(){"notifications/tools/list_changed"=>"toolsListChanged","notifications/resources/list_changed"=>"resourcesListChanged","notifications/prompts/list_changed"=>"promptsListChanged",_=>continue};
                        if accepted[key]!=true{continue;}
                        let params=if modern{json!({"_meta":{"io.modelcontextprotocol/subscriptionId":id.as_deref().and_then(|v|raw::scalar(v).ok())}})}else{json!({})};
                        let notice=json!({"jsonrpc":"2.0","method":method,"params":params});
                        if wire.send(Ok(Event::default().event("message").data(notice.to_string()))).await.is_err(){break;}
                    }
                }
            }
        });
        Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::new().interval(Duration::from_secs(15))).into_response()
    }
}

async fn endpoint(State(gateway):State<Arc<McpGateway>>,request:Request)->Response{
    let mut response=gateway.handle(request).await;
    response.headers_mut().insert(header::CACHE_CONTROL,HeaderValue::from_static("no-store"));response.headers_mut().insert(header::X_CONTENT_TYPE_OPTIONS,HeaderValue::from_static("nosniff"));response
}
fn headers_text(headers:&HeaderMap,name:&str)->Option<String>{if headers.get_all(name).iter().count()!=1{return None;}headers.get(name)?.to_str().ok().map(str::to_owned)}
fn accepts_json(headers:&HeaderMap)->bool{headers.get(header::ACCEPT).and_then(|v|v.to_str().ok()).is_some_and(|v|v.split(',').any(|item|item.trim().split(';').next()==Some("application/json")))}
fn accepts_sse(headers:&HeaderMap)->bool{headers.get(header::ACCEPT).and_then(|v|v.to_str().ok()).is_some_and(|v|v.split(',').any(|item|item.trim().split(';').next()==Some("text/event-stream")))}
fn server_capabilities()->Value{json!({"tools":{"listChanged":true},"resources":{"listChanged":true},"prompts":{"listChanged":true},"completions":{}})}
fn json_response(id:&RawValue,result:Box<RawValue>)->Response{let text=format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",id.get(),result.get());([(header::CONTENT_TYPE,"application/json")],text).into_response()}
fn rpc_error_text(id:Option<&RawValue>,code:i64,message:&str)->String{format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":{code},\"message\":{}}}}}",id.map(RawValue::get).unwrap_or("null"),serde_json::to_string(message).expect("safe message"))}
fn protocol_error(status:StatusCode,id:Option<&RawValue>,code:i64,message:&str)->Response{(status,[(header::CONTENT_TYPE,"application/json")],rpc_error_text(id,code,message)).into_response()}
fn unsupported_version(id:Option<&RawValue>)->Response{let result=format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32022,\"message\":\"Unsupported protocol version.\",\"data\":{{\"supported\":[\"{LEGACY}\",\"{MODERN}\"]}}}}}}",id.map(RawValue::get).unwrap_or("null"));(StatusCode::BAD_REQUEST,[(header::CONTENT_TYPE,"application/json")],result).into_response()}
fn frame(mut result:Box<RawValue>,revision:&str,method:&str,terminal:bool)->Result<Box<RawValue>,RunError>{
    if revision!=MODERN{return Ok(result);}
    if raw::text(&result,"resultType").as_deref()==Some("input_required"){return Ok(result);}
    if terminal||matches!(method,"tools/list"|"resources/list"|"resources/templates/list"|"prompts/list"|"ping"){
        if let Some(kind)=raw::text(&result,"resultType"){if kind!="complete"{return Err(RunError::unsupported(true));}}
        else{result=raw::change(&result,"resultType",raw::raw(&json!("complete"))).map_err(|_|RunError::unsupported(true))?;}
    }
    if matches!(method,"tools/list"|"resources/list"|"resources/templates/list"|"prompts/list"|"resources/read"){
        result=raw::change(&result,"ttlMs",raw::raw(&json!(0))).map_err(|_|RunError::unsupported(true))?;
        result=raw::change(&result,"cacheScope",raw::raw(&json!("private"))).map_err(|_|RunError::unsupported(true))?;
    }
    Ok(result)
}
fn adapt_result(result:Box<RawValue>,downstream:&str,upstream:&str,method:&str)->Result<Box<RawValue>,RunError>{
    if downstream==MODERN&&upstream==MODERN&&raw::text(&result,"resultType").is_none(){return Err(RunError::unsupported(true));}
    frame(result,downstream,method,true)
}
struct OwnedStream{inner:ReceiverStream<Result<Event,Infallible>>,cancel:CancelToken,modern:bool,completed:bool}
impl futures_util::Stream for OwnedStream{
    type Item=Result<Event,Infallible>;
    fn poll_next(mut self:Pin<&mut Self>,cx:&mut std::task::Context<'_>)->std::task::Poll<Option<Self::Item>>{
        let result=futures_util::Stream::poll_next(Pin::new(&mut self.inner),cx);if matches!(result,std::task::Poll::Ready(None)){self.completed=true;}result
    }
}
impl Drop for OwnedStream{fn drop(&mut self){if self.modern&&!self.completed{self.cancel.cancel();}}}
