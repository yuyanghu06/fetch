//! Railway's documented native/public OAuth flow and private CLI context.
//! See docs.railway.com/integrations/oauth/{creating-an-app,login-and-tokens}.
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub(super) const AUTHORIZE: &str = "https://backboard.railway.com/oauth/auth";
pub(super) const REGISTER: &str = "https://backboard.railway.com/oauth/register";
pub(super) const TOKEN: &str = "https://backboard.railway.com/oauth/token";
pub(super) const ME: &str = "https://backboard.railway.com/oauth/me";
pub(super) const SCOPES: &str = "openid email profile offline_access workspace:member project:member";

pub(super) fn matches_record(record: &Value) -> bool {
    if record["config"]["kind"] != "remote" {
        return record["config"]["package"]["name"] == "@railway/mcp-server";
    }
    record["config"]["connection"]["url"].as_str().and_then(|s| url::Url::parse(s).ok())
        .is_some_and(|u| u.scheme() == "https" && u.host_str() == Some("mcp.railway.com")
            && u.port_or_known_default() == Some(443) && u.username().is_empty()
            && u.password().is_none() && u.fragment().is_none())
}
pub(super) fn registration(redirect: &str) -> Value {
    json!({"client_name":"Fetch", "application_type":"native", "redirect_uris":[redirect],
        "token_endpoint_auth_method":"none", "grant_types":["authorization_code","refresh_token"],
        "response_types":["code"]})
}
pub(super) fn installation_id(record: &Value) -> Option<&str> {
    record["_installation"]["installation_id"].as_str().filter(|id| id.strip_prefix("package-")
        .is_some_and(|tail| tail.len() == 32 && tail.bytes().all(|b| b.is_ascii_hexdigit())))
}
pub(super) fn cli_bin(root: &Path, record: &Value) -> Option<PathBuf> {
    let directory = root.join("packages/servers").join(installation_id(record)?).join("railway-cli");
    let bin = directory.join("node_modules/.bin");
    let executable = std::fs::canonicalize(bin.join("railway")).ok()?;
    (executable.starts_with(std::fs::canonicalize(&directory).ok()?) && executable.is_file()).then_some(bin)
}
pub(super) fn subject_is_present(value: &Value) -> bool {
    value["sub"].as_str().is_some_and(|sub| !sub.trim().is_empty())
}
