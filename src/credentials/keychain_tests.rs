//! Cache behavior against a fake OS store. These tests never call the Keychain.
use super::{CredentialError, KeyProvider, MacKeychainKeys, keychain::{Backend, CreateResult}};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, sync::{Arc, Barrier, Condvar, Mutex, mpsc}, thread, time::Duration};
use zeroize::Zeroizing;

type Item = (String, String);
#[derive(Default)]
struct State {
    keys: BTreeMap<Item, [u8; 32]>,
    reads: Vec<Item>,
    creates: usize,
    deny_reads: usize,
    deny_create: bool,
    deny_readback: bool,
    mismatch_create: bool,
    duplicate_create: bool,
}
struct ReadGate { entered: mpsc::Sender<()>, released: Arc<(Mutex<bool>, Condvar)> }
#[derive(Default)]
struct Fake { state: Mutex<State>, next_read_gate: Mutex<Option<ReadGate>> }
fn item(namespace: &str, id: &str) -> Item {
    (format!("com.fetch.local.keys.{namespace}"), format!("{:x}", Sha256::digest(id.as_bytes())))
}
impl Fake {
    fn seed(&self, namespace: &str, id: &str, key: [u8; 32]) {
        self.state.lock().unwrap().keys.insert(item(namespace, id), key);
    }
    fn reads(&self) -> usize { self.state.lock().unwrap().reads.len() }
    fn provider(self: &Arc<Self>, namespace: &str) -> Arc<MacKeychainKeys> {
        Arc::new(MacKeychainKeys::with_backend(namespace.to_owned(), self.clone()))
    }
}
impl Backend for Fake {
    fn read(&self, service: &str, account: &str) -> Result<Option<Zeroizing<[u8; 32]>>, CredentialError> {
        let result = {
            let mut state = self.state.lock().unwrap();
            let item = (service.to_owned(), account.to_owned());
            state.reads.push(item.clone());
            if state.deny_reads > 0 {
                state.deny_reads -= 1;
                Err(CredentialError::KeyUnavailable)
            } else { Ok(state.keys.get(&item).copied().map(Zeroizing::new)) }
        };
        let gate = self.next_read_gate.lock().unwrap().take();
        if let Some(gate) = gate {
            gate.entered.send(()).unwrap();
            let (released, wake) = &*gate.released;
            let mut ready = released.lock().unwrap();
            while !*ready { ready = wake.wait(ready).unwrap(); }
        }
        result
    }
    fn create(&self, service: &str, account: &str, key: &[u8; 32]) -> Result<CreateResult, CredentialError> {
        let mut state = self.state.lock().unwrap();
        state.creates += 1;
        if state.deny_create { return Err(CredentialError::KeyUnavailable); }
        let mut stored = *key;
        if state.mismatch_create { stored[0] ^= 1; }
        if state.duplicate_create { stored = [93; 32]; }
        state.keys.insert((service.to_owned(), account.to_owned()), stored);
        if state.deny_readback { state.deny_reads += 1; }
        Ok(if state.duplicate_create { CreateResult::AlreadyExists } else { CreateResult::Created })
    }

}

#[test]
fn repeated_and_concurrent_lookup_uses_one_backend_read() {
    let backend = Arc::new(Fake::default()); backend.seed("test", "fetch-internal:tls-identity", [7; 32]);
    let provider = backend.provider("test"); let start = Arc::new(Barrier::new(9));
    let threads: Vec<_> = (0..8).map(|_| {
        let provider = provider.clone(); let start = start.clone();
        thread::spawn(move || { start.wait(); assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [7; 32]); })
    }).collect();
    start.wait(); for thread in threads { thread.join().unwrap(); }
    assert_eq!(*provider.key_for_server("fetch-internal:tls-identity").unwrap(), [7; 32]);
    assert_eq!(backend.reads(), 1); assert_eq!(backend.state.lock().unwrap().creates, 0);
}

#[test]
fn separate_application_purposes_and_namespaces_keep_distinct_keys() {
    let backend = Arc::new(Fake::default());
    backend.seed("first", "fetch-internal:tls-identity", [1; 32]); backend.seed("first", "fetch-internal:app-authorization", [2; 32]); backend.seed("second", "fetch-internal:tls-identity", [3; 32]);
    let first = backend.provider("first"); let second = backend.provider("second");
    for _ in 0..2 {
        assert_eq!(*first.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [1; 32]);
        assert_eq!(*first.existing_key_for_server("fetch-internal:app-authorization").unwrap(), [2; 32]);
        assert_eq!(*second.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [3; 32]);
    }
    assert_eq!(backend.reads(), 3);
}

#[test]
fn missing_and_denied_reads_are_retryable_and_never_create_replacement_keys() {
    let backend = Arc::new(Fake::default()); let provider = backend.provider("test");
    backend.state.lock().unwrap().deny_reads = 1;
    assert_eq!(provider.existing_key_for_server("fetch-internal:tls-identity"), Err(CredentialError::KeyUnavailable));
    assert_eq!(provider.existing_key_for_server("fetch-internal:tls-identity"), Err(CredentialError::KeyUnavailable));
    backend.seed("test", "fetch-internal:tls-identity", [12; 32]);
    assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [12; 32]);
    assert_eq!(backend.reads(), 3); assert_eq!(backend.state.lock().unwrap().creates, 0);
}

