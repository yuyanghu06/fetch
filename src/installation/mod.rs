//! Owned, pinned preparation. Constructing this service never downloads or executes anything.
use crate::{discovery::{CapturedDefinition, DiscoveryService}, storage::ServerRepository};
use rand::{RngCore, rngs::OsRng};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt}, io::Write, path::{Path, PathBuf}, sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
use tokio::{process::Command, sync::{Mutex, Notify}};

pub const NODE_VERSION: &str = "24.22.1";
pub const UV_VERSION: &str = "0.13.5";
pub const PYTHON_VERSION: &str = "3.13.18";
pub const RAILWAY_CLI_VERSION: &str = "5.71.0";
const INSTALL_SECONDS: u64 = 600;
const MAX_DOWNLOAD: usize = 256 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct InstallError { pub http_status:u16, pub code:&'static str, pub message:&'static str, pub installation_id:Option<String> }
impl InstallError { fn invalid()->Self { Self{http_status:400,code:"INVALID_REQUEST",message:"Select nonempty configuration-file and server-name pairs.",installation_id:None} } }
#[derive(Clone)]
struct Selection { path:PathBuf, name:String }
struct Job { id:String, status:&'static str, items:Vec<Value> }
struct Jobs { active:Option<String>, all:Vec<Job> }
pub struct Installer { root:PathBuf, repository:ServerRepository, discovery:DiscoveryService, jobs:Mutex<Jobs>, runtime_lock:Mutex<()>, stopping:AtomicBool, stop:Notify, client:reqwest::Client }
impl Installer {
    pub fn new(root:PathBuf, discovery:DiscoveryService)->Arc<Self> {
        let client = reqwest::Client::builder().connect_timeout(Duration::from_secs(15)).timeout(Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::custom(|attempt| {
                if attempt.previous().len() >= 5 || attempt.url().scheme() != "https" || !attempt.url().host_str().is_some_and(|h| h=="github.com" || h=="release-assets.githubusercontent.com" || h=="objects.githubusercontent.com") { attempt.stop() } else { attempt.follow() }
            })).build().expect("static HTTP client configuration");
        Arc::new(Self{repository:ServerRepository::new(root.clone()),root,discovery,jobs:Mutex::new(Jobs{active:None,all:vec![]}),runtime_lock:Mutex::new(()),stopping:AtomicBool::new(false),stop:Notify::new(),client})
    }
    pub async fn submit(self:&Arc<Self>, body:Value)->Result<Value,InstallError> {
        let object=body.as_object().filter(|o|o.len()==1).ok_or_else(InstallError::invalid)?;
        let rows=object.get("servers").and_then(Value::as_array).filter(|a|!a.is_empty() && a.len()<=256).ok_or_else(InstallError::invalid)?;
        let mut selections=Vec::new(); let mut unique=BTreeSet::new();
        for row in rows {
            let o=row.as_object().filter(|o|o.len()==2).ok_or_else(InstallError::invalid)?;
            let p=o.get("config_path").and_then(Value::as_str).filter(|p|!p.trim().is_empty() && !p.contains('\0') && Path::new(p).is_absolute()).ok_or_else(InstallError::invalid)?;
            let n=o.get("server_name").and_then(Value::as_str).filter(|n|!n.trim().is_empty() && !n.contains('\0')).ok_or_else(InstallError::invalid)?;
            if !unique.insert((p.to_owned(),n.to_owned())) { return Err(InstallError::invalid()); }
            selections.push(Selection{path:p.into(),name:n.into()});
        }
        let mut jobs=self.jobs.lock().await;
        if let Some(id)=&jobs.active { return Err(InstallError{http_status:409,code:"INSTALLATION_IN_PROGRESS",message:"An installation is already running. Monitor its progress before submitting another.",installation_id:Some(id.clone())}); }
        if jobs.all.len()>=4096 {return Err(InstallError{http_status:503,code:"INSTALLATION_ACCEPTANCE_FAILED",message:"This Fetch run has reached its installation-history capacity. Existing current-run results remain available.",installation_id:None});}
        if self.stopping.load(Ordering::SeqCst) { return Err(InstallError{http_status:503,code:"INSTALLATION_ACCEPTANCE_FAILED",message:"Fetch is shutting down.",installation_id:None}); }
        let id=opaque("installation");
        let items=selections.iter().map(|s|json!({"config_path":s.path,"server_name":s.name,"server_id":null,"status":"pending","error":null})).collect();
        jobs.active=Some(id.clone()); jobs.all.push(Job{id:id.clone(),status:"installing",items}); drop(jobs);
        let service=self.clone(); let task_id=id.clone();
        tokio::spawn(async move { service.prepare_group(task_id,selections).await; });
        Ok(json!({"installation_id":id,"status":"installing","next_steps":[monitor_installation(&id)]}))
    }
    pub async fn progress(&self,id:&str)->Result<Value,InstallError> {
        let jobs=self.jobs.lock().await;
        let job=jobs.all.iter().find(|j|j.id==id).ok_or(InstallError{http_status:404,code:"INSTALLATION_NOT_FOUND",message:"No matching installation exists in this Fetch run.",installation_id:None})?;
        let steps=if job.status=="installing" {vec![monitor_installation(id)]} else if job.status=="failed" {vec![json!({"actor":"agent","action":"resolve_installation_failures","instructions":"Resolve failed entries and submit only those entries again. Successful installations are preserved."})]} else {vec![json!({"actor":"agent","action":"check_server_status","instructions":"Read the resulting servers through GET /api/v1/servers before authenticating or connecting. Preparation does not establish readiness."})]};
        Ok(json!({"installation_id":id,"status":job.status,"servers":job.items,"next_steps":steps}))
    }
    async fn prepare_group(&self,id:String,selections:Vec<Selection>) {
        for (index,selection) in selections.iter().enumerate() {
            self.item_state(&id,index,"installing",None,None).await;
            let result=if self.stopping.load(Ordering::SeqCst) {Err(())} else {self.prepare(selection).await};
            match result {Ok(server_id)=>self.item_state(&id,index,"succeeded",Some(server_id),None).await,Err(())=>self.item_state(&id,index,"failed",None,Some(json!({"code":"INSTALLATION_FAILED","message":"Preparation could not complete. The source, package, runtime or saved configuration needs attention; no credentials were imported."}))).await}
        }
        let mut jobs=self.jobs.lock().await;
        if let Some(job)=jobs.all.iter_mut().find(|j|j.id==id) {job.status=if job.items.iter().any(|v|v["status"]=="failed"){"failed"}else{"succeeded"};}
        if jobs.active.as_deref()==Some(&id) {jobs.active=None;}
    }
    async fn item_state(&self,id:&str,index:usize,status:&str,server:Option<String>,error:Option<Value>) {
        let mut jobs=self.jobs.lock().await;
        if let Some(job)=jobs.all.iter_mut().find(|j|j.id==id) {job.items[index]["status"]=json!(status);job.items[index]["server_id"]=json!(server);job.items[index]["error"]=json!(error);}
    }
    async fn prepare(&self,selection:&Selection)->Result<String,()> {
        let records=self.repository.list_saved().map_err(|_|())?;
        if let Some(old)=records.iter().find(|v|v["_installation"]["source_path"]==json!(selection.path) && v["_installation"]["source_name"]==selection.name) {return old["server_id"].as_str().map(str::to_owned).ok_or(());}
        let capture=self.discovery.capture(&selection.path,&selection.name).map_err(|_|())?;
        let server_id=opaque("server"); let installation=opaque("package");
        let mut config=capture.config.clone(); let kind=capture.row["type"].as_str().ok_or(())?;
        let prepared=match kind {
            "npm" => self.prepare_npm(&installation,&capture).await,
            "pypi" => self.prepare_python(&installation,&capture).await,
            "local_script" => self.prepare_script(&capture).await,
            "remote" => Ok(config.clone()),
            _=>Err(())
        };
        let prepared=match prepared {Ok(config)=>config,Err(())=>{let _=fs::remove_dir_all(self.root.join("packages/servers").join(&installation));return Err(());}};
        config=prepared;
        if self.stopping.load(Ordering::SeqCst){let _=fs::remove_dir_all(self.root.join("packages/servers").join(&installation));return Err(());}
        let names:BTreeSet<String>=capture.row["credential_variables"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned)
            .chain(config["connection"]["headers"].as_array().into_iter().flatten().filter_map(|v|v["credential_ref"].as_str()).map(str::to_owned)).collect();
        let credentials:Vec<Value>=names.iter().map(|n|json!({"name":n,"status":"missing"})).collect();
        let protected_args=if kind=="npm" {1} else if kind=="local_script" {config["args"].as_array().map_or(0,Vec::len)} else {0};
        let record=json!({"server_id":server_id,"name":selection.name,"runtime":{"status":"stopped","error":null},"authentication":{"status":"unknown","error":null},"restart_required":kind=="remote","restart":null,"config":config,"credentials":credentials,"_installation":{"source_path":selection.path,"source_name":selection.name,"installation_id":installation,"protected_args":protected_args}});
        if self.repository.save(&server_id,&record).is_err() {let _=fs::remove_dir_all(self.root.join("packages/servers").join(&installation));return Err(());}
        Ok(server_id)
    }
    async fn prepare_npm(&self,id:&str,capture:&CapturedDefinition)->Result<Value,()> {
        let node=self.node_runtime().await?; let directory=self.package_directory(id)?;
        let requested=capture.row["package"].as_str().ok_or(())?;
        let version=capture.row["version_spec"].as_str().unwrap_or("latest");
        let spec=format!("{requested}@{version}");
        let mut command=self.command(&node.join("bin/node"),&directory)?;
        command.arg(node.join("lib/node_modules/npm/bin/npm-cli.js")).args(["install","--save-exact","--no-audit","--no-fund","--no-update-notifier","--"]).arg(spec);
        command.env("PATH",format!("{}:/usr/bin:/bin",node.join("bin").display()));
        self.execute(command).await?;
        if requested=="@railway/mcp-server" { self.ensure_railway_cli(id).await?; }
        let package_dir=directory.join("node_modules").join(requested);
        let metadata=read_json(&package_dir.join("package.json"),2*1024*1024)?;
        let installed=metadata["version"].as_str().filter(|v|!v.is_empty()).ok_or(())?;
        let binary=match &metadata["bin"] {Value::String(s)=>s.clone(),Value::Object(o) if o.len()==1=>o.values().next().and_then(Value::as_str).ok_or(())?.to_owned(),Value::Object(o)=>o.get(requested.rsplit('/').next().ok_or(())?).and_then(Value::as_str).ok_or(())?.to_owned(),_=>return Err(())};
        let binary=fs::canonicalize(package_dir.join(binary)).map_err(|_|())?;
        let package_real=fs::canonicalize(&package_dir).map_err(|_|())?;
        if !binary.starts_with(&package_real) || !binary.is_file() {return Err(());}
        let mut config=capture.config.clone(); config["package"]=json!({"name":requested,"requested_version":capture.row["version_spec"],"installed_version":installed});config["executable"]=json!(node.join("bin/node"));
        let mut args=vec![json!(binary)];args.extend(capture.package_arguments.iter().map(|a|json!(a)));config["args"]=json!(args);
        Ok(config)
    }
    /// Called during preparation or an explicit Railway Connect, never status GET.
    /// The CLI's postinstall runs in the existing private preparation HOME.
    pub async fn ensure_railway_cli(&self,id:&str)->Result<PathBuf,()> {
        if !id.strip_prefix("package-").is_some_and(|v|v.len()==32&&v.bytes().all(|b|b.is_ascii_hexdigit())) {return Err(());}
        let package_root=self.root.join("packages/servers").join(id);
        if !package_root.is_dir() {return Err(());}
        let directory=package_root.join("railway-cli");private_directory(&directory)?;
        let manifest=directory.join("package.json");
        if !manifest.exists(){write_private(&manifest,b"{\"name\":\"fetch-private-railway-cli\",\"private\":true}\n")?;}
        let project=read_json(&manifest,2*1024*1024)?;
        if project["name"]!="fetch-private-railway-cli"||project["private"]!=true{return Err(());}
        let bin=directory.join("node_modules/.bin");
        let native=directory.join("node_modules/@railway/cli/bin/railway");
        let metadata=directory.join("node_modules/@railway/cli/package.json");
        let contained=|path:&Path|fs::canonicalize(path).is_ok_and(|p|p.starts_with(&directory));
        let ready=|| bin.join("railway").is_file() && native.is_file() && contained(&bin.join("railway")) && contained(&native)
            && fs::metadata(bin.join("railway")).is_ok_and(|m|m.permissions().mode()&0o111!=0)
            && fs::metadata(&native).is_ok_and(|m|m.permissions().mode()&0o111!=0)
            && read_json(&metadata,2*1024*1024).is_ok_and(|v|v["version"]==RAILWAY_CLI_VERSION);
        if ready(){return Ok(bin);}
        let node=self.node_runtime().await?;
        let mut command=self.command(&node.join("bin/node"),&directory)?;
        command.arg(node.join("lib/node_modules/npm/bin/npm-cli.js")).arg("--prefix").arg(&directory)
            .args(["install","--save-exact","--no-audit","--no-fund","--no-update-notifier","--"])
            .arg(format!("@railway/cli@{RAILWAY_CLI_VERSION}"));
        command.env("PATH",format!("{}:/usr/bin:/bin",node.join("bin").display()));
        self.execute(command).await?;
        if ready(){Ok(bin)}else{Err(())}
    }
    async fn prepare_python(&self,id:&str,capture:&CapturedDefinition)->Result<Value,()> {
        let uv=self.uv_runtime().await?; let directory=self.package_directory(id)?;
        let name=capture.row["package"].as_str().ok_or(())?;
        let requested=capture.row["version_spec"].as_str();
        let mut metadata_url=url::Url::parse("https://pypi.org/pypi/").map_err(|_|())?;
        metadata_url.path_segments_mut().map_err(|_|())?.pop_if_empty().extend([name,"json"]);
        let metadata:Value=serde_json::from_slice(&self.download(metadata_url.as_str(),4*1024*1024).await?).map_err(|_|())?;
        let version=match requested {Some(v) if v!="latest"=>v.to_owned(),_=>metadata["info"]["version"].as_str().ok_or(())?.to_owned()};
        let python_root=self.root.join("packages/runtimes/python");private_directory(&python_root)?;
        let mut install=self.command(&uv.join("uv"),&directory)?;
        install.args(["python","install",PYTHON_VERSION]); install.env("UV_PYTHON_INSTALL_DIR",&python_root).env("UV_PYTHON_INSTALL_BIN","0");
        self.execute(install).await?;
        let mut venv=self.command(&uv.join("uv"),&directory)?;venv.args(["venv","--managed-python","--python",PYTHON_VERSION]).arg(directory.join("venv")).env("UV_PYTHON_INSTALL_DIR",&python_root);
        self.execute(venv).await?;
        let python=directory.join("venv/bin/python");
        let mut pip=self.command(&uv.join("uv"),&directory)?;pip.args(["pip","install","--python"]).arg(&python).arg(format!("{name}=={version}"));self.execute(pip).await?;
        let bin=directory.join("venv/bin");
        let candidates=[name.to_owned(),name.replace('_',"-")];
        let executable=candidates.iter().map(|n|bin.join(n)).find(|p|p.is_file()).ok_or(())?;
        let installed=installed_python_version(&directory,name)?;
        let mut config=capture.config.clone();config["package"]=json!({"name":name,"requested_version":capture.row["version_spec"],"installed_version":installed});config["executable"]=json!(executable);config["args"]=json!(capture.package_arguments);Ok(config)
    }
    async fn prepare_script(&self,capture:&CapturedDefinition)->Result<Value,()> {
        let mut config=capture.config.clone();let original=config["executable"].as_str().ok_or(())?.to_owned();
        let script=capture.row["script_path"].as_str().ok_or(())?;if !Path::new(script).is_file(){return Err(());}
        let interpreter=Path::new(&original).file_name().and_then(|s|s.to_str()).unwrap_or("");
        if matches!(interpreter,"node"|"python"|"python3") {
            if interpreter=="node" {config["executable"]=json!(self.node_runtime().await?.join("bin/node"));}
            else {
                let uv=self.uv_runtime().await?;let root=self.root.join("packages/runtimes/python");private_directory(&root)?;
                let mut install=self.command(&uv.join("uv"),&root)?;install.args(["python","install",PYTHON_VERSION]).env("UV_PYTHON_INSTALL_DIR",&root).env("UV_PYTHON_INSTALL_BIN","0");self.execute(install).await?;
                let python=fs::read_dir(&root).map_err(|_|())?.filter_map(Result::ok).filter(|e|e.file_name().to_string_lossy().starts_with(&format!("cpython-{PYTHON_VERSION}-"))).map(|e|e.path().join("bin/python3")).find(|p|p.is_file()).ok_or(())?;
                config["executable"]=json!(python);
            }
        } else if !Path::new(&original).is_absolute() || !Path::new(&original).is_file() {return Err(());}
        if let Some(cwd)=config["working_directory"].as_str() {if !Path::new(cwd).is_dir(){return Err(());}}
        Ok(config)
    }
    async fn node_runtime(&self)->Result<PathBuf,()> {
        let _guard=self.runtime_lock.lock().await;let dest=self.root.join("packages/runtimes/node").join(NODE_VERSION);
        if dest.join("bin/node").is_file(){return Ok(dest);}
        let arch=architecture()?;let filename=format!("node-v{NODE_VERSION}-darwin-{arch}.tar.gz");let base=format!("https://nodejs.org/download/release/v{NODE_VERSION}");
        let checksums=self.download(&format!("{base}/SHASUMS256.txt"),128*1024).await?;
        let text=std::str::from_utf8(&checksums).map_err(|_|())?;
        let expected=text.lines().find_map(|line|{let mut fields=line.split_whitespace();let hash=fields.next()?;if fields.next()==Some(filename.as_str()){Some(hash.to_owned())}else{None}}).ok_or(())?;
        self.archive_runtime(&dest,&format!("{base}/{filename}"),&expected).await?;Ok(dest)
    }
    async fn uv_runtime(&self)->Result<PathBuf,()> {
        let _guard=self.runtime_lock.lock().await;let dest=self.root.join("packages/runtimes/uv").join(UV_VERSION);
        if dest.join("uv").is_file(){return Ok(dest);}
        let arch=if architecture()?=="arm64"{"aarch64"}else{"x86_64"};let filename=format!("uv-{arch}-apple-darwin.tar.gz");let base=format!("https://github.com/astral-sh/uv/releases/download/{UV_VERSION}");
        let sum=self.download(&format!("{base}/{filename}.sha256"),4096).await?;let text=std::str::from_utf8(&sum).map_err(|_|())?;let hash=text.split_whitespace().next().ok_or(())?;
        self.archive_runtime(&dest,&format!("{base}/{filename}"),hash).await?;Ok(dest)
    }
    async fn archive_runtime(&self,dest:&Path,url:&str,checksum:&str)->Result<(),()> {
        if checksum.len()!=64 || !checksum.bytes().all(|b|b.is_ascii_hexdigit()){return Err(());}
        let bytes=self.download(url,MAX_DOWNLOAD).await?;if format!("{:x}",Sha256::digest(&bytes))!=checksum.to_ascii_lowercase(){return Err(());}
        let parent=dest.parent().ok_or(())?;private_directory(parent)?;let stage=parent.join(opaque("stage"));private_directory(&stage)?;let _cleanup=OwnedStage(stage.clone());let archive=stage.join("runtime.tar.gz");write_private(&archive,&bytes)?;
        let extracted=stage.join("content");private_directory(&extracted)?;
        let mut list=self.command(Path::new("/usr/bin/tar"),&stage)?;list.args(["-tzf"]).arg(&archive).stdout(std::process::Stdio::piped());
        let output=tokio::time::timeout(Duration::from_secs(30),list.output()).await.map_err(|_|())?.map_err(|_|())?;
        if !output.status.success() || output.stdout.len()>1024*1024 {let _=fs::remove_dir_all(&stage);return Err(());}
        let entries=std::str::from_utf8(&output.stdout).map_err(|_|())?;
        if entries.lines().any(|p|Path::new(p).is_absolute() || Path::new(p).components().any(|c|matches!(c,std::path::Component::ParentDir))){let _=fs::remove_dir_all(&stage);return Err(());}
        let mut command=self.command(Path::new("/usr/bin/tar"),&stage)?;command.args(["-xzf"]).arg(&archive).args(["--strip-components","1","-C"]).arg(&extracted);self.execute(command).await?;
        fs::rename(&extracted,dest).map_err(|_|())?;let _=fs::remove_dir_all(&stage);Ok(())
    }
    async fn download(&self,url:&str,limit:usize)->Result<Vec<u8>,()> {
        let mut response=crate::diagnostics::send(crate::diagnostics::UpstreamOperation::PackageDownload,self.client.get(url)).await.map_err(|_|())?;if !response.status().is_success() || response.content_length().is_some_and(|n|n>limit as u64){return Err(());}
        let mut bytes=Vec::new();while let Some(chunk)=response.chunk().await.map_err(|_|())? {if bytes.len().saturating_add(chunk.len())>limit{return Err(());}bytes.extend_from_slice(&chunk);}Ok(bytes)
    }
    fn package_directory(&self,id:&str)->Result<PathBuf,()> {let path=self.root.join("packages/servers").join(id);private_directory(&path)?;Ok(path)}
    fn command(&self,exe:&Path,cwd:&Path)->Result<Command,()> {
        let home=cwd.join(".fetch-preparation-home");private_directory(&home)?;let cache=self.root.join("packages/cache");private_directory(&cache)?;
        let mut c=Command::new(exe);c.current_dir(cwd).env_clear().env("HOME",home).env("PATH","/usr/bin:/bin").env("LANG","en_US.UTF-8").env("UV_CACHE_DIR",cache.join("uv")).env("npm_config_cache",cache.join("npm")).env("UV_NO_CONFIG","1").env("UV_NO_PROGRESS","1").env("UV_PYTHON_PREFERENCE","only-managed").stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).kill_on_drop(true);
        use std::os::unix::process::CommandExt;
        c.as_std_mut().process_group(0);
        Ok(c)
    }
    async fn execute(&self,mut command:Command)->Result<(),()> {
        if self.stopping.load(Ordering::SeqCst){return Err(());}
        let mut child=command.spawn().map_err(|_|())?;
        if self.stopping.load(Ordering::SeqCst){terminate_group(&mut child).await;return Err(());}
        tokio::select! { result=tokio::time::timeout(Duration::from_secs(INSTALL_SECONDS),child.wait())=>match result {Ok(Ok(s)) if s.success()=>Ok(()),_=>{terminate_group(&mut child).await;Err(())}}, _=self.stop.notified()=>{terminate_group(&mut child).await;Err(())} }
    }
    pub fn shutdown(&self){self.stopping.store(true,Ordering::SeqCst);self.stop.notify_waiters();}
}
struct OwnedStage(PathBuf);
impl Drop for OwnedStage{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
async fn terminate_group(child:&mut tokio::process::Child){if let Some(pid)=child.id(){// SAFETY: this child was created in its own process group; never target Fetch's group.
    unsafe{libc::kill(-(pid as i32),libc::SIGKILL);}}
    let _=child.kill().await;let _=child.wait().await;
}
pub fn opaque(prefix:&str)->String {let mut bytes=[0u8;16];OsRng.fill_bytes(&mut bytes);format!("{prefix}-{:032x}",u128::from_be_bytes(bytes))}
fn architecture()->Result<&'static str,()> {if !cfg!(target_os="macos"){return Err(());} match std::env::consts::ARCH {"aarch64"=>Ok("arm64"),"x86_64"=>Ok("x64"),_=>Err(())}}
pub fn private_directory(path:&Path)->Result<(),()> {match fs::symlink_metadata(path) {Ok(m)=>{if !m.is_dir() || m.file_type().is_symlink() || m.permissions().mode()&0o077!=0{return Err(());}},Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{fs::DirBuilder::new().recursive(true).mode(0o700).create(path).map_err(|_|())?;},Err(_)=>return Err(())}Ok(())}
fn write_private(path:&Path,bytes:&[u8])->Result<(),()> {let mut f=fs::OpenOptions::new().write(true).create_new(true).mode(0o600).custom_flags(libc::O_NOFOLLOW).open(path).map_err(|_|())?;f.write_all(bytes).map_err(|_|())?;f.sync_all().map_err(|_|())}
fn read_json(path:&Path,limit:usize)->Result<Value,()> {use std::io::Read;let mut bytes=vec![];fs::File::open(path).map_err(|_|())?.take(limit as u64+1).read_to_end(&mut bytes).map_err(|_|())?;if bytes.len()>limit{return Err(());}serde_json::from_slice(&bytes).map_err(|_|())}
fn monitor_installation(id:&str)->Value {json!({"actor":"agent","action":"check_installation_status","instructions":format!("After one second, GET /api/v1/setup/installations/{id}. Continue only after overall success.")})}
fn installed_python_version(directory:&Path,requested:&str)->Result<String,()>{
    let canonical=|name:&str|name.to_ascii_lowercase().replace(['_','.'],"-");
    let libraries=fs::read_dir(directory.join("venv/lib")).map_err(|_|())?;
    for library in libraries.filter_map(Result::ok){let site=library.path().join("site-packages");let Ok(entries)=fs::read_dir(site)else{continue};for entry in entries.filter_map(Result::ok){if !entry.file_name().to_string_lossy().ends_with(".dist-info"){continue;}let path=entry.path().join("METADATA");let Ok(metadata)=read_text(&path,2*1024*1024)else{continue};let name=metadata.lines().find_map(|line|line.strip_prefix("Name: "));let version=metadata.lines().find_map(|line|line.strip_prefix("Version: "));if name.is_some_and(|n|canonical(n)==canonical(requested)){return version.filter(|v|!v.is_empty()&&v.len()<256&&v.chars().all(|c|c.is_ascii_alphanumeric()||".+_-!".contains(c))).map(str::to_owned).ok_or(());}}}
    Err(())
}
fn read_text(path:&Path,limit:usize)->Result<String,()>{use std::io::Read;let file=fs::File::open(path).map_err(|_|())?;if !file.metadata().map_err(|_|())?.is_file(){return Err(());}let mut bytes=vec![];file.take(limit as u64+1).read_to_end(&mut bytes).map_err(|_|())?;if bytes.len()>limit{return Err(());}String::from_utf8(bytes).map_err(|_|())}
