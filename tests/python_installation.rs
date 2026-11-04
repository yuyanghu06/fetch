//! Python preparation through the actual installer, with uv replaced by an OS-confined fixture.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;

use confinement::Sandbox;
use fetch_local::{discovery::DiscoveryService, installation::{Installer, UV_VERSION}, storage::ServerRepository};
use serde_json::{Value, json};
use std::{path::{Path, PathBuf}, sync::Arc, time::Duration};

const UV_PEER: &str = r#"import json,os,sys
from pathlib import Path
args=sys.argv[1:]
root=Path(__file__).parent
with (root/'uv-invocations.jsonl').open('a') as log:
 log.write(json.dumps({'args':args,'cwd':os.getcwd(),'environment':{key:os.environ.get(key) for key in ['HOME','UV_TOOL_DIR','UV_TOOL_BIN_DIR','UV_PYTHON_INSTALL_DIR','UV_PYTHON_INSTALL_BIN']}})+'\n')
if args[:2]!=['tool','install']:
 sys.exit(20)
spec=args[-1]
name,separator,version=spec.partition('==')
name=name.lower().replace('_','-')
version=version if separator else '2.0.0'
tools=Path(os.environ['UV_TOOL_DIR']); exported=Path(os.environ['UV_TOOL_BIN_DIR'])
environment=tools/name
bin=environment/'bin';bin.mkdir(parents=True,exist_ok=True)
metadata=environment/'lib/python3.12/site-packages'/f'{name.replace("-","_")}-{version}.dist-info'
metadata.mkdir(parents=True,exist_ok=True)
(metadata/'METADATA').write_text(f'Metadata-Version: 2.4\nName: {name}\nVersion: {version}\nRequires-Python: >=3.10\n')
(environment/'pyvenv.cfg').write_text('version = 3.12.9\n')
script=bin/name;script.write_text('#!/bin/sh\nexit 42\n');script.chmod(0o700)
exported.mkdir(parents=True,exist_ok=True)
(exported/name).symlink_to(script)
(environment/'uv-receipt.toml').write_text('[tool]\nrequirements = [{name = "'+name+'"}]\nentrypoints = [{name = "'+name+'", install-path = "'+str(exported/name)+'"}]\n')
"#;

