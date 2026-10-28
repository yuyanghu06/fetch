//! Account-free local OAuth. Provider tokens are never accepted as Fetch authority.
use crate::{authority::{AuthorizationError,Authorizer,Caller},credentials::{CredentialStore,SecretDocument},native_auth::{NativeAuthError,NativeAuthority}};
use axum::{Router,body::to_bytes,extract::{Request,State},http::{HeaderMap,HeaderValue,StatusCode,header},response::{Html,IntoResponse,Response},routing::{get,post},Json};
use base64::{Engine,engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{RngCore,rngs::OsRng};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},path::PathBuf,sync::{Arc,Mutex,atomic::{AtomicBool,Ordering}},time::{SystemTime,UNIX_EPOCH}};
use url::Url;
use zeroize::Zeroizing;

const SCOPES:[&str;4]=["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:manage"];
const DEFAULT_SCOPES:[&str;3]=["fetch:catalog","fetch:tools:call","fetch:content:read"];
const BODY_LIMIT:usize=16*1024;
const APPROVAL_SECONDS:u64=600;
const CODE_SECONDS:u64=60;
const ACCESS_SECONDS:u64=3600;
const REFRESH_SECONDS:u64=90*24*3600;
const APPLICATION_ID:&str="fetch-internal:app-authorization";

