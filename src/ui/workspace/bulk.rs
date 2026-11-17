//! Transfer subagent definitions through the existing Workspace file APIs.
use super::{NativeClient, query};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Component, Path, PathBuf};

const BUNDLE_VERSION: u32 = 1;
pub(in crate::ui) const COPY_AGENT_FIRST: &str = "Copy a subagent or use Copy all in Fetch before pasting here.";

#[derive(Serialize, Deserialize)]
struct Bundle {
    fetch_agents: u32,
    app: String,
    agents: Vec<BundleEntry>,
}

#[derive(Serialize, Deserialize)]
struct BundleEntry {
    relative_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    content: String,
}

#[derive(Clone, Debug)]
pub(in crate::ui) struct PreparedAgent {
    pub(in crate::ui) destination: String,
    pub(in crate::ui) content: Result<String, String>,
    pub(in crate::ui) issues: Vec<Value>,
}

#[derive(Clone, Debug)]
pub(in crate::ui) struct SaveResult {
    pub(in crate::ui) destination: String,
    pub(in crate::ui) result: Result<Value, String>,
}

fn app_extension(app: &str) -> Option<&'static str> {
    match app {
        "claude" => Some("md"),
        "codex" => Some("toml"),
        _ => None,
    }
}

fn relative_path(path: &str) -> Result<&Path, String> {
    let relative = Path::new(path);
    if path.contains('\0') || relative.file_name().is_none()
        || relative.components().any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err("Copied subagent paths must stay inside the destination agent folder.".into());
    }
    Ok(relative)
}

fn source_relative_path<'a>(path: &'a str, agent_directory: &str) -> Result<&'a str, String> {
    let source = Path::new(path);
    let filename = source
        .strip_prefix(agent_directory)
        .ok()
        .filter(|relative| !relative.as_os_str().is_empty())
        .or_else(|| source.file_name().map(Path::new))
        .and_then(Path::to_str)
        .ok_or_else(|| "A subagent filename could not be copied.".to_owned())?;
    relative_path(filename)?;
    Ok(filename)
}

pub(in crate::ui) fn copied_agent_payload(app: &str, name: Option<&str>, path: &str, file: &Value, agent_directory: &str) -> String {
    let relative_path = source_relative_path(path, agent_directory).ok();
    json!({
        "fetch_agent": 1,
        "app": app,
        "relative_path": relative_path,
        "name": name,
        "description": file["registered_description"].as_str(),
        "content": file["content"],
    }).to_string()
}

pub(in crate::ui) async fn copy(
    client: NativeClient,
    folder: Option<String>,
    source_app: String,
    agent_directory: String,
    sources: Vec<(String, Option<String>)>,
) -> Result<String, String> {
    let mut agents = Vec::with_capacity(sources.len());
    for (path, name) in sources {
        let filename = source_relative_path(&path, &agent_directory)?;
        let url = format!("/api/v1/workspace/file{}", query(folder.as_deref(), &[("file_path", &path)]));
        let file = client.request(Method::GET, &url, None).await?;
        let content = file["content"].as_str()
            .ok_or_else(|| "The Workspace response did not contain the subagent definition.".to_owned())?;
        agents.push(BundleEntry {
            relative_path: filename.to_owned(),
            name,
            description: file["registered_description"].as_str().map(str::to_owned),
            content: content.to_owned(),
        });
    }
    serde_json::to_string(&Bundle { fetch_agents: BUNDLE_VERSION, app: source_app, agents })
        .map_err(|error| error.to_string())
}

fn read_bundle(clipboard: &str) -> Result<Bundle, String> {
    if let Ok(bundle) = serde_json::from_str::<Bundle>(clipboard) {
        return Ok(bundle);
    }
    let entry = super::copied_agent(clipboard).ok_or(COPY_AGENT_FIRST)?;
    let text = |field: &str| entry[field].as_str().map(str::to_owned);
    let app = text("app").unwrap_or_default();
    let extension = app_extension(&app).ok_or(COPY_AGENT_FIRST)?;
    let name = text("name");
    let relative_path = text("relative_path")
        .or_else(|| {
            name.as_deref()
                .filter(|name| !name.trim().is_empty())
                .map(|name| format!("{name}.{extension}"))
        })
        .unwrap_or_default();
    Ok(Bundle {
        fetch_agents: BUNDLE_VERSION,
        app,
        agents: vec![BundleEntry {
            relative_path,
            name,
            description: text("description"),
            content: text("content").unwrap_or_default(),
        }],
    })
}

