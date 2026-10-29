//! Generic remote OAuth discovery. The MCP transport remains owned by the runner.
//!
//! The sequence follows the official MCP Rust/TypeScript SDK auth designs:
//! protected-resource metadata, published issuer metadata, public registration,
//! PKCE and an exact resource/issuer binding. No legacy endpoints are invented.
use super::{ManagementError, auth_required, bounded_json};
use crate::diagnostics::{UpstreamOperation, send};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{net::{IpAddr, SocketAddr}, time::Duration};
use url::Url;

const MAX_METADATA_URL: usize = 8192;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Metadata {
    pub issuer: String,
    pub authorization: String,
    pub token: String,
    pub registration: String,
    pub resource: String,
    pub scope: Option<String>,
    pub authorization_response_iss_parameter_supported: bool,
}

impl Metadata {
    /// Saved metadata is encrypted together with the registration and grant.
    /// It must still match this server before any refresh token is transmitted.
    pub fn from_saved(value: &Value, endpoint: &str) -> Result<Self, ManagementError> {
        let metadata: Self = serde_json::from_value(value.clone()).map_err(|_| auth_required())?;
        let endpoint = checked_url(endpoint)?;
        let resource = checked_url(&metadata.resource)?;
        if !resource_covers(&resource, &endpoint) { return Err(auth_required()); }
        for value in [&metadata.issuer, &metadata.authorization, &metadata.token, &metadata.registration] {
            checked_url(value)?;
        }
        if !valid_scope(metadata.scope.as_deref()) || !metadata.authorization_response_iss_parameter_supported { return Err(auth_required()); }
        Ok(metadata)
    }

    /// The code and its PKCE verifier must never be sent to a token endpoint
    /// before the callback proves which authorization issuer produced them.
    pub fn validate_response_issuer(&self, response_issuer: Option<&str>) -> Result<(), ManagementError> {
        if !self.authorization_response_iss_parameter_supported || response_issuer != Some(self.issuer.as_str()) {
            return Err(ManagementError::new(400, "AUTHENTICATION_FAILED", "The provider response did not identify the expected authorization issuer."));
        }
        Ok(())
    }
}

pub(super) async fn discover(endpoint: &str) -> Result<Metadata, ManagementError> {
    let endpoint = checked_url(endpoint)?;
    // GET is read-only. No configured headers, saved token, or cookies are sent
    // while learning where this resource publishes its authentication metadata.
    let response = send(UpstreamOperation::ProviderMetadata, client_for(&endpoint).await?
        .get(endpoint.clone()).header("Accept", "application/json, text/event-stream")).await
        .map_err(|_| auth_required())?;
    let challenge = if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        let mut parsed = None;
        for header in response.headers().get_all(reqwest::header::WWW_AUTHENTICATE) {
            let value = header.to_str().map_err(|_| auth_required())?;
            if let Some(value) = parse_bearer_challenge(value)? {
                if parsed.is_some() { return Err(auth_required()); }
                parsed = Some(value);
            }
        }
        parsed
    } else { None };
    drop(response);
    let scope = challenge.as_ref().and_then(|value| value.1.clone());
    let mut candidates = Vec::new();
    if let Some((Some(url), _)) = challenge { candidates.push(checked_url(&url)?); }
    for candidate in metadata_candidates(&endpoint, "oauth-protected-resource") {
        if !candidates.contains(&candidate) { candidates.push(candidate); }
    }
    let mut resource_metadata = None;
    for candidate in candidates {
        match optional_metadata(&candidate).await? {
            Some(value) => {
                let resource = checked_url(value["resource"].as_str().ok_or_else(auth_required)?)?;
                if !resource_covers(&resource, &endpoint) { return Err(auth_required()); }
                let issuers = value["authorization_servers"].as_array().filter(|v| !v.is_empty() && v.len() <= 8)
                    .ok_or_else(auth_required)?;
                // The first issuer is the resource owner's stated preference.
                // Failure does not silently switch the user to another authority.
                let issuer = checked_url(issuers[0].as_str().ok_or_else(auth_required)?)?;
                resource_metadata = Some((resource, issuer)); break;
            },
            None => {},
        }
    }
    // Older conforming servers can publish RFC 8414 on their own origin without
    // protected-resource metadata. We still require a real metadata document.
    let (resource, issuer) = resource_metadata.unwrap_or_else(|| {
        let mut issuer = endpoint.clone(); issuer.set_path("/"); issuer.set_query(None);
        (endpoint.clone(), issuer)
    });
    for candidate in issuer_metadata_candidates(&issuer) {
        if let Some(value) = optional_metadata(&candidate).await? {
            return parse_metadata(&value, &issuer, &resource, scope);
        }
    }
    Err(auth_required())
}

