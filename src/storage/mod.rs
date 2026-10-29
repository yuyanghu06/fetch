//! Private, atomic local records. No API caller can write through this module directly.
use rand::{RngCore, rngs::OsRng};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd, IntoRawFd},
        unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt},
    },
    path::{Path, PathBuf},
};

const MAX_RECORD_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageError {
    Unavailable,
    InvalidRecord,
    CorruptRecord,
    UnsafePath,
    TooLarge,
}
impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fetch cannot safely access its saved records.")
    }
}
impl std::error::Error for StorageError {}

#[derive(Clone)]
pub struct ServerRepository {
    root: PathBuf,
}
impl ServerRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn root(&self) -> &Path { &self.root }
    /// Backend-only read of durable records. Live state must be supplied by the manager.
    pub fn get_saved(&self, id: &str) -> Result<Option<Value>, StorageError> {
        let Some(dir) = server_directory(&self.root, id, false)? else { return Ok(None) };
        let Some(bytes) = dir.read("config.json", MAX_RECORD_BYTES)? else { return Ok(None) };
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| StorageError::CorruptRecord)?;
        if !valid_record(id, &value) { return Err(StorageError::CorruptRecord); }
        Ok(Some(value))
    }
    pub fn list_saved(&self) -> Result<Vec<Value>, StorageError> {
        self.list()?.into_iter().map(|v| {
            self.get_saved(v["server_id"].as_str().ok_or(StorageError::CorruptRecord)?)?
                .ok_or(StorageError::CorruptRecord)
        }).collect()
    }
    pub fn list(&self) -> Result<Vec<Value>, StorageError> {
        if !path_present(&self.root)? {
            return Ok(vec![]);
        }
        let root = Directory::open(&self.root, false)?;
        let Some(servers) = root.child("servers", false)? else {
            return Ok(vec![]);
        };
        // Enumerate through the held directory handle; individual opens use openat/O_NOFOLLOW.
        let mut result = vec![];
        for name in servers.names()? {
            let name = name.as_str();
            if name.len() != 64
                || !name
                    .bytes()
                    .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase())
            {
                return Err(StorageError::CorruptRecord);
            }
            let dir = servers
                .child(name, false)?
                .ok_or(StorageError::CorruptRecord)?;
            let Some(bytes) = dir.read("config.json", MAX_RECORD_BYTES)? else {
                continue;
            };
            let value: Value =
                serde_json::from_slice(&bytes).map_err(|_| StorageError::CorruptRecord)?;
            let id = value
                .get("server_id")
                .and_then(Value::as_str)
                .ok_or(StorageError::CorruptRecord)?;
            if server_directory_name(id) != name || !valid_record(id, &value) {
                return Err(StorageError::CorruptRecord);
            }
            result.push(project_record(value));
        }
        result.sort_by(|a, b| a["server_id"].as_str().cmp(&b["server_id"].as_str()));
        Ok(result)
    }
    pub fn get(&self, id: &str) -> Result<Option<Value>, StorageError> {
        let Some(dir) = server_directory(&self.root, id, false)? else {
            return Ok(None);
        };
        let Some(bytes) = dir.read("config.json", MAX_RECORD_BYTES)? else {
            return Ok(None);
        };
        let value: Value =
            serde_json::from_slice(&bytes).map_err(|_| StorageError::CorruptRecord)?;
        if !valid_record(id, &value) {
            return Err(StorageError::CorruptRecord);
        }
        Ok(Some(project_record(value)))
    }
    /// Callers must supply an already normalized non-secret record, never an imported source entry.
    /// This primitive is deliberately not exposed as an HTTP write operation.
    pub fn save(&self, id: &str, record: &Value) -> Result<(), StorageError> {
        if !valid_record(id, record) {
            return Err(StorageError::InvalidRecord);
        }
        let bytes = serde_json::to_vec(record).map_err(|_| StorageError::InvalidRecord)?;
        if bytes.len() > MAX_RECORD_BYTES {
            return Err(StorageError::TooLarge);
        }
        server_directory(&self.root, id, true)?
            .ok_or(StorageError::Unavailable)?
            .write("config.json", &bytes)
    }
}

