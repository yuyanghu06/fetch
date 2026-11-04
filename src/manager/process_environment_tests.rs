use super::process_environment::{create_env_for_mcp_server, CUSTOM_CA_ENV_KEYS, DEFAULT_ENV_VARS, NON_INHERITABLE_ENV_VARS};
use std::{collections::BTreeMap, ffi::{OsStr, OsString}, path::PathBuf};

fn variables(values: &[(&str, &str)]) -> BTreeMap<OsString, OsString> {
    values.iter().map(|(name, value)| ((*name).into(), (*value).into())).collect()
}

#[test]
fn unix_defaults_are_exact_and_unrelated_parent_values_are_not_inherited() {
    let expected = ["HOME", "LOGNAME", "PATH", "SHELL", "USER", "__CF_USER_TEXT_ENCODING", "LANG", "LC_ALL", "TERM", "TMPDIR", "TZ"];
    assert_eq!(DEFAULT_ENV_VARS, expected);
    let mut parent = variables(&[("UNRELATED", "private"), ("SERVICE_TOKEN", "private-token"), ("SERVICE_SECRET", "private-secret"), ("XDG_CONFIG_HOME", "/personal/config")]);
    for key in expected { parent.insert(key.into(), format!("parent-{key}").into()); }
    let actual = create_env_for_mcp_server(&parent, BTreeMap::new()).unwrap();
    assert_eq!(actual.len(), expected.len());
    for key in expected { assert_eq!(actual.get(OsStr::new(key)), parent.get(OsStr::new(key))); }
}

#[test]
fn explicit_assignments_override_defaults_and_allow_service_tokens_and_secrets() {
    let parent = variables(&[("LANG", "parent-language"), ("PATH", "/personal/bin"), ("SERVICE_TOKEN", "unassigned-token"), ("OTHER_SECRET", "unassigned-secret")]);
    let extra = variables(&[("LANG", "assigned-language"), ("PATH", "/private/bin"), ("SERVICE_TOKEN", "assigned-token"), ("SERVICE_SECRET", "assigned-secret")]);
    let actual = create_env_for_mcp_server(&parent, extra.clone()).unwrap();
    assert_eq!(actual, extra);
}

#[test]
fn inherited_certificate_paths_are_absolute_without_reading_or_resolving_files() {
    let expected = ["CODEX_CA_CERTIFICATE", "SSL_CERT_FILE", "REQUESTS_CA_BUNDLE", "CURL_CA_BUNDLE", "NODE_EXTRA_CA_CERTS", "GIT_SSL_CAINFO", "CARGO_HTTP_CAINFO", "PIP_CERT", "BUNDLE_SSL_CA_CERT", "npm_config_cafile", "NPM_CONFIG_CAFILE"];
    assert_eq!(CUSTOM_CA_ENV_KEYS, expected);
    let parent: BTreeMap<OsString, OsString> = expected.iter().map(|key| ((*key).into(), "nonexistent-fetch-test-ca/../certificate.pem".into())).collect();
    let actual = create_env_for_mcp_server(&parent, BTreeMap::new()).unwrap();
    let absolute = std::path::absolute("nonexistent-fetch-test-ca/../certificate.pem").unwrap();
    for key in expected { assert_eq!(PathBuf::from(&actual[OsStr::new(key)]), absolute); }
}

#[test]
fn explicit_certificate_paths_replace_parent_case_variants_and_stay_as_supplied() {
    let parent = variables(&[("SSL_CERT_FILE", "host/certificate.pem"), ("npm_config_cafile", "host/lower.pem"), ("NPM_CONFIG_CAFILE", "host/upper.pem")]);
    let extra = variables(&[("sSl_CeRt_FiLe", "relative/assigned.pem"), ("Npm_Config_Cafile", "relative/npm.pem")]);
    let actual = create_env_for_mcp_server(&parent, extra.clone()).unwrap();
    assert_eq!(actual, extra);
}

#[test]
fn empty_inherited_certificate_path_is_omitted() {
    let actual = create_env_for_mcp_server(&variables(&[("SSL_CERT_FILE", "")]), BTreeMap::new()).unwrap();
    assert!(actual.is_empty());
}

#[test]
fn codex_internal_credentials_are_removed_even_when_explicitly_assigned() {
    let expected = ["CODEX_EXEC_SERVER_NOISE_AUTH_TOKEN", "NODE_REPL_AUTH_TOKEN", "CODEX_GUARDIAN_DECISIONS_API_KEY", "OPENAI_FEDERATION_RULE_ID", "OPENAI_IDENTITY_TOKEN_FILE", "OPENAI_WORKLOAD_IDENTITY_CONTEXT"];
    assert_eq!(NON_INHERITABLE_ENV_VARS, expected);
    let mut parent = BTreeMap::new();
    let mut extra = variables(&[("SERVICE_TOKEN", "assigned-token"), ("SERVICE_SECRET", "assigned-secret")]);
    for key in expected {
        parent.insert(OsString::from(key), OsString::from("parent-private"));
        extra.insert(OsString::from(key.to_ascii_lowercase()), OsString::from("assigned-private"));
        extra.insert(OsString::from(key), OsString::from("assigned-private"));
    }
    let actual = create_env_for_mcp_server(&parent, extra).unwrap();
    assert_eq!(actual, variables(&[("SERVICE_TOKEN", "assigned-token"), ("SERVICE_SECRET", "assigned-secret")]));
}

#[test]
fn ordinary_unix_environment_names_remain_case_sensitive() {
    let actual = create_env_for_mcp_server(&variables(&[("LANG", "host-language"), ("lang", "ignored-parent")]), variables(&[("lang", "assigned-lowercase")])).unwrap();
    assert_eq!(actual, variables(&[("LANG", "host-language"), ("lang", "assigned-lowercase")]));
}

#[cfg(unix)]
#[test]
fn pure_builder_preserves_non_utf8_values_without_lossy_conversion() {
    use std::os::unix::ffi::OsStringExt;
    let non_utf8 = OsString::from_vec(vec![b'x', 0xff]);
    let parent = BTreeMap::from([(OsString::from("USER"), non_utf8.clone())]);
    let extra = BTreeMap::from([(OsString::from("SERVICE_TOKEN"), non_utf8.clone())]);
    let actual = create_env_for_mcp_server(&parent, extra).unwrap();
    assert_eq!(actual[OsStr::new("USER")], non_utf8);
    assert_eq!(actual[OsStr::new("SERVICE_TOKEN")], non_utf8);
    let error=super::process_environment::into_transport_environment(actual).unwrap_err();
    assert_eq!(error.kind(),std::io::ErrorKind::InvalidData);
    assert_eq!(error.to_string(),"MCP environment is not valid UTF-8");
}
