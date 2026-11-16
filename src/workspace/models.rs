//! Read the installed coding app's model catalog without submitting a prompt.
//!
//! Codex is read from its existing saved catalog: app-server startup can migrate
//! or compress session files, which would violate Workspace's read-only access.
//! Claude uses an on-demand initialization request without a user prompt. Its
//! safe mode skips personal customizations, but managed policy hooks can apply.
//! Neither source is a successful model-access test or entitlement guarantee.
use super::{App, WorkspaceError, WorkspaceService, success};
use serde_json::{Value, json};
use std::{path::{Path, PathBuf}, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdin, ChildStdout, Command},
};

const CATALOG_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_STDOUT_BYTES: u64 = 2 * 1024 * 1024;
const MAX_STDERR_BYTES: usize = 16 * 1024;

/// Called only when the user asks to refresh the model choices.
pub async fn catalog(app: &str) -> Result<Value, WorkspaceError> {
    catalog_for(&WorkspaceService::from_environment()?, app).await
}

/// The API supplies its configured roots; tests can inject disposable app data.
pub async fn catalog_for(workspace: &WorkspaceService, app: &str) -> Result<Value, WorkspaceError> {
    let app = App::parse(app)?;
    match app {
        App::Codex => {
            catalog_from_codex_cache(&workspace.codex_root.join("models_cache.json")).await
        }
        App::Claude => catalog_with_command(app.as_str(), Command::new(claude_executable())).await,
    }
}

/// Read a saved catalog without asking Codex to start, refresh or modify it.
pub async fn catalog_from_codex_cache(path: &Path) -> Result<Value, WorkspaceError> {
    let file = tokio::fs::File::open(path).await.map_err(|error| {
        WorkspaceError::new(503, "WORKSPACE_MODEL_CATALOG_UNAVAILABLE", format!(
            "Could not read Codex's saved model catalog at {}: {}. Open Codex to populate its catalog, then refresh here.",
            path.display(), error,
        ))
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_STDOUT_BYTES + 1).read_to_end(&mut bytes).await.map_err(protocol_io)?;
    if bytes.len() as u64 > MAX_STDOUT_BYTES {
        return Err(protocol_error("Codex's saved model catalog exceeded 2 MiB."));
    }
    let cache: Value = serde_json::from_slice(&bytes).map_err(|error| protocol_error(error.to_string()))?;
    let models = cache.get("models").and_then(Value::as_array).ok_or_else(|| {
        protocol_error("Codex's saved catalog contains no models array.")
    })?.iter().filter(|model| model.get("visibility").and_then(Value::as_str) == Some("list"))
        .cloned().collect::<Vec<_>>();
    // identity, ETag and account information are not part of the picker result.
    // A saved cache is not revalidated against the current CLI version or login.
    Ok(success(json!({"app":"codex", "source":"codex_cache", "fetched_at":cache.get("fetched_at"), "models":models})))
}

fn claude_executable() -> PathBuf {
    let mut candidates = std::env::var_os("PATH").map(|paths| {
        std::env::split_paths(&paths).map(|path|path.join("claude")).collect::<Vec<_>>()
    }).unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME") {
        candidates.push(PathBuf::from(home).join(".local/bin/claude"));
    }
    candidates.push(PathBuf::from("/opt/homebrew/bin/claude"));
    candidates.push(PathBuf::from("/usr/local/bin/claude"));
    candidates.into_iter().find(|path| {
        path.metadata().is_ok_and(|metadata| {
            if !metadata.is_file() { return false; }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                metadata.permissions().mode() & 0o111 != 0
            }
            #[cfg(not(unix))]
            { true }
        })
    }).unwrap_or_else(||PathBuf::from("claude"))
}