// Saved status is not live evidence in this process. No disk flag can manufacture authentication.
fn project_record(mut v: Value) -> Value {
    v.as_object_mut().map(|o| o.remove("_installation"));
    v["runtime"] = serde_json::json!({"status":"stopped","error":null});
    v["authentication"] = serde_json::json!({"status":"unknown","error":null});
    v["restart"] = Value::Null;
    v
}
fn keys(v: &Value, allowed: &[&str]) -> bool {
    v.as_object().is_some_and(|o| {
        o.len() == allowed.len() && o.keys().all(|k| allowed.contains(&k.as_str()))
    })
}
fn nonempty(v: &Value) -> bool {
    v.as_str()
        .is_some_and(|s| !s.trim().is_empty() && !s.contains('\0'))
}
fn nullable_string(v: &Value) -> bool {
    v.is_null() || v.is_string()
}
fn state(v: &Value, states: &[&str]) -> bool {
    keys(v, &["status", "error"])
        && v["status"].as_str().is_some_and(|s| states.contains(&s))
        && (v["error"].is_null()
            || (keys(&v["error"], &["code", "message"])
                && v["error"]["code"].is_string()
                && v["error"]["message"].is_string()))
}
fn valid_record(id: &str, v: &Value) -> bool {
    if !v.as_object().is_some_and(|o| o.keys().all(|k| [
            "server_id",
            "name",
            "runtime",
            "authentication",
            "restart_required",
            "restart",
            "config",
            "credentials",
            "_installation",
        ].contains(&k.as_str()))) || id.trim().is_empty()
        || v["server_id"] != id
        || !nonempty(&v["name"])
        || !v["restart_required"].is_boolean()
        || !v["restart"].is_null()
    {
        return false;
    }
    if !state(
        &v["runtime"],
        &[
            "stopped",
            "starting",
            "ready",
            "restarting",
            "failed",
            "unknown",
        ],
    ) || !state(
        &v["authentication"],
        &[
            "not_required",
            "required",
            "authenticating",
            "authenticated",
            "failed",
            "unknown",
        ],
    ) {
        return false;
    }
    let c = &v["config"];
    if !c.as_object().is_some_and(|o| o.keys().all(|k| [
            "kind",
            "package",
            "executable",
            "args",
            "working_directory",
            "environment",
            "connection", "execution", "tool_features",
        ].contains(&k.as_str()))) || !matches!(c["kind"].as_str(), Some("npm" | "pypi" | "local_script" | "remote"))
        || !(if c["kind"]=="remote" {c["executable"].is_null()} else {c["executable"].is_string()})
        || !nullable_string(&c["working_directory"])
        || !c["args"]
            .as_array()
            .is_some_and(|a| a.iter().all(Value::is_string))
    {
        return false;
    }
    if c["kind"] == "local_script" || c["kind"] == "remote" {
        if !c["package"].is_null() {
            return false;
        }
    } else if !keys(
        &c["package"],
        &["name", "requested_version", "installed_version"],
    ) || !nonempty(&c["package"]["name"])
        || !nullable_string(&c["package"]["requested_version"])
        || !nonempty(&c["package"]["installed_version"])
    {
        return false;
    }
    if c["kind"] == "remote" {
        let conn = &c["connection"];
        if !keys(conn, &["url", "transport", "headers"])
            || !matches!(conn["transport"].as_str(), Some("streamable_http" | "sse"))
            || !conn["url"].as_str().is_some_and(|u| url::Url::parse(u).is_ok_and(|u| crate::discovery::supported_remote_endpoint(&u)))
            || !conn["headers"].as_array().is_some_and(|a| a.iter().all(|x| keys(x, &["name", "credential_ref"]) && nonempty(&x["name"]) && nonempty(&x["credential_ref"]))) { return false; }
    }
    if let Some(e) = c.get("execution") {
        if !keys(e, &["max_concurrent_calls", "default_timeout_ms", "tool_timeouts_ms"])
            || !e["max_concurrent_calls"].as_u64().is_some_and(|n| n > 0 && n <= 64)
            || !e["default_timeout_ms"].as_u64().is_some_and(|n| n > 0 && n <= 86_400_000)
            || !e["tool_timeouts_ms"].as_object().is_some_and(|o| o.len()<=4096 && o.iter().all(|(k,v)| !k.is_empty() && v.as_u64().is_some_and(|n| n > 0 && n <= 86_400_000))) { return false; }
    }
    if let Some(i) = v.get("_installation") {
        if !keys(i, &["source_path", "source_name", "installation_id", "protected_args"])
            || !nonempty(&i["source_path"]) || !nonempty(&i["source_name"]) || !nonempty(&i["installation_id"])
            || !i["protected_args"].as_u64().is_some() { return false; }
    }
    if !c["environment"].as_array().is_some_and(|a| {
        a.iter().all(|x| {
            keys(x, &["name", "credential_ref"])
                && nonempty(&x["name"])
                && nonempty(&x["credential_ref"])
        })
    }) {
        return false;
    }
    v["credentials"].as_array().is_some_and(|a| {
        a.iter().all(|x| {
            keys(x, &["name", "status"])
                && nonempty(&x["name"])
                && matches!(x["status"].as_str(), Some("stored" | "missing" | "unknown"))
        })
    })
}

