//! Bare helper lookup through the real manager, inside verified OS confinement.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;

use confinement::Sandbox;
use fetch_local::{discovery::DiscoveryService, installation::Installer, manager::Manager,
    remote_oauth::GrantStore, runner::{raw, InvocationContext, Runner}, storage::ServerRepository};
use futures_util::FutureExt;
use serde_json::{Value, json};
use std::{path::PathBuf, sync::Arc, time::Duration};

const PEER: &str = r#"import json,os,subprocess,sys
from pathlib import Path
home=Path(sys.argv[1]);home.mkdir(parents=True,exist_ok=True)
(home/'peer.pid').write_text(str(os.getpid()))
(home/'received-startup.json').write_text(json.dumps({'args':sys.argv[1:],'cwd':os.getcwd(),'environment':{name:os.environ.get(name) for name in ['HOME','XDG_CONFIG_HOME','XDG_STATE_HOME','PATH','LANG']}}))
try:
 result=subprocess.run(['fetch-private-helper','-I','-S','-B','-c','print("private helper found")'],check=True,capture_output=True,text=True)
except FileNotFoundError:
 (home/'helper-missing').write_text('helper was not found by name')
 sys.exit(7)
(home/'helper-result').write_text(result.stdout.strip())
if '--helper' in sys.argv:sys.exit(0)
for line in sys.stdin:
 request=json.loads(line)
 if 'id' not in request:continue
 method=request.get('method')
 if method=='initialize':result={'protocolVersion':'2025-11-25','capabilities':{'tools':{}},'serverInfo':{'name':'private-helper-peer','version':'1'}}
 elif method=='tools/list':result={'tools':[{'name':'echo','inputSchema':{'type':'object'}}]}
 elif method=='tools/call':result={'content':[{'type':'text','text':'private helper found'}]}
 else:result={}
 print(json.dumps({'jsonrpc':'2.0','id':request['id'],'result':result}),flush=True)
"#;

fn installation(letter: char) -> String { format!("package-{}", letter.to_string().repeat(32)) }

fn record(sandbox: &Sandbox, id: &str, letter: char) -> Value {
    let home=sandbox.home.join(id);
    std::fs::create_dir_all(&home).unwrap();
    json!({"server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"npm","package":{"name":"@railway/mcp-server","requested_version":"fixture","installed_version":"fixture"},
            "executable":sandbox.launcher,"args":[sandbox.peer,home],"working_directory":home,"environment":[]},
        "_installation":{"source_path":"/disposable/source.json","source_name":id,
            "installation_id":installation(letter),"protected_args":1}})
}

