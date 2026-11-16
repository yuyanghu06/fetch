//! Connector checks use task-owned HTTPS, synthetic MCP peers and memory grants.
//! No native app, Keychain, personal configuration or external service is used.
use super::{Options, profile::ProfileStore, run_with_io};
use crate::{
    authority::{AuthorizationError, Authorizer, Caller},
    credentials::{CredentialError, CredentialStore, KeyProvider},
    mcp::{ManagementBackend, McpGateway},
    native_auth::{NativeAuthority, NativeSettingsSession},
    oauth::OAuthService,
    remote_oauth::{Backend, GrantError},
    runner::{
        ExecutionPolicy, LEGACY, MODERN, RunError, Runner, ServerRoute, TransportConfig, raw,
    },
};
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response, Sse, sse::Event},
    routing::post,
};
use serde_json::{Value, json, value::RawValue};
use std::{
    collections::BTreeMap,
    convert::Infallible,
    future::Future,
    net::{Ipv4Addr, SocketAddr, TcpListener},
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, DuplexStream, Lines, ReadHalf, WriteHalf},
    sync::{mpsc, oneshot},
};
use tokio_stream::wrappers::ReceiverStream;
use zeroize::Zeroizing;

const RAW_RESULT: &str = r#"{ "content":[{"type":"text","text":"original upstream result","future":true}], "structuredContent":{"number":184467440737095516160000001}, "futureResult":{"keep":true} }"#;
const RAW_FAILURE: &str = r#"{ "content":[{"type":"text","text":"upstream refused this request"}], "isError":true, "errorDetails":{"code":"UPSTREAM_DENIED","details":{"keep":true}} }"#;

#[derive(Default)]
struct MemoryCredentials(Mutex<BTreeMap<(String, String), Vec<u8>>>);
impl Backend for MemoryCredentials {
    fn read(&self, service: &str, account: &str) -> Result<Option<Zeroizing<Vec<u8>>>, GrantError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .get(&(service.into(), account.into()))
            .cloned()
            .map(Zeroizing::new))
    }
    fn write(&self, service: &str, account: &str, value: &[u8]) -> Result<(), GrantError> {
        self.0
            .lock()
            .unwrap()
            .insert((service.into(), account.into()), value.to_vec());
        Ok(())
    }
}
impl MemoryCredentials {
    fn expire_pending(&self, profile: &str) {
        let mut records = self.0.lock().unwrap();
        let bytes = records
            .values_mut()
            .find(|bytes| {
                serde_json::from_slice::<Value>(bytes)
                    .is_ok_and(|value| value["profile"] == profile)
            })
            .expect("owned profile exists");
        let mut document: Value = serde_json::from_slice(bytes).unwrap();
        document["pending"]["expires_at"] = json!(0);
        *bytes = serde_json::to_vec(&document).unwrap();
    }
}

struct FixtureKeys;
impl KeyProvider for FixtureKeys {
    fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
        Ok(Zeroizing::new([91; 32]))
    }
}
#[derive(Default)]
struct EnrollmentCounts {
    created: AtomicUsize,
    claimed: AtomicUsize,
}
struct FixtureAuthority {
    oauth: Arc<OAuthService>,
}
impl Authorizer for FixtureAuthority {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError> {
        let token = headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());
        let owner = match token {
            Some("Bearer fixture-claude-access") => "fixture-claude-owner",
            Some("Bearer fixture-codex-access") => "fixture-codex-owner",
            _ => return self.oauth.authorize(headers),
        };
        Ok(Caller::from_grant(
            owner.into(),
            ["fetch:catalog", "fetch:tools:call", "fetch:content:read"]
                .into_iter()
                .map(str::to_owned),
        ))
    }
}
struct NoManagement {
    runner: Arc<Runner>,
}
impl ManagementBackend for NoManagement {
    fn invoke<'a>(
        &'a self,
        name: &'a str,
        arguments: Box<RawValue>,
        caller: Caller,
        view: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Value, RunError>> + Send + 'a>> {
        Box::pin(async move {
            match name {
                "fetch__list_servers" => Ok(
                    json!({"status":"ok","servers":[{"server_id":"disposable","name":"Disposable peer","status":"ready"}],"next_steps":[]}),
                ),
                "fetch__list_tools" => {
                    let input: Value = serde_json::from_str(arguments.get()).unwrap();
                    if input["server_id"] != json!(["disposable"])
                        || !caller.has_scope("fetch:catalog")
                    {
                        return Err(RunError::invalid());
                    }
                    let origin = self.runner.catalog("disposable", view).await?;
                    Ok(
                        json!({"status":"ok","view":view,"complete":true,"origins":[origin],"not_found_ids":[],"next_steps":[]}),
                    )
                }
                _ => Err(RunError::missing()),
            }
        })
    }
}

#[derive(Default)]
struct Peer {
    calls: AtomicUsize,
    catalogs: AtomicUsize,
    args: Mutex<Vec<String>>,
    replies: Mutex<BTreeMap<String, oneshot::Sender<Value>>>,
    cancels: AtomicUsize,
}

