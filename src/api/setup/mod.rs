//! Read-only discovery uses the same verified project as installation.
use crate::api::{AppState,application_error,authorize_management,empty_body,parse_query};
use axum::{Json,extract::{Request,State},http::StatusCode,response::{IntoResponse,Response}};
use std::{path::PathBuf,sync::Arc};
pub async fn discovery(State(state):State<Arc<AppState>>,request:Request)->Response{
    let caller=match authorize_management(&state,request.headers(),Some("discovery")){Ok(value)=>value,Err(reply)=>return *reply};
    let query=match parse_query(request.uri().query().unwrap_or("")){Ok(value)=>value,Err(_)=>return invalid()};
    if empty_body(request).await.is_err(){return invalid();}
    let(mut path,mut supplied)=(None,None);
    for(key,value)in query{
        match key.as_str(){
            "config_path" if path.is_none()&&!value.trim().is_empty()&&!value.contains('\0')&&PathBuf::from(&value).is_absolute()=>path=Some(PathBuf::from(value)),
            "project_id" if supplied.is_none()&&caller.is_native_settings()&&!value.is_empty()=>supplied=Some(value),
            _=>return invalid(),
        }
    }
    let selected=supplied.as_deref().or(caller.project_id());
    let project=if let Some(id)=selected{
        let Some(registry)=&state.projects else{return application_error(StatusCode::SERVICE_UNAVAILABLE,"PROJECT_STORAGE_UNAVAILABLE","Project records are unavailable.",Some("discovery"));};
        match registry.get(id){Ok(Some(project))=>Some(project),Ok(None)=>return application_error(StatusCode::NOT_FOUND,"PROJECT_NOT_FOUND","The project was not found.",Some("discovery")),Err(error)=>return (StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),Json(error.body())).into_response()}
    }else{None};
    if !caller.is_native_settings(){if let(Some(project),Some(path))=(&project,&path){if !path.canonicalize().is_ok_and(|path|path.starts_with(&project.folder_path)){return application_error(StatusCode::FORBIDDEN,"FORBIDDEN","This connection can discover configurations only inside its project folder.",Some("discovery"));}}}
    match tokio::task::spawn_blocking(move||state.discovery.scan_scoped(path.as_deref(),&state.repository,project.as_ref())).await{
        Ok(Ok(value))=>Json(value).into_response(),Ok(Err(error))=>application_error(StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),error.code,error.message,Some("discovery")),Err(_)=>application_error(StatusCode::INTERNAL_SERVER_ERROR,"DISCOVERY_FAILED","Configuration discovery could not finish safely.",Some("discovery")),
    }
}
fn invalid()->Response{application_error(StatusCode::BAD_REQUEST,"INVALID_REQUEST","Use an optional absolute config_path and native project_id, once each, without a body.",Some("discovery"))}
