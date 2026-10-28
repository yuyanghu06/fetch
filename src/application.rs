//! One application process owns the listener, identity, manager, runner and native UI.
use crate::{api::{AppState, runtime::ManagementAdapter}, credentials::{CredentialStore,MacKeychainKeys}, discovery::DiscoveryService, identity::InstallationIdentity, installation::Installer, manager::Manager, mcp::McpGateway, native_auth::NativeAuthority, oauth::OAuthService, runner::Runner, storage::ServerRepository};
use axum_server::{Handle,tls_rustls::RustlsConfig};
use sha2::{Digest,Sha256};
use std::{fs::{self,File,OpenOptions},io::{self,Write},net::{Ipv4Addr,TcpListener},os::unix::fs::{DirBuilderExt,FileTypeExt,MetadataExt,OpenOptionsExt,PermissionsExt},path::{Path,PathBuf},sync::{Arc,Mutex,atomic::{AtomicBool,Ordering}},time::Duration};
use tokio::io::{AsyncReadExt,AsyncWriteExt};

pub struct ApplicationHost {
    pub state:Arc<AppState>, pub native:Arc<NativeAuthority>,pub certificate_pem:String,pub certificate_path:PathBuf,
    manager:Arc<Manager>,oauth:Arc<OAuthService>,handle:Handle<std::net::SocketAddr>,
    listener:Mutex<Option<TcpListener>>,tls:RustlsConfig,_instance_lock:File,root:PathBuf,
    show:AtomicBool,stopping:AtomicBool,stopped:AtomicBool,shutdown_done:tokio::sync::Notify,
}
impl ApplicationHost {
    /// This function is called only when the user explicitly launches the application.
    pub async fn prepare(root:PathBuf,port:u16,discovery_home:PathBuf,inherit_codex_home:bool)->io::Result<Arc<Self>> {
        if !root.is_absolute()||port<1024{return Err(io::Error::other("Choose an absolute private data folder and an unprivileged fixed HTTPS port."));}
        private_directory(&root)?;let root=fs::canonicalize(root)?;
        let lock=OpenOptions::new().read(true).write(true).create(true).truncate(false).mode(0o600).custom_flags(libc::O_NOFOLLOW|libc::O_CLOEXEC).open(root.join("instance.lock"))?;
        if !lock.metadata()?.is_file()||lock.metadata()?.nlink()!=1||lock.metadata()?.permissions().mode()&0o077!=0||lock.metadata()?.uid()!=unsafe{libc::geteuid()}{return Err(io::Error::other("Fetch's instance lock is unsafe."));}
        fs2::FileExt::try_lock_exclusive(&lock).map_err(|_|io::Error::new(io::ErrorKind::AlreadyExists,"Fetch is already running with this data folder."))?;
        let listener=TcpListener::bind((Ipv4Addr::LOCALHOST,port)).map_err(|_|io::Error::other("Fetch's local HTTPS port is in use. Close the other listener or choose a different explicit port."))?;listener.set_nonblocking(true)?;
        let origin=format!("https://localhost:{port}");
        let namespace=format!("{:x}",Sha256::digest(root.as_os_str().as_encoded_bytes()));
        let credentials=Arc::new(CredentialStore::new(root.clone(),Arc::new(MacKeychainKeys::new(namespace))));
        let protected=credentials.clone();let identity_root=root.clone();
        let identity=tokio::task::spawn_blocking(move||InstallationIdentity::load_or_create(&identity_root,&protected)).await.map_err(|_|io::Error::other("Fetch's installation identity could not be loaded."))?.map_err(io::Error::other)?;
        let certificate_pem=identity.certificate_pem;let certificates=rustls_pemfile::certs(&mut std::io::BufReader::new(certificate_pem.as_bytes())).map(|value|value.map(|v|v.to_vec())).collect::<Result<Vec<_>,_>>()?;
        let tls=RustlsConfig::from_der(certificates,identity.private_key_der.to_vec()).await?;
        let certificate_path=root.join("certificate.pem");write_public_certificate(&root,&certificate_path,certificate_pem.as_bytes())?;
        let native=NativeAuthority::new();let authority_store=credentials.clone();let authority_native=native.clone();let authority_root=root.clone();let authority_origin=origin.clone();
        let oauth=tokio::task::spawn_blocking(move||OAuthService::open(authority_origin,authority_root,authority_store,authority_native)).await.map_err(|_|io::Error::other("Fetch authorization could not start."))?.map_err(io::Error::other)?;
        let codex_home=if inherit_codex_home{std::env::var_os("CODEX_HOME").map(PathBuf::from).filter(|p|p.is_absolute()&&p.is_dir())}else{None};
        let discovery=DiscoveryService::new(discovery_home,codex_home);let repository=ServerRepository::new(root.clone());
        let runner=Runner::persistent(root.clone()).map_err(io::Error::other)?;let installer=Installer::new(root.clone(),discovery.clone());let manager=Manager::new(repository.clone(),credentials,runner.clone(),installer);manager.set_origin(origin.clone());
        let backend=Arc::new(ManagementAdapter{manager:manager.clone(),discovery:discovery.clone()});
        let gateway=McpGateway::new(runner.clone(),backend,oauth.clone(),origin.clone());
        let mut state=AppState::new(origin,repository,discovery,oauth.clone()).map_err(io::Error::other)?;
        state.manager=Some(manager.clone());state.native=Some(native.clone());state.oauth=Some(oauth.clone());state.gateway=Some(gateway.clone());
        let host=Arc::new(Self{state:Arc::new(state),native,certificate_pem,certificate_path,manager,oauth,handle:Handle::new(),listener:Mutex::new(Some(listener)),tls,_instance_lock:lock,root,show:AtomicBool::new(false),stopping:AtomicBool::new(false),stopped:AtomicBool::new(false),shutdown_done:tokio::sync::Notify::new()});
        let mut revocations=host.oauth.subscribe_revocations();let gateway_for_revoke=gateway.clone();
        tokio::spawn(async move{loop{match revocations.recv().await{Ok(owner)=>{runner.revoke_owner(&owner).await;gateway_for_revoke.revoke_owner(&owner).await;},Err(tokio::sync::broadcast::error::RecvError::Lagged(_))=>{gateway_for_revoke.recheck_authority().await;},Err(_)=>break}}});
        Ok(host)
    }
    pub fn start(self:&Arc<Self>)->io::Result<()> {
        let listener=self.listener.lock().map_err(|_|io::Error::other("Fetch's listener is unavailable."))?.take().ok_or_else(||io::Error::other("Fetch's listener was already started."))?;
        let tls=self.tls.clone();let handle=self.handle.clone();let state=self.state.clone();
        tokio::spawn(async move{let server=match axum_server::from_tcp_rustls(listener,tls){Ok(v)=>v,Err(_)=>{eprintln!("Fetch could not start its local HTTPS listener.");return;}};if server.handle(handle).serve(crate::api::router(state).into_make_service()).await.is_err(){eprintln!("Fetch's local HTTPS listener stopped unexpectedly.");}});
        let manager=self.manager.clone();tokio::spawn(async move{manager.startup().await;});
        self.start_local_activation()?;Ok(())
    }
    fn start_local_activation(self:&Arc<Self>)->io::Result<()> {
        let path=self.root.join("activation.sock");
        if let Ok(metadata)=fs::symlink_metadata(&path){if !metadata.file_type().is_socket()||metadata.uid()!=unsafe{libc::geteuid()}{return Err(io::Error::other("Fetch's private activation socket is unsafe."));}fs::remove_file(&path)?;}
        let listener=tokio::net::UnixListener::bind(&path)?;fs::set_permissions(&path,fs::Permissions::from_mode(0o600))?;
        let weak=Arc::downgrade(self);tokio::spawn(async move{while let Ok((mut stream,_))=listener.accept().await{let mut bytes=[0u8;4];if tokio::time::timeout(Duration::from_secs(1),stream.read_exact(&mut bytes)).await.is_ok_and(|r|r.is_ok())&&&bytes==b"show"{let Some(host)=weak.upgrade()else{break;};if host.stopping.load(Ordering::Acquire){break;}host.show.store(true,Ordering::Release);}}});Ok(())
    }
    pub fn take_show_request(&self)->bool{self.show.swap(false,Ordering::AcqRel)}
    pub fn is_stopped(&self)->bool{self.stopped.load(Ordering::Acquire)}
    pub async fn shutdown(&self){
        if self.stopping.swap(true,Ordering::AcqRel){
            loop { let done=self.shutdown_done.notified(); if self.stopped.load(Ordering::Acquire){return;} done.await; }
        }
        self.oauth.shutdown();self.manager.shutdown().await;
        self.handle.graceful_shutdown(Some(Duration::from_secs(3)));
        let _=fs::remove_file(self.root.join("activation.sock"));
        self.stopped.store(true,Ordering::Release);self.shutdown_done.notify_waiters();
    }
}
pub fn default_data_root()->io::Result<PathBuf>{let home=std::env::var_os("HOME").map(PathBuf::from).filter(|p|p.is_absolute()).ok_or_else(||io::Error::other("Fetch could not find the current user's home folder."))?;Ok(home.join("Library/Application Support/Fetch"))}
pub fn home_directory()->io::Result<PathBuf>{std::env::var_os("HOME").map(PathBuf::from).filter(|p|p.is_absolute()).ok_or_else(||io::Error::other("Fetch could not find the current user's home folder."))}
pub async fn show_existing(root:&Path)->io::Result<()>{let mut stream=tokio::time::timeout(Duration::from_secs(2),tokio::net::UnixStream::connect(root.join("activation.sock"))).await.map_err(|_|io::Error::other("Fetch is already running; open it from its Dock or menu."))??;stream.write_all(b"show").await}
fn private_directory(path:&Path)->io::Result<()>{match fs::symlink_metadata(path){Ok(metadata)=>{if !metadata.is_dir()||metadata.file_type().is_symlink()||metadata.permissions().mode()&0o077!=0||metadata.uid()!=unsafe{libc::geteuid()}{return Err(io::Error::other("Fetch requires an owner-only data folder with mode 700."));}},Err(error)if error.kind()==io::ErrorKind::NotFound=>fs::DirBuilder::new().recursive(true).mode(0o700).create(path)?,Err(error)=>return Err(error)}Ok(())}
fn write_public_certificate(root:&Path,path:&Path,bytes:&[u8])->io::Result<()>{if let Ok(metadata)=fs::symlink_metadata(path){if !metadata.is_file()||metadata.file_type().is_symlink()||metadata.nlink()!=1{return Err(io::Error::other("Fetch's public certificate path is unsafe."));}}let temporary=root.join(format!(".certificate-{:032x}",rand::random::<u128>()));let result=(||{let mut file=OpenOptions::new().write(true).create_new(true).mode(0o600).custom_flags(libc::O_NOFOLLOW).open(&temporary)?;file.write_all(bytes)?;file.sync_all()?;fs::rename(&temporary,path)?;File::open(root)?.sync_all()})();if result.is_err(){let _=fs::remove_file(temporary);}result}
