//! Executable folders owned by one managed installation, never the host's PATH.
use serde_json::Value;
use std::{fs, io::ErrorKind, path::{Component, Path, PathBuf}};

/// Keep dependency layout knowledge beside the installer that creates it.
pub(crate) fn relative_directories(config: &Value) -> Vec<String> {
    match config["kind"].as_str() {
        Some("npm") => {
            let mut paths = vec!["node_modules/.bin".into()];
            if config["package"]["name"] == "@railway/mcp-server" {
                paths.push("railway-cli/node_modules/.bin".into());
            }
            paths
        }
        Some("pypi") => vec!["venv/bin".into()],
        _ => vec![],
    }
}

/// uv owns the new tool layout. Record the actual environment's bin so its
/// dependency commands remain available, followed by uv's exported commands.
/// Existing records without this metadata continue to use the old venv layout.
pub(crate) fn python_directories(package: &Path, config: &Value) -> Result<Vec<String>, ()> {
    let executable=Path::new(config["executable"].as_str().ok_or(())?);
    let owned=fs::canonicalize(package).map_err(|_|())?;
    let bin=executable.parent().ok_or(())?.strip_prefix(&owned).map_err(|_|())?;
    let relative=bin.to_str().ok_or(())?.to_owned();
    let paths=serde_json::json!([relative,"bin"]);
    if !valid_directories(&paths){return Err(());}
    Ok(vec![relative,"bin".into()])
}

pub(crate) fn valid_directories(value: &Value) -> bool {
    value.as_array().is_some_and(|paths| paths.iter().all(|value| {
        value.as_str().is_some_and(|path| {
            !path.is_empty() && !path.contains(['\0', '\r', '\n', ':'])
                && Path::new(path).components().all(|part| matches!(part, Component::Normal(_)))
        })
    }))
}

/// Old records use the same installer-owned layout without rewriting saved state.
/// Missing folders are left for ordinary startup to report, rather than becoming
/// a prerequisite check. Existing folders must stay inside this installation.
pub(crate) fn resolve(root: &Path, record: &Value) -> Result<Vec<PathBuf>, ()> {
    let relative = match record["_installation"].get("program_directories") {
        Some(value) if valid_directories(value) => value.as_array().ok_or(())?
            .iter().map(|value| value.as_str().unwrap().to_owned()).collect(),
        Some(_) => return Err(()),
        None => relative_directories(&record["config"]),
    };
    if relative.is_empty() { return Ok(vec![]); }
    let Some(id) = record["_installation"]["installation_id"].as_str() else {
        return Ok(vec![]);
    };
    if !id.strip_prefix("package-").is_some_and(|value| value.len() == 32 && value.bytes().all(|byte| byte.is_ascii_hexdigit())) {
        return Err(());
    }
    let expected = fs::canonicalize(root).map_err(|_| ())?.join("packages/servers").join(id);
    let package = match fs::canonicalize(&expected) {
        Ok(path) => path,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(vec![]),
        Err(_) => return Err(()),
    };
    if package != expected { return Err(()); }
    let mut paths = Vec::new();
    for relative in relative {
        match fs::canonicalize(package.join(relative)) {
            Ok(path) if path.starts_with(&package) && path.is_dir() => paths.push(path),
            Ok(_) => return Err(()),
            Err(error) if error.kind() == ErrorKind::NotFound => {},
            Err(_) => return Err(()),
        }
    }
    Ok(paths)
}
