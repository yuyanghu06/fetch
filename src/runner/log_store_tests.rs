//! Independent persistence checks; no service or credential store is started.
use super::*;
use std::os::unix::fs::{PermissionsExt, symlink};

fn config() -> TransportConfig {
    TransportConfig::Stdio { executable:"/fixture/never-executed".into(), args:vec![], working_directory:None,
        environment:BTreeMap::from([("FIXTURE_KEY".into(),"assigned-private-fixture-value".into())]) }
}
fn setup() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");
    crate::storage::server_directory(&root,"logs",true).unwrap();
    let path=root.join("servers").join(crate::storage::server_directory_name("logs")).join("console-log.json");
    (directory,root,path)
}
fn lines(value:&Value)->String {value["lines"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>().join("\n")}

#[tokio::test]
async fn log_history_flushes_on_shutdown_and_reloads_before_configure() {
    let (_directory,root,path)=setup();let runner=Runner::persistent(root.clone()).unwrap();
    let first=runner.begin_startup_diagnostics("logs",&config()).await;
    first.line("startup original rejection assigned-private-fixture-value");
    first.value("mcp error",&json!({"code":-32001,"message":"service denial","access_token":"returned-private-value"}));
    use std::os::unix::process::ExitStatusExt;
    first.natural_exit(std::process::ExitStatus::from_raw(7<<8));
    runner.shutdown().await;
    assert!(path.exists());assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode()&0o077,0);
    let saved=std::fs::read_to_string(&path).unwrap();
    assert!(!saved.contains("assigned-private-fixture-value"));assert!(!saved.contains("returned-private-value"));
    assert!(!saved.contains("generation"));
    let reloaded=Runner::persistent(root).unwrap();assert!(reloaded.server_ids().await.is_empty());
    let snapshot=reloaded.startup_diagnostics("logs").await.unwrap();
    assert!(lines(&snapshot).contains("startup original rejection [redacted]"));
    assert!(lines(&snapshot).contains("-32001"));assert_eq!(snapshot["exit_code"],7);
    let second=reloaded.begin_startup_diagnostics("logs",&config()).await;
    second.line("auth rejection");first.line("obsolete callback must not append");
    reloaded.flush_startup_logs().await;
    let third=reloaded.begin_startup_diagnostics("logs",&config()).await;third.line("later retry");
    reloaded.shutdown().await;
    let saved:Value=serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();let output=lines(&saved);
    assert_eq!(output.matches("Process exited with status 7.").count(),1);
    assert!(output.contains("auth rejection"));assert!(output.contains("later retry"));assert!(!output.contains("obsolete callback"));
}

#[tokio::test]
async fn newest_history_respects_line_byte_and_total_limits_after_restart() {
    let (_directory,root,path)=setup();let runner=Runner::persistent(root.clone()).unwrap();let capture=runner.begin_startup_diagnostics("logs",&config()).await;
    for index in 0..4200 {capture.line(&format!("fixture line {index:04}"));}
    let snapshot=capture.snapshot();assert!(snapshot["lines"].as_array().unwrap().len()<=4096);
    assert!(lines(&snapshot).starts_with("[fetch] Older console output was removed"));
    assert!(!lines(&snapshot).contains("fixture line 0000"));assert!(lines(&snapshot).contains("fixture line 4199"));
    for index in 0..24 {capture.text("source",&format!("large {index:02} {}","é".repeat(40000)));}
    capture.line("newest retained marker");runner.shutdown().await;
    let restored=Runner::persistent(root).unwrap().startup_diagnostics("logs").await.unwrap();
    let saved:Value=serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let saved_lines=saved["lines"].as_array().unwrap();
    assert!(saved_lines.iter().map(|line|line.as_str().unwrap().len()).sum::<usize>()<=1024*1024);
    assert!(saved_lines.iter().all(|line|line.as_str().unwrap().len()<=64*1024));
    assert!(lines(&restored).contains("newest retained marker"));assert!(!lines(&restored).contains("large 00"));
}

#[tokio::test]
async fn passive_reads_do_not_create_state_or_rewrite_saved_files() {
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("absent");let runner=Runner::persistent(root.clone()).unwrap();
    assert!(runner.startup_diagnostics("missing").await.is_none());assert!(!root.exists());runner.shutdown().await;assert!(!root.exists());
    let (_directory,root,path)=setup();let runner=Runner::persistent(root.clone()).unwrap();let capture=runner.begin_startup_diagnostics("logs",&config()).await;capture.line("saved");runner.shutdown().await;
    let before=std::fs::read(&path).unwrap();let modified=std::fs::metadata(&path).unwrap().modified().unwrap();
    let reader=Runner::persistent(root).unwrap();reader.startup_diagnostics("logs").await.unwrap();reader.shutdown().await;
    assert_eq!(std::fs::read(&path).unwrap(),before);assert_eq!(std::fs::metadata(path).unwrap().modified().unwrap(),modified);
}

#[tokio::test]
async fn unsafe_log_files_are_preserved_and_reported_honestly() {
    let (directory,root,path)=setup();let outside=directory.path().join("untouched");std::fs::write(&outside,"outside canary").unwrap();symlink(&outside,&path).unwrap();
    let runner=Runner::persistent(root).unwrap();let snapshot=runner.startup_diagnostics("logs").await.unwrap();assert!(lines(&snapshot).contains("could not be read"));
    let capture=runner.begin_startup_diagnostics("logs",&config()).await;capture.line("new service output");runner.flush_startup_logs().await;
    assert!(lines(&capture.snapshot()).contains("could not be saved"));assert_eq!(std::fs::read_to_string(outside).unwrap(),"outside canary");
    assert!(std::fs::symlink_metadata(path).unwrap().file_type().is_symlink());runner.shutdown().await;
}
