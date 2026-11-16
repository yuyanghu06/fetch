//! Authority is supplied by the trusted backend, never by request JSON or a tool name.
use axum::http::HeaderMap;
use crate::projects::Project;
use std::{collections::{BTreeMap, BTreeSet},path::Path};

#[derive(Clone)]
pub struct Caller {
    owner: String,
    scopes: BTreeSet<String>,
    // None means the human-approved current/future installed-server grant.
    targets: Option<BTreeMap<String, Option<BTreeSet<String>>>>,
    native_settings: bool,
    resource: Option<String>,
    project: Option<Project>,
}
impl Caller {
    /// Injection boundary for trusted verifiers and the pre-existing fixture API.
    pub fn from_verified_scopes(scopes: impl IntoIterator<Item=String>)->Self {
        Self { owner:"trusted-injected-authority".into(), scopes:scopes.into_iter().collect(), targets:None, native_settings:false,resource:None,project:None }
    }
    pub(crate) fn from_grant(owner:String,scopes:impl IntoIterator<Item=String>)->Self {
        Self { owner,scopes:scopes.into_iter().collect(),targets:None,native_settings:false,resource:None,project:None }
    }
    pub(crate) fn from_native_settings(owner:String)->Self {
        Self { owner,scopes:["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:manage"].into_iter().map(str::to_owned).collect(),targets:None,native_settings:true,resource:None,project:None }
    }
    pub fn owner_id(&self)->&str { &self.owner }
    pub fn has_scope(&self,scope:&str)->bool { self.scopes.contains(scope) }
    pub fn can_manage(&self)->bool { self.has_scope("fetch:manage") }
    pub fn is_native_settings(&self)->bool { self.native_settings }
    pub fn project_id(&self)->Option<&str> { self.project.as_ref().map(|project|project.id.as_str()) }
    pub fn project_folder(&self)->Option<&Path> { self.project.as_ref().map(|project|project.folder_path.as_path()) }
    pub fn resource(&self)->Option<&str> { self.resource.as_deref() }
    /// Trusted verifier/fixture boundary. Request JSON never calls this method.
    pub fn bind_project(mut self,resource:String,project:Option<Project>)->Self {
        self.resource=Some(resource);self.project=project;self
    }
    /// Intersect a verified caller's existing server/item limits with membership.
    /// Explicit native UI filters use the same operation; the native flag stays.
    pub fn restrict_visibility(mut self,ids:impl IntoIterator<Item=String>)->Self {
        let visible:BTreeSet<_>=ids.into_iter().collect();
        match self.targets.as_mut() {
            Some(targets)=>targets.retain(|id,_|visible.contains(id)),
            None=>self.targets=Some(visible.into_iter().map(|id|(id,None)).collect()),
        }
        self
    }
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
