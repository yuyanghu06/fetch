//! Opt-in check of an unmodified no-account package staged in a disposable root.
#![cfg(target_os="macos")]
use fetch_local::{authority::Caller,discovery::DiscoveryService,installation::Installer,manager::Manager,
    remote_oauth::GrantStore,runner::{InvocationContext,Runner,raw},storage::ServerRepository};
use futures_util::FutureExt;
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{os::unix::fs::PermissionsExt,path::{Path,PathBuf},sync::Arc,time::Duration};

async fn restart(manager:&Arc<Manager>,id:&str) {
    let accepted=manager.restart(id,json!({})).await.unwrap();let attempt=accepted["restart_id"].as_str().unwrap();
    tokio::time::timeout(Duration::from_secs(15),async{loop {
        let state=manager.servers(vec![id.into()],Some(attempt.into())).await.unwrap();let record=&state["servers"][0];
        match record["restart"]["status"].as_str(){Some("succeeded")=>break,Some("failed")=>panic!("{record}"),_=>tokio::time::sleep(Duration::from_millis(20)).await}
    }}).await.expect("owned real server restart must finish");
}
fn shell(value:&str)->String{format!("'{}'",value.replace('\'',"'\\''"))}
fn alive(path:&Path)->bool{let pid:i32=std::fs::read_to_string(path).unwrap().parse().unwrap();unsafe{libc::kill(pid,0)==0}}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
#[ignore="Requires separately staged public package under FETCH_NATIVE_REAL_ROOT; never installs or uses personal state"]
async fn real_everything_start_echo_restart_and_registration_removal_preserve_state() {
    let supplied=std::env::var_os("FETCH_NATIVE_REAL_ROOT").expect("explicit task-owned root");
    let root=std::fs::canonicalize(PathBuf::from(supplied)).unwrap();
    assert_eq!(root.parent(),Some(Path::new("/private/tmp")));
    assert!(root.file_name().unwrap().to_str().unwrap().starts_with("fetch-native-real-"));
    assert_eq!(std::fs::read_to_string(root.join(".fetch-native-real")).unwrap(),"Fetch explicit isolated real MCP fixture\n");
    let node=root.join("node-v24.22.1-darwin-arm64/bin/node");
    let package=root.join("package/node_modules/@modelcontextprotocol/server-everything");
    let manifest:Value=serde_json::from_slice(&std::fs::read(package.join("package.json")).unwrap()).unwrap();
    assert_eq!(manifest["version"],"2026.8.31");
    let profile=format!(r#"(version 1)
(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (subpath {root}) (literal "/dev/null") (literal "/dev/random") (literal "/dev/urandom"))
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") (subpath {root}))
(allow file-write* (subpath {root}) (literal "/dev/null"))
(allow process-exec (literal {node}))
(allow process-fork)
(allow sysctl-read)
"#,root=json!(root),node=json!(node));
    let outside=tempfile::Builder::new().prefix("fetch-native-real-outside-").tempdir_in("/private/tmp").unwrap();
    let read=outside.path().join("read");let write=outside.path().join("write");std::fs::write(&read,"outside-canary").unwrap();
    let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let probe=r#"const fs=require('node:fs'),net=require('node:net');const result={};for(const[name,op]of[['read',()=>fs.readFileSync(process.env.OUTSIDE_READ)],['write',()=>fs.writeFileSync(process.env.OUTSIDE_WRITE,'must-not-write')]]){try{op();result[name]='unexpected-success'}catch(e){result[name]=e.code}}fs.writeFileSync(process.env.HOME+'/owned-canary','owned');result.own=fs.readFileSync(process.env.HOME+'/owned-canary','utf8');const socket=net.createConnection({host:'127.0.0.1',port:Number(process.env.OUTSIDE_PORT)});socket.on('connect',()=>{result.network='unexpected-success';socket.destroy();console.log(JSON.stringify(result))});socket.on('error',e=>{result.network=e.code;console.log(JSON.stringify(result))});"#;
    let output=tokio::time::timeout(Duration::from_secs(5),tokio::process::Command::new("/usr/bin/sandbox-exec").args(["-p",&profile]).arg(&node).args(["-e",probe])
        .env_clear().env("HOME",root.join("home")).env("OUTSIDE_READ",&read).env("OUTSIDE_WRITE",&write).env("OUTSIDE_PORT",listener.local_addr().unwrap().port().to_string())
        .current_dir(&root).kill_on_drop(true).output()).await.unwrap().unwrap();
    assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let canary:Value=serde_json::from_slice(&output.stdout).unwrap();
    for key in ["read","write","network"]{assert_eq!(canary[key],"EPERM");}assert_eq!(canary["own"],"owned");
    assert!(!write.exists());assert_eq!(std::fs::read_to_string(&read).unwrap(),"outside-canary");
    assert!(tokio::time::timeout(Duration::from_millis(20),listener.accept()).await.is_err());
    let launcher=root.join("confined-node");
    std::fs::write(&launcher,format!("#!/bin/sh\nprintf '%s' \"$$\" > \"$HOME/real-peer.pid\"\nexec /usr/bin/sandbox-exec -p {} {} \"$@\"\n",shell(&profile),shell(node.to_str().unwrap()))).unwrap();
    std::fs::set_permissions(&launcher,std::fs::Permissions::from_mode(0o700)).unwrap();
    let state=root.join("state");let repository=ServerRepository::new(state.clone());let discovery=DiscoveryService::new(root.join("empty-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),Arc::new(GrantStore::memory(state.clone())),Runner::new(),Installer::new(state.clone(),discovery));
    let result=std::panic::AssertUnwindSafe(async {
        repository.save("real",&json!({"server_id":"real","name":"unmodified Everything","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
            "config":{"kind":"npm","package":{"name":"@modelcontextprotocol/server-everything","requested_version":"2026.8.31","installed_version":"2026.8.31"},"executable":launcher,
                "args":[package.join("dist/index.js"),"stdio"],"working_directory":null,"environment":[]},"credentials":[],
            "_installation":{"source_path":"/disposable/source.json","source_name":"real","installation_id":format!("package-{}","c".repeat(32)),"protected_args":0}})).unwrap();
        restart(&manager,"real").await;
        let catalog=manager.runner.catalog("real","raw").await.unwrap();assert!(catalog["tools"].as_array().unwrap().iter().any(|tool|tool["name"]=="echo"));
        let marker="isolated unmodified Everything response";
        let echoed=manager.runner.call("real","echo",raw::raw(&json!({"message":marker})),InvocationContext::terminal("disposable-real-owner".into())).await.unwrap();
        assert!(echoed.get().contains(marker));
        let home=state.join("servers").join(format!("{:x}",Sha256::digest(b"real"))).join("home");
        let native=home.join("native-preservation-canary");std::fs::write(&native,"owned native state").unwrap();
        let legacy=home.parent().unwrap().join("credentials.enc");std::fs::write(&legacy,b"opaque old encrypted bytes\0\xff").unwrap();
        restart(&manager,"real").await;
        assert_eq!(std::fs::read_to_string(&native).unwrap(),"owned native state");
        let echoed=manager.runner.call("real","echo",raw::raw(&json!({"message":marker})),InvocationContext::terminal("disposable-real-owner".into())).await.unwrap();assert!(echoed.get().contains(marker));
        let deleted=manager.delete_server("real",&Caller::from_verified_scopes(["fetch:manage".into()])).await.unwrap();assert_eq!(deleted["status"],"deleted");
        assert!(!alive(&home.join("real-peer.pid")));assert!(repository.get_saved("real").unwrap().is_none());assert!(package.join("dist/index.js").is_file());
        assert_eq!(std::fs::read_to_string(native).unwrap(),"owned native state");assert_eq!(std::fs::read(legacy).unwrap(),b"opaque old encrypted bytes\0\xff");
        println!("REAL_NATIVE_MCP {}",json!({"package":"@modelcontextprotocol/server-everything","version":"2026.8.31","node":"24.22.1","tools":catalog["tools"].as_array().unwrap().len(),"echo_before_after_restart":true,"canaries_denied":true,"legacy_native_package_preserved":true,"owned_peer_stopped":true}));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.unwrap();
    if let Err(error)=result{std::panic::resume_unwind(error);}
}
