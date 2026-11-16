//! Project-bound history only. Instruction and agent definition routes stay private.
use crate::{api::{AppState,application_error,authorize_scope,empty_body,parse_query},workspace::project_sessions};
use axum::{Json,Router,extract::{Path,Request,State},http::StatusCode,response::{IntoResponse,Response},routing::get};
use std::{collections::BTreeMap,sync::Arc};
pub fn router()->Router<Arc<AppState>>{Router::new().route("/api/v1/sessions",get(list)).route("/api/v1/sessions/{session_id}",get(read))}
async fn handle(state:Arc<AppState>,request:Request,id:Option<String>)->Response{
    let caller=match authorize_scope(&state,request.headers(),"fetch:sessions:read",None){Ok(value)=>value,Err(reply)=>return *reply};
    if caller.project_id().is_none(){return application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Session history requires a project-bound connection.",None);}
    let pairs=match parse_query(request.uri().query().unwrap_or("")){Ok(value)=>value,Err(_)=>return invalid()};
    if empty_body(request).await.is_err(){return invalid();}
    let allowed:&[&str]=if id.is_some(){&["offset","limit"]}else{&["app","cursor","limit"]};
    let mut params=BTreeMap::new();for(key,value)in pairs{if !allowed.contains(&key.as_str())||value.is_empty()||params.insert(key,value).is_some(){return invalid();}}
    let limit=match params.get("limit").map(|value|value.parse::<usize>()).transpose(){Ok(value)=>value,Err(_)=>return invalid()};
    let offset=match params.get("offset").map(|value|value.parse::<u64>()).transpose(){Ok(value)=>value.unwrap_or(0),Err(_)=>return invalid()};
    let (Some(service),Some(registry))=(state.workspace.clone(),state.projects.clone())else{return application_error(StatusCode::SERVICE_UNAVAILABLE,"WORKSPACE_UNAVAILABLE","Fetch could not open the coding-app history folders.",None);};
    match tokio::task::spawn_blocking(move||match id{
        Some(id)=>project_sessions::read(&service,&registry,&caller,&id,offset,limit),
        None=>project_sessions::list(&service,&registry,&caller,params.get("app").map(String::as_str),params.get("cursor").map(String::as_str),limit),
    }).await{
        Ok(Ok(value))=>Json(value).into_response(),Ok(Err(error))=>(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response(),Err(_)=>application_error(StatusCode::INTERNAL_SERVER_ERROR,"WORKSPACE_OPERATION_FAILED","The history read could not finish.",None),
    }
}
async fn list(State(state):State<Arc<AppState>>,request:Request)->Response{handle(state,request,None).await}
async fn read(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{handle(state,request,Some(id)).await}
fn invalid()->Response{application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Use the documented session query fields once each, without a body.",None)}
