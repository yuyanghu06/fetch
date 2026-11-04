//! macOS-only, deny-default Node fixture confinement shared by helper bridge tests.
#![allow(dead_code)]
use std::{path::{Path, PathBuf}, process::Stdio};
use tempfile::TempDir;
use tokio::process::Command;

pub struct Sandbox {
    pub directory: TempDir,
    pub root: PathBuf,
    pub node: PathBuf,
    pub home: PathBuf,
    profile: String,
}

impl Sandbox {
    pub fn new() -> Self {
        let directory = tempfile::Builder::new().prefix("fetch-helper-fixture-").tempdir_in("/private/tmp").unwrap();
        let root = std::fs::canonicalize(directory.path()).unwrap();
        let home = root.join("home");
        std::fs::create_dir(&home).unwrap();
        let source = std::env::var_os("FETCH_HELPER_FIXTURE_NODE").map(PathBuf::from).unwrap_or_else(|| {
            let home = std::env::var_os("HOME").expect("test runtime lookup needs the current home");
            PathBuf::from(home).join("Library/Application Support/Fetch-Development/packages/runtimes/node/24.22.1/bin/node")
        });
        assert!(source.is_file(), "Stage the public Node24 runtime or set FETCH_HELPER_FIXTURE_NODE; no runtime is downloaded by this test");
        let node = root.join("node");
        std::fs::copy(&source, &node).unwrap();
        let profile = format!(r#"(version 1)
(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (subpath {root})
  (literal "/dev/null") (literal "/dev/random") (literal "/dev/urandom"))
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") (subpath {root}))
(allow file-write* (subpath {root}) (literal "/dev/null"))
(allow process-exec (literal {node}))
(allow process-fork)
(allow sysctl-read)
"#, root=quoted(&root), node=quoted(&node));
        Self { directory, root, node, home, profile }
    }

    pub fn command(&self) -> Command {
        let mut command = Command::new("/usr/bin/sandbox-exec");
        command.args(["-p", self.profile.as_str()]).arg(&self.node).env_clear()
            .env("HOME", &self.home).env("TMPDIR", &self.root).env("PATH", "/usr/bin:/bin")
            .current_dir(&self.home).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).kill_on_drop(true);
        command
    }

    pub fn allow_bridge(&mut self, socket: &Path) {
        // The sole network exception is one task-created Unix socket. TCP,
        // other Unix sockets and Mach/Keychain service access remain denied.
        self.profile.push_str(&format!("(allow network-outbound (remote unix-socket (path-literal {})))\n", quoted(socket)));
        self.profile.push_str(&format!("(allow file-read* file-write* (literal {}))\n", quoted(socket)));
    }

    pub fn write(&self, relative: &str, content: impl AsRef<[u8]>) -> PathBuf {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).unwrap(); }
        std::fs::write(&path, content).unwrap();
        path
    }

    /// A trusted fixture launcher enters confinement before executing any Node
    /// adapter code. The owning bridge supplies its one private socket at launch.
    pub fn launcher(&self) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let source = format!(r#"#!/bin/sh
profile={profile}
profile="$profile
(allow network-outbound (remote unix-socket (path-literal \"$FETCH_HELPER_SOCKET\")))
(allow file-read* file-write* (literal \"$FETCH_HELPER_SOCKET\"))"
exec /usr/bin/sandbox-exec -p "$profile" {node} "$@"
"#, profile=shell_quoted(&self.profile), node=shell_quoted(self.node.to_str().unwrap()));
        let path = self.write("confined-node", source);
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
        path
    }

    /// Check disposable positive and negative canaries before any helper fixture.
    pub async fn assert_confinement(&self) {
        let outside = tempfile::Builder::new().prefix("fetch-helper-outside-").tempdir_in("/private/tmp").unwrap();
        let outside_read = outside.path().join("read-canary");
        let outside_write = outside.path().join("write-canary");
        std::fs::write(&outside_read, "disposable outside marker").unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let source = r#"
const fs = require('node:fs');
const net = require('node:net');
const result = {};
for (const [name, operation] of [
  ['outside_read', () => fs.readFileSync(process.env.OUTSIDE_READ)],
  ['outside_write', () => fs.writeFileSync(process.env.OUTSIDE_WRITE, 'must not write')]
]) { try { operation(); result[name] = 'unexpected-success'; } catch (error) { result[name] = error.code; } }
fs.writeFileSync(process.env.HOME + '/owned-canary', 'owned write works');
result.owned = fs.readFileSync(process.env.HOME + '/owned-canary', 'utf8');
const socket = net.createConnection({host:'127.0.0.1',port:Number(process.env.OUTSIDE_PORT)});
const timer = setTimeout(() => { socket.destroy(); result.network = 'timeout'; finish(); }, 1000);
socket.on('connect', () => { result.network = 'unexpected-success'; socket.destroy(); finish(); });
socket.on('error', error => { result.network = error.code; finish(); });
function finish() { clearTimeout(timer); process.stdout.write(JSON.stringify(result)); }
"#;
        let mut command = self.command();
        command.args(["-e", source]).env("OUTSIDE_READ", &outside_read).env("OUTSIDE_WRITE", &outside_write)
            .env("OUTSIDE_PORT", port.to_string());
        let output = tokio::time::timeout(std::time::Duration::from_secs(5), command.output()).await.unwrap().unwrap();
        assert!(output.status.success(), "sandbox fixture startup failed ({:?}), stdout={:?}, stderr={:?}", output.status, String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["outside_read"], "EPERM");
        assert_eq!(result["outside_write"], "EPERM");
        assert_eq!(result["network"], "EPERM");
        assert_eq!(result["owned"], "owned write works");
        assert!(!outside_write.exists());
        assert_eq!(std::fs::read_to_string(&outside_read).unwrap(), "disposable outside marker");
        assert!(tokio::time::timeout(std::time::Duration::from_millis(20), listener.accept()).await.is_err());
    }
}

fn quoted(path: &Path) -> String { serde_json::to_string(path.to_str().unwrap()).unwrap() }
fn shell_quoted(text: &str) -> String { format!("'{}'", text.replace('\'', "'\\''")) }
