//! Disposable source fixtures only: no process, package, provider, Keychain or MCP is started.
use fetch_local::{discovery::DiscoveryService, installation::Installer, storage::ServerRepository};
use serde_json::{Value, json};
use std::{path::Path, time::Duration};

fn source(path: &Path, entries: Value) -> Vec<u8> {
    let bytes = serde_json::to_vec(&json!({"mcpServers":entries})).unwrap();
    std::fs::write(path, &bytes).unwrap();
    bytes
}

fn row<'a>(scan: &'a Value, name: &str) -> &'a Value {
    scan["configs"][0]["servers"].as_array().unwrap().iter()
        .find(|item| item["name"] == name).unwrap()
}

#[test]
fn documented_public_endpoints_capture_without_exposing_or_rewriting_the_url() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("servers.json");
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    for endpoint in [
        "https://mcp.miro.com/",
        "https://mcp.vercel.com/",
        "https://v0.app/api/mcp",
        "https://huggingface.co/mcp?login",
        "https://huggingface.co/mcp?login&no_image_content=true&bouquet=openai",
        "https://huggingface.co/mcp?mix=proxy&no_image_content=false",
    ] {
        let original = source(&path, json!({"server":{"url":endpoint,"headers":{"Authorization":"source-secret-marker"}}}));
        let scan = discovery.scan(Some(&path)).unwrap();
        assert_eq!(row(&scan, "server")["status"], "supported", "{endpoint}");
        let capture = discovery.capture(&path, "server").unwrap();
        assert_eq!(capture.config["connection"]["url"], endpoint);
        assert_eq!(capture.config["connection"]["transport"], "streamable_http");
        assert_eq!(capture.config["connection"]["headers"], json!([{"name":"Authorization","credential_ref":"Authorization"}]));
        let origin = url::Url::parse(endpoint).unwrap().origin().ascii_serialization();
        assert_eq!(row(&scan, "server")["connection"], json!({
            "transport":"streamable_http", "origin":origin, "header_names":["Authorization"]
        }));
        assert!(!scan.to_string().contains(endpoint), "Discovery must project the origin without the supplied endpoint path/query");
        assert!(!scan.to_string().contains("source-secret-marker"));
        assert!(!capture.config.to_string().contains("source-secret-marker"));
        assert_eq!(std::fs::read(&path).unwrap(), original);
    }
}

#[test]
fn endpoint_profiles_reject_secrets_ambiguous_queries_and_lookalike_hosts() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("servers.json");
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    for endpoint in [
        "https://huggingface.co/mcp?token=source-secret-marker",
        "https://huggingface.co/mcp?login=source-secret-marker",
        "https://huggingface.co/mcp?bouquet=source-secret-marker",
        "https://huggingface.co/mcp?login&login",
        "https://huggingface.co/mcp?login&&no_image_content=true",
        "https://huggingface.co/mcp?%6cogin",
        "https://huggingface.co/mcp?mix=proxy&unknown=true",
        "https://huggingface.co:8443/mcp?login",
        "https://huggingface.co.evil.example/mcp?login",
        "https://huggingface.co/secret/mcp?login",
        "https://mcp.vercel.com.evil.example/",
        "https://mcp.miro.com/?token=source-secret-marker",
        "https://mcp.vercel.com:8443/",
        "https://user:source-secret-marker@mcp.miro.com/",
        "https://mcp.vercel.com/#source-secret-marker",
        "http://huggingface.co/mcp",
        "https://example.com/private/source-secret-marker",
    ] {
        source(&path, json!({"server":{"url":endpoint}}));
        let scan = discovery.scan(Some(&path)).unwrap();
        assert_eq!(row(&scan, "server")["status"], "unsupported", "{endpoint}");
        assert!(discovery.capture(&path, "server").is_err(), "{endpoint}");
        assert!(!scan.to_string().contains("source-secret-marker"));
    }
}

#[test]
fn bridge_headers_become_fresh_references_without_source_values_or_arguments() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("servers.json");
    let original = source(&path, json!({"v0":{
        "command":"npx",
        "args":["-y","mcp-remote@0.1.38","https://v0.app/api/mcp","--header","Authorization:Bearer source-secret-marker","--header","X-Example:${SOURCE_HEADER}"],
        "env":{"SOURCE_HEADER":"source-environment-marker"},
        "disabled":true
    }}));
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    let scan = discovery.scan(Some(&path)).unwrap();
    assert_eq!(row(&scan, "v0")["status"], "supported");
    assert_eq!(row(&scan, "v0")["type"], "remote");
    assert_eq!(row(&scan, "v0")["source_enabled"], false);
    assert_eq!(row(&scan, "v0")["selected_by_default"], false);
    let capture = discovery.capture(&path, "v0").unwrap();
    assert_eq!(capture.config["kind"], "remote");
    assert_eq!(capture.config["connection"]["url"], "https://v0.app/api/mcp");
    assert_eq!(capture.config["connection"]["headers"], json!([
        {"name":"Authorization","credential_ref":"Authorization"},
        {"name":"X-Example","credential_ref":"X-Example"}
    ]));
    assert!(capture.config["executable"].is_null());
    assert_eq!(capture.config["args"], json!([]));
    assert_eq!(capture.config["environment"], json!([]));
    assert!(capture.package_arguments.is_empty());
    for value in [&scan, &capture.config] {
        let text = value.to_string();
        for secret in ["source-secret-marker", "source-environment-marker", "${SOURCE_HEADER}", "mcp-remote"] {
            assert!(!text.contains(secret), "{secret}");
        }
    }
    assert_eq!(std::fs::read(&path).unwrap(), original);
}

