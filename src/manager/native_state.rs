//! Shared unmodified process startup and explicitly recorded native setup.
use super::{Manager,ManagementError,bindings,invalid,invalid_config,settings_step,string_array};
use crate::{installation::private_directory,runner::TransportConfig};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{path::{Path,PathBuf},sync::Arc,time::Duration};
use tokio::io::AsyncReadExt;

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
    environment.insert("HOME".into(),home.to_string_lossy().into_owned());environment.insert("XDG_CONFIG_HOME".into(),home.join(".config").to_string_lossy().into_owned());environment.insert("XDG_STATE_HOME".into(),home.join(".local/state").to_string_lossy().into_owned());
    environment.entry("PATH".into()).or_insert_with(||executable.parent().map(|path|format!("{}:/usr/bin:/bin",path.display())).unwrap_or_else(||"/usr/bin:/bin".into()));environment.entry("LANG".into()).or_insert_with(||"en_US.UTF-8".into());
    if environment.iter().any(|(name,value)|name.contains(['=','\0'])||value.contains('\0')){return Err(invalid_config());}
    let working_directory=config["working_directory"].as_str().map(PathBuf::from).unwrap_or(home);
    if !working_directory.is_absolute(){return Err(invalid_config());}
    Ok(TransportConfig::Stdio{executable,args:string_array(&config["args"] )?,working_directory:Some(working_directory),environment})
}
pub(super) async fn run_setup(root:&Path,record:&Value,setup:&Setup,manual:&Value,stop:Arc<tokio::sync::Notify>)->Result<(),ManagementError>{
    let quit=stop.notified();tokio::pin!(quit);quit.as_mut().enable();
    let root=root.to_owned();let record=record.clone();let manual=manual.clone();
    let transport=tokio::task::spawn_blocking(move||transport(&root,&record,&manual)).await.map_err(|_|unsupported())??;
    let TransportConfig::Stdio{environment,..}=transport else{return Err(unsupported())};
    tokio::select!{biased;_= &mut quit=>return Err(setup_failed()),_=std::future::ready(())=>{}}
    let mut command=tokio::process::Command::new(&setup.executable);
    command.args(&setup.args).env_clear().envs(environment).current_dir(&setup.working_directory).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).kill_on_drop(true);
    #[cfg(unix)]command.process_group(0);
    let mut child=command.spawn().map_err(|_|setup_failed())?;
    let owned_group=child.id();
    let stdout=child.stdout.take();let stderr=child.stderr.take();
    let drain=|stream:Option<tokio::process::ChildStdout>|tokio::spawn(async move{if let Some(mut stream)=stream{let mut buffer=[0;4096];while matches!(stream.read(&mut buffer).await,Ok(size)if size>0){}}});
    let output=drain(stdout);let error=tokio::spawn(async move{if let Some(mut stream)=stderr{let mut buffer=[0;4096];while matches!(stream.read(&mut buffer).await,Ok(size)if size>0){}}});
    let result=tokio::select!{value=child.wait()=>value.map_err(|_|setup_failed()).and_then(|status|if status.success(){Ok(())}else{Err(setup_failed())}),_=tokio::time::sleep(Duration::from_secs(600))=>Err(ManagementError::new(409,"AUTHENTICATION_TIMEOUT","The recorded native setup command exceeded ten minutes.")),_= &mut quit=>Err(setup_failed())};
    // The helper owns this process group. Reap its remaining descendants on
    // completion as well as failure so callback/helper processes do not linger.
    #[cfg(unix)] if let Some(group)=owned_group{unsafe{libc::kill(-(group as i32),libc::SIGTERM);}}
    if result.is_err(){let _=child.start_kill();let _=tokio::time::timeout(Duration::from_secs(5),child.wait()).await;}
    output.abort();error.abort();result
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
