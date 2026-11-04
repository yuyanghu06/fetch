//! Protected Fetch application identity and client grants only.
//! Native provider files and Keychain entries are never read, migrated or removed here.
mod keychain;
pub use keychain::MacKeychainKeys;
#[cfg(test)]
mod keychain_tests;
use crate::storage::server_directory;
use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use rand::{RngCore, rngs::OsRng};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use zeroize::Zeroizing;

const MAGIC: &[u8] = b"FETCHSEC\x01";
const MAX_SECRET_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialError {
    KeyUnavailable,
    IntegrityFailed,
    StorageUnavailable,
    TooLarge,
    InvalidDocument,
}
impl std::fmt::Display for CredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fetch cannot safely access its protected application identity or app authorizations.")
    }
}
impl std::error::Error for CredentialError {}

/// A real macOS implementation must supply this through the selected Keychain boundary.
/// There is intentionally no plaintext-file or environment-key implementation.
pub trait KeyProvider: Send + Sync {
    fn key_for_server(&self, server_id: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError>;
    /// Existing encrypted files must not cause replacement-key creation.
    fn existing_key_for_server(&self, server_id:&str)->Result<Zeroizing<[u8;32]>,CredentialError> { self.key_for_server(server_id) }
    /// End this provider's in-memory key session when Fetch quits.
    fn clear_cached_keys(&self) {}
}

pub struct SecretDocument(Zeroizing<Vec<u8>>);
impl SecretDocument {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(Zeroizing::new(bytes))
    }
    pub fn expose(&self) -> &[u8] {
        &self.0
    }
}
impl std::fmt::Debug for SecretDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SecretDocument([REDACTED])")
    }
}

