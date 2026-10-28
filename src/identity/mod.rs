//! Stable per-install TLS identity, protected separately from provider credentials.
use crate::credentials::{CredentialStore,SecretDocument};
use base64::{Engine,engine::general_purpose::STANDARD};
use serde::{Deserialize,Serialize};
use std::path::Path;
use zeroize::{Zeroizing,Zeroize,ZeroizeOnDrop};

pub struct InstallationIdentity { pub certificate_pem:String,pub private_key_der:Zeroizing<Vec<u8>> }
#[derive(Debug)]
pub struct IdentityError;
impl std::fmt::Display for IdentityError { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("Fetch's protected HTTPS identity is unavailable. Existing identity and provider records were preserved.") } }
impl std::error::Error for IdentityError {}
#[derive(Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
struct SavedIdentity { version:u8,certificate_pem:String,private_key_der:String }
impl InstallationIdentity {
    pub fn load_or_create(_root:&Path,credentials:&CredentialStore)->Result<Self,IdentityError> {
        let store=credentials.application_store();
        if let Some(secret)=store.read("fetch-internal:tls-identity").map_err(|_|IdentityError)? {
            let saved:SavedIdentity=serde_json::from_slice(secret.expose()).map_err(|_|IdentityError)?;
            if saved.version!=1 || !saved.certificate_pem.starts_with("-----BEGIN CERTIFICATE-----") { return Err(IdentityError); }
            let key=STANDARD.decode(&saved.private_key_der).map_err(|_|IdentityError)?;
            if key.is_empty() { return Err(IdentityError); }
            return Ok(Self { certificate_pem:saved.certificate_pem.clone(),private_key_der:Zeroizing::new(key) });
        }
        let rcgen::CertifiedKey {cert,signing_key}=rcgen::generate_simple_self_signed(vec!["localhost".into()]).map_err(|_|IdentityError)?;
        let certificate_pem=cert.pem();
        let private_key_der=Zeroizing::new(signing_key.serialize_der());
        let saved=SavedIdentity { version:1,certificate_pem:certificate_pem.clone(),private_key_der:STANDARD.encode(private_key_der.as_slice()) };
        let encoded=Zeroizing::new(serde_json::to_vec(&saved).map_err(|_|IdentityError)?);
        store.save("fetch-internal:tls-identity",&SecretDocument::new(encoded.to_vec())).map_err(|_|IdentityError)?;
        Ok(Self {certificate_pem,private_key_der})
    }
}
