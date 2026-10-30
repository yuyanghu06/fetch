//! Card-local authentication state and the credential editor shared with Settings.
use super::{components::{button, text_input}, theme};
use iced::{Element, widget::{column, row, text}};
use serde_json::Value;
use zeroize::Zeroize;

pub(super) const SPACING: f32 = theme::CONTROL_GAP;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase { Starting, Saving, Pending, Required, Unknown, NeedsVerification, Authenticated, NotRequired, Failed }

pub(super) struct Flow {
    pub phase: Phase,
    pub guidance: Vec<String>,
    pub error: Option<String>,
    pub read_error: Option<String>,
    pub manual: bool,
    pub entry: String,
    pub secret: String,
    pub reconcile: bool,
}
impl Flow {
    pub fn starting() -> Self {
        Self { phase: Phase::Starting, guidance: vec![], error: None, read_error: None, manual: false,
            entry: String::new(), secret: String::new(), reconcile: false }
    }
    pub fn active(&self) -> bool { matches!(self.phase, Phase::Starting | Phase::Saving | Phase::Pending) }
    pub fn title(&self) -> &'static str {
        match self.phase {
            Phase::Starting => "Starting authentication…",
            Phase::Saving => "Confirm on your Mac. Saving and verifying your credential…",
            Phase::Pending => "Complete sign-in in your browser. Waiting for the provider…",
            Phase::Required => "Enter your provider credential below",
            Phase::Unknown => "Automatic authentication is unavailable for this server",
            Phase::NeedsVerification => "Authentication needs to be checked again. Choose Authenticate.",
            Phase::Authenticated => "Authentication verified",
            Phase::NotRequired => "This server does not require authentication",
            Phase::Failed => "Authentication needs attention",
        }
    }
    pub fn result(&mut self, value: &Value, credential_save: bool) {
        self.error = None;
        self.read_error = None;
        // Text is displayed to the person only. It never chooses a UI action.
        self.guidance = value["next_steps"].as_array().into_iter().flatten()
            .filter(|step| step["actor"] == "user")
            .filter_map(|step| step["instructions"].as_str().map(str::to_owned)).collect();
        self.phase = match value["status"].as_str() {
            Some("ok") if credential_save => Phase::Authenticated,
            Some("authenticating") => Phase::Pending,
            Some("required") => Phase::Required,
            Some("authenticated") => Phase::Authenticated,
            Some("not_required") => Phase::NotRequired,
            Some("unknown") => Phase::Unknown,
            _ => { self.error = Some("Fetch returned an unrecognized authentication result. Refresh server status before trying again.".into()); Phase::Failed }
        };
        self.manual = self.phase == Phase::Required;
        self.reconcile = self.phase == Phase::Pending;
        self.secret.zeroize();
    }
    pub fn observe(&mut self, authentication: &Value) {
        if matches!(self.phase, Phase::Starting | Phase::Saving) { return; }
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
        // A rejected/uncertain manual save can leave an older credential valid.
        // That server state must not turn this failed save into apparent success.
        if self.phase == Phase::Failed && self.manual && phase != Phase::Failed {
            self.reconcile = false;
            return;
        }
        // Keep an unchanged manual form intact, but invalidate settled success
        // when a later server read reports changed authentication evidence.
        if !self.reconcile && self.phase == phase { return; }
        self.phase = phase;
        self.manual = phase == Phase::Required || (phase == Phase::Failed && self.manual);
        self.reconcile = phase == Phase::Pending;
        if phase != Phase::Pending {
            self.guidance.clear();
            self.secret.zeroize();
            self.error = authentication["error"]["message"].as_str().map(str::to_owned)
                .or_else(|| matches!(phase, Phase::Required | Phase::Unknown | Phase::NeedsVerification).then(|| self.error.clone()).flatten());
        }
    }
}
impl Drop for Flow { fn drop(&mut self) { self.secret.zeroize(); } }

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
    let mut actions = row![button("Save and verify credential")
        .on_press_maybe((enabled && !name.trim().is_empty() && !value.is_empty()).then_some(save))].spacing(SPACING);
    if let Some(remove) = remove {
        actions = actions.push(button("Remove entry").on_press_maybe((enabled && !name.trim().is_empty()).then_some(remove)));
    }
    column![text("Private credential entry").size(theme::FIELD_TITLE),
        text("Each save requires Mac confirmation. Fetch stores the credential encrypted and verifies it with the provider before reporting success."),
        name_input, secret_input, actions].spacing(SPACING).into()
}