#[derive(Debug)]
pub struct OAuthError;
impl std::fmt::Display for OAuthError { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("Fetch authorization storage is unavailable. Existing connection records were preserved.") } }
impl std::error::Error for OAuthError {}
#[derive(Clone,Serialize,Deserialize)]
struct Client { id:String,name:String,redirect_uris:Vec<String>,grant_types:BTreeSet<String>,created:u64 }
#[derive(Clone,Serialize,Deserialize)]
struct AccessToken { hash:String,expires:u64,scopes:BTreeSet<String> }
#[derive(Clone,Serialize,Deserialize)]
struct RefreshToken { hash:String,expires:u64,spent:bool }
#[derive(Clone,Serialize,Deserialize)]
struct Family { id:String,client:String,resource:String,scopes:BTreeSet<String>,access:Vec<AccessToken>,refresh:Vec<RefreshToken>,revoked:bool,last_refresh:u64 }
#[derive(Clone,Serialize,Deserialize)]
struct Persistent { version:u8,issuer:String,clients:BTreeMap<String,Client>,families:BTreeMap<String,Family> }
#[derive(Clone,PartialEq,Eq)]
struct ApprovalParameters { client:String,redirect:String,resource:String,challenge:String,scopes:BTreeSet<String>,state:Option<String> }
#[derive(Clone)]
struct Approval { id:String,browser_hash:String,csrf:String,parameters:ApprovalParameters,expires:u64,processing:bool }
#[derive(Clone)]
struct Code { parameters:ApprovalParameters,scopes:BTreeSet<String>,expires:u64 }
struct LiveState { persistent:Persistent,approvals:BTreeMap<String,Approval>,codes:BTreeMap<String,Code> }
pub struct OAuthService { origin:String,store:CredentialStore,native:Arc<NativeAuthority>,state:Mutex<LiveState>,available:AtomicBool,revocations:tokio::sync::broadcast::Sender<String> }
impl OAuthService {
    pub fn open(origin:String,_root:PathBuf,credentials:Arc<CredentialStore>,native:Arc<NativeAuthority>)->Result<Arc<Self>,OAuthError> {
        let url=Url::parse(&origin).map_err(|_|OAuthError)?;
        if url.scheme()!="https" || url.host_str()!=Some("localhost") || url.port().is_none() || url.path()!="/" || url.query().is_some() || url.fragment().is_some() || !url.username().is_empty() || url.password().is_some() { return Err(OAuthError); }
        let origin=origin.trim_end_matches('/').to_owned();
        let store=credentials.application_store();
        let persistent=match store.read(APPLICATION_ID).map_err(|_|OAuthError)? {
            Some(document)=>{ let saved:Persistent=serde_json::from_slice(document.expose()).map_err(|_|OAuthError)?;if saved.version!=1 || saved.issuer!=origin { return Err(OAuthError); } saved },
            None=>Persistent { version:1,issuer:origin.clone(),clients:BTreeMap::new(),families:BTreeMap::new() }
        };
        let (revocations,_)=tokio::sync::broadcast::channel(256);
        Ok(Arc::new(Self { origin,store,native,state:Mutex::new(LiveState {persistent,approvals:BTreeMap::new(),codes:BTreeMap::new()}),available:AtomicBool::new(true),revocations }))
    }
    pub fn subscribe_revocations(&self)->tokio::sync::broadcast::Receiver<String> { self.revocations.subscribe() }
    pub fn owner_is_active(&self,id:&str)->bool {
        if !self.available.load(Ordering::Acquire) { return false; }
        if id.starts_with("native:") { return self.native.owner_is_active(id); }
        self.state.lock().ok().and_then(|s|s.persistent.families.get(id).map(|f|!f.revoked && now().saturating_sub(f.last_refresh)<REFRESH_SECONDS)).unwrap_or(false)
    }
    pub fn origin(&self)->&str { &self.origin }
    fn resource(&self)->String { format!("{}/mcp",self.origin) }
    fn persist(&self,value:&Persistent)->Result<(),OAuthError> {
        let bytes=Zeroizing::new(serde_json::to_vec(value).map_err(|_|OAuthError)?);
        if self.store.save(APPLICATION_ID,&SecretDocument::new(bytes.to_vec())).is_err() {
            // A readback failure may follow a committed write. Do not keep issuing from stale memory.
            self.available.store(false,Ordering::Release);return Err(OAuthError);
        } Ok(())
    }
    fn ready(&self)->bool { self.available.load(Ordering::Acquire) }
    pub fn shutdown(&self) { if let Ok(mut state)=self.state.lock() { state.approvals.clear();state.codes.clear(); } self.native.invalidate_all(); }
}
impl Authorizer for OAuthService {
    fn authorize(&self,headers:&HeaderMap)->Result<Caller,AuthorizationError> {
        if !self.ready() { return Err(AuthorizationError::Unavailable); }
        if headers.contains_key("x-fetch-settings-capability") {
            return self.native.authorize_settings_headers(headers).ok_or(AuthorizationError::Unauthenticated);
        }
        let values:Vec<_>=headers.get_all(header::AUTHORIZATION).iter().collect();
        let [value]=values.as_slice() else { return Err(AuthorizationError::Unauthenticated); };
        let value=value.to_str().map_err(|_|AuthorizationError::Unauthenticated)?;
        let (scheme,token)=value.split_once(' ').ok_or(AuthorizationError::Unauthenticated)?;
        if !scheme.eq_ignore_ascii_case("Bearer") || !valid_token(token) { return Err(AuthorizationError::Unauthenticated); }
        let hash=digest(token);let timestamp=now();
        let state=self.state.lock().map_err(|_|AuthorizationError::Unavailable)?;
        let (family,access)=state.persistent.families.values().filter(|f|!f.revoked && f.resource==self.resource()).find_map(|f|f.access.iter().find(|a|a.hash==hash && a.expires>timestamp).map(|a|(f,a))).ok_or(AuthorizationError::Unauthenticated)?;
        Ok(Caller::from_grant(family.id.clone(),access.scopes.iter().cloned()))
    }
}
/// The main router retains protected-resource metadata to avoid duplicate registration.
pub fn router(service:Arc<OAuthService>)->Router {
    Router::new().route("/.well-known/oauth-authorization-server",get(metadata)).route("/oauth/register",post(register)).route("/oauth/authorize",get(authorize_get).post(authorize_post)).route("/oauth/token",post(token)).route("/oauth/revoke",post(revoke)).with_state(service)
}
async fn metadata(State(service):State<Arc<OAuthService>>)->Response {
    standard_json(StatusCode::OK,json!({"issuer":service.origin,"authorization_endpoint":format!("{}/oauth/authorize",service.origin),"token_endpoint":format!("{}/oauth/token",service.origin),"registration_endpoint":format!("{}/oauth/register",service.origin),"revocation_endpoint":format!("{}/oauth/revoke",service.origin),"response_types_supported":["code"],"grant_types_supported":["authorization_code","refresh_token"],"token_endpoint_auth_methods_supported":["none"],"code_challenge_methods_supported":["S256"],"scopes_supported":SCOPES,"authorization_response_iss_parameter_supported":true}))
}
async fn register(State(service):State<Arc<OAuthService>>,request:Request)->Response {
    if !service.ready() { return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable"); }
    if !content_type(request.headers(),"application/json") { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); }
    let bytes=match to_bytes(request.into_body(),BODY_LIMIT).await { Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata") };
    let input:Value=match serde_json::from_slice::<RegistrationObject>(&bytes) { Ok(v)=>Value::Object(v.0),Err(_)=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata") };
    let Some(object)=input.as_object() else { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); };
    if input["token_endpoint_auth_method"]!="none" || object.contains_key("client_secret") { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); }
    let Some(redirect_values)=input["redirect_uris"].as_array() else { return oauth_error(StatusCode::BAD_REQUEST,"invalid_redirect_uri"); };
    if redirect_values.is_empty() || redirect_values.len()>16 { return oauth_error(StatusCode::BAD_REQUEST,"invalid_redirect_uri"); }
    let mut redirects=vec![];
    for value in redirect_values { let Some(uri)=value.as_str() else { return oauth_error(StatusCode::BAD_REQUEST,"invalid_redirect_uri"); };if !valid_redirect(uri) || redirects.contains(&uri.to_owned()) { return oauth_error(StatusCode::BAD_REQUEST,"invalid_redirect_uri"); }redirects.push(uri.to_owned()); }
    let grants=if let Some(values)=input.get("grant_types") { let Some(values)=values.as_array() else { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); };let mut set=BTreeSet::new();for value in values { match value.as_str() {Some("authorization_code")=>{set.insert("authorization_code".into());},Some("refresh_token")=>{set.insert("refresh_token".into());},_=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata") } }if !set.contains("authorization_code") { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); }set } else { ["authorization_code".to_owned()].into_iter().collect() };
    if let Some(values)=input.get("response_types") { if values!=&json!(["code"]) { return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata"); } }
    let name=match input.get("client_name") { None=>"Unnamed app".to_owned(),Some(v)=>match v.as_str() {Some(v) if !v.trim().is_empty() && v.len()<=128 && !v.chars().any(char::is_control)=>v.to_owned(),_=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_client_metadata") } };
    let mut state=match service.state.lock() {Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable")};
    if state.persistent.clients.len()>=256 { return busy(); }
    let client=Client { id:random_token(),name,redirect_uris:redirects,grant_types:grants,created:now() };
    let mut staged=state.persistent.clone();staged.clients.insert(client.id.clone(),client.clone());
    if service.persist(&staged).is_err() { return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable"); }state.persistent=staged;
    standard_json(StatusCode::CREATED,json!({"client_id":client.id,"client_id_issued_at":client.created,"client_name":client.name,"redirect_uris":client.redirect_uris,"grant_types":client.grant_types,"response_types":["code"],"token_endpoint_auth_method":"none"}))
}
async fn authorize_get(State(service):State<Arc<OAuthService>>,request:Request)->Response {
    if !service.ready() { return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable"); }
    let pairs=match form_pairs(request.uri().query().unwrap_or("")) {Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The connection request is invalid.")};
    let params=match one_each(&pairs,&["response_type","client_id","redirect_uri","code_challenge","code_challenge_method","resource","scope","state"]) {Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The connection request is invalid.")};
    if params.get("response_type").map(String::as_str)!=Some("code") || params.get("code_challenge_method").map(String::as_str)!=Some("S256") { return page_error(StatusCode::BAD_REQUEST,"Fetch requires an authorization code with PKCE S256."); }
    let client_id=match params.get("client_id") {Some(v)=>v,None=>return page_error(StatusCode::BAD_REQUEST,"The requesting app is missing.")};
    let redirect=match params.get("redirect_uri") {Some(v)=>v,None=>return page_error(StatusCode::BAD_REQUEST,"The callback address is missing.")};
    let challenge=match params.get("code_challenge") {Some(v) if v.len()==43 && URL_SAFE_NO_PAD.decode(v).is_ok()=>v,None=>return page_error(StatusCode::BAD_REQUEST,"The PKCE challenge is missing."),_=>return page_error(StatusCode::BAD_REQUEST,"The PKCE challenge is invalid.")};
    if params.get("resource").map(String::as_str)!=Some(service.resource().as_str()) { return page_error(StatusCode::BAD_REQUEST,"This request targets a different Fetch service."); }
    let scopes=match parse_scopes(params.get("scope").map(String::as_str),true) {Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The requested permissions are invalid.")};
    let mut state=match service.state.lock(){Ok(v)=>v,Err(_)=>return page_error(StatusCode::SERVICE_UNAVAILABLE,"Fetch authorization is unavailable.")};
    let Some(client)=state.persistent.clients.get(client_id).cloned() else {return page_error(StatusCode::BAD_REQUEST,"The requesting app is not registered.");};
    if !client.redirect_uris.iter().any(|r|redirect_matches(r,redirect)) { return page_error(StatusCode::BAD_REQUEST,"The callback address does not match the app registration."); }
    let timestamp=now();state.approvals.retain(|_,a|a.expires>timestamp);state.codes.retain(|_,c|c.expires>timestamp);
    let parameters=ApprovalParameters {client:client_id.clone(),redirect:redirect.clone(),resource:service.resource(),challenge:challenge.clone(),scopes,state:params.get("state").cloned()};
    let existing_cookie=browser_cookie(request.headers());
    let existing=existing_cookie.as_ref().and_then(|cookie|state.approvals.values().find(|a|a.browser_hash==digest(cookie) && a.parameters==parameters).cloned());
    let (approval,cookie)=if let Some(approval)=existing { (approval,existing_cookie.unwrap_or_default()) } else {
        if state.approvals.len()>=32 {return busy();}
        let cookie=existing_cookie.filter(|cookie|state.approvals.values().any(|a|a.browser_hash==digest(cookie))).unwrap_or_else(random_token);
        let approval=Approval{id:random_token(),browser_hash:digest(&cookie),csrf:random_token(),parameters,expires:timestamp+APPROVAL_SECONDS,processing:false};state.approvals.insert(approval.id.clone(),approval.clone());(approval,cookie)
    };
    let mut response=approval_page(&approval,&client,None);
    if let Ok(value)=HeaderValue::from_str(&format!("fetch-browser={cookie}; Path=/oauth; Secure; HttpOnly; SameSite=Lax; Max-Age=600")) {response.headers_mut().insert(header::SET_COOKIE,value);}
    response
}
async fn authorize_post(State(service):State<Arc<OAuthService>>,request:Request)->Response {
    if !service.ready() {return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}
    if request.headers().get(header::ORIGIN).and_then(|h|h.to_str().ok())!=Some(service.origin.as_str()) || !content_type(request.headers(),"application/x-www-form-urlencoded") {return page_error(StatusCode::FORBIDDEN,"This approval must come from Fetch's own connection page.");}
    let cookie=match browser_cookie(request.headers()){Some(v)=>v,None=>return page_error(StatusCode::FORBIDDEN,"Open a fresh Fetch connection page.")};
    let body=match to_bytes(request.into_body(),BODY_LIMIT).await {Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The approval is invalid.")};
    let body=match std::str::from_utf8(&body){Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The approval is invalid.")};
    let pairs=match form_pairs(body){Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The approval is invalid.")};
    let selected:BTreeSet<String>=pairs.iter().filter(|(key,_)|key=="scope").map(|(_,v)|v.clone()).collect();
    let base:Vec<_>=pairs.iter().filter(|(key,_)|key!="scope").cloned().collect();
    let params=match one_each(&base,&["request_id","decision","csrf"]){Ok(v)=>v,Err(_)=>return page_error(StatusCode::BAD_REQUEST,"The approval is invalid.")};
    let id=match params.get("request_id"){Some(v)=>v.clone(),None=>return page_error(StatusCode::BAD_REQUEST,"The approval is missing.")};
    let approval={
        let mut state=match service.state.lock(){Ok(v)=>v,Err(_)=>return page_error(StatusCode::SERVICE_UNAVAILABLE,"Fetch authorization is unavailable.")};
        let Some(approval)=state.approvals.get_mut(&id) else {return page_error(StatusCode::BAD_REQUEST,"This connection request has ended. Connect again from the app.");};
        if approval.expires<=now() || approval.browser_hash!=digest(&cookie) || params.get("csrf")!=Some(&approval.csrf) {return page_error(StatusCode::FORBIDDEN,"This connection approval is invalid or expired.");}
        if approval.processing {return busy();}
        if params.get("decision").map(String::as_str)==Some("deny"){let approval=approval.clone();state.approvals.remove(&id);return authorization_redirect(&service,&approval.parameters,None,Some("access_denied"));}
        if params.get("decision").map(String::as_str)!=Some("allow") || selected.is_empty() || !selected.is_subset(&approval.parameters.scopes){return page_error(StatusCode::BAD_REQUEST,"Choose only permissions requested by this app.");}
        approval.processing=true;approval.clone()
    };
    let remaining=approval.expires.saturating_sub(now()).min(120);
    let confirmation=tokio::time::timeout(std::time::Duration::from_secs(remaining),service.native.confirm_presence_with_timeout("Allow the selected app to connect to Fetch with the permissions shown in its connection page.",std::time::Duration::from_secs(remaining))).await;
    let mut state=match service.state.lock(){Ok(v)=>v,Err(_)=>return page_error(StatusCode::SERVICE_UNAVAILABLE,"Fetch authorization is unavailable.")};
    let Some(current)=state.approvals.get(&id).cloned() else {return page_error(StatusCode::BAD_REQUEST,"This connection request has ended.");};
    if current.parameters!=approval.parameters || current.expires<=now() {state.approvals.remove(&id);return authorization_redirect(&service,&approval.parameters,None,Some("access_denied"));}
    match confirmation {
        Ok(Ok(()))=>{},
        Ok(Err(NativeAuthError::Unavailable))=>{if let Some(current)=state.approvals.get_mut(&id){current.processing=false;}let Some(client)=state.persistent.clients.get(&approval.parameters.client) else {return page_error(StatusCode::BAD_REQUEST,"The app is no longer registered.");};return approval_page(&approval,client,Some("Bring Fetch forward and choose Allow again. No connection was approved."));},
        _=>{state.approvals.remove(&id);return authorization_redirect(&service,&approval.parameters,None,Some("access_denied"));}
    }
    if state.persistent.families.values().filter(|f|family_active(f,now())).count()>=256 {state.approvals.remove(&id);return busy();}
    let code=random_token();state.codes.insert(digest(&code),Code {parameters:approval.parameters.clone(),scopes:selected,expires:now()+CODE_SECONDS});state.approvals.remove(&id);
    authorization_redirect(&service,&approval.parameters,Some(&code),None)
}
async fn token(State(service):State<Arc<OAuthService>>,request:Request)->Response {
    if !service.ready(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}
    let input=match read_form(request).await {Ok(v)=>v,Err(r)=>return r};
    let params=match one_each(&input,&["grant_type","code","client_id","redirect_uri","code_verifier","resource","refresh_token","scope"]){Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_request")};
    let client_id=match params.get("client_id"){Some(v)=>v,None=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_client")};
    if params.get("resource").map(String::as_str)!=Some(service.resource().as_str()){return oauth_error(StatusCode::BAD_REQUEST,"invalid_target");}
    let mut state=match service.state.lock(){Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable")};
    let Some(client)=state.persistent.clients.get(client_id).cloned() else {return oauth_error(StatusCode::BAD_REQUEST,"invalid_client");};
    match params.get("grant_type").map(String::as_str) {
        Some("authorization_code")=>{
            let code=match params.get("code"){Some(v)=>v,None=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant")};
            let hash=digest(code);let Some(code)=state.codes.get(&hash).cloned() else {return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");};
            let verifier=match params.get("code_verifier"){Some(v) if valid_verifier(v)=>v,_=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant")};
            if code.expires<=now() || code.parameters.client!=*client_id || params.get("redirect_uri")!=Some(&code.parameters.redirect) || URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))!=code.parameters.challenge {return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");}
            if params.contains_key("scope") || params.contains_key("refresh_token") {return oauth_error(StatusCode::BAD_REQUEST,"invalid_request");}
            let timestamp=now();let mut staged=state.persistent.clone();prune(&mut staged,timestamp);
            if staged.families.values().filter(|f|family_active(f,timestamp)).count()>=256 || staged.families.len()>=4096 {return busy();}
            let access=random_token();let refresh=client.grant_types.contains("refresh_token").then(random_token);
            let family=Family {id:random_token(),client:client_id.clone(),resource:service.resource(),scopes:code.scopes.clone(),access:vec![AccessToken {hash:digest(&access),expires:timestamp+ACCESS_SECONDS,scopes:code.scopes.clone()}],refresh:refresh.as_ref().map(|r|vec![RefreshToken {hash:digest(r),expires:timestamp+REFRESH_SECONDS,spent:false}]).unwrap_or_default(),revoked:false,last_refresh:timestamp};
            staged.families.insert(family.id.clone(),family);
            if service.persist(&staged).is_err(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}state.persistent=staged;state.codes.remove(&hash);
            token_response(access,refresh,&code.scopes)
        },
        Some("refresh_token")=>{
            if !client.grant_types.contains("refresh_token"){return oauth_error(StatusCode::BAD_REQUEST,"unauthorized_client");}
            let refresh=match params.get("refresh_token"){Some(v) if valid_token(v)=>v,_=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant")};
            let hash=digest(refresh);let timestamp=now();
            let Some((id,index))=state.persistent.families.iter().find_map(|(id,f)|f.refresh.iter().position(|r|r.hash==hash).map(|i|(id.clone(),i))) else {return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");};
            let family=&state.persistent.families[&id];
            if family.client!=*client_id || family.resource!=service.resource() || family.revoked {return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");}
            if family.refresh[index].spent {
                let mut staged=state.persistent.clone();if let Some(family)=staged.families.get_mut(&id){family.revoked=true;}
                if service.persist(&staged).is_err(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}state.persistent=staged;let _=service.revocations.send(id);return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");
            }
            if family.refresh[index].expires<=timestamp {return oauth_error(StatusCode::BAD_REQUEST,"invalid_grant");}
            let scopes=match params.get("scope"){Some(raw)=>match parse_scopes(Some(raw),false){Ok(v) if v.is_subset(&family.scopes)=>v,_=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_scope")},None=>family.scopes.clone()};
            if params.contains_key("code") || params.contains_key("redirect_uri") || params.contains_key("code_verifier"){return oauth_error(StatusCode::BAD_REQUEST,"invalid_request");}
            if family.refresh.len()>=4096 {return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}
            let access=random_token();let new_refresh=random_token();let mut staged=state.persistent.clone();let family=staged.families.get_mut(&id).expect("existing family");
            family.refresh[index].spent=true;family.refresh.push(RefreshToken {hash:digest(&new_refresh),expires:timestamp+REFRESH_SECONDS,spent:false});family.access.retain(|a|a.expires>timestamp);family.access.push(AccessToken {hash:digest(&access),expires:timestamp+ACCESS_SECONDS,scopes:scopes.clone()});family.last_refresh=timestamp;
            // Scope narrowing is durable for the family; it never broadens previously approved access.
            family.scopes=scopes.clone();
            if service.persist(&staged).is_err(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}state.persistent=staged;
            token_response(access,Some(new_refresh),&scopes)
        },
        _=>oauth_error(StatusCode::BAD_REQUEST,"unsupported_grant_type")
    }
}
async fn revoke(State(service):State<Arc<OAuthService>>,request:Request)->Response {
    if !service.ready(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}
    let input=match read_form(request).await {Ok(v)=>v,Err(r)=>return r};
    let params=match one_each(&input,&["token","client_id","token_type_hint"]){Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::BAD_REQUEST,"invalid_request")};
    let Some(client)=params.get("client_id") else {return oauth_error(StatusCode::BAD_REQUEST,"invalid_client");};let Some(token)=params.get("token") else {return oauth_error(StatusCode::BAD_REQUEST,"invalid_request");};
    let hash=digest(token);let mut state=match service.state.lock(){Ok(v)=>v,Err(_)=>return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable")};
    let id=state.persistent.families.iter().find_map(|(id,f)|if f.client==*client && (f.access.iter().any(|a|a.hash==hash)||f.refresh.iter().any(|r|r.hash==hash)){Some(id.clone())}else{None});
    if let Some(id)=id {if !state.persistent.families[&id].revoked {let mut staged=state.persistent.clone();if let Some(f)=staged.families.get_mut(&id){f.revoked=true;}if service.persist(&staged).is_err(){return oauth_error(StatusCode::SERVICE_UNAVAILABLE,"temporarily_unavailable");}state.persistent=staged;let _=service.revocations.send(id);}}
    secure(StatusCode::OK.into_response())
}
fn token_response(access:String,refresh:Option<String>,scopes:&BTreeSet<String>)->Response {let mut body=json!({"access_token":access,"token_type":"Bearer","expires_in":ACCESS_SECONDS,"scope":scopes.iter().cloned().collect::<Vec<_>>().join(" ")});if let Some(refresh)=refresh{body["refresh_token"]=json!(refresh);}standard_json(StatusCode::OK,body)}
fn family_active(f:&Family,timestamp:u64)->bool { !f.revoked && (f.access.iter().any(|a|a.expires>timestamp) || f.refresh.iter().any(|r|!r.spent && r.expires>timestamp)) }
fn prune(state:&mut Persistent,timestamp:u64){state.families.retain(|_,f|timestamp.saturating_sub(f.last_refresh)<REFRESH_SECONDS);for f in state.families.values_mut(){f.access.retain(|a|a.expires>timestamp);f.refresh.retain(|r|r.spent || r.expires>timestamp);}}
fn parse_scopes(value:Option<&str>,default:bool)->Result<BTreeSet<String>,()> {let raw=match value {Some(v)=>v,None if default=>return Ok(DEFAULT_SCOPES.into_iter().map(str::to_owned).collect()),None=>return Err(())};let set:BTreeSet<_>=raw.split_ascii_whitespace().map(str::to_owned).collect();if set.is_empty() || set.iter().any(|s|!SCOPES.contains(&s.as_str())){Err(())}else{Ok(set)}}
fn valid_redirect(uri:&str)->bool { if uri.len()>2048 || uri.chars().any(char::is_control){return false;}let Ok(url)=Url::parse(uri) else{return false;};url.scheme()=="http" && matches!(url.host_str(),Some("localhost"|"127.0.0.1"|"[::1]"|"::1")) && url.username().is_empty() && url.password().is_none() && url.fragment().is_none() && url.port()!=Some(0) }
fn redirect_matches(registered:&str,actual:&str)->bool {if !valid_redirect(actual){return false;}let (Ok(a),Ok(b))=(Url::parse(registered),Url::parse(actual))else{return false;};a.scheme()==b.scheme() && a.host_str()==b.host_str() && a.path()==b.path() && a.query()==b.query()}
fn valid_verifier(v:&str)->bool {(43..=128).contains(&v.len()) && v.bytes().all(|b|b.is_ascii_alphanumeric() || b"-._~".contains(&b))}
fn valid_token(v:&str)->bool {v.len()==43 && v.bytes().all(|b|b.is_ascii_alphanumeric() || b"-_".contains(&b))}
fn random_token()->String {let mut bytes=[0u8;32];OsRng.fill_bytes(&mut bytes);URL_SAFE_NO_PAD.encode(bytes)}
fn digest(value:&str)->String {format!("{:x}",Sha256::digest(value.as_bytes()))}
fn now()->u64 {SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_secs()).unwrap_or(0)}
fn content_type(headers:&HeaderMap,expected:&str)->bool {headers.get(header::CONTENT_TYPE).and_then(|h|h.to_str().ok()).and_then(|h|h.split(';').next()).is_some_and(|h|h.trim().eq_ignore_ascii_case(expected))}
async fn read_form(request:Request)->Result<Vec<(String,String)>,Response> {if !content_type(request.headers(),"application/x-www-form-urlencoded"){return Err(oauth_error(StatusCode::BAD_REQUEST,"invalid_request"));}let bytes=to_bytes(request.into_body(),BODY_LIMIT).await.map_err(|_|oauth_error(StatusCode::BAD_REQUEST,"invalid_request"))?;let text=std::str::from_utf8(&bytes).map_err(|_|oauth_error(StatusCode::BAD_REQUEST,"invalid_request"))?;form_pairs(text).map_err(|_|oauth_error(StatusCode::BAD_REQUEST,"invalid_request"))}
fn form_pairs(input:&str)->Result<Vec<(String,String)>,()> {if input.len()>BODY_LIMIT{return Err(());}fn decode(input:&str)->Result<String,()>{let mut bytes=input.bytes();let mut output=vec![];while let Some(b)=bytes.next(){match b{b'+'=>output.push(b' '),b'%'=>{let a=(bytes.next().ok_or(())? as char).to_digit(16).ok_or(())?;let b=(bytes.next().ok_or(())? as char).to_digit(16).ok_or(())?;output.push((a*16+b)as u8);},v=>output.push(v)}}String::from_utf8(output).map_err(|_|())}input.split('&').filter(|v|!v.is_empty()).map(|v|{let(k,v)=v.split_once('=').unwrap_or((v,""));Ok((decode(k)?,decode(v)?))}).collect()}
fn one_each(pairs:&[(String,String)],allowed:&[&str])->Result<BTreeMap<String,String>,()> {let mut result=BTreeMap::new();for(k,v)in pairs{if !allowed.contains(&k.as_str()) || result.insert(k.clone(),v.clone()).is_some(){return Err(());}}Ok(result)}
fn browser_cookie(headers:&HeaderMap)->Option<String> {let values:Vec<_>=headers.get_all(header::COOKIE).iter().filter_map(|v|v.to_str().ok()).flat_map(|v|v.split(';')).filter_map(|v|{let(k,v)=v.trim().split_once('=')?;(k=="fetch-browser" && valid_token(v)).then(||v.to_owned())}).collect();match values.as_slice(){[v]=>Some(v.clone()),_=>None}}
fn authorization_redirect(service:&OAuthService,parameters:&ApprovalParameters,code:Option<&str>,error:Option<&str>)->Response {let Ok(mut redirect)=Url::parse(&parameters.redirect)else{return page_error(StatusCode::BAD_REQUEST,"The callback is invalid.");};{let mut query=redirect.query_pairs_mut();if let Some(code)=code{query.append_pair("code",code);}if let Some(error)=error{query.append_pair("error",error);}if let Some(state)=&parameters.state{query.append_pair("state",state);}query.append_pair("iss",&service.origin);}let mut response=StatusCode::SEE_OTHER.into_response();match HeaderValue::from_str(redirect.as_str()){Ok(value)=>{response.headers_mut().insert(header::LOCATION,value);secure(response)},Err(_)=>page_error(StatusCode::BAD_REQUEST,"The callback is invalid.")}}
fn approval_page(approval:&Approval,client:&Client,message:Option<&str>)->Response {let permissions=approval.parameters.scopes.iter().map(|scope|{let(label,description)=match scope.as_str(){"fetch:catalog"=>("See installed MCP catalogs","Read permitted tool, resource and prompt descriptions."),"fetch:tools:call"=>("Use installed tools","Call permitted provider tools. These calls can change data."),"fetch:content:read"=>("Read MCP content","Read permitted resources and prompts."),_=>("Manage servers","Discover, install, configure, authenticate and restart permitted servers.")};let checked=if scope=="fetch:manage"{""}else{" checked"};format!("<label><input type=\"checkbox\" name=\"scope\" value=\"{}\"{}> <strong>{}</strong><br><span>{}</span></label>",escape(scope),checked,label,description)}).collect::<String>();let notice=message.map(|m|format!("<p role=\"alert\">{}</p>",escape(m))).unwrap_or_default();let body=format!("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>Connect to Fetch</title><style>body{{font:16px system-ui;margin:64px auto;max-width:640px;padding:0 24px;color:#17202b;background:white}}label{{display:block;margin:20px 0}}span{{color:#536171}}button{{padding:10px 22px;margin-right:12px}}code{{overflow-wrap:anywhere}}</style><h1>Connect {} to Fetch</h1><p>The app supplied this name. It is not proof of its identity.</p><p>Callback: <code>{}</code></p><p>This access covers permitted current and future installed servers, including tools added later.</p>{}<form method=\"post\" action=\"/oauth/authorize\"><input type=\"hidden\" name=\"request_id\" value=\"{}\"><input type=\"hidden\" name=\"csrf\" value=\"{}\">{}<p>Allow asks your Mac to confirm. Fetch does not receive your Mac password.</p><button name=\"decision\" value=\"allow\">Allow</button><button name=\"decision\" value=\"deny\">Deny</button></form></html>",escape(&client.name),escape(&approval.parameters.redirect),notice,escape(&approval.id),escape(&approval.csrf),permissions);secure(Html(body).into_response())}
fn escape(value:&str)->String {value.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;").replace('\'',"&#39;")}
fn page_error(status:StatusCode,message:&str)->Response {secure((status,Html(format!("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><title>Fetch connection</title><h1>Fetch connection</h1><p>{}</p></html>",escape(message)))).into_response())}
fn oauth_error(status:StatusCode,code:&str)->Response {standard_json(status,json!({"error":code}))}
fn standard_json(status:StatusCode,value:Value)->Response {secure((status,Json(value)).into_response())}
fn busy()->Response {let mut response=oauth_error(StatusCode::TOO_MANY_REQUESTS,"temporarily_unavailable");response.headers_mut().insert(header::RETRY_AFTER,HeaderValue::from_static("5"));response}
fn secure(mut response:Response)->Response {let headers=response.headers_mut();headers.insert(header::CACHE_CONTROL,HeaderValue::from_static("no-store"));headers.insert(header::PRAGMA,HeaderValue::from_static("no-cache"));headers.insert(header::REFERRER_POLICY,HeaderValue::from_static("no-referrer"));headers.insert(header::X_FRAME_OPTIONS,HeaderValue::from_static("DENY"));headers.insert(header::X_CONTENT_TYPE_OPTIONS,HeaderValue::from_static("nosniff"));headers.insert(header::CONTENT_SECURITY_POLICY,HeaderValue::from_static("default-src 'none'; style-src 'unsafe-inline'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'"));response}

// Registration is a security boundary; duplicate top-level fields are not resolved by last-wins.
struct RegistrationObject(serde_json::Map<String,Value>);
impl<'de> Deserialize<'de> for RegistrationObject {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
        struct ObjectVisitor;
        impl<'de> serde::de::Visitor<'de> for ObjectVisitor {
            type Value=RegistrationObject;
            fn expecting(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result { formatter.write_str("a registration object with unique fields") }
            fn visit_map<M:serde::de::MapAccess<'de>>(self,mut map:M)->Result<Self::Value,M::Error> {
                let mut result=serde_json::Map::new();
                while let Some((key,value))=map.next_entry::<String,Value>()? {
                    if result.insert(key,value).is_some() { return Err(serde::de::Error::custom("duplicate registration field")); }
                }
                Ok(RegistrationObject(result))
            }
        }
        deserializer.deserialize_map(ObjectVisitor)
    }
}
