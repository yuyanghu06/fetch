//! Generic destination protections around the official SDK's HTTP requests.
use super::{ManagementError,auth_required};
use std::{net::{IpAddr,SocketAddr},time::Duration};
use url::Url;
const MAX_METADATA_URL:usize=8192;
pub(crate) async fn client_for(endpoint: &Url) -> Result<reqwest::Client, ManagementError> {
    let host = endpoint.host_str().ok_or_else(auth_required)?;
    let port = endpoint.port_or_known_default().ok_or_else(auth_required)?;
    let addresses: Vec<SocketAddr> = tokio::time::timeout(Duration::from_secs(5), tokio::net::lookup_host((host, port))).await
        .map_err(|_| auth_required())?.map_err(|_| auth_required())?.take(33).collect();
    if addresses.is_empty() || addresses.len() > 32 || addresses.iter().any(|a| !public_address(a.ip())) { return Err(auth_required()); }
    reqwest::Client::builder().https_only(true).no_proxy().redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10)).timeout(Duration::from_secs(20))
        .resolve_to_addrs(host, &addresses).build().map_err(|_| auth_required())
}

pub(crate) fn checked_url(value: &str) -> Result<Url, ManagementError> {
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

pub(crate) async fn validate_browser_destination(value:&str)->Result<(),ManagementError>{client_for(&checked_url(value)?).await.map(|_|())}
