//! Open the selected instance's known native state folder for the local person.
use crate::api::{AppState, application_error, empty_body, runtime};
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
    // A bearer grant cannot cause a window to open on the user's desktop.
    let Some(caller) = state.native.as_ref()
        .and_then(|native| native.authorize_settings_headers(request.headers())) else {
        return application_error(
            StatusCode::FORBIDDEN,
            "FORBIDDEN",
            "Open server folder is available only in Fetch's native Settings window.",
            None,
        );
    };
    if let Err(reply) = runtime::target(&caller, "fetch:manage", &id, None) {
        return reply;
    }
    if request.uri().query().is_some() || empty_body(request).await.is_err() {
        return application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "Open server folder does not accept query parameters or a request body.",
            None,
        );
    }
    let manager = match runtime::manager(&state) {
        Ok(manager) => manager,
        Err(reply) => return reply,
    };
    match manager.open_server_folder(&id).await {
        Ok(value) => Json(value).into_response(),
        Err(error) => runtime::management_error(error),
    }
}
