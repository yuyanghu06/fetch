//! HTTP adapters for the selected management and installed-tool operations.
use super::{AppState, application_error, authorize_scope, empty_body, parse_query};
use crate::{authority::Caller, manager::{Manager,ManagementError}, runner::{InvocationContext,RunError,Runner}};
use axum::{Json,body::to_bytes,extract::{Path,Request,State},http::{HeaderMap,HeaderValue,StatusCode,header},response::{Html,IntoResponse,Response}};
use serde_json::{Value,json,value::RawValue};
use std::sync::Arc;

fn manager(state:&AppState)->Result<Arc<Manager>,Response>{state.manager.clone().ok_or_else(||application_error(StatusCode::SERVICE_UNAVAILABLE,"SERVICE_UNAVAILABLE","Start Fetch's native application to use this operation.",None))}
fn management_error(error:ManagementError)->Response{(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response()}
fn accepted(body:Value,location:Option<String>)->Response{let mut response=(StatusCode::ACCEPTED,Json(body)).into_response();if let Some(location)=location{if let Ok(value)=HeaderValue::from_str(&location){response.headers_mut().insert(header::LOCATION,value);}}response}
fn no_query(request:&Request)->bool{request.uri().query().is_none()}
async fn body(request:Request)->Result<Value,Response>{
    if !no_query(&request)||!request.headers().get(header::CONTENT_TYPE).and_then(|v|v.to_str().ok()).is_some_and(|v|v.split(';').next().is_some_and(|v|v.trim().eq_ignore_ascii_case("application/json"))){return Err(invalid());}
    let bytes=to_bytes(request.into_body(),crate::json::MAX_REQUEST_BYTES).await.map_err(|_|invalid())?;
    let raw=crate::json::object(&bytes).map_err(|_|invalid())?;
    serde_json::from_str(raw.get()).map_err(|_|invalid())
}
fn invalid()->Response{application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Use the documented request fields and one JSON object.",None)}
fn target(caller:&Caller,scope:&str,id:&str,item:Option<&str>)->Result<(),Response>{if id.is_empty()||id.contains('\0'){return Err(invalid());}if !caller.permits(scope,id,item){return Err(application_error(StatusCode::NOT_FOUND,"SERVER_NOT_FOUND","The selected server was not found.",None));}Ok(())}

pub async fn install(State(state):State<Arc<AppState>>,request:Request)->Response{
    if let Err(reply)=authorize_scope(&state,request.headers(),"fetch:manage",None){return *reply;}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};let input=match body(request).await{Ok(v)=>v,Err(r)=>return r};
    match manager.install(input).await{Ok(value)=>{let location=value["installation_id"].as_str().map(|id|format!("/api/v1/setup/installations/{}",crate::json::path_component(id)));accepted(value,location)},Err(e)=>management_error(e)}
}
pub async fn installation(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    if let Err(reply)=authorize_scope(&state,request.headers(),"fetch:manage",None){return *reply;}
    if !no_query(&request)||id.is_empty()||id.contains('\0')||empty_body(request).await.is_err(){return invalid();}
    match manager(&state){Ok(manager)=>match manager.installation(&id).await{Ok(v)=>Json(v).into_response(),Err(e)=>management_error(e)},Err(r)=>r}
}
pub async fn config(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:manage",None){Ok(v)=>v,Err(r)=>return *r};if let Err(r)=target(&caller,"fetch:manage",&id,None){return r;}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};let input=match body(request).await{Ok(v)=>v,Err(r)=>return r};
    match manager.config(&id,input,&caller).await{Ok(v)=>Json(v).into_response(),Err(e)=>management_error(e)}
}
pub async fn authenticate(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:manage",None){Ok(v)=>v,Err(r)=>return *r};if let Err(r)=target(&caller,"fetch:manage",&id,None){return r;}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};let input=match body(request).await{Ok(v)=>v,Err(r)=>return r};if input!=json!({}){return invalid();}
    match manager.authenticate(&id).await{Ok(v)=>{if v["status"]=="authenticating"{accepted(v,Some(format!("/api/v1/servers?{}",url::form_urlencoded::Serializer::new(String::new()).append_pair("id",&id).finish())))}else{Json(v).into_response()}},Err(e)=>management_error(e)}
}
pub async fn restart(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:manage",None){Ok(v)=>v,Err(r)=>return *r};if let Err(r)=target(&caller,"fetch:manage",&id,None){return r;}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};let input=match body(request).await{Ok(v)=>v,Err(r)=>return r};
    match manager.restart(&id,input).await{Ok(v)=>{let location=v["restart_id"].as_str().map(|attempt|format!("/api/v1/servers?{}",url::form_urlencoded::Serializer::new(String::new()).append_pair("id",&id).append_pair("restart_id",attempt).finish()));accepted(v,location)},Err(e)=>management_error(e)}
}
pub async fn credentials(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    let Some(native)=&state.native else{return application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Open Fetch's native Settings window to change credentials.",None);};
    // A normal OAuth token cannot reach parsing, OS confirmation or the credential store.
    let Some(caller)=native.authorize_settings_headers(request.headers()) else{return application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Manual credential changes require Fetch's native Settings window.",None);};
    if let Err(r)=target(&caller,"fetch:manage",&id,None){return r;}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};let headers=request.headers().clone();let input=match body(request).await{Ok(v)=>v,Err(r)=>return r};
    let fingerprint=match manager.credential_fingerprint(&id,&input).await{Ok(v)=>v,Err(e)=>return management_error(e)};
    let cap=match native.confirm_settings_headers(&headers,&id,&fingerprint).await{Ok(v)=>v,Err(e)=>{use crate::native_auth::NativeAuthError::*;let(status,code)=match e{Denied|Expired=>(StatusCode::FORBIDDEN,"HUMAN_CONFIRMATION_DENIED"),Timeout=>(StatusCode::REQUEST_TIMEOUT,"HUMAN_CONFIRMATION_TIMEOUT"),Unavailable=>(StatusCode::SERVICE_UNAVAILABLE,"HUMAN_CONFIRMATION_UNAVAILABLE")};return application_error(status,code,&e.to_string(),None);}};
    match manager.save_credentials(&id,input,native,&cap).await{Ok(v)=>Json(v).into_response(),Err(e)=>management_error(e)}
}
pub async fn provider_callback(State(state):State<Arc<AppState>>,request:Request)->Response{
    let query=match parse_query(request.uri().query().unwrap_or("")){Ok(v)=>v,Err(_)=>return invalid()};
    if empty_body(request).await.is_err(){return invalid();}
    let(mut flow,mut code,mut error)=(None,None,None);for(key,value)in query{match key.as_str(){"state" if flow.is_none()=>flow=Some(value),"code" if code.is_none()=>code=Some(value),"error" if error.is_none()=>error=Some(value),"error_description"|"iss"=>{},_=>return invalid()}}
    let Some(flow)=flow else{return invalid();};let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};
    let (status,message)=match manager.provider_callback(&flow,code.as_deref(),error.as_deref()).await{Ok(())=>(StatusCode::OK,"Provider setup finished. Return to Fetch to review the server's authentication and restart state."),Err(_)=>(StatusCode::BAD_REQUEST,"Provider setup did not finish successfully. Return to Fetch to review the server state before retrying.")};
    let mut response=(status,Html(format!("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><title>Fetch provider setup</title><h1>Fetch provider setup</h1><p>{message}</p></html>"))).into_response();response.headers_mut().insert(header::CONTENT_SECURITY_POLICY,HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'; base-uri 'none'"));response
}

pub async fn tools(State(state):State<Arc<AppState>>,request:Request)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:catalog",None){Ok(v)=>v,Err(r)=>return *r};
    let query=match parse_query(request.uri().query().unwrap_or("")){Ok(v)=>v,Err(_)=>return invalid()};if empty_body(request).await.is_err(){return invalid();}
    let(mut ids,mut view)=(Vec::new(),None);for(key,value)in query{match key.as_str(){"server_id" if !value.trim().is_empty()&&!value.contains('\0')=>{if !ids.contains(&value){ids.push(value);}},"view" if view.is_none()&&matches!(value.as_str(),"raw"|"2025-11-25"|"2026-07-28")=>view=Some(value),_=>return invalid()}}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return r};match list_tools(&manager,&caller,ids,view.as_deref().unwrap_or("raw")).await{Ok(v)=>Json(v).into_response(),Err(e)=>management_error(e)}
}
pub async fn list_tools(manager:&Manager,caller:&Caller,ids:Vec<String>,view:&str)->Result<Value,ManagementError>{
    if !caller.has_scope("fetch:catalog"){return Err(ManagementError::new(403,"FORBIDDEN","Catalog permission is required."));}
    if !matches!(view,"raw"|"2025-11-25"|"2026-07-28"){return Err(ManagementError::new(400,"INVALID_REQUEST","The requested catalog view is unsupported."));}
    let installed=manager.repository.list().map_err(|_|ManagementError::new(500,"TOOL_LIST_UNAVAILABLE","Installed server information is unavailable."))?;
    let selected=if ids.is_empty(){installed.iter().filter_map(|r|r["server_id"].as_str()).filter(|id|caller.permits("fetch:catalog",id,None)).map(str::to_owned).collect()}else{ids};
    let(mut origins,mut missing,mut steps)=(Vec::new(),Vec::new(),Vec::new());
    for id in selected{if !caller.permits("fetch:catalog",&id,None)||!installed.iter().any(|r|r["server_id"].as_str()==Some(id.as_str())){missing.push(id);continue;}
        let mut origin=match manager.runner.catalog(&id,view).await{Ok(v)=>v,Err(e)=>crate::runner::catalog::unavailable_origin(&id,&e)};
        if let Some(tools)=origin["tools"].as_array_mut(){tools.retain(|tool|tool["name"].as_str().is_some_and(|name|caller.permits("fetch:catalog",&id,Some(name))));}
        if let Some(rejected)=origin["rejected_tools"].as_array_mut(){rejected.retain(|tool|tool["name"].as_str().is_some_and(|name|caller.permits("fetch:catalog",&id,Some(name))));}
        if origin["status"]=="partial"&&origin["rejected_tools"].as_array().is_some_and(Vec::is_empty){origin["status"]=json!("available");}
        if origin["status"]!="available"{steps.push(json!({"actor":"agent","action":"check_server_status","server_id":id,"instructions":"Inspect the server's current Fetch status before selecting an authentication, configuration or restart action."}));}
        origins.push(origin);
    }
    let complete=missing.is_empty()&&origins.iter().all(|o|o["status"]=="available");
    Ok(json!({"status":"ok","view":view,"complete":complete,"origins":origins,"not_found_ids":missing,"next_steps":steps}))
}
struct CancelOnDrop(crate::runner::CancelToken);
impl Drop for CancelOnDrop{fn drop(&mut self){self.0.cancel();}}
pub async fn call(State(state):State<Arc<AppState>>,Path((id,name)):Path<(String,String)>,request:Request)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:tools:call",None){Ok(v)=>v,Err(r)=>return call_failure(*r)};
    if let Err(r)=target(&caller,"fetch:tools:call",&id,Some(&name)){return call_failure(r);}
    if name.is_empty()||name.contains('\0')||!no_query(&request){return call_failure(invalid());}
    let manager=match manager(&state){Ok(v)=>v,Err(r)=>return call_failure(r)};
    match manager.repository.get(&id){Ok(Some(_))=>{},Ok(None)=>return call_error(&RunError::missing(),None,None),Err(_)=>return call_error(&RunError{http_status:500,code:"SERVER_INFO_UNAVAILABLE",message:"Saved server information is unavailable.",execution:"not_started"},None,None)}
    let headers=request.headers().clone();
    if !headers.get(header::CONTENT_TYPE).and_then(|v|v.to_str().ok()).is_some_and(|v|v.split(';').next().is_some_and(|v|v.trim().eq_ignore_ascii_case("application/json"))){return call_failure(invalid());}
    let bytes=match to_bytes(request.into_body(),crate::json::MAX_REQUEST_BYTES).await{Ok(v)=>v,Err(_)=>return call_failure(invalid())};let arguments=match crate::json::object(&bytes){Ok(v)=>v,Err(_)=>return call_failure(invalid())};
    let mut context=InvocationContext::terminal(caller.owner_id().into());let authority=state.authority.clone();let target_id=id.clone();let target_name=name.clone();context.still_authorized=Some(Arc::new(move||authority.authorize(&headers).is_ok_and(|current|current.permits("fetch:tools:call",&target_id,Some(&target_name)))));let _cancel=CancelOnDrop(context.cancel.clone());
    match manager.runner.call(&id,&name,arguments,context).await{
        Ok(raw)=>{let error=serde_json::from_str::<Value>(raw.get()).ok().is_some_and(|v|v["isError"]==true);#[derive(serde::Serialize)]struct Completed<'a>{server_id:&'a str,name:&'a str,status:&'static str,execution:&'static str,result:Box<RawValue>,error:Option<Value>,next_steps:Vec<Value>}
            Json(Completed{server_id:&id,name:&name,status:if error{"failed"}else{"succeeded"},execution:"completed",result:raw,error:None,next_steps:vec![]}).into_response()},
        Err(e)=>call_error(&e,Some(&id),Some(&name))
    }
}
fn call_error(error:&RunError,id:Option<&str>,name:Option<&str>)->Response{(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::BAD_GATEWAY),Json(json!({"server_id":id,"name":name,"status":"failed","execution":error.execution,"result":null,"error":{"code":error.code,"message":error.message},"next_steps":error.next_steps(id.unwrap_or(""))}))).into_response()}
fn call_failure(mut response:Response)->Response{
    // Existing authorization and framing failures remain private and not_started.
    // A response mapper supplies the D50 call envelope without reading any target metadata.
    let status=response.status();let headers=response.headers().clone();
    let code=match status{StatusCode::UNAUTHORIZED=>"UNAUTHENTICATED",StatusCode::FORBIDDEN=>"FORBIDDEN",StatusCode::NOT_FOUND=>"SERVER_NOT_FOUND",_=>"INVALID_REQUEST"};
    response=call_error(&RunError{http_status:status.as_u16(),code,message:"The call was not started. Check Fetch access, target and request format.",execution:"not_started"},None,None);
    if let Some(value)=headers.get(header::WWW_AUTHENTICATE){response.headers_mut().insert(header::WWW_AUTHENTICATE,value.clone());}response
}

