//! A native-only transport wrapper around portable-pty. Terminal bytes stay in memory.
use base64::{engine::general_purpose::STANDARD, Engine};
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize};
use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};
use serde_json::{json, Value};
use std::{collections::{BTreeMap, VecDeque}, io::{Read, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

const OUTPUT_LIMIT: usize = 1024 * 1024;
const OUTPUT_CHUNK: usize = 64 * 1024;
pub const INPUT_LIMIT: usize = 64 * 1024;

#[derive(Debug)]
pub struct TerminalError { pub http_status: u16, pub code: &'static str, pub message: String }
impl TerminalError {
    pub fn new(http_status: u16, code: &'static str, message: impl Into<String>) -> Self { Self { http_status, code, message: message.into() } }
    pub fn invalid() -> Self { Self::new(400, "INVALID_REQUEST", "Use the documented terminal fields, byte encoding and dimensions.") }
    pub fn body(&self) -> Value { json!({"status":"failed","error":{"code":self.code,"message":self.message},"next_steps":[]}) }
    fn unavailable(error: impl std::fmt::Display) -> Self { Self::new(409, "TERMINAL_UNAVAILABLE", format!("Terminal IO failed: {error}")) }
    fn start(error: impl std::fmt::Display) -> Self { Self::new(503, "TERMINAL_START_FAILED", format!("The terminal could not start: {error}")) }
}
impl From<crate::manager::ManagementError> for TerminalError {
    fn from(value: crate::manager::ManagementError) -> Self { Self::new(value.http_status,value.code,value.message) }
}

pub enum Action {
    Open { columns: u16, rows: u16 },
    Input { session_id: String, data: Vec<u8> },
    Resize { session_id: String, columns: u16, rows: u16 },
    Close { session_id: String },
}
impl Action {
    pub fn parse(value: Value) -> Result<Self, TerminalError> {
        let object=value.as_object().ok_or_else(TerminalError::invalid)?;
        let action=object.get("action").and_then(Value::as_str).ok_or_else(TerminalError::invalid)?;
        let fields: &[&str]=match action { "open"=>&["action","columns","rows"], "input"=>&["action","session_id","data"], "resize"=>&["action","session_id","columns","rows"], "close"=>&["action","session_id"], _=>return Err(TerminalError::invalid()) };
        if object.keys().any(|name|!fields.contains(&name.as_str())) {return Err(TerminalError::invalid());}
        let dimension=|name:&str,default:Option<u16>|->Result<u16,TerminalError>{
            match object.get(name) { None=>default.ok_or_else(TerminalError::invalid), Some(value)=>value.as_u64().filter(|v|(1..=1000).contains(v)).map(|v|v as u16).ok_or_else(TerminalError::invalid) }
        };
        let session=||object.get("session_id").and_then(Value::as_str).filter(|v|!v.is_empty()&&!v.contains('\0')).map(str::to_owned).ok_or_else(TerminalError::invalid);
        Ok(match action {
            "open"=>Self::Open{columns:dimension("columns",Some(80))?,rows:dimension("rows",Some(24))?},
            "resize"=>Self::Resize{session_id:session()?,columns:dimension("columns",None)?,rows:dimension("rows",None)?},
            "close"=>Self::Close{session_id:session()?},
            "input"=>{
                let encoded=object.get("data").and_then(Value::as_str).filter(|v|v.len()<=4*INPUT_LIMIT.div_ceil(3)).ok_or_else(TerminalError::invalid)?;
                let data=STANDARD.decode(encoded).map_err(|_|TerminalError::invalid())?;
                if data.len()>INPUT_LIMIT {return Err(TerminalError::invalid());}
                Self::Input{session_id:session()?,data}
            },
            _=>return Err(TerminalError::invalid()),
        })
    }
}

pub struct Launch { pub shell: PathBuf, pub working_directory: PathBuf, pub environment: BTreeMap<String,String>, #[cfg(test)] pub args: Vec<String> }
impl Launch {
    pub fn system(working_directory: PathBuf,environment: BTreeMap<String,String>) -> Result<Self,TerminalError> {
        Ok(Self{shell:system_shell()?,working_directory,environment,#[cfg(test)]args:vec![]})
    }
}

#[derive(Default)]
struct Output { bytes: VecDeque<u8>, first: u64, end: u64, status: &'static str, reader_done:bool, exit_code: Option<u32>, signal: Option<String> }
impl Output {
    fn append(&mut self,bytes:&[u8]) {
        self.end+=bytes.len() as u64;
        if bytes.len()>=OUTPUT_LIMIT {self.bytes.clear();self.bytes.extend(&bytes[bytes.len()-OUTPUT_LIMIT..]);}
        else {let discard=(self.bytes.len()+bytes.len()).saturating_sub(OUTPUT_LIMIT);self.bytes.drain(..discard);self.bytes.extend(bytes);}
        self.first=self.end-self.bytes.len() as u64;
    }
    fn exited(&mut self,status:portable_pty::ExitStatus) {
        if self.status!="closed"&&self.reader_done {self.status="exited";}
        self.exit_code=Some(status.exit_code());self.signal=status.signal().map(str::to_owned);
    }
    fn finish_read(&mut self){self.reader_done=true;if self.status!="closed"&&self.exit_code.is_some(){self.status="exited";}}
}
struct Input { writer:Box<dyn Write+Send>, readiness:OwnedFd }
struct Session {
    id:String, server:String, shell:PathBuf, cwd:PathBuf,
    output:Arc<Mutex<Output>>, master:Mutex<Option<Box<dyn MasterPty+Send>>>, writer:Mutex<Option<Input>>, child:Mutex<Box<dyn Child+Send+Sync>>, close_guard:Mutex<()>,
    closed:Arc<AtomicBool>, reader:Mutex<Option<thread::JoinHandle<()>>>, waiter:Mutex<Option<thread::JoinHandle<()>>>,
}
impl Session {
    fn spawn(server:&str,launch:Launch,columns:u16,rows:u16) -> Result<Arc<Self>,TerminalError> {
        let pair=portable_pty::native_pty_system().openpty(size(columns,rows)).map_err(TerminalError::start)?;
        let mut command=CommandBuilder::new(&launch.shell);
        command.env_clear();command.cwd(&launch.working_directory);
        for (name,value) in launch.environment {command.env(name,value);}
        command.env("TERM","xterm-256color");command.env("SHELL",&launch.shell);
        #[cfg(test)] command.args(&launch.args);
        let reader=pair.master.try_clone_reader().map_err(TerminalError::start)?;
        let writer=pair.master.take_writer().map_err(TerminalError::start)?;
        let descriptor=pair.master.as_raw_fd().ok_or_else(||TerminalError::start("The PTY has no native file descriptor."))?;
        // The PTY clones share nonblocking state. Read/write polling lets close
        // interrupt pending IO even when another process still holds the slave.
        let flags=unsafe{libc::fcntl(descriptor,libc::F_GETFL)};
        if flags<0||unsafe{libc::fcntl(descriptor,libc::F_SETFL,flags|libc::O_NONBLOCK)}<0{return Err(TerminalError::start(std::io::Error::last_os_error()));}
        let read_ready=unsafe{BorrowedFd::borrow_raw(descriptor)}.try_clone_to_owned().map_err(TerminalError::start)?;
        let write_ready=unsafe{BorrowedFd::borrow_raw(descriptor)}.try_clone_to_owned().map_err(TerminalError::start)?;
        let child=pair.slave.spawn_command(command).map_err(TerminalError::start)?;
        drop(pair.slave);
        let output=Arc::new(Mutex::new(Output{status:"running",..Output::default()}));
        let closed=Arc::new(AtomicBool::new(false));
        let session=Arc::new(Self{id:crate::installation::opaque("terminal"),server:server.into(),shell:launch.shell,cwd:launch.working_directory,output:output.clone(),master:Mutex::new(Some(pair.master)),writer:Mutex::new(Some(Input{writer,readiness:write_ready})),child:Mutex::new(child),close_guard:Mutex::new(()),closed:closed.clone(),reader:Mutex::new(None),waiter:Mutex::new(None)});
        let read_thread=thread::Builder::new().name("fetch-terminal-output".into()).spawn(move||read_output(reader,read_ready,output,closed));
        match read_thread {Ok(handle)=>*session.reader.lock().unwrap()=Some(handle),Err(error)=>{let _=session.close();return Err(TerminalError::start(error));}}
        // A weak reference avoids a thread keeping an abandoned service alive.
        let weak=Arc::downgrade(&session);
        let wait_thread=thread::Builder::new().name("fetch-terminal-status".into()).spawn(move||loop {
            let Some(session)=weak.upgrade() else{break};
            if session.closed.load(Ordering::SeqCst) {break;}
            let result=session.child.lock().unwrap().try_wait();
            match result {Ok(Some(status))=>{session.output.lock().unwrap().exited(status);break;},Ok(None)=>{},Err(_)=>break}
            drop(session);thread::sleep(Duration::from_millis(25));
        });
        match wait_thread {Ok(handle)=>*session.waiter.lock().unwrap()=Some(handle),Err(error)=>{let _=session.close();return Err(TerminalError::start(error));}}
        Ok(session)
    }
    fn summary(&self,include_paths:bool) -> Value {
        let output=self.output.lock().unwrap();
        let mut value=json!({"server_id":self.server,"session_id":self.id,"status":output.status,"output_cursor":output.end,"exit_code":output.exit_code,"signal":output.signal,"next_steps":[]});
        if include_paths {value["shell"]=json!(self.shell);value["working_directory"]=json!(self.cwd);}
        value
    }
    fn running(&self)->bool {self.output.lock().unwrap().status=="running"&&!self.closed.load(Ordering::SeqCst)}
    fn read(&self,after:u64)->Result<Value,TerminalError> {
        let output=self.output.lock().unwrap();
        if after>output.end {return Err(TerminalError::invalid());}
        let start=after.max(output.first);let offset=(start-output.first) as usize;
        let bytes:Vec<u8>=output.bytes.iter().skip(offset).take(OUTPUT_CHUNK).copied().collect();
        Ok(json!({"server_id":self.server,"session_id":self.id,"status":output.status,"data":STANDARD.encode(&bytes),"next_cursor":start+bytes.len() as u64,"first_available_cursor":output.first,"gap":after<output.first,"exit_code":output.exit_code,"signal":output.signal,"next_steps":[]}))
    }
    fn input(&self,data:&[u8])->Result<Value,TerminalError> {
        if !self.running() {return Err(TerminalError::unavailable("The shell has exited or was closed."));}
        let mut input=self.writer.lock().unwrap();let input=input.as_mut().ok_or_else(||TerminalError::unavailable("The terminal input is closed."))?;
        let mut remaining=data;
        while !remaining.is_empty(){
            if self.closed.load(Ordering::SeqCst){return Err(TerminalError::unavailable("The terminal input is closed."));}
            match input.writer.write(remaining){
                Ok(0)=>return Err(TerminalError::unavailable("The PTY accepted no input bytes.")),
                Ok(count)=>remaining=&remaining[count..],
                Err(error) if error.kind()==std::io::ErrorKind::Interrupted=>continue,
                Err(error) if error.kind()==std::io::ErrorKind::WouldBlock=>{wait_ready(&input.readiness,libc::POLLOUT).map_err(TerminalError::unavailable)?;},
                Err(error)=>return Err(TerminalError::unavailable(error)),
            }
        }
        input.writer.flush().map_err(TerminalError::unavailable)?;
        Ok(self.summary(false))
    }
    fn resize(&self,columns:u16,rows:u16)->Result<Value,TerminalError> {
        if !self.running() {return Err(TerminalError::unavailable("The shell has exited or was closed."));}
        self.master.lock().unwrap().as_ref().ok_or_else(||TerminalError::unavailable("The terminal is closed."))?.resize(size(columns,rows)).map_err(TerminalError::unavailable)?;
        Ok(self.summary(false))
    }
    fn close(&self)->Result<Value,TerminalError> {
        let _close=self.close_guard.lock().unwrap();
        if self.closed.load(Ordering::SeqCst) {return Ok(self.summary(false));}
        let mut child=self.child.lock().unwrap();
        match child.try_wait().map_err(TerminalError::unavailable)? {
            Some(status)=>self.output.lock().unwrap().exited(status),
            None=>{
                // portable-pty starts a new session with its own controlling PTY.
                // Only the child's group and that PTY's foreground group are ours.
                let shell=child.process_id().map(|v|v as libc::pid_t).filter(|v|*v>1);
                let foreground=||self.master.lock().unwrap().as_ref().and_then(|master|master.process_group_leader()).filter(|v|*v>1);
                let mut status=None;
                if let Some(group)=foreground().filter(|group|Some(*group)!=shell){
                    if let Err(error)=signal_group(group,libc::SIGHUP){
                        if foreground()==Some(group){return Err(TerminalError::unavailable(error));}
                    }
                }
                if let Some(group)=shell{
                    if let Err(error)=signal_group(group,libc::SIGHUP){
                        status=child.try_wait().map_err(TerminalError::unavailable)?;
                        if status.is_none(){return Err(TerminalError::unavailable(error));}
                    }
                }
                if status.is_none(){thread::sleep(Duration::from_millis(50));}
                // SIGHUP commonly finishes the shell. Reap it before deciding
                // whether another signal is necessary; a reaped PID/group must
                // never be used as a target for a later fallback signal.
                if status.is_none(){status=child.try_wait().map_err(TerminalError::unavailable)?;}
                if let Some(group)=foreground().filter(|group|Some(*group)!=shell){
                    if let Err(error)=signal_group(group,libc::SIGKILL){
                        if foreground()==Some(group){return Err(TerminalError::unavailable(error));}
                    }
                }
                if status.is_none(){
                    if let Some(group)=shell{
                        if let Err(error)=signal_group(group,libc::SIGKILL){
                            status=child.try_wait().map_err(TerminalError::unavailable)?;
                            if status.is_none(){return Err(TerminalError::unavailable(error));}
                        }
                    }
                    if status.is_none(){
                        // A process may exit between the check and portable-pty's
                        // fallback kill. A confirmed exit is success, even when
                        // macOS rejects the now-dead process signal with EPERM.
                        if let Err(error)=child.kill(){
                            status=child.try_wait().map_err(TerminalError::unavailable)?;
                            if status.is_none(){return Err(TerminalError::unavailable(error));}
                        }
                    }
                }
                let status=match status{Some(status)=>status,None=>child.wait().map_err(TerminalError::unavailable)?};
                self.output.lock().unwrap().exited(status);
            }
        }
        drop(child);
        self.closed.store(true,Ordering::SeqCst);
        self.output.lock().unwrap().status="closed";
        // Closing is observable by the bounded IO polls before taking their
        // locks. Join both workers and release every descriptor we own.
        if let Some(reader)=self.reader.lock().unwrap().take(){let _=reader.join();}
        self.writer.lock().unwrap().take();self.master.lock().unwrap().take();
        if let Some(waiter)=self.waiter.lock().unwrap().take(){let _=waiter.join();}
        Ok(self.summary(false))
    }
}
fn signal_group(group:libc::pid_t,signal:libc::c_int)->std::io::Result<()> {
    if unsafe{libc::kill(-group,signal)}==0{return Ok(());}
    let error=std::io::Error::last_os_error();
    if error.raw_os_error()==Some(libc::ESRCH){Ok(())}else{Err(error)}
}
fn size(columns:u16,rows:u16)->PtySize {PtySize{rows,cols:columns,pixel_width:0,pixel_height:0}}
fn wait_ready(descriptor:&OwnedFd,event:libc::c_short)->std::io::Result<bool>{
    let mut poll=libc::pollfd{fd:descriptor.as_raw_fd(),events:event,revents:0};
    let ready=unsafe{libc::poll(&mut poll,1,50)};
    if ready<0 {let error=std::io::Error::last_os_error();if error.kind()==std::io::ErrorKind::Interrupted{return Ok(false);}return Err(error);}
    Ok(ready>0)
}
fn read_output(mut reader:Box<dyn Read+Send>,readiness:OwnedFd,output:Arc<Mutex<Output>>,closed:Arc<AtomicBool>) {
    let mut bytes=[0u8;8192];
    while !closed.load(Ordering::SeqCst){
        match wait_ready(&readiness,libc::POLLIN){Ok(false) if output.lock().unwrap().exit_code.is_none()=>continue,Err(_)=>break,Ok(_)=>{}}
        match reader.read(&mut bytes){
            Ok(0)=>break,Ok(count)=>output.lock().unwrap().append(&bytes[..count]),
            Err(error) if error.kind()==std::io::ErrorKind::Interrupted=>continue,
            Err(error) if error.kind()==std::io::ErrorKind::WouldBlock=>{if output.lock().unwrap().exit_code.is_some(){break;}continue;},Err(_)=>break,
        }
    }
    output.lock().unwrap().finish_read();
}

#[derive(Default)]
pub struct TerminalService {sessions:Mutex<BTreeMap<String,Arc<Session>>>,stopping:AtomicBool}
impl TerminalService {
    pub fn new()->Arc<Self> {Arc::new(Self::default())}
    pub fn existing(&self,server:&str)->Option<Value> {self.sessions.lock().unwrap().get(server).filter(|session|session.running()).map(|session|session.summary(true))}
    pub fn open(&self,server:&str,launch:Launch,columns:u16,rows:u16)->Result<(Value,bool),TerminalError> {
        if !(1..=1000).contains(&columns)||!(1..=1000).contains(&rows){return Err(TerminalError::invalid());}
        let mut sessions=self.sessions.lock().unwrap();
        if self.stopping.load(Ordering::SeqCst){return Err(TerminalError::new(503,"TERMINAL_START_FAILED","Fetch is shutting down."));}
        if let Some(session)=sessions.get(server) {if session.running(){return Ok((session.summary(true),false));}session.close()?;}
        let session=Session::spawn(server,launch,columns,rows)?;let value=session.summary(true);sessions.insert(server.to_owned(),session);Ok((value,true))
    }
    fn session(&self,server:&str,id:&str)->Result<Arc<Session>,TerminalError> {self.sessions.lock().unwrap().get(server).filter(|s|s.id==id).cloned().ok_or_else(||TerminalError::new(404,"TERMINAL_NOT_FOUND","The selected terminal session was not found."))}
    pub fn apply(&self,server:&str,action:Action)->Result<Value,TerminalError> {match action {
        Action::Input{session_id,data}=>self.session(server,&session_id)?.input(&data),
        Action::Resize{session_id,columns,rows}=>self.session(server,&session_id)?.resize(columns,rows),
        Action::Close{session_id}=>self.session(server,&session_id)?.close(),
        Action::Open{..}=>Err(TerminalError::invalid()),
    }}
    pub fn read(&self,server:&str,id:&str,after:u64)->Result<Value,TerminalError> {self.session(server,id)?.read(after)}
    pub fn remove(&self,server:&str)->Result<(),TerminalError> {let mut sessions=self.sessions.lock().unwrap();if let Some(session)=sessions.get(server){session.close()?;}sessions.remove(server);Ok(())}
    pub fn shutdown(&self)->Result<(),TerminalError> {self.stopping.store(true,Ordering::SeqCst);let servers:Vec<_>=self.sessions.lock().unwrap().keys().cloned().collect();let mut failure=None;for server in servers{if let Err(error)=self.remove(&server){failure=Some(error);}}match failure{Some(error)=>Err(error),None=>Ok(())}}
}
impl Drop for TerminalService {fn drop(&mut self){let _=self.shutdown();}}

fn system_shell()->Result<PathBuf,TerminalError> {
    use std::{ffi::CStr,os::unix::{ffi::OsStrExt,fs::PermissionsExt}};
    let unavailable=|message:String|TerminalError::new(503,"TERMINAL_SHELL_UNAVAILABLE",message);
    let mut storage=vec![0u8;16*1024];let mut account=std::mem::MaybeUninit::<libc::passwd>::uninit();let mut result=std::ptr::null_mut();
    let code=unsafe{libc::getpwuid_r(libc::geteuid(),account.as_mut_ptr(),storage.as_mut_ptr().cast(),storage.len(),&mut result)};
    if code!=0 {return Err(unavailable(format!("The account shell could not be read: {}",std::io::Error::from_raw_os_error(code))));}
    if result.is_null(){return Err(unavailable("The current account has no system shell record.".into()));}
    let account=unsafe{account.assume_init()};if account.pw_shell.is_null(){return Err(unavailable("The current account has no configured system shell.".into()));}
    let bytes=unsafe{CStr::from_ptr(account.pw_shell)}.to_bytes();let shell=Path::new(std::ffi::OsStr::from_bytes(bytes)).to_owned();
    let metadata=std::fs::metadata(&shell).map_err(|error|unavailable(format!("The configured account shell is unavailable: {error}")))?;
    if !shell.is_absolute()||!metadata.is_file()||metadata.permissions().mode()&0o111==0 {return Err(unavailable("The configured account shell is not an executable absolute file.".into()));}
    Ok(shell)
}

#[cfg(test)]
mod tests;
