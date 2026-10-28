//! Authority is supplied by the trusted backend, never by request JSON or a tool name.
use axum::http::HeaderMap;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct Caller {
    owner: String,
    scopes: BTreeSet<String>,
    // None means the human-approved current/future installed-server grant.
    targets: Option<BTreeMap<String, Option<BTreeSet<String>>>>,
    native_settings: bool,
}
impl Caller {
    /// Injection boundary for trusted verifiers and the pre-existing fixture API.
    pub fn from_verified_scopes(scopes: impl IntoIterator<Item=String>)->Self {
        Self { owner:"trusted-injected-authority".into(), scopes:scopes.into_iter().collect(), targets:None, native_settings:false }
    }
    pub(crate) fn from_grant(owner:String,scopes:impl IntoIterator<Item=String>)->Self {
        Self { owner,scopes:scopes.into_iter().collect(),targets:None,native_settings:false }
    }
    pub(crate) fn from_native_settings(owner:String)->Self {
        Self { owner,scopes:["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:manage"].into_iter().map(str::to_owned).collect(),targets:None,native_settings:true }
    }
    pub fn owner_id(&self)->&str { &self.owner }
    pub fn has_scope(&self,scope:&str)->bool { self.scopes.contains(scope) }
    pub fn can_manage(&self)->bool { self.has_scope("fetch:manage") }
    pub fn is_native_settings(&self)->bool { self.native_settings }
    pub fn permits(&self,scope:&str,server:&str,item:Option<&str>)->bool {
        if !self.has_scope(scope) { return false; }
        match &self.targets {
            None=>true,
            Some(servers)=>match servers.get(server) { Some(None)=>true,Some(Some(items))=>item.is_some_and(|i|items.contains(i)),None=>false }
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum AuthorizationError { Unauthenticated, Unavailable }
pub trait Authorizer:Send+Sync { fn authorize(&self,headers:&HeaderMap)->Result<Caller,AuthorizationError>; }
/// Developer fixtures retain a deny-by-default mode without a bypass token.
pub struct PendingNativeAuthority;
impl Authorizer for PendingNativeAuthority { fn authorize(&self,_:&HeaderMap)->Result<Caller,AuthorizationError> { Err(AuthorizationError::Unauthenticated) } }
