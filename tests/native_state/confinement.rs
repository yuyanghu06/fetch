//! Deny-default process sandbox, independent of Fetch's provider/key storage.
use std::{path::{Path, PathBuf}, process::Stdio};
use tempfile::TempDir;
use tokio::process::Command;

pub struct Sandbox {
    pub _directory: TempDir,
    pub root: PathBuf,
    pub home: PathBuf,
    pub launcher: PathBuf,
    pub peer: PathBuf,
    python: PathBuf,
    profile: String,
}
impl Sandbox {
    pub fn new() -> Self {
        use std::os::unix::fs::PermissionsExt;
        let directory = tempfile::Builder::new().prefix("fetch-native-fixture-").tempdir_in("/private/tmp").unwrap();
        let root = std::fs::canonicalize(directory.path()).unwrap();
        let home = root.join("home");
        std::fs::create_dir(&home).unwrap();
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).unwrap();
        // The public CLT Python and its standard library are read-only. -I -S -B
        // excludes user packages, Python environment overrides and bytecode writes.
        let framework = PathBuf::from("/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework");
        let python = std::fs::canonicalize(framework.join("Versions/3.9/Resources/Python.app/Contents/MacOS/Python")).expect("public CLT Python3 runtime");
        let profile = format!(r#"(version 1)
(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (subpath {framework}) (subpath {root})
  (literal "/dev/null") (literal "/dev/random") (literal "/dev/urandom"))
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") (subpath {framework}) (subpath {root}))
(allow file-write* (subpath {root}) (literal "/dev/null"))
(allow process-exec (literal {python}))
(allow process-fork)
(allow sysctl-read)
"#, framework=quoted(&framework), root=quoted(&root), python=quoted(&python));
        let launcher = root.join("confined-python");
        let source = format!("#!/bin/sh\nexec /usr/bin/sandbox-exec -p {} {} -I -S -B \"$@\"\n", shell_quoted(&profile), shell_quoted(python.to_str().unwrap()));
        std::fs::write(&launcher, source).unwrap();
        std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o700)).unwrap();
        let peer = root.join("peer.py");
        std::fs::write(&peer, include_str!("peer.py")).unwrap();
        Self { _directory:directory, root, home, launcher, peer, python, profile }
    }
    pub fn environment(&self) -> std::collections::BTreeMap<String, String> {
        [("HOME", self.home.clone()), ("XDG_CONFIG_HOME", self.home.join(".config")),
         ("XDG_DATA_HOME", self.home.join(".local/share")), ("XDG_STATE_HOME", self.home.join(".local/state"))]
            .into_iter().map(|(name, value)| (name.into(), value.to_string_lossy().into_owned())).collect()
    }
    pub fn account(&self, account: &str) {
        std::fs::write(self.home.join("account.json"), serde_json::json!({"account":account}).to_string()).unwrap();
    }
    pub async fn assert_confinement(&self, sibling: &Path) {
        let outside = tempfile::Builder::new().prefix("fetch-native-outside-").tempdir_in("/private/tmp").unwrap();
        let read = outside.path().join("read-canary");
        let write = outside.path().join("write-canary");
        std::fs::write(&read, "disposable outside marker").unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let script = r#"import errno,json,os,socket
from pathlib import Path
result={}
for name,operation in [('outside_read',lambda:Path(os.environ['OUTSIDE_READ']).read_bytes()),('outside_write',lambda:Path(os.environ['OUTSIDE_WRITE']).write_bytes(b'forbidden')),('sibling_read',lambda:Path(os.environ['SIBLING']).read_bytes())]:
 try: operation(); result[name]='unexpected-success'
 except OSError as error: result[name]=error.errno
own=Path(os.environ['HOME'])/'owned-canary'
own.write_text('owned write works')
result['owned']=own.read_text()
try:
 with socket.socket() as connection: connection.settimeout(1); connection.connect(('127.0.0.1',int(os.environ['OUTSIDE_PORT'])))
 result['network']='unexpected-success'
except OSError as error: result['network']=error.errno
print(json.dumps(result))
"#;
        let mut command = Command::new("/usr/bin/sandbox-exec");
        command.args(["-p", &self.profile]).arg(&self.python).args(["-I", "-S", "-B", "-c", script])
            .env_clear().envs(self.environment()).env("OUTSIDE_READ", &read).env("OUTSIDE_WRITE", &write)
            .env("SIBLING", sibling).env("OUTSIDE_PORT", listener.local_addr().unwrap().port().to_string())
            .current_dir(&self.home).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).kill_on_drop(true);
        let output = tokio::time::timeout(std::time::Duration::from_secs(8), command.output()).await.unwrap().unwrap();
        assert!(output.status.success(), "confined Python failed: {:?} {}", output.status, String::from_utf8_lossy(&output.stderr));
        let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        for name in ["outside_read", "outside_write", "sibling_read", "network"] { assert_eq!(result[name], libc::EPERM, "{name}: {result}"); }
        assert_eq!(result["owned"], "owned write works");
        assert!(!write.exists());
        assert_eq!(std::fs::read_to_string(read).unwrap(), "disposable outside marker");
        assert!(tokio::time::timeout(std::time::Duration::from_millis(20), listener.accept()).await.is_err());
    }
}
fn quoted(path: &Path) -> String { serde_json::to_string(path.to_str().unwrap()).unwrap() }
fn shell_quoted(value: &str) -> String { format!("'{}'", value.replace('\'', "'\\''")) }
