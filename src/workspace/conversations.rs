//! Complete native conversations through pinned, separately installed readers.
//! Raw session reads remain in sessions.rs and never enter this path.
use super::{App, WorkspaceError, WorkspaceService, sessions, success};
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}, process::Stdio, time::Duration};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, process::Command, sync::oneshot};

const RESPONSE_LIMIT: usize = 32 * 1024 * 1024;
const READER_DEADLINE: Duration = Duration::from_secs(120);

fn failed(message:impl Into<String>)->WorkspaceError {
    WorkspaceError::new(502,"SESSION_READER_FAILED",message)
}
fn too_large()->WorkspaceError {
    WorkspaceError::new(413,"SESSION_CONVERSATION_TOO_LARGE","The complete conversation exceeds Fetch's 32 MiB response limit.")
}
fn bounded(value:Value)->Result<Value,WorkspaceError> {
    if serde_json::to_vec(&value).map_err(|error|failed(error.to_string()))?.len()>RESPONSE_LIMIT {Err(too_large())} else {Ok(value)}
}

pub(super) async fn read(service:&WorkspaceService, folder_path:Option<&str>, file_path:&str)->Result<Value,WorkspaceError>{
    // Reuse the original path and selected-project checks before starting a helper.
    let session=sessions::selected(service,folder_path,file_path)?;
    let result=invoke(service,session.app,json!({"op":"conversation","file_path":session.file_path})).await?;
    if result["complete"]!=true || !result["messages"].is_array() {
        return Err(failed("The session reader did not return a complete conversation."));
    }
    bounded(success(json!({"session":session.value(),"messages":result["messages"],"complete":true})))
}

pub(super) async fn list(service:&WorkspaceService,folder_path:Option<&str>,app:Option<&str>,cursor:Option<&str>,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let folder=service.folder(folder_path)?;
    let selected=app.map(App::parse).transpose()?;
    let offset=cursor.map(|value|value.parse::<usize>().map_err(|_|WorkspaceError::new(400,"INVALID_SESSION_CURSOR","Use the next_cursor returned by the session list."))).transpose()?.unwrap_or(0);
    let limit=limit.unwrap_or(50);
    if !(1..=200).contains(&limit){return Err(WorkspaceError::new(400,"INVALID_SESSION_LIMIT","Read between 1 and 200 session names at a time."));}
    let candidates=sessions::all(service,folder.as_deref(),selected)?;
    let mut output=Vec::new();
    // One helper invocation per app, rather than one process per listed file.
    for app in [App::Claude,App::Codex] {
        let batch:Vec<Value>=candidates.iter().filter(|session|session.app==app).map(|session|session.value()).collect();
        if batch.is_empty(){continue;}
        let result=invoke(service,app,json!({"op":"list","candidates":batch})).await?;
        let values=result["sessions"].as_array().ok_or_else(||failed("The session reader did not return a conversation list."))?;
        for value in values {
            // The reader selects and names candidates; it cannot introduce a path
            // which bypassed the native list's history/project authorization.
            if !batch.iter().any(|candidate|candidate["id"]==value["id"]&&candidate["file_path"]==value["file_path"]) {
                return Err(failed("The session reader returned a conversation outside its supplied candidates."));
            }
            output.push(value.clone());
        }
    }
    output.sort_by(|left,right|right["updated_at"].as_u64().cmp(&left["updated_at"].as_u64()).then_with(||left["id"].as_str().cmp(&right["id"].as_str())));
    let page:Vec<_>=output.iter().skip(offset).take(limit).cloned().collect();
    let end=offset.saturating_add(page.len());
    bounded(success(json!({"folder_path":folder,"sessions":page,"next_cursor":if end<output.len(){Some(end.to_string())}else{None}})))
}

