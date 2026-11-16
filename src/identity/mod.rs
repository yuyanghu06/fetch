//! Stable per-install TLS identity, protected separately from provider credentials.
use crate::credentials::{CredentialStore,CredentialError,SecretDocument};
use base64::{Engine,engine::general_purpose::STANDARD};
use serde::{Deserialize,Serialize};
use std::path::Path;
use zeroize::{Zeroizing,Zeroize,ZeroizeOnDrop};

pub struct InstallationIdentity { pub certificate_pem:String,pub private_key_der:Zeroizing<Vec<u8>> }
#[derive(Debug)]
pub enum IdentityError { ProtectedStorage(CredentialError), InvalidSavedIdentity, GenerationFailed }
impl std::fmt::Display for IdentityError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
        f.write_str("Fetch's protected HTTPS identity is unavailable. ")?;
        match self {
            Self::ProtectedStorage(CredentialError::KeyUnavailable)=>f.write_str("Its encryption key is unavailable in macOS Keychain. ")?,
            Self::ProtectedStorage(CredentialError::StorageUnavailable)=>f.write_str("Its encrypted file could not be read or written safely. ")?,
            Self::ProtectedStorage(CredentialError::IntegrityFailed)=>f.write_str("Its encrypted file could not be verified with the available key. ")?,
            Self::ProtectedStorage(error)=>write!(f,"{error} ")?,
            Self::InvalidSavedIdentity=>f.write_str("The decrypted identity has an invalid format. ")?,
            Self::GenerationFailed=>f.write_str("A new identity could not be generated. ")?,
        }
        f.write_str("Existing identity and provider records were preserved.")
    }
}
impl std::error::Error for IdentityError {}
#[derive(Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
struct SavedIdentity { version:u8,certificate_pem:String,private_key_der:String }
impl InstallationIdentity {
    pub fn load_or_create(_root:&Path,credentials:&CredentialStore)->Result<Self,IdentityError> {
        let store=credentials.application_store();
        if let Some(secret)=store.read("fetch-internal:tls-identity").map_err(IdentityError::ProtectedStorage)? {
            let saved:SavedIdentity=serde_json::from_slice(secret.expose()).map_err(|_|IdentityError::InvalidSavedIdentity)?;
            if saved.version!=1 || !saved.certificate_pem.starts_with("-----BEGIN CERTIFICATE-----") { return Err(IdentityError::InvalidSavedIdentity); }
            let key=STANDARD.decode(&saved.private_key_der).map_err(|_|IdentityError::InvalidSavedIdentity)?;
            if key.is_empty() { return Err(IdentityError::InvalidSavedIdentity); }
            return Ok(Self { certificate_pem:saved.certificate_pem.clone(),private_key_der:Zeroizing::new(key) });
        }
        let rcgen::CertifiedKey {cert,signing_key}=rcgen::generate_simple_self_signed(vec!["localhost".into()]).map_err(|_|IdentityError::GenerationFailed)?;
        let certificate_pem=cert.pem();
        let private_key_der=Zeroizing::new(signing_key.serialize_der());
        let saved=SavedIdentity { version:1,certificate_pem:certificate_pem.clone(),private_key_der:STANDARD.encode(private_key_der.as_slice()) };
        let encoded=Zeroizing::new(serde_json::to_vec(&saved).map_err(|_|IdentityError::GenerationFailed)?);
        store.save("fetch-internal:tls-identity",&SecretDocument::new(encoded.to_vec())).map_err(IdentityError::ProtectedStorage)?;
        Ok(Self {certificate_pem,private_key_der})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::KeyProvider;
    use std::{sync::Arc,fs};
    struct Keys(Option<CredentialError>);
    impl KeyProvider for Keys {
        fn key_for_server(&self,_:&str)->Result<Zeroizing<[u8;32]>,CredentialError>{
            match self.0{Some(error)=>Err(error),None=>Ok(Zeroizing::new([7;32]))}
        }
    }
    fn saved_file(root:&Path)->std::path::PathBuf{
        use sha2::{Digest,Sha256};
        root.join("application/servers").join(format!("{:x}",Sha256::digest(b"fetch-internal:tls-identity"))).join("credentials.enc")
    }
    #[test]
    fn blocked_keychain_startup_keeps_os_status_and_preserves_identity() {
        let directory=tempfile::tempdir().unwrap();let root=directory.path();
        let store=CredentialStore::new(root.to_owned(),Arc::new(Keys(None)));
        assert!(InstallationIdentity::load_or_create(root,&store).is_ok());
        let before=fs::read(saved_file(root)).unwrap();
        let blocked=CredentialStore::new(root.to_owned(),Arc::new(Keys(Some(CredentialError::KeychainUnavailable(-25308)))));
        let message=match InstallationIdentity::load_or_create(root,&blocked){Err(error)=>error.to_string(),Ok(_)=>panic!("blocked Keychain must fail startup")};
        assert!(message.contains("OSStatus -25308")&&message.contains("User interaction is not allowed"));
        assert!(fs::read(saved_file(root)).unwrap()==before,"startup must not replace an unreadable identity");
    }
    #[test]
    fn invalid_saved_identity_is_distinct_from_keychain_denial() {
        let directory=tempfile::tempdir().unwrap();let root=directory.path();
        let store=CredentialStore::new(root.to_owned(),Arc::new(Keys(None)));
        store.application_store().save("fetch-internal:tls-identity",&SecretDocument::new(b"fixture-invalid-identity".to_vec())).unwrap();
        let before=fs::read(saved_file(root)).unwrap();
        let message=match InstallationIdentity::load_or_create(root,&store){Err(error)=>error.to_string(),Ok(_)=>panic!("invalid saved identity must fail startup")};
        assert!(message.contains("decrypted identity has an invalid format")&&!message.contains("fixture-invalid-identity"));
        assert!(fs::read(saved_file(root)).unwrap()==before);
    }
}
