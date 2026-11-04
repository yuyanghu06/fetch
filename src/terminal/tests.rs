//! Real disposable shells under deny-default OS confinement, plus route authority checks.
use super::*;
use std::time::{Duration,Instant};

struct Fixture { _directory:tempfile::TempDir, root:PathBuf, home:PathBuf, work:PathBuf, outside:PathBuf, profile:String }
impl Fixture {
    fn new()->Self {
        let directory=tempfile::Builder::new().prefix("fetch-terminal-test-").tempdir_in("/private/tmp").unwrap();let root=std::fs::canonicalize(directory.path()).unwrap();
        let home=root.join("home");let work=root.join("work");std::fs::create_dir(&home).unwrap();std::fs::create_dir(&work).unwrap();
        let outside=root.with_extension("outside-canary");std::fs::write(&outside,"outside denied marker").unwrap();
        let profile=format!(r#"(version 1)(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (subpath "/bin") (subpath "/usr/bin") (subpath "/dev") (subpath {root}))
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") (subpath "/bin") (subpath "/usr/bin"))
(allow file-write* (subpath {root}) (subpath "/dev"))
(allow file-ioctl (subpath "/dev"))
(allow process-exec (subpath "/bin") (subpath "/usr/bin"))
(allow process-fork)(allow signal (target children))(allow sysctl-read)
"#,root=serde_json::to_string(root.to_str().unwrap()).unwrap());
        Self{_directory:directory,root,home,work,outside,profile}
    }
    fn launch(&self)->Launch { Launch{shell:"/usr/bin/sandbox-exec".into(),args:vec!["-p".into(),self.profile.clone(),"/bin/sh".into()],working_directory:self.work.clone(),environment:BTreeMap::from([
        ("HOME".into(),self.home.to_string_lossy().into()),("PATH".into(),format!("{}/private-bin:/bin:/usr/bin",self.root.display())),("FIXTURE_SETTING".into(),"assigned-fixture-value".into())])} }
}
impl Drop for Fixture {fn drop(&mut self){let _=std::fs::remove_file(&self.outside);}}
fn input(service:&TerminalService,id:&str,command:&str) {service.apply("server",Action::Input{session_id:id.into(),data:command.as_bytes().to_vec()}).unwrap_or_else(|error|panic!("input {command:?} failed: {error:?}, output {:?}",service.read("server",id,0).unwrap()));}
fn drain_until(service:&TerminalService,id:&str,cursor:&mut u64,predicate:impl Fn(&str,&Value)->bool)->(String,Value) {
    let started=Instant::now();let mut all=String::new();loop{let value=service.read("server",id,*cursor).unwrap();*cursor=value["next_cursor"].as_u64().unwrap();all.push_str(&String::from_utf8_lossy(&STANDARD.decode(value["data"].as_str().unwrap()).unwrap()));
        if predicate(&all,&value){return(all,value);}assert!(started.elapsed()<Duration::from_secs(8),"terminal did not produce expected output: {all:?}, {value}");thread::sleep(Duration::from_millis(10));}
}
#[test]
fn action_parser_enforces_exact_fields_encoding_and_dimensions() {
    assert!(matches!(Action::parse(json!({"action":"open"})).unwrap(),Action::Open{columns:80,rows:24}));
    for value in [json!({"action":"open","command":"evil"}),json!({"action":"open","rows":0}),json!({"action":"open","columns":1001}),json!({"action":"resize","session_id":"x","rows":20}),json!({"action":"input","session_id":"x","data":"!"}),json!({"action":"input","session_id":"x","data":STANDARD.encode(vec![0u8;INPUT_LIMIT+1])})] {assert!(Action::parse(value).is_err());}
}
#[test]
fn output_ring_reports_real_gaps_and_bounded_chunks() {
    let mut output=Output{status:"running",..Output::default()};output.append(&vec![b'a';OUTPUT_LIMIT+99]);assert_eq!(output.first,99);assert_eq!(output.end,OUTPUT_LIMIT as u64+99);assert_eq!(output.bytes.len(),OUTPUT_LIMIT);
    output.append(b"tail");assert_eq!(output.first,103);assert_eq!(output.bytes.iter().rev().take(4).copied().collect::<Vec<_>>(),b"liat");
}
#[test]
fn account_shell_is_resolved_independently_of_mcp_shell_env() {
    let launch=Launch::system("/fixture".into(),BTreeMap::from([("SHELL".into(),"/invalid/custom/override".into())])).unwrap();
    assert!(launch.shell.is_absolute());assert!(launch.shell.is_file());assert_ne!(launch.shell,PathBuf::from("/invalid/custom/override"));assert!(launch.args.is_empty());
}
#[test]
#[cfg(target_os="macos")]
fn confined_shell_preserves_environment_cwd_resize_and_real_exit_output() {
    let fixture=Fixture::new();let listener=std::net::TcpListener::bind("127.0.0.1:0").unwrap();listener.set_nonblocking(true).unwrap();let service=TerminalService::new();
    assert!(service.read("server","missing",0).is_err());assert!(service.existing("server").is_none());
    let (opened,created)=service.open("server",fixture.launch(),80,24).unwrap();assert!(created);let id=opened["session_id"].as_str().unwrap().to_owned();
    let (same,created)=service.open("server",fixture.launch(),99,50).unwrap();assert!(!created);assert_eq!(same["session_id"],id);
    assert_eq!(service.read("wrong-server",&id,0).unwrap_err().code,"TERMINAL_NOT_FOUND");assert!(service.read("server",&id,u64::MAX).is_err());
    input(&service,&id,"stty -echo\n");let mut cursor=0;thread::sleep(Duration::from_millis(80));let _=service.read("server",&id,0).unwrap();
    input(&service,&id,&format!("printf 'HOME=<%s> SET=<%s> PATH=<%s>\\n' \"$HOME\" \"$FIXTURE_SETTING\" \"$PATH\"; pwd; cat '{}'; printf 'OWNED' > \"$HOME/owned\"; printf 'END-ENV\\n'\n",fixture.outside.display()));
    let (output,_)=drain_until(&service,&id,&mut cursor,|text,_|text.contains("END-ENV"));
    assert!(output.contains(&format!("HOME=<{}>",fixture.home.display())),"{output}");assert!(output.contains("SET=<assigned-fixture-value>"));assert!(output.contains(&fixture.work.to_string_lossy().to_string()));assert!(output.contains("Operation not permitted")||output.contains("Permission denied"),"{output}");assert!(!output.contains("outside denied marker"));assert_eq!(std::fs::read_to_string(fixture.home.join("owned")).unwrap(),"OWNED");
    input(&service,&id,&format!("printf BAD > '{}'; nc -z -w 1 127.0.0.1 {}; printf 'NETWORK_STATUS=%s\\n' \"$?\"; printf 'CONFINEMENT-DONE\\n'\n",fixture.outside.display(),listener.local_addr().unwrap().port()));
    let(output,_)=drain_until(&service,&id,&mut cursor,|text,_|text.contains("CONFINEMENT-DONE"));assert!(output.contains("NETWORK_STATUS=1"),"{output}");assert_eq!(std::fs::read_to_string(&fixture.outside).unwrap(),"outside denied marker");assert!(matches!(listener.accept(),Err(error) if error.kind()==std::io::ErrorKind::WouldBlock));
    service.apply("server",Action::Resize{session_id:id.clone(),columns:101,rows:37}).unwrap();input(&service,&id,"stty size; cd \"$HOME\"; pwd; printf 'RESIZED\\n'\n");
    let (output,_)=drain_until(&service,&id,&mut cursor,|text,_|text.contains("RESIZED"));assert!(output.contains("37 101"),"{output}");assert!(output.contains(&fixture.home.to_string_lossy().to_string()));
    input(&service,&id,"printf 'final-trailing-output'; exit 17\n");let (output,status)=drain_until(&service,&id,&mut cursor,|_,value|value["status"]=="exited"&&value["data"]=="");assert!(output.contains("final-trailing-output"));assert_eq!(status["exit_code"],17);
    assert!(service.apply("server",Action::Input{session_id:id.clone(),data:b"never".to_vec()}).is_err());service.shutdown().unwrap();
}
#[test]
#[cfg(target_os="macos")]
fn close_stops_owned_shell_and_foreground_child_and_releases_threads() {
    let fixture=Fixture::new();let service=TerminalService::new();let(opened,_)=service.open("server",fixture.launch(),80,24).unwrap();let id=opened["session_id"].as_str().unwrap().to_owned();
    input(&service,&id,"sleep 60\n");thread::sleep(Duration::from_millis(100));
    let session=service.session("server",&id).unwrap();let pid=session.child.lock().unwrap().process_id().unwrap();
    let foreground=session.master.lock().unwrap().as_ref().unwrap().process_group_leader().expect("foreground sleep group");assert!(foreground>1);assert_ne!(foreground,unsafe{libc::getpgrp()});assert_ne!(foreground,pid as i32,"fixture sleep must own the foreground group");
    let started=Instant::now();let closed=service.apply("server",Action::Close{session_id:id.clone()}).unwrap();assert_eq!(closed["status"],"closed");assert!(started.elapsed()<Duration::from_secs(2));
    assert!(session.reader.lock().unwrap().is_none());assert!(session.waiter.lock().unwrap().is_none());assert!(session.master.lock().unwrap().is_none());assert!(session.writer.lock().unwrap().is_none());assert_eq!(unsafe{libc::kill(pid as i32,0)},-1);
    let deadline=Instant::now()+Duration::from_secs(2);
    while unsafe{libc::kill(foreground,0)}==0 && Instant::now()<deadline {thread::sleep(Duration::from_millis(10));}
    assert_eq!(unsafe{libc::kill(foreground,0)},-1,"owned foreground sleep must be gone after close");
    assert_eq!(service.apply("server",Action::Close{session_id:id}).unwrap()["status"],"closed");service.remove("server").unwrap();assert!(service.existing("server").is_none());service.shutdown().unwrap();
}

#[tokio::test]
async fn terminal_routes_require_live_native_authority_and_get_does_not_spawn() {
    use crate::{api::{AppState,router},authority::PendingNativeAuthority,discovery::DiscoveryService,installation::Installer,manager::Manager,native_auth::NativeAuthority,remote_oauth::GrantStore,runner::Runner,storage::ServerRepository};
    use axum::{body::Body,http::Request};use tower::ServiceExt;use http_body_util::BodyExt;
    let directory=tempfile::tempdir().unwrap();let root=directory.path().join("state");let repository=ServerRepository::new(root.clone());let discovery=DiscoveryService::new(root.join("unused"),None);let native=NativeAuthority::new();let grants=Arc::new(GrantStore::memory(root.clone()));
    let manager=Manager::with_grant_store(repository.clone(),grants,Runner::new(),Installer::new(root.clone(),discovery.clone()));
    repository.save("remote",&json!({"server_id":"remote","name":"remote","runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":false,"restart":null,"config":{"kind":"remote","package":null,"executable":null,"args":[],"working_directory":null,"environment":[],"connection":{"url":"https://example.invalid/mcp","transport":"streamable_http","headers":[]}}})).unwrap();
    let mut state=AppState::new("https://localhost:53672".into(),repository,discovery,Arc::new(PendingNativeAuthority)).unwrap();state.manager=Some(manager.clone());state.native=Some(native.clone());let app=router(Arc::new(state));
    let session=native.open_settings();
    for (method,uri,auth,body,expected) in [
        ("POST","/api/v1/servers/remote/terminal",None,Some(json!({"action":"open"})),403),
        ("GET","/api/v1/servers/remote/terminal?session_id=missing",None,None,403),
        ("GET","/api/v1/servers/remote/terminal?session_id=missing",Some(session.header_value()),None,404),
        ("POST","/api/v1/servers/remote/terminal",Some(session.header_value()),Some(json!({"action":"open","command":"bad"})),400),
        ("POST","/api/v1/servers/remote/terminal",Some(session.header_value()),Some(json!({"action":"open"})),422),
        ("GET","/api/v1/servers/remote/terminal?session_id=x&after=-1",Some(session.header_value()),None,400),
    ]{
        let mut request=Request::builder().uri(uri).method(method).header("host","localhost:53672").header("content-type","application/json");if let Some(auth)=auth{request=request.header("x-fetch-settings-capability",auth);}else{request=request.header("x-fetch-background-auth",manager.background_token());}
        let response=app.clone().oneshot(request.body(body.map(|v|Body::from(v.to_string())).unwrap_or_else(Body::empty)).unwrap()).await.unwrap();assert_eq!(response.status().as_u16(),expected,"{method} {uri}");let value:Value=serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();assert!(value["next_steps"].is_array());
    }
    let bearer=Request::builder().uri("/api/v1/servers/remote/terminal").method("POST").header("host","localhost:53672").header("content-type","application/json").header("authorization","Bearer unrelated-fixture-bearer").body(Body::from(r#"{"action":"open"}"#)).unwrap();
    assert_eq!(app.clone().oneshot(bearer).await.unwrap().status().as_u16(),403);
    native.invalidate_all();
    let stale=Request::builder().uri("/api/v1/servers/remote/terminal?session_id=missing").header("host","localhost:53672").header("x-fetch-settings-capability",session.header_value()).body(Body::empty()).unwrap();
    assert_eq!(app.oneshot(stale).await.unwrap().status().as_u16(),403);
    assert!(manager.runner.server_ids().await.is_empty());manager.shutdown().await;
}

#[test]
#[cfg(target_os="macos")]
fn close_interrupts_blocked_terminal_input_without_waiting_for_writer_lock() {
    let fixture=Fixture::new();let service=TerminalService::new();let(opened,_)=service.open("server",fixture.launch(),80,24).unwrap();let id=opened["session_id"].as_str().unwrap().to_owned();
    input(&service,&id,"stty -icanon -echo; printf 'BLOCKED-INPUT-READY\\n'; sleep 60\n");let mut cursor=0;
    drain_until(&service,&id,&mut cursor,|text,_|text.contains("BLOCKED-INPUT-READY\r\n"));
    thread::sleep(Duration::from_millis(100));let writer_service=service.clone();let writer_id=id.clone();let(tx,rx)=std::sync::mpsc::channel();
    let writer=thread::spawn(move||{let mut result=Ok(json!({}));for _ in 0..128{result=writer_service.apply("server",Action::Input{session_id:writer_id.clone(),data:vec![b'x';INPUT_LIMIT]});if result.is_err(){break;}}tx.send(result.is_err()).unwrap();});
    assert!(rx.recv_timeout(Duration::from_millis(150)).is_err(),"fixture writer must actually be blocked before close");
    let started=Instant::now();service.apply("server",Action::Close{session_id:id}).unwrap();assert!(started.elapsed()<Duration::from_secs(2));assert!(rx.recv_timeout(Duration::from_secs(2)).unwrap());writer.join().unwrap();service.shutdown().unwrap();
}
