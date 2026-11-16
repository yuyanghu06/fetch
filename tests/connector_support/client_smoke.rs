//! Opt-in real-client discovery against a disposable HTTPS peer.
//! The explicit fixture subprocess calls the production bridge with memory grants.
//! It does not exercise production Keychain or native approval.
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
};
use fetch_local::{
    connector::{self, Options, profile::ProfileStore},
    remote_oauth::{Backend, GrantError},
    runner::LEGACY,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    net::{Ipv4Addr, SocketAddr, TcpListener},
    os::unix::{fs::PermissionsExt, process::CommandExt},
    path::{Path, PathBuf},
    process::{ExitCode, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
};
use zeroize::Zeroizing;

#[derive(Default)]
struct MemoryBackend(Mutex<BTreeMap<(String, String), Vec<u8>>>);
impl Backend for MemoryBackend {
    fn read(&self, s: &str, a: &str) -> Result<Option<Zeroizing<Vec<u8>>>, GrantError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .get(&(s.into(), a.into()))
            .cloned()
            .map(Zeroizing::new))
    }
    fn write(&self, s: &str, a: &str, v: &[u8]) -> Result<(), GrantError> {
        self.0
            .lock()
            .unwrap()
            .insert((s.into(), a.into()), v.to_vec());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let result = if args.get(1).is_some_and(|v| v == "--fixture-stdio") {
        fixture_stdio(&args[2..]).await
    } else if std::env::var("FETCH_CONNECTOR_CLIENT_SMOKE").as_deref() != Ok("1") {
        println!(
            "SKIP connector_clients: set FETCH_CONNECTOR_CLIENT_SMOKE=1 for disposable, confined real-client discovery."
        );
        return ExitCode::SUCCESS;
    } else {
        match tokio::time::timeout(Duration::from_secs(70), smoke()).await {
            Ok(result) => result,
            Err(_) => Err(
                "Real-client smoke exceeded its fixed timeout; owned processes were stopped."
                    .into(),
            ),
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("connector_clients: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn fixture_stdio(args: &[String]) -> Result<(), String> {
    if args.len() != 3 {
        return Err("Fixture requires its disposable root, port and client label.".into());
    }
    let root = fs::canonicalize(&args[0]).map_err(|_| "Missing disposable connector root.")?;
    if root.parent() != Some(Path::new("/private/tmp"))
        || !root
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("fetch-connector-clients-")
        || fs::read_to_string(root.join(".connector-fixture"))
            .ok()
            .as_deref()
            != Some("Fetch confined connector fixture\n")
    {
        return Err("Refusing a fixture root without its task marker.".into());
    }
    let options = Options {
        root,
        port: args[1].parse().map_err(|_| "Invalid fixture port.")?,
        client: args[2].clone(),
        project_id:None,
    };
    let store = ProfileStore::with_backend(
        options.root.clone(),
        options.client.clone(),
        Arc::new(MemoryBackend::default()),
    )?;
    store.save_response(&json!({"client_id":"fixture-client","access_token":"fixture-approved-token","refresh_token":"fixture-refresh-token","expires_in":3600,"scope":"fetch:catalog fetch:tools:call fetch:content:read"})).await?;
    connector::run_with_io(options, store, tokio::io::stdin(), tokio::io::stdout()).await
}

#[derive(Default)]
struct Peer {
    initializations: AtomicUsize,
    lists: AtomicUsize,
    calls: AtomicUsize,
}
async fn endpoint(State(peer): State<Arc<Peer>>, headers: HeaderMap, body: String) -> Response {
    if headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        != Some("Bearer fixture-approved-token")
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let message: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let result=match message["method"].as_str().unwrap_or(""){
        "initialize"=>{peer.initializations.fetch_add(1,Ordering::SeqCst);json!({"protocolVersion":LEGACY,"capabilities":{"tools":{}},"serverInfo":{"name":"disposable Fetch connector fixture","version":"1"}})},
        "tools/list"=>{peer.lists.fetch_add(1,Ordering::SeqCst);json!({"tools":[{"name":"fixture_echo","description":"Return the supplied disposable marker","inputSchema":{"type":"object","properties":{"marker":{"type":"string"}},"required":["marker"]}}]})},
        "tools/call"=>{peer.calls.fetch_add(1,Ordering::SeqCst);json!({"content":[{"type":"text","text":message["params"]["arguments"]["marker"]}]})},
        "notifications/initialized"|"notifications/cancelled"=>return StatusCode::ACCEPTED.into_response(),
        _=>return ([(header::CONTENT_TYPE,"application/json")],json!({"jsonrpc":"2.0","id":message["id"],"error":{"code":-32601,"message":"Fixture supports ordinary MCP discovery only."}}).to_string()).into_response(),
    };
    (
        [
            ("content-type", "application/json"),
            ("mcp-session-id", "fixture-session"),
        ],
        json!({"jsonrpc":"2.0","id":message["id"],"result":result}).to_string(),
    )
        .into_response()
}

struct OwnedProcess(Child);
impl OwnedProcess {
    async fn stop(&mut self) {
        if let Some(id) = self.0.id() {
            unsafe {
                libc::kill(-(id as i32), libc::SIGKILL);
            }
        }
        let _ = self.0.start_kill();
        let _ = tokio::time::timeout(Duration::from_secs(3), self.0.wait()).await;
    }
}
impl Drop for OwnedProcess {
    fn drop(&mut self) {
        if let Some(id) = self.0.id() {
            unsafe {
                libc::kill(-(id as i32), libc::SIGKILL);
            }
        }
        let _ = self.0.start_kill();
    }
}
struct HttpsFixture {
    handle: axum_server::Handle<SocketAddr>,
    task: tokio::task::JoinHandle<std::io::Result<()>>,
}
impl Drop for HttpsFixture {
    fn drop(&mut self) {
        self.handle.shutdown();
        self.task.abort();
    }
}

fn executable(name: &str) -> Result<PathBuf, String> {
    for folder in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        let path = folder.join(name);
        if path.is_file() {
            return fs::canonicalize(path).map_err(|_| format!("Cannot resolve installed {name}."));
        }
    }
    Err(format!(
        "Installed {name} executable not found; no installation attempted."
    ))
}
fn copy_executable(source: &Path, target: &Path) -> Result<(), String> {
    fs::copy(source, target)
        .map_err(|_| "Could not copy public client/test executable into fixture.")?;
    fs::set_permissions(target, fs::Permissions::from_mode(0o700))
        .map_err(|_| "Could not set fixture executable permissions.".into())
}
fn folder(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|_| "Could not prepare private fixture directories.")?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|_| "Could not protect fixture directory.".into())
}
fn sandbox(profile: &str, root: &Path, program: &Path) -> Command {
    let mut command = Command::new("/usr/bin/sandbox-exec");
    command.args(["-p", profile]).arg(program).env_clear();
    command
        .env("HOME", root.join("home"))
        .env("TMPDIR", root.join("tmp"))
        .env("PATH", "/usr/bin:/bin")
        .env("LANG", "en_US.UTF-8")
        .env("TERM", "dumb")
        .current_dir(root.join("workspace"))
        .kill_on_drop(true);
    command.as_std_mut().process_group(0);
    command
}

