//! Authority is supplied by the trusted backend, never by request JSON.
use axum::http::HeaderMap;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Caller {
    scopes: BTreeSet<String>,
}

impl Caller {
    /// Called by a trusted token verifier, after validating a saved grant.
    pub fn from_verified_scopes(scopes: impl IntoIterator<Item = String>) -> Self {
        Self {
            scopes: scopes.into_iter().collect(),
        }
    }

    pub fn can_manage(&self) -> bool {
        self.scopes.contains("fetch:manage")
    }
}

pub enum AuthorizationError {
    Unauthenticated,
    Unavailable,
}

pub trait Authorizer: Send + Sync {
    fn authorize(&self, headers: &HeaderMap) -> Result<Caller, AuthorizationError>;
}

/// No bypass token: real token validation/native approval is a later increment.
pub struct PendingNativeAuthority;

impl Authorizer for PendingNativeAuthority {
    fn authorize(&self, _: &HeaderMap) -> Result<Caller, AuthorizationError> {
        Err(AuthorizationError::Unauthenticated)
    }
}
