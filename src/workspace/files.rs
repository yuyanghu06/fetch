//! Discover and edit the existing native instruction and agent files.
use super::{App, MAX_FILE_BYTES, WorkspaceError, WorkspaceService, absolute_path, resolved_path, success};
use serde_json::{Value, json};
use std::{fs, io::{Read, Write}, path::{Path, PathBuf}};

struct Location { path: PathBuf, root: PathBuf, app: &'static str, kind: &'static str }

fn locations(service: &WorkspaceService, folder: Option<&Path>) -> Vec<Location> {
    let mut locations = Vec::new();
    let mut add = |path: PathBuf, root: PathBuf, app, kind| locations.push(Location {path,root,app,kind});
    if let Some(root) = folder {
        add(root.join("AGENTS.md"),root.to_path_buf(),"shared","instructions");
        add(root.join("CLAUDE.md"),root.to_path_buf(),"claude","instructions");
        add(root.join(".claude/CLAUDE.md"),root.to_path_buf(),"claude","instructions");
        add(root.join(".claude/AGENTS.md"),root.to_path_buf(),"shared","instructions");
        add(root.join(".claude/agents"),root.to_path_buf(),"claude","agent");
        add(root.join(".codex/agents"),root.to_path_buf(),"codex","agent");
    } else {
        add(service.codex_root.join("AGENTS.md"),service.codex_root.clone(),"codex","instructions");
        add(service.claude_root.join("CLAUDE.md"),service.claude_root.clone(),"claude","instructions");
        add(service.claude_root.join("agents"),service.claude_root.clone(),"claude","agent");
        add(service.codex_root.join("agents"),service.codex_root.clone(),"codex","agent");
    }
    locations
}

fn agent_extension(path: &Path, app: &str) -> bool {
    path.extension().and_then(|value|value.to_str()) == Some(if app=="claude" {"md"} else {"toml"})
}

fn collect_agents(root: &Path, directory: &Path, app: &str, output: &mut Vec<Value>) -> Result<(),WorkspaceError> {
    if !directory.exists() { return Ok(()); }
    let allowed = resolved_path(root)?;
    let canonical = directory.canonicalize().map_err(WorkspaceError::io)?;
    if !canonical.starts_with(&allowed) { return Ok(()); }
    let mut entries = fs::read_dir(directory).map_err(WorkspaceError::io)?.collect::<Result<Vec<_>,_>>().map_err(WorkspaceError::io)?;
    entries.sort_by_key(|entry|entry.file_name());
    for entry in entries {
        let path=entry.path();
        // Do not recursively follow directory links; native agent roots may be
        // chosen explicitly, but an arbitrary nested link is not a new root.
        let metadata=entry.file_type().map_err(WorkspaceError::io)?;
        if metadata.is_dir() { collect_agents(root,&path,app,output)?; }
        else if (metadata.is_file() || metadata.is_symlink()) && agent_extension(&path,app) {
            if path.canonicalize().is_ok_and(|path|path.starts_with(&allowed)&&path.is_file()) {
                output.push(descriptor(&path,app,"agent",true));
            }
        }
    }
    Ok(())
}

fn descriptor(path: &Path, app: &str, kind: &str, exists: bool) -> Value {
    json!({"file_path":path,"app":app,"kind":kind,"name":path.file_stem().and_then(|name|name.to_str()).unwrap_or(""),"exists":exists})
}

struct CodexRegistration {
    name: String,
    path: PathBuf,
    resolved_file: PathBuf,
    description: Option<String>,
}

fn codex_config_paths(service: &WorkspaceService, folder: Option<&Path>) -> Vec<PathBuf> {
    let global = service.codex_root.join("config.toml");
    let project = folder.map(|path| path.join(".codex/config.toml"));
    std::iter::once(global).chain(project)
        .flat_map(|path| [resolved_path(&path).ok(), resolved_leaf_path(&path).ok()])
        .flatten()
        .collect()
}

fn codex_registrations(service: &WorkspaceService, folder: Option<&Path>) -> Vec<CodexRegistration> {
    let codex_root = folder.map(|root| root.join(".codex")).unwrap_or_else(|| service.codex_root.clone());
    let Ok(scope) = resolved_path(folder.unwrap_or(&service.codex_root)) else { return Vec::new(); };
    let Ok(config_path) = codex_root.join("config.toml").canonicalize() else { return Vec::new(); };
    if !config_path.starts_with(&scope) {
        return Vec::new();
    }
    let Some(config) = read_text(&config_path).ok().and_then(|text| toml::from_str::<toml::Value>(&text).ok()) else {
        return Vec::new();
    };
    let Some(roles) = config.get("agents").and_then(toml::Value::as_table) else { return Vec::new(); };
    roles.iter().filter_map(|(name, role)| {
        let config_file = Path::new(role.get("config_file")?.as_str()?);
        let path = codex_root.join(config_file);
        let resolved_file = path.canonicalize().ok()?;
        if resolved_file == config_path || !resolved_file.starts_with(&scope) || !resolved_file.is_file() {
            return None;
        }
        let description = role.get("description").and_then(toml::Value::as_str)
            .filter(|text| !text.trim().is_empty()).map(str::to_owned);
        Some(CodexRegistration { name: name.clone(), path, resolved_file, description })
    }).collect()
}

