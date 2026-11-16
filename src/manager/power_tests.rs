//! Private power conflicts and quiet disabled-server paths. No process/network runs.
use super::*;
use crate::{discovery::DiscoveryService, remote_oauth::GrantStore};
use std::sync::atomic::AtomicUsize;

struct NoNetwork(AtomicUsize);
impl rmcp::transport::auth::OAuthHttpClient for NoNetwork {
    fn execute(&self, _:rmcp::transport::auth::OAuthHttpRequest) -> rmcp::transport::auth::OAuthHttpClientFuture<'_> {
        self.0.fetch_add(1,Ordering::SeqCst);
        Box::pin(async { Err(Box::new(std::io::Error::other("fixture forbids network")) as rmcp::transport::auth::OAuthHttpClientError) })
    }
}
fn fixture(enabled:bool) -> (tempfile::TempDir,Arc<Manager>,Arc<GrantStore>) {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");
    let repository=ServerRepository::new(root.clone());
    repository.save("remote",&json!({"server_id":"remote","name":"Remote fixture","runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,
        "config":{"enabled":enabled,"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],
            "connection":{"url":"https://uncontacted.fixture.invalid/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
    let discovery=DiscoveryService::new(directory.path().join("empty-home"),None);
    let grants=Arc::new(GrantStore::memory(root.clone()));
    let manager=Manager::with_grant_store(repository,grants.clone(),Runner::new(),Installer::new(root,discovery));
    (directory,manager,grants)
}

#[tokio::test]
async fn saved_off_is_quiet_across_startup_install_followup_and_remote_auth_paths() {
    let (_directory,manager,grants)=fixture(false);
    // A stale saved SDK section would normally trigger renewal; no live store is used.
    grants.save_json("remote",&json!({"sdk":{"fixture":"stale saved context"}})).await.unwrap();
    let provider=Arc::new(NoNetwork(AtomicUsize::new(0)));manager.set_oauth_http(provider.clone());
    let renewal=Arc::new(AtomicUsize::new(0));let count=renewal.clone();
    manager.set_authenticate_sender(Arc::new(move|_|{count.fetch_add(1,Ordering::SeqCst);Box::pin(async{Ok(json!({"status":"unknown"}))})}));
    manager.set_authentication_expiry("remote",Some(0)).await;
    manager.startup().await;
    manager.clone().connect_installed("remote".into()).await;
    let saved=manager.repository.get_saved("remote").unwrap().unwrap();
    let _=manager.renew_saved_remote(&saved).await;
    let _=manager.before_remote_use("remote").await;
    assert_eq!(manager.authenticate_background("remote").await.unwrap_err().code,"SERVER_DISABLED");
    assert_eq!(manager.authenticate_mode("remote",true).await.unwrap_err().code,"SERVER_DISABLED");
    assert_eq!(renewal.load(Ordering::SeqCst),0,"Off blocks background renewal requests");
    assert_eq!(provider.0.load(Ordering::SeqCst),0,"Off never probes the remote provider");
    assert_eq!(manager.repository.get_saved("remote").unwrap().unwrap()["config"]["enabled"],false);
    assert_eq!(manager.runner.snapshot("remote").await["runtime"]["status"],"stopped");
    manager.shutdown().await;
}

#[tokio::test]
async fn stop_refuses_pending_login_or_restart_without_disabling_the_server() {
    let (_directory,manager,_grants)=fixture(true);
    {
        let mut live=manager.live.lock().await;
        let state=live.entry("remote".into()).or_default();state.authenticating=true;
    }
    let error=manager.restart("remote",json!({"action":"stop"})).await.unwrap_err();
    assert_eq!(error.http_status,409);assert_eq!(error.code,"SERVER_BUSY");
    assert_eq!(manager.repository.get_saved("remote").unwrap().unwrap()["config"]["enabled"],true);
    {
        let mut live=manager.live.lock().await;
        let state=live.get_mut("remote").unwrap();state.authenticating=false;state.restart=Some("held-restart".into());
    }
    manager.attempts.lock().await.insert(("remote".into(),"held-restart".into()),json!({"restart_id":"held-restart","status":"restarting","phase":"waiting_for_calls"}));
    let error=manager.restart("remote",json!({"action":"stop"})).await.unwrap_err();
    assert_eq!(error.http_status,409);assert_eq!(error.code,"SERVER_BUSY");
    assert_eq!(manager.repository.get_saved("remote").unwrap().unwrap()["config"]["enabled"],true);
    manager.shutdown().await;
}