pub(in crate::ui) async fn prepare(
    client: NativeClient,
    source: String,
    target_app: String,
    target_directory: String,
) -> Result<Vec<PreparedAgent>, String> {
    let bundle = read_bundle(&source)?;
    prepare_bundle(&client, bundle, &target_app, &target_directory, None).await
}

async fn prepare_bundle(
    client: &NativeClient,
    bundle: Bundle,
    target_app: &str,
    target_directory: &str,
    selected_path: Option<&str>,
) -> Result<Vec<PreparedAgent>, String> {
    if bundle.fetch_agents != BUNDLE_VERSION || app_extension(&bundle.app).is_none() || bundle.agents.is_empty() {
        return Err(COPY_AGENT_FIRST.into());
    }
    let extension = app_extension(target_app).ok_or("Choose Claude or Codex before pasting subagents.")?;
    let directory = PathBuf::from(target_directory);
    let destinations = bundle.agents.iter().enumerate().map(|(index, entry)| {
        if index == 0 && let Some(path) = selected_path {
            return Ok(path.to_owned());
        }
        let relative = relative_path(&entry.relative_path)?;
        directory.join(relative).with_extension(extension).into_os_string().into_string()
            .map_err(|_| "The destination subagent path could not be read.".to_owned())
    }).collect::<Result<Vec<_>, String>>()?;

    let mut prepared = Vec::with_capacity(bundle.agents.len());
    for (entry, destination) in bundle.agents.into_iter().zip(destinations) {
        let (content, issues) = if bundle.app == target_app {
            (Ok(entry.content), Vec::new())
        } else {
            let body = json!({
                "source_app": bundle.app,
                "target_app": target_app,
                "content": entry.content,
                "fallback_name": entry.name,
                "fallback_description": entry.description,
            });
            match client.request(Method::POST, "/api/v1/workspace/agents/convert", Some(body)).await {
                Ok(response) => {
                    let content = response["content"].as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| "The Workspace response did not contain the converted subagent.".to_owned());
                    (content, response["issues"].as_array().cloned().unwrap_or_default())
                }
                Err(error) => (Err(error), Vec::new()),
            }
        };
        prepared.push(PreparedAgent { destination, content, issues });
    }
    Ok(prepared)
}

pub(in crate::ui) async fn paste_on_agent(
    client: NativeClient,
    folder: Option<String>,
    clipboard: String,
    target_app: String,
    agent_directory: String,
    selected_path: String,
) -> Result<Vec<SaveResult>, String> {
    let bundle = read_bundle(&clipboard)?;
    let agents = prepare_bundle(&client, bundle, &target_app, &agent_directory, Some(&selected_path)).await?;
    Ok(save_prepared(&client, folder, agents, true).await)
}

pub(in crate::ui) async fn save(
    client: NativeClient,
    folder: Option<String>,
    agents: Vec<PreparedAgent>,
) -> Vec<SaveResult> {
    save_prepared(&client, folder, agents, false).await
}

async fn save_prepared(
    client: &NativeClient,
    folder: Option<String>,
    agents: Vec<PreparedAgent>,
    replace_first: bool,
) -> Vec<SaveResult> {
    let mut results = Vec::with_capacity(agents.len());
    for (index, agent) in agents.into_iter().enumerate() {
        let result = match agent.content {
            Ok(content) if agent.issues.is_empty() => {
                let body = json!({"file_path":agent.destination,"folder_path":folder,"content":content});
                if replace_first && index == 0 {
                    client.request(Method::PUT, "/api/v1/workspace/file", Some(body)).await
                } else {
                    client.save_new_workspace_file(body).await
                }
            }
            Ok(_) => {
                let mut message = "Copy and paste this subagent individually to review its settings.".to_owned();
                for issue in agent.issues {
                    if let Some(detail) = issue["message"].as_str() {
                        message.push('\n');
                        message.push_str(detail);
                    }
                }
                Err(message)
            }
            Err(error) => Err(error),
        };
        results.push(SaveResult { destination: agent.destination, result });
    }
    results
}