pub struct ManagementAdapter {pub manager:Arc<Manager>,pub discovery:crate::discovery::DiscoveryService}
impl crate::mcp::ManagementBackend for ManagementAdapter {
    fn invoke<'a>(&'a self,name:&'a str,arguments:Box<RawValue>,caller:Caller,view:&'a str)->std::pin::Pin<Box<dyn std::future::Future<Output=Result<Value,RunError>>+Send+'a>>{
        Box::pin(async move{
            let input:Value=serde_json::from_str(arguments.get()).map_err(|_|RunError::invalid())?;
            if !input.is_object(){return Err(RunError::invalid());}
            let allowed:&[&str]=match name {
                "fetch__discover_servers"=>&["config_path"], "fetch__install_servers"=>&["servers"],
                "fetch__get_installation"=>&["installation_id"], "fetch__get_servers"=>&["id","restart_id"],
                "fetch__list_tools"=>&["server_id","view"], "fetch__authenticate_server"=>&["server_id"],
                "fetch__restart_server"=>&["server_id","force_restart"],
                "fetch__update_server_config"=>&["server_id","args","working_directory","environment","connection","execution"],
                _=>return Err(RunError::invalid()),
            };
            if input.as_object().is_some_and(|object|object.keys().any(|key|!allowed.contains(&key.as_str()))){return Err(RunError::invalid());}
            let project=|result:Result<Value,ManagementError>|Ok(result.unwrap_or_else(|error|error.body()));
            if name=="fetch__list_tools"{let ids=strings(&input,"server_id")?;let view=match input.get("view"){None=>view,Some(value)=>value.as_str().ok_or_else(RunError::invalid)?};return project(list_tools(&self.manager,&caller,ids,view).await);}
            if !caller.can_manage(){return Err(RunError{http_status:403,code:"FORBIDDEN",message:"Server-management permission is required.",execution:"not_started"});}
            let needs_server=matches!(name,"fetch__authenticate_server"|"fetch__restart_server"|"fetch__update_server_config");
            let id=if needs_server{input["server_id"].as_str().filter(|s|!s.trim().is_empty()&&!s.contains('\0')).ok_or_else(RunError::invalid)?}else{""};
            if !id.is_empty()&&!caller.permits("fetch:manage",id,None){return Err(RunError::missing());}
            match name{
                "fetch__discover_servers"=>{let path=input.get("config_path").map(|v|v.as_str().filter(|s|!s.trim().is_empty()&&!s.contains('\0')&&std::path::Path::new(s).is_absolute()).map(std::path::PathBuf::from).ok_or_else(RunError::invalid)).transpose()?;let discovery=self.discovery.clone();tokio::task::spawn_blocking(move||discovery.scan(path.as_deref())).await.map_err(|_|RunError::catalog())?.map_err(|e|RunError{http_status:e.http_status,code:e.code,message:e.message,execution:"not_started"})},
                "fetch__install_servers"=>project(self.manager.install(input).await),
                "fetch__get_installation"=>project(self.manager.installation(input["installation_id"].as_str().filter(|s|!s.trim().is_empty()&&!s.contains('\0')).ok_or_else(RunError::invalid)?).await),
                "fetch__get_servers"=>{let ids=strings(&input,"id")?;project(self.manager.servers_for(&caller,ids,input.get("restart_id").map(|v|v.as_str().filter(|s|!s.trim().is_empty()&&!s.contains('\0')).map(str::to_owned).ok_or_else(RunError::invalid)).transpose()?).await)},
                "fetch__authenticate_server"=>project(self.manager.authenticate(id).await),
                "fetch__restart_server"=>{let mut patch=json!({});if let Some(force)=input.get("force_restart"){patch["force_restart"]=force.clone();}project(self.manager.restart(id,patch).await)},
                "fetch__update_server_config"=>{let id=id.to_owned();let mut patch=input;patch.as_object_mut().ok_or_else(RunError::invalid)?.remove("server_id");project(self.manager.config(&id,patch,&caller).await)},
                _=>Err(RunError::invalid())
            }
        })
    }
}
fn strings(input:&Value,key:&str)->Result<Vec<String>,RunError>{
    let Some(values)=input.get(key)else{return Ok(vec![]);};
    let mut result=Vec::new();
    for value in values.as_array().ok_or_else(RunError::invalid)?{
        let value=value.as_str().filter(|s|!s.trim().is_empty()&&!s.contains('\0')).ok_or_else(RunError::invalid)?;
        if !result.iter().any(|old|old==value){result.push(value.to_owned());}
    }
    Ok(result)
}
