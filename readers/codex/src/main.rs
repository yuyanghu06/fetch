//! One-shot display wrapper around pinned Codex history readers. No app/server startup.
use anyhow::{Context, Result, bail, ensure};
use codex_protocol::protocol::ThreadHistoryMode;
use codex_rollout::RolloutConfig;
use codex_thread_store::{LocalThreadStore, LocalThreadStoreConfig, ListItemsParams,
    ListItemsPosition, ItemSortKey, SortDirection};
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::{Value, json};
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct ReaderFailure { code: &'static str, message: String }
impl std::fmt::Display for ReaderFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(&self.message) }
}
impl std::error::Error for ReaderFailure {}

fn allowed(path: &Path, roots: &[PathBuf]) -> Result<PathBuf> {
    ensure!(path.is_absolute(), "source path must be absolute");
    let path = path.canonicalize().context("source path does not exist")?;
    ensure!(roots.iter().any(|root| path.starts_with(root)), "source path is outside allowed roots");
    ensure!(path.is_file(), "source path is not a file");
    Ok(path)
}

fn roots(request: &Value) -> Result<Vec<PathBuf>> {
    let roots = request["source_roots"].as_array().context("source_roots is required")?;
    ensure!(!roots.is_empty(), "source_roots must not be empty");
    roots.iter().map(|p| {
        let p = PathBuf::from(p.as_str().context("source root must be a path")?);
        ensure!(p.is_absolute(), "source root must be absolute");
        let p = p.canonicalize()?;
        ensure!(p.is_dir(), "source root must be a directory");
        Ok(p)
    }).collect()
}

async fn list(request: &Value, roots: &[PathBuf]) -> Result<Value> {
    let candidates = request["candidates"].as_array().context("candidates is required")?;
    let mut sessions = Vec::new();
    for candidate in candidates {
        let path = allowed(Path::new(candidate["file_path"].as_str().context("file_path is required")?), roots)?;
        let meta = codex_rollout::read_session_meta_line(&path).await
            .with_context(|| format!("cannot read native session metadata: {}", path.display()))?.meta;
        if meta.source.is_non_root_agent() { continue; }
        let mut indexed = false;
        let mut native_name = None;
        if let Some(index) = request["source_index"].as_str() {
            let index = PathBuf::from(index);
            ensure!(index.is_absolute() && index.is_file(), "source_index must be an existing absolute SQLite file");
            if let Some(native) = codex_state::StateRuntime::fetch_get_thread_readonly(&index, meta.id).await
                .context("cannot read native current-rollout index without writes")? {
                let current = match native.rollout_path.canonicalize() {
                    Ok(path) => path,
                    // The native index can retain a deleted rollout. Do not list an
                    // older file for that thread or fail unrelated conversations.
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                    Err(error) => return Err(error).context("native selected rollout is missing"),
                };
                ensure!(roots.iter().any(|root| current.starts_with(root)), "native selected rollout lies outside allowed roots");
                if current != path { continue; }
                indexed = true;
                native_name = LocalThreadStore::fetch_native_thread_name(&native);
            }
        }
        if !indexed {
            let home = roots.iter().find(|root| path.starts_with(root)).and_then(|root| root.parent())
                .context("source session root has no parent")?;
            let native = codex_rollout::find_thread_path_by_id_str(home, &meta.id.to_string(), None).await?;
            let native = match native {
                Some(path) => Some(path),
                None => codex_rollout::find_archived_thread_path_by_id_str(home, &meta.id.to_string(), None).await?,
            };
            if let Some(current) = native {
                if allowed(&current, roots)? != path { continue; }
            }
        }
        let mut session = candidate.clone();
        ensure!(session.is_object(), "session candidate must be an object");
        if let Some(name) = native_name {
            session["name"] = json!(name);
            session["native_name"] = json!(name);
        }
        session["native_selection_source"] = json!(if indexed { "state_index" } else { "native_filesystem_fallback" });
        session["native_thread_id"] = json!(meta.id);
        session["native_source"] = serde_json::to_value(meta.source)?;
        session["native_history_mode"] = serde_json::to_value(meta.history_mode)?;
        session["native_cwd"] = json!(meta.cwd);
        sessions.push(session);
    }
    Ok(json!({"sessions": sessions}))
}