pub(super) fn discover(service: &WorkspaceService, folder_path: Option<&str>) -> Result<Value,WorkspaceError> {
    let folder=service.folder(folder_path)?;
    let mut files=Vec::new();
    let locations=locations(service,folder.as_deref());
    let agent_directories:Vec<Value>=locations.iter().filter(|location|location.kind=="agent")
        .map(|location|json!({"app":location.app,"file_path":location.path})).collect();
    for location in &locations {
        if location.kind=="agent" { collect_agents(&location.root,&location.path,location.app,&mut files)?; }
        else {
            if location.path.exists() && !location.path.canonicalize().is_ok_and(|path|resolved_path(&location.root).is_ok_and(|root|path.starts_with(root))&&path.is_file()) { continue; }
            files.push(descriptor(&location.path,location.app,location.kind,location.path.exists()));
        }
    }
    // Older Codex installations register their role files in config.toml.
    // Read only that registration table, never return other client settings.
    for registration in codex_registrations(service, folder.as_deref()) {
        if !files.iter().any(|file| file["file_path"].as_str() == registration.path.to_str()) {
            let mut file = descriptor(&registration.path, "codex", "agent", true);
            file["name"] = json!(registration.name);
            files.push(file);
        }
    }
    let config_paths = codex_config_paths(service, folder.as_deref());
    files.retain(|file| !file["file_path"].as_str().is_some_and(|path| {
        resolved_path(Path::new(path)).is_ok_and(|path| config_paths.contains(&path))
    }));
    Ok(success(json!({"folder_path":folder,"files":files,"agent_directories":agent_directories})))
}

fn approved(service: &WorkspaceService, folder_path: Option<&str>, file_path: &str) -> Result<(PathBuf,&'static str,&'static str),WorkspaceError> {
    let folder=service.folder(folder_path)?;
    // macOS exposes equivalent /var and /private/var paths. Resolve both the
    // requested file and the known locations before comparing their scope;
    // resolving only the selected folder incorrectly rejects normal saves.
    let path=resolved_path(&absolute_path(file_path)?)?;
    if codex_config_paths(service, folder.as_deref()).contains(&path) {
        return Err(WorkspaceError::new(403, "WORKSPACE_PATH_FORBIDDEN", "The shared Codex configuration is not a subagent definition."));
    }
    for location in locations(service,folder.as_deref()) {
        let location_path=resolved_path(&location.path)?;
        let matched=if location.kind=="instructions" {path==location_path} else {path.starts_with(&location_path)&&path!=location_path&&agent_extension(&path,location.app)};
        if matched {
            let root=resolved_path(&location.root)?;
            if path.starts_with(root) {return Ok((path,location.app,location.kind));}
            return Err(WorkspaceError::new(403,"WORKSPACE_PATH_FORBIDDEN","The selected file points outside this workspace."));
        }
    }
    // An explicitly registered legacy role may live elsewhere inside this scope.
    let listing=discover(service,folder_path)?;
    if let Some(file)=listing["files"].as_array().and_then(|files|files.iter().find(|file|file["kind"]=="agent"&&file["file_path"].as_str().is_some_and(|value|resolved_path(Path::new(value)).is_ok_and(|registered|registered==path)))) {
        if file["app"]=="codex" {return Ok((path,"codex","agent"));}
    }
    Err(WorkspaceError::new(403,"WORKSPACE_PATH_FORBIDDEN","Choose an instruction or subagent definition file from this workspace."))
}

