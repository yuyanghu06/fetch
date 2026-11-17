//! Workspace use cases use disposable homes and projects, never personal state.
use fetch_local::workspace::WorkspaceService;
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}};
use tempfile::TempDir;
use axum::{Router, body::Body, http::{HeaderMap, Request, StatusCode, header}};
use fetch_local::{
    api::{AppState, router},
    authority::{AuthorizationError, Authorizer, Caller},
    discovery::DiscoveryService,
    native_auth::NativeAuthority,
    storage::ServerRepository,
};
use http_body_util::BodyExt;
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    directory: TempDir,
    service: WorkspaceService,
    project: PathBuf,
    other_project: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let home = directory.path().join("fixture-home");
        let project = directory.path().join("project-a");
        let other_project = directory.path().join("project-b");
        for path in [&home, &project, &other_project] { fs::create_dir_all(path).unwrap(); }
        let service = WorkspaceService::new(home.clone(), home.join(".codex"), home.join(".claude"));
        Self { directory, service, project, other_project }
    }

    fn folder(&self) -> &str { self.project.to_str().unwrap() }

    fn write(&self, path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn codex_session(&self, name: &str, project: &Path, body: &str) -> PathBuf {
        let path = self.service.codex_root.join("sessions/2026/11/12").join(format!("{name}.jsonl"));
        let content = format!("{}\n{}\n", json!({
            "type":"session_meta", "payload":{"id":name,"cwd":project,"title":name}
        }), json!({"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":body}]}}));
        self.write(&path, &content);
        path
    }

    fn claude_session(&self, name: &str, project: &Path, body: &str) -> PathBuf {
        let path = self.service.claude_root.join("projects/fixture-project").join(format!("{name}.jsonl"));
        let content = format!("{}\n{}\n", json!({
            "type":"custom-title", "sessionId":name,"cwd":project,"customTitle":name
        }), json!({"type":"assistant","sessionId":name,"cwd":project,"message":{"role":"assistant","content":[{"type":"text","text":body}]}}));
        self.write(&path, &content);
        path
    }
}

#[test]
fn choosing_a_project_reads_and_saves_its_native_files_without_global_copies() {
    let fixture = Fixture::new();
    let global = fixture.service.codex_root.join("AGENTS.md");
    let global_claude = fixture.service.claude_root.join("CLAUDE.md");
    let local = fixture.project.join("AGENTS.md");
    fixture.write(&global, "Global fixture instructions\n");
    fixture.write(&global_claude, "Global Claude instructions\n");
    fixture.write(&local, "Original project instructions\n");

    let global_view = fixture.service.discover(None).unwrap();
    assert!(global_view["files"].as_array().unwrap().iter().all(|file| {
        Path::new(file["file_path"].as_str().unwrap()).starts_with(&fixture.service.home)
    }));
    assert!(global_view["files"].as_array().unwrap().iter().any(|file| file["file_path"] == global.to_str().unwrap()));

    let project_view = fixture.service.discover(Some(fixture.folder())).unwrap();
    let project_root = fixture.project.canonicalize().unwrap();
    assert!(project_view["files"].as_array().unwrap().iter().all(|file| {
        Path::new(file["file_path"].as_str().unwrap()).starts_with(&project_root)
    }));
    let read = fixture.service.read_file(Some(fixture.folder()), local.to_str().unwrap()).unwrap();
    assert_eq!(read["content"], "Original project instructions\n");
    let saved = fixture.service.save_file(Some(fixture.folder()), local.to_str().unwrap(), "Updated project instructions\n").unwrap();
    assert_eq!(saved["saved"], true);
    assert_eq!(fs::read_to_string(&local).unwrap(), "Updated project instructions\n");
    assert_eq!(fs::read_to_string(&global).unwrap(), "Global fixture instructions\n");
    assert_eq!(saved["next_steps"], json!([]));
}