/// Follow only native history_base IDs. The official lineage reader later decides bounds/order.
async fn snapshot(source: &Path, roots: &[PathBuf], home: &Path) -> Result<PathBuf> {
    let mut pending = vec![source.to_path_buf()];
    let mut seen = HashSet::new();
    let mut selected = None;
    while let Some(path) = pending.pop() {
        let path = allowed(&path, roots)?;
        if !seen.insert(path.clone()) { continue; }
        let meta = codex_rollout::read_session_meta_line(&path).await?.meta;
        let archived = roots.iter().any(|root| root.file_name().is_some_and(|n| n == "archived_sessions") && path.starts_with(root));
        let target_root = home.join(if archived { "archived_sessions" } else { "sessions" });
        // Keep original filenames: they encode immutable rollout IDs, including reverted IDs.
        let original_root = roots.iter().find(|root| path.starts_with(root)).context("source root missing")?;
        let target = target_root.join(path.strip_prefix(original_root)?);
        std::fs::create_dir_all(target.parent().unwrap())?;
        std::fs::copy(&path, &target)?;
        if selected.is_none() { selected = Some(target.clone()); }
        // Reject parse errors rather than return complete:true after a skipped durable record.
        let mut reader = codex_rollout::open_rollout_line_reader(&target).await?;
        while let Some(line) = reader.next_line().await? {
            if !line.trim().is_empty() { codex_rollout::parse_rollout_line(&line).context("invalid native rollout record")?; }
        }
        if let Some(base) = meta.history_base {
            let mut found = None;
            for root in roots {
                let name = root.file_name().and_then(|n| n.to_str());
                ensure!(matches!(name, Some("sessions" | "archived_sessions")),
                    "native lineage lookup requires sessions or archived_sessions roots");
                let parent = root.parent().context("session root has no parent")?;
                if let Some(path) = codex_rollout::find_rollout_path_by_rollout_id(parent, base.thread_id).await? {
                    found = Some(allowed(&path, roots)?); break;
                }
            }
            pending.push(found.ok_or_else(|| ReaderFailure {
                code: "CODEX_HISTORY_ANCESTOR_MISSING",
                message: format!("native history_base source rollout {} is missing from allowed session roots", base.thread_id),
            })?);
        }
    }
    selected.context("empty snapshot")
}

/// These are presentation mappings of official UI items, never model-context parsing.
fn display(item: Value, output: &mut Vec<Value>) -> Result<()> {
    let kind = item["type"].as_str().context("native item has no type")?;
    let id = item["id"].clone();
    let message = match kind {
        "userMessage" => {
            let text = item["content"].as_array().context("native user item has no content")?.iter()
                .filter_map(|c| if c["type"] == "text" { c["text"].as_str() } else { None })
                .collect::<Vec<_>>().join("\n");
            Some(json!({"id":id,"kind":"user","text":text,"content":item["content"]}))
        },
        "agentMessage" | "plan" => Some(json!({"id":id,"kind":"assistant","text":item["text"]})),
        "functionCallOutput" => Some(json!({"id":id,"kind":"tool_result","tool_name":item["name"],"tool_call_id":id,"output":item["output"]})),
        "commandExecution" => Some(json!({"id":id,"kind":"tool_result","tool_name":"command_execution","tool_call_id":id,"output":item["aggregatedOutput"],"command":item["command"],"exit_code":item["exitCode"]})),
        "mcpToolCall" => Some(json!({"id":id,"kind":"tool_result","tool_name":format!("{}.{}",item["server"].as_str().unwrap_or_default(),item["tool"].as_str().unwrap_or_default()),"tool_call_id":id,"output":item["result"],"error":item["error"]})),
        "dynamicToolCall" => Some(json!({"id":id,"kind":"tool_result","tool_name":item["tool"],"tool_call_id":id,"output":item["contentItems"],"success":item["success"]})),
        "fileChange" => Some(json!({"id":id,"kind":"tool_result","tool_name":"apply_patch","tool_call_id":id,"output":item["changes"],"status":item["status"]})),
        "collabAgentToolCall" => Some(json!({"id":id,"kind":"tool_result","tool_name":item["tool"],"tool_call_id":id,"output":item["agentsStates"]})),
        "exitedReviewMode" => Some(json!({"id":id,"kind":"assistant","text":item["review"]})),
        "imageView" | "sleep" | "imageGeneration" => Some(json!({"id":id,"kind":"tool_result","tool_name":kind,"tool_call_id":id,"output":item})),
        "webSearch" => Some(json!({"id":id,"kind":"tool_result","tool_name":"web_search","tool_call_id":id,"output":item})),
        // Native reasoning, plans, hooks, settings, lifecycle and context are not conversation messages.
        _ => None,
    };
    if let Some(message) = message { output.push(message); }
    Ok(())
}

