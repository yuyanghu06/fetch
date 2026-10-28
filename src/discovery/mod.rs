//! Read-only, bounded configuration discovery. No command, helper or network request runs here.
use jsonc_parser::{CollectOptions, ParseOptions, ast};
use serde_json::{Map, Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::OpenOptions,
    io::Read,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};
use url::Url;
const MAX_SOURCE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiscoveryError {
    pub code: &'static str,
    pub message: &'static str,
    pub http_status: u16,
}
impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}
impl std::error::Error for DiscoveryError {}
fn error(code: &'static str) -> DiscoveryError {
    let (message, http_status) = match code {
        "INVALID_REQUEST" => ("Choose one absolute configuration file path.", 400),
        "CONFIG_NOT_FOUND" => ("The selected configuration file was not found.", 404),
        "CONFIG_UNREADABLE" => ("The configuration file could not be safely read.", 422),
        "AMBIGUOUS_CONFIG_FILE" => (
            "The selected file contains multiple supported MCP containers.",
            422,
        ),
        "UNSUPPORTED_CONFIG_FILE" => (
            "The selected file does not contain one supported MCP container.",
            422,
        ),
        _ => ("The configuration file could not be safely parsed.", 422),
    };
    DiscoveryError {
        code,
        message,
        http_status,
    }
}