pub(crate) fn server_directory_name(id: &str) -> String {
    format!("{:x}", Sha256::digest(id.as_bytes()))
}
pub(crate) fn server_directory(
    root: &Path,
    id: &str,
    create: bool,
) -> Result<Option<Directory>, StorageError> {
    if !create && !path_present(root)? {
        return Ok(None);
    }
    let root = Directory::open(root, create)?;
    let Some(servers) = root.child("servers", create)? else {
        return Ok(None);
    };
    servers.child(&server_directory_name(id), create)
}

pub(crate) struct Directory {
    file: File,
}
impl Directory {
    fn open(path: &Path, create: bool) -> Result<Self, StorageError> {
        if create && !path_present(path)? {
            std::fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(path)
                .map_err(|_| StorageError::Unavailable)?;
        }
        let c = CString::new(path.as_os_str().as_encoded_bytes())
            .map_err(|_| StorageError::UnsafePath)?;
        // SAFETY: c is a valid NUL-terminated path and open returns an owned fd.
        let fd = unsafe {
            libc::open(
                c.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        Self::from_fd(fd)
    }
    fn from_fd(fd: i32) -> Result<Self, StorageError> {
        if fd < 0 {
            return Err(StorageError::UnsafePath);
        }
        // SAFETY: a successful open/openat fd is uniquely owned here.
        let file = unsafe { File::from_raw_fd(fd) };
        let meta = file.metadata().map_err(|_| StorageError::Unavailable)?;
        if !meta.is_dir() || meta.permissions().mode() & 0o077 != 0 || meta.uid() != effective_uid()
        {
            return Err(StorageError::UnsafePath);
        }
        Ok(Self { file })
    }
    fn names(&self) -> Result<Vec<String>, StorageError> {
        let clone = self
            .file
            .try_clone()
            .map_err(|_| StorageError::Unavailable)?;
        let fd = clone.into_raw_fd();
        // SAFETY: fd is an owned directory descriptor. On success fdopendir owns it.
        let stream = unsafe { libc::fdopendir(fd) };
        if stream.is_null() {
            // SAFETY: fdopendir failed and therefore did not take ownership.
            drop(unsafe { File::from_raw_fd(fd) });
            return Err(StorageError::Unavailable);
        }
        let stream = Entries(stream);
        let mut names = vec![];
        loop {
            // SAFETY: the pointer is this thread's libc errno slot.
            unsafe {
                *errno_slot() = 0;
            }
            // SAFETY: Entries owns an open directory stream until the loop ends.
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                if std::io::Error::last_os_error().raw_os_error() != Some(0) {
                    return Err(StorageError::Unavailable);
                }
                return Ok(names);
            }
            // SAFETY: readdir returned a valid entry; d_name is NUL-terminated and
            // is copied before the next readdir call can invalidate its storage.
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }
                .to_str()
                .map_err(|_| StorageError::CorruptRecord)?
                .to_owned();
            if name != "." && name != ".." {
                names.push(name);
            }
            if names.len() > 4096 {
                return Err(StorageError::TooLarge);
            }
        }
    }
    fn child(&self, name: &str, create: bool) -> Result<Option<Self>, StorageError> {
        let name = c_name(name)?;
        if create {
            // SAFETY: directory fd and name remain valid for this call.
            let result = unsafe { libc::mkdirat(self.file.as_raw_fd(), name.as_ptr(), 0o700) };
            if result < 0
                && std::io::Error::last_os_error().kind() != std::io::ErrorKind::AlreadyExists
            {
                return Err(StorageError::Unavailable);
            }
        }
        // SAFETY: fd/name are valid; no symlink is followed.
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if fd < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::NotFound {
            return Ok(None);
        }
        Self::from_fd(fd).map(Some)
    }
    pub(crate) fn read(&self, name: &str, limit: usize) -> Result<Option<Vec<u8>>, StorageError> {
        let name = c_name(name)?;
        // SAFETY: held fd/name valid; no symlink or FIFO blocking allowed.
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK,
            )
        };
        if fd < 0 {
            return if std::io::Error::last_os_error().kind() == std::io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(StorageError::UnsafePath)
            };
        }
        // SAFETY: successful openat fd ownership transferred exactly once.
        let f = unsafe { File::from_raw_fd(fd) };
        let meta = f.metadata().map_err(|_| StorageError::Unavailable)?;
        if !meta.is_file()
            || meta.permissions().mode() & 0o077 != 0
            || meta.uid() != effective_uid()
            || meta.nlink() != 1
        {
            return Err(StorageError::UnsafePath);
        }
        if meta.len() > limit as u64 {
            return Err(StorageError::TooLarge);
        }
        let mut bytes = vec![];
        f.take(limit as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| StorageError::Unavailable)?;
        if bytes.len() > limit {
            return Err(StorageError::TooLarge);
        }
        Ok(Some(bytes))
    }
    pub(crate) fn write(&self, name: &str, bytes: &[u8]) -> Result<(), StorageError> {
        let name = c_name(name)?;
        // Refuse replacing an unsafe existing destination, including symlinks.
        let name_str = name.to_str().map_err(|_| StorageError::UnsafePath)?;
        let _ = self.read(name_str, MAX_RECORD_BYTES)?;
        let mut random = [0u8; 16];
        OsRng.fill_bytes(&mut random);
        let temporary = c_name(&format!(".write-{:x}", u128::from_be_bytes(random)))?;
        // SAFETY: create a new private file only within the held directory.
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                temporary.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(StorageError::Unavailable);
        }
        // SAFETY: successful openat fd transferred exactly once.
        let mut file = unsafe { File::from_raw_fd(fd) };
        let result = (|| {
            file.write_all(bytes)
                .map_err(|_| StorageError::Unavailable)?;
            file.sync_all().map_err(|_| StorageError::Unavailable)?;
            // SAFETY: rename within one held directory atomically replaces the named entry.
            if unsafe {
                libc::renameat(
                    self.file.as_raw_fd(),
                    temporary.as_ptr(),
                    self.file.as_raw_fd(),
                    name.as_ptr(),
                )
            } < 0
            {
                return Err(StorageError::Unavailable);
            }
            self.file
                .sync_all()
                .map_err(|_| StorageError::Unavailable)?;
            Ok(())
        })();
        // SAFETY: best-effort removal of our random temporary entry, harmless after rename.
        unsafe { libc::unlinkat(self.file.as_raw_fd(), temporary.as_ptr(), 0) };
        result
    }
}
struct Entries(*mut libc::DIR);
impl Drop for Entries {
    fn drop(&mut self) {
        // SAFETY: Entries uniquely owns this successful fdopendir result.
        unsafe {
            libc::closedir(self.0);
        }
    }
}
#[cfg(target_vendor = "apple")]
unsafe fn errno_slot() -> *mut libc::c_int {
    // SAFETY: libc provides this thread's errno storage.
    unsafe { libc::__error() }
}
#[cfg(not(target_vendor = "apple"))]
unsafe fn errno_slot() -> *mut libc::c_int {
    // SAFETY: libc provides this thread's errno storage (Linux developer builds).
    unsafe { libc::__errno_location() }
}
fn path_present(path: &Path) -> Result<bool, StorageError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(StorageError::Unavailable),
    }
}
fn effective_uid() -> u32 {
    // SAFETY: geteuid has no arguments or memory side effects.
    unsafe { libc::geteuid() }
}
fn c_name(name: &str) -> Result<CString, StorageError> {
    if name.is_empty() || name.contains('/') || name == "." || name == ".." {
        return Err(StorageError::UnsafePath);
    }
    CString::new(name).map_err(|_| StorageError::UnsafePath)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn record(id: &str) -> Value {
        json!({"server_id":id,"name":"Example","runtime":{"status":"ready","error":null},"authentication":{"status":"authenticated","error":null},"restart_required":false,"restart":null,"config":{"kind":"local_script","package":null,"executable":"/fixture/node","args":["/fixture/server.js"],"working_directory":null,"environment":[]},"credentials":[]})
    }
    #[test]
    fn opaque_ids_stay_inside_store_and_status_is_not_evidence() {
        let t = tempfile::tempdir().unwrap();
        let repo = ServerRepository::new(t.path().join("state"));
        for id in ["../escape", "normal", "UPPER"] {
            repo.save(id, &record(id)).unwrap();
        }
        let v = repo.list().unwrap();
        assert_eq!(v.len(), 3);
        assert!(
            v.iter().all(|v| v["runtime"]["status"] == "stopped"
                && v["authentication"]["status"] == "unknown")
        );
        assert!(!t.path().join("escape").exists());
    }
    #[test]
    fn corruption_is_not_missing_and_unknown_fields_are_not_saved() {
        let t = tempfile::tempdir().unwrap();
        let repo = ServerRepository::new(t.path().join("state"));
        let mut v = record("x");
        v["secret_token"] = json!("must-not-save");
        assert_eq!(repo.save("x", &v), Err(StorageError::InvalidRecord));
        repo.save("x", &record("x")).unwrap();
        let path = t
            .path()
            .join("state/servers")
            .join(server_directory_name("x"))
            .join("config.json");
        std::fs::write(path, b"broken").unwrap();
        assert!(repo.get("x").is_err());
        assert!(repo.list().is_err());
    }
    #[test]
    fn unsafe_symlinks_and_permissions_are_rejected() {
        use std::os::unix::fs::symlink;
        let t = tempfile::tempdir().unwrap();
        let repo = ServerRepository::new(t.path().join("state"));
        repo.save("x", &record("x")).unwrap();
        let path = t
            .path()
            .join("state/servers")
            .join(server_directory_name("x"))
            .join("config.json");
        std::fs::remove_file(&path).unwrap();
        let external = t.path().join("external");
        std::fs::write(&external, b"keep").unwrap();
        symlink(&external, &path).unwrap();
        assert!(repo.get("x").is_err());
        assert!(repo.save("x", &record("x")).is_err());
        assert_eq!(std::fs::read(external).unwrap(), b"keep");
    }
    #[test]
    fn unsafe_modes_hardlinks_and_dangling_roots_fail_closed() {
        use std::os::unix::fs::symlink;
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("state");
        let repo = ServerRepository::new(root.clone());
        repo.save("x", &record("x")).unwrap();
        let file = root
            .join("servers")
            .join(server_directory_name("x"))
            .join("config.json");
        assert_eq!(
            std::fs::metadata(&root).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            std::fs::metadata(&file).unwrap().permissions().mode() & 0o777,
            0o600
        );
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(repo.get("x").is_err());
        assert!(repo.save("x", &record("x")).is_err());
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        std::fs::hard_link(&file, t.path().join("linked")).unwrap();
        assert!(repo.get("x").is_err());
        let dangling = t.path().join("dangling");
        symlink(t.path().join("missing"), &dangling).unwrap();
        let broken = ServerRepository::new(dangling);
        assert!(broken.list().is_err());
        assert!(broken.get("x").is_err());
    }
}