#[test]
fn bridge_transport_and_header_options_are_explicit_and_bounded() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("servers.json");
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    for (args, transport) in [
        (json!(["mcp-remote","https://v0.app/api/mcp"]), "streamable_http"),
        (json!(["mcp-remote","https://example.com/sse","--transport","sse-only"]), "sse"),
        (json!(["mcp-remote","https://example.com/mcp","--transport","http-only"]), "streamable_http"),
    ] {
        source(&path, json!({"remote":{"command":"npx","args":args}}));
        let capture = discovery.capture(&path, "remote").unwrap();
        assert_eq!(capture.config["connection"]["transport"], transport);
    }
    for args in [
        json!(["mcp-remote","https://example.com/mcp"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--header"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--header","Authorization:one","--header","authorization:two"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--header","Authorization:one\r\nX-Injected:two"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--header","Host:wrong.example"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--transport","http-first"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--header-file","/fixture/secret"]),
        json!(["mcp-remote","https://v0.app/api/mcp","--static-oauth-client-info","source-secret-marker"]),
    ] {
        source(&path, json!({"remote":{"command":"npx","args":args}}));
        assert!(discovery.capture(&path, "remote").is_err(), "{args}");
    }
}

#[test]
fn codex_and_opencode_bridge_shapes_keep_their_own_source_settings() {
    let fixture = tempfile::tempdir().unwrap();
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    let codex = fixture.path().join("codex.toml");
    std::fs::write(&codex, "[mcp_servers.v0]\ncommand='npx'\nargs=['mcp-remote', 'https://v0.app/api/mcp']\ntool_timeout_sec=240\n").unwrap();
    let capture = discovery.capture(&codex, "v0").unwrap();
    assert_eq!(capture.config["execution"]["default_timeout_ms"], 240_000);
    assert_eq!(capture.config["connection"]["url"], "https://v0.app/api/mcp");
    let opencode = fixture.path().join("opencode.json");
    std::fs::write(&opencode, serde_json::to_vec(&json!({"mcp":{"v0":{
        "type":"local","command":["npx","--yes","mcp-remote","https://v0.app/api/mcp"],"timeout":180_000
    }}})).unwrap()).unwrap();
    let capture = discovery.capture(&opencode, "v0").unwrap();
    assert_eq!(capture.config["execution"]["default_timeout_ms"], 180_000);
    assert_eq!(capture.config["args"], json!([]));
}

#[test]
fn curated_local_package_forms_keep_only_runtime_server_arguments() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("servers.json");
    source(&path, json!({
        "everything":{"command":"npx","args":["-y","@modelcontextprotocol/server-everything@2.0.1","stdio"]},
        "time":{"command":"uvx","args":["--from","mcp-server-time==2026.10.19","mcp-server-time","--local-timezone=Etc/UTC"]},
        "bad-time":{"command":"uvx","args":["--from","mcp-server-time==2026.10.19","different-command"]},
        "empty-timezone":{"command":"uvx","args":["mcp-server-time","--local-timezone="]},
        "network-everything":{"command":"npx","args":["@modelcontextprotocol/server-everything","streamableHttp"]}
    }));
    let discovery = DiscoveryService::new(fixture.path().into(), None);
    let everything = discovery.capture(&path, "everything").unwrap();
    assert_eq!(everything.package_arguments, ["stdio"]);
    let time = discovery.capture(&path, "time").unwrap();
    assert_eq!(time.row["package"], "mcp-server-time");
    assert_eq!(time.row["version_spec"], "2026.10.19");
    assert_eq!(time.package_arguments, ["--local-timezone=Etc/UTC"]);
    for name in ["bad-time", "empty-timezone", "network-everything"] {
        assert!(discovery.capture(&path, name).is_err());
    }
}

async fn finished(installer: &Installer, id: &str) -> Value {
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let progress = installer.progress(id).await.unwrap();
            if progress["status"] != "installing" { return progress; }
            tokio::task::yield_now().await;
        }
    }).await.unwrap()
}

#[tokio::test]
async fn remote_preparation_reuses_source_identity_and_preserves_saved_destination() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("private-state");
    let path = fixture.path().join("servers.json");
    source(&path, json!({"hf":{"url":"https://huggingface.co/mcp?login&bouquet=openai"}}));
    let installer = Installer::new(root.clone(), DiscoveryService::new(fixture.path().into(), None));
    let input = json!({"servers":[{"config_path":path,"server_name":"hf"}]});
    let accepted = installer.submit(input.clone()).await.unwrap();
    let progress = finished(&installer, accepted["installation_id"].as_str().unwrap()).await;
    assert_eq!(progress["status"], "succeeded");
    let id = progress["servers"][0]["server_id"].as_str().unwrap();
    let repository = ServerRepository::new(root.clone());
    let saved = repository.get_saved(id).unwrap().unwrap();
    assert_eq!(saved["config"]["connection"]["url"], "https://huggingface.co/mcp?login&bouquet=openai");
    source(&path, json!({"hf":{"url":"https://mcp.miro.com/"}}));
    let accepted = installer.submit(input).await.unwrap();
    let repeated = finished(&installer, accepted["installation_id"].as_str().unwrap()).await;
    assert_eq!(repeated["status"], "succeeded");
    assert_eq!(repeated["servers"][0]["server_id"], id);
    assert_eq!(repository.get_saved(id).unwrap().unwrap(), saved);
    assert_eq!(repository.list_saved().unwrap().len(), 1);
    assert!(!root.join("packages").exists(), "Remote record preparation must not install a bridge or runtime");
}
