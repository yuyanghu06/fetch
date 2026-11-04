//! Deletion preservation checks using disposable files and real symlinks.
use super::{ServerRepository, server_directory_name};
use serde_json::json;
use std::{fs, os::unix::fs::{DirBuilderExt, PermissionsExt, symlink}, path::Path};

fn private_directory(path: &Path) {
    fs::DirBuilder::new().recursive(true).mode(0o700).create(path).unwrap();
}

fn save_fixture(repo: &ServerRepository, id: &str) {
    repo.save(id, &json!({
        "server_id":id,"name":"Disposable deletion fixture",
        "runtime":{"status":"stopped","error":null},
        "authentication":{"status":"unknown","error":null},
        "restart_required":false,"restart":null,
        "config":{"kind":"local_script","package":null,"executable":"/fixture/not-executed",
            "args":[],"working_directory":null,"environment":[]},"credentials":[]
    })).unwrap();
}

#[test]
fn package_deletion_preserves_symlink_targets_siblings_and_shared_runtime() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("state");
    let repo = ServerRepository::new(root.clone());
    save_fixture(&repo, "preserved-server");
    let package_id = "package-0123456789abcdef0123456789abcdef";
    let package = root.join("packages/servers").join(package_id);
    let nested = package.join("node_modules/example/lib");
    private_directory(&nested);
    fs::set_permissions(&nested, fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(nested.join("server.js"), b"disposable package content").unwrap();
    fs::write(package.join("package.json"), b"{}").unwrap();
    let outside = fixture.path().join("outside");
    private_directory(&outside);
    fs::write(outside.join("canary"), b"preserve external target").unwrap();
    symlink(&outside, nested.join("outside-directory")).unwrap();
    symlink(outside.join("canary"), package.join("outside-file")).unwrap();
    let sibling = root.join("packages/servers/package-fedcba9876543210fedcba9876543210");
    private_directory(&sibling);
    fs::write(sibling.join("keep"), b"preserve other package").unwrap();
    let runtime = root.join("packages/runtimes/shared");
    private_directory(&runtime);
    fs::write(runtime.join("keep"), b"preserve shared runtime").unwrap();

    repo.remove_installation_package(package_id).unwrap();
    assert!(!package.exists());
    assert_eq!(fs::read(outside.join("canary")).unwrap(), b"preserve external target");
    assert_eq!(fs::read(sibling.join("keep")).unwrap(), b"preserve other package");
    assert_eq!(fs::read(runtime.join("keep")).unwrap(), b"preserve shared runtime");
    assert!(repo.get_saved("preserved-server").unwrap().is_some());
    repo.remove_installation_package(package_id).unwrap();
    assert_eq!(fs::read(outside.join("canary")).unwrap(), b"preserve external target");
}

#[test]
fn private_state_cleanup_preserves_config_and_external_targets_on_repeat() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path().join("state");
    let repo = ServerRepository::new(root.clone());
    let id = "private-state-fixture";
    save_fixture(&repo, id);
    let directory = root.join("servers").join(server_directory_name(id));
    let config_before = fs::read(directory.join("config.json")).unwrap();
    let home = directory.join("home/.cache/nested");
    private_directory(&home);
    fs::set_permissions(&home, fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(home.join("temporary"), b"remove private state").unwrap();
    fs::write(directory.join("credentials.enc"), b"disposable fixture bytes").unwrap();
    let outside = fixture.path().join("outside");
    private_directory(&outside);
    fs::write(outside.join("canary"), b"preserve external target").unwrap();
    symlink(&outside, directory.join("outside-link")).unwrap();

    repo.clear_server_private_state(id).unwrap();
    assert_eq!(fs::read(directory.join("config.json")).unwrap(), config_before);
    assert_eq!(fs::read(outside.join("canary")).unwrap(), b"preserve external target");
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
    assert!(repo.get_saved(id).unwrap().is_some());
    repo.clear_server_private_state(id).unwrap();
    assert_eq!(fs::read(directory.join("config.json")).unwrap(), config_before);
    assert_eq!(fs::read(outside.join("canary")).unwrap(), b"preserve external target");
}
