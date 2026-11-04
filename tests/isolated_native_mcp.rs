//! Actual synthetic child MCP checks under a verified resource sandbox.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;
use confinement::Sandbox;
use fetch_local::{
    authority::Caller, discovery::DiscoveryService, installation::Installer, manager::Manager,
    remote_oauth::GrantStore,
    runner::{ExecutionPolicy, InvocationContext, Runner, ServerRoute, TransportConfig, raw},
    storage::ServerRepository,
};
use futures_util::FutureExt;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{path::Path, sync::Arc, time::Duration};

fn route(id: &str, sandbox: &Sandbox) -> ServerRoute {
    ServerRoute { server_id:id.into(), transport:TransportConfig::Stdio {
        executable:sandbox.launcher.clone(), args:vec![sandbox.peer.to_string_lossy().into_owned()],
        working_directory:Some(sandbox.home.clone()), environment:sandbox.environment(),
    }, execution:ExecutionPolicy::default() }
}
async fn call(runner: &Runner, id: &str, wait: bool) -> Box<serde_json::value::RawValue> {
    runner.call(id, "echo", raw::raw(&json!({"wait":wait,"future":{"keep":true}})),
        InvocationContext::terminal("disposable-owner".into())).await.unwrap()
}
fn alive(pid_file: &Path) -> bool {
    let pid:i32 = std::fs::read_to_string(pid_file).unwrap().parse().unwrap();
    // Signal 0 checks existence only; it never sends a signal to another process.
    unsafe { libc::kill(pid, 0) == 0 }
}

#[tokio::test(flavor="multi_thread", worker_threads=4)]
async fn native_files_survive_explicit_restart_instances_stay_separate_and_other_calls_respond() {
    let personal = Sandbox::new(); let work = Sandbox::new();
    personal.account("personal-account"); work.account("work-account");
    personal.assert_confinement(&work.home.join("account.json")).await;
    work.assert_confinement(&personal.home.join("account.json")).await;
    let runner = Runner::new();
    let result = std::panic::AssertUnwindSafe(async {
        runner.configure(route("personal", &personal)).await.unwrap();
        runner.configure(route("work", &work)).await.unwrap();
        let (first, second) = tokio::join!(runner.connect("personal"), runner.connect("work"));
        first.unwrap(); second.unwrap();
        let raw = call(&runner, "personal", false).await;
        assert_eq!(raw.get(), "{ \"account\":\"personal-account\", \"number\":184467440737095516160000001, \"content\":[{\"type\":\"image\",\"mimeType\":\"image/png\",\"data\":\"fixture-base64\"}], \"future\":{\"keep\":true} }");
        assert_eq!(serde_json::from_str::<Value>(call(&runner, "work", false).await.get()).unwrap()["account"], "work-account");
        for sandbox in [&personal, &work] {
            for folder in [".config", ".local/share", ".local/state"] {
                assert!(sandbox.home.join(folder).join("native-state.json").is_file());
            }
        }
        personal.account("new-personal-account");
        // Editing ordinary native files does not silently restart a live server.
        assert_eq!(serde_json::from_str::<Value>(call(&runner, "personal", false).await.get()).unwrap()["account"], "personal-account");
        assert_eq!(std::fs::read_to_string(personal.home.join("starts.log")).unwrap().lines().count(), 1);
        runner.disconnect("personal", false).await.unwrap();
        assert!(!alive(&personal.home.join("peer.pid")));
        runner.connect("personal").await.unwrap();
        assert_eq!(serde_json::from_str::<Value>(call(&runner, "personal", false).await.get()).unwrap()["account"], "new-personal-account");
        assert_eq!(std::fs::read_to_string(personal.home.join("starts.log")).unwrap().lines().count(), 2);
        assert_eq!(std::fs::read_to_string(work.home.join("starts.log")).unwrap().lines().count(), 1);
        let slow = { let runner=runner.clone(); tokio::spawn(async move {call(&runner,"personal",true).await}) };
        tokio::time::timeout(Duration::from_secs(2), async {
            while runner.snapshot("personal").await["tool_execution"]["active_calls"] != 1 { tokio::task::yield_now().await; }
        }).await.unwrap();
        let quick = tokio::time::timeout(Duration::from_millis(400), call(&runner,"work",false)).await.expect("a separate peer stays responsive while the first is busy");
        assert_eq!(serde_json::from_str::<Value>(quick.get()).unwrap()["account"],"work-account");
        slow.await.unwrap();
        let argument = serde_json::value::RawValue::from_string("{ \"number\":184467440737095516160000009, \"object\":{\"unchanged\":true} }".into()).unwrap();
        runner.call("work","echo",argument,InvocationContext::terminal("disposable-owner".into())).await.unwrap();
        let request = std::fs::read_to_string(work.home.join("last-request.json")).unwrap();
        assert!(request.contains("{ \"number\":184467440737095516160000009, \"object\":{\"unchanged\":true} }"));
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8), runner.shutdown()).await.expect("owned peer shutdown");
    if let Err(error) = result { std::panic::resume_unwind(error); }
    assert!(!alive(&personal.home.join("peer.pid"))); assert!(!alive(&work.home.join("peer.pid")));
    let personal_root=personal.root.clone(); let work_root=work.root.clone();
    drop(personal); drop(work);
    assert!(!personal_root.exists()); assert!(!work_root.exists());
}

