//! A shared verifier bound to repository-owned request descriptions, never model input.
use super::{ManagementError, bindings::{InputBindings, Source}, verification_error};
use crate::diagnostics::{send, UpstreamOperation};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::{BTreeMap, BTreeSet}, sync::OnceLock};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Profile {
    pub id: String,
    pub targets: Targets,
    #[serde(default)] pub login: LoginMode,
    #[serde(default)] pub manual: Vec<ManualCheck>,
    pub oauth: Option<RequestSpec>,
}

/// A verifier does not select a login protocol. New remote profiles use standard
/// metadata-driven OAuth unless trusted compatibility data explicitly says otherwise.
#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum LoginMode { #[default] Standard, Manual, Linear }

#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(super) struct Targets {
    #[serde(default)] pub packages: Vec<String>,
    #[serde(default)] pub hosts: Vec<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Selector { pub source: Source, pub names: Vec<String> }

#[derive(Clone, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum ManualCheck {
    Each { selectors: Vec<Selector>, input: String, request: RequestSpec },
    Together { inputs: BTreeMap<String, Selector>, request: RequestSpec },
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestSpec {
    pub method: String,
    pub url: String,
    #[serde(default)] pub headers: BTreeMap<String, String>,
    pub body: Option<Value>,
    pub credentials: Vec<Placement>,
    #[serde(default)] pub success: Vec<Predicate>,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "placement", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Placement {
    Header { input: String, name: String, #[serde(default)] prefix: String, #[serde(default)] strip_prefix: Option<String> },
    Json { input: String, pointer: String },
}

#[derive(Clone, Deserialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Predicate {
    String { pointer: String, #[serde(default)] nonempty: bool },
    Equals { pointer: String, value: Value },
    Absent { pointer: String },
}

struct BoundRequest { spec: RequestSpec, values: BTreeMap<String, String> }
pub(super) struct BoundPlan { requests: Vec<BoundRequest> }

pub(super) fn builtin_for(record: &Value) -> Option<&'static Profile> {
    static PROFILES: OnceLock<Vec<Profile>> = OnceLock::new();
    PROFILES.get_or_init(|| serde_json::from_str(include_str!("verification_profiles.json")).expect("repository-owned verification profiles"))
        .iter().find(|profile| profile.matches_record(record))
}

impl Profile {
    pub fn matches_record(&self, record: &Value) -> bool {
        if record["config"]["kind"] == "remote" {
            record["config"]["connection"]["url"].as_str().and_then(|url| url::Url::parse(url).ok()).is_some_and(|url|
                url.scheme() == "https" && url.port_or_known_default() == Some(443) && url.username().is_empty()
                    && url.password().is_none() && url.fragment().is_none()
                    && url.host_str().is_some_and(|host| self.targets.hosts.iter().any(|known| known == host)))
        } else {
            record["config"]["package"]["name"].as_str().is_some_and(|package| self.targets.packages.iter().any(|known| known == package))
        }
    }

    pub fn manual_references(&self, bindings: &InputBindings) -> BTreeSet<String> {
        self.manual.iter().flat_map(|check| match check {
            ManualCheck::Each { selectors, .. } => selectors.iter().collect::<Vec<_>>(),
            ManualCheck::Together { inputs, .. } => inputs.values().collect(),
        }).flat_map(|selector| bindings.matching(selector.source, &selector.names)).map(|binding| binding.reference.clone()).collect()
    }