pub struct CredentialStore {
    root: PathBuf,
    keys: Arc<dyn KeyProvider>,
    writes: Mutex<()>,
    application:bool,
}
impl CredentialStore {
    pub fn new(root: PathBuf, keys: Arc<dyn KeyProvider>) -> Self {
        Self {
            root,
            keys,
            writes: Mutex::new(()),
            application:false,
        }
    }
    pub(crate) fn clear_cached_keys(&self) { self.keys.clear_cached_keys(); }
    /// App tokens and TLS identity use a separate encrypted application tree.
    pub fn application_store(&self)->Self { let mut store=Self::new(self.root.join("application"),self.keys.clone());store.application=true;store }
    pub fn read(&self, server_id: &str) -> Result<Option<SecretDocument>, CredentialError> {
        self.application_id(server_id)?;
        let Some(dir) = server_directory(&self.root, server_id, false)
            .map_err(|_| CredentialError::StorageUnavailable)?
        else {
            return Ok(None);
        };
        let Some(bytes) = dir
            .read("credentials.enc", MAX_SECRET_BYTES + MAGIC.len() + 28)
            .map_err(|_| CredentialError::StorageUnavailable)?
        else {
            return Ok(None);
        };
        let key = self.keys.existing_key_for_server(server_id)?;
        decrypt(server_id, &key, &bytes).map(Some)
    }
    pub fn save(&self, server_id: &str, value: &SecretDocument) -> Result<(), CredentialError> {
        let _guard = self
            .writes
            .lock()
            .map_err(|_| CredentialError::StorageUnavailable)?;
        self.save_locked(server_id, value)
    }
    fn application_id(&self,id:&str)->Result<(),CredentialError>{
        if self.application && matches!(id,"fetch-internal:tls-identity"|"fetch-internal:app-authorization") {Ok(())} else {Err(CredentialError::StorageUnavailable)}
    }
    fn save_locked(&self, server_id: &str, value: &SecretDocument) -> Result<(), CredentialError> {
        self.application_id(server_id)?;
        if value.expose().len() > MAX_SECRET_BYTES {
            return Err(CredentialError::TooLarge);
        }
        // Refuse to overwrite an existing document that cannot be verified with its current key.
        let existing = match server_directory(&self.root, server_id, false).map_err(|_|CredentialError::StorageUnavailable)? {
            Some(dir)=>dir.read("credentials.enc",MAX_SECRET_BYTES+MAGIC.len()+28).map_err(|_|CredentialError::StorageUnavailable)?,None=>None
        };
        let key=if existing.is_some() { self.keys.existing_key_for_server(server_id)? } else { self.keys.key_for_server(server_id)? };
        if let Some(existing)=existing { let _=decrypt(server_id,&key,&existing)?; }
        let cipher =
            Aes256Gcm::new_from_slice(key.as_ref()).map_err(|_| CredentialError::KeyUnavailable)?;
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let aad = aad(server_id);
        let encrypted = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: value.expose(),
                    aad: &aad,
                },
            )
            .map_err(|_| CredentialError::IntegrityFailed)?;
        let mut bytes = Vec::with_capacity(MAGIC.len() + 12 + encrypted.len());
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&nonce);
        bytes.extend_from_slice(&encrypted);
        let dir = server_directory(&self.root, server_id, true)
            .map_err(|_| CredentialError::StorageUnavailable)?
            .ok_or(CredentialError::StorageUnavailable)?;
        dir.write("credentials.enc", &bytes)
            .map_err(|_| CredentialError::StorageUnavailable)?;
        let verified = self
            .read(server_id)?
            .ok_or(CredentialError::StorageUnavailable)?;
        if verified.expose() != value.expose() {
            return Err(CredentialError::IntegrityFailed);
        }
        Ok(())
    }
}
fn aad(server_id: &str) -> Vec<u8> {
    let mut a = b"Fetch credentials v1\0".to_vec();
    a.extend_from_slice(server_id.as_bytes());
    a
}
fn decrypt(id: &str, key: &[u8; 32], bytes: &[u8]) -> Result<SecretDocument, CredentialError> {
    if bytes.len() < MAGIC.len() + 28 || !bytes.starts_with(MAGIC) {
        return Err(CredentialError::IntegrityFailed);
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CredentialError::KeyUnavailable)?;
    let nonce = Nonce::from_slice(&bytes[MAGIC.len()..MAGIC.len() + 12]);
    let aad = aad(id);
    let value = cipher
        .decrypt(
            nonce,
            Payload {
                msg: &bytes[MAGIC.len() + 12..],
                aad: &aad,
            },
        )
        .map_err(|_| CredentialError::IntegrityFailed)?;
    Ok(SecretDocument::new(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::server_directory_name;
    struct FixtureKeys;
    impl KeyProvider for FixtureKeys {
        fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
            Ok(Zeroizing::new([7; 32]))
        }
    }
    struct Missing;
    impl KeyProvider for Missing {
        fn key_for_server(&self, _: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError> {
            Err(CredentialError::KeyUnavailable)
        }
    }
    #[test]
    fn provider_records_are_never_read_written_or_unlocked(){
        struct NoKeys;
        impl KeyProvider for NoKeys{fn key_for_server(&self,_:&str)->Result<Zeroizing<[u8;32]>,CredentialError>{panic!("provider key access must not occur")}}
        let temp=tempfile::tempdir().unwrap();let root=temp.path().join("state");
        let legacy=root.join("servers").join(server_directory_name("native-provider"));
        std::fs::create_dir_all(&legacy).unwrap();let file=legacy.join("credentials.enc");std::fs::write(&file,b"opaque-legacy-canary").unwrap();
        let store=CredentialStore::new(root,Arc::new(NoKeys));
        assert!(matches!(store.read("native-provider"),Err(CredentialError::StorageUnavailable)));
        assert!(store.save("native-provider",&SecretDocument::new(b"must-not-save".to_vec())).is_err());
        assert!(store.application_store().read("native-provider").is_err());
        assert_eq!(std::fs::read(file).unwrap(),b"opaque-legacy-canary");
    }
    #[test]
    fn encrypted_roundtrip_random_nonce_and_redacted_debug() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("state");
        let s = CredentialStore::new(root.clone(), Arc::new(FixtureKeys)).application_store();
        let secret = SecretDocument::new(b"private-fixture-value".to_vec());
        s.save("fetch-internal:tls-identity", &secret).unwrap();
        let file = root
            .join("application/servers")
            .join(server_directory_name("fetch-internal:tls-identity"))
            .join("credentials.enc");
        let a = std::fs::read(&file).unwrap();
        assert!(
            !a.windows(secret.expose().len())
                .any(|w| w == secret.expose())
        );
        s.save("fetch-internal:tls-identity", &secret).unwrap();
        assert_ne!(a, std::fs::read(file).unwrap());
        assert_eq!(s.read("fetch-internal:tls-identity").unwrap().unwrap().expose(), secret.expose());
        assert_eq!(format!("{secret:?}"), "SecretDocument([REDACTED])");
    }
    #[test]
    fn cross_server_copy_tampering_and_missing_keys_preserve_data() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("state");
        let s = CredentialStore::new(root.clone(), Arc::new(FixtureKeys)).application_store();
        let secret = SecretDocument::new(b"fixture".to_vec());
        s.save("fetch-internal:tls-identity", &secret).unwrap();
        s.save("fetch-internal:app-authorization", &secret).unwrap();
        let a = root
            .join("application/servers")
            .join(server_directory_name("fetch-internal:tls-identity"))
            .join("credentials.enc");
        let b = root
            .join("application/servers")
            .join(server_directory_name("fetch-internal:app-authorization"))
            .join("credentials.enc");
        let bytes = std::fs::read(&a).unwrap();
        std::fs::write(&b, &bytes).unwrap();
        assert!(matches!(s.read("fetch-internal:app-authorization"), Err(CredentialError::IntegrityFailed)));
        assert!(s.save("fetch-internal:app-authorization", &secret).is_err());
        assert_eq!(std::fs::read(&b).unwrap(), bytes);
        let missing = CredentialStore::new(root, Arc::new(Missing)).application_store();
        assert!(matches!(
            missing.read("fetch-internal:tls-identity"),
            Err(CredentialError::KeyUnavailable)
        ));
        assert!(missing.save("fetch-internal:tls-identity", &secret).is_err());
        assert_eq!(std::fs::read(a).unwrap(), bytes);
    }
}