#[tokio::test(flavor="multi_thread", worker_threads=4)]
async fn removing_registration_stops_only_its_peer_and_preserves_native_legacy_and_package_files() {
    let first = Sandbox::new(); let second = Sandbox::new();
    first.account("first-account"); second.account("second-account");
    first.assert_confinement(&second.home.join("account.json")).await;
    let state=tempfile::tempdir().unwrap(); let root=state.path().join("state");
    let repository=ServerRepository::new(root.clone()); let runner=Runner::new();
    let discovery=DiscoveryService::new(state.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository.clone(),Arc::new(GrantStore::memory(root.clone())),runner.clone(),Installer::new(root.clone(),discovery));
    let result=std::panic::AssertUnwindSafe(async {
        for (id,sandbox) in [("first",&first),("second",&second)] {
            let record=json!({"server_id":id,"name":id,"runtime":{"status":"stopped","error":null},
                "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
                "config":{"kind":"npm","package":{"name":"disposable-native-peer","requested_version":"1","installed_version":"1"},
                    "executable":sandbox.launcher,"args":[sandbox.peer],"working_directory":sandbox.home,"environment":[]},"credentials":[],
                "_installation":{"source_path":"/disposable/source.json","source_name":id,"installation_id":format!("package-{}",if id=="first"{"a".repeat(32)}else{"b".repeat(32)}),"protected_args":0}});
            repository.save(id,&record).unwrap(); runner.configure(route(id,sandbox)).await.unwrap(); runner.connect(id).await.unwrap();
        }
        let legacy=root.join("servers").join(format!("{:x}",Sha256::digest(b"first"))).join("credentials.enc");
        std::fs::write(&legacy,b"opaque legacy bytes\0\xff").unwrap();
        let package=root.join("packages/servers").join(format!("package-{}","a".repeat(32)));
        std::fs::create_dir_all(&package).unwrap(); std::fs::write(package.join("native-token.json"),b"package-native-state").unwrap();
        let first_before=std::fs::read(first.home.join("account.json")).unwrap();
        let response=manager.delete_server("first",&Caller::from_verified_scopes(["fetch:manage".into()])).await.unwrap();
        assert_eq!(response["status"],"deleted"); assert_eq!(response["next_steps"],json!([]));
        assert!(repository.get_saved("first").unwrap().is_none()); assert!(repository.get_saved("second").unwrap().is_some());
        assert_eq!(std::fs::read(&legacy).unwrap(),b"opaque legacy bytes\0\xff");
        assert_eq!(std::fs::read(package.join("native-token.json")).unwrap(),b"package-native-state");
        assert_eq!(std::fs::read(first.home.join("account.json")).unwrap(),first_before);
        assert!(!alive(&first.home.join("peer.pid"))); assert!(alive(&second.home.join("peer.pid")));
        assert_eq!(serde_json::from_str::<Value>(call(&runner,"second",false).await.get()).unwrap()["account"],"second-account");
        assert_eq!(manager.delete_server("first",&Caller::from_verified_scopes(["fetch:manage".into()])).await.unwrap_err().code,"SERVER_NOT_FOUND");
    }).catch_unwind().await;
    tokio::time::timeout(Duration::from_secs(8),manager.shutdown()).await.expect("owned peer shutdown");
    if let Err(error)=result {std::panic::resume_unwind(error);}
    assert!(!alive(&second.home.join("peer.pid")));
    let first_root=first.root.clone();let second_root=second.root.clone();drop(first);drop(second);drop(state);
    assert!(!first_root.exists());assert!(!second_root.exists());
}