    /// Bind every check from one encrypted readback before dispatching any request.
    pub fn bind(&self, bindings: &InputBindings, document: &Value, submitted: &[String], oauth: Option<&str>) -> Result<BoundPlan, ManagementError> {
        let covered = self.manual_references(bindings);
        let imported = bindings.references();
        if self.id.is_empty() || submitted.iter().any(|name| !imported.contains(name) || !covered.contains(name)) { return Err(verification_error()); }
        let mut requests = Vec::new();
        for check in &self.manual {
            match check {
                ManualCheck::Each { selectors, input, request } => {
                    let references: BTreeSet<_> = selectors.iter().flat_map(|selector| bindings.matching(selector.source, &selector.names))
                        .map(|binding| binding.reference.as_str()).collect();
                    for reference in references {
                        if oauth.is_some() && !submitted.iter().any(|name| name == reference) { continue; }
                        let value = document["manual"][reference].as_str().ok_or_else(verification_error)?;
                        requests.push(BoundRequest::new(request, BTreeMap::from([(input.clone(), value.to_owned())]))?);
                    }
                }
                ManualCheck::Together { inputs, request } => {
                    let touched = inputs.values().flat_map(|selector| bindings.matching(selector.source, &selector.names))
                        .any(|binding| submitted.iter().any(|name| name == &binding.reference));
                    if oauth.is_some() && !touched { continue; }
                    let mut values = BTreeMap::new();
                    let mut references = BTreeSet::new();
                    for (input, selector) in inputs {
                        let selected = bindings.matching(selector.source, &selector.names);
                        if selected.len() != 1 || !references.insert(selected[0].reference.clone()) { return Err(verification_error()); }
                        let value = document["manual"][&selected[0].reference].as_str().ok_or_else(verification_error)?;
                        values.insert(input.clone(), value.to_owned());
                    }
                    requests.push(BoundRequest::new(request, values)?);
                }
            }
        }
        if let Some(token) = oauth {
            requests.push(BoundRequest::new(self.oauth.as_ref().ok_or_else(verification_error)?, BTreeMap::from([("token".to_owned(), token.to_owned())]))?);
        }
        if requests.is_empty() { return Err(verification_error()); }
        Ok(BoundPlan { requests })
    }
}

impl BoundRequest {
    fn new(spec: &RequestSpec, values: BTreeMap<String, String>) -> Result<Self, ManagementError> {
        let url = url::Url::parse(&spec.url).map_err(|_| verification_error())?;
        if !matches!(spec.method.as_str(), "GET" | "POST") || url.scheme() != "https" || url.host_str().is_none()
            || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() || url.query().is_some()
            || values.is_empty() { return Err(verification_error()); }
        let mut destinations = BTreeSet::new();
        for name in spec.headers.keys() {
            if !destinations.insert(format!("header:{}", name.to_ascii_lowercase())) { return Err(verification_error()); }
        }
        let mut used = BTreeSet::new();
        let mut json_paths: Vec<Vec<String>> = Vec::new();
        let mut body = spec.body.clone().unwrap_or_else(|| json!({}));
        for placement in &spec.credentials {
            let input = match placement { Placement::Header { input, .. } | Placement::Json { input, .. } => input };
            if !values.contains_key(input) || !used.insert(input.clone()) { return Err(verification_error()); }
            let destination = match placement {
                Placement::Header { name, .. } => {
                    reqwest::header::HeaderName::from_bytes(name.as_bytes()).map_err(|_| verification_error())?;
                    format!("header:{}", name.to_ascii_lowercase())
                }
                Placement::Json { pointer, .. } => {
                    let canonical = json_destination(pointer)?;
                    let path: Vec<String> = serde_json::from_str(&canonical).map_err(|_| verification_error())?;
                    if json_paths.iter().any(|other| path.starts_with(other) || other.starts_with(&path)) { return Err(verification_error()); }
                    json_paths.push(path);
                    insert_json(&mut body, pointer, Value::Null)?;
                    format!("json:{canonical}")
                }
            };
            if !destinations.insert(destination) { return Err(verification_error()); }
        }
        if used.len() != values.len() || spec.method == "GET" && spec.credentials.iter().any(|p| matches!(p, Placement::Json { .. })) { return Err(verification_error()); }
        for predicate in &spec.success {
            let pointer = match predicate { Predicate::String { pointer, .. } | Predicate::Equals { pointer, .. } | Predicate::Absent { pointer } => pointer };
            if !pointer.is_empty() { json_destination(pointer)?; }
        }
        Ok(Self { spec: spec.clone(), values })
    }

