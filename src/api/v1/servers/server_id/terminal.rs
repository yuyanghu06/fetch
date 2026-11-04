//! Native Settings alone may operate the backend-owned terminal.
use crate::{api::{AppState, application_error, empty_body, parse_query, runtime}, terminal::{Action,TerminalError}};
use axum::{Json, body::to_bytes, extract::{Path,Request,State}, http::{StatusCode,header}, response::{IntoResponse,Response}};
use std::sync::Arc;

fn authorize(state:&AppState,id:&str,request:&Request)->Result<(),Response>{
    let Some(caller)=state.native.as_ref().and_then(|native|native.authorize_settings_headers(request.headers())) else{return Err(application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Terminal access is available only in Fetch's native Settings window.",None));};
    runtime::target(&caller,"fetch:manage",id,None)
}
fn failure(error:TerminalError)->Response{(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response()}
fn invalid()->Response{failure(TerminalError::invalid())}

pub async fn post(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    if let Err(response)=authorize(&state,&id,&request){return response;}
    if request.uri().query().is_some(){return invalid();}
    let json_content=request.headers().get(header::CONTENT_TYPE).and_then(|v|v.to_str().ok()).is_some_and(|v|v.split(';').next().is_some_and(|v|v.trim().eq_ignore_ascii_case("application/json")));
    if !json_content{return invalid();}
    let bytes=match to_bytes(request.into_body(),128*1024).await{Ok(bytes)=>bytes,Err(_)=>return invalid()};
    let raw=match crate::json::object(&bytes){Ok(raw)=>raw,Err(_)=>return invalid()};
    let value=match serde_json::from_str(raw.get()){Ok(value)=>value,Err(_)=>return invalid()};
    let action=match Action::parse(value){Ok(action)=>action,Err(error)=>return failure(error)};
    let manager=match runtime::manager(&state){Ok(manager)=>manager,Err(response)=>return response};
    match manager.terminal_action(&id,action).await{Ok((value,created))=>(if created{StatusCode::CREATED}else{StatusCode::OK},Json(value)).into_response(),Err(error)=>failure(error)}
}

pub async fn get(State(state):State<Arc<AppState>>,Path(id):Path<String>,request:Request)->Response{
    if let Err(response)=authorize(&state,&id,&request){return response;}
    let query=match parse_query(request.uri().query().unwrap_or("")){Ok(query)=>query,Err(_)=>return invalid()};
    let mut session=None;let mut after=None;
    for (key,value) in query{match key.as_str(){
        "session_id" if session.is_none()&&!value.is_empty()&&!value.contains('\0')=>session=Some(value),
        "after" if after.is_none()&&!value.is_empty()&&value.bytes().all(|v|v.is_ascii_digit())=>match value.parse::<u64>(){Ok(value)=>after=Some(value),Err(_)=>return invalid()},
        _=>return invalid(),
    }}
    let Some(session)=session else{return invalid()};
    if empty_body(request).await.is_err(){return invalid();}
    let manager=match runtime::manager(&state){Ok(manager)=>manager,Err(response)=>return response};
    match manager.terminal_output(&id,&session,after.unwrap_or(0)).await{Ok(value)=>Json(value).into_response(),Err(error)=>failure(error)}
}
