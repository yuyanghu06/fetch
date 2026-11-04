//! Card-local state for native server setup and standard remote OAuth.
use super::{components::{button,text_input},theme};
use iced::{Element,widget::{column,row,text}};
use serde_json::Value;
use zeroize::Zeroize;

pub(super) const SPACING: f32 = theme::CONTROL_GAP;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase { Starting, Saving, Stored, Pending, Required, Unknown, NeedsVerification, Authenticated, NotRequired, Failed }

#[derive(Clone, PartialEq, Eq)]
pub(super) struct SignIn {
    pub host: String,
    pub opened: bool,
}

struct OutputBaseline {
    generation: Option<u64>,
    lines: usize,
    exit: Option<(Option<i64>, Option<i64>)>,
}

pub(super) struct Flow {
    pub phase: Phase,
    pub guidance: Vec<String>,
    pub error: Option<String>,
    pub read_error: Option<String>,
    pub reconcile: bool,
    pub entry: String,
    pub secret: String,
    pub storage_result: bool,
    pub sign_in: Option<SignIn>,
    pub log: Vec<String>,
    logged_error: Option<String>,
    logged_read_error: Option<String>,
    startup_generation: Option<u64>,
    startup_lines: Vec<String>,
    startup_exit: Option<(Option<i64>,Option<i64>)>,
    request_output_baseline: Option<OutputBaseline>,
}
impl Flow {
    pub fn starting() -> Self {
        let mut flow=Self { phase: Phase::Starting, guidance: vec![], error: None, read_error: None, reconcile: false, entry:String::new(), secret:String::new(), storage_result:false, sign_in:None,
            log:vec![],logged_error:None,logged_read_error:None,
            startup_generation:None,startup_lines:vec![],startup_exit:None,request_output_baseline:None };
        flow.record_state();
        flow
    }
    /// Displaying a server is a read, not an authentication request.
    pub fn passive(authentication: &Value) -> Self {
        let mut flow = Self::starting();
        flow.phase = Phase::Unknown;
        flow.observe(authentication);
        flow
    }
    pub fn begin_authentication(&mut self) {
        self.request_output_baseline=Some(OutputBaseline {
            generation:self.startup_generation,
            lines:self.startup_lines.len(),
            exit:self.startup_exit,
        });
        self.secret.zeroize();
        self.entry.clear();
        self.phase = Phase::Starting;
        self.guidance.clear();
        self.error = None;
        self.read_error = None;
        self.reconcile = false;
        self.storage_result = false;
        self.sign_in = None;
        self.logged_error = None;
        self.logged_read_error = None;
        self.log.retain(|line| !line.starts_with("[error] "));
    }
    pub fn active(&self) -> bool { matches!(self.phase, Phase::Starting | Phase::Saving | Phase::Pending) }
    pub fn title(&self) -> &'static str {
        match self.phase {
            Phase::Starting => "Starting authentication…",
            Phase::Saving => "Confirm on your Mac to save the manual key…",
            Phase::Stored => "Manual key changes saved in macOS Keychain. Account access has not been verified.",
            Phase::Pending => "Starting the server or waiting for sign-in…",
            Phase::Required => "Server setup is needed",
            Phase::Unknown => "Account sign-in has not been verified",
            Phase::NeedsVerification => "Authentication needs to be checked again. Choose Authenticate.",
            Phase::Authenticated => "Authentication verified",
            Phase::NotRequired => "This server does not require authentication",
            Phase::Failed => "Authentication needs attention",
        }
    }
    pub fn result(&mut self, value: &Value) {
        self.error = None;
        self.read_error = None;
        self.sign_in = Self::read_sign_in(value);
        // Text is displayed to the person only. It never chooses a UI action.
        self.guidance = value["next_steps"].as_array().into_iter().flatten()
            .filter(|step| step["actor"] == "user")
            .filter_map(|step| step["instructions"].as_str().map(str::to_owned)).collect();
        self.phase = match value["status"].as_str() {
            Some("authenticating") => Phase::Pending,
            Some("required") => Phase::Required,
            Some("authenticated") => Phase::Authenticated,
            Some("not_required") => Phase::NotRequired,
            Some("unknown") => Phase::Unknown,
            _ => { self.error = Some("Fetch returned an unrecognized authentication result. Refresh server status before trying again.".into()); Phase::Failed }
        };
        self.reconcile = self.phase == Phase::Pending;
        self.record_state();
    }
    pub fn stored(&mut self,value:&Value) {
        self.secret.zeroize();self.storage_result=true;self.reconcile=false;self.read_error=None;self.sign_in=None;
        if value["status"]=="stored"&&value["verified"]==false{
            self.phase=Phase::Stored;self.error=None;
        }else{
            self.phase=Phase::Failed;self.error=Some("Fetch did not confirm storing the manual key. Refresh its status before retrying.".into());
        }
        self.record_state();
    }

    fn record_startup(&mut self, authentication:&Value) {
        let startup=&authentication["startup"];
        let Some(generation)=startup["generation"].as_u64() else{return};
        self.startup_generation=Some(generation);
        self.startup_lines=startup["lines"].as_array().into_iter().flatten()
            .filter_map(|line|line.as_str().map(str::to_owned)).collect();
        // The backend owns the bounded, cumulative history across attempts.
        // Rebuild from its snapshot so a new capture ID never repeats a prefix.
        self.log=self.startup_lines.iter().map(|line|format!("[server] {line}")).collect();
        let exit=(startup["exit_code"].as_i64(),startup["signal"].as_i64());
        self.startup_exit=if exit.0.is_some()||exit.1.is_some(){Some(exit)}else{None};
        if self.startup_exit.is_some(){
            if let Some(code)=exit.0{self.log.push(format!("[server] Process exited with status {code}."));}
            else if let Some(signal)=exit.1{self.log.push(format!("[server] Process ended with signal {signal}."));}
        }
        self.logged_error=None;self.logged_read_error=None;
        self.record_state();
    }

    fn has_current_service_output(&self) -> bool {
        let Some(baseline)=&self.request_output_baseline else {
            return !self.startup_lines.is_empty() || self.startup_exit.is_some();
        };
        let new_capture=self.startup_generation!=baseline.generation;
        let carried_exit=usize::from(new_capture && baseline.exit.is_some());
        self.startup_lines.len()>baseline.lines+carried_exit
            || self.startup_exit.is_some() && (new_capture || self.startup_exit!=baseline.exit)
    }
    pub fn observe(&mut self, authentication: &Value) {
        self.record_startup(authentication);
        // Storage results describe this edit only; an older login cannot approve it.
        if self.storage_result||matches!(self.phase, Phase::Starting|Phase::Saving) { return; }
        self.read_error = None;
        self.sign_in = Self::read_sign_in(authentication);
        let phase = match authentication["status"].as_str() {
            Some("authenticating") => Phase::Pending,
            Some("authenticated") => Phase::Authenticated,
            Some("not_required") => Phase::NotRequired,
            Some("failed") => Phase::Failed,
            Some("required") => Phase::Required,
            Some("unknown") => if self.phase == Phase::Unknown { Phase::Unknown } else { Phase::NeedsVerification },
            _ => return,
        };
        // A later server read can invalidate previously settled evidence.
        self.phase = phase;
        self.reconcile = phase == Phase::Pending;
        if phase != Phase::Pending {
            self.guidance.clear();
            if self.request_output_baseline.is_none() || self.has_current_service_output() || self.error.is_none() {
                self.error = authentication["error"]["message"].as_str().map(str::to_owned)
                    .or_else(|| matches!(phase, Phase::Required | Phase::Unknown | Phase::NeedsVerification).then(|| self.error.clone()).flatten());
            }
        }
        self.record_state();
    }

    /// The console displays captured service output. Only Fetch's own request
    /// failures are a fallback when the service has supplied no output.
    pub(super) fn record_state(&mut self) {
        let service_output = self.has_current_service_output();
        if service_output {
            self.log.retain(|line| !line.starts_with("[error] "));
        }
        if self.logged_error != self.error {
            if !service_output { if let Some(error)=&self.error {self.log.push(format!("[error] {error}"));} }
            self.logged_error=self.error.clone();
        }
        if self.logged_read_error != self.read_error {
            if !service_output { if let Some(error)=&self.read_error {self.log.push(format!("[error] {error}"));} }
            self.logged_read_error=self.read_error.clone();
        }
    }

    fn read_sign_in(value: &Value) -> Option<SignIn> {
        if value["status"] != "authenticating" { return None; }
        let host = value["sign_in"]["host"].as_str()?;
        if host.is_empty() || host.chars().any(char::is_control) { return None; }
        Some(SignIn { host:host.to_owned(), opened:value["sign_in"]["opened"].as_bool()? })
    }
}