#[test]
fn file_operations_reject_unrelated_paths_and_links_outside_the_selected_project() {
    let fixture = Fixture::new();
    let unrelated = fixture.project.join("private.txt");
    fixture.write(&unrelated, "Not an instruction file\n");
    let error = fixture.service.read_file(Some(fixture.folder()), unrelated.to_str().unwrap()).unwrap_err();
    assert_eq!(error.http_status, 403);
    assert!(fixture.service.save_file(Some(fixture.folder()), unrelated.to_str().unwrap(), "must not save").is_err());
    assert_eq!(fs::read_to_string(&unrelated).unwrap(), "Not an instruction file\n");

    #[cfg(unix)] {
        let outside = fixture.directory.path().join("outside/AGENTS.md");
        fixture.write(&outside, "Outside fixture instructions\n");
        let linked = fixture.project.join("AGENTS.md");
        std::os::unix::fs::symlink(&outside, &linked).unwrap();
        assert_eq!(fixture.service.read_file(Some(fixture.folder()), linked.to_str().unwrap()).unwrap_err().http_status, 403);
        assert_eq!(fixture.service.save_file(Some(fixture.folder()), linked.to_str().unwrap(), "must not escape").unwrap_err().http_status, 403);
        let discovered = fixture.service.discover(Some(fixture.folder())).unwrap();
        assert!(!discovered["files"].as_array().unwrap().iter().any(|file| file["file_path"] == linked.to_str().unwrap()));
        assert_eq!(fs::read_to_string(&outside).unwrap(), "Outside fixture instructions\n");
    }
}

#[test]
fn pasting_an_agent_converts_common_fields_and_flags_source_settings_before_explicit_save() {
    let fixture = Fixture::new();
    let source_path = fixture.project.join(".claude/agents/reviewer.md");
    let target_path = fixture.project.join(".codex/agents/reviewer.toml");
    let instructions = "Review the user's requested change.\nPreserve quoted text: \"hello\" and Unicode 🌱.\n";
    let source = format!("---\nname: reviewer\ndescription: Reviews requested changes\nmodel: fixture-claude-model\ntools:\n  - Read\n  - Grep\n---\n\n{instructions}");
    fixture.write(&source_path, &source);

    let draft = fixture.service.convert_agent("claude", "codex", &source).unwrap();
    let converted = draft["content"].as_str().unwrap();
    let parsed: toml::Value = toml::from_str(converted).unwrap();
    assert_eq!(parsed["name"].as_str(), Some("reviewer"));
    assert_eq!(parsed["description"].as_str(), Some("Reviews requested changes"));
    assert_eq!(parsed["developer_instructions"].as_str(), Some(instructions));
    assert!(parsed.get("model").is_none());
    for field in ["model", "tools"] {
        assert!(draft["issues"].as_array().unwrap().iter().any(|issue| issue["field"] == field && issue["requires_action"] == true));
    }
    assert_eq!(draft["source_content"], source);
    assert_eq!(draft["requires_review"], true);
    assert!(!target_path.exists(), "conversion prepares a draft without writing files");
    assert_eq!(fs::read_to_string(&source_path).unwrap(), source);

    let with_model = fixture.service.set_agent_model("codex", converted, "fixture-codex-model").unwrap();
    let destination = with_model["content"].as_str().unwrap();
    fixture.service.save_file(Some(fixture.folder()), target_path.to_str().unwrap(), destination).unwrap();
    assert_eq!(fs::read_to_string(&target_path).unwrap(), destination);
    assert_eq!(fs::read_to_string(&source_path).unwrap(), source);

    let back = fixture.service.convert_agent("codex", "claude", destination).unwrap();
    let markdown = back["content"].as_str().unwrap();
    let (_, rest) = markdown.split_once("---\n").unwrap();
    let (header, body) = rest.split_once("---\n").unwrap();
    let fields: Value = serde_saphyr::from_str(header).unwrap();
    assert_eq!(fields["name"], "reviewer");
    assert_eq!(fields["description"], "Reviews requested changes");
    assert_eq!(body.trim_start_matches('\n'), instructions);
    assert!(back["issues"].as_array().unwrap().iter().any(|issue| issue["field"] == "model"));
}

