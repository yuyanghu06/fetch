//! Native window authority and fresh, request-bound Mac confirmation.
//! No HTTP handler can create a Settings session or turn a bearer token into one.
use crate::authority::Caller;
use axum::http::HeaderMap;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{RngCore, rngs::OsRng};
use std::{collections::BTreeMap, ffi::CString, sync::{Arc, Mutex, Weak, atomic::{AtomicBool,Ordering}}, time::{Duration, Instant}};

#[derive(Clone, Copy, Debug)]
pub enum NativeAuthError { Denied, Timeout, Unavailable, Expired }
impl std::fmt::Display for NativeAuthError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str(match self { Self::Denied=>"Mac confirmation was cancelled or denied.", Self::Timeout=>"Mac confirmation expired.", Self::Unavailable=>"Bring Fetch forward to confirm this change on your Mac.", Self::Expired=>"Reopen Fetch Settings and confirm the change again." }) }
}
impl std::error::Error for NativeAuthError {}

pub struct NativeAuthority { sessions:Mutex<BTreeMap<String, Instant>>, prompts:tokio::sync::Mutex<()> }
pub struct NativeSettingsSession { token:String, authority:Weak<NativeAuthority> }
impl NativeSettingsSession { pub fn header_value(&self)->&str { &self.token } }
impl Drop for NativeSettingsSession { fn drop(&mut self) { if let Some(authority)=self.authority.upgrade() { if let Ok(mut sessions)=authority.sessions.lock() { sessions.remove(&self.token); } } } }
/// This type has no public constructor, serializer or Clone implementation.
pub struct SettingsCapability { authority:Weak<NativeAuthority>, session:String, server:String, fingerprint:String, expires:Instant, used:AtomicBool }
impl NativeAuthority {
    pub fn new()->Arc<Self> { Arc::new(Self { sessions:Mutex::new(BTreeMap::new()), prompts:tokio::sync::Mutex::new(()) }) }
    /// Called only by the in-process native window creation path.
    pub fn open_settings(self:&Arc<Self>)->NativeSettingsSession {
        let token=random_token();
        if let Ok(mut sessions)=self.sessions.lock() { sessions.insert(token.clone(), Instant::now()); }
        NativeSettingsSession { token, authority:Arc::downgrade(self) }
    }
    pub fn authorize_settings_headers(&self,headers:&HeaderMap)->Option<Caller> {
        let values:Vec<_>=headers.get_all("x-fetch-settings-capability").iter().collect();
        let [value]=values.as_slice() else { return None; };
        let token=value.to_str().ok()?;
        self.sessions.lock().ok()?.get(token)?;
        Some(Caller::from_native_settings(format!("native:{}", token_digest(token))))
    }
    /// Mac presence for app approval; it deliberately returns no Settings capability.
    pub async fn confirm_presence(&self, reason:&str)->Result<(),NativeAuthError> {
        self.confirm_presence_with_timeout(reason,Duration::from_secs(120)).await
    }
    pub async fn confirm_presence_with_timeout(&self,reason:&str,budget:Duration)->Result<(),NativeAuthError> {
        if budget.is_zero() { return Err(NativeAuthError::Timeout); }
        let _prompt=self.prompts.try_lock().map_err(|_|NativeAuthError::Unavailable)?;
        let reason=CString::new(reason).map_err(|_|NativeAuthError::Unavailable)?;
        let seconds=budget.as_secs().saturating_add(u64::from(budget.subsec_nanos()!=0)).min(120) as u32;
        tokio::task::spawn_blocking(move || platform_confirm(reason,seconds)).await.map_err(|_|NativeAuthError::Unavailable)?
    }
    pub async fn confirm_settings(self:&Arc<Self>,session:&NativeSettingsSession,server_id:&str,mutation_fingerprint:&str)->Result<SettingsCapability,NativeAuthError> {
        self.session_valid(session)?;
        let reason=format!("Save the selected credential changes for {server_id} in Fetch Settings.");
        self.confirm_presence(&reason).await?;
        self.session_valid(session)?;
        Ok(SettingsCapability { authority:Arc::downgrade(self),session:session.token.clone(),server:server_id.to_owned(),fingerprint:mutation_fingerprint.to_owned(),expires:Instant::now()+Duration::from_secs(30),used:AtomicBool::new(false) })
    }
    fn session_valid(self:&Arc<Self>,session:&NativeSettingsSession)->Result<(),NativeAuthError> {
        if !session.authority.upgrade().is_some_and(|a|Arc::ptr_eq(&a,self)) || !self.sessions.lock().map_err(|_|NativeAuthError::Unavailable)?.contains_key(&session.token) { return Err(NativeAuthError::Expired); } Ok(())
    }
    pub fn validate(self:&Arc<Self>,capability:&SettingsCapability)->Result<(),NativeAuthError> {
        if capability.expires<=Instant::now() || !capability.authority.upgrade().is_some_and(|a|Arc::ptr_eq(&a,self)) || !self.sessions.lock().map_err(|_|NativeAuthError::Unavailable)?.contains_key(&capability.session) { return Err(NativeAuthError::Expired); } Ok(())
    }
    pub fn validate_for(self:&Arc<Self>,capability:&SettingsCapability,server_id:&str,fingerprint:&str)->Result<(),NativeAuthError> {
        self.validate(capability)?;
        if capability.server!=server_id || capability.fingerprint!=fingerprint || capability.used.compare_exchange(false,true,Ordering::AcqRel,Ordering::Acquire).is_err() { return Err(NativeAuthError::Denied); } Ok(())
    }
    pub async fn confirm_settings_headers(self:&Arc<Self>,headers:&HeaderMap,server_id:&str,fingerprint:&str)->Result<SettingsCapability,NativeAuthError> {
        let values:Vec<_>=headers.get_all("x-fetch-settings-capability").iter().collect();
        let [value]=values.as_slice() else { return Err(NativeAuthError::Denied); };
        let token=value.to_str().map_err(|_|NativeAuthError::Denied)?.to_owned();
        if !self.sessions.lock().map_err(|_|NativeAuthError::Unavailable)?.contains_key(&token) { return Err(NativeAuthError::Denied); }
        self.confirm_presence(&format!("Save the selected credential changes for {server_id} in Fetch Settings.")).await?;
        if !self.sessions.lock().map_err(|_|NativeAuthError::Unavailable)?.contains_key(&token) { return Err(NativeAuthError::Expired); }
        Ok(SettingsCapability { authority:Arc::downgrade(self),session:token,server:server_id.to_owned(),fingerprint:fingerprint.to_owned(),expires:Instant::now()+Duration::from_secs(30),used:AtomicBool::new(false) })
    }
    pub fn owner_is_active(&self,owner:&str)->bool { self.sessions.lock().map(|sessions|sessions.keys().any(|token|owner==format!("native:{}",token_digest(token)))).unwrap_or(false) }
    pub fn invalidate_all(&self) { if let Ok(mut sessions)=self.sessions.lock() { sessions.clear(); } }
}
fn token_digest(token:&str)->String { use sha2::{Digest,Sha256}; format!("{:x}",Sha256::digest(token.as_bytes())) }
fn random_token()->String { let mut value=[0u8;32];OsRng.fill_bytes(&mut value);URL_SAFE_NO_PAD.encode(value) }
#[cfg(target_os="macos")]
unsafe extern "C" { fn fetch_confirm_owner(reason:*const std::ffi::c_char,timeout_seconds:u32)->i32; }
#[cfg(target_os="macos")]
fn platform_confirm(reason:CString,seconds:u32)->Result<(),NativeAuthError> {
    // SAFETY: immutable NUL-terminated reason remains alive until the bounded native call returns.
    match unsafe { fetch_confirm_owner(reason.as_ptr(),seconds) } { 0=>Ok(()),1=>Err(NativeAuthError::Denied),2=>Err(NativeAuthError::Timeout),_=>Err(NativeAuthError::Unavailable) }
}
#[cfg(not(target_os="macos"))]
fn platform_confirm(_:CString,_:u32)->Result<(),NativeAuthError> { Err(NativeAuthError::Unavailable) }

#[cfg(target_os="macos")]
unsafe extern "C" { fn fetch_install_app_menu(); fn fetch_take_app_action()->i32; }
/// Register native actions without taking over Iced's application delegate.
pub fn install_app_menu() {
    #[cfg(target_os="macos")]
    // SAFETY: the bridge dispatches UI setup onto the existing main queue.
    unsafe { fetch_install_app_menu(); }
}
pub fn take_app_action()->i32 {
    #[cfg(target_os="macos")]
    // SAFETY: this reads and clears a bridge-owned atomic action flag.
    unsafe { return fetch_take_app_action(); }
    #[cfg(not(target_os="macos"))]
    { 0 }
}
