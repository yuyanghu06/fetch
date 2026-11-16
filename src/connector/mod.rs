//! Raw stdio ↔ the existing local HTTPS MCP endpoint. Approval may open Fetch;
//! connector exit never owns app/server shutdown.
pub mod profile;
use crate::runner::{CancelToken, LEGACY, MODERN, SseDecoder, catalog, raw};
use futures_util::StreamExt;
use reqwest::{Client, Response};
use serde_json::{Value, json, value::RawValue};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    process::Stdio,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    sync::{Mutex, RwLock, mpsc},
};

#[cfg(test)]
#[path = "../../tests/connector_support/checks.rs"]
mod integration_tests;

const MAX_MESSAGE: usize = 16 * 1024 * 1024;
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,serde::Serialize,serde::Deserialize)]
pub struct ConnectionContext {
    pub client: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub project_id: Option<String>,
}
impl ConnectionContext {
    pub fn global(client:String)->Self { Self {client,project_id:None} }
    pub fn validate(&self)->Result<(),String> {
        if self.client.trim().is_empty()||self.client.len()>128||self.client.chars().any(char::is_control)
            ||self.project_id.as_deref().is_some_and(|id|!crate::projects::valid_project_id(id)) {
            return Err("Choose a client connection name and a registered Fetch project ID.".into());
        }
        Ok(())
    }
    pub fn resource(&self,origin:&str)->String {
        match &self.project_id {Some(id)=>format!("{origin}/mcp/projects/{id}"),None=>format!("{origin}/mcp")}
    }
}
#[derive(Clone)]
pub struct Options {
    pub root: PathBuf,
    pub port: u16,
    pub client: String,
    pub project_id: Option<String>,
}
impl Options {
    pub fn origin(&self) -> String {
        format!("https://localhost:{}", self.port)
    }
    pub fn context(&self)->ConnectionContext {ConnectionContext {client:self.client.clone(),project_id:self.project_id.clone()}}
    pub fn resource(&self)->String {self.context().resource(&self.origin())}
    pub fn endpoint(&self)->String {self.resource()}
}

pub fn pinned_client(options: &Options) -> Result<Client, String> {
    options.context().validate()?;
    if !options.root.is_absolute() || options.port < 1024 {
        return Err("Choose an absolute Fetch data folder and its HTTPS port.".into());
    }
    let path = options.root.join("certificate.pem");
    let metadata=std::fs::symlink_metadata(&path).map_err(|_|"Fetch is unavailable or its saved certificate is missing. Open Fetch before connecting.".to_owned())?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("Fetch's saved certificate is not a regular file.".into());
    }
    let certificate = std::fs::read(&path)
        .map_err(|_| "Fetch's saved certificate could not be read.".to_owned())?;
    let certificate = reqwest::Certificate::from_pem(&certificate)
        .map_err(|_| "Fetch's saved certificate could not be read.".to_owned())?;
    Client::builder()
        .https_only(true)
        .no_proxy()
        .tls_built_in_root_certs(false)
        .add_root_certificate(certificate)
        .resolve(
            "localhost",
            std::net::SocketAddr::from(([127, 0, 0, 1], options.port)),
        )
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .map_err(|_| "Fetch connector could not prepare its verified HTTPS connection.".into())
}

pub async fn run(options: Options) -> Result<(), String> {
    let store = profile::ProfileStore::for_context(options.root.clone(), options.context())?;
    ensure_approved(&options, &store).await?;
    run_with_io(options, store, tokio::io::stdin(), tokio::io::stdout()).await
}

