//! Native approval for local stdio connectors. Requests themselves grant no access.
use super::*;
use axum::extract::Path;

#[cfg(test)]
#[path = "connections_tests.rs"]
mod tests;

#[derive(Clone, PartialEq, Eq)]
enum Status {
    Pending,
    Confirming(BTreeSet<String>),
    Approved(BTreeSet<String>),
    Denied,
}

#[derive(Clone)]
struct ConnectionRequest {
    id: String,
    client_name: String,
    claim_hash: String,
    code: String,
    scopes: BTreeSet<String>,
    expires: u64,
    status: Status,
    project: Option<Project>,
    resource: String,
}

#[derive(Default)]
pub(super) struct Requests(BTreeMap<String, ConnectionRequest>);
impl Requests {
    pub(super) fn clear(&mut self) {
        self.0.clear();
    }
}

impl OAuthService {
    /// Nonsecret wake-up signal only. The window still reads and decides through the API.
    pub fn has_pending_connections(&self) -> bool {
        self.ready()
            && self.state.lock().is_ok_and(|state| {
                state
                    .connections
                    .0
                    .values()
                    .any(|request| request.expires > now() && request.status == Status::Pending)
            })
    }
}

pub(super) fn router() -> Router<Arc<OAuthService>> {
    Router::new()
        .route("/api/v1/connections/requests", post(create).get(list))
        .route("/api/v1/connections/requests/{id}/claim", post(claim))
        .route("/api/v1/connections/requests/{id}/decision", post(decision))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Create {
    client_name: String,
    scopes: Option<Vec<String>>,
    project_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Claim {
    claim_secret: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Decision {
    decision: String,
    #[serde(default)]
    scopes: Vec<String>,
}

async fn body<T: serde::de::DeserializeOwned>(request: Request) -> Result<T, Response> {
    if request.uri().query().is_some() || !content_type(request.headers(), "application/json") {
        return Err(invalid());
    }
    let bytes = to_bytes(request.into_body(), BODY_LIMIT)
        .await
        .map_err(|_| invalid())?;
    serde_json::from_slice(&bytes).map_err(|_| invalid())
}

fn requested_scopes(values: Option<Vec<String>>) -> Result<BTreeSet<String>, Response> {
    let values = values.unwrap_or_else(|| DEFAULT_SCOPES.into_iter().map(str::to_owned).collect());
    let scopes: BTreeSet<_> = values.into_iter().collect();
    if scopes.is_empty() || scopes.iter().any(|scope| !SCOPES.contains(&scope.as_str())) {
        return Err(invalid());
    }
    Ok(scopes)
}

async fn create(State(service): State<Arc<OAuthService>>, request: Request) -> Response {
    if !service.ready() {
        return unavailable();
    }
    let input: Create = match body(request).await {
        Ok(input) => input,
        Err(error) => return error,
    };
    if input.client_name.trim().is_empty()
        || input.client_name.len() > 128
        || input.client_name.chars().any(char::is_control)
    {
        return invalid();
    }
    let scopes = match requested_scopes(input.scopes) {
        Ok(scopes) => scopes,
        Err(error) => return error,
    };
    let resource=match service.resource_for_project(input.project_id.as_deref()) {
        Ok(resource)=>resource,
        Err(failure)=>return app_json(StatusCode::from_u16(failure.http_status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE),failure.body()),
    };
    let project=match service.project_for_resource(&resource) {Ok(project)=>project,Err(_)=>return unavailable()};
    if project.is_none() && scopes.contains("fetch:sessions:read") {return invalid();}
    let mut state = match service.state.lock() {
        Ok(state) => state,
        Err(_) => return unavailable(),
    };
    let timestamp = now();
    state
        .connections
        .0
        .retain(|_, request| request.expires > timestamp);
    if state.connections.0.len() >= 32 {
        return admission_busy();
    }
    let claim_secret = random_token();
    let value = ConnectionRequest {
        id: random_token(),
        client_name: input.client_name,
        claim_hash: digest(&claim_secret),
        code: format!("{:06}", rand::random::<u32>() % 1_000_000),
        scopes,
        expires: timestamp + APPROVAL_SECONDS,
        status: Status::Pending,
        project,
        resource,
    };
    let mut reply = safe_request(&value);
    reply["claim_secret"] = json!(claim_secret);
    reply["next_steps"] = json!([]);
    state.connections.0.insert(value.id.clone(), value);
    service.connection_requested.store(true, Ordering::Release);
    app_json(StatusCode::ACCEPTED, reply)
}

async fn list(State(service): State<Arc<OAuthService>>, request: Request) -> Response {
    if !service.ready() {
        return unavailable();
    }
    if service
        .native
        .authorize_settings_headers(request.headers())
        .is_none()
    {
        return native_required();
    }
    if request.uri().query().is_some() {
        return invalid();
    }
    let bytes = match to_bytes(request.into_body(), BODY_LIMIT).await {
        Ok(bytes) => bytes,
        Err(_) => return invalid(),
    };
    if !bytes.is_empty() {
        return invalid();
    }
    let state = match service.state.lock() {
        Ok(state) => state,
        Err(_) => return unavailable(),
    };
    let requests: Vec<_> = state
        .connections
        .0
        .values()
        .filter(|request| request.expires > now())
        .map(safe_request)
        .collect();
    app_json(
        StatusCode::OK,
        json!({"status":"ok", "requests":requests, "next_steps":[]}),
    )
}

async fn claim(
    State(service): State<Arc<OAuthService>>,
    Path(id): Path<String>,
    request: Request,
) -> Response {
    if !service.ready() {
        return unavailable();
    }
    let input: Claim = match body(request).await {
        Ok(input) => input,
        Err(error) => return error,
    };
    let mut state = match service.state.lock() {
        Ok(state) => state,
        Err(_) => return unavailable(),
    };
    let Some(request) = state
        .connections
        .0
        .get(&id)
        .filter(|request| {
            valid_token(&input.claim_secret) && request.claim_hash == digest(&input.claim_secret)
        })
        .cloned()
    else {
        return missing();
    };
    if request.expires <= now() {
        return expired();
    }
    let scopes = match request.status {
        Status::Pending | Status::Confirming(_) => {
            return app_json(
                StatusCode::ACCEPTED,
                json!({"status":"pending", "request_id":id, "poll_interval_seconds":2, "expires_at":request.expires, "next_steps":[]}),
            );
        }
        Status::Denied => return denied(),
        Status::Approved(scopes) => scopes,
    };
    if service.project_for_resource(&request.resource).is_err(){return unavailable();}
    let timestamp = now();
    let mut staged = state.persistent.clone();
    prune(&mut staged, timestamp);
    if staged.clients.len() >= 256
        || staged.families.len() >= 4096
        || staged
            .families
            .values()
            .filter(|family| family_active(family, timestamp))
            .count()
            >= 256
    {
        return admission_busy();
    }
    let client_id = random_token();
    let client = Client {
        id: client_id.clone(),
        name: request.client_name,
        redirect_uris: vec![],
        grant_types: ["refresh_token".to_owned()].into_iter().collect(),
        created: timestamp,
    };
    staged.clients.insert(client_id.clone(), client);
    let (access, refresh) = issue_family(
        &mut staged,
        &client_id,
        &request.resource,
        &scopes,
        true,
        timestamp,
    );
    if service.persist(&staged).is_err() {
        return unavailable();
    }
    state.persistent = staged;
    state.connections.0.remove(&id);
    app_json(
        StatusCode::OK,
        json!({"status":"approved", "client_id":client_id, "access_token":access, "refresh_token":refresh.expect("renewable grant"), "token_type":"Bearer", "expires_in":ACCESS_SECONDS, "scope":scopes.iter().cloned().collect::<Vec<_>>().join(" "), "resource":request.resource,"project_id":request.project.as_ref().map(|project|project.id.as_str()),"next_steps":[]}),
    )
}

async fn decision(
    State(service): State<Arc<OAuthService>>,
    Path(id): Path<String>,
    request: Request,
) -> Response {
    if !service.ready() {
        return unavailable();
    }
    let headers = request.headers().clone();
    if service
        .native
        .authorize_settings_headers(&headers)
        .is_none()
    {
        return native_required();
    }
    let input: Decision = match body(request).await {
        Ok(input) => input,
        Err(error) => return error,
    };
    if !matches!(input.decision.as_str(), "allow" | "deny") {
        return invalid();
    }
    let selected: BTreeSet<_> = input.scopes.into_iter().collect();
    let pending = {
        let mut state = match service.state.lock() {
            Ok(state) => state,
            Err(_) => return unavailable(),
        };
        let Some(pending) = state.connections.0.get_mut(&id) else {
            return missing();
        };
        if pending.expires <= now() {
            return expired();
        }
        if pending.status != Status::Pending {
            return admission_busy();
        }
        if !selected.is_subset(&pending.scopes) {
            return invalid();
        }
        if input.decision == "deny" || selected.is_empty() {
            pending.status = Status::Denied;
            return app_json(
                StatusCode::OK,
                json!({"status":"denied", "request_id":id, "next_steps":[]}),
            );
        }
        pending.status = Status::Confirming(selected.clone());
        pending.clone()
    };
    let remaining = pending.expires.saturating_sub(now()).min(120);
    let budget = std::time::Duration::from_secs(remaining);
    let destination=pending.project.as_ref().map(|project|format!("project {} ({})",project.name,project.folder_path.display())).unwrap_or_else(||"Shared servers".to_owned());
    let reason = format!("Allow {} to connect to Fetch for {} with the permissions selected in its approval popup.",pending.client_name,destination);
    let confirmation = tokio::time::timeout(
        budget,
        service
            .native
            .confirm_presence_with_timeout(&reason, budget),
    )
    .await;
    let mut state = match service.state.lock() {
        Ok(state) => state,
        Err(_) => return unavailable(),
    };
    let Some(current) = state.connections.0.get_mut(&id) else {
        return missing();
    };
    if current.expires <= now() {
        current.status = Status::Denied;
        return expired();
    }
    if current.status != Status::Confirming(selected.clone())
        || current.claim_hash != pending.claim_hash
    {
        return missing();
    }
    if service
        .native
        .authorize_settings_headers(&headers)
        .is_none()
    {
        current.status = Status::Pending;
        return native_required();
    }
    match confirmation {
        Ok(Ok(())) => {
            current.status = Status::Approved(selected);
            app_json(
                StatusCode::OK,
                json!({"status":"approved", "request_id":id, "next_steps":[]}),
            )
        }
        Ok(Err(NativeAuthError::Unavailable)) => {
            current.status = Status::Pending;
            error(
                StatusCode::SERVICE_UNAVAILABLE,
                "NATIVE_CONFIRMATION_UNAVAILABLE",
                "Bring Fetch forward and choose Allow again. No connection was approved.",
            )
        }
        _ => {
            current.status = Status::Denied;
            denied()
        }
    }
}

fn safe_request(request: &ConnectionRequest) -> Value {
    let status = match request.status {
        Status::Pending => "pending",
        Status::Confirming(_) => "confirming",
        Status::Approved(_) => "approved",
        Status::Denied => "denied",
    };
    json!({"request_id":request.id, "client_name":request.client_name, "code":request.code, "scopes":request.scopes, "status":status, "expires_at":request.expires,"project_id":request.project.as_ref().map(|project|project.id.as_str()),"project":request.project,"resource":request.resource})
}
fn app_json(status: StatusCode, value: Value) -> Response {
    standard_json(status, value)
}
fn error(status: StatusCode, code: &str, message: &str) -> Response {
    app_json(
        status,
        json!({"status":"failed", "error":{"code":code,"message":message}, "next_steps":[]}),
    )
}
fn invalid() -> Response {
    error(
        StatusCode::BAD_REQUEST,
        "INVALID_REQUEST",
        "Supply the connection request fields and only supported permissions.",
    )
}
fn missing() -> Response {
    error(
        StatusCode::NOT_FOUND,
        "CONNECTION_REQUEST_NOT_FOUND",
        "The connection request was not found.",
    )
}
fn expired() -> Response {
    error(
        StatusCode::GONE,
        "CONNECTION_REQUEST_EXPIRED",
        "This connection request expired. Start setup again.",
    )
}
fn denied() -> Response {
    error(
        StatusCode::FORBIDDEN,
        "CONNECTION_DENIED",
        "The connection was not approved.",
    )
}
fn native_required() -> Response {
    error(
        StatusCode::UNAUTHORIZED,
        "UNAUTHENTICATED",
        "Open Fetch to review connection requests.",
    )
}
fn unavailable() -> Response {
    error(
        StatusCode::SERVICE_UNAVAILABLE,
        "CONNECTION_AUTHORIZATION_UNAVAILABLE",
        "Fetch could not safely authorize this connection. Existing credentials were preserved.",
    )
}
fn admission_busy() -> Response {
    let mut response = error(
        StatusCode::TOO_MANY_REQUESTS,
        "CONNECTION_REQUEST_BUSY",
        "Fetch cannot accept another connection request yet.",
    );
    response
        .headers_mut()
        .insert(header::RETRY_AFTER, HeaderValue::from_static("5"));
    response
}