async fn prove_confinement(
    profile: &str,
    root: &Path,
    python: &Path,
    port: u16,
) -> Result<(), String> {
    let outside = tempfile::Builder::new()
        .prefix("fetch-connector-outside-")
        .tempdir_in("/private/tmp")
        .map_err(|_| "Cannot create disposable outside canaries.")?;
    let read = outside.path().join("read");
    let write = outside.path().join("write");
    fs::write(&read, "disposable denied marker").map_err(|_| "Cannot prepare read canary.")?;
    let blocked = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .map_err(|_| "Cannot bind disposable denied listener.")?;
    let script = r#"import ctypes,json,os,socket
from pathlib import Path
r={}
for n,op in [('read',lambda:Path(os.environ['OUTSIDE_READ']).read_bytes()),('write',lambda:Path(os.environ['OUTSIDE_WRITE']).write_text('must-not-write'))]:
 try:op();r[n]='unexpected-success'
 except OSError as e:r[n]=e.errno
p=Path(os.environ['HOME'])/'owned-canary';p.write_text('owned');r['owned']=p.read_text()
for n,port in [('allowed_network',os.environ['ALLOWED_PORT']),('blocked_network',os.environ['BLOCKED_PORT'])]:
 try:
  with socket.socket() as s:s.settimeout(2);s.connect(('127.0.0.1',int(port)))
  r[n]='connected'
 except OSError as e:r[n]=e.errno
lib=ctypes.CDLL('/usr/lib/libSystem.B.dylib')
task=ctypes.c_uint.in_dll(lib,'mach_task_self_').value
bootstrap=ctypes.c_uint()
r['bootstrap_port']=lib.task_get_special_port(task,4,ctypes.byref(bootstrap))
for name in ['com.apple.securityd','com.apple.securityd.xpc','com.apple.SecurityServer']:
 port=ctypes.c_uint()
 r[name]=lib.bootstrap_look_up(bootstrap,name.encode(),ctypes.byref(port))
print(json.dumps(r))
"#;
    let mut command = sandbox(profile, root, python);
    command
        .args(["-I", "-S", "-B", "-c", script])
        .env("OUTSIDE_READ", &read)
        .env("OUTSIDE_WRITE", &write)
        .env("ALLOWED_PORT", port.to_string())
        .env(
            "BLOCKED_PORT",
            blocked.local_addr().unwrap().port().to_string(),
        );
    let output = tokio::time::timeout(Duration::from_secs(10), command.output())
        .await
        .map_err(|_| "Confinement probe timed out.")?
        .map_err(|_| "Could not run confinement probe.")?;
    if !output.status.success() {
        return Err(format!(
            "Confinement probe could not run; no real clients were started. {}",
            String::from_utf8_lossy(&output.stderr).replace(root.to_str().unwrap(), "<fixture>")
        ));
    }
    let result: Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| "Confinement probe did not return readable results.")?;
    if result["read"] != libc::EPERM
        || result["write"] != libc::EPERM
        || result["blocked_network"] != libc::EPERM
        || result["owned"] != "owned"
        || result["allowed_network"] != "connected"
        || [
            "com.apple.securityd",
            "com.apple.securityd.xpc",
            "com.apple.SecurityServer",
        ]
        .into_iter()
        .any(|name| result[name] != 1100)
    {
        return Err(format!(
            "Confinement not established; no real clients started. Disposable probe: {result}"
        ));
    }
    if write.exists()
        || fs::read_to_string(&read).ok().as_deref() != Some("disposable denied marker")
        || tokio::time::timeout(Duration::from_millis(30), blocked.accept())
            .await
            .is_ok()
    {
        return Err(
            "Outside canary changed or forbidden listener reached; no real clients started.".into(),
        );
    }
    Ok(())
}