/// Dropping an HTTP request closes cancellation. A supervisor still owns the
/// child and work directory until it has killed and reaped the reader.
async fn invoke(service:&WorkspaceService,app:App,request:Value)->Result<Value,WorkspaceError>{
    let root=service.readers_root.clone();
    let roots=sessions::roots(service,app).into_iter().filter(|path|path.exists())
        .map(|path|path.canonicalize().map_err(WorkspaceError::io)).collect::<Result<Vec<_>,_>>()?;
    let (cancel, cancelled)=oneshot::channel::<()>();
    let mut request=request;
    let mut index_files=Vec::new();
    if app==App::Codex {
        let index=service.codex_root.join("state_5.sqlite");
        if index.is_file(){
            let allowed=service.codex_root.canonicalize().map_err(WorkspaceError::io)?;
            let index=index.canonicalize().map_err(WorkspaceError::io)?;
            if !index.starts_with(&allowed){return Err(WorkspaceError::new(403,"SESSION_PATH_FORBIDDEN","Codex's native session index points outside its configuration folder."));}
            request["source_index"]=json!(index);
            index_files.push(index.clone());
            for suffix in ["-wal","-shm"] {
                let sidecar=PathBuf::from(format!("{}{suffix}",index.display()));
                if sidecar.is_file(){
                    let sidecar=sidecar.canonicalize().map_err(WorkspaceError::io)?;
                    if !sidecar.starts_with(&allowed){return Err(WorkspaceError::new(403,"SESSION_PATH_FORBIDDEN","Codex's session index sidecar points outside its configuration folder."));}
                    index_files.push(sidecar);
                }
            }
        }
    }
    let task=tokio::spawn(supervise(root,app,roots,index_files,request,cancelled));
    let result=task.await.map_err(|error|failed(error.to_string()))?;
    drop(cancel);
    result
}

struct WorkDirectory(PathBuf);
impl WorkDirectory {
    fn new()->Result<Self,WorkspaceError>{
        use std::os::unix::fs::DirBuilderExt;
        let path=std::env::temp_dir().join(format!("fetch-session-reader-{}-{:016x}",std::process::id(),rand::random::<u64>()));
        fs::DirBuilder::new().mode(0o700).create(&path).map_err(WorkspaceError::io)?;
        Ok(Self(path.canonicalize().map_err(WorkspaceError::io)?))
    }
}
impl Drop for WorkDirectory {fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}

async fn supervise(root:PathBuf,app:App,roots:Vec<PathBuf>,index_files:Vec<PathBuf>,mut request:Value,mut cancelled:oneshot::Receiver<()>)->Result<Value,WorkspaceError>{
    let (executable,arguments)=match app {
        App::Codex=>(root.join("codex/fetch-codex-session-reader"),Vec::new()),
        App::Claude=>(root.join("claude/bin/python"),vec!["-I".to_owned(),"-B".to_owned(),root.join("claude/helper.py").to_string_lossy().into_owned()]),
    };
    if !executable.is_file() || (app==App::Claude&&!root.join("claude/helper.py").is_file()) {
        return Err(WorkspaceError::new(503,"SESSION_READER_UNAVAILABLE",format!("The {} session reader is not installed in {}. Run its separate setup script first.",app.as_str(),root.display())));
    }
    let work=WorkDirectory::new()?;
    request["source_roots"]=json!(roots);
    request["work_dir"]=json!(work.0);
    let input=serde_json::to_vec(&request).map_err(|error|failed(error.to_string()))?;
    let mut command=confined_command(&root,&executable,&arguments,&roots,&index_files,&work.0)?;
    command.current_dir(&work.0).env_clear()
        .env("HOME",&work.0).env("TMPDIR",&work.0).env("PATH","/usr/bin:/bin")
        .env("PYTHONDONTWRITEBYTECODE","1").env("LANG","en_US.UTF-8")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .kill_on_drop(true).process_group(0);
    let mut child=command.spawn().map_err(|error|failed(error.to_string()))?;
    let pid=child.id();
    let mut stdin=child.stdin.take().expect("reader stdin is piped");
    let stdout=child.stdout.take().expect("reader stdout is piped");
    let stderr=child.stderr.take().expect("reader stderr is piped");
    let exchange=async {
        let write=async move {stdin.write_all(&input).await?;drop(stdin);Ok::<_,std::io::Error>(())};
        let read=async {let mut bytes=Vec::new();stdout.take((RESPONSE_LIMIT+1) as u64).read_to_end(&mut bytes).await?;Ok::<_,std::io::Error>(bytes)};
        let errors=async {let mut bytes=Vec::new();stderr.take(128*1024).read_to_end(&mut bytes).await?;Ok::<_,std::io::Error>(bytes)};
        let (_,bytes,errors)=tokio::try_join!(write,read,errors).map_err(|error|failed(error.to_string()))?;
        if bytes.len()>RESPONSE_LIMIT{return Err(too_large());}
        let status=child.wait().await.map_err(|error|failed(error.to_string()))?;
        let diagnostics=crate::runner::startup_diagnostics::sanitize_text(&String::from_utf8_lossy(&errors),&[]);
        // Official helpers can return a useful structured failure and exit 1.
        // Keep that failure before falling back to process status/diagnostics.
        let parsed=serde_json::from_slice::<Value>(&bytes);
        if !status.success() && parsed.as_ref().map_or(true,|value|value.get("error").is_none()) {
            return Err(failed(format!("Session reader exited with {status}. {diagnostics}")));
        }
        let value=parsed.map_err(|error|failed(format!("{error}. {diagnostics}")))?;
        if let Some(upstream)=value.get("error") {
            let code=upstream.get("code").and_then(Value::as_str).unwrap_or("");
            let unsupported=code.contains("Unsupported")||matches!(code,"SESSION_READER_UNSUPPORTED"|"CODEX_HISTORY_ANCESTOR_MISSING"|"CODEX_HISTORY_VERSION_UNSUPPORTED"|"CODEX_NOT_MAIN_CONVERSATION");
            let mut error=WorkspaceError::new(if unsupported{503}else{502},if unsupported{"SESSION_READER_UNSUPPORTED"}else{"SESSION_READER_FAILED"},crate::runner::startup_diagnostics::sanitize_text(upstream.get("message").and_then(Value::as_str).unwrap_or("The official session reader failed."),&[]));
            error.details=Some(redact(upstream));
            return Err(error);
        }
        Ok(value)
    };
    let result=tokio::select!{
        result=tokio::time::timeout(READER_DEADLINE,exchange)=>match result{
            Ok(result)=>result,
            Err(_)=>Err(WorkspaceError::new(504,"SESSION_READER_TIMEOUT","The official session reader did not finish within 120 seconds.")),
        },
        _=&mut cancelled=>Err(failed("The conversation request was cancelled.")),
    };
    // Reap on every path before private snapshots are removed. The group is
    // owned by this request, so a reader descendant cannot outlive cancellation.
    if let Some(pid)=pid {unsafe {libc::kill(-(pid as i32),libc::SIGKILL);}}
    let _=child.start_kill();
    let _=child.wait().await;
    drop(work);
    result
}

