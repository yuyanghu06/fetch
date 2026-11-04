use super::program_paths;
use serde_json::{Value, json};
use tempfile::TempDir;

fn record(kind: &str, name: &str) -> Value {
    json!({"config":{"kind":kind,"package":{"name":name}},
        "_installation":{"installation_id":format!("package-{}", "a".repeat(32))}})
}

fn package(root: &std::path::Path) -> std::path::PathBuf {
    root.join("packages/servers").join(format!("package-{}", "a".repeat(32)))
}

#[test]
fn installer_records_package_bins_and_its_prepared_helper_without_host_lookup() {
    assert_eq!(program_paths::relative_directories(&record("npm", "ordinary")["config"]), ["node_modules/.bin"]);
    assert_eq!(program_paths::relative_directories(&record("npm", "@railway/mcp-server")["config"]), ["node_modules/.bin", "railway-cli/node_modules/.bin"]);
    assert_eq!(program_paths::relative_directories(&record("pypi", "ordinary")["config"]), ["venv/bin"]);
    assert!(program_paths::relative_directories(&record("local_script", "unused")["config"]).is_empty());
    assert!(program_paths::relative_directories(&record("remote", "unused")["config"]).is_empty());
}

#[test]
fn legacy_railway_resolves_existing_private_cli_without_rewriting_record() {
    let root = TempDir::new().unwrap();
    let cli = package(root.path()).join("railway-cli/node_modules/.bin");
    std::fs::create_dir_all(&cli).unwrap();
    let saved = record("npm", "@railway/mcp-server");
    let before = saved.clone();
    assert_eq!(program_paths::resolve(root.path(), &saved).unwrap(), [std::fs::canonicalize(cli).unwrap()]);
    assert_eq!(saved, before);
}

#[test]
fn recorded_program_folders_override_layout_and_stay_with_their_installation() {
    let root = TempDir::new().unwrap();
    let custom = package(root.path()).join("tools/bin");
    std::fs::create_dir_all(&custom).unwrap();
    let mut saved = record("npm", "ordinary");
    saved["_installation"]["program_directories"] = json!(["tools/bin"]);
    assert_eq!(program_paths::resolve(root.path(), &saved).unwrap(), [std::fs::canonicalize(&custom).unwrap()]);
    saved["_installation"]["installation_id"] = json!(format!("package-{}", "b".repeat(32)));
    assert!(program_paths::resolve(root.path(), &saved).unwrap().is_empty());
}

#[test]
fn missing_bins_do_not_become_a_startup_prerequisite() {
    let root = TempDir::new().unwrap();
    let saved = record("npm", "ordinary");
    std::fs::create_dir_all(package(root.path())).unwrap();
    assert!(program_paths::resolve(root.path(), &saved).unwrap().is_empty());
}

#[test]
fn program_folder_records_reject_absolute_paths_and_parent_traversal() {
    assert!(program_paths::valid_directories(&json!([])));
    assert!(program_paths::valid_directories(&json!(["node_modules/.bin", "tools with spaces/bin"])));
    for paths in [json!([""]), json!(["/usr/bin"]), json!(["../sibling/bin"]),
        json!(["tools/../bin"]), json!(["."]), json!([1]), json!("bin"), json!(["bin:other"])] {
        assert!(!program_paths::valid_directories(&paths), "accepted {paths}");
    }
    let root = TempDir::new().unwrap();
    let mut saved = record("npm", "ordinary");
    saved["_installation"]["program_directories"] = json!(["../sibling/bin"]);
    assert!(program_paths::resolve(root.path(), &saved).is_err());
    saved["_installation"].as_object_mut().unwrap().remove("program_directories");
    saved["_installation"]["installation_id"] = json!("../sibling");
    assert!(program_paths::resolve(root.path(), &saved).is_err());
}

#[test]
fn symlinked_program_folder_cannot_escape_the_owned_package() {
    use std::os::unix::fs::symlink;
    let root = TempDir::new().unwrap();
    let external = root.path().join("other-installation/bin");
    std::fs::create_dir_all(&external).unwrap();
    let parent = package(root.path()).join("node_modules");
    std::fs::create_dir_all(&parent).unwrap();
    symlink(&external, parent.join(".bin")).unwrap();
    assert!(program_paths::resolve(root.path(), &record("npm", "ordinary")).is_err());
}
