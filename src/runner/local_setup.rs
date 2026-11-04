//! Read only browser sign-in links from an explicit local setup attempt.
//!
//! Arbitrary process output stays private. Links are transient browser targets,
//! not authentication evidence or data to save in a server record.
use tokio::{io::{AsyncRead, AsyncReadExt}, sync::mpsc};
use url::Url;

const MAX_LINE_BYTES: usize = 64 * 1024;
const MAX_URL_BYTES: usize = 8192;

/// Drain a setup stream without retaining unbounded output or blocking the child
/// when the consumer has already received a link or ended its attempt.
pub(crate) async fn forward_login_urls<R: AsyncRead + Unpin>(reader: R, urls: mpsc::Sender<String>) {
    forward_startup_output(reader, Some(urls), None).await
}

pub(crate) async fn forward_startup_output<R: AsyncRead + Unpin>(reader: R, urls: Option<mpsc::Sender<String>>, diagnostics: Option<std::sync::Arc<super::startup_diagnostics::StartupDiagnostics>>) {
    forward_service_output(reader, urls, diagnostics, "stderr").await;
}

pub(crate) async fn forward_service_output<R: AsyncRead + Unpin>(mut reader: R, urls: Option<mpsc::Sender<String>>, diagnostics: Option<std::sync::Arc<super::startup_diagnostics::StartupDiagnostics>>, source: &str) {
    let mut buffer = [0u8; 4096];
    let mut line = Vec::with_capacity(4096);
    let mut oversized = false;
    loop {
        let count = match reader.read(&mut buffer).await { Ok(0) | Err(_) => break, Ok(count) => count };
        for &byte in &buffer[..count] {
            if byte == b'\n' {
                if !oversized { forward_line(&line, urls.as_ref(), diagnostics.as_deref(), source); }
                line.clear();
                oversized = false;
            } else if !oversized {
                if line.len() == MAX_LINE_BYTES {
                    if let Some(diagnostics) = &diagnostics {
                        diagnostics.text(source, "[line exceeded 65536 bytes and was omitted]");
                    }
                    line.clear();
                    oversized = true;
                } else {
                    line.push(byte);
                }
            }
        }
    }
    if !oversized { forward_line(&line, urls.as_ref(), diagnostics.as_deref(), source); }
}

fn forward_line(line: &[u8], urls: Option<&mpsc::Sender<String>>, diagnostics: Option<&super::startup_diagnostics::StartupDiagnostics>, source: &str) {
    if let Ok(line) = std::str::from_utf8(line) {
        if let Some(urls) = urls { if let Some(url) = login_url(line) { let _ = urls.try_send(url); } }
        if let Some(diagnostics) = diagnostics { diagnostics.text(source, line); }
    }
}

fn login_url(line: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    let instruction = ["sign in", "sign-in", "log in", "login at", "login:", "login url", "authorize", "authorization url", "authentication url"]
        .iter().any(|phrase| lower.contains(phrase));
    for (offset, _) in line.match_indices("https://") {
        let candidate = line[offset..].split(|character: char| character.is_whitespace() || matches!(character, '"' | '\'' | '<' | '>' | '`')).next()?;
        let candidate = candidate.trim_end_matches([')', ']', '}', ',', '.', ';']);
        if candidate.len() > MAX_URL_BYTES || candidate.chars().any(char::is_control) { continue; }
        let Ok(url) = Url::parse(candidate) else { continue };
        if url.scheme() != "https" || url.host_str().is_none() || !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() { continue; }
        let mut authorization_code = false;
        let mut client_id = false;
        let mut contains_secret = false;
        for (name, value) in url.query_pairs() {
            let name = name.to_ascii_lowercase();
            if ["access_token", "refresh_token", "id_token", "token", "code", "client_secret", "secret", "password", "authorization", "api_key", "apikey"].contains(&name.as_str()) {
                contains_secret = true;
            }
            authorization_code |= name == "response_type" && value == "code";
            client_id |= name == "client_id" && !value.is_empty();
        }
        if !contains_secret && (instruction || authorization_code && client_id) { return Some(url.to_string()); }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[test]
    fn google_style_instruction_returns_only_the_authorization_url() {
        let url = "https://accounts.example.com/o/oauth2/v2/auth?response_type=code&client_id=public-client&state=opaque-state&redirect_uri=http%3A%2F%2Flocalhost%3A54321";
        assert_eq!(login_url(&format!("[INFO] Authorize this app by visiting this url: {url}")), Some(url.into()));
    }

    #[test]
    fn unrelated_diagnostics_and_credential_urls_do_not_become_sign_in_links() {
        assert!(login_url("client_secret=private-fixture; documentation https://example.com/docs").is_none());
        for tail in ["access_token=private", "refresh_token=private", "id_token=private", "code=private", "client_secret=private", "password=private", "%74oken=private"] {
            assert!(login_url(&format!("Sign in at https://example.com/login?{tail}")).is_none());
        }
        assert_eq!(login_url("https://example.com/auth?response_type=code&client_id=public&state=opaque"), Some("https://example.com/auth?response_type=code&client_id=public&state=opaque".into()));
    }

    #[test]
    fn malformed_browser_targets_are_not_forwarded() {
        for value in ["http://example.com/login", "https://user:password@example.com/login", "https://example.com/login#access_token=secret", "https://", "https://example.com/login\u{1b}[0m"] {
            assert!(login_url(&format!("Sign in at {value}")).is_none());
        }
    }

    #[tokio::test]
    async fn initial_output_is_delivered_before_stream_completion_and_queue_does_not_block() {
        let (mut writer, reader) = tokio::io::duplex(256);
        let (sender, mut urls) = mpsc::channel(1);
        let task = tokio::spawn(forward_login_urls(reader, sender));
        writer.write_all(b"Authorize this app: https://example.com/login?state=first\n").await.unwrap();
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(1), urls.recv()).await.unwrap().unwrap(), "https://example.com/login?state=first");
        writer.write_all(b"Sign in at https://example.com/login?state=second\nSign in at https://example.com/login?state=third\n").await.unwrap();
        drop(writer);
        tokio::time::timeout(std::time::Duration::from_secs(1), task).await.unwrap().unwrap();
        assert_eq!(urls.recv().await.unwrap(), "https://example.com/login?state=second");
    }

    #[tokio::test]
    async fn oversized_output_is_drained_and_next_instruction_is_kept() {
        let mut output = b"Sign in at https://example.com/login?state=discarded ".to_vec();
        output.extend(std::iter::repeat_n(b'x', MAX_LINE_BYTES * 3));
        output.extend_from_slice(b"\nSign in at https://example.com/login?state=kept\n");
        let (sender, mut urls) = mpsc::channel(4);
        forward_login_urls(output.as_slice(), sender).await;
        assert_eq!(urls.recv().await.unwrap(), "https://example.com/login?state=kept");
        assert!(urls.recv().await.is_none());
    }
}
