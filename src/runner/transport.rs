//! Persistent upstream connections. No provider credentials are logged or returned.
use super::{CancelToken, InvocationContext, RelayEvent, RunError, TransportConfig, raw};
use futures_util::StreamExt;
use reqwest::{Client, Response, header::{HeaderMap, HeaderName, HeaderValue}};
use serde::Deserialize;
use serde_json::{Value, json, value::RawValue};
use std::{collections::{BTreeMap, HashMap}, sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}}, time::Duration};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, process::{Child, Command}, sync::{Mutex, RwLock, broadcast, mpsc, oneshot}};

pub const LEGACY: &str = "2025-11-25";
pub const MODERN: &str = "2026-07-28";
pub const MAX_MESSAGE: usize = 64 * 1024 * 1024;

#[derive(Deserialize)]
struct Message {
    jsonrpc: Option<String>,
    id: Option<Box<RawValue>>,
    method: Option<String>,
    params: Option<Box<RawValue>>,
    result: Option<Box<RawValue>>,
    error: Option<Box<RawValue>>,
}
struct Pending {
    reply: oneshot::Sender<Result<Box<RawValue>, RunError>>,
    events: Option<mpsc::Sender<RelayEvent>>,
    progress_token: Option<Box<RawValue>>,
    capabilities: Value,
    method:String,
    cancel:CancelToken,
}
enum Wire {
    Stdio(mpsc::Sender<Vec<u8>>),
    Remote { client: Client, endpoint: String, headers: RwLock<HeaderMap>, sse: bool },
}
pub struct Connection {
    wire: Wire,
    sequence: AtomicU64,
    pending: Mutex<HashMap<String, Pending>>,
    session: RwLock<Option<String>>,
    pub revision: RwLock<String>,
    pub capabilities: RwLock<Value>,
    child: Mutex<Option<Child>>,
    pub closed: AtomicBool,
    pub changes: broadcast::Sender<String>,
    close: CancelToken,
    legacy_post_endpoint: RwLock<Option<String>>,
    background: Mutex<Vec<tokio::task::JoinHandle<()>>>,
}
pub struct RpcRequest {
    pub id: String,
    pub response: oneshot::Receiver<Result<Box<RawValue>, RunError>>,
}