/// The transport entry point used by real stdio and isolated duplex fixtures.
pub async fn run_with_io<R, W>(
    options: Options,
    store: profile::ProfileStore,
    input: R,
    mut output: W,
) -> Result<(), String>
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let http = pinned_client(&options)?;
    // Probe the actual endpoint with its approved credential only when handling
    // MCP. Reading a saved grant never creates authority or a backend process.
    if store.read().await?.is_none() {
        return Err(format!(
            "Fetch is waiting for approval for {}. Approve the popup in Fetch, then reconnect the coding client.",
            options.client
        ));
    }
    let (write, mut writes) = mpsc::channel::<Vec<u8>>(128);
    let (fatal, mut failures) = mpsc::unbounded_channel::<String>();
    let writer_failure = fatal.clone();
    let writer = tokio::spawn(async move {
        let result = async move {
            while let Some(message) = writes.recv().await {
                output
                    .write_all(&message)
                    .await
                    .map_err(|_| "The coding client closed its MCP output.".to_owned())?;
                output
                    .write_all(b"\n")
                    .await
                    .map_err(|_| "The coding client closed its MCP output.".to_owned())?;
                output
                    .flush()
                    .await
                    .map_err(|_| "The coding client closed its MCP output.".to_owned())?;
            }
            Ok::<(), String>(())
        }
        .await;
        if let Err(error) = &result {
            let _ = writer_failure.send(error.clone());
        }
        result
    });
    let bridge = Arc::new(Bridge {
        http,
        origin: options.origin(),
        endpoint: options.endpoint(),
        store,
        session: RwLock::new(None),
        revision: RwLock::new(LEGACY.into()),
        mapping: Mutex::new(Mapping::default()),
        tools: Mutex::new(BTreeMap::new()),
        sequence: AtomicU64::new(0),
        prefix: format!("fetch-connector-{:032x}", rand::random::<u128>()),
        write,
        fatal,
        closed: CancelToken::new(),
        changes_started: AtomicBool::new(false),
    });
    let mut tasks = tokio::task::JoinSet::new();
    let mut reader = BufReader::new(input);
    let mut line = Vec::new();
    let result = loop {
        line.clear();
        let mut limited = (&mut reader).take((MAX_MESSAGE + 1) as u64);
        let read = tokio::select! {
            failure=failures.recv()=>break Err(failure.unwrap_or_else(||"Fetch connector connection ended.".into())),
            read=limited.read_until(b'\n',&mut line)=>read,
        };
        match read {
            Ok(0) => break Ok(()),
            Ok(_) if line.len() <= MAX_MESSAGE => {}
            Ok(_) => {
                break Err(
                    "The coding client's MCP message exceeds Fetch's supported message size."
                        .into(),
                );
            }
            Err(_) => break Err("The coding client's MCP input could not be read.".into()),
        }
        if line.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        let message = match raw::parse(&line) {
            Ok(message) => message,
            Err(_) => break Err("The coding client sent an unreadable MCP message.".into()),
        };
        let method = raw::text(&message, "method");
        let is_initialize = method.as_deref() == Some("initialize");
        let initialized = method.as_deref() == Some("notifications/initialized");
        let prepared = match bridge.prepare(message).await {
            Ok(value) => value,
            Err(error) => break Err(error),
        };
        if is_initialize || initialized {
            if let Err(error) = bridge.clone().post(prepared).await {
                break Err(error);
            }
            if initialized {
                bridge.start_changes(&mut tasks).await;
            }
        } else {
            let task = bridge.clone();
            tasks.spawn(async move {
                if let Err(error) = task.clone().post(prepared).await {
                    let _ = task.fatal.send(error);
                }
            });
        }
        while let Some(completed) = tasks.try_join_next() {
            if completed.is_err() {
                let _ = bridge
                    .fatal
                    .send("Fetch connector forwarding task ended unexpectedly.".into());
            }
        }
    };
    bridge.closed.cancel();
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
    let _ = tokio::time::timeout(Duration::from_secs(2), bridge.close_session()).await;
    if let Err(error) = &result {
        bridge.fail_outstanding(error).await;
    }
    drop(bridge);
    match tokio::time::timeout(Duration::from_secs(2), writer).await {
        Ok(Ok(Ok(()))) => result,
        Ok(Ok(Err(error))) => Err(error),
        _ => Err("Fetch connector could not finish its MCP output.".into()),
    }
}