#[derive(Clone)]
pub struct DiscoveryService {
    home: PathBuf,
    codex_home: Option<PathBuf>,
}
impl DiscoveryService {
    pub fn new(home: PathBuf, codex_home: Option<PathBuf>) -> Self {
        Self { home, codex_home }
    }
    pub fn scan(&self, config_path: Option<&Path>) -> Result<Value, DiscoveryError> {
        let mut groups = vec![];
        if let Some(path) = config_path {
            if !path.is_absolute() || path.as_os_str().as_encoded_bytes().contains(&0) {
                return Err(error("INVALID_REQUEST"));
            }
            groups.push(read_group(path, None)?);
        } else {
            let codex = self
                .codex_home
                .as_ref()
                .filter(|p| p.is_absolute() && p.is_dir())
                .cloned()
                .unwrap_or_else(|| self.home.join(".codex"));
            let candidates = [
                (self.home.join(".claude.json"), "claude_code"),
                (codex.join("config.toml"), "codex"),
                (self.home.join(".config/opencode/opencode.json"), "opencode"),
                (
                    self.home.join(".config/opencode/opencode.jsonc"),
                    "opencode",
                ),
            ];
            let multiple = candidates[2].0.exists() && candidates[3].0.exists();
            for (path, harness) in candidates {
                if matches!(std::fs::symlink_metadata(&path), Err(e) if e.kind() == std::io::ErrorKind::NotFound)
                {
                    continue;
                }
                let mut group = match read_group(&path, Some(harness)) {
                    Ok(v) => v,
                    Err(e) => {
                        json!({"path":path,"harness":harness,"status":"failed","error":{"code":e.code,"message":e.message},"servers":[],"warnings":[]})
                    }
                };
                if multiple && harness == "opencode" {
                    group["warnings"] = json!([{"code":"MULTIPLE_CONFIG_FILES","message":"Both OpenCode configuration files exist; Fetch lists them separately."}]);
                }
                groups.push(group);
            }
        }
        let selectable = groups.iter().any(|g| {
            g["servers"]
                .as_array()
                .is_some_and(|a| a.iter().any(|r| r["status"] == "supported"))
        });
        let warnings = groups.iter().any(|g| {
            g["status"] == "failed" || g["warnings"].as_array().is_some_and(|a| !a.is_empty())
        });
        let mut steps = vec![];
        if selectable {
            steps.push(json!({"actor":"agent","action":"review_discovery_results","instructions":"Review supported rows and the user's selection before submitting file-path/name pairs to installation. Source-disabled rows require explicit opt-in."}));
        }
        if warnings {
            steps.push(json!({"actor":"user","action":"review_discovery_warnings","instructions":"Review the configuration warnings. Fetch has not changed the source files or copied credentials."}));
        }
        if !selectable {
            steps.push(json!({"actor":"user","action":"choose_config_file","instructions":"Choose a supported configuration file or correct its blocked definitions. Do not submit an empty installation request."}));
        }
        Ok(json!({"status":"ok","configs":groups,"next_steps":steps}))
    }
}
fn read_group(path: &Path, expected: Option<&str>) -> Result<Value, DiscoveryError> {
    let mut options = OpenOptions::new();
    options
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    let file = options.open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            error("CONFIG_NOT_FOUND")
        } else {
            error("CONFIG_UNREADABLE")
        }
    })?;
    let metadata = file.metadata().map_err(|_| error("CONFIG_UNREADABLE"))?;
    if !metadata.is_file() || metadata.len() > MAX_SOURCE_BYTES as u64 {
        return Err(error("CONFIG_UNREADABLE"));
    }
    let mut bytes = vec![];
    file.take(MAX_SOURCE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| error("CONFIG_UNREADABLE"))?;
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(error("CONFIG_UNREADABLE"));
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| error("INVALID_CONFIG_FILE"))?;
    let (value, format) = parse_document(text, expected)?;
    let object = value
        .as_object()
        .ok_or_else(|| error("UNSUPPORTED_CONFIG_FILE"))?;
    let names = [
        ("mcpServers", "claude_code"),
        ("mcp_servers", "codex"),
        ("mcp", "opencode"),
    ];
    let (container, harness) = if let Some(h) = expected {
        (names.iter().find(|(_, x)| *x == h).unwrap().0, h)
    } else {
        let found: Vec<_> = names
            .iter()
            .filter(|(n, _)| object.contains_key(*n))
            .collect();
        if found.len() > 1 {
            return Err(error("AMBIGUOUS_CONFIG_FILE"));
        }
        let Some((name, harness)) = found.first().copied() else {
            return Err(error("UNSUPPORTED_CONFIG_FILE"));
        };
        (*name, *harness)
    };
    // A JSON spelling of a TOML container is not a supported custom Codex file.
    if (harness == "codex" && format != "toml")
        || (harness == "claude_code" && format != "json")
        || (harness == "opencode" && format == "toml")
    {
        return Err(error("UNSUPPORTED_CONFIG_FILE"));
    }
    let empty = Map::new();
    let map = match object.get(container) {
        Some(v) => v
            .as_object()
            .ok_or_else(|| error("UNSUPPORTED_CONFIG_FILE"))?,
        None if expected.is_some() => &empty,
        None => return Err(error("UNSUPPORTED_CONFIG_FILE")),
    };
    let mut servers: Vec<Value> = map
        .iter()
        .map(|(name, definition)| classify(name, definition, harness))
        .collect();
    servers.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    Ok(
        json!({"path":path,"harness":harness,"status":"ok","error":null,"servers":servers,"warnings":[]}),
    )
}
fn parse_document(
    text: &str,
    expected: Option<&str>,
) -> Result<(Value, &'static str), DiscoveryError> {
    if expected != Some("codex") {
        if let Ok(v) = parse_json(text, false) {
            return Ok((v, "json"));
        }
        if expected != Some("claude_code")
            && let Ok(v) = parse_json(text, true)
        {
            return Ok((v, "jsonc"));
        }
    }
    if (expected.is_none() || expected == Some("codex"))
        && let Ok(v) = toml::from_str::<toml::Value>(text)
    {
        let v = serde_json::to_value(v).map_err(|_| error("INVALID_CONFIG_FILE"))?;
        return Ok((v, "toml"));
    }
    Err(error("INVALID_CONFIG_FILE"))
}
fn parse_json(text: &str, comments: bool) -> Result<Value, DiscoveryError> {
    let options = ParseOptions {
        allow_comments: comments,
        allow_trailing_commas: comments,
        allow_loose_object_property_names: false,
        allow_missing_commas: false,
        allow_single_quoted_strings: false,
        allow_hexadecimal_numbers: false,
        allow_unary_plus_numbers: false,
        allow_bare_decimal_point_numbers: false,
        allow_non_finite_numbers: false,
        allow_extended_string_escapes: false,
    };
    let parsed = jsonc_parser::parse_to_ast(text, &CollectOptions::default(), &options)
        .map_err(|_| error("INVALID_CONFIG_FILE"))?;
    ast_value(parsed.value.ok_or_else(|| error("INVALID_CONFIG_FILE"))?, 0)
}
fn ast_value(v: ast::Value<'_>, depth: usize) -> Result<Value, DiscoveryError> {
    if depth > 128 {
        return Err(error("INVALID_CONFIG_FILE"));
    }
    Ok(match v {
        ast::Value::Object(o) => {
            let mut result = Map::new();
            for p in o.properties {
                let key = p.name.into_string();
                if result.contains_key(&key) {
                    return Err(error("INVALID_CONFIG_FILE"));
                }
                result.insert(key, ast_value(p.value, depth + 1)?);
            }
            Value::Object(result)
        }
        ast::Value::Array(a) => Value::Array(
            a.elements
                .into_iter()
                .map(|v| ast_value(v, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        ast::Value::StringLit(s) => Value::String(s.value.into_owned()),
        ast::Value::NumberLit(n) => {
            serde_json::from_str(n.value).map_err(|_| error("INVALID_CONFIG_FILE"))?
        }
        ast::Value::BooleanLit(b) => Value::Bool(b.value),
        ast::Value::NullKeyword(_) => Value::Null,
    })
}
fn empty_row(name: &str) -> Value {
    json!({"name":name,"status":"unsupported","type":null,"package":null,"version_spec":null,"executable":null,"script_path":null,"working_directory":null,"credential_variables":[],"source_enabled":null,"selected_by_default":false,"error":null,"connection":null})
}
fn blocked(mut row: Value, invalid: bool) -> Value {
    row["status"] = json!(if invalid { "invalid" } else { "unsupported" });
    row["selected_by_default"] = json!(false);
    row["error"] = json!({"code":if invalid{"INVALID_SERVER_DEFINITION"}else{"UNSUPPORTED_SERVER_DEFINITION"},"message":if invalid{"This entry is not a valid supported server definition."}else{"This launch form needs a supported literal configuration; Fetch did not run or resolve it."}});
    row
}
fn literal(s: &str) -> bool {
    !s.is_empty() && !s.contains(['\0', '$', '{', '}', '\n', '\r'])
}
fn variable(s: &str) -> bool {
    let mut b = s.bytes();
    b.next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == b'_')
        && b.all(|c| c.is_ascii_alphanumeric() || c == b'_')
}
fn classify(name: &str, definition: &Value, harness: &str) -> Value {
    let mut row = empty_row(name);
    if name.trim().is_empty() {
        return blocked(row, true);
    }
    let Some(o) = definition.as_object() else {
        return blocked(row, true);
    };
    let mut variables = BTreeSet::new();
    for key in ["env", "environment"] {
        if let Some(v) = o.get(key) {
            let Some(map) = v.as_object() else {
                return blocked(row, true);
            };
            for (k, v) in map {
                if !variable(k) || !v.is_string() {
                    return blocked(row, true);
                }
                variables.insert(k.clone());
            }
        }
    }
    for key in ["env_vars", "bearer_token_env_var"] {
        if let Some(v) = o.get(key) {
            match v {
                Value::String(s) if variable(s) => {
                    variables.insert(s.clone());
                }
                Value::Array(a) => {
                    for s in a {
                        let Some(s) = s.as_str().filter(|s| variable(s)) else {
                            return blocked(row, true);
                        };
                        variables.insert(s.to_owned());
                    }
                }
                _ => return blocked(row, true),
            }
        }
    }
    row["credential_variables"] = json!(variables);
    if o.get("type").is_some_and(|v| !v.is_string()) {
        return blocked(row, true);
    }
    let enabled = match (o.get("enabled"), o.get("disabled")) {
        (Some(Value::Bool(e)), None) => Some(*e),
        (None, Some(Value::Bool(d))) => Some(!d),
        (None, None) => None,
        _ => return blocked(row, true),
    };
    row["source_enabled"] = json!(enabled);
    let allowed = [
        "type",
        "command",
        "args",
        "env",
        "environment",
        "env_vars",
        "bearer_token_env_var",
        "cwd",
        "working_directory",
        "enabled",
        "disabled",
        "url",
        "headers",
        "http_headers",
        "startup_timeout_sec",
        "tool_timeout_sec",
        "timeout",
    ];
    if o.keys().any(|k| !allowed.contains(&k.as_str())) {
        return blocked(row, false);
    }
    let working = o.get("cwd").or_else(|| o.get("working_directory"));
    if let Some(w) = working {
        let Some(w) = w.as_str() else {
            return blocked(row, true);
        };
        if !literal(w) || !Path::new(w).is_absolute() {
            return blocked(row, false);
        }
        row["working_directory"] = json!(w);
    }
    if o.contains_key("cwd") && o.contains_key("working_directory") {
        return blocked(row, false);
    }
    if let Some(url) = o.get("url") {
        if working.is_some()
            || o.contains_key("command")
            || o.contains_key("args")
            || o.contains_key("env")
            || o.contains_key("environment")
        {
            return blocked(row, false);
        }
        let Some(text) = url.as_str() else {
            return blocked(row, true);
        };
        let Ok(url) = Url::parse(text) else {
            return blocked(row, false);
        };
        if !literal(text)
            || !matches!(url.scheme(), "http" | "https")
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.host_str().is_none()
        {
            return blocked(row, false);
        }
        let transport = match o.get("type").and_then(Value::as_str) {
            Some("sse") => "sse",
            Some("http" | "streamable_http" | "remote") | None => "streamable_http",
            _ => return blocked(row, false),
        };
        let mut headers = BTreeMap::new();
        for key in ["headers", "http_headers"] {
            if let Some(v) = o.get(key) {
                let Some(h) = v.as_object() else {
                    return blocked(row, true);
                };
                for (name, value) in h {
                    if !header_name(name) || !value.is_string() {
                        return blocked(row, true);
                    }
                    if value.as_str().is_some_and(|v| v.contains(['$', '{', '}'])) {
                        return blocked(row, false);
                    }
                    headers
                        .entry(name.to_ascii_lowercase())
                        .or_insert_with(|| name.clone());
                }
            }
        }
        row["type"] = json!("remote");
        row["connection"] = json!({"transport":transport,"origin":url.origin().ascii_serialization(),"header_names":headers.into_values().collect::<Vec<_>>()});
    } else {
        if o.contains_key("headers")
            || o.contains_key("http_headers")
            || o.contains_key("bearer_token_env_var")
        {
            return blocked(row, false);
        }
        if let Some(t) = o.get("type")
            && !matches!(t.as_str(), Some("stdio" | "local"))
        {
            return blocked(row, false);
        }
        let (command, args) = if harness == "opencode" {
            let Some(c) = o.get("command").and_then(Value::as_array) else {
                return blocked(row, true);
            };
            if c.is_empty() || o.contains_key("args") {
                return blocked(row, true);
            };
            let Some(parts) = c.iter().map(Value::as_str).collect::<Option<Vec<_>>>() else {
                return blocked(row, true);
            };
            (
                parts[0].to_owned(),
                parts[1..]
                    .iter()
                    .map(|x| (*x).to_owned())
                    .collect::<Vec<_>>(),
            )
        } else {
            let Some(c) = o.get("command").and_then(Value::as_str) else {
                return blocked(row, true);
            };
            let args = match o.get("args") {
                None => vec![],
                Some(Value::Array(a)) => {
                    let Some(a) = a
                        .iter()
                        .map(Value::as_str)
                        .map(|x| x.map(str::to_owned))
                        .collect::<Option<Vec<_>>>()
                    else {
                        return blocked(row, true);
                    };
                    a
                }
                _ => return blocked(row, true),
            };
            (c.to_owned(), args)
        };
        if !literal(&command) || args.iter().any(|a| !literal(a)) {
            return blocked(row, false);
        }
        if command == "npx" {
            let args = if args.first().is_some_and(|s| s == "-y" || s == "--yes") {
                &args[1..]
            } else {
                &args[..]
            };
            if args.len() != 1 {
                return blocked(row, false);
            }
            let Some((package, version)) = npm_package(&args[0]) else {
                return blocked(row, false);
            };
            row["type"] = json!("npm");
            row["package"] = json!(package);
            row["version_spec"] = json!(version);
        } else if command == "uvx" {
            if args.len() != 1 {
                return blocked(row, false);
            }
            let Some((package, version)) = python_package(&args[0]) else {
                return blocked(row, false);
            };
            row["type"] = json!("pypi");
            row["package"] = json!(package);
            row["version_spec"] = json!(version);
        } else if matches!(command.as_str(), "node" | "python" | "python3")
            || (Path::new(&command).is_absolute()
                && matches!(
                    Path::new(&command).file_name().and_then(|s| s.to_str()),
                    Some("node" | "python" | "python3")
                ))
        {
            if args.len() != 1 || !Path::new(&args[0]).is_absolute() {
                return blocked(row, false);
            }
            row["type"] = json!("local_script");
            row["executable"] = json!(command);
            row["script_path"] = json!(args[0]);
        } else if Path::new(&command).is_absolute() && args.is_empty() {
            row["type"] = json!("local_script");
            row["executable"] = json!(command);
            row["script_path"] = json!(command);
        } else {
            return blocked(row, false);
        }
    }
    row["status"] = json!("supported");
    row["selected_by_default"] = json!(enabled != Some(false));
    row
}
fn header_name(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&c))
}
fn npm_package(s: &str) -> Option<(&str, Option<&str>)> {
    let split = if let Some(unscoped) = s.strip_prefix('@') {
        unscoped.rfind('@').map(|i| i + 1)
    } else {
        s.rfind('@')
    };
    let (name, version) = if let Some(i) = split {
        (&s[..i], Some(&s[i + 1..]))
    } else {
        (s, None)
    };
    let parts: Vec<_> = name.split('/').collect();
    let valid = if name.starts_with('@') {
        parts.len() == 2
            && parts[0].len() > 1
            && package_word(&parts[0][1..])
            && package_word(parts[1])
    } else {
        parts.len() == 1 && package_word(name)
    };
    if !valid
        || version.is_some_and(|v| {
            v.is_empty()
                || !v
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"._-^~*+".contains(&c))
        })
    {
        return None;
    }
    Some((name, version))
}
fn python_package(s: &str) -> Option<(&str, Option<&str>)> {
    let (name, version) = match s.split_once("==") {
        Some((n, v)) => (n, Some(v)),
        None => (s, None),
    };
    if !package_word(name)
        || version.is_some_and(|v| {
            v.is_empty()
                || !v
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"._-+*".contains(&c))
        })
    {
        return None;
    }
    Some((name, version))
}
fn package_word(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with(['.', '-', '_'])
        && s.bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._-".contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn write(path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    #[test]
    fn defaults_partial_sources_disabled_rows_and_secret_values() {
        let t = tempfile::tempdir().unwrap();
        write(
            &t.path().join(".claude.json"),
            r#"{"mcpServers":{"safe":{"command":"npx","args":["-y","@example/server@1.2.3"],"env":{"API_KEY":"secret-never-return"}},"disabled":{"command":"uvx","args":["example-server==2.0"],"disabled":true},"bad":false},"projects":{"x":{"mcpServers":{"ignore":{}}}}}"#,
        );
        write(&t.path().join(".codex/config.toml"), "bad = [");
        let v = DiscoveryService::new(t.path().to_owned(), None)
            .scan(None)
            .unwrap();
        assert_eq!(v["configs"].as_array().unwrap().len(), 2);
        assert_eq!(v["configs"][0]["servers"][0]["status"], "invalid");
        assert_eq!(v["configs"][0]["servers"][1]["selected_by_default"], false);
        assert_eq!(v["configs"][1]["status"], "failed");
        let text = v.to_string();
        assert!(!text.contains("secret-never-return"));
        assert!(!text.contains("ignore"));
        assert_eq!(
            v["configs"][0]["servers"][2]["credential_variables"],
            json!(["API_KEY"])
        );
    }
    #[test]
    fn jsonc_duplicate_decoded_keys_and_json5_are_rejected() {
        let t = tempfile::tempdir().unwrap();
        let p = t.path().join("config.jsonc");
        write(
            &p,
            r#"{"mcp": {/* comment */ "a":{"type":"local","command":["npx","pkg"]},},}"#,
        );
        let s = DiscoveryService::new(t.path().to_owned(), None);
        assert_eq!(
            s.scan(Some(&p)).unwrap()["configs"][0]["servers"][0]["status"],
            "supported"
        );
        for text in [
            r#"{"mcp":{"a":{},"\u0061":{}}}"#,
            r#"{mcp:{}}"#,
            r#"{'mcp':{}}"#,
            r#"{"mcp":{} "other":1}"#,
        ] {
            write(&p, text);
            assert!(s.scan(Some(&p)).is_err(), "{text}");
        }
    }
    #[test]
    fn remote_metadata_hides_identity_secrets_and_header_values() {
        let t = tempfile::tempdir().unwrap();
        let p = t.path().join("config.json");
        write(
            &p,
            r#"{"mcpServers":{"remote":{"type":"http","url":"https://example.com/private/path","headers":{"authorization":"secret-header","Authorization":"second-secret","X-Example":"hidden"}},"unsafe":{"url":"https://user:password@example.com/mcp?token=hidden"}}}"#,
        );
        let v = DiscoveryService::new(t.path().to_owned(), None)
            .scan(Some(&p))
            .unwrap();
        let a = &v["configs"][0]["servers"][0];
        assert_eq!(a["connection"]["origin"], "https://example.com");
        assert_eq!(a["connection"]["header_names"].as_array().unwrap().len(), 2);
        let text = v.to_string();
        for secret in [
            "private/path",
            "secret-header",
            "second-secret",
            "password",
            "token=hidden",
        ] {
            assert!(!text.contains(secret))
        }
        assert_eq!(v["configs"][0]["servers"][1]["status"], "unsupported");
    }
    #[test]
    fn explicit_missing_ambiguous_and_nested_projects_are_distinct() {
        let t = tempfile::tempdir().unwrap();
        let p = t.path().join("config");
        let s = DiscoveryService::new(t.path().to_owned(), None);
        assert_eq!(s.scan(Some(&p)).unwrap_err().code, "CONFIG_NOT_FOUND");
        write(&p, r#"{"mcp":{},"mcpServers":{}}"#);
        assert_eq!(s.scan(Some(&p)).unwrap_err().code, "AMBIGUOUS_CONFIG_FILE");
        write(&p, r#"{"projects":{"mcpServers":{}}}"#);
        assert_eq!(
            s.scan(Some(&p)).unwrap_err().code,
            "UNSUPPORTED_CONFIG_FILE"
        );
        assert_eq!(
            s.scan(Some(Path::new("relative"))).unwrap_err().code,
            "INVALID_REQUEST"
        );
    }
    #[test]
    fn toml_and_both_opencode_sources_remain_separate() {
        let t = tempfile::tempdir().unwrap();
        write(
            &t.path().join(".codex/config.toml"),
            "[mcp_servers.same]\ncommand = 'uvx'\nargs = ['example-server']\nenv_vars = ['API_KEY']\n",
        );
        for name in ["opencode.json", "opencode.jsonc"] {
            write(
                &t.path().join(".config/opencode").join(name),
                r#"{"mcp":{"same":{"type":"local","command":["npx","example-server"]}}}"#,
            );
        }
        let v = DiscoveryService::new(t.path().to_owned(), None)
            .scan(None)
            .unwrap();
        assert_eq!(v["configs"].as_array().unwrap().len(), 3);
        assert_eq!(v["configs"][0]["servers"][0]["type"], "pypi");
        assert_eq!(
            v["configs"][1]["warnings"][0]["code"],
            "MULTIPLE_CONFIG_FILES"
        );
        assert_eq!(
            v["configs"][2]["warnings"][0]["code"],
            "MULTIPLE_CONFIG_FILES"
        );
    }
    #[test]
    fn source_read_bounds_and_wrong_types_do_not_become_supported() {
        let t = tempfile::tempdir().unwrap();
        let p = t.path().join("source.json");
        let service = DiscoveryService::new(t.path().to_owned(), None);
        std::fs::write(&p, vec![b' '; MAX_SOURCE_BYTES + 1]).unwrap();
        assert_eq!(
            service.scan(Some(&p)).unwrap_err().code,
            "CONFIG_UNREADABLE"
        );
        write(
            &p,
            r#"{"mcpServers":{"":{"command":"npx","args":["pkg"]},"bad":{"type":false,"url":"https://example.com/mcp"}}}"#,
        );
        let result = service.scan(Some(&p)).unwrap();
        assert!(
            result["configs"][0]["servers"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row["status"] == "invalid")
        );
        for text in ["[mcp]\n", "[mcpServers]\n", r#"{"mcp_servers":{}}"#] {
            write(&p, text);
            assert_eq!(
                service.scan(Some(&p)).unwrap_err().code,
                "UNSUPPORTED_CONFIG_FILE"
            );
        }
        write(&p, "[mcp_servers.a]\ncommand='npx'\ncommand='uvx'\n");
        assert_eq!(
            service.scan(Some(&p)).unwrap_err().code,
            "INVALID_CONFIG_FILE"
        );
    }
    #[test]
    fn unreadable_default_ancestors_remain_failed_sources() {
        use std::os::unix::fs::PermissionsExt;
        // Root can bypass fixture permissions, so this OS-level case is not meaningful there.
        // SAFETY: geteuid has no arguments or memory side effects.
        if unsafe { libc::geteuid() } == 0 {
            return;
        }
        let t = tempfile::tempdir().unwrap();
        let directory = t.path().join(".config");
        std::fs::create_dir(&directory).unwrap();
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o000)).unwrap();
        let result = DiscoveryService::new(t.path().to_owned(), None).scan(None);
        // Restore before any assertion so ordinary temporary-directory cleanup remains possible.
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
        let result = result.unwrap();
        let sources = result["configs"].as_array().unwrap();
        assert_eq!(sources.len(), 2);
        assert!(
            sources.iter().all(|source| source["status"] == "failed"
                && source["error"]["code"] == "CONFIG_UNREADABLE")
        );
    }
}