async fn conversation(request: &Value, roots: &[PathBuf]) -> Result<Value> {
    let source = allowed(Path::new(request["file_path"].as_str().context("file_path is required")?), roots)?;
    let work = PathBuf::from(request["work_dir"].as_str().context("work_dir is required")?);
    ensure!(work.is_absolute(), "work_dir must be absolute");
    let work = work.canonicalize().context("work_dir must exist")?;
    ensure!(roots.iter().all(|root| !work.starts_with(root) && !root.starts_with(&work)), "work_dir must be separate from original sessions");
    let home = work.join("codex-snapshot");
    std::fs::create_dir(&home).context("work_dir must be fresh for each request")?;
    let path = snapshot(&source, roots, &home).await?;
    let meta = codex_rollout::read_session_meta_line(&path).await?.meta;
    if meta.source.is_non_root_agent() {
        return Err(ReaderFailure { code: "CODEX_NOT_MAIN_CONVERSATION", message: "selected rollout is a native internal or subagent session, not a main conversation".into() }.into());
    }
    let sqlite = codex_state::SqliteConfig::from_sqlite_home_with_nfs_enforcement(AbsolutePathBuf::try_from(home.clone())?, false);
    let config = RolloutConfig {codex_home: home.clone(), sqlite, cwd: work, model_provider_id: "openai".into(), generate_memories: false};
    let state = codex_rollout::state_db::try_init(&config).await.context("cannot initialize private native history store")?;
    // Backfill discovers headers. Pin this exact requested immutable file for the page reader.
    let mut metadata = state.get_thread(meta.id).await?.context("private native metadata backfill did not find selected thread")?;
    metadata.rollout_path = path.clone();
    metadata.history_mode = meta.history_mode;
    state.upsert_thread(&metadata).await?;
    let store = LocalThreadStore::new(LocalThreadStoreConfig::from_config(&config), Some(state));
    let mut messages = Vec::new();
    match meta.history_mode {
        ThreadHistoryMode::Legacy => {
            let thread = store.read_thread_by_rollout_path(path, true, true).await?;
            let history = thread.history.context("native legacy reader returned no history")?;
            for turn in codex_app_server_protocol::build_turns_from_rollout_items(&history.items) {
                for item in turn.items { display(serde_json::to_value(item)?, &mut messages)?; }
            }
        },
        ThreadHistoryMode::Paginated => {
            store.fetch_materialize_history(meta.id, path).await?;
            let mut cursor = None;
            loop {
                let page = store.list_items(ListItemsParams { item_ids: None, thread_id: meta.id, turn_id: None,
                    include_archived: true, position: cursor.map(ListItemsPosition::Cursor), page_size: 256,
                    sort_direction: SortDirection::Asc, sort_key: ItemSortKey::CreatedAtOrdinal,
                    after_updated_at_ordinal: None }).await?;
                for item in page.items { display(serde_json::from_slice(&item.item_json)?, &mut messages)?; }
                cursor = page.next_cursor;
                if cursor.is_none() { break; }
            }
        }
    }
    Ok(json!({"messages":messages,"complete":true,"native_thread_id":meta.id}))
}

async fn run() -> Result<Value> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let request: Value = serde_json::from_str(&input).context("invalid request JSON")?;
    let roots = roots(&request)?;
    match request["op"].as_str() {
        Some("list") => list(&request, &roots).await,
        Some("conversation") => conversation(&request, &roots).await,
        _ => bail!("op must be list or conversation"),
    }
}

#[tokio::main]
async fn main() {
    let (response, failed) = match run().await {
        Ok(value) => (value, false),
        Err(error) => {
            let code = error.downcast_ref::<ReaderFailure>().map_or_else(
                || if format!("{error:#}").contains("invalid session metadata history_mode") { "CODEX_HISTORY_VERSION_UNSUPPORTED" } else { "CODEX_READER_FAILED" },
                |failure| failure.code,
            );
            (json!({"error":{"code":code,"message":format!("{error:#}")}}), true)
        },
    };
    let mut stdout = std::io::stdout().lock();
    let _ = serde_json::to_writer(&mut stdout, &response);
    let _ = stdout.write_all(b"\n");
    if failed { std::process::exit(1); }
}
