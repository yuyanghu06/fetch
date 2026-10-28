//! Existing product routes wrap shared backend services.
pub mod setup;

use crate::{
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    storage::ServerRepository,
};
use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;

pub struct AppState {
    pub origin: String,
    pub repository: ServerRepository,
    pub discovery: DiscoveryService,
    pub authority: Arc<dyn Authorizer>,
}

impl AppState {
    pub fn new(
        origin: String,
        repository: ServerRepository,
        discovery: DiscoveryService,
        authority: Arc<dyn Authorizer>,
    ) -> Result<Self, &'static str> {
        let url = url::Url::parse(&origin).map_err(|_| "Invalid Fetch origin")?;
        if url.scheme() != "https"
            || url.host_str() != Some("localhost")
            || url.port().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.path() != "/"
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err("Fetch requires an explicit HTTPS localhost origin and port");
        }
        Ok(Self {
            origin: origin.trim_end_matches('/').to_owned(),
            repository,
            discovery,
            authority,
        })
    }
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/.well-known/oauth-protected-resource/mcp",
            get(resource_metadata),
        )
        .route(
            "/.well-known/oauth-authorization-server",
            get(authorization_metadata),
        )
        .route("/api/v1/servers", get(servers))
        .route("/api/v1/setup/discovery", get(setup::discovery))
        .route(
            "/mcp",
            get(mcp_pending).post(mcp_pending).delete(mcp_pending),
        )
        .method_not_allowed_fallback(method_not_allowed)
        .fallback(not_found)
        .layer(middleware::from_fn_with_state(state.clone(), boundary))
        .with_state(state)
}

async fn boundary(State(state): State<Arc<AppState>>, request: Request, next: Next) -> Response {
    let authority = state
        .origin
        .strip_prefix("https://")
        .expect("validated origin");
    let hosts: Vec<_> = request.headers().get_all(header::HOST).iter().collect();
    let valid_host = match hosts.as_slice() {
        [host] => host.to_str().ok() == Some(authority),
        [] if request.version() == axum::http::Version::HTTP_2 => request
            .uri()
            .authority()
            .is_some_and(|a| a.as_str() == authority),
        _ => false,
    };
    let uri_authority = request
        .uri()
        .authority()
        .is_none_or(|a| a.as_str() == authority);
    let origins: Vec<_> = request.headers().get_all(header::ORIGIN).iter().collect();
    let valid_origin = origins.is_empty()
        || (origins.len() == 1 && origins[0].to_str().ok() == Some(state.origin.as_str()));
    let mut response = if !valid_host || !uri_authority {
        application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "Use Fetch's configured local address.",
            None,
        )
    } else if !valid_origin {
        application_error(
            StatusCode::FORBIDDEN,
            "FORBIDDEN",
            "This browser origin is not allowed.",
            None,
        )
    } else {
        next.run(request).await
    };
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    response.headers_mut().insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    response
}

async fn resource_metadata(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(
        json!({"resource": format!("{}/mcp", state.origin), "authorization_servers": [state.origin], "scopes_supported": ["fetch:catalog", "fetch:tools:call", "fetch:content:read", "fetch:manage"], "bearer_methods_supported": ["header"]}),
    )
}

async fn authorization_metadata() -> Response {
    // Advertising incomplete grant endpoints would mislead connecting clients.
    (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error":"temporarily_unavailable", "error_description":"Native approval and token issuance are not implemented in this development increment."}))).into_response()
}

pub(super) fn authorize_management(
    state: &AppState,
    headers: &HeaderMap,
    shape: Option<&str>,
) -> Result<Caller, Box<Response>> {
    let caller = match state.authority.authorize(headers) {
        Ok(caller) => caller,
        Err(AuthorizationError::Unauthenticated) => {
            let mut reply = application_error(
                StatusCode::UNAUTHORIZED,
                "UNAUTHENTICATED",
                "Connect this app to Fetch before using this operation.",
                shape,
            );
            let challenge = format!(
                "Bearer resource_metadata=\"{}/.well-known/oauth-protected-resource/mcp\"",
                state.origin
            );
            reply.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                HeaderValue::from_str(&challenge).expect("validated origin"),
            );
            return Err(Box::new(reply));
        }
        Err(AuthorizationError::Unavailable) => {
            return Err(Box::new(application_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "AUTHORIZATION_UNAVAILABLE",
                "Fetch authorization is unavailable.",
                shape,
            )));
        }
    };
    if !caller.can_manage() {
        return Err(Box::new(application_error(
            StatusCode::FORBIDDEN,
            "FORBIDDEN",
            "Server-management permission is required.",
            shape,
        )));
    }
    Ok(caller)
}

pub(super) fn application_error(
    status: StatusCode,
    code: &str,
    message: &str,
    shape: Option<&str>,
) -> Response {
    let mut body =
        json!({"status":"failed", "error":{"code":code,"message":message}, "next_steps":[]});
    match shape {
        Some("servers") => {
            body["servers"] = json!([]);
            body["not_found_ids"] = json!([]);
        }
        Some("discovery") => {
            body["configs"] = json!([]);
        }
        _ => {}
    }
    (status, Json(body)).into_response()
}

pub(super) async fn empty_body(request: Request) -> Result<HeaderMap, ()> {
    let (parts, body) = request.into_parts();
    match to_bytes(body, 1).await {
        Ok(bytes) if bytes.is_empty() => Ok(parts.headers),
        _ => Err(()),
    }
}