async fn smoke() -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Err("This isolated real-client check requires macOS sandbox-exec.".into());
    }
    let directory = tempfile::Builder::new()
        .prefix("fetch-connector-clients-")
        .tempdir_in("/private/tmp")
        .map_err(|_| "Cannot create disposable real-client root.")?;
    let root = fs::canonicalize(directory.path()).map_err(|_| "Cannot resolve fixture root.")?;
    folder(&root)?;
    for name in [
        "home",
        "tmp",
        "workspace",
        "codex-home",
        "codex-sqlite",
        "claude-home",
    ] {
        folder(&root.join(name))?;
    }
    fs::write(
        root.join(".connector-fixture"),
        "Fetch confined connector fixture\n",
    )
    .map_err(|_| "Cannot create fixture marker.")?;
    let codex = root.join("codex");
    let claude = root.join("claude");
    let fixture = root.join("connector-fixture");
    copy_executable(&executable("codex")?, &codex)?;
    copy_executable(&executable("claude")?, &claude)?;
    copy_executable(
        &std::env::current_exe().map_err(|_| "Cannot resolve fixture executable.")?,
        &fixture,
    )?;
    let python_framework =
        Path::new("/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework");
    let python = fs::canonicalize(
        python_framework.join("Versions/3.9/Resources/Python.app/Contents/MacOS/Python"),
    )
    .map_err(|_| "Public CLT Python unavailable; no client started.")?;
    let peer = Arc::new(Peer::default());
    let rcgen::CertifiedKey { cert, signing_key } =
        rcgen::generate_simple_self_signed(vec!["localhost".into()])
            .map_err(|_| "Cannot create fixture HTTPS identity.")?;
    fs::write(root.join("certificate.pem"), cert.pem())
        .map_err(|_| "Cannot save public fixture certificate.")?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .map_err(|_| "Cannot bind fixture HTTPS listener.")?;
    let port = listener.local_addr().unwrap().port();
    listener.set_nonblocking(true).unwrap();
    let tls = axum_server::tls_rustls::RustlsConfig::from_der(
        vec![cert.der().to_vec()],
        signing_key.serialize_der(),
    )
    .await
    .map_err(|_| "Cannot prepare fixture HTTPS.")?;
    let app = Router::new()
        .route(
            "/mcp",
            post(endpoint)
                .get(|| async {
                    axum::response::Sse::new(tokio_stream::pending::<
                        Result<axum::response::sse::Event, std::convert::Infallible>,
                    >())
                })
                .delete(|| async { StatusCode::NO_CONTENT }),
        )
        .with_state(peer.clone());
    let handle = axum_server::Handle::new();
    let server_handle = handle.clone();
    let task = tokio::spawn(async move {
        axum_server::from_tcp_rustls(listener, tls)
            .unwrap()
            .handle(server_handle)
            .serve(app.into_make_service())
            .await
    });
    let https = HttpsFixture { handle, task };
    tokio::time::timeout(Duration::from_secs(5), https.handle.listening())
        .await
        .map_err(|_| "Fixture HTTPS did not listen.")?;
    let profile = format!(
        r#"(version 1)
(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (subpath "/usr/share") (subpath {framework}) (subpath {root}) (literal "/dev/null") (literal "/dev/random") (literal "/dev/urandom"))
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") (subpath {framework}) (subpath {root}))
(allow file-write* (subpath {root}) (literal "/dev/null"))
(allow process-exec (subpath {root}) (literal {python}) (literal "/bin/sh"))
(allow process-fork)
(allow sysctl-read)
(allow network-outbound (remote ip "localhost:{port}"))
"#,
        framework = json!(python_framework),
        root = json!(root),
        python = json!(python)
    );
    prove_confinement(&profile, &root, &python, port).await?;
    println!(
        "PASS confinement: owned files/backend reachable; outside files, unrelated listener and Keychain Mach lookup denied."
    );
    let codex_result = codex_smoke(&profile, &root, &codex, &fixture, port, &peer).await;
    let codex_counts = (
        peer.initializations.load(Ordering::SeqCst),
        peer.lists.load(Ordering::SeqCst),
        peer.calls.load(Ordering::SeqCst),
    );
    println!(
        "Codex fixture MCP requests: initialize={}, tools/list={}, tools/call={}",
        codex_counts.0, codex_counts.1, codex_counts.2
    );
    let claude_result = claude_smoke(&profile, &root, &claude, &fixture, port, &peer).await;
    println!(
        "Claude fixture MCP requests: initialize={}, tools/list={}, tools/call={}",
        peer.initializations.load(Ordering::SeqCst) - codex_counts.0,
        peer.lists.load(Ordering::SeqCst) - codex_counts.1,
        peer.calls.load(Ordering::SeqCst) - codex_counts.2
    );
    match (&codex_result, &claude_result) {
        (Ok(_), Ok(_)) => {}
        _ => {
            return Err(format!(
                "Protocol-only results: Codex={}; Claude={}. No model turn, real account/credential, native approval or production Keychain was used; Claude receives only an inert fixture key marker.",
                codex_result
                    .as_ref()
                    .err()
                    .map(String::as_str)
                    .unwrap_or("PASS"),
                claude_result
                    .as_ref()
                    .err()
                    .map(String::as_str)
                    .unwrap_or("PASS")
            ));
        }
    }
    println!(
        "PASS real-client discovery: Codex and Claude reached production stdio connector and disposable HTTPS peer; native approval/Keychain/shipped CLI remain separate acceptance."
    );
    Ok(())
}

