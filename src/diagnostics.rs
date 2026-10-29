//! Safe request diagnostics on stderr; no bodies, headers, queries or raw URLs.
//!
//! A completed HTTP request means response headers are ready. Streaming MCP
//! output may continue afterward; these events do not claim tool completion.
use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use chrono::{SecondsFormat, Utc};
use serde_json::json;
use std::{io::Write, time::Instant};

/// Created by Fetch, never copied from a caller-controlled header.
#[derive(Clone)]
pub(crate) struct RequestId(String);
tokio::task_local! { static ACTIVE_REQUEST: RequestId; }
pub(crate) fn current_request_id() -> Option<RequestId> { ACTIVE_REQUEST.try_with(Clone::clone).ok() }
pub(crate) fn application_failure(code: &'static str) {
    let request_id=current_request_id();
    write_event(json!({"timestamp": Utc::now().to_rfc3339_opts(SecondsFormat::Millis,true),
        "event":"application_error", "request_id":request_id.as_ref().map(|id|id.0.as_str()),"error_code":code}));
}
pub(crate) async fn with_request_context<F: std::future::Future>(request_id: Option<RequestId>, future: F) -> F::Output {
    match request_id { Some(id) => ACTIVE_REQUEST.scope(id, future).await, None => future.await }
}

/// Only fixed Fetch-owned outcomes can enter authentication diagnostics.
pub(crate) enum AuthenticationOutcome {
    BrowserPending, CredentialRequired, UnsupportedProvider, Verified, NotRequired, Failed,
}

/// Fixed operation names avoid putting URLs, provider payloads or secrets in logs.
#[derive(Clone, Copy)]
pub(crate) enum UpstreamOperation {
    ProviderMetadata, ProviderRegistration, ProviderTokenExchange, ProviderRefresh,
    ProviderVerification, PackageDownload, McpHttp, LocalApi,
}
impl UpstreamOperation {
    fn name(self) -> &'static str {
        match self {
            Self::ProviderMetadata => "provider_metadata",
            Self::ProviderRegistration => "provider_registration",
            Self::ProviderTokenExchange => "provider_token_exchange",
            Self::ProviderRefresh => "provider_refresh",
            Self::ProviderVerification => "provider_verification",
            Self::PackageDownload => "package_download",
            Self::McpHttp => "mcp_http",
            Self::LocalApi => "local_api",
        }
    }
}
struct UpstreamLog { id: String, parent: Option<RequestId>, operation: UpstreamOperation, started: Instant, finished: bool }
impl UpstreamLog {
    fn emit(&self, event: &'static str, status: Option<u16>) {
        write_event(json!({"timestamp": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "event": event, "request_id": self.id, "operation": self.operation.name(),
            "parent_request_id": self.parent.as_ref().map(|id|id.0.as_str()),
            "status": status, "elapsed_ms": self.started.elapsed().as_millis().min(u64::MAX as u128) as u64}));
    }
}
impl Drop for UpstreamLog {
    fn drop(&mut self) { if !self.finished { self.emit("upstream_request_cancelled", None); } }
}
pub(crate) async fn send(operation: UpstreamOperation, request: reqwest::RequestBuilder) -> Result<reqwest::Response, reqwest::Error> {
    let mut log = UpstreamLog { id: format!("fetch-{:032x}", rand::random::<u128>()), parent: current_request_id(), operation, started: Instant::now(), finished: false };
    log.emit("upstream_request_started", None);
    let result = request.send().await;
    match &result {
        Ok(response) => log.emit("upstream_request_completed", Some(response.status().as_u16())),
        Err(_) => log.emit("upstream_request_failed", None),
    }
    log.finished = true;
    result
}
pub(crate) fn authentication_outcome(request_id: Option<&RequestId>, outcome: AuthenticationOutcome) {
    let active_request=current_request_id();let request_id=request_id.or(active_request.as_ref());
    let outcome = match outcome {
        AuthenticationOutcome::BrowserPending => "browser_pending",
        AuthenticationOutcome::CredentialRequired => "credential_required",
        AuthenticationOutcome::UnsupportedProvider => "unsupported_provider",
        AuthenticationOutcome::Verified => "verified",
        AuthenticationOutcome::NotRequired => "not_required",
        AuthenticationOutcome::Failed => "failed",
    };
    write_event(json!({
        "timestamp": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        "event": "authentication_outcome",
        "request_id": request_id.map(|id| id.0.as_str()),
        "outcome": outcome,
    }));
}

struct RequestLog {
    id: RequestId,
    method: &'static str,
    route: String,
    started: Instant,
    completed: bool,
}
impl RequestLog {
    fn start(request: &Request) -> Self {
        let method = match request.method().as_str() {
            "GET" => "GET", "POST" => "POST", "PUT" => "PUT", "PATCH" => "PATCH",
            "DELETE" => "DELETE", "HEAD" => "HEAD", "OPTIONS" => "OPTIONS",
            "CONNECT" => "CONNECT", "TRACE" => "TRACE", _ => "<other>",
        };
        // MatchedPath is the registered route template, never the request URI.
        // The fallback deliberately provides no information about unknown paths.
        let route = request.extensions().get::<MatchedPath>()
            .map(|path| path.as_str()).unwrap_or("<unmatched>").to_owned();
        let log = Self { id: RequestId(format!("fetch-{:032x}", rand::random::<u128>())),
            method, route, started: Instant::now(), completed: false };
        log.emit("request_started", None);
        log
    }
    fn emit(&self, event: &'static str, status: Option<u16>) {
        write_event(json!({
            "timestamp": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "event": event,
            "request_id": self.id.0,
            "method": self.method,
            "route": self.route,
            "status": status,
            "elapsed_ms": self.started.elapsed().as_millis().min(u64::MAX as u128) as u64,
        }));
    }
    fn complete(mut self, status: u16) {
        self.emit("request_completed", Some(status));
        self.completed = true;
    }
}
impl Drop for RequestLog {
    fn drop(&mut self) {
        if !self.completed { self.emit("request_cancelled", None); }
    }
}

fn write_event(value: serde_json::Value) {
    if let Ok(mut line) = serde_json::to_vec(&value) {
        line.push(b'\n');
        // Diagnostics must never fail an otherwise valid product operation.
        let _ = std::io::stderr().lock().write_all(&line);
    }
}

pub(crate) async fn log_request(mut request: Request, next: Next) -> Response {
    let log = RequestLog::start(&request);
    request.extensions_mut().insert(log.id.clone());
    let response = ACTIVE_REQUEST.scope(log.id.clone(), next.run(request)).await;
    log.complete(response.status().as_u16());
    response
}