#[test]
fn new_key_is_read_back_from_backend_before_reuse() {
    let backend = Arc::new(Fake::default()); let provider = backend.provider("test");
    let key = provider.key_for_server("fetch-internal:tls-identity").unwrap();
    let stored = backend.state.lock().unwrap().keys[&item("test", "fetch-internal:tls-identity")];
    assert_eq!(*key, stored); assert_eq!(backend.reads(), 2);
    assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), stored);
    assert_eq!(backend.reads(), 2); assert_eq!(backend.state.lock().unwrap().creates, 1);
}

#[test]
fn failed_creation_readback_or_comparison_does_not_cache_a_key() {
    for mode in 0..3 {
        let backend = Arc::new(Fake::default()); let provider = backend.provider("test");
        { let mut state = backend.state.lock().unwrap();
          state.deny_create = mode == 0; state.deny_readback = mode == 1; state.mismatch_create = mode == 2; }
        let expected = if mode == 2 { CredentialError::IntegrityFailed } else { CredentialError::KeyUnavailable };
        assert_eq!(provider.key_for_server("fetch-internal:tls-identity"), Err(expected));
        let reads = backend.reads(); backend.seed("test", "fetch-internal:tls-identity", [45; 32]);
        assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [45; 32]);
        assert_eq!(backend.reads(), reads + 1);
    }
}

#[test]
fn duplicate_creation_uses_the_key_that_won_in_the_backend() {
    let backend = Arc::new(Fake::default()); backend.state.lock().unwrap().duplicate_create = true;
    let provider = backend.provider("test");
    assert_eq!(*provider.key_for_server("fetch-internal:tls-identity").unwrap(), [93; 32]); assert_eq!(backend.reads(), 2);
    assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [93; 32]); assert_eq!(backend.reads(), 2);
}

#[test]
fn cache_clear_closes_retained_handles_and_blocks_late_read_refill() {
    let backend = Arc::new(Fake::default()); backend.seed("test", "fetch-internal:tls-identity", [4; 32]); backend.seed("test", "fetch-internal:app-authorization", [5; 32]);
    let provider = backend.provider("test"); provider.existing_key_for_server("fetch-internal:tls-identity").unwrap();
    let retained: Arc<dyn KeyProvider> = provider.clone();
    let (entered_tx, entered_rx) = mpsc::channel(); let released = Arc::new((Mutex::new(false), Condvar::new()));
    *backend.next_read_gate.lock().unwrap() = Some(ReadGate { entered: entered_tx, released: released.clone() });
    let pending_provider = provider.clone();
    let pending = thread::spawn(move || pending_provider.existing_key_for_server("fetch-internal:app-authorization"));
    entered_rx.recv_timeout(Duration::from_secs(2)).unwrap();
    let (closed_tx, closed_rx) = mpsc::channel(); let closing = provider.clone();
    let close = thread::spawn(move || { closing.clear_cached_keys(); closed_tx.send(()).unwrap(); });
    // Close may wait for the active OS read, but its closed flag must already
    // prevent the read from putting its result back into the live cache.
    let closed_before_release = closed_rx.recv_timeout(Duration::from_millis(100)).is_ok();
    { let (ready, wake) = &*released; *ready.lock().unwrap() = true; wake.notify_all(); }
    let result = pending.join().unwrap(); close.join().unwrap();
    if !closed_before_release { closed_rx.recv_timeout(Duration::from_secs(2)).unwrap(); }
    assert_eq!(result, Err(CredentialError::KeyUnavailable));
    let reads = backend.reads();
    assert_eq!(retained.existing_key_for_server("fetch-internal:tls-identity"), Err(CredentialError::KeyUnavailable));
    assert_eq!(retained.key_for_server("new"), Err(CredentialError::KeyUnavailable));
    retained.clear_cached_keys(); assert_eq!(backend.reads(), reads);
    assert_eq!(backend.state.lock().unwrap().creates, 0);
}

#[tokio::test]
async fn application_shutdown_closes_keys_even_while_host_and_store_are_retained() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("state");
    let backend = Arc::new(Fake::default()); backend.seed("host", "fetch-internal:tls-identity", [31; 32]);
    let provider = backend.provider("host");
    let credentials = Arc::new(super::CredentialStore::new(root.clone(), provider.clone()));
    let host = crate::application::ApplicationHost::fixture(root, credentials.clone()).await.unwrap();
    assert_eq!(*provider.existing_key_for_server("fetch-internal:tls-identity").unwrap(), [31; 32]);
    let reads = backend.reads();
    host.shutdown().await; host.shutdown().await;
    assert!(host.is_stopped());
    assert_eq!(provider.existing_key_for_server("fetch-internal:tls-identity"), Err(CredentialError::KeyUnavailable));
    assert_eq!(provider.key_for_server("new"), Err(CredentialError::KeyUnavailable));
    credentials.clear_cached_keys();
    assert_eq!(backend.reads(), reads);
}