async fn servers(State(state): State<Arc<AppState>>, request: Request) -> Response {
    if let Err(reply) = authorize_management(&state, request.headers(), Some("servers")) {
        return *reply;
    }
    let query = request.uri().query().unwrap_or("").to_owned();
    if empty_body(request).await.is_err() {
        return application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "This read does not accept a body.",
            Some("servers"),
        );
    }
    let mut ids = Vec::new();
    let mut id_count = 0;
    let mut restart = None;
    let query = match parse_query(&query) {
        Ok(query) => query,
        Err(()) => {
            return application_error(
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                "The query is not valid UTF-8 form encoding.",
                Some("servers"),
            );
        }
    };
    for (key, value) in query {
        match key.as_ref() {
            "id" if !value.trim().is_empty() => {
                id_count += 1;
                if !ids.contains(&value) {
                    ids.push(value);
                }
            }
            "restart_id" if restart.is_none() && !value.trim().is_empty() => restart = Some(value),
            _ => {
                return application_error(
                    StatusCode::BAD_REQUEST,
                    "INVALID_REQUEST",
                    "Use nonempty id parameters or one restart_id.",
                    Some("servers"),
                );
            }
        }
    }
    if restart.is_some() && id_count != 1 {
        return application_error(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "A restart lookup requires exactly one server ID.",
            Some("servers"),
        );
    }
    let repository = state.clone();
    let selected_ids = ids.clone();
    let records = match tokio::task::spawn_blocking(move || {
        if selected_ids.is_empty() {
            repository.repository.list()
        } else {
            selected_ids
                .iter()
                .filter_map(|id| repository.repository.get(id).transpose())
                .collect()
        }
    })
    .await
    {
        Ok(Ok(records)) => records,
        _ => {
            return application_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                if restart.is_some() {
                    "RESTART_STATUS_UNAVAILABLE"
                } else {
                    "SERVER_INFO_UNAVAILABLE"
                },
                "Saved server information could not be read safely.",
                Some("servers"),
            );
        }
    };
    if restart.is_some() {
        if records.is_empty() {
            return (StatusCode::NOT_FOUND, Json(json!({"status":"failed","error":{"code":"SERVER_NOT_FOUND","message":"The selected server was not found."},"servers":[],"not_found_ids":ids,"next_steps":[]}))).into_response();
        }
        return application_error(
            StatusCode::NOT_FOUND,
            "RESTART_NOT_FOUND",
            "No matching restart attempt exists in this Fetch run.",
            Some("servers"),
        );
    }
    let mut found = Vec::new();
    let mut missing = Vec::new();
    if ids.is_empty() {
        found = records;
    } else {
        for id in ids {
            if let Some(record) = records
                .iter()
                .find(|r| r["server_id"].as_str() == Some(&id))
            {
                found.push(record.clone());
            } else {
                missing.push(id);
            }
        }
    }
    let steps: Vec<Value> = found.iter().map(|record| json!({"actor":"user","action":"review_server_settings","instructions":"This developer build has no running server or verified provider login. Server startup and authentication are not implemented yet.","server_id":record["server_id"]})).collect();
    Json(json!({"status":"ok","servers":found,"not_found_ids":missing,"next_steps":steps}))
        .into_response()
}

async fn mcp_pending(State(state): State<Arc<AppState>>, request: Request<Body>) -> Response {
    match state.authority.authorize(request.headers()) {
        Err(AuthorizationError::Unavailable) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Fetch authorization is unavailable.",
        )
            .into_response(),
        Err(AuthorizationError::Unauthenticated) => {
            let mut reply = StatusCode::UNAUTHORIZED.into_response();
            let challenge = format!(
                "Bearer resource_metadata=\"{}/.well-known/oauth-protected-resource/mcp\"",
                state.origin
            );
            reply.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                HeaderValue::from_str(&challenge).expect("validated origin"),
            );
            reply
        }
        Ok(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "MCP sharing is not implemented in this development increment.",
        )
            .into_response(),
    }
}

async fn not_found() -> Response {
    application_error(
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "This route is not available.",
        None,
    )
}

async fn method_not_allowed() -> Response {
    application_error(
        StatusCode::METHOD_NOT_ALLOWED,
        "METHOD_NOT_ALLOWED",
        "This route does not accept that HTTP method.",
        None,
    )
}

/// Unlike lossy form decoding, reject malformed escapes and invalid UTF-8 IDs.
pub(super) fn parse_query(query: &str) -> Result<Vec<(String, String)>, ()> {
    if query.len() > 16 * 1024 {
        return Err(());
    }
    fn component(input: &str) -> Result<String, ()> {
        let mut output = Vec::new();
        let mut bytes = input.bytes();
        while let Some(byte) = bytes.next() {
            match byte {
                b'+' => output.push(b' '),
                b'%' => {
                    let hi = (bytes.next().ok_or(())? as char).to_digit(16).ok_or(())?;
                    let lo = (bytes.next().ok_or(())? as char).to_digit(16).ok_or(())?;
                    output.push((hi * 16 + lo) as u8);
                }
                value => output.push(value),
            }
        }
        String::from_utf8(output).map_err(|_| ())
    }
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            Ok((component(key)?, component(value)?))
        })
        .collect()
}
