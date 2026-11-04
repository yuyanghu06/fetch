//! Supervision and the first adapter for shared helper-owned credential storage.
use super::{Manager, ManagementError, Live, auth_response, auth_required, bindings, invalid_config, now, server_step, settings_step, store_error, verification_error};
use crate::{credentials::bridge::{self, BridgeError, Factory, Policy}, installation::{opaque, private_directory}};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

pub(super) const ADAPTER: &str = "google-docs-mcp@1.11.3";
const OVERRIDDEN: &[&str] = &["HOME", "XDG_CONFIG_HOME", "GOOGLE_MCP_PROFILE", "PATH", "LOG_LEVEL", "NODE_OPTIONS", "NODE_PATH", "MCP_TRANSPORT"];
const APP_INPUTS: &[&str] = &["GOOGLE_CLIENT_ID", "GOOGLE_CLIENT_SECRET"];

pub(super) fn matches(record: &Value) -> bool {
    record["config"]["kind"] != "remote" && record["config"]["package"]["name"] == "@a-bonus/google-docs-mcp"
}
pub(super) fn overridden_binding(record: &Value, name: &str) -> bool { matches(record) && OVERRIDDEN.contains(&name) }

#[derive(Clone)]
pub(super) struct Context {
    pub factory: Factory,
    pub revision: u64,
    pub active: Arc<AtomicBool>,
    pub attempt: Option<String>,
}
impl Context {
    async fn revoke(&self) { self.active.store(false, Ordering::Release); self.factory.controller().revoke().await; }
}