impl Drop for Flow { fn drop(&mut self) { self.secret.zeroize(); } }


/// Own the submitted value directly rather than cloning another UI secret string.
pub(super) fn manual_patch(name:String,value:Option<String>)->Value{
    let (set,remove)=if let Some(value)=value{
        let mut entry=serde_json::Map::new();entry.insert("name".into(),Value::String(name));entry.insert("value".into(),Value::String(value));
        (vec![Value::Object(entry)],vec![])
    }else{(vec![],vec![Value::String(name)])};
    let mut patch=serde_json::Map::new();patch.insert("set".into(),Value::Array(set));patch.insert("remove".into(),Value::Array(remove));Value::Object(patch)
}
/// Only existing references are offered; this does not create a binding or infer a provider.
pub(super) fn credential_references(server: &Value) -> Vec<(String, String)> {
    server["config"]["environment"].as_array().into_iter().flatten()
        .chain(server["config"]["connection"]["headers"].as_array().into_iter().flatten())
        .filter_map(|binding| Some((binding["name"].as_str()?.to_owned(), binding["credential_ref"].as_str()?.to_owned())))
        .collect()
}

pub(super) fn credential_editor<'a, M: Clone + 'a>(
    name: &'a str, value: &'a str, enabled: bool,
    on_name: impl Fn(String) -> M + 'a, on_value: impl Fn(String) -> M + 'a,
    save: M, remove: Option<M>,
) -> Element<'a, M> {
    let mut name_input = text_input("Credential entry name", name);
    let mut secret_input = text_input("API key or token", value).secure(true);
    if enabled { name_input = name_input.on_input(on_name); secret_input = secret_input.on_input(on_value); }
    let mut actions = row![button("Save manual key")
        .on_press_maybe((enabled && !name.trim().is_empty() && !value.is_empty()).then_some(save))].spacing(SPACING);
    if let Some(remove) = remove {
        actions = actions.push(button("Remove entry").on_press_maybe((enabled && !name.trim().is_empty()).then_some(remove)));
    }
    column![text("Manual API key or fixed token").size(theme::FIELD_TITLE),
        text("Use the server’s own setup first. When it needs an environment or header key, save only an existing assigned entry. Mac confirmation saves it directly in Keychain; storage alone does not verify account access."),
        name_input, secret_input, actions].spacing(SPACING).into()
}