#[test]
fn session_names_are_project_filtered_and_bounded_chunks_reconstruct_large_utf8_logs() {
    let fixture = Fixture::new();
    let huge_message = "private-chunk-fixture:".to_owned() + &"字🌱".repeat(30_000);
    let selected = fixture.codex_session("Project A review", &fixture.project, &huge_message);
    let other = fixture.claude_session("Project B work", &fixture.other_project, "other-project-private-body");
    let original = fs::read_to_string(&selected).unwrap();

    let global = fixture.service.list_sessions(None, None, None, None).unwrap();
    assert_eq!(global["sessions"].as_array().unwrap().len(), 2);
    let names = fixture.service.list_sessions(Some(fixture.folder()), None, None, None).unwrap();
    let sessions = names["sessions"].as_array().unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0]["name"], "Project A review");
    assert_eq!(sessions[0]["app"], "codex");
    assert!(sessions[0]["id"].is_string());
    assert!(sessions[0]["updated_at"].is_number());
    assert!(sessions[0].get("content").is_none());
    assert!(!names.to_string().contains("private-chunk-fixture"));

    let mut offset = 0;
    let mut reconstructed = String::new();
    loop {
        let chunk = fixture.service.read_session(Some(fixture.folder()), selected.to_str().unwrap(), offset, Some(4097)).unwrap();
        let content = chunk["content"].as_str().unwrap();
        assert!(content.len() <= 4097);
        assert_eq!(chunk["offset"], offset);
        reconstructed.push_str(content);
        let next = chunk["next_offset"].as_u64().unwrap();
        assert!(next > offset || chunk["eof"] == true);
        offset = next;
        if chunk["eof"] == true { break; }
    }
    assert_eq!(reconstructed, original);
    assert_eq!(fs::read_to_string(&selected).unwrap(), original);
    assert_eq!(fixture.service.read_session(Some(fixture.folder()), other.to_str().unwrap(), 0, None).unwrap_err().http_status, 403);
    let other_id = global["sessions"].as_array().unwrap().iter().find(|session| session["app"] == "claude").unwrap()["id"].as_str().unwrap();
    assert_eq!(fixture.service.read_session_by_id(Some(fixture.folder()), other_id, 0, None).unwrap_err().http_status, 404);

    // Codex can retain the same recorded stream in a compressed JSONL archive.
    // Listing and chunked reads must work without rewriting that source file.
    let archived = fixture.service.codex_root.join("archived_sessions/compressed-review.jsonl.zst");
    let archived_stream = original.replace("Project A review", "Archived project A review");
    let compressed = zstd::stream::encode_all(archived_stream.as_bytes(), 1).unwrap();
    fs::create_dir_all(archived.parent().unwrap()).unwrap();
    fs::write(&archived, &compressed).unwrap();
    let names = fixture.service.list_sessions(Some(fixture.folder()), Some("codex"), None, None).unwrap();
    let archived_metadata = names["sessions"].as_array().unwrap().iter()
        .find(|session| session["source_format"] == "jsonl.zst").unwrap();
    assert_eq!(archived_metadata["name"], "Archived project A review");
    assert_eq!(Path::new(archived_metadata["project_path"].as_str().unwrap()), fixture.project.canonicalize().unwrap());
    assert!(archived_metadata.get("content").is_none());
    let mut offset = 0;
    let mut reconstructed = String::new();
    let mut chunks = 0;
    loop {
        let chunk = fixture.service.read_session(Some(fixture.folder()), archived.to_str().unwrap(), offset, Some(4097)).unwrap();
        let content = chunk["content"].as_str().unwrap();
        assert!(content.len() <= 4097);
        assert_eq!(chunk["offset"], offset);
        reconstructed.push_str(content);
        chunks += 1;
        let next = chunk["next_offset"].as_u64().unwrap();
        assert!(next > offset || chunk["eof"] == true);
        offset = next;
        if chunk["eof"] == true { break; }
    }
    assert!(chunks > 1);
    assert_eq!(reconstructed, archived_stream);
    assert_eq!(fs::read(&archived).unwrap(), compressed);
}