fn redact(value:&Value)->Value {
    match value {
        Value::String(value)=>json!(crate::runner::startup_diagnostics::sanitize_text(value,&[])),
        Value::Array(values)=>Value::Array(values.iter().map(redact).collect()),
        Value::Object(values)=>Value::Object(values.iter().map(|(key,value)|(key.clone(),redact(value))).collect()),
        _=>value.clone(),
    }
}

#[cfg(target_os="macos")]
fn confined_command(root:&Path,executable:&Path,arguments:&[String],roots:&[PathBuf],index_files:&[PathBuf],work:&Path)->Result<Command,WorkspaceError>{
    let executable_real=executable.canonicalize().map_err(WorkspaceError::io)?;
    let runtime=if executable.ends_with("bin/python") {
        executable_real.parent().and_then(Path::parent).ok_or_else(||failed("The private Python runtime path is invalid."))?.to_path_buf()
    }else{executable_real.parent().ok_or_else(||failed("The reader executable path is invalid."))?.to_path_buf()};
    let quoted=|path:&Path|serde_json::to_string(&path.to_string_lossy()).expect("path is JSON text");
    let mut reads=vec![root.join(app_directory(executable)).canonicalize().map_err(WorkspaceError::io)?,runtime,work.to_path_buf()];
    reads.extend(roots.iter().cloned());
    let readable=reads.iter().map(|path|format!("(subpath {})",quoted(path))).collect::<Vec<_>>().join(" ");
    let index_reads=index_files.iter().map(|path|format!("(literal {})",quoted(path))).collect::<Vec<_>>().join(" ");
    let profile=format!(r#"(version 1)(deny default)
(allow file-read-metadata)
(allow file-read* (literal "/") (subpath "/System") (subpath "/usr/lib") (literal "/dev/null") (literal "/dev/random") (literal "/dev/urandom") {readable} {index_reads})
(allow file-map-executable (subpath "/System") (subpath "/usr/lib") {readable})
(allow file-write* (subpath {work}) (literal "/dev/null"))
(allow process-exec (literal {executable}) (literal {executable_real}))
(allow process-fork)(allow signal (target children))(allow sysctl-read)
"#,work=quoted(work),executable=quoted(executable),executable_real=quoted(&executable_real));
    let mut command=Command::new("/usr/bin/sandbox-exec");
    command.args(["-p",&profile]).arg(executable).args(arguments);
    Ok(command)
}
#[cfg(target_os="macos")]
fn app_directory(executable:&Path)->&'static str{if executable.ends_with("bin/python"){"claude"}else{"codex"}}

#[cfg(not(target_os="macos"))]
fn confined_command(_root:&Path,_executable:&Path,_arguments:&[String],_roots:&[PathBuf],_index_files:&[PathBuf],_work:&Path)->Result<Command,WorkspaceError>{
    Err(WorkspaceError::new(503,"SESSION_READER_UNAVAILABLE","Complete native conversation readers require macOS process confinement."))
}

#[cfg(all(test,target_os="macos"))]
mod tests {
    use super::*;
    use sha2::{Digest,Sha256};

    fn installed_readers()->PathBuf {
        crate::application::default_data_root().unwrap().join("session-readers")
    }
    #[tokio::test]
    async fn codex_confined_complete_modern_conversation_reads_every_native_page(){
        let fixture=tempfile::tempdir().unwrap();let home=fixture.path().join("home");
        let project=fixture.path().join("project");fs::create_dir(&project).unwrap();
        let directory=home.join(".codex/sessions/2026/01/01");fs::create_dir_all(&directory).unwrap();
        let thread="00000000-0000-4000-8000-000000000001";
        let path=directory.join(format!("rollout-2026-01-01T00-00-00-{thread}.jsonl"));
        let stamp="2026-01-01T00:00:00.000Z";
        let mut records=vec![json!({"timestamp":stamp,"ordinal":0,"type":"session_meta","payload":{"id":thread,"timestamp":stamp,"cwd":project,"originator":"fixture","cli_version":"0.167.2","source":"cli","history_mode":"paginated"}})];
        for index in 0..140 {
            let turn=format!("turn-{index}");
            let events=vec![
                json!({"type":"task_started","turn_id":turn}),
                json!({"type":"item_completed","thread_id":thread,"turn_id":turn,"item":{"type":"UserMessage","id":format!("{turn}-user"),"content":[{"type":"text","text":format!("typed {index}")}]},"completed_at_ms":1}),
                json!({"type":"item_completed","thread_id":thread,"turn_id":turn,"item":{"type":"AgentMessage","id":format!("{turn}-agent"),"content":[{"type":"Text","text":format!("answer {index}")}]},"completed_at_ms":1}),
                json!({"type":"task_complete","turn_id":turn}),
            ];
            for event in events {records.push(json!({"timestamp":stamp,"ordinal":records.len(),"type":"event_msg","payload":event}));}
        }
        let original=records.iter().map(Value::to_string).collect::<Vec<_>>().join("\n")+"\n";fs::write(&path,&original).unwrap();
        let service=WorkspaceService::new(home.clone(),home.join(".codex"),home.join(".claude")).with_readers_root(installed_readers());
        let value=service.read_conversation(Some(project.to_str().unwrap()),path.to_str().unwrap()).await.unwrap();
        let messages=value["messages"].as_array().unwrap();
        assert_eq!(messages.len(),280);
        assert_eq!(messages[0]["text"],"typed 0");
        assert_eq!(messages.last().unwrap()["text"],"answer 139");
        assert_eq!(value["complete"],true);
        assert_eq!(Sha256::digest(fs::read(&path).unwrap()),Sha256::digest(original.as_bytes()));
        assert!(!home.join(".codex/state_5.sqlite").exists());
    }
    #[tokio::test]
    async fn claude_complete_conversation_preserves_parallel_results_and_original_file(){
        let fixture=tempfile::tempdir().unwrap();
        let home=fixture.path().join("home");
        let project=fixture.path().join("project");fs::create_dir(&project).unwrap();
        let transcripts=home.join(".claude/projects/project");fs::create_dir_all(&transcripts).unwrap();
        let path=transcripts.join("11111111-1111-4111-8111-111111111111.jsonl");
        let full_reply=format!("Beginning {} final reply 🦀", "complete reply ".repeat(5000));
        let entry=|uid:&str,parent:Option<&str>,kind:&str,content:Value,message_id:&str|json!({"uuid":uid,"parentUuid":parent,"type":kind,"sessionId":"11111111-1111-4111-8111-111111111111","cwd":project,"message":{"id":message_id,"role":kind,"content":content}});
        let mut meta=entry("meta",None,"user",json!("<recommended_plugins>injected setup</recommended_plugins>"),"meta");meta["isMeta"]=json!(true);
        let records=vec![meta,
            entry("manual",Some("meta"),"user",json!("Read both files, then explain them."),"u1"),
            entry("call1",Some("manual"),"assistant",json!([{"type":"tool_use","id":"one","name":"Read","input":{"path":"one"}}]),"parallel"),
            entry("call2",Some("call1"),"assistant",json!([{"type":"tool_use","id":"two","name":"Read","input":{"path":"two"}}]),"parallel"),
            entry("result1",Some("call1"),"user",json!([{"type":"tool_result","tool_use_id":"one","content":"one unchanged output"}]),"r1"),
            entry("result2",Some("call2"),"user",json!([{"type":"tool_result","tool_use_id":"two","content":[{"type":"text","text":"two unchanged output"}]}]),"r2"),
            entry("discarded",Some("manual"),"user",json!("unused fork"),"fork"),
            entry("final",Some("result1"),"assistant",json!([{"type":"text","text":full_reply}]),"answer")];
        let original=records.iter().map(Value::to_string).collect::<Vec<_>>().join("\n")+"\n";
        fs::write(&path,&original).unwrap();
        let service=WorkspaceService::new(home.clone(),home.join(".codex"),home.join(".claude")).with_readers_root(installed_readers());
        let result=service.read_conversation(Some(project.to_str().unwrap()),path.to_str().unwrap()).await.unwrap();
        let messages=result["messages"].as_array().unwrap();
        assert_eq!(messages.len(),4,"{result}");
        assert_eq!(messages[0]["text"],"Read both files, then explain them.");
        assert!(messages.iter().any(|message|message["tool_call_id"]=="one"&&message["tool_name"]=="Read"&&message["output"]=="one unchanged output"));
        assert!(messages.iter().any(|message|message["tool_call_id"]=="two"&&message["output"]==json!([{"type":"text","text":"two unchanged output"}])));
        assert_eq!(messages.last().unwrap()["text"],full_reply);
        assert_eq!(result["complete"],true);
        assert_eq!(Sha256::digest(fs::read(&path).unwrap()),Sha256::digest(original.as_bytes()));
        let list=service.list_conversations(None,Some("claude"),None,None).await.unwrap();
        assert_eq!(list["sessions"].as_array().unwrap().len(),1);
        assert_eq!(list["sessions"][0]["name"],"Read both files, then explain them.");
        let another=fixture.path().join("another-project");fs::create_dir(&another).unwrap();
        assert_eq!(service.read_conversation(Some(another.to_str().unwrap()),path.to_str().unwrap()).await.unwrap_err().code,"SESSION_PROJECT_FORBIDDEN");
        let raw=service.read_session(None,path.to_str().unwrap(),0,Some(64)).unwrap();
        assert_eq!(raw["content"].as_str().unwrap().as_bytes(),&original.as_bytes()[..64]);
    }

    #[tokio::test]
    async fn reader_confinement_denies_original_writes_network_and_unrelated_reads(){
        let fixture=tempfile::tempdir().unwrap();let root=fixture.path().canonicalize().unwrap();
        let source=root.join("history");let work=root.join("work");fs::create_dir(&source).unwrap();fs::create_dir(&work).unwrap();
        let original=source.join("original");fs::write(&original,"unchanged").unwrap();
        let canary=root.join("unrelated");fs::write(&canary,"private").unwrap();
        let readers=installed_readers();let executable=readers.join("claude/bin/python");
        let code=r#"import json,pathlib,socket,sys
source,canary,work=map(pathlib.Path,sys.argv[1:])
checks={"history_read":source.read_text()=="unchanged"}
for name,action in [("history_write",lambda:source.write_text("bad")),("unrelated_read",lambda:canary.read_text()),("network",lambda:socket.socket().connect(("127.0.0.1",9)))]:
 try: action(); checks[name]=False
 except PermissionError: checks[name]=True
(work/"allowed").write_text("ok")
print(json.dumps(checks))"#;
        let args=vec!["-I".into(),"-c".into(),code.into(),original.to_string_lossy().into_owned(),canary.to_string_lossy().into_owned(),work.to_string_lossy().into_owned()];
        let output=confined_command(&readers,&executable,&args,&[source],&[],&work).unwrap().env_clear().current_dir(&work).output().await.unwrap();
        assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
        let checks:Value=serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(checks,json!({"history_read":true,"history_write":true,"unrelated_read":true,"network":true}));
        assert_eq!(fs::read_to_string(&original).unwrap(),"unchanged");
        assert_eq!(fs::read_to_string(work.join("allowed")).unwrap(),"ok");
    }
}
