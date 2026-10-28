//! Explicit developer-only launcher. It never creates a lasting TLS private key.
use crate::{
    api::{AppState, router},
    authority::PendingNativeAuthority,
    discovery::DiscoveryService,
    storage::ServerRepository,
};
use axum_server::{Handle, tls_rustls::RustlsConfig};
use rand::random;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};

pub struct DevelopmentOptions {
    pub data_dir: PathBuf,
    pub port: u16,
}

pub struct DevelopmentServer {
    pub origin: String,
    pub certificate_path: PathBuf,
    pub certificate_pem: String,
    pub state: Arc<AppState>,
    listener: TcpListener,
    tls: RustlsConfig,
    _instance_lock: File,
}

impl DevelopmentServer {
    /// Only creates explicitly selected developer state. No personal home scan.
    pub async fn prepare(options: DevelopmentOptions) -> io::Result<Self> {
        if options.port != 0 && options.port < 1024 {
            return Err(io::Error::other("Choose an unprivileged development port"));
        }
        let root = private_directory(&options.data_dir)?;
        let lock_path = root.join("instance.lock");
        let mut open = OpenOptions::new();
        open.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            open.mode(0o600)
                .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        }
        let lock = open.open(lock_path)?;
        fs2::FileExt::try_lock_exclusive(&lock).map_err(|_| {
            io::Error::other("Fetch is already using this developer data directory")
        })?;
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, options.port))?;
        listener.set_nonblocking(true)?;
        let port = listener.local_addr()?.port();
        let origin = format!("https://localhost:{port}");

        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(vec!["localhost".into()]).map_err(|_| {
                io::Error::other("Could not create the ephemeral developer TLS identity")
            })?;
        let certificate_pem = cert.pem();
        // The private key is passed straight to rustls; it is never written to disk.
        let key_der = zeroize::Zeroizing::new(signing_key.serialize_der());
        let tls = RustlsConfig::from_der(vec![cert.der().to_vec()], key_der.to_vec()).await?;
        let certificate_path = root.join("certificate.pem");
        let temporary = root.join(format!(".certificate-{:016x}", random::<u64>()));
        let result = (|| {
            let mut open = OpenOptions::new();
            open.write(true).create_new(true);
            #[cfg(unix)]
            {
                open.mode(0o600).custom_flags(libc::O_NOFOLLOW);
            }
            let mut file = open.open(&temporary)?;
            file.write_all(certificate_pem.as_bytes())?;
            file.sync_all()?;
            fs::rename(&temporary, &certificate_path)?;
            File::open(&root)?.sync_all()
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
        let fixture_home = private_directory(&root.join("config-fixtures"))?;
        let state = Arc::new(
            AppState::new(
                origin.clone(),
                ServerRepository::new(root.clone()),
                DiscoveryService::new(fixture_home, None),
                Arc::new(PendingNativeAuthority),
            )
            .map_err(io::Error::other)?,
        );
        Ok(Self {
            origin,
            certificate_path,
            certificate_pem,
            state,
            listener,
            tls,
            _instance_lock: lock,
        })
    }

    pub fn address(&self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn run(self, handle: Handle<SocketAddr>) -> io::Result<()> {
        // Keep the single-instance lock alive until the listener has finished.
        let _lock = self._instance_lock;
        axum_server::from_tcp_rustls(self.listener, self.tls)?
            .handle(handle)
            .serve(router(self.state).into_make_service())
            .await
    }
}

fn private_directory(path: &Path) -> io::Result<PathBuf> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(io::Error::other(
                    "The developer data path must be a real directory",
                ));
            }
            #[cfg(unix)]
            if metadata.permissions().mode() & 0o077 != 0 {
                return Err(io::Error::other(
                    "The developer data directory must be private (mode 700)",
                ));
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            builder.recursive(true);
            #[cfg(unix)]
            {
                builder.mode(0o700);
            }
            builder.create(path)?;
        }
        Err(error) => return Err(error),
    }
    fs::canonicalize(path)
}

pub fn shutdown_timeout() -> Duration {
    Duration::from_secs(3)
}
