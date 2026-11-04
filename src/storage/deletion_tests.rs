//! Removing a registration must leave native credentials and package state intact.
use super::{ServerRepository,server_directory_name};
use serde_json::json;
use std::{fs,os::unix::fs::{DirBuilderExt,symlink},path::Path};
fn private(path:&Path){fs::DirBuilder::new().recursive(true).mode(0o700).create(path).unwrap();}
fn saved(repo:&ServerRepository,id:&str){repo.save(id,&json!({"server_id":id,"name":"Disposable fixture","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"kind":"local_script","package":null,"executable":"/fixture/not-executed","args":[],"working_directory":null,"environment":[]}})).unwrap();}
#[test]
fn registration_removal_preserves_native_legacy_package_and_other_instance(){
    let fixture=tempfile::tempdir().unwrap();let root=fixture.path().join("state");let repo=ServerRepository::new(root.clone());
    saved(&repo,"one");saved(&repo,"two");let server=root.join("servers").join(server_directory_name("one"));
    private(&server.join("home/native"));fs::write(server.join("home/native/token.json"),b"disposable-native-login").unwrap();
    fs::write(server.join("credentials.enc"),b"opaque-old-ciphertext").unwrap();
    let package=root.join("packages/servers/package-fixture");private(&package);fs::write(package.join("native-state"),b"package-state").unwrap();
    let outside=fixture.path().join("outside");private(&outside);fs::write(outside.join("canary"),b"outside").unwrap();symlink(&outside,server.join("native-link")).unwrap();
    repo.remove_registration("one").unwrap();assert!(repo.get_saved("one").unwrap().is_none());assert!(repo.get_saved("two").unwrap().is_some());
    assert_eq!(fs::read(server.join("credentials.enc")).unwrap(),b"opaque-old-ciphertext");
    assert_eq!(fs::read(server.join("home/native/token.json")).unwrap(),b"disposable-native-login");
    assert_eq!(fs::read(package.join("native-state")).unwrap(),b"package-state");assert_eq!(fs::read(outside.join("canary")).unwrap(),b"outside");
    assert!(fs::symlink_metadata(server.join("native-link")).unwrap().file_type().is_symlink());
    repo.remove_registration("one").unwrap();assert_eq!(fs::read(server.join("credentials.enc")).unwrap(),b"opaque-old-ciphertext");
}
#[test]
fn registration_removal_does_not_follow_substituted_server_directory(){
    let fixture=tempfile::tempdir().unwrap();let root=fixture.path().join("state");let repo=ServerRepository::new(root.clone());saved(&repo,"one");
    let path=root.join("servers").join(server_directory_name("one"));let moved=fixture.path().join("moved");fs::rename(&path,&moved).unwrap();symlink(&moved,&path).unwrap();
    let before=fs::read(moved.join("config.json")).unwrap();assert!(repo.remove_registration("one").is_err());assert_eq!(fs::read(moved.join("config.json")).unwrap(),before);
}