impl Connection {
    pub async fn open(config: &TransportConfig) -> Result<Arc<Self>, RunError> {
        let (changes, _) = broadcast::channel(128);
        match config {
            TransportConfig::Stdio { executable, args, working_directory, environment } => {
                if !executable.is_absolute() { return Err(RunError::not_ready()); }
                let mut command = Command::new(executable);
                command.args(args).env_clear().envs(environment)
                    .stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped()).kill_on_drop(true);
                if let Some(directory) = working_directory { command.current_dir(directory); }
                // A dedicated process group permits owned descendant cleanup on Quit.
                #[cfg(unix)]
                { command.process_group(0); }
                let mut child = command.spawn().map_err(|_| RunError::not_ready())?;
                let mut input = child.stdin.take().ok_or_else(RunError::not_ready)?;
                let output = child.stdout.take().ok_or_else(RunError::not_ready)?;
                let error = child.stderr.take();
                let (writer, mut queue) = mpsc::channel::<Vec<u8>>(128);
                let connection = Arc::new(Self {
                    wire: Wire::Stdio(writer), sequence: AtomicU64::new(0), pending: Mutex::new(HashMap::new()),
                    session: RwLock::new(None), revision: RwLock::new(LEGACY.into()), capabilities: RwLock::new(json!({})),
                    child: Mutex::new(Some(child)), closed: AtomicBool::new(false), changes, close: CancelToken::new(),
                    legacy_post_endpoint: RwLock::new(None), background: Mutex::new(vec![]),
                });
                let mut opening=CloseOnDrop::new(connection.clone());
                let c = connection.clone();
                let writer_task = tokio::spawn(async move {
                    while let Some(bytes) = queue.recv().await {
                        if input.write_all(&bytes).await.is_err() || input.write_all(b"\n").await.is_err() || input.flush().await.is_err() { break; }
                    }
                    c.fail_closed().await;
                });
                let c = connection.clone();
                let reader_task = tokio::spawn(async move {
                    let mut reader = BufReader::new(output);
                    loop {
                        let mut line = Vec::new();
                        let read = (&mut reader).take((MAX_MESSAGE + 1) as u64).read_until(b'\n', &mut line).await;
                        if !matches!(read, Ok(n) if n > 0) || line.len() > MAX_MESSAGE { break; }
                        if line.iter().all(u8::is_ascii_whitespace) { continue; }
                        if c.receive(&line, None).await.is_err() { break; }
                    }
                    c.fail_closed().await;
                });
                let mut tasks = vec![writer_task, reader_task];
                if let Some(mut error) = error {
                    // Consume private stderr without retaining provider output in memory/logs.
                    tasks.push(tokio::spawn(async move { let mut buffer = [0u8; 4096]; while matches!(error.read(&mut buffer).await, Ok(n) if n > 0) {} }));
                }
                *connection.background.lock().await = tasks;
                opening.disarm();
                Ok(connection)
            }
            TransportConfig::Http { endpoint, headers } | TransportConfig::Sse { endpoint, headers } => {
                validate_endpoint(endpoint)?;
                let headers = private_headers(headers)?;
                let client = Client::builder().redirect(reqwest::redirect::Policy::none()).connect_timeout(Duration::from_secs(15))
                    .user_agent("Fetch/0.1.0").build().map_err(|_| RunError::not_ready())?;
                let connection = Arc::new(Self {
                    wire: Wire::Remote { client, endpoint: endpoint.clone(), headers: RwLock::new(headers), sse: matches!(config, TransportConfig::Sse { .. }) },
                    sequence: AtomicU64::new(0), pending: Mutex::new(HashMap::new()), session: RwLock::new(None),
                    revision: RwLock::new(LEGACY.into()), capabilities: RwLock::new(json!({})), child: Mutex::new(None),
                    closed: AtomicBool::new(false), changes, close: CancelToken::new(),
                    legacy_post_endpoint: RwLock::new(None), background: Mutex::new(vec![]),
                });
                let mut opening=CloseOnDrop::new(connection.clone());
                if matches!(config, TransportConfig::Sse { .. }) { connection.start_legacy_sse().await?; }
                opening.disarm();
                Ok(connection)
            }
        }
    }

    pub async fn setup(self: &Arc<Self>) -> Result<(), RunError> {
        // An initialize request is read-only setup. It is never a tool invocation.
        let params = raw::raw(&json!({"protocolVersion":LEGACY,"capabilities":{},"clientInfo":{"name":"Fetch","version":env!("CARGO_PKG_VERSION")}}));
        let request = self.start("initialize", params, &InvocationContext::terminal("fetch-internal".into()), BTreeMap::new()).await?;
        let result = tokio::time::timeout(Duration::from_secs(30), request.response).await
            .map_err(|_| RunError::not_ready())?.map_err(|_| RunError::not_ready())?;
        match result {
            Ok(result) => {
                let version = raw::text(&result, "protocolVersion").ok_or_else(RunError::not_ready)?;
                if version != LEGACY { return Err(RunError::unsupported(false)); }
                *self.capabilities.write().await = raw::field(&result, "capabilities").and_then(|v| raw::scalar(&v).ok()).unwrap_or_else(|| json!({}));
                self.notify("notifications/initialized", raw::raw(&json!({}))).await?;
                self.start_legacy_get().await;
                Ok(())
            }
            // Only a method/version compatibility signal permits switching era.
            Err(error) if error.code == "MCP_VERSION_UNSUPPORTED" => {
                if matches!(&self.wire, Wire::Remote { sse: true, .. }) { return Err(error); }
                *self.revision.write().await = MODERN.into();
                let request = self.start("server/discover", raw::raw(&json!({})), &InvocationContext::terminal("fetch-internal".into()), BTreeMap::new()).await?;
                let result = tokio::time::timeout(Duration::from_secs(30), request.response).await.map_err(|_| RunError::not_ready())?
                    .map_err(|_| RunError::not_ready())??;
                if raw::text(&result,"resultType").as_deref() != Some("complete") { return Err(RunError::not_ready()); }
                let supported=raw::field(&result,"supportedVersions").and_then(|v|serde_json::from_str::<Vec<String>>(v.get()).ok()).unwrap_or_default();
                if !supported.iter().any(|version|version==MODERN){return Err(RunError::unsupported(false));}
                *self.capabilities.write().await = raw::field(&result, "capabilities").and_then(|v| raw::scalar(&v).ok()).unwrap_or_else(|| json!({}));
                self.start_modern_changes().await;
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub async fn start(self: &Arc<Self>, method: &str, params: Box<RawValue>, context: &InvocationContext, mirrored: BTreeMap<String, String>) -> Result<RpcRequest, RunError> {
        if self.closed.load(Ordering::Acquire) { return Err(RunError::not_ready()); }
        let id = format!("fetch-{}", self.sequence.fetch_add(1, Ordering::Relaxed));
        let revision = self.revision.read().await.clone();
        let mut params = raw::object(&params).map_err(|_| RunError::invalid())?;
        let mut meta = params.remove("_meta").and_then(|v| raw::object(&v).ok()).unwrap_or_default();
        if context.progress_token.is_some() { meta.insert("progressToken".into(), raw::raw(&json!(id))); }
        if revision == MODERN {
            meta.insert("io.modelcontextprotocol/protocolVersion".into(), raw::raw(&json!(MODERN)));
            meta.insert("io.modelcontextprotocol/clientInfo".into(), raw::raw(&json!({"name":"Fetch","version":env!("CARGO_PKG_VERSION")})));
            // No union of legacy callers or claim of optional interaction support.
            meta.insert("io.modelcontextprotocol/clientCapabilities".into(), raw::raw(&context.capabilities));
        }
        if !meta.is_empty() { params.insert("_meta".into(), serde_json::value::to_raw_value(&meta).map_err(|_| RunError::invalid())?); }
        let params = serde_json::value::to_raw_value(&params).map_err(|_| RunError::invalid())?;
        let message = format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"method\":{},\"params\":{}}}", serde_json::to_string(&id).expect("string"),serde_json::to_string(method).expect("string"),params.get());
        if message.len() > MAX_MESSAGE { return Err(RunError::invalid()); }
        let (reply, receiver) = oneshot::channel();
        {
            let mut pending = self.pending.lock().await;
            if pending.len() >= 256 { return Err(RunError::busy()); }
            pending.insert(id.clone(), Pending { reply, events: context.events.clone(), progress_token: context.progress_token.clone(), capabilities: context.capabilities.clone(),method:method.to_owned(),cancel:CancelToken::new() });
        }
        let result = match &self.wire {
            Wire::Stdio(writer) => writer.send(message.into_bytes()).await.map_err(|_| RunError::not_ready()),
            Wire::Remote { .. } => {
                let c = self.clone(); let request_id = id.clone(); let method = method.to_owned();
                tokio::spawn(async move {
                    if let Err(error) = c.post(message.into_bytes(), Some(&method), Some(&request_id), mirrored).await {
                        if let Some(pending) = c.pending.lock().await.remove(&request_id) { let _ = pending.reply.send(Err(error)); }
                    }
                });
                Ok(())
            }
        };
        if let Err(error) = result { self.pending.lock().await.remove(&id); return Err(error); }
        Ok(RpcRequest { id, response: receiver })
    }

    pub async fn notify(self: &Arc<Self>, method: &str, params: Box<RawValue>) -> Result<(), RunError> {
        let message = format!("{{\"jsonrpc\":\"2.0\",\"method\":{},\"params\":{}}}",serde_json::to_string(method).expect("string"),params.get());
        self.send_reply(message.into_bytes()).await
    }
    fn send_reply<'a>(self: &'a Arc<Self>, bytes: Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output=Result<(),RunError>>+Send+'a>> {
        Box::pin(async move { match &self.wire {
            Wire::Stdio(writer) => writer.send(bytes).await.map_err(|_| RunError::not_ready()),
            Wire::Remote { .. } => self.post(bytes, None, None, BTreeMap::new()).await,
        } })
    }

    async fn post(self: &Arc<Self>, bytes: Vec<u8>, method: Option<&str>, parent: Option<&str>, mirrored: BTreeMap<String,String>) -> Result<(), RunError> {
        let Wire::Remote { client, endpoint, headers, sse } = &self.wire else { return Err(RunError::not_ready()); };
        let target = if *sse { self.legacy_post_endpoint.read().await.clone().ok_or_else(RunError::not_ready)? } else { endpoint.clone() };
        let revision = self.revision.read().await.clone();
        let mut request = client.post(&target).headers(headers.read().await.clone()).header("accept","application/json, text/event-stream").header("content-type","application/json").header("mcp-protocol-version", &revision);
        if revision == MODERN {
            if let Some(method) = method { request = request.header("mcp-method", method); }
            if let Ok(message) = serde_json::from_slice::<Message>(&bytes) {
                if let Some(params) = message.params {
                    if matches!(method, Some("tools/call" | "prompts/get" | "resources/read")) {
                        let selector = if method == Some("resources/read") { "uri" } else { "name" };
                        if let Some(name) = raw::text(&params, selector) { request = request.header("mcp-name", super::catalog::encode_header(&name)); }
                    }
                }
            }
            for (name,value) in mirrored { request = request.header(name, value); }
        } else if let Some(session) = self.session.read().await.as_ref() { request = request.header("mcp-session-id", session); }
        let request_cancel=if let Some(parent)=parent{self.pending.lock().await.get(parent).map(|pending|pending.cancel.clone()).unwrap_or_default()}else{CancelToken::new()};
        let response = tokio::select!{response=crate::diagnostics::send(crate::diagnostics::UpstreamOperation::McpHttp,request.body(bytes))=>response.map_err(|_|RunError::unavailable())?,_=request_cancel.cancelled()=>return Err(RunError::unavailable()),_=self.close.cancelled()=>return Err(RunError::unavailable())};
        if let Some(session) = response.headers().get("mcp-session-id").and_then(|v| v.to_str().ok()) {
            if session.len() > 256 || session.is_empty() || !session.bytes().all(|b| (0x21..=0x7e).contains(&b)) { return Err(RunError::unavailable()); }
            *self.session.write().await = Some(session.to_owned());
        }
        if !response.status().is_success() {
            let status = response.status().as_u16();
            // Inspect only a bounded protocol error code; never expose a response diagnostic.
            let bytes = tokio::select!{bytes=bounded_response(response)=>bytes?,_=request_cancel.cancelled()=>return Err(RunError::unavailable())};
            let protocol_error=serde_json::from_slice::<Value>(&bytes).ok().filter(|value|value["jsonrpc"]=="2.0"&&parent.is_some_and(|id|value["id"].as_str()==Some(id)));
            let error_code = protocol_error.as_ref().and_then(|v|v["error"]["code"].as_i64());
            return Err(if matches!(error_code,Some(-32601 | -32022)) && method == Some("initialize") {
                RunError::version()
            } else if status == 401 || status == 403 { RunError::provider_auth() } else if error_code.is_some(){RunError::rpc()}else { RunError::unavailable() });
        }
        if response.status().as_u16() == 202 || response.status().as_u16() == 204 { return Ok(()); }
        let content = response.headers().get("content-type").and_then(|v|v.to_str().ok()).unwrap_or("").to_owned();
        if content.starts_with("text/event-stream") {
            self.consume_sse(response, parent.map(str::to_owned), false).await
        } else if content.starts_with("application/json") {
            let bytes = tokio::select!{bytes=bounded_response(response)=>bytes?,_=request_cancel.cancelled()=>return Err(RunError::unavailable())};
            self.receive(&bytes, parent).await
        } else { Err(RunError::unavailable()) }
    }

    async fn receive(self: &Arc<Self>, bytes: &[u8], parent: Option<&str>) -> Result<(), RunError> {
        let original = raw::parse(bytes).map_err(|_| RunError::unavailable())?;
        let message: Message = serde_json::from_slice(bytes).map_err(|_| RunError::unavailable())?;
        if message.jsonrpc.as_deref() != Some("2.0") { return Err(RunError::unavailable()); }
        if let Some(method) = message.method {
            if let Some(request_id) = message.id {
                let params = message.params.unwrap_or_else(|| raw::raw(&json!({})));
                self.interaction(request_id, method, params, parent).await;
            } else if method == "notifications/progress" {
                if let Some(params) = message.params {
                    let token = raw::text(&params,"progressToken");
                    if let Some(token) = token {
                        let pending = self.pending.lock().await;
                        if let Some(pending) = pending.get(&token) {
                            if let (Some(events),Some(original_token)) = (&pending.events,&pending.progress_token) {
                                if let Ok(params) = raw::change(&params,"progressToken",original_token.clone()) {
                                    let notice = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/progress\",\"params\":{}}}",params.get());
                                    if let Ok(notice) = RawValue::from_string(notice) { let _ = events.try_send(RelayEvent::Notification(notice)); }
                                }
                            }
                        }
                    }
                }
            } else if matches!(method.as_str(), "notifications/tools/list_changed"|"notifications/resources/list_changed"|"notifications/prompts/list_changed") {
                let _ = self.changes.send(method);
            }
            return Ok(());
        }
        let Some(id) = message.id.and_then(|id| serde_json::from_str::<String>(id.get()).ok()) else { return Err(RunError::unavailable()); };
        if let Some(parent) = parent { if parent != id { return Err(RunError::unavailable()); } }
        let Some(pending) = self.pending.lock().await.remove(&id) else { return Ok(()); };
        let result = match (message.result,message.error) {
            (Some(result),None) if result.get().starts_with('{') => Ok(result),
            (None,Some(error)) => {
                let code = raw::field(&error,"code").and_then(|v|v.get().parse::<i64>().ok());
                Err(if pending.method=="initialize"&&matches!(code,Some(-32601 | -32022)) { RunError::version() } else { RunError::rpc() })
            }
            _ => Err(RunError::unavailable()),
        };
        let _ = pending.reply.send(result);
        drop(original);
        Ok(())
    }

    async fn interaction(self: &Arc<Self>, id: Box<RawValue>, method: String, params: Box<RawValue>, parent: Option<&str>) {
        // Stdio/standalone SSE have no standard parent request identity. Never guess.
        let pending = if let Some(parent) = parent {
            self.pending.lock().await.get(parent).map(|p|(p.events.clone(),p.capabilities.clone()))
        } else { None };
        let supported = pending.as_ref().is_some_and(|(_,caps)|super::interaction_supported(caps,&method,&params));
        let mut result = None;
        if supported {
            if let Some((Some(events),_)) = pending {
                let (reply, answer) = oneshot::channel();
                if events.send(RelayEvent::Request { id: id.clone(), method: method.clone(), params: params.clone(), reply }).await.is_ok() {
                    if let Ok(Ok(value)) = answer.await { result = Some(value); }
                }
            }
        }
        let message = match result {
            Some(result) => format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",id.get(),result.get()),
            None => format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32601,\"message\":\"This request has no supported owning client context.\"}}}}",id.get()),
        };
        // Spawning breaks recursive async transport receive/send types, with bounded
        // pending requests and one answer per received owned interaction.
        let c = self.clone(); tokio::spawn(async move { let _ = c.send_reply(message.into_bytes()).await; });
    }

    async fn start_legacy_sse(self: &Arc<Self>) -> Result<(), RunError> {
        let Wire::Remote { client, endpoint, headers, .. } = &self.wire else { return Err(RunError::not_ready()); };
        let response = crate::diagnostics::send(crate::diagnostics::UpstreamOperation::McpHttp,client.get(endpoint).headers(headers.read().await.clone()).header("accept","text/event-stream")).await.map_err(|_| RunError::not_ready())?;
        if !response.status().is_success() { return Err(RunError::not_ready()); }
        let c = self.clone();
        let task = tokio::spawn(async move { if c.consume_sse(response,None,true).await.is_err() { c.fail_closed().await; } });
        self.background.lock().await.push(task);
        let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
        loop {
            if self.legacy_post_endpoint.read().await.is_some() { return Ok(()); }
            if self.closed.load(Ordering::Acquire) || tokio::time::Instant::now() >= deadline { return Err(RunError::not_ready()); }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    async fn start_legacy_get(self: &Arc<Self>) {
        let Wire::Remote { client, endpoint, headers, sse:false } = &self.wire else { return; };
        let Some(session) = self.session.read().await.clone() else { return; };
        let client=client.clone(); let endpoint=endpoint.clone(); let headers=headers.read().await.clone(); let c=self.clone();
        let task=tokio::spawn(async move {
            if let Ok(response)=crate::diagnostics::send(crate::diagnostics::UpstreamOperation::McpHttp,client.get(endpoint).headers(headers).header("accept","text/event-stream").header("mcp-session-id",session).header("mcp-protocol-version",LEGACY)).await {
                if response.status().is_success() { let _=c.consume_sse(response,None,false).await; }
            }
        });
        self.background.lock().await.push(task);
    }
    async fn start_modern_changes(self:&Arc<Self>){
        let capabilities=self.capabilities.read().await.clone();let mut notifications=json!({});
        for (feature,filter) in [("tools","toolsListChanged"),("resources","resourcesListChanged"),("prompts","promptsListChanged")]{if capabilities[feature]["listChanged"]==true{notifications[filter]=json!(true);}}
        if notifications.as_object().is_none_or(|map|map.is_empty()){return;}
        if let Ok(request)=self.start("subscriptions/listen",raw::raw(&json!({"notifications":notifications})),&InvocationContext::terminal("fetch-catalog-changes".into()),BTreeMap::new()).await{
            let task=tokio::spawn(async move{let _=request.response.await;});self.background.lock().await.push(task);
        }
    }
    async fn consume_sse(self: &Arc<Self>, response: Response, parent: Option<String>, endpoint_events: bool) -> Result<(), RunError> {
        let mut stream=response.bytes_stream(); let mut decoder=SseDecoder::default(); let mut total=0usize;
        let request_cancel=if let Some(parent)=&parent{self.pending.lock().await.get(parent).map(|pending|pending.cancel.clone()).unwrap_or_default()}else{CancelToken::new()};
        loop {
            let chunk=tokio::select!{chunk=stream.next()=>chunk,_=self.close.cancelled()=>return Err(RunError::unavailable()),_=request_cancel.cancelled()=>return Err(RunError::unavailable())};
            let Some(chunk)=chunk else { break; };
            let chunk=chunk.map_err(|_|RunError::unavailable())?;
            total=total.saturating_add(chunk.len());
            if parent.is_some() && total > MAX_MESSAGE*2 { return Err(RunError::unavailable()); }
            for (event,data) in decoder.push(&chunk)? {
                if event=="endpoint" && endpoint_events {
                    let Wire::Remote { endpoint,.. }=&self.wire else { return Err(RunError::not_ready()); };
                    let base=url::Url::parse(endpoint).map_err(|_|RunError::not_ready())?;
                    let target=base.join(&data).map_err(|_|RunError::not_ready())?;
                    if target.origin()!=base.origin() || target.fragment().is_some() || !target.username().is_empty() || target.password().is_some() { return Err(RunError::not_ready()); }
                    *self.legacy_post_endpoint.write().await=Some(target.to_string());
                } else if event=="message" || event.is_empty() {
                    self.receive(data.as_bytes(),parent.as_deref()).await?;
                    if let Some(parent)=&parent { if !self.pending.lock().await.contains_key(parent) { return Ok(()); } }
                }
            }
        }
        if parent.as_ref().is_some_and(|_|true) { Err(RunError::unavailable()) } else { Ok(()) }
    }
    pub async fn cancel(self:&Arc<Self>,id:&str) {
        // Do not drop the pending receiver: a late terminal reply resolves capacity.
        if self.revision.read().await.as_str()==MODERN && matches!(&self.wire,Wire::Remote{..}) { if let Some(pending)=self.pending.lock().await.get(id){pending.cancel.cancel();}return; }
        let _=self.notify("notifications/cancelled",raw::raw(&json!({"requestId":id,"reason":"The owning Fetch request ended."}))).await;
    }
    pub async fn close(self:&Arc<Self>)->bool {
        self.close.cancel(); self.closed.store(true,Ordering::Release);
        let confirmed=if let Some(mut child)=self.child.lock().await.take() {
            #[cfg(unix)]
            if let Some(id)=child.id() { unsafe { libc::kill(-(id as i32),libc::SIGTERM); } }
            match tokio::time::timeout(Duration::from_secs(5),child.wait()).await {
                Ok(Ok(_))=>true,
                _=>{ let _=child.start_kill(); matches!(tokio::time::timeout(Duration::from_secs(5),child.wait()).await,Ok(Ok(_))) }
            }
        } else { false };
        for task in self.background.lock().await.drain(..) { task.abort(); }
        self.fail_closed().await;
        confirmed
    }
    async fn fail_closed(&self) {
        self.closed.store(true,Ordering::Release);
        for (_,pending) in self.pending.lock().await.drain() { let _=pending.reply.send(Err(RunError::unavailable())); }
    }
    pub fn is_local(&self)->bool { matches!(&self.wire,Wire::Stdio(_)) }
    pub fn modern_http(&self)->bool { matches!(&self.wire,Wire::Remote{sse:false,..}) }
    pub async fn update_headers(&self,values:BTreeMap<String,String>)->Result<(),RunError>{
        let Wire::Remote{headers,..}=&self.wire else{return Err(RunError::invalid());};
        *headers.write().await=private_headers(&values)?;Ok(())
    }
    pub async fn confirmed_child_exit(&self)->bool{
        if let Some(child)=self.child.lock().await.as_mut(){matches!(child.try_wait(),Ok(Some(_)))}else{false}
    }
}

async fn bounded_response(response:Response)->Result<Vec<u8>,RunError> {
    if response.content_length().is_some_and(|n|n>MAX_MESSAGE as u64) { return Err(RunError::unavailable()); }
    let mut stream=response.bytes_stream(); let mut bytes=vec![];
    while let Some(chunk)=stream.next().await {
        let chunk=chunk.map_err(|_|RunError::unavailable())?;
        if bytes.len().saturating_add(chunk.len())>MAX_MESSAGE { return Err(RunError::unavailable()); }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
pub fn validate_endpoint(endpoint:&str)->Result<(),RunError> {
    let url=url::Url::parse(endpoint).map_err(|_|RunError::invalid())?;
    if !matches!(url.scheme(),"https"|"http") || url.host_str().is_none() || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() { return Err(RunError::invalid()); }
    if url.scheme()=="http" && !url.host_str().is_some_and(|h|h=="localhost" || h.parse::<std::net::IpAddr>().is_ok_and(|ip|ip.is_loopback())) { return Err(RunError::invalid()); }
    Ok(())
}
fn private_headers(values:&BTreeMap<String,String>)->Result<HeaderMap,RunError> {
    let mut headers=HeaderMap::new();
    for (name,value) in values {
        let lower=name.to_ascii_lowercase();
        if matches!(lower.as_str(),"host"|"connection"|"content-length"|"transfer-encoding"|"cookie"|"origin"|"accept"|"content-type") || lower.starts_with("mcp-") { return Err(RunError::invalid()); }
        let name=HeaderName::from_bytes(name.as_bytes()).map_err(|_|RunError::invalid())?;
        let mut value=HeaderValue::from_str(value).map_err(|_|RunError::invalid())?; value.set_sensitive(true);
        if headers.insert(name,value).is_some() { return Err(RunError::invalid()); }
    }
    Ok(headers)
}

#[derive(Default)]
pub struct SseDecoder { buffer:Vec<u8>, event:String, data:String }
impl SseDecoder {
    pub fn push(&mut self,bytes:&[u8])->Result<Vec<(String,String)>,RunError> {
        if self.buffer.len().saturating_add(bytes.len()).saturating_add(self.data.len())>MAX_MESSAGE { return Err(RunError::unavailable()); }
        self.buffer.extend_from_slice(bytes); let mut result=vec![];
        while let Some(index)=self.buffer.iter().position(|b|*b==b'\n') {
            let line=self.buffer.drain(..=index).collect::<Vec<_>>();
            let line=std::str::from_utf8(&line[..line.len()-1]).map_err(|_|RunError::unavailable())?.trim_end_matches('\r');
            if line.is_empty() {
                if !self.data.is_empty() { self.data.pop(); result.push((std::mem::take(&mut self.event),std::mem::take(&mut self.data))); }
                self.event.clear();
            } else if let Some(value)=line.strip_prefix("event:") { self.event=value.strip_prefix(' ').unwrap_or(value).to_owned(); }
            else if let Some(value)=line.strip_prefix("data:") { self.data.push_str(value.strip_prefix(' ').unwrap_or(value)); self.data.push('\n'); }
        }
        Ok(result)
    }
}

/// Preparation callers may abandon a bounded startup future. Its owned child or
/// event stream must still be closed even before it is published to the manager.
pub(crate) struct CloseOnDrop{connection:Option<Arc<Connection>>}
impl CloseOnDrop{
    pub(crate) fn new(connection:Arc<Connection>)->Self{Self{connection:Some(connection)}}
    pub(crate) fn disarm(&mut self){self.connection=None;}
}
impl Drop for CloseOnDrop{
    fn drop(&mut self){if let Some(connection)=self.connection.take(){connection.closed.store(true,Ordering::Release);connection.close.cancel();if let Ok(runtime)=tokio::runtime::Handle::try_current(){runtime.spawn(async move{let _=connection.close().await;});}}}
}