/// Supply a fake executable in tests, or an explicitly resolved Claude CLI.
/// Arguments are appended as individual values; no shell command is constructed.
pub async fn catalog_with_command(
    app: &str,
    mut command: Command,
) -> Result<Value, WorkspaceError> {
    let app = App::parse(app)?;
    if app == App::Codex {
        return Err(protocol_error("Codex models are read from its saved catalog; Workspace does not start Codex app-server."));
    }
    command.args([
                "--print", "--input-format", "stream-json", "--output-format",
                "stream-json", "--verbose", "--safe-mode", "--strict-mcp-config",
                "--mcp-config", "{\"mcpServers\":{}}", "--tools", "",
                "--no-session-persistence",
            ]);
    command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    command.kill_on_drop(true);
    let mut child = command.spawn().map_err(|error| {
        WorkspaceError::new(503, "WORKSPACE_MODEL_CATALOG_FAILED", error.to_string())
    })?;
    // All three handles are guaranteed by the Stdio::piped settings above.
    let mut stdin = child.stdin.take().expect("catalog stdin is piped");
    let stdout = child.stdout.take().expect("catalog stdout is piped");
    let mut stderr = child.stderr.take().expect("catalog stderr is piped");
    let mut stderr_task = tokio::spawn(async move {
        let mut captured = Vec::new();
        let mut buffer = [0u8; 4096];
        while let Ok(count) = stderr.read(&mut buffer).await {
            if count == 0 { break; }
            let remaining = MAX_STDERR_BYTES.saturating_sub(captured.len());
            captured.extend_from_slice(&buffer[..count.min(remaining)]);
        }
        String::from_utf8_lossy(&captured).into_owned()
    });
    let exchange = async {
        let mut output = CatalogOutput::new(stdout);
        claude_catalog(&mut stdin, &mut output).await
    };
    let result = match tokio::time::timeout(CATALOG_TIMEOUT, exchange).await {
        Ok(result) => result,
        Err(_) => Err(WorkspaceError::new(
            504, "WORKSPACE_MODEL_CATALOG_TIMEOUT",
            "The coding app did not return its model catalog within 10 seconds.",
        )),
    };
    // Stop and reap only the child created for this request, including success.
    drop(stdin);
    let _ = child.start_kill();
    let cleanup = child.wait().await;
    // A descendant could keep stderr open after the CLI exits. Bound that drain.
    let captured_stderr = match tokio::time::timeout(Duration::from_secs(1), &mut stderr_task).await {
        Ok(Ok(text)) => text,
        _ => { stderr_task.abort(); String::new() }
    };
    match result {
        Ok(models) => {
            cleanup.map_err(|error| WorkspaceError::new(
                503, "WORKSPACE_MODEL_CATALOG_FAILED", error.to_string(),
            ))?;
            Ok(success(json!({"app":app.as_str(), "models":models})))
        }
        Err(mut error) => {
            if !captured_stderr.trim().is_empty() {
                error.message.push('\n');
                error.message.push_str(captured_stderr.trim());
            }
            Err(error)
        }
    }
}

struct CatalogOutput {
    // Take bounds an individual line as well as the cumulative stream. A CLI
    // writing one enormous line cannot grow read_line's allocation indefinitely.
    reader: BufReader<tokio::io::Take<ChildStdout>>,
    consumed: u64,
}

impl CatalogOutput {
    fn new(stdout: ChildStdout) -> Self {
        Self { reader: BufReader::new(stdout.take(MAX_STDOUT_BYTES + 1)), consumed: 0 }
    }
    async fn next(&mut self) -> Result<Value, WorkspaceError> {
        loop {
            let mut line = String::new();
            let read = self.reader.read_line(&mut line).await.map_err(protocol_io)?;
            self.consumed += read as u64;
            if self.consumed > MAX_STDOUT_BYTES {
                return Err(protocol_error("The coding app's catalog output exceeded 2 MiB."));
            }
            if read == 0 {
                return Err(protocol_error("The coding app closed its output before returning the model catalog."));
            }
            if line.trim().is_empty() { continue; }
            return serde_json::from_str(&line).map_err(|error| {
                protocol_error(format!("{}\n{}", error, line.trim()))
            });
        }
    }
}

async fn send(input: &mut ChildStdin, value: Value) -> Result<(), WorkspaceError> {
    let mut bytes = serde_json::to_vec(&value).map_err(|error| protocol_error(error.to_string()))?;
    bytes.push(b'\n');
    input.write_all(&bytes).await.map_err(protocol_io)?;
    input.flush().await.map_err(protocol_io)
}

async fn claude_catalog(input: &mut ChildStdin, output: &mut CatalogOutput) -> Result<Vec<Value>, WorkspaceError> {
    send(input, json!({"type":"control_request","request_id":"fetch-models", "request":{
        "subtype":"initialize", "hooks":null, "skills":[]
    }})).await?;
    loop {
        let response = output.next().await?;
        if response.get("type").and_then(Value::as_str) != Some("control_response") { continue; }
        let envelope = &response["response"];
        if envelope.get("request_id").and_then(Value::as_str) != Some("fetch-models") { continue; }
        if envelope.get("subtype").and_then(Value::as_str) == Some("error") {
            return Err(protocol_error(envelope.get("error").unwrap_or(envelope).to_string()));
        }
        return envelope["response"]["models"].as_array().cloned().ok_or_else(|| {
            protocol_error(format!("The coding app returned no model list: {response}"))
        });
    }
}

fn protocol_error(message: impl Into<String>) -> WorkspaceError {
    WorkspaceError::new(503, "WORKSPACE_MODEL_CATALOG_FAILED", message)
}
fn protocol_io(error: std::io::Error) -> WorkspaceError { protocol_error(error.to_string()) }

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
