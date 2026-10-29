//! A Railway native-OAuth callback bound only to one numeric-loopback flow.
use super::{Manager, auth_required, now};
use axum::{
    Router,
    body::to_bytes,
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode, header},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::{collections::BTreeMap, sync::{Arc, Weak}, time::Duration};
use tokio::{net::TcpListener, sync::{Notify, oneshot}};

pub(super) struct Listener {
    socket: TcpListener,
    authority: String,
    redirect: String,
}
pub(super) struct Lease(Option<oneshot::Sender<()>>);
impl Drop for Lease {
    fn drop(&mut self) { if let Some(stop) = self.0.take() { let _ = stop.send(()); } }
}
pub(super) struct AttemptGuard {
    manager: Weak<Manager>, server: String, attempt: String, armed: bool,
}
impl AttemptGuard {
    pub fn new(manager: Weak<Manager>, server: String, attempt: String) -> Self {
        Self { manager, server, attempt, armed: true }
    }
    pub fn disarm(&mut self) { self.armed = false; }
}
impl Drop for AttemptGuard {
    fn drop(&mut self) {
        if !self.armed { return; }
        let manager=self.manager.clone();let server=self.server.clone();let attempt=self.attempt.clone();
        if let Ok(runtime)=tokio::runtime::Handle::try_current(){
            let request_id=crate::diagnostics::current_request_id();
            runtime.spawn(crate::diagnostics::with_request_context(request_id,async move{
                if let Some(manager)=manager.upgrade(){manager.end_auth_failure(&server,&attempt,"AUTHENTICATION_FAILED","The sign-in callback was interrupted. Retry Authenticate to start a fresh sign-in.").await;}
            }));
        }
    }
}
#[derive(Clone)]
struct CallbackState {
    manager: Weak<Manager>,
    expected: String,
    authority: String,
    deadline: u64,
}
impl Listener {
    pub async fn bind() -> Result<Self, super::ManagementError> {
        let socket = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).await.map_err(|_| auth_required())?;
        let authority = socket.local_addr().map_err(|_| auth_required())?.to_string();
        Ok(Self { redirect: format!("http://{authority}/oauth/provider/callback"), authority, socket })
    }
    pub fn redirect(&self) -> &str { &self.redirect }
    /// The lease stays inside the pending ProviderFlow. Removing that flow closes
    /// this listener, while allowing its accepted callback response to finish.
    pub fn start(self, manager: Weak<Manager>, shutdown: Arc<Notify>, server: String, expected: String, deadline: u64) -> Lease {
        let (stop, stopped) = oneshot::channel();
        let state = Arc::new(CallbackState { manager: manager.clone(), expected: expected.clone(), authority: self.authority, deadline });
        let routes = Router::new().route("/oauth/provider/callback", get(callback))
            .method_not_allowed_fallback(|| async { page(StatusCode::METHOD_NOT_ALLOWED, "Use the provider's sign-in callback.") })
            .fallback(|| async { page(StatusCode::NOT_FOUND, "This sign-in callback was not found.") })
            .with_state(state)
            .layer(middleware::from_fn(crate::diagnostics::log_request));
        let request_id = crate::diagnostics::current_request_id();
        let quit = shutdown.notified_owned();
        tokio::spawn(crate::diagnostics::with_request_context(request_id, async move {
            let shutdown = async move {
                tokio::select! {
                    _ = stopped => {},
                    _ = quit => {},
                    _ = tokio::time::sleep(Duration::from_secs(deadline.saturating_sub(now()))) => {},
                }
            };
            if axum::serve(self.socket, routes).with_graceful_shutdown(shutdown).await.is_err() {
                if let Some(manager) = manager.upgrade() {
                    manager.flows.lock().await.remove(&expected);
                    manager.end_auth_failure(&server, &expected, "AUTHENTICATION_FAILED", "The temporary sign-in callback stopped. Retry Authenticate to start a fresh browser sign-in.").await;
                }
            }
        }));
        Lease(Some(stop))
    }
}

