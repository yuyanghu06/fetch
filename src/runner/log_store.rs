//! Coalesced persistence of already sanitized console history, never terminal IO.
use super::startup_diagnostics::StartupDiagnostics;
use crate::storage::server_directory;
use serde_json::Value;
use std::{collections::{BTreeMap, BTreeSet}, path::PathBuf, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, time::Duration};
use tokio::{sync::{Mutex as AsyncMutex, Notify}, task::JoinHandle};

const FILE_NAME: &str = "console-log.json";
// JSON escaping can be larger than the retained one-MiB text history.
const MAX_FILE_BYTES: usize = 3 * 1024 * 1024;
const SAVE_FAILURE: &str = "Console history could not be saved; it is still available in this Fetch run.";

#[derive(Default)]
struct Pending {
    captures: BTreeMap<String, Arc<StartupDiagnostics>>,
    dirty: BTreeSet<String>,
}
pub(super) struct LogStore {
    root: PathBuf,
    pending: Mutex<Pending>,
    changed: Arc<Notify>,
    worker: Mutex<Option<JoinHandle<()>>>,
    io: AsyncMutex<()>,
    stopping: AtomicBool,
}
impl LogStore {
    pub(super) fn new(root: PathBuf) -> Arc<Self> {
        Arc::new(Self { root, pending: Mutex::new(Pending::default()), changed: Arc::new(Notify::new()),
            worker: Mutex::new(None), io: AsyncMutex::new(()), stopping: AtomicBool::new(false) })
    }
    pub(super) fn track(self: &Arc<Self>, id: &str, capture: &Arc<StartupDiagnostics>) {
        {
            let mut pending = self.pending.lock().expect("console persistence");
            pending.captures.insert(id.to_owned(), capture.clone());
            pending.dirty.insert(id.to_owned());
        }
        self.start(); self.changed.notify_one();
    }
    pub(super) fn changed(&self, id: &str) {
        let mut pending = self.pending.lock().expect("console persistence");
        if pending.captures.contains_key(id) {
            pending.dirty.insert(id.to_owned()); self.changed.notify_one();
        }
    }
    fn start(self: &Arc<Self>) {
        let mut worker = self.worker.lock().expect("console persistence worker");
        if worker.is_some() || self.stopping.load(Ordering::Acquire) { return; }
        let Ok(runtime) = tokio::runtime::Handle::try_current() else { return; };
        let weak = Arc::downgrade(self); let changed = self.changed.clone();
        *worker = Some(runtime.spawn(async move {
            loop {
                changed.notified().await;
                let Some(store) = weak.upgrade() else { break; };
                if store.stopping.load(Ordering::Acquire) { store.flush().await; break; }
                // Multiple lines/attempt updates are saved together, not fsynced per line.
                tokio::time::sleep(Duration::from_millis(150)).await;
                store.save_batch().await;
                if store.stopping.load(Ordering::Acquire) { store.flush().await; break; }
            }
        }));
    }
    async fn save_batch(&self) {
        let _io = self.io.lock().await;
        let captures = {
            let mut pending = self.pending.lock().expect("console persistence");
            let dirty = std::mem::take(&mut pending.dirty);
            dirty.into_iter().filter_map(|id| pending.captures.get(&id).cloned().map(|capture| (id, capture))).collect::<Vec<_>>()
        };
        for (id, capture) in captures {
            let bytes = match serde_json::to_vec(&capture.saved_snapshot()) {
                Ok(bytes) if bytes.len() <= MAX_FILE_BYTES => bytes,
                _ => { capture.persistence_failed(SAVE_FAILURE); continue; },
            };
            let root = self.root.clone(); let saved_id = id.clone();
            let result = tokio::task::spawn_blocking(move || {
                let directory = server_directory(&root, &saved_id, false).map_err(|_| ())?.ok_or(())?;
                directory.write(FILE_NAME, &bytes).map_err(|_| ())
            }).await;
            if !matches!(result, Ok(Ok(()))) { capture.persistence_failed(SAVE_FAILURE); }
            // Retain a removed attempt until its final pending output is saved.
            // A frozen attempt has no future output; do not keep deleted logs alive.
            if capture.is_frozen() {
                let mut pending = self.pending.lock().expect("console persistence");
                if !pending.dirty.contains(&id) && pending.captures.get(&id).is_some_and(|current| Arc::ptr_eq(current, &capture)) {
                    pending.captures.remove(&id);
                }
            }
        }
    }
    pub(super) async fn load(&self, id: &str) -> Result<Option<Value>, ()> {
        let root = self.root.clone(); let id = id.to_owned();
        tokio::task::spawn_blocking(move || {
            let Some(directory) = server_directory(&root, &id, false).map_err(|_| ())? else { return Ok(None); };
            let Some(bytes) = directory.read(FILE_NAME, MAX_FILE_BYTES).map_err(|_| ())? else { return Ok(None); };
            serde_json::from_slice(&bytes).map(Some).map_err(|_| ())
        }).await.map_err(|_| ())?
    }
    pub(super) async fn flush(&self) {
        loop {
            self.save_batch().await;
            if self.pending.lock().expect("console persistence").dirty.is_empty() { break; }
        }
    }
    pub(super) async fn shutdown(&self) {
        self.stopping.store(true, Ordering::Release); self.changed.notify_one();
        let worker = self.worker.lock().expect("console persistence worker").take();
        if let Some(worker) = worker { let _ = worker.await; }
        self.flush().await;
    }
}
impl Drop for LogStore {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.get_mut().expect("console persistence worker").take() { worker.abort(); }
    }
}
