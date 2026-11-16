//! Native Workspace routes wrap shared file services; none are MCP operations.
use crate::{api::{AppState, application_error, empty_body, parse_query}, workspace::{WorkspaceError, WorkspaceService}};
use axum::{Json, Router, body::to_bytes, extract::{Request, State}, http::{HeaderMap, StatusCode, header}, response::{IntoResponse, Response}, routing::{get, post}};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::Arc};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/workspace",get(discover))
        .route("/api/v1/workspace/file",get(read_file).put(save_file))
        .route("/api/v1/workspace/file/reveal",post(reveal_file))
        .route("/api/v1/workspace/agents/convert",post(convert_agent))
        .route("/api/v1/workspace/models",get(models))
        .route("/api/v1/workspace/sessions",get(sessions))
        .route("/api/v1/workspace/sessions/read",get(read_session))
        .route("/api/v1/workspace/sessions/conversation",get(conversation))
}

fn native(state:&AppState, headers:&HeaderMap) -> Result<(),Response> {
    if state.native.as_ref().and_then(|native|native.authorize_settings_headers(headers)).is_none() {
        return Err(application_error(StatusCode::FORBIDDEN,"FORBIDDEN","Workspace files and history are available only through Fetch's native window.",None));
    }
    Ok(())
}
fn service(state:&AppState) -> Result<Arc<WorkspaceService>,Response> {
    state.workspace.clone().ok_or_else(||application_error(StatusCode::SERVICE_UNAVAILABLE,"WORKSPACE_UNAVAILABLE","Fetch could not resolve the coding-app folders for this user.",None))
}
fn failure(error:WorkspaceError) -> Response {
    (StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response()
}
fn invalid(message:&str) -> Response { application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST",message,None) }
async fn run(operation:impl FnOnce()->Result<Value,WorkspaceError>+Send+'static) -> Response {
    match tokio::task::spawn_blocking(operation).await {
        Ok(Ok(value))=>Json(value).into_response(),Ok(Err(error))=>failure(error),
        Err(_)=>application_error(StatusCode::INTERNAL_SERVER_ERROR,"WORKSPACE_OPERATION_FAILED","The Workspace operation could not finish.",None),
    }
}
async fn parameters(state:&AppState,request:Request,allowed:&[&str])->Result<BTreeMap<String,String>,Response>{
    native(state,request.headers())?;
    let query=request.uri().query().unwrap_or("").to_owned();
    empty_body(request).await.map_err(|_|invalid("Workspace reads do not accept a request body."))?;
    let pairs=parse_query(&query).map_err(|_|invalid("The Workspace query could not be read."))?;
    let mut output=BTreeMap::new();
    for (key,value) in pairs {
        if !allowed.contains(&key.as_str())||value.is_empty()||output.insert(key,value).is_some(){return Err(invalid("Use the documented Workspace query fields once each."));}
    }
    Ok(output)
}
fn required<'a>(params:&'a BTreeMap<String,String>,name:&str)->Result<&'a str,Response>{
    params.get(name).map(String::as_str).ok_or_else(||invalid(&format!("Provide {name}.")))
}
fn number<T:std::str::FromStr>(params:&BTreeMap<String,String>,name:&str)->Result<Option<T>,Response>{
    params.get(name).map(|value|value.parse().map_err(|_|invalid(&format!("{name} must be a nonnegative integer.")))).transpose()
}
async fn input<T:serde::de::DeserializeOwned>(state:&AppState,request:Request)->Result<T,Response>{
    native(state,request.headers())?;
    if request.uri().query().is_some(){return Err(invalid("Send Workspace changes as a JSON body."));}
    if request.headers().get(header::CONTENT_TYPE).and_then(|value|value.to_str().ok()).is_none_or(|value|value.split(';').next().map(str::trim)!=Some("application/json")){
        return Err(invalid("Use an application/json body."));
    }
    let bytes=to_bytes(request.into_body(),2*1024*1024).await.map_err(|_|application_error(StatusCode::PAYLOAD_TOO_LARGE,"WORKSPACE_FILE_TOO_LARGE","The Workspace request exceeds 2 MiB.",None))?;
    serde_json::from_slice(&bytes).map_err(|error|invalid(&error.to_string()))
}