fn manager(sandbox: &Sandbox) -> (Arc<Manager>, ServerRepository, PathBuf) {
    let root=sandbox.root.join("fetch state with spaces");
    let repository=ServerRepository::new(root.clone());
    let discovery=DiscoveryService::new(sandbox.root.join("unused-source-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),Arc::new(GrantStore::memory(root.clone())),
        Runner::new(),Installer::new(root.clone(),discovery));
    (manager,repository,root)
}

fn install_helper(root: &std::path::Path, letter: char, relative: &str) {
    let bin=root.join("packages/servers").join(installation(letter)).join(relative);
    std::fs::create_dir_all(&bin).unwrap();
    let python=std::fs::canonicalize("/Library/Developer/CommandLineTools/Library/Frameworks/Python3.framework/Versions/3.9/Resources/Python.app/Contents/MacOS/Python").unwrap();
    std::os::unix::fs::symlink(python,bin.join("fetch-private-helper")).unwrap();
}

fn assert_received_startup(sandbox: &Sandbox, root: &std::path::Path, id: &str, letter: char, relative: &str, setup: bool) {
    let output=sandbox.home.join(id);
    let actual:Value=serde_json::from_slice(&std::fs::read(output.join("received-startup.json")).unwrap()).unwrap();
    let mut expected_args=vec![json!(output)];
    if setup {expected_args.push(json!("--helper"));}
    assert_eq!(actual["args"],json!(expected_args));
    assert_eq!(actual["cwd"],json!(output));
    let environment=&actual["environment"];
    let private_home=PathBuf::from(environment["HOME"].as_str().unwrap());
    assert!(private_home.starts_with(root.join("servers")));
    assert_eq!(environment["XDG_CONFIG_HOME"],json!(private_home.join(".config")));
    assert_eq!(environment["XDG_STATE_HOME"],json!(private_home.join(".local/state")));
    let paths:Vec<_>=std::env::split_paths(environment["PATH"].as_str().unwrap()).collect();
    let mut expected_paths=vec![std::fs::canonicalize(root.join("packages/servers").join(installation(letter)).join(relative)).unwrap(),sandbox.launcher.parent().unwrap().to_owned()];
    expected_paths.extend(std::env::var_os("PATH").map(|path|std::env::split_paths(&path).collect::<Vec<_>>()).unwrap_or_else(||vec![PathBuf::from("/usr/bin"),PathBuf::from("/bin")]));
    assert_eq!(paths,expected_paths);
    assert_eq!(environment["LANG"],json!(std::env::var("LANG").unwrap_or_else(|_|"en_US.UTF-8".into())));
}

async fn until(manager: &Manager, id: &str, predicate: impl Fn(&Value)->bool) -> Value {
    tokio::time::timeout(Duration::from_secs(8),async {
        loop {
            let state=manager.servers(vec![id.into()],None).await.unwrap()["servers"][0].clone();
            if predicate(&state) {break state;}
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("expected terminal fixture state")
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn legacy_startup_finds_private_helper_and_another_installation_cannot_find_it() {
    let sandbox=Sandbox::new();let outside=Sandbox::new();outside.account("separate");
    sandbox.assert_confinement(&outside.home.join("account.json")).await;
    std::fs::write(&sandbox.peer,PEER).unwrap();
    let (manager,repository,root)=manager(&sandbox);
    let first=record(&sandbox,"first",'a');let second=record(&sandbox,"second",'b');
    repository.save("first",&first).unwrap();repository.save("second",&second).unwrap();
    install_helper(&root,'a',"railway-cli/node_modules/.bin");
    std::fs::create_dir_all(root.join("packages/servers").join(installation('b'))).unwrap();
    let result=std::panic::AssertUnwindSafe(async {
        let restart=manager.restart("first",json!({})).await.unwrap();
        let restart_id=restart["restart_id"].as_str().unwrap();
        tokio::time::timeout(Duration::from_secs(8),async {
            loop {
                let state=manager.servers(vec!["first".into()],Some(restart_id.into())).await.unwrap();
                if state["servers"][0]["restart"]["status"]=="succeeded" {break;}
                assert_ne!(state["servers"][0]["restart"]["status"],"failed", "{state}");
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("ordinary startup must finish before tool use");
        assert_eq!(std::fs::read_to_string(sandbox.home.join("first/helper-result")).unwrap(),"private helper found");
        assert_received_startup(&sandbox,&root,"first",'a',"railway-cli/node_modules/.bin",false);
        let value=manager.runner.call("first","echo",raw::raw(&json!({})),InvocationContext::terminal("fixture-owner".into())).await.unwrap();
        assert_eq!(serde_json::from_str::<Value>(value.get()).unwrap()["content"][0]["text"],"private helper found");
        assert!(repository.get_saved("first").unwrap().unwrap()["_installation"].get("program_directories").is_none(),"legacy lookup must not rewrite the record");
        manager.restart("second",json!({})).await.unwrap();
        until(&manager,"second",|value|value["runtime"]["status"]=="failed").await;
        assert!(sandbox.home.join("second/helper-missing").is_file());
        assert!(!sandbox.home.join("second/helper-result").exists());
        assert_eq!(manager.servers(vec!["first".into()],None).await.unwrap()["servers"][0]["runtime"]["status"],"ready");
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let task_root=sandbox.root.clone();drop(manager);drop(repository);drop(sandbox);drop(outside);
    assert!(!task_root.exists());
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn recorded_setup_uses_recorded_program_folders_and_metadata_stays_private() {
    let sandbox=Sandbox::new();let outside=Sandbox::new();outside.account("separate");
    sandbox.assert_confinement(&outside.home.join("account.json")).await;
    std::fs::write(&sandbox.peer,PEER).unwrap();
    let (manager,repository,root)=manager(&sandbox);
    let mut saved=record(&sandbox,"setup",'c');
    let home=sandbox.home.join("setup");
    saved["_installation"]["program_directories"]=json!(["private tools/bin"]);
    saved["_installation"]["native_setup"]=json!({"executable":sandbox.launcher,
        "args":[sandbox.peer,home,"--helper"],"working_directory":home,"state_directory":home});
    repository.save("setup",&saved).unwrap();
    install_helper(&root,'c',"private tools/bin");
    let result=std::panic::AssertUnwindSafe(async {
        assert!(repository.get("setup").unwrap().unwrap().get("_installation").is_none());
        assert_eq!(manager.restart("setup",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
        let state=until(&manager,"setup",|value|value["authentication"]["status"]!="authenticating").await;
        assert_eq!(state["authentication"]["status"],"unknown");
        assert!(state.get("_installation").is_none());
        assert_eq!(std::fs::read_to_string(home.join("helper-result")).unwrap(),"private helper found");
        assert_received_startup(&sandbox,&root,"setup",'c',"private tools/bin",true);
        assert_eq!(repository.get_saved("setup").unwrap().unwrap()["_installation"]["program_directories"],json!(["private tools/bin"]));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let task_root=sandbox.root.clone();drop(manager);drop(repository);drop(sandbox);drop(outside);
    assert!(!task_root.exists());
}
