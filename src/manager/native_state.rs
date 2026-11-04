//! Shared unmodified process startup and explicitly recorded native setup.
use super::{Manager,ManagementError,bindings,invalid,invalid_config,settings_step,string_array};
use crate::{installation::private_directory,runner::TransportConfig};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{path::{Path,PathBuf},sync::Arc,time::Duration};


#[derive(Clone)]
pub(super) struct Setup {executable:PathBuf,args:Vec<String>,working_directory:PathBuf,state_directory:PathBuf}
pub(super) fn setup(record:&Value)->Result<Setup,ManagementError>{
    let value=&record["_installation"]["native_setup"];
    let object=value.as_object().filter(|value|value.len()==4&&["executable","args","working_directory","state_directory"].iter().all(|key|value.contains_key(*key))).ok_or_else(unsupported)?;
    let path=|name:&str|object[name].as_str().filter(|value|Path::new(value).is_absolute()&&!value.contains('\0')).map(PathBuf::from).ok_or_else(unsupported);
    Ok(Setup{executable:path("executable")?,args:string_array(&object["args"] )?,working_directory:path("working_directory")?,state_directory:path("state_directory")?})
}
fn unsupported()->ManagementError{let mut error=ManagementError::new(422,"SERVER_SETUP_UNSUPPORTED","This instance has no recorded native setup command and state folder. Fetch does not guess login commands or read provider token files.");error.next_steps=vec![settings_step("Use the server's own supported setup. A native setup command must be recorded explicitly before Fetch can run it.")];error}
fn home(root:&Path,id:&str)->PathBuf{root.join("servers").join(format!("{:x}",Sha256::digest(id.as_bytes()))).join("home")}
pub(super) fn transport(root:&Path,record:&Value,manual:&Value)->Result<TransportConfig,ManagementError>{
    let id=record["server_id"].as_str().ok_or_else(invalid)?;let config=&record["config"];
    let executable=PathBuf::from(config["executable"].as_str().ok_or_else(invalid)?);if !executable.is_absolute(){return Err(invalid_config());}
    let mut environment=bindings::InputBindings::from_record(record)?.resolve_source(bindings::Source::Environment,manual,&["HOME","XDG_CONFIG_HOME","XDG_STATE_HOME"])?;
    let home=home(root,id);private_directory(&home).map_err(|_|invalid_config())?;
    for directory in [home.join(".config"),home.join(".local/state")]{private_directory(&directory).map_err(|_|invalid_config())?;}
    environment.insert("HOME".into(),home.to_str().ok_or_else(invalid_config)?.to_owned());environment.insert("XDG_CONFIG_HOME".into(),home.join(".config").to_str().ok_or_else(invalid_config)?.to_owned());environment.insert("XDG_STATE_HOME".into(),home.join(".local/state").to_str().ok_or_else(invalid_config)?.to_owned());
    let assigned_path=environment.contains_key("PATH");
    let mut environment=super::process_environment::environment(environment).map_err(|_|invalid_config())?;
    let mut program_paths=crate::installation::program_paths::resolve(root,record).map_err(|_|invalid_config())?;
    if !assigned_path {
        if let Some(directory)=executable.parent(){program_paths.push(directory.to_owned());}
    }
    if let Some(path)=environment.get("PATH") {
        program_paths.extend(std::env::split_paths(path));
    }else{
        program_paths.extend([PathBuf::from("/usr/bin"),PathBuf::from("/bin")]);
    }
    let path=std::env::join_paths(program_paths).map_err(|_|invalid_config())?.into_string().map_err(|_|invalid_config())?;
    environment.insert("PATH".into(),path);
    environment.entry("LANG".into()).or_insert_with(||"en_US.UTF-8".into());
    if environment.iter().any(|(name,value)|name.contains(['=','\0'])||value.contains('\0')){return Err(invalid_config());}
    let working_directory=config["working_directory"].as_str().map(PathBuf::from).unwrap_or(home);
    if !working_directory.is_absolute(){return Err(invalid_config());}
    Ok(TransportConfig::Stdio{executable,args:string_array(&config["args"] )?,working_directory:Some(working_directory),environment})
}
pub(super) async fn run_setup(root:&Path,record:&Value,setup:&Setup,manual:&Value,stop:Arc<tokio::sync::Notify>,urls:tokio::sync::mpsc::Sender<String>,diagnostics:Option<Arc<crate::runner::startup_diagnostics::StartupDiagnostics>>)->Result<(),ManagementError>{
    let quit=stop.notified();tokio::pin!(quit);quit.as_mut().enable();
    let root=root.to_owned();let record=record.clone();let manual=manual.clone();
    let transport=tokio::task::spawn_blocking(move||transport(&root,&record,&manual)).await.map_err(|_|unsupported())??;
    let TransportConfig::Stdio{environment,..}=transport else{return Err(unsupported())};
    tokio::select!{biased;_= &mut quit=>return Err(setup_failed()),_=std::future::ready(())=>{}}
    let mut command=tokio::process::Command::new(&setup.executable);
    command.args(&setup.args).env_clear().envs(environment).current_dir(&setup.working_directory).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).kill_on_drop(true);
    #[cfg(unix)]command.process_group(0);
    if let Some(capture)=&diagnostics{
        capture.add_redactions(setup.args.iter().flat_map(|argument|std::iter::once(argument.clone()).chain(argument.split_once('=').map(|(_,value)|value.to_owned()))));
    }
    let mut child=command.spawn().map_err(|error|{if let Some(capture)=&diagnostics{capture.text("setup",&format!("Setup process could not start: {error}"));}setup_failed()})?;
    let owned_group=child.id();
    let stdout=child.stdout.take();let stderr=child.stderr.take();
    let output_urls=urls.clone();let output_capture=diagnostics.clone();let error_capture=diagnostics.clone();
    let mut output=tokio::spawn(async move{if let Some(stream)=stdout{crate::runner::local_setup::forward_service_output(stream,Some(output_urls),output_capture,"stdout").await;}});
    let mut error=tokio::spawn(async move{if let Some(stream)=stderr{crate::runner::local_setup::forward_service_output(stream,Some(urls),error_capture,"stderr").await;}});
    let result=tokio::select!{value=child.wait()=>value.map_err(|error|{if let Some(capture)=&diagnostics{capture.text("setup",&format!("Waiting for the setup process failed: {error}"));}setup_failed()}).and_then(|status|{if let Some(capture)=&diagnostics{capture.natural_exit(status);}if status.success(){Ok(())}else{Err(setup_failed())}}),_=tokio::time::sleep(Duration::from_secs(600))=>Err(ManagementError::new(409,"AUTHENTICATION_TIMEOUT","The recorded native setup command exceeded ten minutes.")),_= &mut quit=>Err(setup_failed())};
    // The helper owns this process group. Reap its remaining descendants on
    // completion as well as failure so callback/helper processes do not linger.
    #[cfg(unix)] if let Some(group)=owned_group{unsafe{libc::kill(-(group as i32),libc::SIGTERM);}}
    if result.is_err(){let _=child.start_kill();let _=tokio::time::timeout(Duration::from_secs(5),child.wait()).await;}
    // Read the final buffered output before freezing this attempt's capture.
    for task in [&mut output,&mut error]{if tokio::time::timeout(Duration::from_secs(1),&mut *task).await.is_err(){task.abort();let _=task.await;}}
    result
}
fn setup_failed()->ManagementError{ManagementError::new(409,"NATIVE_SETUP_FAILED","The recorded native setup command did not finish successfully. Fetch did not read or copy its credentials.")}
impl Manager {
    pub(super) async fn native_setup_for(&self,record:&Value)->Result<Setup,ManagementError>{
        let descriptor=setup(record)?;let target=descriptor.state_directory.clone();let id=record["server_id"].clone();let repository=self.repository.clone();
        tokio::task::spawn_blocking(move||{
            let target=canonical_location(&target)?;
            let records=repository.list_saved().map_err(|_|folder_unavailable())?;
            for other in records{if other["server_id"]==id{continue;}if let Ok(other)=setup(&other){if canonical_location(&other.state_directory)?==target{return Err(ManagementError::new(409,"SERVER_STATE_SHARED","Two instances point to the same native state folder. Record separate supported locations before using native setup."));}}}
            Ok(())
        }).await.map_err(|_|folder_unavailable())??;
        Ok(descriptor)
    }
    pub async fn open_server_folder(&self,id:&str)->Result<Value,ManagementError>{
        let gate=self.lock(id).await;let _guard=gate.lock().await;let record=self.record(id)?;
        let setup=self.native_setup_for(&record).await.map_err(|_|folder_unavailable())?;
        let directory=tokio::fs::canonicalize(&setup.state_directory).await.map_err(|_|folder_unavailable())?;
        if !tokio::fs::metadata(&directory).await.is_ok_and(|metadata|metadata.is_dir()){return Err(folder_unavailable());}
        #[cfg(target_os="macos")]{let mut command=tokio::process::Command::new("/usr/bin/open");command.arg(&directory).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).kill_on_drop(true);match tokio::time::timeout(Duration::from_secs(5),command.status()).await{Ok(Ok(status))if status.success()=>Ok(json!({"server_id":id,"status":"opened","next_steps":[]})),_=>Err(ManagementError::new(503,"SERVER_FOLDER_OPEN_FAILED","Finder could not open the recorded server folder."))}}
        #[cfg(not(target_os="macos"))]{let _=directory;Err(ManagementError::new(503,"SERVER_FOLDER_OPEN_FAILED","Native folder opening is supported only on macOS."))}
    }
}
fn folder_unavailable()->ManagementError{ManagementError::new(409,"SERVER_FOLDER_UNAVAILABLE","This instance has no available recorded native configuration/state folder. Fetch will not guess that its package or HOME is the actual location.")}