fn read_text(path: &Path) -> Result<String,WorkspaceError> {
    let file=fs::File::open(path).map_err(WorkspaceError::io)?;
    if !file.metadata().map_err(WorkspaceError::io)?.is_file() {return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_FILE","Choose a regular text file."));}
    let mut bytes=Vec::new();file.take(MAX_FILE_BYTES+1).read_to_end(&mut bytes).map_err(WorkspaceError::io)?;
    if bytes.len() as u64>MAX_FILE_BYTES {return Err(WorkspaceError::new(413,"WORKSPACE_FILE_TOO_LARGE","This instruction or agent file exceeds 1 MiB."));}
    String::from_utf8(bytes).map_err(|_|WorkspaceError::new(422,"INVALID_WORKSPACE_FILE","This file is not UTF-8 text."))
}

pub(super) fn read(service: &WorkspaceService, folder_path: Option<&str>, file_path: &str) -> Result<Value,WorkspaceError> {
    let (path,app,kind)=approved(service,folder_path,file_path)?;
    let mut reply = json!({"file_path":path,"app":app,"kind":kind,"content":read_text(&path)?});
    if app == "codex" && kind == "agent" {
        let folder = service.folder(folder_path)?;
        if let Some(description) = codex_registrations(service, folder.as_deref()).into_iter()
            .find(|registration| registration.resolved_file == path)
            .and_then(|registration| registration.description)
        {
            reply["registered_description"] = json!(description);
        }
    }
    Ok(success(reply))
}

fn resolved_leaf_path(path: &Path) -> Result<PathBuf, WorkspaceError> {
    let (Some(parent), Some(name)) = (path.parent(), path.file_name()) else {
        return Err(WorkspaceError::new(400, "INVALID_WORKSPACE_PATH", "Choose a subagent file."));
    };
    Ok(resolved_path(parent)?.join(name))
}

pub(super) fn delete_agent_file(
    service: &WorkspaceService, folder_path: Option<&str>, file_path: &str,
) -> Result<Value, WorkspaceError> {
    let selected = absolute_path(file_path)?;
    let leaf = resolved_leaf_path(&selected)?;
    let folder = service.folder(folder_path)?;
    let config_paths = codex_config_paths(service, folder.as_deref());
    if config_paths.contains(&leaf) || resolved_path(&leaf).is_ok_and(|path| config_paths.contains(&path)) {
        return Err(WorkspaceError::new(403, "WORKSPACE_PATH_FORBIDDEN", "The shared Codex configuration is not a subagent definition."));
    }
    let metadata = fs::symlink_metadata(&leaf);
    let app = if metadata.as_ref().is_ok_and(|entry| entry.is_symlink()) {
        // Reading may resolve a link to an authorized target. Deleting a link
        // instead requires the link's own leaf to be listed as an agent.
        let listing = discover(service, folder_path)?;
        let listed = listing["files"].as_array().into_iter().flatten().find(|file| {
            file["kind"] == "agent" && file["file_path"].as_str().is_some_and(|path| {
                resolved_leaf_path(Path::new(path)).is_ok_and(|listed| listed == leaf)
            })
        });
        let app = listed.and_then(|file| file["app"].as_str()).ok_or_else(|| {
            WorkspaceError::new(403, "WORKSPACE_PATH_FORBIDDEN", "Choose a subagent file listed in this workspace.")
        })?;
        App::parse(app)?.as_str()
    } else {
        let (_, app, kind) = approved(service, folder_path, file_path)?;
        if kind != "agent" {
            return Err(WorkspaceError::new(400, "INVALID_REQUEST", "Choose a subagent file to delete."));
        }
        if !metadata.map_err(WorkspaceError::io)?.is_file() {
            return Err(WorkspaceError::new(400, "INVALID_WORKSPACE_FILE", "Choose a regular subagent file."));
        }
        app
    };
    fs::remove_file(&leaf).map_err(WorkspaceError::io)?;
    Ok(success(json!({"file_path":leaf,"app":app,"kind":"agent","deleted":true})))
}

/// Validate identically to the editor, then ask Finder to select that file.
pub(super) fn reveal_path(service:&WorkspaceService,folder_path:Option<&str>,file_path:&str)->Result<PathBuf,WorkspaceError>{
    let (path,_,_)=approved(service,folder_path,file_path)?;
    let path=path.canonicalize().map_err(WorkspaceError::io)?;
    if !path.is_file(){return Err(WorkspaceError::new(400,"INVALID_WORKSPACE_FILE","Choose a regular instruction or subagent file."));}
    Ok(path)
}
pub(super) async fn reveal(service:&WorkspaceService,folder_path:Option<&str>,file_path:&str)->Result<Value,WorkspaceError>{
    let path=reveal_path(service,folder_path,file_path)?;
    reveal_with_command(path,tokio::process::Command::new("/usr/bin/open")).await
}
/// Fixtures supply their own harmless executable; they never launch Finder.
pub(super) async fn reveal_with_command(path:PathBuf,mut command:tokio::process::Command)->Result<Value,WorkspaceError>{
    command.arg("-R").arg(&path).kill_on_drop(true);
    let output=command.output().await.map_err(|error|WorkspaceError::new(503,"WORKSPACE_REVEAL_FAILED",error.to_string()))?;
    if !output.status.success(){
        return Err(WorkspaceError::new(503,"WORKSPACE_REVEAL_FAILED",crate::runner::startup_diagnostics::sanitize_text(&String::from_utf8_lossy(&output.stderr),&[])));
    }
    Ok(json!({"status":"revealed","file_path":path,"next_steps":[]}))
}

fn occupied_agent_file() -> WorkspaceError {
    WorkspaceError::new(
        412,
        "WORKSPACE_FILE_ALREADY_EXISTS",
        "A subagent with this filename already exists. Choose another name.",
    )
}

pub(super) fn save(service: &WorkspaceService, folder_path: Option<&str>, file_path: &str, content: &str, create_only: bool) -> Result<Value,WorkspaceError> {
    if content.len() as u64>MAX_FILE_BYTES {return Err(WorkspaceError::new(413,"WORKSPACE_FILE_TOO_LARGE","This instruction or agent file exceeds 1 MiB."));}
    let (path,app,kind)=approved(service,folder_path,file_path)?;
    if create_only {
        if kind != "agent" {
            return Err(WorkspaceError::new(400, "INVALID_REQUEST", "Choose a subagent file to create."));
        }
        match fs::symlink_metadata(&path) {
            Ok(_) => return Err(occupied_agent_file()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(WorkspaceError::io(error)),
        }
    }
    if kind=="agent" {super::agents::validate_save(App::parse(app)?,content,create_only || !path.exists())?;}
    let parent=path.parent().ok_or_else(||WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","Choose a file inside this workspace."))?;
    fs::create_dir_all(parent).map_err(WorkspaceError::io)?;
    // Recheck after creating parents, before writing through them.
    let (checked_path, _, _) = approved(service,folder_path,file_path)?;
    if create_only && checked_path != path {
        return Err(WorkspaceError::new(403, "WORKSPACE_PATH_FORBIDDEN", "The selected destination changed before Fetch could save it."));
    }
    let destination = if !create_only && path.exists() {
        path.canonicalize().map_err(WorkspaceError::io)?
    } else {
        path.clone()
    };
    let parent=destination.parent().unwrap();
    let temporary=parent.join(format!(".fetch-workspace-{:016x}.tmp",rand::random::<u64>()));
    let mut options=fs::OpenOptions::new();options.write(true).create_new(true);
    #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
    let mut file=options.open(&temporary).map_err(WorkspaceError::io)?;
    let result=(|| {
        if !create_only && let Ok(metadata) = fs::metadata(&destination) {
            file.set_permissions(metadata.permissions()).map_err(WorkspaceError::io)?;
        }
        file.write_all(content.as_bytes()).map_err(WorkspaceError::io)?;file.sync_all().map_err(WorkspaceError::io)?;
        if create_only {
            // Publish the completed file only if the destination is still free.
            fs::hard_link(&temporary, &destination).map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    occupied_agent_file()
                } else {
                    WorkspaceError::io(error)
                }
            })?;
        } else {
            fs::rename(&temporary,&destination).map_err(WorkspaceError::io)?;
        }
        Ok::<_,WorkspaceError>(())
    })();
    if create_only || result.is_err(){let _=fs::remove_file(&temporary);}result?;
    Ok(success(json!({"file_path":path,"app":app,"kind":kind,"saved":true})))
}