#[derive(Default)]
struct Mapping {
    outgoing: BTreeMap<String, (Box<RawValue>, String)>,
    incoming: BTreeMap<String, String>,
    cancelled: BTreeSet<String>,
    dispatched: BTreeMap<String, CancelToken>,
    requests: BTreeMap<String, CancelToken>,
}
struct Bridge {
    http: Client,
    origin: String,
    endpoint: String,
    store: profile::ProfileStore,
    session: RwLock<Option<String>>,
    revision: RwLock<String>,
    mapping: Mutex<Mapping>,
    tools: Mutex<BTreeMap<String, Box<RawValue>>>,
    sequence: AtomicU64,
    prefix: String,
    write: mpsc::Sender<Vec<u8>>,
    fatal: mpsc::UnboundedSender<String>,
    closed: CancelToken,
    changes_started: AtomicBool,
}
impl Bridge {
    async fn prepare(&self, mut message: Box<RawValue>) -> Result<Box<RawValue>, String> {
        let method = raw::text(&message, "method");
        if let Some(method) = &method {
            if let Some(id) = raw::field(&message, "id") {
                let mut mapping = self.mapping.lock().await;
                if mapping.incoming.contains_key(id.get()) {
                    return Err("The coding client reused an active MCP request ID.".into());
                }
                let wire = raw::raw(&json!(format!(
                    "{}-{}",
                    self.prefix,
                    self.sequence.fetch_add(1, Ordering::Relaxed)
                )));
                mapping
                    .incoming
                    .insert(id.get().to_owned(), wire.get().to_owned());
                mapping
                    .outgoing
                    .insert(wire.get().to_owned(), (id, method.clone()));
                mapping
                    .dispatched
                    .insert(wire.get().to_owned(), CancelToken::new());
                mapping
                    .requests
                    .insert(wire.get().to_owned(), CancelToken::new());
                message = raw::change(&message, "id", wire)
                    .map_err(|_| "The coding client's MCP envelope is unreadable.".to_owned())?;
            }
            if method == "notifications/cancelled" {
                if let Some(params) = raw::field(&message, "params") {
                    if let Some(id) = raw::field(&params, "requestId") {
                        let mut mapping = self.mapping.lock().await;
                        let Some(wire) = mapping.incoming.get(id.get()) else {
                            // A late cancellation may never target another connector's IDs.
                            return raw::change(
                                &message,
                                "params",
                                raw::change(
                                    &params,
                                    "requestId",
                                    raw::raw(&json!(format!("{}-ended", self.prefix))),
                                )
                                .map_err(|_| "The MCP cancellation is unreadable.".to_owned())?,
                            )
                            .map_err(|_| "The MCP cancellation is unreadable.".into());
                        };
                        let wire = wire.clone();
                        mapping.cancelled.insert(wire.clone());
                        let wire: Box<RawValue> = serde_json::from_str(&wire)
                            .map_err(|_| "The MCP cancellation is unreadable.".to_owned())?;
                        message = raw::change(
                            &message,
                            "params",
                            raw::change(&params, "requestId", wire)
                                .map_err(|_| "The MCP cancellation is unreadable.".to_owned())?,
                        )
                        .map_err(|_| "The MCP cancellation is unreadable.".to_owned())?;
                    }
                }
            }
        }
        Ok(message)
    }
    async fn headers(
        &self,
        mut request: reqwest::RequestBuilder,
        message: Option<&RawValue>,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, String> {
        request = request
            .bearer_auth(token)
            .header("accept", "application/json, text/event-stream");
        let revision = self.revision.read().await.clone();
        if let Some(session) = self.session.read().await.as_ref() {
            request = request.header("mcp-session-id", session);
        }
        let method = message.and_then(|m| raw::text(m, "method"));
        if method.as_deref() != Some("initialize") {
            request = request.header("mcp-protocol-version", &revision);
        }
        if revision == MODERN {
            if let Some(method) = method {
                request = request.header("mcp-method", &method);
                if let Some(params) = message.and_then(|m| raw::field(m, "params")) {
                    if matches!(
                        method.as_str(),
                        "tools/call" | "prompts/get" | "resources/read"
                    ) {
                        let key = if method == "resources/read" {
                            "uri"
                        } else {
                            "name"
                        };
                        if let Some(name) = raw::text(&params, key) {
                            request = request.header("mcp-name", catalog::encode_header(&name));
                            if method == "tools/call" {
                                if let Some(tool) = self.tools.lock().await.get(&name) {
                                    let arguments = raw::field(&params, "arguments")
                                        .unwrap_or_else(|| raw::raw(&json!({})));
                                    for (key,value) in catalog::mirrored_headers(tool,&arguments).map_err(|_|"Fetch connector could not mirror the tool's declared MCP headers.".to_owned())?{request=request.header(key,value);}
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(request)
    }
    async fn post(self: Arc<Self>, message: Box<RawValue>) -> Result<(), String> {
        let method = raw::text(&message, "method");
        if method.as_deref() == Some("notifications/cancelled") {
            let modern = self.revision.read().await.as_str() == MODERN;
            if let Some(id) =
                raw::field(&message, "params").and_then(|params| raw::field(&params, "requestId"))
            {
                if modern {
                    if let Some(request) = self.mapping.lock().await.requests.get(id.get()) {
                        request.cancel();
                    }
                    // Modern HTTP cancellation closes only this request's
                    // stream. The stdio notification is not an HTTP method.
                    return Ok(());
                }
                let dispatched = self.mapping.lock().await.dispatched.get(id.get()).cloned();
                if let Some(dispatched) = dispatched {
                    // The backend registers a call before returning its HTTP
                    // headers. A cancellation arriving before those headers
                    // would otherwise be silently lost while the POST starts.
                    tokio::select! {_=dispatched.cancelled()=>{},_=self.closed.cancelled()=>return Ok(())}
                }
            }
            if modern {
                return Ok(());
            }
        }
        if method.as_deref() == Some("server/discover") {
            *self.revision.write().await = MODERN.into();
        }
        let wire_id = raw::field(&message, "id");
        let request_cancel = wire_id.as_deref().map(|id| id.get().to_owned());
        let request_cancel = if let Some(id) = request_cancel {
            self.mapping
                .lock()
                .await
                .requests
                .get(&id)
                .cloned()
                .unwrap_or_default()
        } else {
            CancelToken::new()
        };
        let mut token = self
            .store
            .access_token(&self.http, &self.origin, None)
            .await?;
        let endpoint = self.endpoint.clone();
        let mut request = self
            .headers(self.http.post(&endpoint), Some(&message), &token)
            .await?
            .header("content-type", "application/json")
            .body(message.get().to_owned());
        let mut response = tokio::select! {biased;
            _=request_cancel.cancelled()=>{self.retire_request(wire_id.as_deref()).await;return Ok(())},
            _=self.closed.cancelled()=>return Ok(()),
            response=request.send()=>response.map_err(|_|"Fetch is unavailable or its HTTPS certificate does not match this installation. No tool call was replayed.".to_owned())?
        };
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            // The backend checks authority before dispatch. This rejection is
            // therefore safe to retry once after coordinated credential renewal.
            token = self
                .store
                .access_token(&self.http, &self.origin, Some(&token))
                .await?;
            request = self
                .headers(self.http.post(&endpoint), Some(&message), &token)
                .await?
                .header("content-type", "application/json")
                .body(message.get().to_owned());
            response = tokio::select! {biased;
                _=request_cancel.cancelled()=>{self.retire_request(wire_id.as_deref()).await;return Ok(())},
                _=self.closed.cancelled()=>return Ok(()),
                response=request.send()=>response.map_err(|_|"Fetch became unavailable after renewing this connection; no dispatched call was replayed.".to_owned())?
            };
        }
        if method.as_deref() == Some("initialize") {
            if let Some(session) = response
                .headers()
                .get("mcp-session-id")
                .and_then(|value| value.to_str().ok())
            {
                *self.session.write().await = Some(session.to_owned());
            }
        }
        if let Some(id) = raw::field(&message, "id") {
            if let Some(dispatched) = self.mapping.lock().await.dispatched.get(id.get()) {
                dispatched.cancel();
            }
        }
        tokio::select! {biased;
            _=request_cancel.cancelled()=>{self.retire_request(wire_id.as_deref()).await;Ok(())},
            _=self.closed.cancelled()=>Ok(()),
            result=self.consume(response,Some(&message),false)=>result
        }
    }
    async fn retire_request(&self, id: Option<&RawValue>) {
        let Some(id) = id else { return };
        let mut mapping = self.mapping.lock().await;
        if let Some((original, _)) = mapping.outgoing.remove(id.get()) {
            mapping.incoming.remove(original.get());
        }
        mapping.cancelled.remove(id.get());
        mapping.dispatched.remove(id.get());
        mapping.requests.remove(id.get());
    }
    async fn consume(
        &self,
        response: Response,
        request: Option<&RawValue>,
        changes: bool,
    ) -> Result<(), String> {
        let status = response.status();
        if status == reqwest::StatusCode::ACCEPTED || status == reqwest::StatusCode::NO_CONTENT {
            return Ok(());
        }
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        if content_type.split(';').next().unwrap_or("").trim() == "text/event-stream"
            && status.is_success()
        {
            let mut decoder = SseDecoder::default();
            let mut stream = response.bytes_stream();
            loop {
                let next = tokio::select! {_=self.closed.cancelled()=>return Ok(()),next=stream.next()=>next};
                let Some(next) = next else { break };
                let bytes = next.map_err(|_| {
                    "Fetch's MCP event stream ended before its response was complete.".to_owned()
                })?;
                for (event, data) in decoder
                    .push(&bytes)
                    .map_err(|_| "Fetch's MCP event stream could not be read.".to_owned())?
                {
                    if event.is_empty() || event == "message" {
                        if let Ok(message) = raw::parse(data.as_bytes()) {
                            if let Some(error) = self
                                .unattributed_error(&message, Some(status.as_u16()), request)
                                .await
                            {
                                return Err(error);
                            }
                        }
                        self.receive(data.as_bytes()).await?;
                    }
                }
            }
            if changes && !self.closed.is_cancelled() {
                return Err("Fetch's notification connection ended. Reconnect the coding client after Fetch is available.".into());
            }
            if let Some(request_id) = request.and_then(|r| raw::field(r, "id")) {
                let mut mapping = self.mapping.lock().await;
                if mapping.cancelled.remove(request_id.get()) {
                    mapping.dispatched.remove(request_id.get());
                    mapping.requests.remove(request_id.get());
                    if let Some((original, _)) = mapping.outgoing.remove(request_id.get()) {
                        mapping.incoming.remove(original.get());
                    }
                } else if mapping.outgoing.contains_key(request_id.get()) {
                    return Err("Fetch's MCP stream ended without a final response. The call outcome may be unknown; it was not replayed.".into());
                }
            }
            return Ok(());
        }
        let mut bytes = Vec::new();
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| {
                "Fetch's MCP response ended before it was complete. The call was not replayed."
                    .to_owned()
            })?;
            if bytes.len().saturating_add(chunk.len()) > MAX_MESSAGE {
                return Err("Fetch's MCP response exceeds the supported message size.".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        if !bytes.is_empty() {
            if let Ok(message) = raw::parse(&bytes) {
                if raw::text(&message, "jsonrpc").as_deref() == Some("2.0") {
                    if let Some(error) = self
                        .unattributed_error(&message, Some(status.as_u16()), request)
                        .await
                    {
                        return Err(error);
                    }
                    self.receive(message.get().as_bytes()).await?;
                    return Ok(());
                }
            }
        }
        let details = String::from_utf8_lossy(&bytes);
        Err(format!(
            "Fetch returned HTTP {}{}{}. No tool call was replayed.",
            status.as_u16(),
            if details.is_empty() { "" } else { ": " },
            details
        ))
    }
    async fn receive(&self, bytes: &[u8]) -> Result<(), String> {
        let mut message = raw::parse(bytes)
            .map_err(|_| "Fetch returned an unreadable MCP message.".to_owned())?;
        if let Some(error) = self.unattributed_error(&message, None, None).await {
            return Err(error);
        }
        message = self.restore_subscription_reference(message).await?;
        if raw::text(&message, "method").is_none() {
            if let Some(wire_id) = raw::field(&message, "id") {
                let mut mapping = self.mapping.lock().await;
                let Some((original, method)) = mapping.outgoing.remove(wire_id.get()) else {
                    return Err(
                        "Fetch returned a response for another connection or an ended request."
                            .into(),
                    );
                };
                mapping.cancelled.remove(wire_id.get());
                mapping.dispatched.remove(wire_id.get());
                mapping.requests.remove(wire_id.get());
                mapping.incoming.remove(original.get());
                drop(mapping);
                if let Some(result) = raw::field(&message, "result") {
                    if method == "initialize" {
                        if let Some(revision) = raw::text(&result, "protocolVersion") {
                            *self.revision.write().await = revision;
                        }
                    }
                    if method == "tools/list" {
                        if let Some(tools) = raw::field(&result, "tools") {
                            if let Ok(tools) =
                                serde_json::from_str::<Vec<Box<RawValue>>>(tools.get())
                            {
                                let mut cache = self.tools.lock().await;
                                for tool in tools {
                                    if let Some(name) = raw::text(&tool, "name") {
                                        cache.insert(name, tool);
                                    }
                                }
                            }
                        }
                    }
                }
                message = raw::change(&message, "id", original)
                    .map_err(|_| "Fetch's MCP response envelope is unreadable.".to_owned())?;
            }
        }
        self.write
            .send(message.get().as_bytes().to_vec())
            .await
            .map_err(|_| "The coding client closed its MCP output.".into())
    }
    /// Translate only transport-owned subscription correlation fields. Tool
    /// results, structured content and extension metadata stay raw and untouched.
    async fn restore_subscription_reference(
        &self,
        mut message: Box<RawValue>,
    ) -> Result<Box<RawValue>, String> {
        const KEY: &str = "io.modelcontextprotocol/subscriptionId";
        let section = if raw::text(&message, "method")
            .is_some_and(|method| method.starts_with("notifications/"))
        {
            "params"
        } else {
            let Some(id) = raw::field(&message, "id") else {
                return Ok(message);
            };
            if !self
                .mapping
                .lock()
                .await
                .outgoing
                .get(id.get())
                .is_some_and(|(_, method)| method == "subscriptions/listen")
            {
                return Ok(message);
            }
            "result"
        };
        let Some(body) = raw::field(&message, section) else {
            return Ok(message);
        };
        let Some(meta) = raw::field(&body, "_meta") else {
            return Ok(message);
        };
        let Some(reference) = raw::field(&meta, KEY) else {
            return Ok(message);
        };
        let original = {
            let mapping = self.mapping.lock().await;
            let Some((original, method)) = mapping.outgoing.get(reference.get()) else {
                return Err("Fetch returned a subscription reference for another connection or an ended request.".into());
            };
            if method != "subscriptions/listen"
                || (section == "result"
                    && raw::field(&message, "id").is_none_or(|id| id.get() != reference.get()))
            {
                return Err("Fetch returned a subscription reference that does not match its subscription request.".into());
            }
            original.clone()
        };
        let meta = raw::change(&meta, KEY, original)
            .map_err(|_| "Fetch's subscription metadata is unreadable.".to_owned())?;
        let body = raw::change(&body, "_meta", meta)
            .map_err(|_| "Fetch's subscription metadata is unreadable.".to_owned())?;
        message = raw::change(&message, section, body)
            .map_err(|_| "Fetch's subscription envelope is unreadable.".to_owned())?;
        Ok(message)
    }
    /// Notifications have no request ID. A backend rejection with id:null is an
    /// uncorrelated RPC error, not evidence of a cross-connection response.
    async fn unattributed_error(
        &self,
        message: &RawValue,
        status: Option<u16>,
        request: Option<&RawValue>,
    ) -> Option<String> {
        if raw::text(message, "jsonrpc").as_deref() != Some("2.0")
            || raw::field(message, "method").is_some()
        {
            return None;
        }
        if raw::field(message, "id").is_some_and(|id| id.get().trim() != "null") {
            return None;
        }
        let error = raw::field(message, "error")?;
        let mut secrets = zeroize::Zeroizing::new(Vec::new());
        if let Ok(Some(grant)) = self.store.read().await {
            secrets.push(grant.access_token.clone());
            secrets.push(grant.refresh_token.clone());
        }
        let details = crate::runner::startup_diagnostics::sanitize_text(error.get(), &secrets);
        let mut context = Vec::new();
        if let Some(status) = status {
            context.push(format!("HTTP {status}"));
        }
        if let Some(method) = request.and_then(|request| raw::text(request, "method")) {
            context.push(format!("method {method}"));
        }
        let context = if context.is_empty() {
            String::new()
        } else {
            format!(" ({})", context.join(", "))
        };
        Some(format!("Fetch returned MCP error{context}: {details}"))
    }
    async fn start_changes(self: &Arc<Self>, tasks: &mut tokio::task::JoinSet<()>) {
        if self.changes_started.swap(true, Ordering::AcqRel) {
            return;
        }
        let bridge = self.clone();
        tasks.spawn(async move {
            let result = async {
                let token = bridge
                    .store
                    .access_token(&bridge.http, &bridge.origin, None)
                    .await?;
                let request = bridge
                    .headers(
                        bridge.http.get(&bridge.endpoint),
                        None,
                        &token,
                    )
                    .await?
                    .header("accept", "text/event-stream");
                let response = request.send().await.map_err(|_| {
                    "Fetch's MCP notification connection is unavailable.".to_owned()
                })?;
                bridge.consume(response, None, true).await
            }
            .await;
            if let Err(error) = result {
                if !bridge.closed.is_cancelled() {
                    let _ = bridge.fatal.send(error);
                }
            }
        });
    }
    async fn close_session(&self) {
        if self.session.read().await.is_none() {
            return;
        }
        // This deletes only the owned HTTP session. It is not logout, app
        // shutdown, server restart or shared upstream connection closure.
        if let Ok(Some(grant)) = self.store.read().await {
            if let Ok(request) = self
                .headers(
                    self.http.delete(&self.endpoint),
                    None,
                    &grant.access_token,
                )
                .await
            {
                let _ = tokio::time::timeout(Duration::from_secs(2), request.send()).await;
            }
        }
    }
    async fn fail_outstanding(&self, message: &str) {
        let mut mapping = self.mapping.lock().await;
        for (_, (id, _)) in std::mem::take(&mut mapping.outgoing) {
            let error = raw::raw(
                &json!({"jsonrpc":"2.0","id":id,"error":{"code":-32603,"message":format!("Fetch connector: {message}"),"data":{"source":"fetch_connector","execution":"unknown"}}}),
            );
            let _ = self.write.send(error.get().as_bytes().to_vec()).await;
        }
        mapping.incoming.clear();
        mapping.dispatched.clear();
        mapping.requests.clear();
    }
}

/// Initial connector startup requests approval automatically. Its durable proof
/// belongs to the native app even if the client startup deadline kills stdio.
async fn ensure_approved(options: &Options, store: &profile::ProfileStore) -> Result<(), String> {
    ensure_approved_with_launcher(options, store, || open_for_approval(options)).await
}

/// Injectable app-opening boundary for isolated source fixtures. Production
/// always uses open_for_approval; no CLI bypass or authority is introduced.
pub(crate) async fn ensure_approved_with_launcher<F, Fut>(
    options: &Options,
    store: &profile::ProfileStore,
    launcher: F,
) -> Result<(), String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Client, String>>,
{
    if store.read().await?.is_some() {
        return Ok(());
    }
    let http = launcher().await?;
    let Some(pending) = store.ensure_pending(&http, &options.origin()).await? else {
        return Ok(());
    };
    let _ = crate::application::show_connection_context(&options.root, &options.context()).await;
    eprintln!(
        "Approve {} in the Fetch popup. Match request code {}. If this client times out first, approve in Fetch and reconnect it.",
        options.client, pending.code
    );
    wait_for_approval(store, &options.origin(), &pending).await
}

/// Does not fabricate MCP initialization: caller input stays in its pipe until
/// the grant is saved, or a timed-out coding client starts a fresh connector.
pub async fn wait_for_approval(
    store: &profile::ProfileStore,
    origin: &str,
    pending: &profile::Pending,
) -> Result<(), String> {
    loop {
        if store.read().await?.is_some() {
            return Ok(());
        }
        let current = store.pending(origin).await?;
        if current.as_ref() != Some(pending) {
            return Err("Fetch connection approval ended without saved access. Reconnect the coding client to request approval again.".into());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

async fn open_for_approval(options: &Options) -> Result<Client, String> {
    if crate::application::show_connection_context(&options.root, &options.context())
        .await
        .is_err()
    {
        let executable = std::env::current_exe().map_err(|_| {
            "Fetch connector could not locate its application executable.".to_owned()
        })?;
        let mut command = std::process::Command::new(executable);
        command
            .arg("app")
            .arg("--data-dir")
            .arg(&options.root)
            .arg("--port")
            .arg(options.port.to_string())
            .arg("--connect-client")
            .arg(&options.client)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if let Some(project)=&options.project_id { command.arg("--connect-project").arg(project); }
        // Client supervisors may terminate an MCP process group at startup
        // timeout. Fetch must survive to finish the user's pending approval.
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        command
            .spawn()
            .map_err(|_| "Fetch could not be opened for connection approval.".to_owned())?;
    }
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    loop {
        if let Ok(client) = pinned_client(options) {
            if client
                .get(format!(
                    "{}/.well-known/oauth-authorization-server",
                    options.origin()
                ))
                .send()
                .await
                .is_ok_and(|r| r.status().is_success())
            {
                return Ok(client);
            }
        }
        if tokio::time::Instant::now() >= deadline {
            return Err("Fetch did not become available for approval. Open Fetch and inspect its startup error.".into());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

/// Optional explicit setup shares automatic startup's pending request. Only the
/// native app claims credentials; this process merely waits for saved access.
pub async fn setup(options: Options) -> Result<(), String> {
    let store = profile::ProfileStore::for_context(options.root.clone(), options.context())?;
    ensure_approved(&options, &store).await
}
fn api_error(value: &Value, status: u16) -> String {
    let code = value["error"]["code"]
        .as_str()
        .unwrap_or("CONNECTION_REQUEST_FAILED");
    let details = value["error"]
        .get("details")
        .filter(|v| !v.is_null())
        .map(|v| {
            format!(
                " Details: {}",
                crate::runner::startup_diagnostics::sanitize_text(&v.to_string(), &[])
            )
        })
        .unwrap_or_default();
    format!(
        "Fetch connection setup failed (HTTP {status}, {code}): {}{details}",
        value["error"]["message"]
            .as_str()
            .unwrap_or("The requested connection could not be approved.")
    )
}