fn parse_metadata(value: &Value, expected_issuer: &Url, resource: &Url, scope: Option<String>) -> Result<Metadata, ManagementError> {
    let issuer = checked_url(value["issuer"].as_str().ok_or_else(auth_required)?)?;
    if issuer != *expected_issuer { return Err(auth_required()); }
    if !value["code_challenge_methods_supported"].as_array().is_some_and(|a| a.iter().any(|v| v == "S256"))
        || value["response_types_supported"].as_array().is_some_and(|a| !a.iter().any(|v| v == "code"))
        || value["grant_types_supported"].as_array().is_some_and(|a| !a.iter().any(|v| v == "authorization_code"))
        || value["token_endpoint_auth_methods_supported"].as_array().is_some_and(|a| !a.iter().any(|v| v == "none"))
        || value["authorization_response_iss_parameter_supported"] != true
        || !valid_scope(scope.as_deref()) { return Err(auth_required()); }
    Ok(Metadata {
        // Keep the issuer's exact published spelling for the RFC 9207 callback
        // comparison. URL normalization must not weaken the equality check.
        issuer: value["issuer"].as_str().ok_or_else(auth_required)?.to_owned(),
        authorization: checked_url(value["authorization_endpoint"].as_str().ok_or_else(auth_required)?)?.to_string(),
        token: checked_url(value["token_endpoint"].as_str().ok_or_else(auth_required)?)?.to_string(),
        registration: checked_url(value["registration_endpoint"].as_str().ok_or_else(auth_required)?)?.to_string(),
        resource: resource.to_string(), scope, authorization_response_iss_parameter_supported: true,
    })
}

pub(super) async fn register(metadata: &Metadata, redirect: &str) -> Result<Value, ManagementError> {
    let endpoint = checked_url(&metadata.registration)?;
    let response = send(UpstreamOperation::ProviderRegistration, client_for(&endpoint).await?.post(endpoint)
        .json(&json!({"client_name":"Fetch", "redirect_uris":[redirect],
            "token_endpoint_auth_method":"none", "grant_types":["authorization_code","refresh_token"],
            "response_types":["code"]}))).await.map_err(|_| auth_required())?;
    let registration = bounded_json(response).await?;
    if registration.get("client_secret").is_some_and(|v| !v.is_null() && v != "")
        || registration.get("token_endpoint_auth_method").is_some_and(|v| v != "none")
        || registration.get("redirect_uris").is_some_and(|v| v.as_array().is_none_or(|a| !a.iter().any(|v| v == redirect))) {
        return Err(auth_required());
    }
    Ok(registration)
}

pub(super) async fn token(metadata: &Metadata, fields: &[(&str, &str)], refresh: bool) -> Result<Value, ManagementError> {
    let endpoint = checked_url(&metadata.token)?;
    let response = send(if refresh { UpstreamOperation::ProviderRefresh } else { UpstreamOperation::ProviderTokenExchange },
        client_for(&endpoint).await?.post(endpoint).form(fields)).await.map_err(|_| auth_required())?;
    bounded_json(response).await
}

pub(super) async fn validate_browser_destination(value: &str) -> Result<(), ManagementError> {
    client_for(&checked_url(value)?).await.map(|_| ())
}

async fn optional_metadata(endpoint: &Url) -> Result<Option<Value>, ManagementError> {
    let response = send(UpstreamOperation::ProviderMetadata, client_for(endpoint).await?.get(endpoint.clone())
        .header("Accept", "application/json")).await.map_err(|_| auth_required())?;
    if matches!(response.status(), reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::METHOD_NOT_ALLOWED) {
        return Ok(None);
    }
    bounded_json(response).await.map(Some)
}

