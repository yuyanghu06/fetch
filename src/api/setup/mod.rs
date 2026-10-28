//! Read-only configuration discovery. Preparation is a separate later operation.
use crate::api::{AppState, application_error, authorize_management, empty_body, parse_query};
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{path::PathBuf, sync::Arc};

pub async fn discovery(State(state): State<Arc<AppState>>, request: Request) -> Response {
    if let Err(reply) = authorize_management(&state, request.headers(), Some("discovery")) {
        return *reply;
    }
    let query = request.uri().query().unwrap_or("").to_owned();
    if empty_body(request).await.is_err() {
        return application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "This read does not accept a body.",
            Some("discovery"),
        );
    }
    let mut path = None;
    let query = match parse_query(&query) {
        Ok(query) => query,
        Err(()) => {
            return application_error(
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                "The query is not valid UTF-8 form encoding.",
                Some("discovery"),
            );
        }
    };
    for (key, value) in query {
        if key != "config_path"
            || path.is_some()
            || value.trim().is_empty()
            || value.contains('\0')
            || !PathBuf::from(value.as_str()).is_absolute()
        {
            return application_error(
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                "Use one absolute config_path or no query parameters.",
                Some("discovery"),
            );
        }
        path = Some(PathBuf::from(value));
    }
    match tokio::task::spawn_blocking(move || state.discovery.scan(path.as_deref())).await {
        Ok(Ok(value)) => Json(value).into_response(),
        Ok(Err(error)) => application_error(
            StatusCode::from_u16(error.http_status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            error.code,
            error.message,
            Some("discovery"),
        ),
        Err(_) => application_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DISCOVERY_FAILED",
            "Configuration discovery could not finish safely.",
            Some("discovery"),
        ),
    }
}
