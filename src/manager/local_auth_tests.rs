//! Native browser consent tests use a fake presenter and never spawn a server.
use super::*;
use crate::{discovery::DiscoveryService, remote_oauth::GrantStore};
use std::sync::atomic::AtomicUsize;

fn fixture()->(tempfile::TempDir,Arc<Manager>){
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");
    let repository=ServerRepository::new(root.clone());
    repository.save("local",&json!({"server_id":"local","name":"local","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"kind":"local_script","package":null,"executable":"/fixture/not-executed","args":[],"working_directory":null,"environment":[]}})).unwrap();
    let discovery=DiscoveryService::new(directory.path().join("empty-home"),None);
    let manager=Manager::with_grant_store(repository,Arc::new(GrantStore::memory(root.clone())),Runner::new(),Installer::new(root,discovery));
    (directory,manager)
}

#[tokio::test]
async fn local_link_needs_second_explicit_click_and_never_leaks_the_url(){
    let (_directory,manager)=fixture();let opened=Arc::new(AtomicUsize::new(0));let counted=opened.clone();
    manager.set_browser_opener(Arc::new(move|url|{
        assert_eq!(url,"https://accounts.example.com/authorize?client_id=fixture&response_type=code&state=private-fixture-state");
        counted.fetch_add(1,Ordering::SeqCst);Box::pin(async{Ok(json!({}))})
    }));
    {
        let mut states=manager.live.lock().await;let state=states.entry("local".into()).or_default();
        state.authenticating=true;state.auth_attempt=Some("fixture-attempt".into());state.auth=json!({"status":"authenticating","error":null});
    }
    manager.observe_native_sign_in("local","fixture-attempt","https://accounts.example.com/authorize?client_id=fixture&response_type=code&state=private-fixture-state".into()).await;
    assert_eq!(opened.load(Ordering::SeqCst),0,"receiving output must never open a browser");
    let status=manager.servers(vec!["local".into()],None).await.unwrap();
    assert_eq!(status["servers"][0]["authentication"]["sign_in"],json!({"host":"accounts.example.com","opened":false}));
    assert!(!status.to_string().contains("private-fixture-state"));
    assert!(!status.to_string().contains("client_id="));
    assert_eq!(manager.authenticate_mode("local",false).await.unwrap_err().code,"FORBIDDEN");
    assert_eq!(opened.load(Ordering::SeqCst),0);
    assert_eq!(manager.restart("local",json!({"interactive":true})).await.unwrap()["status"],"authenticating");
    assert_eq!(opened.load(Ordering::SeqCst),1);
    assert_eq!(manager.authenticate("local").await.unwrap()["status"],"authenticating");
    assert_eq!(opened.load(Ordering::SeqCst),1,"a repeated click must not reopen the same URL");
    let status=manager.servers(vec!["local".into()],None).await.unwrap();
    assert_eq!(status["servers"][0]["authentication"]["sign_in"]["opened"],true);
    assert_eq!(manager.repository.get_saved("local").unwrap().unwrap()["authentication"]["status"],"unknown");
    manager.shutdown().await;
}

#[tokio::test]
async fn native_browser_failure_can_be_retried_without_duplicate_startup(){
    let (_directory,manager)=fixture();let opens=Arc::new(AtomicUsize::new(0));let counted=opens.clone();
    manager.set_browser_opener(Arc::new(move|_|{let first=counted.fetch_add(1,Ordering::SeqCst)==0;Box::pin(async move{if first{Err(ManagementError::new(503,"AUTHENTICATION_BROWSER_UNAVAILABLE","Fixture browser failed."))}else{Ok(json!({}))}})}));
    {
        let mut states=manager.live.lock().await;let state=states.entry("local".into()).or_default();
        state.authenticating=true;state.auth_attempt=Some("retry-attempt".into());state.auth=json!({"status":"authenticating","error":null});
    }
    manager.observe_native_sign_in("local","retry-attempt","https://accounts.example.com/login?client_id=fixture&response_type=code".into()).await;
    assert_eq!(manager.authenticate("local").await.unwrap_err().code,"AUTHENTICATION_BROWSER_UNAVAILABLE");
    assert_eq!(manager.servers(vec!["local".into()],None).await.unwrap()["servers"][0]["authentication"]["sign_in"]["opened"],false);
    manager.restart("local",json!({"interactive":true})).await.unwrap();
    assert_eq!(opens.load(Ordering::SeqCst),2);
    assert!(manager.runner.server_ids().await.is_empty(),"consent must not spawn a second server");
    manager.shutdown().await;
}

#[tokio::test]
async fn remembered_terminal_result_is_last_known_and_never_current_proof(){
    let (_directory,manager)=fixture();
    manager.live.lock().await.entry("local".into()).or_default().auth=json!({"status":"authenticated","error":null});
    manager.remember_live_authentication("local").await.unwrap();
    assert_eq!(manager.repository.get_saved("local").unwrap().unwrap()["authentication"]["status"],"authenticated");
    manager.live.lock().await.clear();
    let status=manager.servers(vec!["local".into()],None).await.unwrap();
    assert_eq!(status["servers"][0]["authentication"]["status"],"unknown");
    assert_eq!(status["servers"][0]["authentication"]["last_known"]["status"],"authenticated");
    manager.live.lock().await.entry("local".into()).or_default().auth=json!({"status":"authenticating","error":null});
    manager.remember_live_authentication("local").await.unwrap();
    assert_eq!(manager.repository.get_saved("local").unwrap().unwrap()["authentication"]["status"],"authenticated","pending state is not persisted");
    assert_eq!(manager.restart("local",json!({"interactive":true,"force_restart":true})).await.unwrap_err().code,"INVALID_REQUEST");
    manager.shutdown().await;
}