async fn codex_smoke(
    profile: &str,
    root: &Path,
    codex: &Path,
    fixture: &Path,
    port: u16,
    peer: &Arc<Peer>,
) -> Result<(), String> {
    let config = format!(
        "cli_auth_credentials_store = \"file\"\nmcp_oauth_credentials_store = \"file\"\n[features]\napps = false\nplugins = false\nremote_plugin = false\n[analytics]\nenabled = false\n[mcp_servers.fetch]\ncommand = {}\nargs = {}\n",
        json!(fixture),
        json!([
            "--fixture-stdio",
            root.to_str().unwrap(),
            &port.to_string(),
            "codex"
        ])
    );
    fs::write(root.join("codex-home/config.toml"), config)
        .map_err(|_| "Cannot write disposable Codex settings.")?;
    let mut command = sandbox(profile, root, codex);
    command
        .args(["--no-daemon", "app-server", "--stdio"])
        .env("CODEX_HOME", root.join("codex-home"))
        .env("CODEX_SQLITE_HOME", root.join("codex-sqlite"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = OwnedProcess(
        command
            .spawn()
            .map_err(|_| "Confined Codex process did not start.")?,
    );
    let mut input = child.0.stdin.take().unwrap();
    let mut lines = BufReader::new(child.0.stdout.take().unwrap()).lines();
    let outcome=tokio::time::timeout(Duration::from_secs(18),async{
        input.write_all(format!("{}\n",json!({"id":1,"method":"initialize","params":{"clientInfo":{"name":"fetch-connector-check","version":"1"},"capabilities":{"experimentalApi":true}}})).as_bytes()).await.map_err(|_|"Codex input closed.")?;
        loop{let line=lines.next_line().await.map_err(|_|"Cannot read Codex protocol.")?.ok_or("Codex exited before initialization.")?;let value:Value=serde_json::from_str(&line).map_err(|_|"Codex stdout was not protocol JSON.")?;if value["id"]==1{if value.get("error").is_some(){return Err("Codex refused protocol initialization.".into())}break;}}
        input.write_all(b"{\"method\":\"initialized\",\"params\":{}}\n").await.map_err(|_|"Codex input closed.")?;
        for id in 2..12 {
            input.write_all(format!("{}\n",json!({"id":id,"method":"mcpServerStatus/list","params":{}})).as_bytes()).await.map_err(|_|"Codex status input closed.")?;
            loop{let line=lines.next_line().await.map_err(|_|"Cannot read Codex status.")?.ok_or("Codex exited before MCP discovery.")?;let value:Value=serde_json::from_str(&line).map_err(|_|"Codex status was not JSON.")?;if value["id"]==id {if value.get("error").is_some(){return Err(format!("Installed Codex rejected protocol-only MCP status request: {}",value["error"]))}let text=value.to_string();if text.contains("fixture_echo")&&peer.lists.load(Ordering::SeqCst)>0{return Ok(())}break;}}
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
        Err("Codex did not discover fixture_echo; no account was supplied.".into())
    }).await.unwrap_or_else(|_|Err("Codex protocol discovery timed out; no account was supplied.".into()));
    child.stop().await;
    if outcome.is_ok() {
        println!(
            "PASS Codex: real client initialized and discovered fixture_echo without a model/account."
        );
    }
    outcome
}

async fn claude_smoke(
    profile: &str,
    root: &Path,
    claude: &Path,
    fixture: &Path,
    port: u16,
    peer: &Arc<Peer>,
) -> Result<(), String> {
    let config = root.join("claude-mcp.json");
    fs::write(&config,json!({"mcpServers":{"fetch":{"type":"stdio","command":fixture,"args":["--fixture-stdio",root,port.to_string(),"claude"]}}}).to_string()).map_err(|_|"Cannot write disposable Claude MCP settings.")?;
    let before = peer.initializations.load(Ordering::SeqCst);
    let mut command = sandbox(profile, root, claude);
    command
        .args([
            "--bare",
            "--print",
            "--input-format",
            "stream-json",
            "--output-format",
            "stream-json",
            "--verbose",
            "--strict-mcp-config",
            "--mcp-config",
        ])
        .arg(&config)
        .args(["--setting-sources", ""])
        .arg("--debug-file")
        .arg(root.join("claude-debug.log"))
        .env("CLAUDE_CONFIG_DIR", root.join("claude-home"))
        .env("CLAUDE_CODE_TMPDIR", root.join("tmp"))
        .env("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1")
        // --bare only reads an explicit environment key. This inert marker
        // satisfies local presence checks; no model turn is sent and the
        // verified sandbox denies every public/provider network destination.
        .env("ANTHROPIC_API_KEY", "disposable-fixture-not-a-real-key")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = OwnedProcess(
        command
            .spawn()
            .map_err(|_| "Confined Claude process did not start.")?,
    );
    let mut input = child.0.stdin.take().unwrap();
    let mut lines = BufReader::new(child.0.stdout.take().unwrap()).lines();
    let outcome=tokio::time::timeout(Duration::from_secs(18),async{
        input.write_all(format!("{}\n",json!({"type":"control_request","request_id":"fixture-init","request":{"subtype":"initialize"}})).as_bytes()).await.map_err(|_|"Claude protocol input closed.")?;
        loop{
            let line=lines.next_line().await.map_err(|_|"Cannot read Claude protocol.")?.ok_or("Claude exited before protocol initialization; no account was supplied.")?;
            let value:Value=serde_json::from_str(&line).map_err(|_|"Claude stdout was not protocol JSON.")?;
            if value["type"]=="control_response"&&value["response"]["request_id"]=="fixture-init"{break;}
        }
        for _ in 0..10 {
            if peer.initializations.load(Ordering::SeqCst)>before {return Ok(())}
            input.write_all(format!("{}\n",json!({"type":"control_request","request_id":"fixture-status","request":{"subtype":"mcp_status"}})).as_bytes()).await.map_err(|_|"Claude MCP status input closed.")?;
            tokio::select!{line=lines.next_line()=>{if line.map_err(|_|"Cannot read Claude MCP status.")?.is_none(){return Err("Claude exited before MCP initialization; no account supplied.".into())}},_=tokio::time::sleep(Duration::from_millis(300))=>{}}
        }
        Err("Claude did not initialize its MCP connection without an account/model turn.".into())
    }).await.unwrap_or_else(|_|Err("Claude protocol discovery timed out; no account/model turn was supplied.".into()));
    child.stop().await;
    let mut outcome = outcome;
    if outcome.is_err() {
        let mut diagnostics = String::new();
        if let Some(mut stderr) = child.0.stderr.take() {
            let _ = tokio::time::timeout(
                Duration::from_secs(1),
                stderr.read_to_string(&mut diagnostics),
            )
            .await;
        }
        if let Ok(debug) = fs::read_to_string(root.join("claude-debug.log")) {
            let relevant = debug
                .lines()
                .filter(|line| {
                    line.contains("MCP")
                        || line.contains("mcp")
                        || line.contains("EPERM")
                        || line.contains("[ERROR]")
                })
                .take(12)
                .collect::<Vec<_>>()
                .join(" | ");
            diagnostics.push_str(&relevant);
        }
        diagnostics = diagnostics.replace(root.to_str().unwrap(), "<fixture>");
        diagnostics.truncate(diagnostics.len().min(1800));
        if !diagnostics.trim().is_empty() {
            outcome = Err(format!(
                "{} Fixture-only diagnostic: {}",
                outcome.err().unwrap(),
                diagnostics.trim()
            ));
        }
    }
    if outcome.is_ok() {
        println!(
            "PASS Claude Code: real client initialized its production connector without a model/account."
        );
    }
    outcome
}
