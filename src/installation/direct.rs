//! Literal direct inputs translated into the same definitions as file imports.
use crate::discovery::{CapturedDefinition, capture_definition};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub(super) struct DirectDefinition {
    pub capture: CapturedDefinition,
    pub identity: String,
}

pub(super) fn command(name: &str, text: &str) -> Result<DirectDefinition, ()> {
    literal_command(text)?;
    let words = shlex::split(text).filter(|words| !words.is_empty()).ok_or(())?;
    let definition = json!({"command":words[0],"args":words[1..]});
    let capture = capture_definition(name, &definition, "claude_code").map_err(|_| ())?;
    // Imported bridges discard existing header values. Direct input cannot
    // retain those values in the original definition used as its identity.
    if capture.row["type"] == "remote" && capture.config["connection"]["headers"].as_array().is_some_and(|headers| !headers.is_empty()) {
        return Err(());
    }
    let identity = identity(name, &json!({"kind":"command","command":words[0],"args":words[1..]}))?;
    Ok(DirectDefinition { capture, identity })
}

pub(super) fn url(name: &str, text: &str) -> Result<DirectDefinition, ()> {
    let capture = capture_definition(name, &json!({"url":text,"type":"streamable_http"}), "claude_code").map_err(|_| ())?;
    let identity = identity(name, &json!({"kind":"url","url":capture.config["connection"]["url"]}))?;
    Ok(DirectDefinition { capture, identity })
}

fn identity(name: &str, definition: &Value) -> Result<String, ()> {
    let bytes = serde_json::to_vec(&json!({"name":name,"definition":definition})).map_err(|_| ())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

/// shlex handles words and quoting, but deliberately does not interpret shell
/// operators or expansions. Reject that syntax before quoting is removed, and
/// reject comments rather than silently dropping part of the submitted input.
fn literal_command(text: &str) -> Result<(), ()> {
    let mut quote = None;
    let mut chars = text.chars();
    while let Some(character) = chars.next() {
        if matches!(character, '\0' | '\n' | '\r') { return Err(()); }
        match (quote, character) {
            (Some('\''), '\'') | (Some('"'), '"') => quote = None,
            (Some('\''), _) => {},
            (_, '\\') => {
                let escaped = chars.next().ok_or(())?;
                if matches!(escaped, '\0' | '\n' | '\r') { return Err(()); }
            },
            (None, '\'' | '"') => quote = Some(character),
            (None, ';' | '|' | '&' | '<' | '>' | '(' | ')' | '#' | '*' | '?' | '[' | ']' | '~' | '{' | '}')
                | (None | Some('"'), '$' | '`') => return Err(()),
            _ => {},
        }
    }
    if quote.is_some() { return Err(()); }
    Ok(())
}
