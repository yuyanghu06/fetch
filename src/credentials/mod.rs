//! Encrypted server-local bytes. Persistence is not provider authentication.
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
}
impl std::fmt::Display for CredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fetch cannot safely access this server's encrypted credentials.")
    }
}
impl std::error::Error for CredentialError {}

/// A real macOS implementation must supply this through the selected Keychain boundary.
/// There is intentionally no plaintext-file or environment-key implementation.
pub trait KeyProvider: Send + Sync {
    fn key_for_server(&self, server_id: &str) -> Result<Zeroizing<[u8; 32]>, CredentialError>;
    /// Existing encrypted files must not cause replacement-key creation.
    fn existing_key_for_server(&self, server_id:&str)->Result<Zeroizing<[u8;32]>,CredentialError> { self.key_for_server(server_id) }
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
}
impl CredentialStore {
    pub fn new(root: PathBuf, keys: Arc<dyn KeyProvider>) -> Self {
        Self {
            root,
            keys,
            writes: Mutex::new(()),
        }
    }
    /// App tokens and TLS identity use a separate encrypted application tree.
    pub fn application_store(&self)->Self { Self::new(self.root.join("application"),self.keys.clone()) }
    pub fn read(&self, server_id: &str) -> Result<Option<SecretDocument>, CredentialError> {
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
        if value.expose().len() > MAX_SECRET_BYTES {
            return Err(CredentialError::TooLarge);
        }
        let _guard = self
            .writes
            .lock()
            .map_err(|_| CredentialError::StorageUnavailable)?;
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
    fn encrypted_roundtrip_random_nonce_and_redacted_debug() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("state");
        let s = CredentialStore::new(root.clone(), Arc::new(FixtureKeys));
        let secret = SecretDocument::new(b"private-fixture-value".to_vec());
        s.save("a", &secret).unwrap();
        let file = root
            .join("servers")
            .join(server_directory_name("a"))
            .join("credentials.enc");
        let a = std::fs::read(&file).unwrap();
        assert!(
            !a.windows(secret.expose().len())
                .any(|w| w == secret.expose())
        );
        s.save("a", &secret).unwrap();
        assert_ne!(a, std::fs::read(file).unwrap());
        assert_eq!(s.read("a").unwrap().unwrap().expose(), secret.expose());
        assert_eq!(format!("{secret:?}"), "SecretDocument([REDACTED])");
    }
    #[test]
    fn cross_server_copy_tampering_and_missing_keys_preserve_data() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("state");
        let s = CredentialStore::new(root.clone(), Arc::new(FixtureKeys));
        let secret = SecretDocument::new(b"fixture".to_vec());
        s.save("a", &secret).unwrap();
        s.save("b", &secret).unwrap();
        let a = root
            .join("servers")
            .join(server_directory_name("a"))
            .join("credentials.enc");
        let b = root
            .join("servers")
            .join(server_directory_name("b"))
            .join("credentials.enc");
        let bytes = std::fs::read(&a).unwrap();
        std::fs::write(&b, &bytes).unwrap();
        assert!(matches!(s.read("b"), Err(CredentialError::IntegrityFailed)));
        assert!(s.save("b", &secret).is_err());
        assert_eq!(std::fs::read(&b).unwrap(), bytes);
        let missing = CredentialStore::new(root, Arc::new(Missing));
        assert!(matches!(
            missing.read("a"),
            Err(CredentialError::KeyUnavailable)
        ));
        assert!(missing.save("a", &secret).is_err());
        assert_eq!(std::fs::read(a).unwrap(), bytes);
    }
}


/// Each item is a random file key held by the Mac login Keychain. There is no plaintext fallback.
pub struct MacKeychainKeys { service:String, creation:Mutex<()> }
impl MacKeychainKeys {
    pub fn new(namespace:String)->Self { Self { service:format!("com.fetch.local.keys.{namespace}"),creation:Mutex::new(()) } }
    fn read_key(&self,server_id:&str)->Result<Option<Zeroizing<[u8;32]>>,CredentialError> {
        #[cfg(target_os="macos")]
        {
            use std::ffi::CString;
            use sha2::{Digest,Sha256};
            let service=CString::new(self.service.as_str()).map_err(|_|CredentialError::KeyUnavailable)?;
            let account=CString::new(format!("{:x}",Sha256::digest(server_id.as_bytes()))).map_err(|_|CredentialError::KeyUnavailable)?;
            let mut key=Zeroizing::new([0u8;32]);
            // SAFETY: all pointers refer to valid buffers for the synchronous native call.
            let status=unsafe { fetch_keychain_read(service.as_ptr(),account.as_ptr(),key.as_mut_ptr(),32) };
            match status { 0=>Ok(Some(key)),-25300=>Ok(None),_=>Err(CredentialError::KeyUnavailable) }
        }
        #[cfg(not(target_os="macos"))]
        { let _=server_id;Err(CredentialError::KeyUnavailable) }
    }
}
impl KeyProvider for MacKeychainKeys {
    fn existing_key_for_server(&self,server_id:&str)->Result<Zeroizing<[u8;32]>,CredentialError> { self.read_key(server_id)?.ok_or(CredentialError::KeyUnavailable) }
    fn key_for_server(&self,server_id:&str)->Result<Zeroizing<[u8;32]>,CredentialError> {
        let _lock=self.creation.lock().map_err(|_|CredentialError::KeyUnavailable)?;
        if let Some(key)=self.read_key(server_id)? { return Ok(key); }
        #[cfg(target_os="macos")]
        {
            use std::ffi::CString;
            use sha2::{Digest,Sha256};
            let service=CString::new(self.service.as_str()).map_err(|_|CredentialError::KeyUnavailable)?;
            let account=CString::new(format!("{:x}",Sha256::digest(server_id.as_bytes()))).map_err(|_|CredentialError::KeyUnavailable)?;
            let mut key=Zeroizing::new([0u8;32]);OsRng.fill_bytes(key.as_mut());
            // SAFETY: all pointers remain valid until the native call completes.
            let status=unsafe { fetch_keychain_create(service.as_ptr(),account.as_ptr(),key.as_ptr(),32) };
            if status!=0 && status!= -25299 { return Err(CredentialError::KeyUnavailable); }
            let stored=self.existing_key_for_server(server_id)?;
            if status==0 && *stored!=*key { return Err(CredentialError::IntegrityFailed); }
            Ok(stored)
        }
        #[cfg(not(target_os="macos"))]
        { Err(CredentialError::KeyUnavailable) }
    }
}
#[cfg(target_os="macos")]
unsafe extern "C" {
    fn fetch_keychain_read(service:*const std::ffi::c_char,account:*const std::ffi::c_char,output:*mut u8,length:usize)->i32;
    fn fetch_keychain_create(service:*const std::ffi::c_char,account:*const std::ffi::c_char,input:*const u8,length:usize)->i32;
}
