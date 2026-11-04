//! Startup service output uses disposable peers in verified OS confinement.
#![cfg(target_os = "macos")]
#[path = "native_state/confinement.rs"]
mod confinement;

use confinement::Sandbox;
use fetch_local::{authority::Caller, discovery::DiscoveryService, installation::Installer,
    manager::Manager, native_auth::NativeAuthority, remote_oauth::GrantStore,
    runner::{raw, ExecutionPolicy, InvocationContext, Runner, ServerRoute, TransportConfig}, storage::ServerRepository};
use futures_util::FutureExt;
use serde_json::{json, Value};
use std::{collections::BTreeMap, sync::Arc, time::Duration};

fn native_caller() -> Caller {
    let native=NativeAuthority::new();let session=native.open_settings();
    let mut headers=axum::http::HeaderMap::new();
    headers.insert("x-fetch-settings-capability",session.header_value().parse().unwrap());
    native.authorize_settings_headers(&headers).unwrap()
}
fn record(sandbox:&Sandbox,mode:&str)->Value {
    json!({"server_id":"logs","name":"logs","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"kind":"local_script","package":null,"executable":sandbox.launcher,
            "args":[sandbox.peer,sandbox.home,mode],"working_directory":sandbox.home,
            "environment":[{"name":"FIXTURE_LOG_SECRET","credential_ref":"fixture-key"}]}})
}
async fn manager(sandbox:&Sandbox,mode:&str)->(Arc<Manager>,ServerRepository) {
    let root=sandbox.root.join("fetch-state");let repository=ServerRepository::new(root.clone());
    repository.save("logs",&record(sandbox,mode)).unwrap();
    let grants=Arc::new(GrantStore::memory(root.clone()));
    grants.save_section("logs","manual",json!({"fixture-key":"assigned-private-log-value"})).await.unwrap();
    let discovery=DiscoveryService::new(root.join("unused-home"),None);
    (Manager::with_grant_store(repository.clone(),grants,Runner::new(),Installer::new(root,discovery)),repository)
}
async fn failed(manager:&Manager)->Value {
    tokio::time::timeout(Duration::from_secs(8),async {
        loop {
            let value=manager.servers(vec!["logs".into()],None).await.unwrap();
            if value["servers"][0]["authentication"]["status"]=="failed" {break value;}
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.expect("fixture must fail promptly")
}
async fn native_log(manager:&Manager)->Value {
    manager.servers_for(&native_caller(),vec!["logs".into()],None).await.unwrap()["servers"][0]["authentication"]["startup"].clone()
}
fn text(value:&Value)->String {value["lines"].as_array().unwrap().iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n")}
async fn sandbox()->(Sandbox,Sandbox) {
    let sandbox=Sandbox::new();let sibling=Sandbox::new();sibling.account("separate");
    sandbox.assert_confinement(&sibling.home.join("account.json")).await;
    std::fs::write(&sandbox.peer,include_str!("native_state/service_log_peer.py")).unwrap();
    (sandbox,sibling)
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn railway_shaped_startup_retains_history_instructions_and_original_rejection_privately() {
    let (sandbox,_sibling)=sandbox().await;let (manager,repository)=manager(&sandbox,"fixture-tools-reject-mode").await;
    let result=std::panic::AssertUnwindSafe(async {
        manager.restart("logs",json!({"interactive":true})).await.unwrap();
        let ordinary=failed(&manager).await;
        assert!(ordinary["servers"][0]["authentication"].get("startup").is_none());
        let log=native_log(&manager).await;let output=text(&log);
        for useful in ["startup history line 000","startup history line 079","server instructions",
            "Not logged in; run railway login","-32001","original service rejection","service log before ready"] {
            assert!(output.contains(useful),"missing {useful}: {output}");
        }
        for secret in ["assigned-private-log-value","nested-private-token","private-cookie-value","successful-frame-must-stay-private"] {
            assert!(!log.to_string().contains(secret),"leaked {secret}");
        }
        assert!(!repository.get_saved("logs").unwrap().unwrap().to_string().contains("original service rejection"));
        let agent=Caller::from_verified_scopes(["fetch:manage".into()]);
        assert!(manager.servers_for(&agent,vec!["logs".into()],None).await.unwrap()["servers"][0]["authentication"].get("startup").is_none());
    }).catch_unwind().await;
    manager.shutdown().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn recorded_setup_keeps_both_output_streams_and_natural_exit() {
    let (sandbox,_sibling)=sandbox().await;let (manager,repository)=manager(&sandbox,"helper").await;
    let mut saved=repository.get_saved("logs").unwrap().unwrap();
    saved["_installation"]=json!({"source_path":"/fixture/source.json","source_name":"logs",
        "installation_id":format!("package-{}","a".repeat(32)),"protected_args":0,
        "native_setup":{"executable":sandbox.launcher,"args":[sandbox.peer,sandbox.home,"helper"],
            "working_directory":sandbox.home,"state_directory":sandbox.home}});
    repository.save("logs",&saved).unwrap();
    let result=std::panic::AssertUnwindSafe(async {
        manager.restart("logs",json!({"interactive":true})).await.unwrap();failed(&manager).await;
        let log=native_log(&manager).await;let output=text(&log);
        assert!(output.contains("setup stdout final useful reason"));assert!(output.contains("setup stderr final useful reason"));
        assert!(output.contains("startup history line 000"));assert_eq!(log["exit_code"],9);
        assert!(!log.to_string().contains("assigned-private-log-value"));
    }).catch_unwind().await;
    manager.shutdown().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn nonprotocol_stdout_is_explained_without_accepting_it_as_mcp() {
    let (sandbox,_sibling)=sandbox().await;let (manager,_repository)=manager(&sandbox,"fixture-invalid-stdout-mode").await;
    let result=std::panic::AssertUnwindSafe(async {
        manager.restart("logs",json!({"interactive":true})).await.unwrap();failed(&manager).await;
        let output=text(&native_log(&manager).await);
        assert!(output.contains("invalid startup stdout diagnostic"));
        assert_ne!(manager.runner.snapshot("logs").await["runtime"]["status"],"ready");
    }).catch_unwind().await;
    manager.shutdown().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn successful_protocol_and_tool_payloads_do_not_enter_startup_log() {
    let (sandbox,_sibling)=sandbox().await;let runner=Runner::new();
    let mut environment=sandbox.environment();environment.insert("FIXTURE_LOG_SECRET".into(),"assigned-private-log-value".into());
    runner.configure(ServerRoute{server_id:"success".into(),transport:TransportConfig::Stdio{
        executable:sandbox.launcher.clone(),args:vec![sandbox.peer.to_string_lossy().into_owned(),sandbox.home.to_string_lossy().into_owned(),"success".into()],
        working_directory:Some(sandbox.home.clone()),environment},execution:ExecutionPolicy::default()}).await.unwrap();
    let result=std::panic::AssertUnwindSafe(async {
        runner.connect("success").await.unwrap();let before=runner.startup_diagnostics("success").await.unwrap();
        let output=runner.call("success","echo",raw::raw(&json!({})),InvocationContext::terminal("fixture".into())).await.unwrap();
        assert!(output.get().contains("successful-tool-result-must-stay-private"));
        tokio::time::sleep(Duration::from_millis(30)).await;
        let after=runner.startup_diagnostics("success").await.unwrap();assert_eq!(before,after);
        for private in ["successful-frame-must-stay-private","successful-tool-result-must-stay-private","post-ready-output-must-stay-private"] {assert!(!after.to_string().contains(private));}
    }).catch_unwind().await;
    runner.shutdown().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test]
async fn remote_http_rejection_keeps_status_and_original_error_without_credentials() {
    use axum::response::IntoResponse;
    let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint=format!("http://{}/mcp",listener.local_addr().unwrap());
    let upstream=tokio::spawn(async move {axum::serve(listener,axum::Router::new().route("/mcp",axum::routing::post(||async {
        (axum::http::StatusCode::FORBIDDEN,axum::Json(json!({"error":"access_denied","reason":"original HTTP denial","access_token":"returned-private-token"}))).into_response()
    }))).await.unwrap()});
    let runner=Runner::new();runner.configure(ServerRoute{server_id:"http".into(),transport:TransportConfig::Http{
        endpoint,headers:BTreeMap::from([("Authorization".into(),"Bearer assigned-private-token".into())])},execution:ExecutionPolicy::default()}).await.unwrap();
    let result=std::panic::AssertUnwindSafe(async {
        assert!(runner.connect("http").await.is_err());let log=runner.startup_diagnostics("http").await.unwrap();let output=text(&log);
        assert!(output.contains("403"));assert!(output.contains("original HTTP denial"));
        for secret in ["assigned-private-token","returned-private-token"]{assert!(!log.to_string().contains(secret));}
    }).catch_unwind().await;
    runner.shutdown().await;upstream.abort();let _=upstream.await;if let Err(error)=result{std::panic::resume_unwind(error);}
}

#[tokio::test(flavor="multi_thread",worker_threads=4)]
async fn initial_unpolled_startup_survives_the_later_authentication_process() {
    let (sandbox,_sibling)=sandbox().await;let (manager,repository)=manager(&sandbox,"fixture-tools-reject-mode").await;
    let result=std::panic::AssertUnwindSafe(async {
        tokio::time::timeout(Duration::from_secs(8),manager.startup()).await
            .expect("initial app startup should reach its actual service rejection");
        let status=manager.servers(vec!["logs".into()],None).await.unwrap();
        assert_eq!(status["servers"][0]["runtime"]["status"],"failed");
        assert_eq!(status["servers"][0]["runtime"]["error"]["code"],"TOOL_RPC_ERROR");
        // No native startup log read occurs before the next operation.
        manager.restart("logs",json!({"interactive":true})).await.unwrap();failed(&manager).await;
        let log=native_log(&manager).await;let output=text(&log);
        assert_eq!(output.matches("startup history line 000").count(),2,"both real processes must remain in the cumulative console");
        assert_eq!(output.matches("startup history line 079").count(),2);
        assert_eq!(output.matches("original service rejection").count(),2);
        assert!(!output.contains("assigned-private-log-value"));assert!(!output.contains("nested-private-token"));
        assert!(!repository.get_saved("logs").unwrap().unwrap().to_string().contains("startup history line"));
        let ordinary=manager.servers(vec!["logs".into()],None).await.unwrap();
        assert!(ordinary["servers"][0]["authentication"].get("startup").is_none());
    }).catch_unwind().await;
    manager.shutdown().await;if let Err(error)=result{std::panic::resume_unwind(error);}
}
