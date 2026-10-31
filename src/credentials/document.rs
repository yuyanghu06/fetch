use super::SecretDocument;
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DocumentError;

pub(crate) fn decode(bytes: &[u8]) -> Result<Value, DocumentError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| DocumentError)?;
    if !value.is_object() { return Err(DocumentError); }
    Ok(value)
}

pub(crate) fn encode(value: &Value) -> Result<SecretDocument, DocumentError> {
    if !value.is_object() { return Err(DocumentError); }
    Ok(SecretDocument::new(serde_json::to_vec(value).map_err(|_| DocumentError)?))
}

pub(crate) struct FieldMapping<'a> {
    pub name: &'a str,
    pub pointer: &'a str,
    pub required: bool,
}

pub(crate) fn map_fields(source: &Value, fields: &[FieldMapping<'_>]) -> Result<Map<String, Value>, DocumentError> {
    if !source.is_object() { return Err(DocumentError); }
    let mut mapped = Map::new();
    let mut names = BTreeSet::new();
    let mut pointers = BTreeSet::new();
    for field in fields {
        if !valid_name(field.name) || !names.insert(field.name) || !valid_pointer(field.pointer) || !pointers.insert(field.pointer) { return Err(DocumentError); }
        match source.pointer(field.pointer) {
            Some(value) if field.required && value.is_null() => return Err(DocumentError),
            Some(value) => { mapped.insert(field.name.to_owned(), value.clone()); }
            None if field.required => return Err(DocumentError),
            None => {},
        }
    }
    Ok(mapped)
}

pub(crate) fn update_section(document: &mut Value, section: &str, set: &Map<String, Value>, remove: &[String]) -> Result<(), DocumentError> {
    if !valid_name(section) || !document.is_object() { return Err(DocumentError); }
    let mut names = BTreeSet::new();
    for name in set.keys().chain(remove.iter()) {
        if !valid_name(name) || !names.insert(name) { return Err(DocumentError); }
    }
    let mut updated = match document.get(section) {
        Some(Value::Object(object)) => object.clone(),
        None | Some(Value::Null) => Map::new(),
        _ => return Err(DocumentError),
    };
    for (name, value) in set { updated.insert(name.clone(), value.clone()); }
    for name in remove { updated.remove(name); }
    document[section] = Value::Object(updated);
    Ok(())
}

pub(crate) struct OAuthFields<'a> {
    pub access_token: &'a str,
    pub refresh_token: &'a str,
    pub token_type: &'a str,
    pub expires_in: &'a str,
}
impl OAuthFields<'static> {
    pub const STANDARD: Self = Self { access_token: "/access_token", refresh_token: "/refresh_token", token_type: "/token_type", expires_in: "/expires_in" };
}

#[derive(Clone, Copy, Default)]
pub(crate) struct OAuthPolicy {
    pub require_new_refresh: bool,
    pub require_expiry: bool,
}

pub(crate) fn normalize_oauth(response: &Value, fields: &OAuthFields<'_>, prior_refresh: Option<&str>, now: u64, policy: OAuthPolicy) -> Result<Value, DocumentError> {
    let token = map_fields(response, &[
        FieldMapping { name: "access_token", pointer: fields.access_token, required: true },
        FieldMapping { name: "token_type", pointer: fields.token_type, required: true },
        FieldMapping { name: "refresh_token", pointer: fields.refresh_token, required: false },
        FieldMapping { name: "expires_in", pointer: fields.expires_in, required: false },
    ])?;
    let access = token["access_token"].as_str().filter(|value| valid_token(value)).ok_or(DocumentError)?;
    if !token["token_type"].as_str().is_some_and(|value| value.eq_ignore_ascii_case("bearer")) { return Err(DocumentError); }
    let returned_refresh = token.get("refresh_token").map(|value| value.as_str().filter(|value| valid_token(value)).ok_or(DocumentError)).transpose()?;
    if policy.require_new_refresh && returned_refresh.is_none() { return Err(DocumentError); }
    let refresh = returned_refresh.or(prior_refresh);
    if refresh.is_some_and(|value| !valid_token(value)) { return Err(DocumentError); }
    let expiry = token.get("expires_in").map(|value| value.as_u64().filter(|seconds| *seconds > 0).and_then(|seconds| now.checked_add(seconds)).ok_or(DocumentError)).transpose()?;
    if policy.require_expiry && expiry.is_none() { return Err(DocumentError); }
    Ok(json!({"access_token":access,"refresh_token":refresh,"expires_at":expiry,"verified":false}))
}

fn valid_name(name: &str) -> bool { !name.is_empty() && !name.contains('\0') }
fn valid_token(value: &str) -> bool { !value.is_empty() && value.len() <= 65536 && !value.contains(['\r', '\n', '\0']) }
fn valid_pointer(pointer: &str) -> bool {
    if !pointer.starts_with('/') { return false; }
    let mut characters = pointer.chars();
    while let Some(character) = characters.next() {
        if character == '~' && !matches!(characters.next(), Some('0' | '1')) { return false; }
    }
    true
}
