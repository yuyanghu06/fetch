//! Native-only registered-folder operations. Browsing never registers a folder.
use crate::{api::{AppState,application_error,empty_body},projects::Project};
use axum::{Json,Router,body::to_bytes,extract::{Request,State},http::{StatusCode,header},response::{IntoResponse,Response},routing::get};
use serde::Deserialize;
use serde_json::{Value,json};
use std::sync::Arc;
pub fn router()->Router<Arc<AppState>>{Router::new().route("/api/v1/projects",get(list).post(register))}
fn native(state:&AppState,request:&Request)->Result<(),Response>{
    if state.native.as_ref().and_then(|native|native.authorize_settings_headers(request.headers())).is_none(){return Err(application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Register and view project folders in Fetch's native window.",None));}Ok(())
}
pub fn value(origin:&str,project:Project)->Value{
    let mut value=serde_json::to_value(&project).expect("project JSON");value["mcp_url"]=json!(format!("{origin}/mcp/projects/{}",project.id));value
}
pub(super) fn error(error:crate::projects::ProjectError)->Response{(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response()}
async fn list(State(state):State<Arc<AppState>>,request:Request)->Response{
    if let Err(reply)=native(&state,&request){return reply;}
    if request.uri().query().is_some()||empty_body(request).await.is_err(){return application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Project listing does not accept a body or query.",None);}
    let Some(registry)=state.projects.clone()else{return unavailable();};
    let origin=state.origin.clone();
    match tokio::task::spawn_blocking(move||registry.list()).await{
        Ok(Ok(projects))=>Json(json!({"status":"ok","projects":projects.into_iter().map(|project|value(&origin,project)).collect::<Vec<_>>(),"next_steps":[]})).into_response(),Ok(Err(failure))=>error(failure),Err(_)=>unavailable(),
    }
}
#[derive(Deserialize)]#[serde(deny_unknown_fields)]struct Registration{folder_path:String,name:Option<String>}
async fn register(State(state):State<Arc<AppState>>,request:Request)->Response{
    if let Err(reply)=native(&state,&request){return reply;}
    if request.uri().query().is_some()||request.headers().get(header::CONTENT_TYPE).and_then(|value|value.to_str().ok()).and_then(|value|value.split(';').next()).map(str::trim)!=Some("application/json"){return application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Register a project with one JSON object.",None);}
    let bytes=match to_bytes(request.into_body(),crate::json::MAX_REQUEST_BYTES).await{Ok(value)=>value,Err(_)=>return invalid()};
    let body:Registration=match crate::json::object(&bytes).ok().and_then(|raw|serde_json::from_str(raw.get()).ok()){Some(value)=>value,None=>return invalid()};
    let Some(registry)=state.projects.clone()else{return unavailable();};
    let origin=state.origin.clone();
    match tokio::task::spawn_blocking(move||registry.register(&body.folder_path,body.name.as_deref())).await{
        Ok(Ok((project,created)))=>(if created{StatusCode::CREATED}else{StatusCode::OK},Json(json!({"status":"ok","project":value(&origin,project),"next_steps":[]}))).into_response(),Ok(Err(failure))=>error(failure),Err(_)=>unavailable(),
    }
}
fn invalid()->Response{application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Provide folder_path and optional name.",None)}
fn unavailable()->Response{application_error(StatusCode::SERVICE_UNAVAILABLE,"PROJECT_STORAGE_UNAVAILABLE","Fetch's saved project folders are unavailable.",None)}
