//! Delete one managed server through the shared management backend.
use crate::api::{AppState, application_error, authorize_scope, empty_body, runtime};
use axum::{
    Json,
    extract::{Path, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    request: Request,
) -> Response {
    let caller = match authorize_scope(&state, request.headers(), "fetch:manage", None) {
        Ok(caller) => caller,
        Err(reply) => return *reply,
    };
    if let Err(reply) = runtime::target(&caller, "fetch:manage", &id, None) {
        return reply;
    }
    if request.uri().query().is_some() || empty_body(request).await.is_err() {
        return application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "Server deletion does not accept query parameters or a request body.",
            None,
        );
    }
    let manager = match runtime::manager(&state) {
        Ok(manager) => manager,
        Err(reply) => return reply,
    };
    match manager.delete_server(&id, &caller).await {
        Ok(value) => Json(value).into_response(),
        Err(error) => runtime::management_error(error),
    }
}
