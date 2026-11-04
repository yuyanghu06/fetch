//! Private, sanitized startup output shared by process and remote login attempts.
use super::TransportConfig;
use serde_json::{Value, json};
use std::{collections::VecDeque, process::ExitStatus, sync::{Arc, Mutex, Weak}};
use zeroize::{Zeroize, Zeroizing};

pub(super) const MAX_LINES: usize = 4096;
pub(super) const MAX_BYTES: usize = 1024 * 1024;
pub(super) const MAX_LINE_BYTES: usize = 64 * 1024;
const LIMIT_NOTICE: &str = "[fetch] Older console output was removed (1 MiB or 4096-line history limit).";

struct State {
    lines: VecDeque<String>, bytes: usize, exit_code: Option<i32>, signal: Option<i32>,
    frozen: bool, truncated: bool, redactions: Zeroizing<Vec<String>>, save_error_reported: bool,
}
pub(crate) struct StartupDiagnostics { generation: u64, state: Mutex<State>, persistence: Mutex<Option<(Weak<super::log_store::LogStore>, String)>> }
impl StartupDiagnostics {
    pub(crate) fn new(generation: u64, config: &TransportConfig) -> Option<Arc<Self>> {
        let mut redactions = Vec::new();
        match config {
            TransportConfig::Stdio { args, environment, .. } => {
                redactions.extend(environment.values().filter(|value| !value.is_empty()).cloned());
                for arg in args {
                    if !arg.is_empty() { redactions.push(arg.clone()); }
                    if let Some((_, value)) = arg.split_once('=') { if !value.is_empty() { redactions.push(value.into()); } }
                }
            }
            TransportConfig::Http { headers, .. } | TransportConfig::Sse { headers, .. } => {
                redactions.extend(headers.values().filter(|value| !value.is_empty()).cloned());
                for value in headers.values() {
                    if let Some(value) = value.strip_prefix("Bearer ") { if !value.is_empty() { redactions.push(value.into()); } }
                }
            }
        }
        normalize_redactions(&mut redactions);
        Some(Arc::new(Self { generation, state: Mutex::new(State { lines: VecDeque::new(), bytes: 0, exit_code: None, signal: None, frozen: false, truncated: false, redactions: Zeroizing::new(redactions), save_error_reported: false }), persistence: Mutex::new(None) }))
    }
    /// Carry only retained, already sanitized output into a fresh attempt.
    /// Account values and the previous attempt's live state never carry over.
    pub(crate) fn inherit_history(&self, previous: &Self) {
        let mut previous = previous.state.lock().expect("startup diagnostics");
        previous.frozen = true;
        previous.redactions.zeroize();
        let mut state = self.state.lock().expect("startup diagnostics");
        state.lines = previous.lines.clone();
        state.bytes = previous.bytes;
        state.truncated = previous.truncated;
        if let Some(code) = previous.exit_code {
            retain_line(&mut state, "", &format!("Process exited with status {code}."));
        } else if let Some(signal) = previous.signal {
            retain_line(&mut state, "", &format!("Process ended with signal {signal}."));
        }
    }
    pub(crate) fn add_redactions(&self, values: impl IntoIterator<Item = String>) {
        let mut state = self.state.lock().expect("startup diagnostics");
        if state.frozen { return; }
        state.redactions.extend(values.into_iter().filter(|value| !value.is_empty()));
        normalize_redactions(&mut state.redactions);
    }
    pub(crate) fn line(&self, line: &str) { self.text("", line); }
    pub(crate) fn text(&self, source: &str, text: &str) {
        if let Ok(value) = serde_json::from_str::<Value>(text) { self.value(source, &value); return; }
        let mut state = self.state.lock().expect("startup diagnostics");
        if state.frozen { return; }
        for line in text.lines() {
            let safe = sanitize_text(line, &state.redactions);
            retain_line(&mut state, source, &safe);
        }
        drop(state); self.changed();
    }
    pub(crate) fn value(&self, source: &str, value: &Value) {
        let mut state = self.state.lock().expect("startup diagnostics");
        if state.frozen { return; }
        let mut redactions = Zeroizing::new(state.redactions.to_vec());
        collect_secrets(value, &mut redactions);
        normalize_redactions(&mut redactions);
        let safe = sanitize_value(value, &redactions);
        retain_line(&mut state, source, &safe.to_string());
        drop(state); self.changed();
    }
    pub(crate) fn natural_exit(&self, status: ExitStatus) {
        let mut state = self.state.lock().expect("startup diagnostics");
        if state.frozen { return; }
        state.exit_code = status.code();
        #[cfg(unix)] { use std::os::unix::process::ExitStatusExt; state.signal = status.signal(); }
        drop(state); self.changed();
    }
    pub(crate) fn is_frozen(&self) -> bool { self.state.lock().expect("startup diagnostics").frozen }
    pub(crate) fn freeze(&self) {
        let mut state = self.state.lock().expect("startup diagnostics");
        state.frozen = true; state.redactions.zeroize();
        drop(state); self.changed();
    }
    pub(super) fn attach_store(self: &Arc<Self>, store: &Arc<super::log_store::LogStore>, id: &str) {
        *self.persistence.lock().expect("console log persistence") = Some((Arc::downgrade(store), id.to_owned()));
        store.track(id, self);
    }
    fn changed(&self) {
        if let Some((store, id)) = self.persistence.lock().expect("console log persistence").as_ref() {
            if let Some(store) = store.upgrade() { store.changed(id); }
        }
    }
    pub(super) fn persistence_failed(&self, message: &str) {
        let mut state = self.state.lock().expect("startup diagnostics");
        if !state.save_error_reported {
            retain_line(&mut state, "fetch", message);
            state.save_error_reported = true;
        }
    }
    pub(super) fn restore(generation: u64, value: &Value) -> Result<Arc<Self>, ()> {
        let object = value.as_object().ok_or(())?;
        if object.len() != 4 || value["schemaVersion"] != 1 { return Err(()); }
        let lines = value["lines"].as_array().filter(|lines| lines.len() <= MAX_LINES).ok_or(())?;
        let mut restored = VecDeque::new(); let mut bytes = 0;
        for line in lines {
            let line = line.as_str().filter(|line| line.len() <= MAX_LINE_BYTES && !line.chars().any(|c| c.is_control() && c != '\t')).ok_or(())?;
            bytes += line.len(); if bytes > MAX_BYTES { return Err(()); }
            restored.push_back(line.to_owned());
        }
        let exit = |name: &str| if value[name].is_null() { Ok(None) } else { value[name].as_i64().and_then(|v| i32::try_from(v).ok()).map(Some).ok_or(()) };
        let truncated = restored.front().is_some_and(|line| line == LIMIT_NOTICE);
        Ok(Arc::new(Self { generation, state: Mutex::new(State {
            lines: restored, bytes, exit_code: exit("exit_code")?, signal: exit("signal")?, frozen: true,
            truncated, redactions: Zeroizing::new(vec![]), save_error_reported: false,
        }), persistence: Mutex::new(None) }))
    }
    pub(super) fn saved_snapshot(&self) -> Value {
        let state = self.state.lock().expect("startup diagnostics");
        json!({"schemaVersion":1,"lines":state.lines,"exit_code":state.exit_code,"signal":state.signal})
    }
    pub(crate) fn snapshot(&self) -> Value {
        let state = self.state.lock().expect("startup diagnostics");
        json!({"generation":self.generation,"lines":state.lines,"exit_code":state.exit_code,"signal":state.signal})
    }
}
fn normalize_redactions(values: &mut Vec<String>) {
    values.sort_by_key(|value| std::cmp::Reverse(value.len())); values.dedup();
}
fn retain_line(state: &mut State, source: &str, line: &str) {
    let line = line.chars().filter(|character| !character.is_control() || *character == '\t').collect::<String>();
    let line = line.trim();
    if line.is_empty() { return; }
    let mut line = if source.is_empty() { line.to_owned() } else { format!("[{source}] {line}") };
    if line.len() > MAX_LINE_BYTES {
        const NOTICE: &str = " [line truncated at 65536 bytes]";
        let mut end = MAX_LINE_BYTES - NOTICE.len();
        while !line.is_char_boundary(end) { end -= 1; }
        line.truncate(end); line.push_str(NOTICE);
    }
    if state.lines.front().is_some_and(|line| line == LIMIT_NOTICE) {
        state.bytes -= state.lines.pop_front().unwrap().len();
    }
    while state.lines.len() >= MAX_LINES - 1 || state.bytes + line.len() + LIMIT_NOTICE.len() > MAX_BYTES {
        let Some(old) = state.lines.pop_front() else { break; };
        state.bytes -= old.len(); state.truncated = true;
    }
    if state.truncated { state.lines.push_front(LIMIT_NOTICE.into()); state.bytes += LIMIT_NOTICE.len(); }
    state.bytes += line.len(); state.lines.push_back(line);
}
fn sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase().replace('-', "_");
    key.contains("token") || key.contains("secret") || key.contains("password") || key.contains("credential")
        || matches!(key.as_str(), "authorization" | "cookie" | "set_cookie" | "api_key" | "apikey" | "code_verifier" | "code" | "state")
}
fn collect_secrets(value: &Value, secrets: &mut Vec<String>) {
    match value {
        Value::Object(fields) => for (name, value) in fields {
            if sensitive_key(name) && !(name == "code" && value.is_number()) {
                collect_sensitive_values(value, secrets);
            } else {
                collect_secrets(value, secrets);
            }
        },
        Value::Array(values) => for value in values { collect_secrets(value, secrets); },
        _ => {},
    }
}
fn collect_sensitive_values(value: &Value, secrets: &mut Vec<String>) {
    match value {
        Value::String(value) if !value.is_empty() => secrets.push(value.clone()),
        Value::Number(value) => secrets.push(value.to_string()),
        Value::Object(fields) => for value in fields.values() { collect_sensitive_values(value, secrets); },
        Value::Array(values) => for value in values { collect_sensitive_values(value, secrets); },
        _ => {},
    }
}
fn sanitize_value(value: &Value, secrets: &[String]) -> Value {
    match value {
        Value::Object(fields) => Value::Object(fields.iter().map(|(name, value)| {
            let value = if sensitive_key(name) && !(name == "code" && value.is_number()) && !value.is_null() { json!("[redacted]") } else { sanitize_value(value, secrets) };
            (sanitize_text(name, secrets), value)
        }).collect()),
        Value::Array(values) => Value::Array(values.iter().map(|value| sanitize_value(value, secrets)).collect()),
        Value::String(value) => json!(sanitize_text(value, secrets)),
        _ => value.clone(),
    }
}
fn sanitize_text(value: &str, secrets: &[String]) -> String {
    if let Some(offset) = value.find('{') {
        if let Ok(embedded) = serde_json::from_str::<Value>(&value[offset..]) {
            let mut redactions = Zeroizing::new(secrets.to_vec());
            collect_secrets(&embedded, &mut redactions);
            normalize_redactions(&mut redactions);
            return format!("{}{}", sanitize_plain_text(&value[..offset], &redactions), sanitize_value(&embedded, &redactions));
        }
    }
    sanitize_plain_text(value, secrets)
}
fn sanitize_plain_text(value: &str, secrets: &[String]) -> String {
    let mut value = strip_urls(value);
    for secret in secrets { value = value.replace(secret, "[redacted]"); }
    redact_assignments(&value)
}
fn strip_urls(value: &str) -> String {
    let mut remaining = value;
    let mut output = String::new();
    loop {
        let lower = remaining.to_ascii_lowercase();
        let offset = ["https://", "http://"].iter().filter_map(|scheme| lower.find(scheme)).min();
        let Some(offset) = offset else { output.push_str(remaining); return output };
        output.push_str(&remaining[..offset]); output.push_str("[URL removed]");
        let tail = &remaining[offset..];
        let end = tail.find(|character: char| character.is_whitespace() || matches!(character, '"' | '\'' | '<' | '>' | '`')).unwrap_or(tail.len());
        remaining = &tail[end..];
    }
}
fn redact_assignments(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    let mut cutoff = lower.match_indices("bearer").find_map(|(offset, _)| {
        let mut end = offset + "bearer".len();
        if !lower.as_bytes().get(end).is_some_and(u8::is_ascii_whitespace) { return None; }
        while lower.as_bytes().get(end).is_some_and(u8::is_ascii_whitespace) { end += 1; }
        Some(end)
    });
    for label in ["token", "secret", "password", "authorization", "api_key", "apikey", "credential", "cookie", "code_verifier"] {
        for (offset, _) in lower.match_indices(label) {
            let mut end = offset + label.len();
            while lower.as_bytes().get(end).is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'_' | b'-')) { end += 1; }
            while lower.as_bytes().get(end).is_some_and(|byte| byte.is_ascii_whitespace() || matches!(*byte, b'\'' | b'"')) { end += 1; }
            if lower.as_bytes().get(end).is_some_and(|byte| matches!(*byte, b'=' | b':')) {
                end += 1;
                while lower.as_bytes().get(end).is_some_and(u8::is_ascii_whitespace) { end += 1; }
                cutoff = Some(cutoff.map_or(end, |current| current.min(end)));
            }
        }
    }
    match cutoff { Some(end) => format!("{}[redacted]", &value[..end]), None => value.to_owned() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::BTreeMap, path::PathBuf};

    fn diagnostics() -> Arc<StartupDiagnostics> {
        StartupDiagnostics::new(7, &TransportConfig::Stdio { executable: PathBuf::from("/fixture/node"), args: vec!["/fixture/entry.js".into(), "--api-key=argument-secret".into()], working_directory: None, environment: BTreeMap::from([("SECRET".into(), "environment-secret".into())]) }).unwrap()
    }
    #[test]
    fn output_is_sanitized_before_retention_without_losing_the_actual_failure() {
        let capture = diagnostics();
        capture.line("FATAL: Missing GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET");
        capture.line("value environment-secret argument-secret /fixture/entry.js");
        capture.line("Authorize at https://example.com/login?state=private-state&code=private-code");
        capture.line("Request failed: Authorization: Bearer unknown-private-value");
        capture.line("token=unknown-private-token");
        let value = capture.snapshot().to_string();
        assert!(value.contains("Missing GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET"));
        for secret in ["environment-secret", "argument-secret", "/fixture/entry.js", "https://", "private-state", "private-code", "unknown-private-value", "unknown-private-token"] { assert!(!value.contains(secret), "secret retained: {secret}"); }
    }
    #[test]
    fn attempt_history_is_bounded_and_freezes_at_the_terminal_startup_result() {
        let capture = diagnostics();
        for n in 0..100 { capture.line(&format!("line {n} {}", "x".repeat(2000))); }
        let value = capture.snapshot();
        let lines = value["lines"].as_array().unwrap();
        assert!(lines.len() <= MAX_LINES);
        assert!(lines.iter().map(|line| line.as_str().unwrap().len()).sum::<usize>() <= MAX_BYTES);
        assert!(lines.last().unwrap().as_str().unwrap().starts_with("line 99"));
        capture.freeze(); capture.line("post-startup output is not retained");
        assert_eq!(capture.snapshot(), value);
    }
}