fn rpc_result(id: &Value, result: &str) -> String {
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{result}}}")
}
fn json_reply(body: String) -> Response {
    ([(header::CONTENT_TYPE, "application/json")], body).into_response()
}
async fn peer_request(State(peer): State<Arc<Peer>>, body: String) -> Response {
    let message: Value = serde_json::from_str(&body).unwrap();
    let method = message["method"].as_str().unwrap_or("");
    let id = &message["id"];
    match method {
        "initialize" => json_reply(rpc_result(id, &json!({"protocolVersion":LEGACY,"capabilities":{"tools":{"listChanged":true}},"serverInfo":{"name":"connector disposable peer","version":"1"}}).to_string())),
        "tools/list" => { peer.catalogs.fetch_add(1,Ordering::SeqCst); json_reply(rpc_result(id, r#"{"tools":[{"name":"echo","description":"Disposable connector fixture","inputSchema":{"type":"object"},"futureTool":{"counter":184467440737095516160000007}}]}"#)) },
        "tools/call" => {
            peer.calls.fetch_add(1, Ordering::SeqCst);
            let raw_message = raw::parse(body.as_bytes()).unwrap();
            let arguments = raw::field(&raw::field(&raw_message, "params").unwrap(), "arguments").unwrap();
            peer.args.lock().unwrap().push(arguments.get().to_owned());
            let mode = message["params"]["arguments"]["mode"].as_str().unwrap_or("echo");
            if mode == "fail" { return json_reply(rpc_result(id, RAW_FAILURE)); }
            if mode == "echo" { return json_reply(rpc_result(id, RAW_RESULT)); }
            let (wire, rx) = mpsc::channel::<Result<Event, Infallible>>(8);
            let message = message.clone();
            let mode = mode.to_owned();
            tokio::spawn(async move {
                let progress = json!({"jsonrpc":"2.0","method":"notifications/progress","params":{"progressToken":message["params"]["_meta"]["progressToken"],"progress":1,"total":2,"message":message["params"]["arguments"]["marker"],"futureProgress":{"keep":true}}});
                if wire.send(Ok(Event::default().event("message").data(progress.to_string()))).await.is_err() { return; }
                if mode == "refresh" {
                    let _ = wire.send(Ok(Event::default().event("message").data(json!({"jsonrpc":"2.0","method":"notifications/tools/list_changed","params":{}}).to_string()))).await;
                    let _ = wire.send(Ok(Event::default().event("message").data(rpc_result(&message["id"], RAW_RESULT)))).await;
                    return;
                }
                let control_id = if mode == "ask" { format!("ask-{}", message["id"]) } else { message["id"].as_str().unwrap().to_owned() };
                let (reply, answer) = oneshot::channel();
                peer.replies.lock().unwrap().insert(control_id.clone(), reply);
                if mode == "ask" {
                    let request = json!({"jsonrpc":"2.0","id":control_id,"method":"elicitation/create","params":{"mode":"form","message":message["params"]["arguments"]["marker"],"requestedSchema":{"type":"object","properties":{"answer":{"type":"string"}}},"futureRequest":{"keep":true}}});
                    let _ = wire.send(Ok(Event::default().event("message").data(request.to_string()))).await;
                }
                if let Ok(reply) = answer.await {
                    if mode == "ask" {
                        let _ = wire.send(Ok(Event::default().event("message").data(rpc_result(&message["id"], &json!({"content":[{"type":"text","text":reply["result"]["content"]["answer"]}],"futureResult":{"interactive":true}}).to_string())))).await;
                    }
                }
                peer.replies.lock().unwrap().remove(&control_id);
            });
            Sse::new(ReceiverStream::new(rx)).into_response()
        }
        "notifications/cancelled" => {
            peer.cancels.fetch_add(1, Ordering::SeqCst);
            if let Some(id) = message["params"]["requestId"].as_str() { peer.replies.lock().unwrap().remove(id); }
            StatusCode::ACCEPTED.into_response()
        }
        "" => {
            if let Some(id) = id.as_str() {
                if let Some(reply) = peer.replies.lock().unwrap().remove(id) { let _ = reply.send(message); }
            }
            StatusCode::ACCEPTED.into_response()
        }
        _ => StatusCode::ACCEPTED.into_response(),
    }
}

struct Fixture {
    directory: tempfile::TempDir,
    port: u16,
    certificate: String,
    key: Vec<u8>,
    handle: axum_server::Handle<SocketAddr>,
    task: Option<tokio::task::JoinHandle<std::io::Result<()>>>,
    peer: Arc<Peer>,
    peer_url: String,
    peer_task: tokio::task::JoinHandle<std::io::Result<()>>,
    runner: Arc<Runner>,
    credentials: Arc<MemoryCredentials>,
    backend_cancels: Arc<AtomicUsize>,
    enrollment: Arc<EnrollmentCounts>,
    app_credentials: Arc<CredentialStore>,
    oauth: Arc<OAuthService>,
    native: Arc<NativeAuthority>,
    native_session: NativeSettingsSession,
}
impl Fixture {
    async fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))
                .unwrap();
        }
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let certificate = cert.pem();
        std::fs::write(directory.path().join("certificate.pem"), &certificate).unwrap();
        let key = signing_key.serialize_der();
        let peer = Arc::new(Peer::default());
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .unwrap();
        let peer_url = format!(
            "http://127.0.0.1:{}/mcp",
            listener.local_addr().unwrap().port()
        );
        let app = Router::new()
            .route("/mcp", post(peer_request))
            .with_state(peer.clone());
        let peer_task = tokio::spawn(async move { axum::serve(listener, app).await });
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let runner = Self::runner(&peer_url).await;
        // Real app grants authorize durable server membership. Keep the same
        // task-owned Shared record beside the already configured synthetic peer.
        crate::storage::ServerRepository::new(directory.path().to_owned()).save("disposable",&json!({
            "server_id":"disposable","name":"Disposable fixture peer",
            "runtime":{"status":"ready","error":null},
            "authentication":{"status":"not_required","error":null},
            "restart_required":false,"restart":null,"credentials":[],
            "config":{"kind":"remote","package":null,"executable":null,"args":[],
                "working_directory":null,"environment":[],"scope":{"project_id":null},
                "connection":{"url":peer_url,"transport":"streamable_http","headers":[]}}
        })).unwrap();
        let backend_cancels = Arc::new(AtomicUsize::new(0));
        let enrollment = Arc::new(EnrollmentCounts::default());
        let native = NativeAuthority::fixture();
        let native_session = native.open_settings();
        let app_credentials = Arc::new(CredentialStore::new(
            directory.path().to_owned(),
            Arc::new(FixtureKeys),
        ));
        let oauth = OAuthService::open(
            format!("https://localhost:{port}"),
            directory.path().to_owned(),
            app_credentials.clone(),
            native.clone(),
        )
        .unwrap();
        let (handle, task) = Self::listen(
            listener,
            &certificate,
            &key,
            runner.clone(),
            backend_cancels.clone(),
            enrollment.clone(),
            oauth.clone(),
        )
        .await;
        Self {
            directory,
            port,
            certificate,
            key,
            handle,
            task: Some(task),
            peer,
            peer_url,
            peer_task,
            runner,
            credentials: Arc::new(MemoryCredentials::default()),
            backend_cancels,
            enrollment,
            app_credentials,
            oauth,
            native,
            native_session,
        }
    }
    async fn runner(peer_url: &str) -> Arc<Runner> {
        let runner = Runner::new();
        runner
            .configure(ServerRoute {
                server_id: "disposable".into(),
                transport: TransportConfig::Http {
                    endpoint: peer_url.into(),
                    headers: BTreeMap::new(),
                },
                execution: ExecutionPolicy {
                    max_concurrent_calls: 4,
                    ..ExecutionPolicy::default()
                },
            })
            .await
            .unwrap();
        runner.connect("disposable").await.unwrap();
        runner
    }
    async fn listen(
        listener: TcpListener,
        certificate: &str,
        key: &[u8],
        runner: Arc<Runner>,
        backend_cancels: Arc<AtomicUsize>,
        enrollment: Arc<EnrollmentCounts>,
        oauth: Arc<OAuthService>,
    ) -> (
        axum_server::Handle<SocketAddr>,
        tokio::task::JoinHandle<std::io::Result<()>>,
    ) {
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let certs = rustls_pemfile::certs(&mut std::io::BufReader::new(certificate.as_bytes()))
            .map(|v| v.unwrap().to_vec())
            .collect();
        let tls = axum_server::tls_rustls::RustlsConfig::from_der(certs, key.to_vec())
            .await
            .unwrap();
        let gateway = McpGateway::new(
            runner.clone(),
            Arc::new(NoManagement { runner }),
            Arc::new(FixtureAuthority {
                oauth: oauth.clone(),
            }),
            format!("https://localhost:{port}"),
        );
        let app=gateway.router().merge(crate::oauth::router(oauth)).layer(axum::middleware::from_fn(move |request:axum::extract::Request,next:axum::middleware::Next| {
            let cancels=backend_cancels.clone();let enrollment=enrollment.clone();
            async move {
                let (parts,body)=request.into_parts();
                if parts.method==axum::http::Method::POST&&parts.uri.path()=="/api/v1/connections/requests" {enrollment.created.fetch_add(1,Ordering::SeqCst);}
                if parts.method==axum::http::Method::POST&&parts.uri.path().ends_with("/claim") {enrollment.claimed.fetch_add(1,Ordering::SeqCst);}
                let bytes=axum::body::to_bytes(body,16*1024*1024).await.unwrap();
                let message:Value=serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                if message["method"]=="notifications/cancelled" {cancels.fetch_add(1,Ordering::SeqCst);}
                if message["method"]=="fixture/raw-error" {
                    return json_reply(format!(r#"{{"jsonrpc":"2.0","id":{},"error":{{ "code":-32042, "message":"Original backend rejection", "data":{{"number":184467440737095516160000013,"details":["keep",true]}} }}}}"#,message["id"]));
                }
                next.run(axum::extract::Request::from_parts(parts,axum::body::Body::from(bytes))).await
            }
        }));
        let handle = axum_server::Handle::new();
        let server_handle = handle.clone();
        let task = tokio::spawn(async move {
            axum_server::from_tcp_rustls(listener, tls)
                .expect("disposable TLS listener")
                .handle(server_handle)
                .serve(app.into_make_service())
                .await
        });
        assert!(
            tokio::time::timeout(Duration::from_secs(5), handle.listening())
                .await
                .unwrap()
                .is_some()
        );
        (handle, task)
    }
    fn options(&self, name: &str) -> Options {
        Options {
            root: self.directory.path().to_owned(),
            port: self.port,
            client: name.into(),
            project_id:None,
        }
    }
    fn profile(&self, name: &str) -> ProfileStore {
        ProfileStore::with_backend(
            self.directory.path().to_owned(),
            name.into(),
            self.credentials.clone(),
        )
        .unwrap()
    }
    async fn decide(&self, pending: &super::profile::Pending, allow: bool) {
        let http = super::pinned_client(&self.options(&pending.profile)).unwrap();
        let response=http.post(format!("https://localhost:{}/api/v1/connections/requests/{}/decision",self.port,crate::json::path_component(&pending.request_id)))
            .header("x-fetch-settings-capability",self.native_session.header_value())
            .json(&json!({"decision":if allow {"allow"} else {"deny"},"scopes":if allow {vec!["fetch:catalog","fetch:tools:call","fetch:content:read"]}else{vec![]}})).send().await.unwrap();
        let status = response.status();
        let value: Value = response.json().await.unwrap();
        assert_eq!(status, StatusCode::OK, "{value}");
    }
    async fn await_pending(&self, name: &str) -> super::profile::Pending {
        let store = self.profile(name);
        let origin = self.options(name).origin();
        tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                if let Some(pending) = store.pending(&origin).await.unwrap() {
                    break pending;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("automatic startup should persist its pending proof")
    }
    async fn uninitialized_bridge(&self, name: &str) -> Bridge {
        let options = Options {
            root: self.directory.path().to_owned(),
            port: self.port,
            client: name.into(),
            project_id:None,
        };
        let store =
            ProfileStore::with_backend(options.root.clone(), name.into(), self.credentials.clone())
                .unwrap();
        if store.read().await.unwrap().is_none() {
            store.save_response(&json!({"client_id":format!("fixture-{name}-client"),"access_token":format!("fixture-{name}-access"),"refresh_token":format!("fixture-{name}-refresh"),"expires_in":3600,"scope":"fetch:catalog fetch:tools:call fetch:content:read"})).await.unwrap();
        }
        let (client, connector) = tokio::io::duplex(256 * 1024);
        let (input, output) = tokio::io::split(connector);
        let task = tokio::spawn(run_with_io(options, store, input, output));
        let (input, output) = tokio::io::split(client);
        Bridge {
            input: BufReader::new(input).lines(),
            output,
            task,
            tool_name: "echo".into(),
        }
    }
    async fn bridge(&self, name: &str) -> Bridge {
        let mut bridge = self.uninitialized_bridge(name).await;
        bridge.send(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":LEGACY,"capabilities":{"elicitation":{"form":{}}},"clientInfo":{"name":name,"version":"1"}}})).await;
        assert_eq!(bridge.next().await["result"]["protocolVersion"], LEGACY);
        bridge
            .send(json!({"jsonrpc":"2.0","method":"notifications/initialized","params":{}}))
            .await;
        bridge.send(json!({"jsonrpc":"2.0","id":-1,"method":"tools/call","params":{"name":"fetch__list_tools","arguments":{"server_id":["disposable"]}}})).await;
        assert!(
            !bridge.response(-1).await["result"]["isError"]
                .as_bool()
                .unwrap_or(false)
        );
        bridge
    }
    async fn restart(&mut self) {
        self.handle.shutdown();
        tokio::time::timeout(Duration::from_secs(5), self.task.take().unwrap())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        self.runner.shutdown().await;
        self.oauth.shutdown();
        self.native = NativeAuthority::fixture();
        self.native_session = self.native.open_settings();
        self.oauth = OAuthService::open(
            format!("https://localhost:{}", self.port),
            self.directory.path().to_owned(),
            self.app_credentials.clone(),
            self.native.clone(),
        )
        .unwrap();
        self.runner = Self::runner(&self.peer_url).await;
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, self.port)).unwrap();
        let (handle, task) = Self::listen(
            listener,
            &self.certificate,
            &self.key,
            self.runner.clone(),
            self.backend_cancels.clone(),
            self.enrollment.clone(),
            self.oauth.clone(),
        )
        .await;
        self.handle = handle;
        self.task = Some(task);
    }
    async fn finish(mut self) {
        self.handle.shutdown();
        if let Some(task) = self.task.take() {
            let _ = tokio::time::timeout(Duration::from_secs(5), task).await;
        }
        self.runner.shutdown().await;
        self.peer_task.abort();
        let _ = (&mut self.peer_task).await;
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.handle.shutdown();
        self.peer_task.abort();
    }
}
struct Bridge {
    input: Lines<BufReader<ReadHalf<DuplexStream>>>,
    output: WriteHalf<DuplexStream>,
    task: tokio::task::JoinHandle<Result<(), String>>,
    tool_name: String,
}
impl Bridge {
    async fn send(&mut self, message: Value) {
        self.send_raw(&message.to_string()).await;
    }
    async fn send_raw(&mut self, message: &str) {
        self.output.write_all(message.as_bytes()).await.unwrap();
        self.output.write_all(b"\n").await.unwrap();
        self.output.flush().await.unwrap();
    }
    async fn next_raw(&mut self) -> String {
        tokio::time::timeout(Duration::from_secs(6), self.input.next_line())
            .await
            .expect("connector response timeout")
            .unwrap()
            .expect("connector output closed")
    }
    async fn next(&mut self) -> Value {
        serde_json::from_str(&self.next_raw().await).expect("stdout contains only MCP JSON")
    }
    async fn response(&mut self, id: i64) -> Value {
        loop {
            let value = self.next().await;
            if value["id"] == id {
                return value;
            }
        }
    }
    async fn close(mut self) {
        self.output.shutdown().await.unwrap();
        tokio::time::timeout(Duration::from_secs(6), &mut self.task)
            .await
            .expect("connector EOF should stop only connector")
            .unwrap()
            .unwrap();
    }
    async fn tool(&mut self, id: i64, args: Value) {
        self.send(json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"fetch__call_tool","arguments":{"server_id":"disposable","tool_name":self.tool_name,"arguments":args},"_meta":{"progressToken":format!("progress-{id}")}}})).await;
    }
}