/// DNS answers are checked and pinned on the very client that makes this one
/// request, closing the resolve/check/connect gap and ignoring inherited proxies.
async fn client_for(endpoint: &Url) -> Result<reqwest::Client, ManagementError> {
    let host = endpoint.host_str().ok_or_else(auth_required)?;
    let port = endpoint.port_or_known_default().ok_or_else(auth_required)?;
    let addresses: Vec<SocketAddr> = tokio::time::timeout(Duration::from_secs(5), tokio::net::lookup_host((host, port))).await
        .map_err(|_| auth_required())?.map_err(|_| auth_required())?.take(33).collect();
    if addresses.is_empty() || addresses.len() > 32 || addresses.iter().any(|a| !public_address(a.ip())) { return Err(auth_required()); }
    reqwest::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(20))
        .resolve_to_addrs(host, &addresses).build().map_err(|_| auth_required())
}

fn checked_url(value: &str) -> Result<Url, ManagementError> {
    if value.len() > MAX_METADATA_URL || value.chars().any(char::is_control) { return Err(auth_required()); }
    let url = Url::parse(value).map_err(|_| auth_required())?;
    let host = url.host_str().ok_or_else(auth_required)?;
    if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some()
        || host.eq_ignore_ascii_case("localhost") || host.ends_with(".localhost") || host.ends_with(".local")
        || matches!(url.host(), Some(url::Host::Ipv4(ip)) if !public_address(IpAddr::V4(ip)))
        || matches!(url.host(), Some(url::Host::Ipv6(ip)) if !public_address(IpAddr::V6(ip))) { return Err(auth_required()); }
    Ok(url)
}

fn public_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            let [a,b,c,_] = ip.octets();
            !(a == 0 || a == 10 || a == 127 || a >= 224 || (a == 100 && (64..=127).contains(&b))
                || (a == 169 && b == 254) || (a == 172 && (16..=31).contains(&b)) || (a == 192 && b == 168)
                || (a == 192 && b == 0 && c == 0) || (a == 192 && b == 0 && c == 2)
                || (a == 198 && (b == 18 || b == 19 || (b == 51 && c == 100)))
                || (a == 203 && b == 0 && c == 113))
        },
        IpAddr::V6(ip) => {
            if let Some(mapped) = ip.to_ipv4_mapped() { return public_address(IpAddr::V4(mapped)); }
            let segments = ip.segments();
            // Public IPv6 global unicast only; exclude documentation, Teredo,
            // 6to4 and special-purpose protocol assignments as well.
            (segments[0] & 0xe000) == 0x2000 && !(segments[0] == 0x2001 && segments[1] <= 0x1ff)
                && !(segments[0] == 0x2001 && segments[1] == 0xdb8) && segments[0] != 0x2002
                && !(segments[0] == 0x3fff && segments[1] <= 0x0fff)
        },
    }
}

fn resource_covers(resource: &Url, endpoint: &Url) -> bool {
    if resource.origin() != endpoint.origin() { return false; }
    if resource.query().is_some() && resource.query() != endpoint.query() { return false; }
    let prefix = resource.path().trim_end_matches('/');
    endpoint.path() == resource.path() || prefix.is_empty() || endpoint.path().strip_prefix(prefix).is_some_and(|tail| tail.starts_with('/'))
}

fn metadata_candidates(base: &Url, kind: &str) -> Vec<Url> {
    let mut origin = base.clone(); origin.set_query(None); origin.set_fragment(None);
    let mut candidates = Vec::new();
    if base.path() != "/" {
        let mut path = origin.clone(); path.set_path(&format!("/.well-known/{kind}{}", base.path())); candidates.push(path);
    }
    origin.set_path(&format!("/.well-known/{kind}")); candidates.push(origin); candidates
}
fn issuer_metadata_candidates(issuer: &Url) -> Vec<Url> {
    let mut candidates = metadata_candidates(issuer, "oauth-authorization-server");
    candidates.retain(|url| issuer.path() == "/" || url.path() != "/.well-known/oauth-authorization-server");
    let mut oidc = issuer.clone(); oidc.set_query(None);
    oidc.set_path(&format!("{}/.well-known/openid-configuration", issuer.path().trim_end_matches('/')));
    candidates.push(oidc); candidates
}
fn valid_scope(scope: Option<&str>) -> bool {
    scope.is_none_or(|value| !value.is_empty() && value.len() <= 4096 && value.bytes().all(|b| b == b' ' || (0x21..=0x7e).contains(&b) && b != b'"' && b != b'\\'))
}