fn private(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::create_dir_all(path).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

fn shell_quote(value: &str) -> String { format!("'{}'", value.replace('\'', "'\\''")) }

fn fixture_uv(sandbox: &Sandbox, root: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(&sandbox.peer, UV_PEER).unwrap();
    let runtime=root.join("packages/runtimes/uv").join(UV_VERSION);
    private(&runtime);
    let source=format!("#!/bin/sh\nexec {} {} \"$@\"\n",shell_quote(sandbox.launcher.to_str().unwrap()),shell_quote(sandbox.peer.to_str().unwrap()));
    std::fs::write(runtime.join("uv"),source).unwrap();
    std::fs::set_permissions(runtime.join("uv"),std::fs::Permissions::from_mode(0o700)).unwrap();
}

async fn finished(installer: &Arc<Installer>, id: &str) -> Value {
    tokio::time::timeout(Duration::from_secs(8),async {
        loop {
            let progress=installer.progress(id).await.unwrap();
            if progress["status"]!="installing" { break progress; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("confined preparation must finish")
}

#[tokio::test(flavor="multi_thread", worker_threads=4)]
async fn imported_python_servers_use_uv_environments_and_preserve_explicit_python() {
    let sandbox=Sandbox::new();let outside=Sandbox::new();outside.account("unrelated account");
    sandbox.assert_confinement(&outside.home.join("account.json")).await;
    let root=sandbox.root.join("fetch state with spaces");private(&root);fixture_uv(&sandbox,&root);
    let source=sandbox.root.join("mcp.json");
    std::fs::write(&source,json!({"mcpServers":{
        "blender":{"command":"uvx","args":["blender-mcp"]},
        "time":{"command":"uvx","args":["--python","3.12","--from","mcp-server-time==2026.10.19","mcp-server-time","--local-timezone=America/New_York"]}
    }}).to_string()).unwrap();
    let original=std::fs::read(&source).unwrap();
    let installer=Installer::new(root.clone(),DiscoveryService::new(sandbox.root.clone(),None));
    let accepted=installer.submit(json!({"servers":[
        {"config_path":source,"server_name":"blender"},
        {"config_path":source,"server_name":"time"}
    ]})).await.unwrap();
    let result=finished(&installer,accepted["installation_id"].as_str().unwrap()).await;
    installer.shutdown();
    assert_eq!(result["status"],"succeeded","{result}");
    let repository=ServerRepository::new(root.clone());
    let saved:Vec<Value>=result["servers"].as_array().unwrap().iter().map(|item|repository.get_saved(item["server_id"].as_str().unwrap()).unwrap().unwrap()).collect();
    assert_eq!(saved[0]["config"]["package"]["installed_version"],"2.0.0");
    assert_eq!(saved[0]["config"]["args"],json!([]));
    assert_eq!(saved[1]["config"]["package"]["requested_version"],"2026.10.19");
    assert_eq!(saved[1]["config"]["package"]["installed_version"],"2026.10.19");
    assert_eq!(saved[1]["config"]["args"],json!(["--local-timezone=America/New_York"]));
    assert_ne!(saved[0]["_installation"]["installation_id"],saved[1]["_installation"]["installation_id"]);
    let calls:Vec<Value>=std::fs::read_to_string(sandbox.root.join("uv-invocations.jsonl")).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    assert_eq!(calls.len(),2,"only uv tool installation should prepare each server");
    for (index,(call,record)) in calls.iter().zip(&saved).enumerate() {
        let args:Vec<&str>=call["args"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();
        assert_eq!(&args[..2],["tool","install"]);
        assert!(args.contains(&"--managed-python"));
        assert!(!args.contains(&"3.13.18"));
        if index==0 { assert!(!args.iter().any(|arg|arg.starts_with("--python"))); }
        else { assert!(args.windows(2).any(|pair|pair==["--python","3.12"]),"{args:?}"); }
        let package=root.join("packages/servers").join(record["_installation"]["installation_id"].as_str().unwrap());
        for variable in ["HOME","UV_TOOL_DIR","UV_TOOL_BIN_DIR"] {
            assert!(Path::new(call["environment"][variable].as_str().unwrap()).starts_with(&package),"{variable}: {call}");
        }
        assert_eq!(call["environment"]["UV_PYTHON_INSTALL_DIR"],json!(root.join("packages/runtimes/python")));
        assert_eq!(call["environment"]["UV_PYTHON_INSTALL_BIN"],"0");
        let executable=PathBuf::from(record["config"]["executable"].as_str().unwrap());
        assert!(executable.is_file());assert!(executable.starts_with(&package));
        let canonical_parent=std::fs::canonicalize(&executable).unwrap().parent().unwrap().to_owned();
        let folders:Vec<PathBuf>=record["_installation"]["program_directories"].as_array().unwrap().iter().map(|value|std::fs::canonicalize(package.join(value.as_str().unwrap())).unwrap()).collect();
        assert!(folders.contains(&canonical_parent),"server helper folder missing: {folders:?}");
        assert_eq!(record["runtime"]["status"],"stopped","preparation must not start the MCP");
    }
    assert_ne!(calls[0]["environment"]["UV_TOOL_DIR"],calls[1]["environment"]["UV_TOOL_DIR"]);
    assert_ne!(calls[0]["environment"]["HOME"],calls[1]["environment"]["HOME"]);
    assert_eq!(std::fs::read(source).unwrap(),original);
    assert_eq!(std::fs::read_to_string(outside.home.join("account.json")).unwrap(),"{\"account\":\"unrelated account\"}");
}