async fn discover(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["folder_path"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    run(move||service.discover(params.get("folder_path").map(String::as_str))).await
}
async fn read_file(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["folder_path","file_path"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let path=match required(&params,"file_path"){Ok(value)=>value.to_owned(),Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    run(move||service.read_file(params.get("folder_path").map(String::as_str),&path)).await
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Save {folder_path:Option<String>,file_path:String,content:String}
async fn save_file(State(state):State<Arc<AppState>>,request:Request)->Response{
    let body:Save=match input(&state,request).await{Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    run(move||service.save_file(body.folder_path.as_deref(),&body.file_path,&body.content)).await
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Convert {source_app:String,target_app:String,content:String,target_model:Option<String>}
async fn convert_agent(State(state):State<Arc<AppState>>,request:Request)->Response{
    let body:Convert=match input(&state,request).await{Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    run(move||{
        let mut value=service.convert_agent(&body.source_app,&body.target_app,&body.content)?;
        if let Some(model)=body.target_model{
            let content=value["content"].as_str().unwrap_or("");
            let changed=service.set_agent_model(&body.target_app,content,&model)?;
            value["content"]=changed["content"].clone();
            if let Some(issues)=value["issues"].as_array_mut(){issues.retain(|issue|issue["field"]!="model");}
            value["warnings"]=json!(value["issues"].as_array().into_iter().flatten().filter_map(|issue|issue["message"].as_str()).collect::<Vec<_>>());
            value["requires_review"]=json!(value["issues"].as_array().is_some_and(|issues|!issues.is_empty()));
        }
        Ok(value)
    }).await
}
async fn models(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["app"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let app=match required(&params,"app"){Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    match crate::workspace::models::catalog_for(&service,app).await{Ok(value)=>Json(value).into_response(),Err(error)=>failure(error)}
}
async fn sessions(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["folder_path","app","cursor","limit","view"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let limit=match number::<usize>(&params,"limit"){Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    if let Some(view)=params.get("view") {
        if view!="conversation"{return invalid("Use view=conversation or omit view for raw session listing.");}
        return match service.list_conversations(params.get("folder_path").map(String::as_str),params.get("app").map(String::as_str),params.get("cursor").map(String::as_str),limit).await{Ok(value)=>Json(value).into_response(),Err(error)=>failure(error)};
    }
    run(move||service.list_sessions(params.get("folder_path").map(String::as_str),params.get("app").map(String::as_str),params.get("cursor").map(String::as_str),limit)).await
}
async fn conversation(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["folder_path","file_path"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let path=match required(&params,"file_path"){Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    match service.read_conversation(params.get("folder_path").map(String::as_str),path).await{Ok(value)=>Json(value).into_response(),Err(error)=>failure(error)}
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RevealFile {file_path:String,folder_path:Option<String>}
async fn reveal_file(State(state):State<Arc<AppState>>,request:Request)->Response{
    let body:RevealFile=match input(&state,request).await{Ok(value)=>value,Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    match service.reveal_file(body.folder_path.as_deref(),&body.file_path).await{Ok(value)=>Json(value).into_response(),Err(error)=>failure(error)}
}
async fn read_session(State(state):State<Arc<AppState>>,request:Request)->Response{
    let params=match parameters(&state,request,&["folder_path","file_path","offset","limit"]).await{Ok(value)=>value,Err(reply)=>return reply};
    let path=match required(&params,"file_path"){Ok(value)=>value.to_owned(),Err(reply)=>return reply};
    let limit=match number::<usize>(&params,"limit"){Ok(value)=>value,Err(reply)=>return reply};
    let offset=match number::<u64>(&params,"offset"){Ok(value)=>value.unwrap_or(0),Err(reply)=>return reply};
    let service=match service(&state){Ok(value)=>value,Err(reply)=>return reply};
    run(move||service.read_session(params.get("folder_path").map(String::as_str),&path,offset,limit)).await
}