async fn callback(State(state): State<Arc<CallbackState>>, request: Request) -> Response {
    let hosts: Vec<_> = request.headers().get_all(header::HOST).iter().collect();
    let valid_host = hosts.len() == 1 && hosts[0].to_str().ok() == Some(state.authority.as_str());
    if request.method() != Method::GET || request.uri().path() != "/oauth/provider/callback"
        || !valid_host || request.uri().authority().is_some_and(|a| a.as_str() != state.authority.as_str()) {
        return page(StatusCode::BAD_REQUEST, "This sign-in callback is invalid.");
    }
    let Some(query) = query(request.uri().query().unwrap_or("")) else {
        return page(StatusCode::BAD_REQUEST, "This sign-in callback is invalid.");
    };
    if query.get("state").map(String::as_str) != Some(state.expected.as_str()) || now() >= state.deadline {
        return page(StatusCode::BAD_REQUEST, "This sign-in callback does not match a pending login.");
    }
    if !matches!(tokio::time::timeout(Duration::from_secs(1), to_bytes(request.into_body(), 0)).await, Ok(Ok(_))) {
        return page(StatusCode::BAD_REQUEST, "This sign-in callback is invalid.");
    }
    let Some(manager) = state.manager.upgrade() else {
        return page(StatusCode::SERVICE_UNAVAILABLE, "Fetch has stopped. Start a fresh sign-in from Fetch.");
    };
    match manager.provider_callback(&state.expected, query.get("code").map(String::as_str), query.get("error").map(String::as_str), query.get("iss").map(String::as_str)).await {
        Ok(()) => page(StatusCode::OK, "Railway sign-in is verified. Return to Fetch, then use Connect to start the server or apply the new login."),
        Err(_) => page(StatusCode::BAD_REQUEST, "Railway sign-in did not complete. Return to Fetch to review its status and retry Authenticate."),
    }
}

fn query(raw: &str) -> Option<BTreeMap<String, String>> {
    if raw.is_empty() || raw.len() > 16 * 1024 { return None; }
    let mut values = BTreeMap::new();
    for pair in raw.split('&') {
        let (key, value) = pair.split_once('=')?;
        let key = decode(key)?;
        if !["state", "code", "error", "error_description", "iss"].contains(&key.as_str()) { return None; }
        let value = decode(value)?;
        if value.contains('\0') || values.insert(key, value).is_some() { return None; }
    }
    if values.get("state").is_none_or(|value| value.is_empty() || value.len() > 256) { return None; }
    let code = values.get("code").is_some_and(|value| !value.is_empty() && value.len() <= 8192);
    let error = values.get("error").is_some_and(|value| !value.is_empty() && value.len() <= 256);
    if code == error || (values.contains_key("code") && !code) || (values.contains_key("error") && !error) { return None; }
    Some(values)
}
fn decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' => {
                let high = (*bytes.get(index + 1)? as char).to_digit(16)?;
                let low = (*bytes.get(index + 2)? as char).to_digit(16)?;
                decoded.push((high * 16 + low) as u8); index += 3;
            },
            b'+' => { decoded.push(b' '); index += 1; },
            byte => { decoded.push(byte); index += 1; },
        }
    }
    String::from_utf8(decoded).ok()
}
fn page(status: StatusCode, message: &'static str) -> Response {
    let mut response = (status, Html(crate::browser::message_page("Fetch Railway sign-in", message))).into_response();
    for (name, value) in [
        (header::CACHE_CONTROL, "no-store"),
        (header::REFERRER_POLICY, "no-referrer"),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::CONTENT_SECURITY_POLICY, "default-src 'none'; frame-ancestors 'none'; base-uri 'none'"),
    ] { response.headers_mut().insert(name, HeaderValue::from_static(value)); }
    response
}