#[cfg(test)]
mod reveal_tests {
    use super::*;
    #[tokio::test]
    async fn reveal_selects_authorized_file_and_keeps_it_unchanged_without_finder(){
        use std::os::unix::fs::PermissionsExt;
        let fixture=tempfile::tempdir().unwrap();let home=fixture.path().join("home");
        let codex=home.join(".codex");fs::create_dir_all(&codex).unwrap();
        let path=codex.join("AGENTS.md");fs::write(&path,"original instructions").unwrap();
        let service=WorkspaceService::new(home.clone(),codex,home.join(".claude"));
        let selected=reveal_path(&service,None,path.to_str().unwrap()).unwrap();
        let executable=fixture.path().join("fake-finder");
        fs::write(&executable,"#!/bin/sh\ntest \"$1\" = '-R' && test -f \"$2\"\n").unwrap();
        fs::set_permissions(&executable,fs::Permissions::from_mode(0o700)).unwrap();
        let value=reveal_with_command(selected.clone(),tokio::process::Command::new(&executable)).await.unwrap();
        assert_eq!(value,json!({"status":"revealed","file_path":selected,"next_steps":[]}));
        assert_eq!(fs::read_to_string(&path).unwrap(),"original instructions");
        fs::remove_file(&path).unwrap();
        assert_eq!(reveal_path(&service,None,path.to_str().unwrap()).unwrap_err().http_status,404);
        let outside=fixture.path().join("other.md");fs::write(&outside,"outside").unwrap();
        assert_eq!(reveal_path(&service,None,outside.to_str().unwrap()).unwrap_err().code,"WORKSPACE_PATH_FORBIDDEN");
    }
}
