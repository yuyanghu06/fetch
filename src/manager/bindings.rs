//! Imported input names are data, independent of the service that consumes them.
use super::{ManagementError, auth_required, invalid_config};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Source { Environment, Header }

#[derive(Clone)]
pub(super) struct InputBinding {
    pub source: Source,
    pub name: String,
    pub reference: String,
}

pub(super) struct InputBindings { entries: Vec<InputBinding>, active_source: Source }

impl InputBindings {
    pub fn from_record(record: &Value) -> Result<Self, ManagementError> {
        Self::from_config(&record["config"])
    }

    pub fn from_config(config: &Value) -> Result<Self, ManagementError> {
        let mut entries = Vec::new();
        let mut unique = BTreeSet::new();
        for (source, value) in [(Source::Environment, &config["environment"]), (Source::Header, &config["connection"]["headers"])] {
            if value.is_null() { continue; }
            for entry in value.as_array().ok_or_else(invalid_config)? {
                let name = entry["name"].as_str().filter(|s| !s.is_empty()).ok_or_else(invalid_config)?;
                let reference = entry["credential_ref"].as_str().filter(|s| !s.is_empty()).ok_or_else(invalid_config)?;
                let key = match source { Source::Environment => format!("env:{name}"), Source::Header => format!("header:{}", name.to_ascii_lowercase()) };
                if !unique.insert(key) { return Err(invalid_config()); }
                entries.push(InputBinding { source, name: name.to_owned(), reference: reference.to_owned() });
            }
        }
        let active_source = if config["kind"] == "remote" { Source::Header } else { Source::Environment };
        Ok(Self { entries, active_source })
    }

    pub fn references(&self) -> BTreeSet<String> {
        self.entries.iter().map(|entry| entry.reference.clone()).collect()
    }

    pub fn matching(&self, source: Source, names: &[String]) -> Vec<&InputBinding> {
        self.entries.iter().filter(|entry| source == self.active_source && entry.source == source && names.iter().any(|name| match source {
            Source::Environment => entry.name == *name,
            Source::Header => entry.name.eq_ignore_ascii_case(name),
        })).collect()
    }

    /// Resolve only this connector's assigned values, without recognizing a provider.
    pub fn resolve_source(&self, source: Source, document: &Value, skip: &[&str]) -> Result<BTreeMap<String, String>, ManagementError> {
        self.entries.iter().filter(|entry| entry.source == source && !skip.iter().any(|name| match source {
            Source::Environment => entry.name == *name,
            Source::Header => entry.name.eq_ignore_ascii_case(name),
        })).map(|entry| {
            let value = document["manual"][&entry.reference].as_str().ok_or_else(auth_required)?;
            Ok((entry.name.clone(), value.to_owned()))
        }).collect()
    }
}
