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
    let codex_root=folder.as_ref().map(|root|root.join(".codex")).unwrap_or_else(||service.codex_root.clone());
    let config_path=codex_root.join("config.toml");
    let config_scope=folder.as_ref().unwrap_or(&service.codex_root);
    if config_path.is_file() && config_path.canonicalize().is_ok_and(|path|resolved_path(config_scope).is_ok_and(|root|path.starts_with(root))) {
        if let Ok(text)=read_text(&config_path) {
            if let Ok(config)=toml::from_str::<toml::Value>(&text) {
                if let Some(roles)=config.get("agents").and_then(toml::Value::as_table) {
                    for (name,role) in roles {
                        let Some(value)=role.get("config_file").and_then(toml::Value::as_str) else {continue;};
                        let path=if Path::new(value).is_absolute(){PathBuf::from(value)}else{codex_root.join(value)};
                        let root=folder.as_ref().cloned().unwrap_or_else(||service.codex_root.clone());
                        if path.is_file() && path.canonicalize().is_ok_and(|path|resolved_path(&root).is_ok_and(|root|path.starts_with(root))) && !files.iter().any(|file|file["file_path"].as_str()==path.to_str()) {
                            let mut file=descriptor(&path,"codex","agent",true);file["name"]=json!(name);files.push(file);
                        }
                    }
                }
            }
        }
    }
    Ok(success(json!({"folder_path":folder,"files":files,"agent_directories":agent_directories})))
}

fn approved(service: &WorkspaceService, folder_path: Option<&str>, file_path: &str) -> Result<(PathBuf,&'static str,&'static str),WorkspaceError> {
    let folder=service.folder(folder_path)?;
    // macOS exposes equivalent /var and /private/var paths. Resolve both the
    // requested file and the known locations before comparing their scope;
    // resolving only the selected folder incorrectly rejects normal saves.
    let path=resolved_path(&absolute_path(file_path)?)?;
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
    Ok(success(json!({"file_path":path,"app":app,"kind":kind,"content":read_text(&path)?})))
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

pub(super) fn save(service: &WorkspaceService, folder_path: Option<&str>, file_path: &str, content: &str) -> Result<Value,WorkspaceError> {
    if content.len() as u64>MAX_FILE_BYTES {return Err(WorkspaceError::new(413,"WORKSPACE_FILE_TOO_LARGE","This instruction or agent file exceeds 1 MiB."));}
    let (path,app,kind)=approved(service,folder_path,file_path)?;
    if kind=="agent" {super::agents::validate_save(App::parse(app)?,content,!path.exists())?;}
    let parent=path.parent().ok_or_else(||WorkspaceError::new(400,"INVALID_WORKSPACE_PATH","Choose a file inside this workspace."))?;
    fs::create_dir_all(parent).map_err(WorkspaceError::io)?;
    // Recheck after creating parents, before writing through them.
    approved(service,folder_path,file_path)?;
    let destination=if path.exists(){path.canonicalize().map_err(WorkspaceError::io)?}else{path.clone()};
    let parent=destination.parent().unwrap();
    let temporary=parent.join(format!(".fetch-workspace-{:016x}.tmp",rand::random::<u64>()));
    let result=(|| {
        let mut options=fs::OpenOptions::new();options.write(true).create_new(true);
        #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;options.mode(0o600);}
        let mut file=options.open(&temporary).map_err(WorkspaceError::io)?;
        if let Ok(metadata)=fs::metadata(&destination){file.set_permissions(metadata.permissions()).map_err(WorkspaceError::io)?;}
        file.write_all(content.as_bytes()).map_err(WorkspaceError::io)?;file.sync_all().map_err(WorkspaceError::io)?;
        fs::rename(&temporary,&destination).map_err(WorkspaceError::io)?;
        Ok::<_,WorkspaceError>(())
    })();
    if result.is_err(){let _=fs::remove_file(&temporary);}result?;
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
