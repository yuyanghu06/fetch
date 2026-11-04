//! Card-local state for native server setup and standard remote OAuth.
use super::{components::{button,text_input},theme};
use iced::{Element,widget::{column,row,text}};
use serde_json::Value;
use zeroize::Zeroize;

pub(super) const SPACING: f32 = theme::CONTROL_GAP;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase { Starting, Saving, Stored, Pending, Required, Unknown, NeedsVerification, Authenticated, NotRequired, Failed }

pub(super) struct Flow {
    pub phase: Phase,
    pub guidance: Vec<String>,
    pub error: Option<String>,
    pub read_error: Option<String>,
    pub reconcile: bool,
    pub entry: String,
    pub secret: String,
    pub storage_result: bool,
}
impl Flow {
    pub fn starting() -> Self {
        Self { phase: Phase::Starting, guidance: vec![], error: None, read_error: None, reconcile: false, entry:String::new(), secret:String::new(), storage_result:false }
    }
    pub fn active(&self) -> bool { matches!(self.phase, Phase::Starting | Phase::Saving | Phase::Pending) }
    pub fn title(&self) -> &'static str {
        match self.phase {
            Phase::Starting => "Starting authentication…",
            Phase::Saving => "Confirm on your Mac to save the manual key…",
            Phase::Stored => "Manual key changes saved in macOS Keychain. Account access has not been verified.",
            Phase::Pending => "Complete sign-in in your browser. Waiting for the provider…",
            Phase::Required => "Complete setup using the server’s own files and authentication process",
            Phase::Unknown => "Automatic authentication is unavailable for this server",
            Phase::NeedsVerification => "Authentication needs to be checked again. Choose Authenticate.",
            Phase::Authenticated => "Authentication verified",
            Phase::NotRequired => "This server does not require authentication",
            Phase::Failed => "Authentication needs attention",
        }
    }
    pub fn result(&mut self, value: &Value) {
        self.error = None;
        self.read_error = None;
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
    }
    pub fn stored(&mut self,value:&Value) {
        self.secret.zeroize();self.storage_result=true;self.reconcile=false;self.read_error=None;
        if value["status"]=="stored"&&value["verified"]==false{
            self.phase=Phase::Stored;self.error=None;
        }else{
            self.phase=Phase::Failed;self.error=Some("Fetch did not confirm storing the manual key. Refresh its status before retrying.".into());
        }
    }
    pub fn observe(&mut self, authentication: &Value) {
        // Storage results describe this edit only; an older login cannot approve it.
        if self.storage_result||matches!(self.phase, Phase::Starting|Phase::Saving) { return; }
        self.read_error = None;
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
        if !self.reconcile && self.phase == phase { return; }
        self.phase = phase;
        self.reconcile = phase == Phase::Pending;
        if phase != Phase::Pending {
            self.guidance.clear();
            self.error = authentication["error"]["message"].as_str().map(str::to_owned)
                .or_else(|| matches!(phase, Phase::Required | Phase::Unknown | Phase::NeedsVerification).then(|| self.error.clone()).flatten());
        }
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