    fn request(&self, client: &reqwest::Client) -> Result<reqwest::RequestBuilder, ManagementError> {
        let method = reqwest::Method::from_bytes(self.spec.method.as_bytes()).map_err(|_| verification_error())?;
        let mut request = client.request(method, &self.spec.url);
        for (name, value) in &self.spec.headers { request = request.header(name, value); }
        let mut body = self.spec.body.clone().unwrap_or_else(|| json!({}));
        let mut has_body = self.spec.body.is_some();
        for placement in &self.spec.credentials {
            match placement {
                Placement::Header { input, name, prefix, strip_prefix } => {
                    let value = &self.values[input];
                    let value = strip_prefix.as_ref().and_then(|prefix| value.strip_prefix(prefix)).unwrap_or(value);
                    request = request.header(name, format!("{prefix}{value}"));
                }
                Placement::Json { input, pointer } => {
                    insert_json(&mut body, pointer, Value::String(self.values[input].clone()))?;
                    has_body = true;
                }
            }
        }
        if has_body { request = request.json(&body); }
        Ok(request)
    }
}

impl BoundPlan {
    #[cfg(test)]
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        for request in &mut self.requests { request.spec.url = endpoint.to_owned(); }
        self
    }

    pub async fn execute(&self, client: &reqwest::Client) -> Result<(), ManagementError> {
        // Build first, so malformed later checks cannot dispatch an earlier one.
        let requests = self.requests.iter().map(|check| check.request(client)).collect::<Result<Vec<_>, _>>()?;
        for request in &requests {
            request.try_clone().ok_or_else(verification_error)?.build().map_err(|_| verification_error())?;
        }
        for (check, request) in self.requests.iter().zip(requests) {
            let mut response = send(UpstreamOperation::ProviderVerification, request).await.map_err(|_| verification_error())?;
            if !response.status().is_success() { return Err(verification_error()); }
            if check.spec.success.is_empty() { continue; }
            if response.content_length().is_some_and(|length| length > 1024 * 1024) { return Err(verification_error()); }
            let mut bytes = Vec::new();
            while let Some(chunk) = response.chunk().await.map_err(|_| verification_error())? {
                if bytes.len() + chunk.len() > 1024 * 1024 { return Err(verification_error()); }
                bytes.extend_from_slice(&chunk);
            }
            let value: Value = serde_json::from_slice(&bytes).map_err(|_| verification_error())?;
            if !check.spec.success.iter().all(|predicate| match predicate {
                Predicate::String { pointer, nonempty } => value.pointer(pointer).and_then(Value::as_str).is_some_and(|value| !*nonempty || !value.trim().is_empty()),
                Predicate::Equals { pointer, value: expected } => value.pointer(pointer) == Some(expected),
                Predicate::Absent { pointer } => value.pointer(pointer).is_none(),
            }) { return Err(verification_error()); }
        }
        Ok(())
    }
}

fn json_destination(pointer: &str) -> Result<String, ManagementError> {
    if !pointer.starts_with('/') { return Err(verification_error()); }
    let mut canonical = Vec::new();
    for token in pointer[1..].split('/') {
        let mut decoded = String::new();
        let mut chars = token.chars();
        while let Some(character) = chars.next() {
            decoded.push(if character == '~' { match chars.next() { Some('0') => '~', Some('1') => '/', _ => return Err(verification_error()) } } else { character });
        }
        canonical.push(decoded);
    }
    serde_json::to_string(&canonical).map_err(|_| verification_error())
}

fn insert_json(body: &mut Value, pointer: &str, value: Value) -> Result<(), ManagementError> {
    json_destination(pointer)?;
    let (parent, key) = pointer.rsplit_once('/').ok_or_else(verification_error)?;
    let key = key.replace("~1", "/").replace("~0", "~");
    let object = body.pointer_mut(parent).and_then(Value::as_object_mut).ok_or_else(verification_error)?;
    if object.get(&key).is_some_and(|value| !value.is_null()) { return Err(verification_error()); }
    object.insert(key, value);
    Ok(())
}
