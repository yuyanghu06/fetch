//! Native browser presentation only; all OAuth rules live in the official SDK.
use super::ManagementError;
pub(super) async fn open_browser(target:&str)->Result<(),ManagementError>{
    let mut browser=tokio::process::Command::new("/usr/bin/open");
    browser.arg(target).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).kill_on_drop(true);
    match tokio::time::timeout(std::time::Duration::from_secs(5),browser.status()).await{
        Ok(Ok(status)) if status.success()=>Ok(()),
        _=>Err(ManagementError::new(503,"AUTHENTICATION_BROWSER_UNAVAILABLE","The sign-in browser could not be opened. Retry Authenticate explicitly.")),
    }
}
