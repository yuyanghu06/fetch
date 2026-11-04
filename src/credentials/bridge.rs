//! Private, fixed-owner credential storage for supervised local helpers.
//! This channel never carries MCP messages and cannot choose another server/store.
use super::{CredentialError, CredentialStore};
use rand::{RngCore, rngs::OsRng};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fmt, future::Future, path::PathBuf, pin::Pin,
    sync::{Arc, Mutex as StdMutex, Weak, atomic::{AtomicBool, AtomicU64, Ordering}},
    time::Duration, os::unix::fs::{DirBuilderExt, PermissionsExt}};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{UnixListener, UnixStream}, sync::{Mutex, Notify}};

pub(crate) const BRIDGE_CLIENT: &str = include_str!("bridge_client.mjs");
const MAX_FRAME: usize = 1024 * 1024;
const REQUEST_SECONDS: u64 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BridgeError { Invalid, Storage, Stale, Revoked, Verification, Unavailable }
impl BridgeError {
    fn code(self) -> &'static str { match self { Self::Invalid=>"invalid", Self::Storage=>"storage", Self::Stale=>"stale", Self::Revoked=>"revoked", Self::Verification=>"verification", Self::Unavailable=>"unavailable" } }
}
impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("The helper credential operation could not be completed safely.") }
}
impl std::error::Error for BridgeError {}

pub(crate) type Verifier = Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output=Result<(), BridgeError>> + Send>> + Send + Sync>;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Policy { Login, Runtime }