// Resolve symlinks through the nearest existing ancestor, including a native
// state directory that the first setup command has not created yet.
fn canonical_location(path:&Path)->Result<PathBuf,ManagementError>{
    if !path.is_absolute(){return Err(folder_unavailable());}
    let mut existing=path;let mut missing=Vec::new();loop{match std::fs::canonicalize(existing){Ok(mut canonical)=>{for component in missing.iter().rev(){canonical.push(component);}return Ok(canonical);},Err(error)if error.kind()==std::io::ErrorKind::NotFound=>{let name=existing.file_name().ok_or_else(folder_unavailable)?;missing.push(name.to_owned());existing=existing.parent().ok_or_else(folder_unavailable)?;},Err(_)=>return Err(folder_unavailable())}}
}

// The shell receives the very same assigned settings and private account paths
// as a command-based MCP. It never reads provider-owned credential files.
impl Manager {
    pub async fn terminal_action(&self,id:&str,action:crate::terminal::Action)->Result<(Value,bool),crate::terminal::TerminalError>{
        use crate::terminal::{Action,Launch,TerminalError};
        let gate=self.lock(id).await;let _guard=gate.lock().await;
        let record=self.record(id)?;
        if self.stopping.load(std::sync::atomic::Ordering::SeqCst){return Err(TerminalError::new(503,"TERMINAL_UNAVAILABLE","Fetch is shutting down."));}
        if record["config"]["kind"]=="remote" {return Err(TerminalError::new(422,"TERMINAL_NOT_LOCAL","This server uses a remote URL and has no local terminal environment."));}
        let terminals=self.terminals.clone();let server=id.to_owned();
        if let Action::Open{columns,rows}=action {
            if let Some(value)=terminals.existing(id){return Ok((value,false));}
            let manual=self.manual_for_record(&record).await?;let root=self.repository.root().to_owned();
            tokio::task::spawn_blocking(move||{
                let TransportConfig::Stdio{working_directory:Some(working_directory),environment,..}=transport(&root,&record,&manual)? else{return Err(TerminalError::new(422,"TERMINAL_NOT_LOCAL","This server has no local terminal environment."))};
                terminals.open(&server,Launch::system(working_directory,environment)?,columns,rows)
            }).await.map_err(|_|TerminalError::new(503,"TERMINAL_START_FAILED","The terminal worker stopped before completing the request."))?
        }else{
            // Input may block on the PTY. It must not prevent a separate close
            // or server-removal request from terminating that same shell.
            drop(_guard);
            tokio::task::spawn_blocking(move||terminals.apply(&server,action).map(|value|(value,false))).await.map_err(|_|TerminalError::new(409,"TERMINAL_UNAVAILABLE","The terminal worker stopped before completing the request."))?
        }
    }
    pub async fn terminal_output(&self,id:&str,session:&str,after:u64)->Result<Value,crate::terminal::TerminalError>{
        self.record(id)?;
        self.terminals.read(id,session,after)
    }
}