fn modern_message(id: i64, method: &str, mut params: Value) -> Value {
    params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{"elicitation":{"form":{}}}});
    json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn modern_startup_and_selected_tool_call_cross_stdio_and_verified_https() {
    let fixture = Fixture::new().await;
    let baseline = fixture.peer.catalogs.load(Ordering::SeqCst);
    let mut bridge = fixture.uninitialized_bridge("claude").await;
    bridge
        .send(modern_message(0, "server/discover", json!({})))
        .await;
    let discovered = bridge.response(0).await;
    assert_eq!(discovered["result"]["resultType"], "complete");
    assert!(
        discovered["result"]["instructions"]
            .as_str()
            .unwrap()
            .contains("disposable")
    );
    bridge
        .send(modern_message(1, "tools/list", json!({})))
        .await;
    let listed = bridge.response(1).await;
    assert_eq!(listed["result"]["tools"].as_array().unwrap().len(), 3);
    assert_eq!(
        fixture.peer.catalogs.load(Ordering::SeqCst),
        baseline,
        "modern startup does not query upstream catalogs"
    );
    bridge
        .send(modern_message(
            2,
            "tools/call",
            json!({"name":"fetch__list_tools","arguments":{"server_id":["disposable"]}}),
        ))
        .await;
    let selected = bridge.response(2).await;
    let selected: Value =
        serde_json::from_str(selected["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(selected["origins"][0]["tools"][0]["name"], "echo");
    bridge.send(modern_message(3,"tools/call",json!({"name":"fetch__call_tool","arguments":{"server_id":"disposable","tool_name":"echo","arguments":{"mode":"echo","future":{"keep":true}}}}))).await;
    let result = bridge.response(3).await;
    assert_eq!(result["result"]["resultType"], "complete");
    assert_eq!(
        result["result"]["content"][0]["text"],
        "original upstream result"
    );
    assert_eq!(
        result["result"]["structuredContent"]["number"].to_string(),
        "184467440737095516160000001"
    );
    assert_eq!(result["result"]["futureResult"]["keep"], true);
    bridge.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn raw_catalog_arguments_results_and_tool_failures_cross_real_https_gateway() {
    let fixture = Fixture::new().await;
    let mut bridge = fixture.bridge("claude").await;
    bridge
        .send(json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}))
        .await;
    let listed = bridge.response(1).await;
    let native = listed["result"]["tools"].as_array().unwrap();
    assert_eq!(native.len(), 3);
    assert!(
        native
            .iter()
            .any(|tool| tool["name"] == "fetch__list_servers"
                && tool["description"].as_str().unwrap().contains("disposable"))
    );
    assert!(!native.iter().any(|tool| tool["name"] == bridge.tool_name));
    let baseline = fixture.peer.catalogs.load(Ordering::SeqCst);
    bridge
        .send(json!({"jsonrpc":"2.0","id":11,"method":"tools/list","params":{}}))
        .await;
    let _ = bridge.response(11).await;
    assert_eq!(
        fixture.peer.catalogs.load(Ordering::SeqCst),
        baseline,
        "fixed startup catalog never re-queries upstream tools"
    );
    bridge.send(json!({"jsonrpc":"2.0","id":12,"method":"tools/call","params":{"name":"fetch__list_tools","arguments":{"server_id":["disposable"]}}})).await;
    let selected = bridge.response(12).await;
    let selected: Value =
        serde_json::from_str(selected["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    let tool = &selected["origins"][0]["tools"][0];
    assert_eq!(tool["name"], "echo");
    assert_eq!(
        tool["futureTool"]["counter"].to_string(),
        "184467440737095516160000007"
    );
    let arguments = r#"{ "mode":"echo", "number":184467440737095516160000009, "futureArgument":{"keep":true} }"#;
    bridge.send_raw(&format!(r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"fetch__call_tool","arguments":{{"server_id":"disposable","tool_name":"echo","arguments":{arguments}}}}}}}"#)).await;
    let result = bridge.next_raw().await;
    assert!(
        result.contains(RAW_RESULT),
        "raw terminal result changed: {result}"
    );
    assert_eq!(fixture.peer.args.lock().unwrap().last().unwrap(), arguments);
    bridge.tool(3, json!({"mode":"fail"})).await;
    let failed = bridge.next_raw().await;
    assert!(
        failed.contains(RAW_FAILURE),
        "upstream error result changed: {failed}"
    );
    bridge
        .send(json!({"jsonrpc":"2.0","id":4,"method":"unsupported/read","params":{}}))
        .await;
    let error = bridge.response(4).await;
    assert_eq!(error["error"]["code"], -32601);
    assert_eq!(
        error["error"]["message"],
        "This MCP method is not implemented."
    );
    bridge
        .send(json!({"jsonrpc":"2.0","id":5,"method":"fixture/raw-error","params":{}}))
        .await;
    let raw_error = bridge.response(5).await;
    assert_eq!(raw_error["error"]["code"], -32042);
    assert_eq!(raw_error["error"]["message"], "Original backend rejection");
    assert_eq!(
        raw_error["error"]["data"]["number"].to_string(),
        "184467440737095516160000013"
    );
    assert_eq!(raw_error["error"]["data"]["details"], json!(["keep", true]));
    bridge.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_profiles_and_same_profile_keep_equal_ids_progress_and_interaction_separate() {
    let fixture = Fixture::new().await;
    let mut first = fixture.bridge("claude").await;
    let mut second = fixture.bridge("codex").await;
    let mut same_profile = fixture.bridge("claude").await;
    for (bridge, marker) in [
        (&mut first, "first"),
        (&mut second, "second"),
        (&mut same_profile, "same-profile"),
    ] {
        bridge.tool(7, json!({"mode":"ask","marker":marker})).await;
    }
    for (bridge, marker) in [
        (&mut first, "first"),
        (&mut second, "second"),
        (&mut same_profile, "same-profile"),
    ] {
        let progress = bridge.next().await;
        assert_eq!(progress["method"], "notifications/progress");
        assert_eq!(progress["params"]["progressToken"], "progress-7");
        assert_eq!(progress["params"]["message"], marker);
        let request = bridge.next().await;
        assert_eq!(request["method"], "elicitation/create");
        assert_eq!(request["params"]["message"], marker);
        assert_eq!(request["params"]["futureRequest"]["keep"], true);
        bridge.send(json!({"jsonrpc":"2.0","id":request["id"],"result":{"action":"accept","content":{"answer":format!("reply-{marker}")}}})).await;
        let result = bridge.response(7).await;
        assert_eq!(
            result["result"]["content"][0]["text"],
            format!("reply-{marker}")
        );
    }
    first.close().await;
    second.tool(8, json!({"mode":"echo"})).await;
    assert!(
        second.response(8).await["result"]["futureResult"]["keep"]
            .as_bool()
            .unwrap()
    );
    assert_eq!(
        fixture.runner.snapshot("disposable").await["runtime"]["status"],
        "ready"
    );
    second.close().await;
    same_profile.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cancellation_and_catalog_notifications_leave_another_connector_usable() {
    let fixture = Fixture::new().await;
    let mut slow = fixture.bridge("claude").await;
    let mut other = fixture.bridge("codex").await;
    slow.tool(9, json!({"mode":"wait","marker":"cancel-only-this"}))
        .await;
    assert_eq!(slow.next().await["method"], "notifications/progress");
    slow.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":9,"reason":"user cancelled fixture"}})).await;
    other.tool(9, json!({"mode":"echo"})).await;
    assert_eq!(
        other.response(9).await["result"]["futureResult"]["keep"],
        true
    );
    tokio::time::timeout(Duration::from_secs(3), async {
        while fixture.peer.cancels.load(Ordering::SeqCst) == 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("cancellation reaches owned upstream call");
    other
        .tool(10, json!({"mode":"refresh","marker":"catalog"}))
        .await;
    let mut changed = false;
    let mut completed = false;
    while !changed || !completed {
        let value = other.next().await;
        changed |= value["method"] == "notifications/tools/list_changed";
        completed |= value["id"] == 10;
    }
    slow.close().await;
    other.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn saved_profile_reconnects_after_fetch_restart_without_replaying_prior_calls() {
    let mut fixture = Fixture::new().await;
    let mut first = fixture.bridge("claude").await;
    first.tool(1, json!({"mode":"echo"})).await;
    assert!(first.response(1).await.get("result").is_some());
    first.close().await;
    let mut again = fixture.bridge("claude").await;
    again.tool(1, json!({"mode":"echo"})).await;
    assert!(again.response(1).await.get("result").is_some());
    again
        .tool(
            2,
            json!({"mode":"wait","marker":"do-not-replay-after-backend-restart"}),
        )
        .await;
    assert_eq!(again.next().await["method"], "notifications/progress");
    assert_eq!(fixture.peer.calls.load(Ordering::SeqCst), 3);
    fixture.restart().await;
    let disconnected = tokio::time::timeout(Duration::from_secs(7), &mut again.task)
        .await
        .expect("old connector must report backend loss")
        .unwrap();
    assert!(
        disconnected.is_err(),
        "losing an in-flight response must not claim successful completion"
    );
    let mut restarted = fixture.bridge("claude").await;
    restarted.tool(1, json!({"mode":"echo"})).await;
    assert!(restarted.response(1).await.get("result").is_some());
    assert_eq!(
        fixture.peer.calls.load(Ordering::SeqCst),
        4,
        "reconnection must not replay completed or interrupted tool calls"
    );
    restarted.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn connector_rejects_a_different_saved_certificate_without_trusting_the_system() {
    let fixture = Fixture::new().await;
    let bridge = fixture.bridge("claude").await;
    bridge.close().await;
    let wrong = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    std::fs::write(
        fixture.directory.path().join("certificate.pem"),
        wrong.cert.pem(),
    )
    .unwrap();
    let options = Options {
        root: fixture.directory.path().to_owned(),
        port: fixture.port,
        client: "claude".into(),
        project_id:None,
    };
    let store = ProfileStore::with_backend(
        options.root.clone(),
        options.client.clone(),
        fixture.credentials.clone(),
    )
    .unwrap();
    let (client, connector) = tokio::io::duplex(4096);
    let (input, output) = tokio::io::split(connector);
    let task = tokio::spawn(run_with_io(options, store, input, output));
    let (_read, mut write) = tokio::io::split(client);
    write.write_all(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":LEGACY,"capabilities":{},"clientInfo":{"name":"fixture","version":"1"}}}).to_string().as_bytes()).await.unwrap();
    write.write_all(b"\n").await.unwrap();
    let error = tokio::time::timeout(Duration::from_secs(6), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap_err();
    assert!(
        error.to_ascii_lowercase().contains("certificate")
            || error.to_ascii_lowercase().contains("tls"),
        "certificate rejection must be explained: {error}"
    );
    assert_eq!(fixture.peer.calls.load(Ordering::SeqCst), 0);
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn immediate_cancel_reaches_backend_before_waiting_for_progress_and_other_calls_work() {
    let fixture = Fixture::new().await;
    let mut first = fixture.bridge("claude").await;
    let mut second = fixture.bridge("codex").await;
    first
        .tool(21, json!({"mode":"wait","marker":"immediately-cancel"}))
        .await;
    first.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":21,"reason":"immediate user cancel"}})).await;
    first.tool(22, json!({"mode":"echo"})).await;
    second.tool(21, json!({"mode":"echo"})).await;
    assert_eq!(
        first.response(22).await["result"]["futureResult"]["keep"],
        true
    );
    assert_eq!(
        second.response(21).await["result"]["futureResult"]["keep"],
        true
    );
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let snapshot = fixture.runner.snapshot("disposable").await;
            let execution = &snapshot["tool_execution"];
            // A cancellation is not proof that a dispatched action stopped.
            // The existing scheduler may correctly retain its uncertain slot.
            let accounted = execution["active_calls"] == 0
                || execution["uncertain_calls"].as_u64().unwrap_or(0) > 0;
            if fixture.backend_cancels.load(Ordering::SeqCst) > 0 && accounted {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect(
        "immediate cancellation reaches the backend and dispatched work remains honestly accounted",
    );
    let snapshot = fixture.runner.snapshot("disposable").await;
    assert!(
        snapshot["tool_execution"]["active_calls"].as_u64().unwrap() <= 1,
        "unrelated calls must finish: {snapshot}"
    );
    first.close().await;
    second.close().await;
    fixture.finish().await;
}

fn pending_startup(
    fixture: &Fixture,
    name: &str,
    launches: Arc<AtomicUsize>,
) -> tokio::task::JoinHandle<Result<(), String>> {
    let options = fixture.options(name);
    let store = fixture.profile(name);
    tokio::spawn(async move {
        let http = super::pinned_client(&options)?;
        super::ensure_approved_with_launcher(&options, &store, move || async move {
            launches.fetch_add(1, Ordering::SeqCst);
            Ok(http)
        })
        .await
    })
}
fn fixture_grant() -> Value {
    json!({"client_id":"existing-fixture-client","access_token":"existing-approved-access","refresh_token":"existing-approved-refresh","expires_in":3600,"scope":"fetch:catalog"})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn automatic_startup_coalesces_pending_request_and_app_finishes_after_waiter_is_killed() {
    let fixture = Fixture::new().await;
    let options = fixture.options("claude");
    let origin = options.origin();
    let store = fixture.profile("claude");
    let launches = Arc::new(AtomicUsize::new(0));
    let first = pending_startup(&fixture, "claude", launches.clone());
    let second = pending_startup(&fixture, "claude", launches.clone());
    let pending = fixture.await_pending("claude").await;
    tokio::time::timeout(Duration::from_secs(3), async {
        while launches.load(Ordering::SeqCst) != 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let http = super::pinned_client(&options).unwrap();
    let reused = store.ensure_pending(&http, &origin).await.unwrap().unwrap();
    assert_eq!(reused.request_id, pending.request_id);
    assert_eq!(
        fixture.enrollment.created.load(Ordering::SeqCst),
        1,
        "same-profile starts share one request"
    );
    assert_eq!(
        fixture.enrollment.claimed.load(Ordering::SeqCst),
        0,
        "connector waiters must not claim the request"
    );
    assert!(store.read().await.unwrap().is_none());
    let (client, io) = tokio::io::duplex(4096);
    let (input, output) = tokio::io::split(io);
    let unapproved = run_with_io(options.clone(), store.clone(), input, output)
        .await
        .unwrap_err();
    assert!(unapproved.contains("waiting for approval"));
    drop(client);
    assert_eq!(
        fixture.peer.calls.load(Ordering::SeqCst),
        0,
        "startup does not grant tool access"
    );
    first.abort();
    assert!(first.await.unwrap_err().is_cancelled());
    assert!(
        fixture
            .profile("claude")
            .pending(&origin)
            .await
            .unwrap()
            .is_some(),
        "private proof outlives the waiting connector"
    );
    fixture.decide(&pending, true).await;
    let app_store = fixture.profile("claude");
    assert!(
        app_store
            .complete_pending(&http, &origin, &pending.request_id)
            .await
            .unwrap()
    );
    tokio::time::timeout(Duration::from_secs(3), second)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        fixture.enrollment.claimed.load(Ordering::SeqCst),
        1,
        "the app is the sole claimant"
    );
    assert!(app_store.pending(&origin).await.unwrap().is_none());
    let grant = app_store.read().await.unwrap().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", grant.access_token).parse().unwrap(),
    );
    assert!(
        fixture
            .oauth
            .authorize(&headers)
            .unwrap()
            .has_scope("fetch:tools:call")
    );
    let before = launches.load(Ordering::SeqCst);
    let approved = pending_startup(&fixture, "claude", launches.clone());
    approved.await.unwrap().unwrap();
    assert_eq!(
        launches.load(Ordering::SeqCst),
        before,
        "saved approval avoids another app launch"
    );
    let mut reconnected = fixture.bridge("claude").await;
    reconnected.tool(71, json!({"mode":"echo"})).await;
    assert_eq!(
        reconnected.response(71).await["result"]["futureResult"]["keep"],
        true
    );
    reconnected.close().await;
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn automatic_approval_cannot_claim_another_request_profile_or_private_proof() {
    let fixture = Fixture::new().await;
    let options = fixture.options("claude");
    let origin = options.origin();
    let http = super::pinned_client(&options).unwrap();
    let claude = fixture.profile("claude");
    let codex = fixture.profile("codex");
    let first = claude
        .ensure_pending(&http, &origin)
        .await
        .unwrap()
        .unwrap();
    let second = codex.ensure_pending(&http, &origin).await.unwrap().unwrap();
    assert!(
        !claude
            .complete_pending(&http, &origin, &second.request_id)
            .await
            .unwrap()
    );
    assert!(
        !codex
            .complete_pending(&http, &origin, &first.request_id)
            .await
            .unwrap()
    );
    assert_eq!(
        fixture.enrollment.claimed.load(Ordering::SeqCst),
        0,
        "mismatched profile/request does not reach claim"
    );
    fixture.decide(&first, true).await;
    let wrong = serde_json::to_value(&second).unwrap();
    let response = http
        .post(format!(
            "{origin}/api/v1/connections/requests/{}/claim",
            crate::json::path_component(&first.request_id)
        ))
        .json(&json!({"claim_secret":wrong["claim_secret"]}))
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "another pending request's proof cannot adopt an approved grant"
    );
    assert!(claude.read().await.unwrap().is_none());
    assert!(codex.read().await.unwrap().is_none());
    assert!(
        claude
            .complete_pending(&http, &origin, &first.request_id)
            .await
            .unwrap()
    );
    assert!(claude.read().await.unwrap().is_some());
    assert!(codex.read().await.unwrap().is_none());
    let waiting_store = codex.clone();
    let waiting_origin = origin.clone();
    let waiting_request = second.clone();
    let waiting = tokio::spawn(async move {
        super::wait_for_approval(&waiting_store, &waiting_origin, &waiting_request).await
    });
    fixture.decide(&second, false).await;
    let response = http
        .get(format!("{origin}/api/v1/connections/requests"))
        .header(
            "x-fetch-settings-capability",
            fixture.native_session.header_value(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let safe: Value = response.json().await.unwrap();
    let denied = safe["requests"]
        .as_array()
        .unwrap()
        .iter()
        .find(|request| request["request_id"] == second.request_id)
        .unwrap();
    assert_eq!(
        denied["status"], "denied",
        "native safe-list observation drives denied cleanup"
    );
    assert!(denied.get("claim_secret").is_none());
    assert!(
        !waiting.is_finished(),
        "pending connector is still waiting before native cleanup"
    );
    let created = fixture.enrollment.created.load(Ordering::SeqCst);
    let claims = fixture.enrollment.claimed.load(Ordering::SeqCst);
    assert!(
        codex
            .clear_pending(&origin, denied["request_id"].as_str().unwrap())
            .await
            .unwrap()
    );
    let error = tokio::time::timeout(Duration::from_secs(2), waiting)
        .await
        .expect("native denied cleanup should end the waiting connector promptly")
        .unwrap()
        .unwrap_err();
    assert!(error.contains("ended without saved access"));
    assert!(codex.read().await.unwrap().is_none());
    assert!(codex.pending(&origin).await.unwrap().is_none());
    assert_eq!(
        fixture.enrollment.created.load(Ordering::SeqCst),
        created,
        "denial must not automatically request access again"
    );
    assert_eq!(
        fixture.enrollment.claimed.load(Ordering::SeqCst),
        claims,
        "native denied cleanup must not claim credentials"
    );
    assert!(
        claude.read().await.unwrap().is_some(),
        "other approved client is preserved"
    );
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn denied_expired_or_restart_stale_pending_proof_clears_without_losing_an_existing_grant() {
    let mut fixture = Fixture::new().await;
    let options = fixture.options("denied");
    let origin = options.origin();
    let http = super::pinned_client(&options).unwrap();
    let denied = fixture.profile("denied");
    let pending = denied
        .ensure_pending(&http, &origin)
        .await
        .unwrap()
        .unwrap();
    denied.save_response(&fixture_grant()).await.unwrap();
    fixture.decide(&pending, false).await;
    assert!(
        denied
            .complete_pending(&http, &origin, &pending.request_id)
            .await
            .is_err()
    );
    assert!(denied.pending(&origin).await.unwrap().is_none());
    assert_eq!(
        denied.read().await.unwrap().unwrap().access_token,
        "existing-approved-access"
    );
    let expired = fixture.profile("expired");
    let pending = expired
        .ensure_pending(&http, &origin)
        .await
        .unwrap()
        .unwrap();
    expired.save_response(&fixture_grant()).await.unwrap();
    fixture.credentials.expire_pending("expired");
    assert!(
        !expired
            .complete_pending(&http, &origin, &pending.request_id)
            .await
            .unwrap()
    );
    assert!(expired.pending(&origin).await.unwrap().is_none());
    assert_eq!(
        expired.read().await.unwrap().unwrap().access_token,
        "existing-approved-access"
    );
    let stale = fixture.profile("stale");
    let pending = stale.ensure_pending(&http, &origin).await.unwrap().unwrap();
    stale.save_response(&fixture_grant()).await.unwrap();
    fixture.restart().await;
    assert!(
        stale
            .complete_pending(&http, &origin, &pending.request_id)
            .await
            .is_err()
    );
    assert!(stale.pending(&origin).await.unwrap().is_none());
    assert_eq!(
        stale.read().await.unwrap().unwrap().access_token,
        "existing-approved-access"
    );
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unsupported_idless_notification_reports_the_actual_backend_error_without_cross_connection_claim()
 {
    let fixture = Fixture::new().await;
    let mut bridge = fixture.bridge("claude").await;
    let before = fixture.peer.calls.load(Ordering::SeqCst);
    bridge
        .send(json!({"jsonrpc":"2.0","method":"notifications/roots/list_changed","params":{}}))
        .await;
    let error = tokio::time::timeout(Duration::from_secs(3), &mut bridge.task)
        .await
        .expect("backend refusal must end the connector promptly")
        .unwrap()
        .unwrap_err();
    assert!(error.contains("HTTP 400"), "{error}");
    assert!(error.contains("-32601"), "{error}");
    assert!(
        error.contains("This notification is not supported."),
        "{error}"
    );
    assert!(
        error.contains("notifications/roots/list_changed"),
        "{error}"
    );
    assert!(
        !error.contains("another connection"),
        "null notification error IDs are not foreign request IDs: {error}"
    );
    assert_eq!(
        fixture.peer.calls.load(Ordering::SeqCst),
        before,
        "the refused notification must not replay any tool call"
    );
    drop(bridge);
    fixture.finish().await;
}

#[tokio::test]
async fn foreign_nonnull_response_ids_remain_rejected_while_null_errors_keep_redacted_details() {
    let directory = tempfile::tempdir().unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let store = ProfileStore::with_backend(
        directory.path().to_owned(),
        "diagnostic".into(),
        Arc::new(MemoryCredentials::default()),
    )
    .unwrap();
    store.save_response(&json!({"client_id":"owned-client","access_token":"private-access-fixture","refresh_token":"private-refresh-fixture","expires_in":3600,"scope":"fetch:catalog fetch:tools:call"})).await.unwrap();
    let (write, mut writes) = mpsc::channel(4);
    let (fatal, _failures) = mpsc::unbounded_channel();
    let bridge = super::Bridge {
        http: reqwest::Client::new(),
        origin: "https://localhost:53672".into(),
        endpoint: "https://localhost:53672/mcp".into(),
        store,
        session: tokio::sync::RwLock::new(None),
        revision: tokio::sync::RwLock::new(LEGACY.into()),
        mapping: tokio::sync::Mutex::new(Default::default()),
        tools: tokio::sync::Mutex::new(BTreeMap::new()),
        sequence: std::sync::atomic::AtomicU64::new(0),
        prefix: "owned-fixture".into(),
        write,
        fatal,
        closed: super::CancelToken::new(),
        changes_started: std::sync::atomic::AtomicBool::new(false),
    };
    let error = bridge
        .receive(br#"{"jsonrpc":"2.0","id":"not-an-owned-request","result":{"content":[]}}"#)
        .await
        .unwrap_err();
    assert_eq!(
        error,
        "Fetch returned a response for another connection or an ended request."
    );
    assert!(
        matches!(
            writes.try_recv(),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty)
        ),
        "foreign responses must not reach the coding client"
    );
    let error = bridge.receive(br#"{"jsonrpc":"2.0","id":null,"error":{"code":-32099,"message":"Original upstream diagnostic","data":{"counter":184467440737095516160000013,"details":{"keep":true},"echo":"private-access-fixture","echo_renewal":"private-refresh-fixture"}}}"#).await.unwrap_err();
    for expected in [
        "-32099",
        "Original upstream diagnostic",
        "184467440737095516160000013",
        "details",
        "keep",
    ] {
        assert!(error.contains(expected), "{error}");
    }
    assert!(!error.contains("private-access-fixture"), "{error}");
    assert!(!error.contains("private-refresh-fixture"), "{error}");
    assert!(!error.contains("another connection"), "{error}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn maintained_modern_sdk_subscription_ack_changes_and_cancel_keep_other_calls_usable() {
    use rmcp::{
        ClientLifecycleMode, ClientServiceExt,
        model::{
            CallToolRequestParams, GetMeta, ProtocolVersion, ServerNotification, SubscriptionFilter,
        },
    };
    let fixture = Fixture::new().await;
    let Bridge {
        input,
        output,
        mut task,
        ..
    } = fixture.uninitialized_bridge("claude").await;
    let transport = tokio::io::join(input.into_inner().into_inner(), output);
    let client = tokio::time::timeout(
        Duration::from_secs(3),
        ().serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        ),
    )
    .await
    .expect("maintained SDK bootstrap must finish")
    .expect("maintained SDK accepts Fetch discovery");
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        client.listen(SubscriptionFilter::builder().tools_list_changed().build()),
    )
    .await;
    let mut subscription = match result {
        Ok(Ok(subscription)) => subscription,
        failed => {
            let reason = match failed {
                Ok(Err(error)) => format!("{error}"),
                Err(_) => "subscription acknowledgment timed out".into(),
                _ => unreachable!(),
            };
            let _ = tokio::time::timeout(Duration::from_secs(1), client.cancel()).await;
            task.abort();
            let _ = task.await;
            fixture.finish().await;
            panic!("maintained SDK must accept remapped subscription acknowledgment: {reason}");
        }
    };
    assert_eq!(
        subscription.acknowledged(),
        &SubscriptionFilter::builder().tools_list_changed().build()
    );
    let _ = fixture.runner.changes.send((
        "disposable".into(),
        "notifications/tools/list_changed".into(),
    ));
    let changed = tokio::time::timeout(Duration::from_secs(3), subscription.next())
        .await
        .expect("SDK receives catalog change")
        .expect("SDK accepts change metadata")
        .expect("subscription remains active");
    assert!(matches!(
        changed,
        ServerNotification::ToolListChangedNotification(_)
    ));
    assert_eq!(
        changed.get_meta().subscription_id(),
        Some(subscription.id().clone())
    );
    let arguments=serde_json::from_value::<CallToolRequestParams>(json!({"name":"fetch__call_tool","arguments":{"server_id":"disposable","tool_name":"echo","arguments":{"mode":"echo"}}})).unwrap();
    let result = tokio::time::timeout(Duration::from_secs(3), client.call_tool(arguments.clone()))
        .await
        .expect("call during live subscription completes")
        .expect("generic upstream call succeeds");
    assert_eq!(
        result.structured_content.unwrap()["number"].to_string(),
        "184467440737095516160000001"
    );
    tokio::time::timeout(Duration::from_secs(3), subscription.cancel())
        .await
        .expect("subscription cancel finishes")
        .expect("SDK can cancel owned subscription");
    let listed = tokio::time::timeout(Duration::from_secs(3), client.list_tools(None))
        .await
        .expect("listing after cancel completes")
        .expect("cancel did not close connector");
    assert_eq!(listed.tools.len(), 3);
    let _ = tokio::time::timeout(Duration::from_secs(3), client.call_tool(arguments))
        .await
        .expect("call after cancel completes")
        .expect("cancel did not affect normal tool calls");
    client.cancel().await.unwrap();
    tokio::time::timeout(Duration::from_secs(3), &mut task)
        .await
        .expect("SDK close ends its connector")
        .unwrap()
        .unwrap();
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn modern_same_profile_subscriptions_restore_equal_ids_and_cancel_only_the_owned_stream() {
    let fixture = Fixture::new().await;
    let mut first = fixture.uninitialized_bridge("claude").await;
    let mut second = fixture.uninitialized_bridge("claude").await;
    for bridge in [&mut first, &mut second] {
        bridge
            .send(modern_message(0, "server/discover", json!({})))
            .await;
        assert_eq!(bridge.response(0).await["result"]["resultType"], "complete");
        bridge
            .send(modern_message(
                7,
                "subscriptions/listen",
                json!({"notifications":{"toolsListChanged":true}}),
            ))
            .await;
        let ack = bridge.next().await;
        assert_eq!(ack["method"], "notifications/subscriptions/acknowledged");
        assert_eq!(
            ack["params"]["_meta"]["io.modelcontextprotocol/subscriptionId"], 7,
            "ACK must use that coding client's original request ID"
        );
    }
    let _ = fixture.runner.changes.send((
        "disposable".into(),
        "notifications/tools/list_changed".into(),
    ));
    for bridge in [&mut first, &mut second] {
        let changed = bridge.next().await;
        assert_eq!(changed["method"], "notifications/tools/list_changed");
        assert_eq!(
            changed["params"]["_meta"]["io.modelcontextprotocol/subscriptionId"],
            7
        );
    }
    first.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7,"_meta":{"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{}}}})).await;
    first.send(modern_message(8, "tools/list", json!({}))).await;
    assert_eq!(
        first.response(8).await["result"]["tools"]
            .as_array()
            .unwrap()
            .len(),
        3,
        "cancel must not close its connector"
    );
    let _ = fixture.runner.changes.send((
        "disposable".into(),
        "notifications/tools/list_changed".into(),
    ));
    let changed = second.next().await;
    assert_eq!(changed["method"], "notifications/tools/list_changed");
    assert_eq!(
        changed["params"]["_meta"]["io.modelcontextprotocol/subscriptionId"], 7,
        "other connector subscription remains live"
    );
    second.send(modern_message(8,"tools/call",json!({"name":"fetch__call_tool","arguments":{"server_id":"disposable","tool_name":"echo","arguments":{"mode":"echo"}}}))).await;
    assert_eq!(
        second.response(8).await["result"]["futureResult"]["keep"],
        true
    );
    second.send(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7,"_meta":{"io.modelcontextprotocol/protocolVersion":MODERN,"io.modelcontextprotocol/clientCapabilities":{}}}})).await;
    first.close().await;
    second.close().await;
    fixture.finish().await;
}

#[tokio::test]
async fn subscription_metadata_translation_leaves_nested_user_data_unchanged() {
    let directory = tempfile::tempdir().unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let store = ProfileStore::with_backend(
        directory.path().to_owned(),
        "nested-metadata".into(),
        Arc::new(MemoryCredentials::default()),
    )
    .unwrap();
    let (write, mut writes) = mpsc::channel(4);
    let (fatal, _failures) = mpsc::unbounded_channel();
    let bridge = super::Bridge {
        http: reqwest::Client::new(),
        origin: "https://localhost:53672".into(),
        endpoint: "https://localhost:53672/mcp".into(),
        store,
        session: tokio::sync::RwLock::new(None),
        revision: tokio::sync::RwLock::new(MODERN.into()),
        mapping: tokio::sync::Mutex::new(Default::default()),
        tools: tokio::sync::Mutex::new(BTreeMap::new()),
        sequence: std::sync::atomic::AtomicU64::new(0),
        prefix: "metadata-fixture".into(),
        write,
        fatal,
        closed: super::CancelToken::new(),
        changes_started: std::sync::atomic::AtomicBool::new(false),
    };
    let prepared=bridge.prepare(raw::raw(&json!({"jsonrpc":"2.0","id":7,"method":"subscriptions/listen","params":{"notifications":{"toolsListChanged":true}}}))).await.unwrap();
    let wire = raw::field(&prepared, "id").unwrap();
    let wire: Value = serde_json::from_str(wire.get()).unwrap();
    let nested = json!({"id":wire,"subscriptionId":wire,"_meta":{"io.modelcontextprotocol/subscriptionId":wire},"number":18446744073709551615u64});
    let ack = json!({"jsonrpc":"2.0","method":"notifications/subscriptions/acknowledged","params":{"_meta":{"io.modelcontextprotocol/subscriptionId":wire},"notifications":{"toolsListChanged":true},"userData":nested}});
    bridge
        .receive(&serde_json::to_vec(&ack).unwrap())
        .await
        .unwrap();
    let restored: Value = serde_json::from_slice(&writes.recv().await.unwrap()).unwrap();
    assert_eq!(
        restored["params"]["_meta"]["io.modelcontextprotocol/subscriptionId"],
        7
    );
    assert_eq!(
        restored["params"]["userData"], nested,
        "only the reserved envelope subscription reference is translated"
    );
    let result = json!({"jsonrpc":"2.0","id":wire,"result":{"_meta":{"io.modelcontextprotocol/subscriptionId":wire},"userData":nested}});
    bridge
        .receive(&serde_json::to_vec(&result).unwrap())
        .await
        .unwrap();
    let restored: Value = serde_json::from_slice(&writes.recv().await.unwrap()).unwrap();
    assert_eq!(restored["id"], 7);
    assert_eq!(
        restored["result"]["_meta"]["io.modelcontextprotocol/subscriptionId"],
        7
    );
    assert_eq!(restored["result"]["userData"], nested);
}