#[derive(Clone)]
pub struct Factory(Arc<Owner>);
#[derive(Clone)]
pub(crate) struct Controller(Arc<Owner>);
struct Owner {
    store: Arc<CredentialStore>, server: String, adapter: String, verifier: Verifier,
    gate: Mutex<()>, disk_gate: StdMutex<()>, epoch: AtomicU64, revoked: AtomicBool, sessions: StdMutex<Vec<Weak<Session>>>,
    failure: StdMutex<Option<Arc<dyn Fn(BridgeError, Arc<dyn Fn() -> bool + Send + Sync>) + Send + Sync>>>,
}
struct Session {
    owner: Weak<Owner>, epoch: u64, policy: Policy, capability: String,
    directory: PathBuf, socket: PathBuf, closed: AtomicBool, stop: Notify,
}
pub(crate) struct Lease(Arc<Session>);
impl Session {
    fn close(&self) {
        self.closed.store(true, Ordering::Release); self.stop.notify_waiters();
        // Only these two task-created paths are removed. No recursive cleanup.
        let _ = std::fs::remove_file(&self.socket);
        let _ = std::fs::remove_dir(&self.directory);
    }
    fn active(&self, owner: &Owner) -> Result<(), BridgeError> {
        if self.closed.load(Ordering::Acquire) || owner.revoked.load(Ordering::Acquire) || self.epoch != owner.epoch.load(Ordering::Acquire) { Err(BridgeError::Revoked) } else { Ok(()) }
    }
    async fn stopped(&self) {
        loop { let notice=self.stop.notified(); if self.closed.load(Ordering::Acquire) { return; } notice.await; }
    }
}
impl Drop for Session { fn drop(&mut self) { self.close(); } }
impl Lease {
    pub fn environment(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("FETCH_HELPER_SOCKET".into(),self.0.socket.to_string_lossy().into_owned()),
            ("FETCH_HELPER_CAPABILITY".into(),self.0.capability.clone())])
    }
    pub fn close(&self) { self.0.close(); }
}
impl Drop for Lease { fn drop(&mut self) { self.close(); } }
impl Factory {
    pub(crate) fn new(store: Arc<CredentialStore>, server: String, adapter: String, verifier: Verifier) -> Self {
        Self(Arc::new(Owner { store,server,adapter,verifier,gate:Mutex::new(()),disk_gate:StdMutex::new(()),epoch:AtomicU64::new(0),revoked:AtomicBool::new(false),sessions:StdMutex::new(vec![]),failure:StdMutex::new(None) }))
    }
    pub(crate) fn with_failure_handler(self, handler: Arc<dyn Fn(BridgeError, Arc<dyn Fn() -> bool + Send + Sync>) + Send + Sync>) -> Self {
        if let Ok(mut stored)=self.0.failure.lock() { *stored=Some(handler); }
        self
    }
    pub(crate) fn controller(&self) -> Controller { Controller(self.0.clone()) }
    pub(crate) async fn start(&self, policy: Policy) -> Result<Lease, BridgeError> {
        let _guard=self.0.gate.lock().await;
        if self.0.revoked.load(Ordering::Acquire) { return Err(BridgeError::Revoked); }
        if self.0.server.is_empty() || self.0.adapter.is_empty() { return Err(BridgeError::Invalid); }
        // macOS has a short Unix-socket path limit; Application Support paths do not fit.
        let mut random=[0u8;32]; OsRng.fill_bytes(&mut random);
        let capability=random.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
        let directory=PathBuf::from("/private/tmp").join(format!("fetch-helper-{}",&capability[..24]));
        let mut builder=std::fs::DirBuilder::new(); builder.mode(0o700);
        builder.create(&directory).map_err(|_|BridgeError::Unavailable)?;
        let socket=directory.join("s");
        let listener=match UnixListener::bind(&socket) { Ok(listener)=>listener,Err(_)=>{let _=std::fs::remove_dir(&directory);return Err(BridgeError::Unavailable);} };
        if std::fs::set_permissions(&socket,std::fs::Permissions::from_mode(0o600)).is_err() {
            let _=std::fs::remove_file(&socket);let _=std::fs::remove_dir(&directory);return Err(BridgeError::Unavailable);
        }
        let session=Arc::new(Session {owner:Arc::downgrade(&self.0),epoch:self.0.epoch.load(Ordering::Acquire),policy,capability,directory,socket,closed:AtomicBool::new(false),stop:Notify::new()});
        session.active(&self.0)?;
        { let mut sessions=self.0.sessions.lock().map_err(|_|BridgeError::Unavailable)?; sessions.retain(|entry|entry.strong_count()>0);sessions.push(Arc::downgrade(&session)); }
        let running=session.clone();
        tokio::spawn(async move {
            loop {
                let accepted=tokio::select! { biased; _=running.stopped()=>break, value=listener.accept()=>value };
                let Ok((stream,_))=accepted else { break; };
                let lease=running.clone();
                tokio::spawn(async move {
                    let request=serve(stream,lease.clone());
                    tokio::select! { biased; _=lease.stopped()=>{},_=request=>{} }
                });
            }
            running.close();
        });
        Ok(Lease(session))
    }
}
impl Controller {
    pub async fn revoke(&self) {
        self.0.revoked.store(true,Ordering::Release);
        self.0.epoch.fetch_add(1,Ordering::AcqRel);
        if let Ok(mut sessions)=self.0.sessions.lock() { for session in sessions.drain(..).filter_map(|entry|entry.upgrade()) { session.close(); } }
        // Fence a blocking write already in flight before a replacement context starts.
        let _guard=self.0.gate.lock().await;
        let owner=self.0.clone();
        let _=tokio::task::spawn_blocking(move || {let _guard=owner.disk_gate.lock();}).await;
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request { capability: String, op: String, #[serde(default)] expected_revision: Option<u64>, #[serde(default)] payload: Option<Value> }
#[derive(Clone)]
struct Snapshot { revision: u64, payload: Value, verified: bool }
impl Snapshot {
    fn response(&self) -> Value { json!({"ok":true,"revision":self.revision,"payload":self.payload,"verified":self.verified}) }
}
fn snapshot(document: &Value, adapter: &str) -> Result<Snapshot, BridgeError> {
    if !document.is_object() { return Err(BridgeError::Storage); }
    let Some(helper)=document.get("helper") else { return Ok(Snapshot {revision:0,payload:Value::Null,verified:false}); };
    let object=helper.as_object().ok_or(BridgeError::Storage)?;
    if object.len()!=4 || !["adapter","revision","payload","verified_revision"].iter().all(|key|object.contains_key(*key))
        || helper["adapter"]!=adapter || !helper["payload"].is_object() { return Err(BridgeError::Storage); }
    let revision=helper["revision"].as_u64().filter(|value|*value>0).ok_or(BridgeError::Storage)?;
    let verified=match &helper["verified_revision"] {Value::Null=>false,value=>value.as_u64().filter(|value|*value==revision).map(|_|true).ok_or(BridgeError::Storage)?};
    Ok(Snapshot {revision,payload:helper["payload"].clone(),verified})
}
async fn read(owner: &Arc<Owner>) -> Result<Snapshot, BridgeError> {
    let owner=owner.clone();
    tokio::task::spawn_blocking(move || {
        let document=owner.store.read_json(&owner.server).map_err(|_|BridgeError::Storage)?.unwrap_or_else(||json!({"manual":{},"oauth":null}));
        snapshot(&document,&owner.adapter)
    }).await.map_err(|_|BridgeError::Unavailable)?
}
async fn write(owner: &Arc<Owner>, session: &Arc<Session>, expected: u64, payload: Option<Value>, verified: bool) -> Result<Snapshot, BridgeError> {
    let owner=owner.clone();let session=session.clone();
    tokio::task::spawn_blocking(move || {
        let _guard=owner.disk_gate.lock().map_err(|_|BridgeError::Storage)?;
        let mut failure=None;
        let updated=owner.store.update_json(&owner.server,&json!({"manual":{},"oauth":null}),|document| {
            let current=match snapshot(document,&owner.adapter) {Ok(current)=>current,Err(error)=>{failure=Some(error);return Err(CredentialError::InvalidDocument);}};
            if let Err(error)=session.active(&owner) {failure=Some(error);return Err(CredentialError::InvalidDocument);}
            if current.revision!=expected {failure=Some(BridgeError::Stale);return Err(CredentialError::InvalidDocument);}
            let revision=if payload.is_some(){expected.checked_add(1).ok_or(CredentialError::InvalidDocument)?}else{expected};
            let value=payload.as_ref().unwrap_or(&current.payload);
            if !value.is_object() || revision==0 {failure=Some(BridgeError::Invalid);return Err(CredentialError::InvalidDocument);}
            document["helper"]=json!({"adapter":owner.adapter,"revision":revision,"payload":value,"verified_revision":if verified{Some(revision)}else{None}});
            Ok(())
        });
        let document=updated.map_err(|_|failure.unwrap_or(BridgeError::Storage))?.document;
        session.active(&owner)?;
        snapshot(&document,&owner.adapter)
    }).await.map_err(|_|BridgeError::Unavailable)?
}
async fn operate(session: &Arc<Session>, request: Request) -> Result<Snapshot, BridgeError> {
    if request.capability!=session.capability {return Err(BridgeError::Invalid);}
    let owner=session.owner.upgrade().ok_or(BridgeError::Revoked)?;
    let _guard=owner.gate.lock().await;session.active(&owner)?;
    match request.op.as_str() {
        "load" if request.expected_revision.is_none() && request.payload.is_none()=>read(&owner).await,
        "update" if request.payload.as_ref().is_some_and(Value::is_object)=>{
            let saved=write(&owner,session,request.expected_revision.ok_or(BridgeError::Invalid)?,request.payload,false).await?;
            if session.policy==Policy::Login {return Ok(saved);}
            (owner.verifier)(saved.payload.clone()).await.map_err(|_|BridgeError::Verification)?;
            session.active(&owner)?;
            write(&owner,session,saved.revision,None,true).await
        },
        "verify" if request.payload.is_none()=>{
            let saved=read(&owner).await?;
            if saved.revision!=request.expected_revision.ok_or(BridgeError::Invalid)? {return Err(BridgeError::Stale);}
            if !saved.payload.is_object() {return Err(BridgeError::Invalid);}
            // A rejected recheck must not leave an earlier proof durable. Keep
            // the payload/revision and invalidate only its verified marker.
            let saved=write(&owner,session,saved.revision,None,false).await?;
            (owner.verifier)(saved.payload.clone()).await.map_err(|_|BridgeError::Verification)?;
            session.active(&owner)?;
            write(&owner,session,saved.revision,None,true).await
        },
        _=>Err(BridgeError::Invalid),
    }
}
async fn serve(stream: UnixStream, session: Arc<Session>) -> Result<(), BridgeError> {
    let (reader,mut writer)=stream.into_split();let mut reader=BufReader::new(reader);
    let mut bytes=Vec::new();
    use tokio::io::AsyncReadExt;
    let count=tokio::time::timeout(Duration::from_secs(REQUEST_SECONDS),(&mut reader).take((MAX_FRAME+1) as u64).read_until(b'\n',&mut bytes))
        .await.map_err(|_|BridgeError::Unavailable)?.map_err(|_|BridgeError::Unavailable)?;
    if count==0 || bytes.len()>MAX_FRAME || bytes.last()!=Some(&b'\n') {return Err(BridgeError::Invalid);}
    let result=match serde_json::from_slice::<Request>(&bytes) {
        Ok(request)=>{
            let authorized=request.capability==session.capability;
            let result=tokio::time::timeout(Duration::from_secs(REQUEST_SECONDS),operate(&session,request)).await.unwrap_or(Err(BridgeError::Unavailable));
            if authorized {if let Err(error)=result {if let Some(owner)=session.owner.upgrade() {
                let handler=owner.failure.lock().ok().and_then(|stored|stored.clone());
                if let Some(handler)=handler {
                    // The owner can survive a process restart. A queued failure
                    // must still belong to this exact, open process lease.
                    let pending=Arc::downgrade(&session);
                    let current:Arc<dyn Fn()->bool+Send+Sync>=Arc::new(move|| {
                        let Some(session)=pending.upgrade()else{return false;};
                        session.owner.upgrade().is_some_and(|owner|session.active(&owner).is_ok())
                    });
                    handler(error,current);
                }
            }}}
            result
        },
        Err(_)=>Err(BridgeError::Invalid),
    };
    let response=match result {Ok(saved)=>saved.response(),Err(error)=>json!({"ok":false,"error":error.code()})};
    let mut bytes=serde_json::to_vec(&response).map_err(|_|BridgeError::Invalid)?;bytes.push(b'\n');
    tokio::time::timeout(Duration::from_secs(REQUEST_SECONDS),writer.write_all(&bytes)).await.map_err(|_|BridgeError::Unavailable)?.map_err(|_|BridgeError::Unavailable)?;
    writer.shutdown().await.map_err(|_|BridgeError::Unavailable)
}