#[derive(Clone)]
pub(super) struct GoogleLaunch {
    pub executable: PathBuf,
    pub entry: PathBuf,
    pub home: PathBuf,
    #[cfg(test)] pub script_override: Option<String>,
}
impl GoogleLaunch {
    pub fn inspect(root: &Path, record: &Value) -> Result<Self, ManagementError> {
        if !matches(record) || record["config"]["package"]["installed_version"] != "1.11.3" { return Err(compatibility_error()); }
        let installation = super::railway::installation_id(record).ok_or_else(compatibility_error)?;
        let package = root.join("packages/servers").join(installation).join("node_modules/@a-bonus/google-docs-mcp");
        let package = std::fs::canonicalize(package).map_err(|_| compatibility_error())?;
        let entry = std::fs::canonicalize(package.join("dist/index.js")).map_err(|_| compatibility_error())?;
        let args = record["config"]["args"].as_array().filter(|args| args.len() == 1).ok_or_else(compatibility_error)?;
        let saved_entry = args[0].as_str().ok_or_else(compatibility_error)?;
        if std::fs::canonicalize(saved_entry).map_err(|_| compatibility_error())? != entry || !entry.starts_with(&package) { return Err(compatibility_error()); }
        for (path, expected) in [
            (package.join("dist/auth.js"), "d6965355e0a9462b30b19504462238a1eeee1b0d8d3c14c4988a7e14ebf5f9dd"),
            (entry.clone(), "bb76ed9d24f15d198bbfb5392415b2ba594ece97f1d1d43c1ce1229442ddb806"),
        ] {
            let bytes = std::fs::read(path).map_err(|_| compatibility_error())?;
            if format!("{:x}", Sha256::digest(bytes)) != expected { return Err(compatibility_error()); }
        }
        let manifest: Value = serde_json::from_slice(&std::fs::read(package.join("package.json")).map_err(|_| compatibility_error())?).map_err(|_| compatibility_error())?;
        if manifest["name"] != "@a-bonus/google-docs-mcp" || manifest["version"] != "1.11.3" { return Err(compatibility_error()); }
        let executable = PathBuf::from(record["config"]["executable"].as_str().ok_or_else(compatibility_error)?);
        if !executable.is_absolute() || !executable.is_file() { return Err(compatibility_error()); }
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        let home = root.join("servers").join(format!("{:x}", Sha256::digest(id.as_bytes()))).join("home");
        private_directory(&home).map_err(|_| store_error())?;
        Ok(Self { executable, entry, home, #[cfg(test)] script_override: None })
    }
    pub fn args(&self, mode: &str) -> Vec<String> {
        #[cfg(test)]
        let source = self.script_override.clone().unwrap_or_else(script);
        #[cfg(not(test))]
        let source = script();
        vec!["--input-type=module".into(), "--eval".into(), source, "--".into(), mode.into(), self.entry.to_string_lossy().into_owned()]
    }
}

pub(super) fn script() -> String { format!("{}\n{}", bridge::BRIDGE_CLIENT, include_str!("google_helper.mjs")) }

pub(super) fn input_names(record: &Value) -> Result<Vec<String>, ManagementError> {
    let inputs = bindings::InputBindings::from_record(record)?;
    for name in ["SERVICE_ACCOUNT_PATH", "GOOGLE_IMPERSONATE_USER"] {
        if !inputs.matching(bindings::Source::Environment, &[name.to_owned()]).is_empty() {
            return Err(ManagementError::new(422, "INVALID_CONFIG", "This Google integration supports its OAuth helper. A configured service-account or impersonation method is unsupported; Fetch will not read its files or switch methods."));
        }
    }
    APP_INPUTS.iter().map(|name| {
        let names = vec![(*name).to_owned()];
        let selected = inputs.matching(bindings::Source::Environment, &names);
        if selected.len() != 1 { return Err(required_inputs()); }
        Ok(selected[0].reference.clone())
    }).collect()
}

pub(super) fn environment(record: &Value, document: &Value, launch: &GoogleLaunch) -> Result<BTreeMap<String, String>, ManagementError> {
    let required = input_names(record)?;
    if required.iter().any(|name| !document["manual"][name].as_str().is_some_and(|value| !value.trim().is_empty())) { return Err(required_inputs()); }
    let mut environment = bindings::InputBindings::from_record(record)?.resolve_source(bindings::Source::Environment, document, OVERRIDDEN)?;
    environment.insert("HOME".into(), launch.home.to_string_lossy().into_owned());
    environment.insert("XDG_CONFIG_HOME".into(), launch.home.join(".config").to_string_lossy().into_owned());
    environment.insert("GOOGLE_MCP_PROFILE".into(), "fetch".into());
    environment.insert("PATH".into(), format!("{}:/usr/bin:/bin", launch.executable.parent().ok_or_else(compatibility_error)?.display()));
    environment.insert("LOG_LEVEL".into(), "silent".into());
    environment.insert("LANG".into(), "en_US.UTF-8".into());
    Ok(environment)
}

impl Manager {
    pub(super) async fn revoke_helper(&self, id: &str) {
        let context = self.helpers.lock().await.remove(id);
        if let Some(context) = context { context.revoke().await; }
    }
    async fn revoke_helper_context(&self, id: &str, context: &Context) {
        let removed = {
            let mut helpers = self.helpers.lock().await;
            if helpers.get(id).is_some_and(|current| Arc::ptr_eq(&current.active, &context.active)) { helpers.remove(id) } else { None }
        };
        if let Some(context) = removed { context.revoke().await; }
    }
    pub(super) async fn revoke_helpers(&self) {
        let contexts = std::mem::take(&mut *self.helpers.lock().await);
        for (_, context) in contexts { context.revoke().await; }
    }
    pub(super) async fn invalidate_helper_proof(&self, id: &str) -> Result<(), ManagementError> {
        let store = self.credentials.clone(); let id = id.to_owned();
        tokio::task::spawn_blocking(move || {
            if store.read_json(&id)?.is_none() { return Ok(()); }
            store.update_json(&id, &json!({"manual":{},"oauth":null}), |document| {
                if document["helper"].is_object() { document["helper"]["verified_revision"] = Value::Null; }
                Ok(())
            })?;
            Ok::<_, crate::credentials::CredentialError>(())
        }).await.map_err(|_| store_error())?.map_err(|_| store_error())
    }
    pub(super) async fn helper_context(&self, record: &Value) -> Result<Context, ManagementError> {
        if self.stopping.load(Ordering::SeqCst) { return Err(auth_required()); }
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        let state = self.live.lock().await.get(id).cloned().unwrap_or_default();
        let revision = state.revision;
        if let Some(context) = self.helpers.lock().await.get(id).filter(|context| context.revision == revision && context.active.load(Ordering::Acquire)).cloned() { return Ok(context); }
        self.revoke_helper(id).await;
        let profile = self.verification_profile(record).ok_or_else(verification_error)?;
        let http = self.http.clone();
        #[cfg(test)] let endpoint = self.verification_test_endpoint.clone();
        let verifier: bridge::Verifier = Arc::new(move |payload| {
            let profile = profile.clone(); let bindings = bindings::InputBindings::from_config(&json!({"kind":"npm","environment":[]})); let http = http.clone();
            #[cfg(test)] let endpoint = endpoint.clone();
            Box::pin(async move {
                let token = payload["access_token"].as_str().filter(|token| !token.is_empty()).ok_or(BridgeError::Verification)?;
                let plan = profile.bind(&bindings.map_err(|_| BridgeError::Invalid)?, &json!({"manual":{}}), &[], Some(token)).map_err(|_| BridgeError::Verification)?;
                #[cfg(test)] let plan = if let Some(endpoint) = endpoint { plan.with_endpoint(&endpoint) } else { plan };
                plan.execute(&http).await.map_err(|_| BridgeError::Verification)
            })
        });
        // Profile binding above deliberately supplies only the helper's saved
        // OAuth payload; app inputs are proven by forced helper refresh on save.
        let active = Arc::new(AtomicBool::new(true));
        let current = active.clone(); let states = self.live.clone(); let server = id.to_owned();
        let failure = Arc::new(move |error: BridgeError, session: Arc<dyn Fn() -> bool + Send + Sync>| {
            let current = current.clone(); let states = states.clone(); let server = server.clone();
            tokio::spawn(async move {
                if !current.load(Ordering::Acquire) || !session() { return; }
                let mut states = states.lock().await;
                if let Some(live) = states.get_mut(&server) {
                    if current.load(Ordering::Acquire) && session() {
                        let error = bridge_error(error);
                        live.auth = json!({"status":"failed","error":{"code":error.code,"message":error.message}});
                    }
                }
            });
        });
        let factory = Factory::new(self.credentials.clone(), id.to_owned(), ADAPTER.into(), verifier).with_failure_handler(failure);
        let context = Context { factory, revision, active, attempt: state.auth_attempt };
        let mut helpers = self.helpers.lock().await;
        if self.stopping.load(Ordering::SeqCst) { return Err(auth_required()); }
        helpers.insert(id.to_owned(), context.clone());
        Ok(context)
    }
    async fn google_launch(&self, record: &Value) -> Result<GoogleLaunch, ManagementError> {
        #[cfg(test)] if let Some(launch) = &self.helper_test_launch { return Ok(launch.clone()); }
        let root = self.repository.root().to_owned(); let record = record.clone();
        tokio::task::spawn_blocking(move || GoogleLaunch::inspect(&root, &record)).await.map_err(|_| compatibility_error())?
    }
    pub(super) async fn helper_document(&self, id: &str) -> Result<Value, ManagementError> {
        let store = self.credentials.clone(); let id = id.to_owned();
        Ok(tokio::task::spawn_blocking(move || store.read_json(&id)).await.map_err(|_| store_error())?.map_err(|_| store_error())?
            .unwrap_or_else(|| json!({"manual":{},"oauth":null})))
    }
    async fn run_google(&self, record: &Value, context: &Context, mode: &str, deadline: u64) -> Result<(), ManagementError> {
        let launch = self.google_launch(record).await?;
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        self.check_helper_owner(id, context, deadline).await?;
        let document = self.helper_document(id).await?;
        let mut environment = environment(record, &document, &launch)?;
        let lease = context.factory.start(Policy::Login).await.map_err(bridge_error)?;
        environment.extend(lease.environment());
        let quit = self.stop.notified(); tokio::pin!(quit); quit.as_mut().enable();
        self.check_helper_owner(id, context, deadline).await?;
        let mut command = tokio::process::Command::new(&launch.executable);
        command.args(launch.args(mode)).env_clear().envs(environment).current_dir(&launch.home)
            .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).kill_on_drop(true);
        #[cfg(unix)] command.process_group(0);
        let mut child = command.spawn().map_err(|_| process_error())?;
        let output = child.stdout.take().ok_or_else(process_error)?;
        let stderr = child.stderr.take().ok_or_else(process_error)?;
        let diagnostics = tokio::spawn(async move { let mut error = stderr; let mut buffer = [0; 4096]; while matches!(error.read(&mut buffer).await, Ok(n) if n > 0) {} });
        let outcome = {
            let work = async {
                let mut reader = BufReader::new(output);
                loop {
                    let mut line = Vec::new();
                    let count = (&mut reader).take(16_385).read_until(b'\n', &mut line).await.map_err(|_| process_error())?;
                    if count == 0 { break; }
                    if line.len() > 16_384 { return Err(process_error()); }
                    let event: Value = serde_json::from_slice(&line).map_err(|_| process_error())?;
                    if event["event"] == "failure" { return Err(match event["code"].as_str() {
                        Some("storage") => store_error(), Some("verification") => verification_error(),
                        Some("stale" | "revoked") => bridge_error(BridgeError::Stale), _ => process_error(),
                    }); }
                    if mode != "auth" || event["event"] != "authorization_url" { return Err(process_error()); }
                    let destination = event["url"].as_str().ok_or_else(process_error)?;
                    let url = url::Url::parse(destination).map_err(|_| process_error())?;
                    if destination.len() > 8192 || url.scheme() != "https" || url.host_str() != Some("accounts.google.com")
                        || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() { return Err(process_error()); }
                    self.check_helper_owner(id, context, deadline).await?;
                    super::authentication::open_browser(url.as_str()).await?;
                }
                if child.wait().await.map_err(|_| process_error())?.success() { Ok(()) } else { Err(process_error()) }
            };
            tokio::select! {
                result = work => result,
                _ = &mut quit => Err(auth_required()),
                _ = tokio::time::sleep(Duration::from_secs(deadline.saturating_sub(now()))) => Err(ManagementError::new(400,"AUTHENTICATION_TIMEOUT","Google authentication exceeded its original deadline. Start a fresh attempt.")),
            }
        };
        lease.close();
        if outcome.is_err() {
            #[cfg(unix)] if let Some(id) = child.id() { unsafe { libc::kill(-(id as i32), libc::SIGTERM); } }
            let _ = child.start_kill();
            let _ = tokio::time::timeout(Duration::from_secs(5), child.wait()).await;
        }
        diagnostics.abort();
        outcome
    }
    pub(super) async fn prepare_helper_access(&self, record: &Value, force_refresh: bool) -> Result<Value, ManagementError> {
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        let revision = self.live.lock().await.get(id).map_or(0, |state| state.revision);
        let known = self.helpers.lock().await.get(id).is_some_and(|context| context.revision == revision && context.active.load(Ordering::Acquire));
        let context = self.helper_context(record).await?;
        // A new owner must prove the currently assigned app credentials. Reused
        // runtime owners may check a still-current saved access token.
        let result = self.prepare_helper_with(record, &context, force_refresh || !known, now() + 60).await;
        if let Err(error) = &result {
            let mut states = self.live.lock().await;
            if let Some(state) = states.get_mut(id) { if context.active.load(Ordering::Acquire) && state.revision == context.revision {
                state.auth = json!({"status":"failed","error":{"code":error.code,"message":error.message}});
            } }
            drop(states);
            self.revoke_helper_context(id, &context).await;
        }
        result
    }
    async fn check_helper_owner(&self, id: &str, context: &Context, deadline: u64) -> Result<(), ManagementError> {
        if self.stopping.load(Ordering::SeqCst) || now() >= deadline || !context.active.load(Ordering::Acquire) { return Err(auth_required()); }
        let state = self.live.lock().await.get(id).cloned().unwrap_or_default();
        if state.revision != context.revision || (state.authenticating && state.auth_attempt != context.attempt) { return Err(auth_required()); }
        Ok(())
    }
    async fn prepare_helper_with(&self, record: &Value, context: &Context, force_refresh: bool, deadline: u64) -> Result<Value, ManagementError> {
        self.run_google(record, context, if force_refresh { "check" } else { "prepare" }, deadline).await?;
        let document = self.helper_document(record["server_id"].as_str().ok_or_else(invalid_config)?).await?;
        let helper = &document["helper"];
        if !context.active.load(Ordering::Acquire) || helper["adapter"] != ADAPTER
            || helper["revision"].as_u64().is_none_or(|revision| helper["verified_revision"].as_u64() != Some(revision))
            || !helper["payload"]["access_token"].is_string() { return Err(verification_error()); }
        Ok(document)
    }
    pub(super) async fn authenticate_helper_locked(self: &Arc<Self>, id: &str, record: Value, live: Live) -> Result<Value, ManagementError> {
        let document = self.helper_document(id).await?;
        let refs = input_names(&record)?;
        if refs.iter().any(|name| !document["manual"][name].as_str().is_some_and(|value| !value.trim().is_empty())) {
            self.live.lock().await.entry(id.to_owned()).or_default().auth = json!({"status":"required","error":null});
            return Ok(auth_response(id, "required", vec![settings_step("Save this server's assigned Google client ID and client secret in native Settings. Fetch will not read credentials from personal files or environment defaults.")]));
        }
        let fresh_login = live.auth["status"] == "failed";
        let attempt = opaque("provider"); let deadline = now() + 600; let revision = live.revision;
        { let mut states = self.live.lock().await; let state = states.entry(id.to_owned()).or_default(); state.authenticating = true; state.auth_attempt = Some(attempt.clone()); state.auth_deadline = deadline; state.auth = json!({"status":"authenticating","error":null}); }
        let manager = self.clone(); let server = id.to_owned();
        tokio::spawn(async move {
            manager.revoke_helper(&server).await;
            let result = manager.finish_helper_login(&record, revision, &attempt, deadline, fresh_login).await;
            if let Err(error) = result { manager.end_auth_failure(&server, &attempt, error.code, error.message).await; }
        });
        Ok(auth_response(id, "authenticating", vec![json!({"actor":"user","action":"complete_browser_login","instructions":"Complete Google's existing helper sign-in if a browser opens. Fetch waits for encrypted saved-access verification before reporting success."}), server_step(id)]))
    }
    async fn finish_helper_login(&self, record: &Value, revision: u64, attempt: &str, deadline: u64, fresh_login: bool) -> Result<(), ManagementError> {
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        {
            let state = self.live.lock().await.get(id).cloned().unwrap_or_default();
            if self.stopping.load(Ordering::SeqCst) || now() >= deadline || state.revision != revision
                || !state.authenticating || state.auth_attempt.as_deref() != Some(attempt) { return Err(auth_required()); }
        }
        let context = self.helper_context(record).await?;
        let result = async {
            let document = self.helper_document(id).await?;
            let reuse = !fresh_login && document["helper"]["adapter"] == ADAPTER && document["helper"]["payload"]["refresh_token"].is_string();
            if !reuse { self.run_google(record, &context, "auth", deadline).await?; }
            self.prepare_helper_with(record, &context, reuse, deadline).await?;
            let gate = self.lock(id).await; let _guard = gate.lock().await;
            let mut states = self.live.lock().await; let state = states.get_mut(id).ok_or_else(auth_required)?;
            if !context.active.load(Ordering::Acquire) || self.stopping.load(Ordering::SeqCst) || now() >= deadline
                || state.revision != revision || !state.authenticating || state.auth_attempt.as_deref() != Some(attempt) { return Err(auth_required()); }
            state.authenticating = false; state.auth_attempt = None; state.auth = json!({"status":"authenticated","error":null});
            if state.applied.is_some() { state.oauth_restart_pending = true; }
            Ok(())
        }.await;
        if result.is_err() { self.revoke_helper_context(id, &context).await; }
        result
    }
    pub(super) async fn helper_transport(&self, record: &Value) -> Result<crate::runner::TransportConfig, ManagementError> {
        let launch = self.google_launch(record).await?;
        let id = record["server_id"].as_str().ok_or_else(invalid_config)?;
        let document = self.helper_document(id).await?;
        { let mut states = self.live.lock().await; let state = states.entry(id.to_owned()).or_default(); state.current_manual_hashes = super::manual_hashes(&document); }
        if document["helper"]["adapter"] != ADAPTER || document["helper"]["revision"].as_u64().is_none_or(|revision| document["helper"]["verified_revision"].as_u64() != Some(revision)) { return Err(auth_required()); }
        let environment = environment(record, &document, &launch)?;
        let context = self.helper_context(record).await?;
        Ok(crate::runner::TransportConfig::ManagedStdio { executable: launch.executable.clone(), args: launch.args("runtime"),
            working_directory: Some(record["config"]["working_directory"].as_str().map(PathBuf::from).unwrap_or(launch.home)), environment, bridge: context.factory })
    }
}

fn required_inputs() -> ManagementError { ManagementError::new(409, "SERVER_AUTH_REQUIRED", "Google needs assigned client ID and client secret entries in this server's encrypted Settings. Personal credential files and ambient credentials are not used.") }
fn compatibility_error() -> ManagementError { ManagementError::new(503, "SERVER_RUNTIME_UNAVAILABLE", "This installed Google helper does not match Fetch's supported 1.11.3 interface. Fetch has not run it or tried another authentication method.") }
fn process_error() -> ManagementError { ManagementError::new(422, "CREDENTIAL_VERIFICATION_FAILED", "The Google helper could not complete its selected operation. Its saved credentials may have changed; no verified success or alternate login is reported.") }
fn bridge_error(error: BridgeError) -> ManagementError { match error { BridgeError::Storage => store_error(), BridgeError::Verification => verification_error(), BridgeError::Stale | BridgeError::Revoked => ManagementError::new(409,"SERVER_AUTH_REQUIRED","This Google credential context changed or was cancelled. Start a fresh explicit attempt."), _ => process_error() } }
