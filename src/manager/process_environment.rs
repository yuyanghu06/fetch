// SPDX-License-Identifier: Apache-2.0
// Modified source from OpenAI Codex (Copyright OpenAI), licensed under Apache-2.0.
// Pinned upstream: 75424f87ca2b1c482e0509a5b1ceb7459587f396
// - codex-rs/rmcp-client/src/utils.rs::create_env_for_mcp_server / DEFAULT_ENV_VARS
//   SHA-256: 29e49df7365ecdd1f718c8c3e2d084c1a4aea5787bf8cf1f337e05f20ebfdcbf
// - codex-rs/network-proxy/src/certs.rs::CUSTOM_CA_ENV_KEYS
// - codex-rs/protocol/src/shell_environment.rs::NON_INHERITABLE_ENV_VARS
// Fetch changes: inject parent values for testing, use BTreeMap and std::io,
// omit Codex-specific config types, and convert losslessly at Fetch's existing
// string-valued transport boundary. Fetch supplies its protected paths as
// explicit overrides. See LICENSE-CODEX-APACHE-2.0 and NOTICE-CODEX at the
// repository root for licensing and attribution.

//! Build a child environment without reading service-owned credentials.
use std::{collections::BTreeMap, ffi::OsString, io};

pub(super) const DEFAULT_ENV_VARS: &[&str] = &[
    "HOME",
    "LOGNAME",
    "PATH",
    "SHELL",
    "USER",
    "__CF_USER_TEXT_ENCODING",
    "LANG",
    "LC_ALL",
    "TERM",
    "TMPDIR",
    "TZ",
];

pub(super) const CUSTOM_CA_ENV_KEYS: &[&str] = &[
    "CODEX_CA_CERTIFICATE",
    "SSL_CERT_FILE",
    "REQUESTS_CA_BUNDLE",
    "CURL_CA_BUNDLE",
    "NODE_EXTRA_CA_CERTS",
    "GIT_SSL_CAINFO",
    "CARGO_HTTP_CAINFO",
    "PIP_CERT",
    "BUNDLE_SSL_CA_CERT",
    "npm_config_cafile",
    "NPM_CONFIG_CAFILE",
];

pub(super) const NON_INHERITABLE_ENV_VARS: &[&str] = &[
    "CODEX_EXEC_SERVER_NOISE_AUTH_TOKEN",
    "NODE_REPL_AUTH_TOKEN",
    "CODEX_GUARDIAN_DECISIONS_API_KEY",
    "OPENAI_FEDERATION_RULE_ID",
    "OPENAI_IDENTITY_TOKEN_FILE",
    "OPENAI_WORKLOAD_IDENTITY_CONTEXT",
];

/// The Codex merge algorithm with an explicit parent snapshot instead of global
/// environment reads. Inherited CA paths are made absolute without opening them;
/// explicitly assigned values are kept exactly as supplied.
pub(super) fn create_env_for_mcp_server(
    parent_env: &BTreeMap<OsString, OsString>,
    extra_env: BTreeMap<OsString, OsString>,
) -> io::Result<BTreeMap<OsString, OsString>> {
    let mut env: BTreeMap<OsString, OsString> = DEFAULT_ENV_VARS
        .iter()
        .filter_map(|name| {
            parent_env
                .get(std::ffi::OsStr::new(name))
                .map(|value| (OsString::from(name), value.clone()))
        })
        .collect();
    for name in CUSTOM_CA_ENV_KEYS {
        let Some(value) = parent_env.get(std::ffi::OsStr::new(name)) else {
            continue;
        };
        if value.is_empty() {
            continue;
        }
        let value = std::path::absolute(value)?.into_os_string();
        #[cfg(windows)]
        env.retain(|key, _| !key.to_str().is_some_and(|key| key.eq_ignore_ascii_case(name)));
        env.insert(OsString::from(name), value);
    }
    for (name, value) in extra_env {
        if cfg!(windows)
            || name.to_str().is_some_and(|name| {
                CUSTOM_CA_ENV_KEYS
                    .iter()
                    .any(|ca_name| ca_name.eq_ignore_ascii_case(name))
            })
        {
            env.retain(|key, _| {
                !key.to_str().zip(name.to_str()).is_some_and(|(key, name)| {
                    key.eq_ignore_ascii_case(name)
                })
            });
        }
        env.insert(name, value);
    }
    env.retain(|name, _| {
        name.to_str().is_none_or(|name| {
            !NON_INHERITABLE_ENV_VARS
                .iter()
                .any(|restricted| restricted.eq_ignore_ascii_case(name))
        })
    });
    Ok(env)
}

/// Read only the upstream allowlist, then apply Fetch's assigned settings.
pub(super) fn environment(
    extra_env: BTreeMap<String, String>,
) -> io::Result<BTreeMap<String, String>> {
    let parent_env = DEFAULT_ENV_VARS
        .iter()
        .chain(CUSTOM_CA_ENV_KEYS)
        .filter_map(|name| std::env::var_os(name).map(|value| (OsString::from(name), value)))
        .collect();
    let extra_env = extra_env
        .into_iter()
        .map(|(name, value)| (OsString::from(name), OsString::from(value)))
        .collect();
    into_transport_environment(create_env_for_mcp_server(&parent_env, extra_env)?)
}

/// Fetch's transport currently stores strings. Reject unrepresentable values
/// instead of changing their bytes or including environment values in an error.
pub(super) fn into_transport_environment(
    environment: BTreeMap<OsString, OsString>,
) -> io::Result<BTreeMap<String, String>> {
    environment
        .into_iter()
        .map(|(name, value)| {
            let invalid = || io::Error::new(io::ErrorKind::InvalidData, "MCP environment is not valid UTF-8");
            Ok((name.into_string().map_err(|_| invalid())?, value.into_string().map_err(|_| invalid())?))
        })
        .collect()
}