/// This injected fixture grant deliberately has all ordinary app permissions.
/// Workspace must still demand the native-window capability instead.
struct OrdinaryGrant;
impl Authorizer for OrdinaryGrant {
    fn authorize(&self, _: &HeaderMap) -> Result<Caller, AuthorizationError> {
        Ok(Caller::from_verified_scopes(["fetch:manage", "fetch:catalog", "fetch:content:read", "fetch:tools:call"].into_iter().map(str::to_owned)))
    }
}

fn api_fixture(fixture: &Fixture) -> (Router, Arc<NativeAuthority>) {
    let native = NativeAuthority::new();
    let mut state = AppState::new(
        "https://localhost:53671".into(),
        ServerRepository::new(fixture.directory.path().join("fetch-state")),
        DiscoveryService::new(fixture.service.home.clone(), None),
        Arc::new(OrdinaryGrant),
    ).unwrap();
    state.workspace = Some(Arc::new(fixture.service.clone()));
    state.native = Some(native.clone());
    (router(Arc::new(state)), native)
}

fn url(path: &str, fields: &[(&str, &str)]) -> String {
    let mut query = url::form_urlencoded::Serializer::new(String::new());
    for (key, value) in fields { query.append_pair(key, value); }
    format!("{path}?{}", query.finish())
}

fn api_request(method: &str, uri: &str, input: Option<Value>, capability: Option<&str>) -> Request<Body> {
    let mut request = Request::builder().method(method).uri(uri).header(header::HOST, "localhost:53671");
    if let Some(capability) = capability { request = request.header("x-fetch-settings-capability", capability); }
    let body = match input {
        Some(input) => { request = request.header(header::CONTENT_TYPE, "application/json"); Body::from(input.to_string()) },
        None => Body::empty(),
    };
    request.body(body).unwrap()
}

async fn api_value(app: &Router, request: Request<Body>, expected: StatusCode) -> Value {
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), expected);
    assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    let value: Value = serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert!(value["next_steps"].is_array());
    value
}