fn parse_bearer_challenge(value: &str) -> Result<Option<(Option<String>, Option<String>)>, ManagementError> {
    let Some((scheme, fields)) = value.trim().split_once(' ') else { return Ok(None); };
    if !scheme.eq_ignore_ascii_case("Bearer") { return Ok(None); }
    if fields.len() > 16 * 1024 { return Err(auth_required()); }
    let mut metadata = None; let mut scope = None;
    // Quoted-string parsing keeps commas inside a quoted URI or scope value.
    let mut rest = fields.trim();
    while !rest.is_empty() {
        let (name, tail) = rest.split_once('=').ok_or_else(auth_required)?;
        let name = name.trim(); let tail = tail.trim_start();
        let (field, remaining) = if let Some(tail) = tail.strip_prefix('"') {
            let end = tail.find('"').ok_or_else(auth_required)?;
            let field = &tail[..end];
            if field.contains(['\\', '\r', '\n', '\0']) { return Err(auth_required()); }
            (field, tail[end + 1..].trim_start())
        } else { tail.split_once(',').map(|(v,r)| (v.trim(), r)).unwrap_or((tail.trim(), "")) };
        match name.to_ascii_lowercase().as_str() {
            "resource_metadata" => { if metadata.replace(field.to_owned()).is_some() { return Err(auth_required()); } },
            "scope" => { if scope.replace(field.to_owned()).is_some() { return Err(auth_required()); } },
            _ => {},
        }
        rest = if remaining.is_empty() { "" } else if tail.starts_with('"') {
            remaining.strip_prefix(',').ok_or_else(auth_required)?.trim_start()
        } else { remaining.trim_start() };
    }
    if !valid_scope(scope.as_deref()) { return Err(auth_required()); }
    Ok(Some((metadata, scope)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata() -> Value { json!({"issuer":"https://auth.example.com/tenant", "authorization_endpoint":"https://auth.example.com/authorize",
        "token_endpoint":"https://auth.example.com/token", "registration_endpoint":"https://auth.example.com/register",
        "code_challenge_methods_supported":["S256"], "token_endpoint_auth_methods_supported":["none"],
        "authorization_response_iss_parameter_supported":true}) }

    #[test]
    fn discovery_requires_published_matching_issuer_pkce_and_public_client() {
        let issuer = Url::parse("https://auth.example.com/tenant").unwrap(); let resource = Url::parse("https://resource.example.com/mcp").unwrap();
        assert!(parse_metadata(&metadata(), &issuer, &resource, None).is_ok());
        for (key, value) in [("issuer", json!("https://other.example.com/tenant")), ("code_challenge_methods_supported", json!(["plain"])),
            ("token_endpoint_auth_methods_supported", json!(["client_secret_basic"])), ("token_endpoint", json!("http://127.0.0.1/token")),
            ("authorization_response_iss_parameter_supported", json!(false))] {
            let mut altered = metadata(); altered[key] = value; assert!(parse_metadata(&altered, &issuer, &resource, None).is_err(), "{key}");
        }
        let mut missing = metadata(); missing.as_object_mut().unwrap().remove("authorization_response_iss_parameter_supported");
        assert!(parse_metadata(&missing, &issuer, &resource, None).is_err());
    }
    #[test]
    fn resource_binding_rejects_other_origin_path_prefix_and_query_changes() {
        let resource = Url::parse("https://resource.example.com/mcp").unwrap();
        assert!(resource_covers(&resource, &Url::parse("https://resource.example.com/mcp?login").unwrap()));
        assert!(!resource_covers(&resource, &Url::parse("https://resource.example.com/mcp-attacker").unwrap()));
        assert!(!resource_covers(&resource, &Url::parse("https://other.example.com/mcp").unwrap()));
        let resource = Url::parse("https://resource.example.com/mcp?audience=one").unwrap();
        assert!(!resource_covers(&resource, &Url::parse("https://resource.example.com/mcp?audience=two").unwrap()));
    }
    #[test]
    fn dns_policy_rejects_private_mapped_and_special_destinations() {
        for value in ["127.0.0.1", "10.0.0.1", "169.254.169.254", "100.64.0.1", "192.0.2.1", "::1", "::ffff:127.0.0.1", "fc00::1", "fe80::1", "2001:db8::1", "2002:7f00:1::1"] {
            assert!(!public_address(value.parse().unwrap()), "{value}");
        }
        for value in ["8.8.8.8", "1.1.1.1", "2606:4700:4700::1111"] { assert!(public_address(value.parse().unwrap()), "{value}"); }
    }
    #[test]
    fn discovery_urls_keep_issuer_tenant_and_drop_resource_queries() {
        let base = Url::parse("https://resource.example.com/mcp?login").unwrap();
        let candidates = metadata_candidates(&base, "oauth-protected-resource");
        assert_eq!(candidates[0].as_str(), "https://resource.example.com/.well-known/oauth-protected-resource/mcp");
        assert_eq!(candidates[1].as_str(), "https://resource.example.com/.well-known/oauth-protected-resource");
        let issuer = Url::parse("https://auth.example.com/tenant").unwrap();
        let issuers = issuer_metadata_candidates(&issuer);
        assert_eq!(issuers[0].as_str(), "https://auth.example.com/.well-known/oauth-authorization-server/tenant");
        assert_eq!(issuers[1].as_str(), "https://auth.example.com/tenant/.well-known/openid-configuration");
    }
    #[test]
    fn challenge_preserves_quoted_fields_and_rejects_duplicate_metadata() {
        let result = parse_bearer_challenge("Bearer resource_metadata=\"https://example.com/metadata?a=1,b=2\", scope=\"read tools\"").unwrap().unwrap();
        assert_eq!(result.0.as_deref(), Some("https://example.com/metadata?a=1,b=2")); assert_eq!(result.1.as_deref(), Some("read tools"));
        assert!(parse_bearer_challenge("Bearer resource_metadata=\"https://one.example\", resource_metadata=\"https://two.example\"").is_err());
        assert!(parse_bearer_challenge("Bearer realm=\"one\" scope=\"read\"").is_err());
        assert!(parse_bearer_challenge("Basic realm=\"elsewhere\"").unwrap().is_none());
    }
    #[test]
    fn saved_refresh_metadata_cannot_move_to_another_resource() {
        let issuer = Url::parse("https://auth.example.com/tenant").unwrap(); let resource = Url::parse("https://resource.example.com/mcp").unwrap();
        let saved = serde_json::to_value(parse_metadata(&metadata(), &issuer, &resource, None).unwrap()).unwrap();
        assert!(Metadata::from_saved(&saved, "https://resource.example.com/mcp").is_ok());
        assert!(Metadata::from_saved(&saved, "https://attacker.example.com/mcp").is_err());
        let mut altered = saved; altered["token"] = json!("https://127.0.0.1/token");
        assert!(Metadata::from_saved(&altered, "https://resource.example.com/mcp").is_err());
    }
    #[test]
    fn authorization_response_requires_exact_published_issuer() {
        let issuer = Url::parse("https://auth.example.com/tenant").unwrap(); let resource = Url::parse("https://resource.example.com/mcp").unwrap();
        let parsed = parse_metadata(&metadata(), &issuer, &resource, None).unwrap();
        assert!(parsed.validate_response_issuer(Some("https://auth.example.com/tenant")).is_ok());
        for received in [None, Some("https://attacker.example.com/tenant"), Some("https://auth.example.com/tenant/"), Some("https://AUTH.example.com/tenant")] {
            let failure = parsed.validate_response_issuer(received).unwrap_err();
            assert_eq!(failure.code, "AUTHENTICATION_FAILED");
        }
        let mut saved = serde_json::to_value(&parsed).unwrap(); saved["authorization_response_iss_parameter_supported"] = json!(false);
        assert!(Metadata::from_saved(&saved, resource.as_str()).is_err());
    }
}
