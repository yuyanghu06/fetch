//! Fetch-as-client grants for remote HTTP MCP OAuth. Local server credentials
//! never enter this store. Native Keychain I/O stays off the async executor.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::PathBuf, sync::{Arc, Mutex}};
use zeroize::Zeroizing;

pub(crate) mod sdk;
const MAX_GRANT_BYTES: usize = 1024 * 1024;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrantError { StorageUnavailable, InvalidDocument, IntegrityFailed }
impl std::fmt::Display for GrantError {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
        f.write_str("Fetch could not read or save this remote MCP login in macOS Keychain. Existing credentials were not migrated or removed.")
    }
}
impl std::error::Error for GrantError {}

/// Storage seam for disposable fixtures. Production uses only NativeKeychain.
/// Implementations must return errors rather than falling back to another store.
pub trait Backend: Send + Sync {
    fn read(&self, service:&str, account:&str)->Result<Option<Zeroizing<Vec<u8>>>, GrantError>;
    fn write(&self, service:&str, account:&str, value:&[u8])->Result<(), GrantError>;
}
struct Inner {
    service:String,
    backend:Arc<dyn Backend>,
    gates:Mutex<BTreeMap<String, Arc<Mutex<()>>>>,
    refresh_gates:Mutex<BTreeMap<String,Arc<tokio::sync::Mutex<()>>>>,
}
#[derive(Clone)]
pub struct GrantStore { inner:Arc<Inner> }
impl GrantStore {
    pub fn new(root:PathBuf)->Self { Self::with_backend(root, Arc::new(NativeKeychain)) }
    pub fn with_backend(root:PathBuf, backend:Arc<dyn Backend>)->Self {
        let root=std::fs::canonicalize(&root).unwrap_or(root);
        let namespace=format!("{:x}",Sha256::digest(root.as_os_str().as_encoded_bytes()));
        Self { inner:Arc::new(Inner { service:format!("com.fetch.local.remote-oauth.{namespace}"), backend, gates:Mutex::new(BTreeMap::new()), refresh_gates:Mutex::new(BTreeMap::new()) }) }
    }
    /// Explicit fixture backend; never selected as a storage fallback.
    pub fn memory(root:PathBuf)->Self { Self::with_backend(root,Arc::new(MemoryBackend::default())) }
    pub async fn read_json(&self,id:&str)->Result<Option<Value>,GrantError> {
        let account=account(id)?; let id=id.to_owned(); let inner=self.inner.clone();
        tokio::task::spawn_blocking(move||{
            let gate=gate(&inner,&account)?; let _guard=gate.lock().map_err(|_|GrantError::StorageUnavailable)?;
            read(&inner,&account,&id)
        }).await.map_err(|_|GrantError::StorageUnavailable)?
    }
    pub async fn save_json(&self,id:&str,document:&Value)->Result<(),GrantError> {
        let account=account(id)?;
        if !document.is_object() { return Err(GrantError::InvalidDocument); }
        let bytes=Zeroizing::new(serde_json::to_vec(&json!({"version":1,"server_id":id,"document":document})).map_err(|_|GrantError::InvalidDocument)?);
        if bytes.len()>MAX_GRANT_BYTES { return Err(GrantError::InvalidDocument); }
        let id=id.to_owned(); let inner=self.inner.clone();
        tokio::task::spawn_blocking(move||{
            let gate=gate(&inner,&account)?;let _guard=gate.lock().map_err(|_|GrantError::StorageUnavailable)?;
            // Fail closed on malformed/cross-instance data rather than overwriting it.
            let _=read(&inner,&account,&id)?;
            inner.backend.write(&inner.service,&account,&bytes)?;
            let saved=inner.backend.read(&inner.service,&account)?.ok_or(GrantError::IntegrityFailed)?;
            if saved.as_slice()!=bytes.as_slice() { return Err(GrantError::IntegrityFailed); }
            Ok(())
        }).await.map_err(|_|GrantError::StorageUnavailable)?
    }
    pub(crate) fn refresh_gate(&self,id:&str)->Result<Arc<tokio::sync::Mutex<()>>,GrantError>{let account=account(id)?;Ok(self.inner.refresh_gates.lock().map_err(|_|GrantError::StorageUnavailable)?.entry(account).or_default().clone())}
    /// Replace one independent section without losing concurrent SDK/manual updates.
    pub async fn save_section(&self,id:&str,section:&str,value:Value)->Result<(),GrantError>{
        if !["manual","sdk"].contains(&section){return Err(GrantError::InvalidDocument);}
        let account=account(id)?;let id=id.to_owned();let section=section.to_owned();let inner=self.inner.clone();
        tokio::task::spawn_blocking(move||{
            let gate=gate(&inner,&account)?;let _guard=gate.lock().map_err(|_|GrantError::StorageUnavailable)?;
            let mut document=read(&inner,&account,&id)?.unwrap_or_else(||json!({}));document[&section]=value;
            let bytes=Zeroizing::new(serde_json::to_vec(&json!({"version":1,"server_id":id,"document":document})).map_err(|_|GrantError::InvalidDocument)?);
            if bytes.len()>MAX_GRANT_BYTES{return Err(GrantError::InvalidDocument);}
            inner.backend.write(&inner.service,&account,&bytes)?;
            let saved=inner.backend.read(&inner.service,&account)?.ok_or(GrantError::IntegrityFailed)?;
            if saved.as_slice()!=bytes.as_slice(){return Err(GrantError::IntegrityFailed);}
            Ok(())
        }).await.map_err(|_|GrantError::StorageUnavailable)?
    }

}
fn account(id:&str)->Result<String,GrantError> {
    if id.is_empty() || id.len()>1024 || id.contains('\0') || id.starts_with("fetch-internal:") { return Err(GrantError::InvalidDocument); }
    Ok(format!("{:x}",Sha256::digest(id.as_bytes())))
}
fn gate(inner:&Inner,account:&str)->Result<Arc<Mutex<()>>,GrantError>{
    Ok(inner.gates.lock().map_err(|_|GrantError::StorageUnavailable)?.entry(account.to_owned()).or_default().clone())
}
fn read(inner:&Inner,account:&str,id:&str)->Result<Option<Value>,GrantError>{
    let Some(bytes)=inner.backend.read(&inner.service,account)? else {return Ok(None)};
    if bytes.len()>MAX_GRANT_BYTES {return Err(GrantError::InvalidDocument)}
    let value:Value=serde_json::from_slice(&bytes).map_err(|_|GrantError::InvalidDocument)?;
    if value["version"]!=1 || value["server_id"]!=id || !value["document"].is_object(){return Err(GrantError::IntegrityFailed)}
    Ok(Some(value["document"].clone()))
}
#[derive(Default)]
struct MemoryBackend(Mutex<BTreeMap<(String,String),Zeroizing<Vec<u8>>>>);
impl Backend for MemoryBackend {
    fn read(&self,service:&str,account:&str)->Result<Option<Zeroizing<Vec<u8>>>,GrantError>{
        Ok(self.0.lock().map_err(|_|GrantError::StorageUnavailable)?.get(&(service.to_owned(),account.to_owned())).cloned())
    }
    fn write(&self,service:&str,account:&str,value:&[u8])->Result<(),GrantError>{
        self.0.lock().map_err(|_|GrantError::StorageUnavailable)?.insert((service.to_owned(),account.to_owned()),Zeroizing::new(value.to_vec()));Ok(())
    }
}
struct NativeKeychain;
impl Backend for NativeKeychain {
    fn read(&self,service:&str,account:&str)->Result<Option<Zeroizing<Vec<u8>>>,GrantError>{
        #[cfg(target_os="macos")]
        {
            let service=std::ffi::CString::new(service).map_err(|_|GrantError::InvalidDocument)?;
            let account=std::ffi::CString::new(account).map_err(|_|GrantError::InvalidDocument)?;
            let mut bytes=Zeroizing::new(vec![0;MAX_GRANT_BYTES]);let mut length=0;
            // SAFETY: native code copies at most capacity bytes and reports its length.
            let status=unsafe { fetch_remote_grant_read(service.as_ptr(),account.as_ptr(),bytes.as_mut_ptr(),bytes.len(),&mut length) };
            match status { 0 if length<=bytes.len()=>{bytes.truncate(length);Ok(Some(bytes))},-25300=>Ok(None),_=>Err(GrantError::StorageUnavailable) }
        }
        #[cfg(not(target_os="macos"))]
        { let _=(service,account);Err(GrantError::StorageUnavailable) }
    }
    fn write(&self,service:&str,account:&str,value:&[u8])->Result<(),GrantError>{
        #[cfg(target_os="macos")]
        {
            let service=std::ffi::CString::new(service).map_err(|_|GrantError::InvalidDocument)?;
            let account=std::ffi::CString::new(account).map_err(|_|GrantError::InvalidDocument)?;
            // SAFETY: input and NUL-terminated strings remain alive during the call.
            let status=unsafe {fetch_remote_grant_write(service.as_ptr(),account.as_ptr(),value.as_ptr(),value.len())};
            if status==0 {Ok(())}else{Err(GrantError::StorageUnavailable)}
        }
        #[cfg(not(target_os="macos"))]
        { let _=(service,account,value);Err(GrantError::StorageUnavailable) }
    }
}
#[cfg(target_os="macos")]
unsafe extern "C" {
    fn fetch_remote_grant_read(service:*const std::ffi::c_char,account:*const std::ffi::c_char,output:*mut u8,capacity:usize,length:*mut usize)->i32;
    fn fetch_remote_grant_write(service:*const std::ffi::c_char,account:*const std::ffi::c_char,input:*const u8,length:usize)->i32;
}
#[cfg(test)]
mod tests;