#[tokio::test]
async fn native_workspace_apis_edit_the_fixture_file_convert_agents_and_read_session_chunks() {
    let fixture = Fixture::new();
    let instruction = fixture.project.join("AGENTS.md");
    fixture.write(&instruction, "Original API fixture instructions\n");
    let log = fixture.codex_session("API fixture session", &fixture.project, "Read this history through the native API.");
    let model_cache = json!({"identity":"fixture-private-root-identity","models":[{
        "slug":"fixture-dynamic-model-from-installed-cache","display_name":"Model from fixture app cache","visibility":"list"
    }]});
    let cache_path = fixture.service.codex_root.join("models_cache.json");
    fixture.write(&cache_path, &model_cache.to_string());
    let (app, native) = api_fixture(&fixture);
    let session = native.open_settings();
    let capability = session.header_value();

    let discovered = api_value(&app, api_request("GET", &url("/api/v1/workspace", &[("folder_path", fixture.folder())]), None, Some(capability)), StatusCode::OK).await;
    let canonical_instruction = instruction.canonicalize().unwrap();
    assert!(discovered["files"].as_array().unwrap().iter().any(|file| file["file_path"] == canonical_instruction.to_str().unwrap()));
    let file_uri = url("/api/v1/workspace/file", &[("folder_path", fixture.folder()), ("file_path", instruction.to_str().unwrap())]);
    let read = api_value(&app, api_request("GET", &file_uri, None, Some(capability)), StatusCode::OK).await;
    assert_eq!(read["content"], "Original API fixture instructions\n");
    api_value(&app, api_request("PUT", "/api/v1/workspace/file", Some(json!({
        "folder_path":fixture.folder(),"file_path":instruction,"content":"Saved through Workspace API\n"
    })), Some(capability)), StatusCode::OK).await;
    assert_eq!(fs::read_to_string(&instruction).unwrap(), "Saved through Workspace API\n");

    let converted = api_value(&app, api_request("POST", "/api/v1/workspace/agents/convert", Some(json!({
        "source_app":"claude","target_app":"codex","content":"---\nname: reviewer\ndescription: Reviews code\nmodel: fixture-source-model\n---\nReview carefully.\n",
        "fallback_name":"different-card-label",
        "fallback_description":"Registration text should not replace this definition's description",
        "target_model":"fixture-destination-model"
    })), Some(capability)), StatusCode::OK).await;
    let config: toml::Value = toml::from_str(converted["content"].as_str().unwrap()).unwrap();
    assert_eq!(config["name"].as_str(), Some("reviewer"));
    assert_eq!(config["description"].as_str(), Some("Reviews code"));
    assert_eq!(config["model"].as_str(), Some("fixture-destination-model"));
    assert_eq!(config["developer_instructions"].as_str(), Some("Review carefully.\n"));
    assert!(!fixture.project.join(".codex/agents/reviewer.toml").exists());

    let legacy = "developer_instructions = 'Make the requested change.'\n";
    let converted = api_value(&app, api_request("POST", "/api/v1/workspace/agents/convert", Some(json!({
        "source_app":"codex","target_app":"claude","content":legacy,"fallback_name":"coder"
    })), Some(capability)), StatusCode::OK).await;
    assert_eq!(converted["name"], "coder");
    assert_eq!(converted["source_content"], legacy);
    let issues = converted["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0]["field"], "description");

    let legacy_path = fixture.project.join(".codex/agents/coder.toml");
    let legacy = "sandbox_mode = 'read-only'\ndeveloper_instructions = 'Inspect the requested change.'\n";
    fixture.write(&legacy_path, legacy);
    fixture.write(&fixture.project.join(".codex/config.toml"),
        "[agents.coder]\nconfig_file = 'agents/coder.toml'\ndescription = 'Inspect project code'\n");
    fixture.write(&fixture.service.codex_root.join("config.toml"),
        "[agents.coder]\nconfig_file = 'agents/coder.toml'\ndescription = 'Global registration'\n");
    let legacy_uri = url("/api/v1/workspace/file", &[("folder_path", fixture.folder()), ("file_path", legacy_path.to_str().unwrap())]);
    let read = api_value(&app, api_request("GET", &legacy_uri, None, Some(capability)), StatusCode::OK).await;
    assert_eq!(read["content"], legacy);
    assert_eq!(read["registered_description"], "Inspect project code");
    let converted = api_value(&app, api_request("POST", "/api/v1/workspace/agents/convert", Some(json!({
        "source_app":"codex","target_app":"claude","content":read["content"],"fallback_name":"coder",
        "fallback_description":read["registered_description"]
    })), Some(capability)), StatusCode::OK).await;
    let (header, _) = converted["content"].as_str().unwrap().strip_prefix("---\n").unwrap().split_once("\n---\n").unwrap();
    let fields: Value = serde_saphyr::from_str(header).unwrap();
    assert_eq!(fields["description"], "Inspect project code");
    assert_eq!(fields["permissionMode"], "plan");
    assert!(fields.get("sandbox_mode").is_none());
    assert!(converted["issues"].as_array().unwrap().is_empty());
    assert_eq!(fs::read_to_string(&legacy_path).unwrap(), legacy);

    let names = api_value(&app, api_request("GET", &url("/api/v1/workspace/sessions", &[("folder_path", fixture.folder()), ("app", "codex")]), None, Some(capability)), StatusCode::OK).await;
    assert_eq!(names["sessions"][0]["name"], "API fixture session");
    assert!(names["sessions"][0].get("content").is_none());
    let chunk = api_value(&app, api_request("GET", &url("/api/v1/workspace/sessions/read", &[("folder_path", fixture.folder()), ("file_path", log.to_str().unwrap()), ("limit", "64")]), None, Some(capability)), StatusCode::OK).await;
    assert!(chunk["content"].as_str().unwrap().len() <= 64);
    assert_eq!(chunk["eof"], false);
    assert!(chunk["next_offset"].as_u64().unwrap() > 0);

    let models = api_value(&app, api_request("GET", "/api/v1/workspace/models?app=codex", None, Some(capability)), StatusCode::OK).await;
    assert_eq!(models["models"][0]["slug"], "fixture-dynamic-model-from-installed-cache");
    assert!(!models.to_string().contains("fixture-private-root-identity"));
    assert_eq!(serde_json::from_slice::<Value>(&fs::read(&cache_path).unwrap()).unwrap(), model_cache);
}

#[tokio::test]
async fn creating_a_subagent_refuses_to_replace_an_existing_definition() {
    let fixture = Fixture::new();
    let folder = fixture.project.join(".codex/agents");
    let existing = folder.join("reviewer.toml");
    let renamed = folder.join("project-reviewer.toml");
    let original = "name = \"reviewer\"\ndescription = \"Existing agent\"\ndeveloper_instructions = \"Keep the original.\"\n";
    let draft = "name = \"reviewer\"\ndescription = \"New agent\"\ndeveloper_instructions = \"Read the project instructions.\"\n";
    fixture.write(&existing, original);
    let (app, native) = api_fixture(&fixture);
    let session = native.open_settings();
    let request = |path: &Path, content: &str| api_request(
        "PUT",
        "/api/v1/workspace/file",
        Some(json!({"folder_path":fixture.folder(),"file_path":path,"content":content})),
        Some(session.header_value()),
    );
    let create_only = |path: &Path| {
        let mut request = request(path, draft);
        request.headers_mut().insert(header::IF_NONE_MATCH, header::HeaderValue::from_static("*"));
        request
    };

    let conflict = api_value(&app, create_only(&existing), StatusCode::PRECONDITION_FAILED).await;
    assert_eq!(conflict["error"]["code"], "WORKSPACE_FILE_ALREADY_EXISTS");
    assert_eq!(conflict["error"]["message"], "A subagent with this filename already exists. Choose another name.");
    assert_eq!(fs::read_to_string(&existing).unwrap(), original);

    let saved = api_value(&app, create_only(&renamed), StatusCode::OK).await;
    assert_eq!(saved["saved"], true);
    assert_eq!(fs::read_to_string(&renamed).unwrap(), draft);
    assert_eq!(fs::read_to_string(&existing).unwrap(), original);
    assert_eq!(fs::read_dir(&folder).unwrap().count(), 2, "only the two definitions remain");

    // A save from Edit still uses the same route without a creation condition.
    api_value(&app, request(&existing, draft), StatusCode::OK).await;
    assert_eq!(fs::read_to_string(&existing).unwrap(), draft);
}

#[tokio::test]
async fn subagent_actions_protect_the_shared_codex_config_registered_as_a_definition() {
    let fixture = Fixture::new();
    let config = fixture.project.join(".codex/config.toml");
    let content = "[agents.shared]\nconfig_file = \"config.toml\"\ndescription = \"Shared settings\"\n";
    fixture.write(&config, content);
    let (app, native) = api_fixture(&fixture);
    let session = native.open_settings();
    let uri = url("/api/v1/workspace/file", &[
        ("folder_path", fixture.folder()),
        ("file_path", config.to_str().unwrap()),
    ]);
    let denied = api_value(
        &app, api_request("DELETE", &uri, None, Some(session.header_value())), StatusCode::FORBIDDEN,
    ).await;
    assert_eq!(denied["error"]["code"], "WORKSPACE_PATH_FORBIDDEN");
    let replacement = json!({
        "folder_path": fixture.folder(),
        "file_path": config,
        "content": "name = 'replacement'\ndescription = 'Replacement'\n",
    });
    let denied = api_value(&app, api_request(
        "PUT", "/api/v1/workspace/file", Some(replacement), Some(session.header_value()),
    ), StatusCode::FORBIDDEN).await;
    assert_eq!(denied["error"]["code"], "WORKSPACE_PATH_FORBIDDEN");
    assert_eq!(fs::read_to_string(&config).unwrap(), content);
}

#[tokio::test]
async fn native_conversation_api_lists_a_whole_claude_session_and_preserves_raw_history() {
    use sha2::{Digest, Sha256};

    let snapshot_prefix = format!("fetch-session-reader-{}-", std::process::id());
    let snapshots = || {
        let mut paths: Vec<_> = fs::read_dir(std::env::temp_dir()).unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with(&snapshot_prefix))
            .map(|entry| entry.path()).collect();
        paths.sort();
        paths
    };
    let snapshots_before = snapshots();
    let mut fixture = Fixture::new();
    fixture.service = fixture.service.clone().with_readers_root(
        fetch_local::application::default_data_root().unwrap().join("session-readers"),
    );
    let session_id = "22222222-2222-4222-8222-222222222222";
    let path = fixture.service.claude_root.join("projects/api-fixture").join(format!("{session_id}.jsonl"));
    let full_reply = format!("First paragraph.\n\n{}\nLast paragraph 🦀", "Complete assistant reply. ".repeat(5000));
    let entry = |id: &str, parent: Option<&str>, kind: &str, content: Value| json!({
        "uuid":id, "parentUuid":parent, "type":kind, "sessionId":session_id,
        "cwd":fixture.project, "timestamp":"2026-11-13T12:00:00.000Z",
        "message":{"id":id,"role":kind,"content":content}
    });
    let mut injected = entry("setup", None, "user", json!("<recommended_plugins>generated setup</recommended_plugins>"));
    injected["isMeta"] = json!(true);
    let records = [
        injected,
        entry("typed-one", Some("setup"), "user", json!("Explain the fixture file.")),
        entry("call", Some("typed-one"), "assistant", json!([
            {"type":"tool_use","id":"read-fixture","name":"Read","input":{"path":"fixture.txt"}}
        ])),
        entry("result", Some("call"), "user", json!([
            {"type":"tool_result","tool_use_id":"read-fixture","content":"exact tool output\nsecond line"}
        ])),
        entry("answer", Some("result"), "assistant", json!([{"type":"text","text":full_reply}])),
        entry("typed-two", Some("answer"), "user", json!("Thanks. What happens next?")),
        entry("final", Some("typed-two"), "assistant", json!([{"type":"text","text":"The complete conversation ends here."}])),
    ];
    let original = records.iter().map(Value::to_string).collect::<Vec<_>>().join("\n") + "\n";
    fixture.write(&path, &original);
    fixture.claude_session("33333333-3333-4333-8333-333333333333", &fixture.other_project, "Other project history");
    let original_hash = Sha256::digest(original.as_bytes());
    let (app, native) = api_fixture(&fixture);
    let settings = native.open_settings();
    let capability = settings.header_value();

    let listed = api_value(&app, api_request("GET", &url("/api/v1/workspace/sessions", &[
        ("folder_path", fixture.folder()), ("app", "claude"), ("view", "conversation"),
    ]), None, Some(capability)), StatusCode::OK).await;
    assert_eq!(listed["sessions"].as_array().unwrap().len(), 1);
    assert_eq!(listed["sessions"][0]["name"], "Explain the fixture file.");
    assert_eq!(Path::new(listed["sessions"][0]["file_path"].as_str().unwrap()).canonicalize().unwrap(), path.canonicalize().unwrap());
    assert!(listed["next_cursor"].is_null());

    let conversation_uri = url("/api/v1/workspace/sessions/conversation", &[
        ("folder_path", fixture.folder()), ("file_path", path.to_str().unwrap()),
    ]);
    let result = api_value(&app, api_request("GET", &conversation_uri, None, Some(capability)), StatusCode::OK).await;
    assert_eq!(result["complete"], true);
    let messages = result["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 5);
    assert_eq!(messages.iter().map(|message| message["kind"].as_str().unwrap()).collect::<Vec<_>>(),
        ["user", "tool_result", "assistant", "user", "assistant"]);
    assert_eq!(messages[0]["text"], "Explain the fixture file.");
    assert_eq!(messages[1]["tool_name"], "Read");
    assert_eq!(messages[1]["tool_call_id"], "read-fixture");
    assert_eq!(messages[1]["output"], "exact tool output\nsecond line");
    assert_eq!(messages[2]["text"], full_reply);
    assert_eq!(messages[4]["text"], "The complete conversation ends here.");
    assert!(!messages.iter().any(|message| message.to_string().contains("generated setup")));

    // Old raw APIs still list the original file and return its original bytes.
    let raw_list = api_value(&app, api_request("GET", &url("/api/v1/workspace/sessions", &[
        ("folder_path", fixture.folder()), ("app", "claude"),
    ]), None, Some(capability)), StatusCode::OK).await;
    assert_eq!(raw_list["sessions"].as_array().unwrap().len(), 1);
    let raw = api_value(&app, api_request("GET", &url("/api/v1/workspace/sessions/read", &[
        ("folder_path", fixture.folder()), ("file_path", path.to_str().unwrap()), ("limit", "64"),
    ]), None, Some(capability)), StatusCode::OK).await;
    assert_eq!(raw["content"].as_str().unwrap().as_bytes(), &original.as_bytes()[..64]);
    assert_eq!(raw["eof"], false);
    assert_eq!(Sha256::digest(fs::read(&path).unwrap()), original_hash);

    let wrong_project = url("/api/v1/workspace/sessions/conversation", &[
        ("folder_path", fixture.other_project.to_str().unwrap()), ("file_path", path.to_str().unwrap()),
    ]);
    let denied = api_value(&app, api_request("GET", &wrong_project, None, Some(capability)), StatusCode::FORBIDDEN).await;
    assert_eq!(denied["error"]["code"], "SESSION_PROJECT_FORBIDDEN");
    assert_eq!(snapshots(), snapshots_before, "Reader snapshots must be removed before returning the response");
}

#[tokio::test]
async fn ordinary_bearer_and_background_requests_cannot_access_workspace_files_history_or_models() {
    let fixture = Fixture::new();
    let instruction = fixture.project.join("AGENTS.md");
    fixture.write(&instruction, "Private fixture instructions\n");
    let log = fixture.codex_session("Private fixture session", &fixture.project, "Private fixture history");
    let (app, _native) = api_fixture(&fixture);
    let attempts = [
        ("GET", url("/api/v1/workspace", &[("folder_path", fixture.folder())]), None),
        ("GET", url("/api/v1/workspace/file", &[("folder_path", fixture.folder()), ("file_path", instruction.to_str().unwrap())]), None),
        ("PUT", "/api/v1/workspace/file".into(), Some(json!({"folder_path":fixture.folder(),"file_path":instruction,"content":"must not write"}))),
        ("DELETE", url("/api/v1/workspace/file", &[("folder_path", fixture.folder()), ("file_path", instruction.to_str().unwrap())]), None),
        ("POST", "/api/v1/workspace/agents/convert".into(), Some(json!({"source_app":"claude","target_app":"codex","content":"must not parse"}))),
        ("GET", url("/api/v1/workspace/sessions", &[("folder_path", fixture.folder())]), None),
        ("GET", url("/api/v1/workspace/sessions/read", &[("file_path", log.to_str().unwrap())]), None),
        ("GET", url("/api/v1/workspace/sessions", &[("view", "conversation")]), None),
        ("GET", url("/api/v1/workspace/sessions/conversation", &[("file_path", log.to_str().unwrap())]), None),
        ("POST", "/api/v1/workspace/file/reveal".into(), Some(json!({"file_path":instruction}))),
        ("GET", "/api/v1/workspace/models?app=codex".into(), None),
    ];
    for auth in [None, Some(("authorization", "Bearer fixture-ordinary-grant")), Some(("x-fetch-background-auth", "fixture-background-token"))] {
        for (method, uri, body) in &attempts {
            let mut request = api_request(method, uri, body.clone(), None);
            if let Some((key, value)) = auth { request.headers_mut().insert(key, value.parse().unwrap()); }
            let denied = api_value(&app, request, StatusCode::FORBIDDEN).await;
            assert_eq!(denied["error"]["code"], "FORBIDDEN");
            assert!(!denied.to_string().contains("Private fixture"));
        }
    }
    assert_eq!(fs::read_to_string(&instruction).unwrap(), "Private fixture instructions\n");
}
