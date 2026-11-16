//! Read original session files with bounded metadata and bounded UTF-8 chunks.
use super::{App, DEFAULT_CHUNK_BYTES, MAX_CHUNK_BYTES, WorkspaceError, WorkspaceService, absolute_path, success};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::{BufRead,BufReader,Read,Seek,SeekFrom},path::{Path,PathBuf},time::UNIX_EPOCH};

const METADATA_WINDOW: usize=64*1024;
#[derive(Clone)]
pub(super) struct Session {
    pub(super) id:String,pub(super) name:String,pub(super) app:App,pub(super) file_path:PathBuf,pub(super) project_path:Option<PathBuf>,pub(super) updated_at:u64,
}
impl Session {
    pub(super) fn value(&self)->Value {
        json!({"id":self.id,"name":self.name,"app":self.app.as_str(),"file_path":self.file_path,"project_path":self.project_path,"updated_at":self.updated_at,"source_format":source_format(&self.file_path)})
    }
}

fn compressed(path:&Path)->bool{
    path.file_name().and_then(|name|name.to_str()).is_some_and(|name|name.ends_with(".jsonl.zst"))
}
fn source_format(path:&Path)->Option<&'static str>{
    if compressed(path){Some("jsonl.zst")}
    else if path.extension().and_then(|extension|extension.to_str())==Some("jsonl"){Some("jsonl")}
    else{None}
}

pub(super) fn roots(service:&WorkspaceService,app:App)->Vec<PathBuf>{
    match app {App::Claude=>vec![service.claude_root.join("projects")],App::Codex=>vec![service.codex_root.join("sessions"),service.codex_root.join("archived_sessions")]}
}

fn collect(directory:&Path,root:&Path,files:&mut Vec<PathBuf>)->Result<(),WorkspaceError>{
    if !directory.exists(){return Ok(());}
    let mut entries=fs::read_dir(directory).map_err(WorkspaceError::io)?.collect::<Result<Vec<_>,_>>().map_err(WorkspaceError::io)?;
    entries.sort_by_key(|entry|entry.file_name());
    for entry in entries {
        let kind=entry.file_type().map_err(WorkspaceError::io)?;
        let path=entry.path();
        if kind.is_dir(){collect(&path,root,files)?;}
        else if (kind.is_file()||kind.is_symlink())&&source_format(&path).is_some() {
            if path.canonicalize().is_ok_and(|path|path.starts_with(root)&&path.is_file()){files.push(path);}
        }
    }
    Ok(())
}

/// Head/tail windows are enough for recorded cwd, session IDs and title events.
/// In particular this never allocates a whole huge JSONL record to list a name.
fn metadata_records(path:&Path)->Result<Vec<Value>,WorkspaceError>{
    let mut file=fs::File::open(path).map_err(WorkspaceError::io)?;
    if compressed(path){
        // Finding a compressed tail would require decoding the entire log.
        // Read only the bounded prefix, then use the native title index where
        // available. No decompressed file or cache is written.
        let decoder=zstd::stream::read::Decoder::new(file).map_err(WorkspaceError::io)?;
        let mut head=Vec::with_capacity(METADATA_WINDOW);
        decoder.take(METADATA_WINDOW as u64).read_to_end(&mut head).map_err(WorkspaceError::io)?;
        let mut records=Vec::new();parse_lines(&head,false,false,&mut records);
        return Ok(records);
    }
    let size=file.metadata().map_err(WorkspaceError::io)?.len();
    let mut head=vec![0;METADATA_WINDOW.min(size as usize)];file.read_exact(&mut head).map_err(WorkspaceError::io)?;
    let mut records=Vec::new();
    parse_lines(&head,false,(head.len() as u64)<size,&mut records);
    if size>head.len() as u64 {
        let start=(head.len() as u64).max(size.saturating_sub(METADATA_WINDOW as u64));
        file.seek(SeekFrom::Start(start)).map_err(WorkspaceError::io)?;
        let mut tail=Vec::with_capacity(METADATA_WINDOW);file.take(METADATA_WINDOW as u64).read_to_end(&mut tail).map_err(WorkspaceError::io)?;
        parse_lines(&tail,start>head.len() as u64 || head.last()!=Some(&b'\n'),false,&mut records);
    }
    Ok(records)
}
fn parse_lines(bytes:&[u8],skip_first:bool,skip_last:bool,output:&mut Vec<Value>){
    let lines:Vec<_>=bytes.split(|byte|*byte==b'\n').collect();
    for (index,line) in lines.iter().enumerate(){
        if (skip_first&&index==0)||(skip_last&&index+1==lines.len()){continue;}
        if let Ok(value)=serde_json::from_slice::<Value>(line){output.push(value);}
    }
}

fn text_content(value:&Value)->Option<String>{
    if let Some(text)=value.as_str(){return Some(text.to_owned());}
    value.as_array().map(|blocks|blocks.iter().filter_map(|block|block.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join("\n")).filter(|text|!text.is_empty())
}
fn short_title(text:&str)->Option<String>{
    let line=text.lines().find(|line|!line.trim().is_empty())?.trim();
    if line.starts_with("# AGENTS.md instructions")||line.starts_with("<environment_context>")||line.starts_with("<INSTRUCTIONS>"){return None;}
    Some(line.chars().take(160).collect())
}
fn canonical_project(value:&str)->Option<PathBuf>{
    let path=PathBuf::from(value);if !path.is_absolute(){return None;}
    Some(path.canonicalize().unwrap_or(path))
}
fn metadata(app:App,path:&Path,titles:&BTreeMap<String,String>)->Result<Session,WorkspaceError>{
    let records=metadata_records(path)?;
    let mut project=None;let mut title=None;let mut custom_title=None;let mut ai_title=None;let mut last_prompt=None;let mut summary=None;let mut prompt=None;let mut native_id=None;
    for record in &records {
        let payload=record.get("payload").unwrap_or(record);
        if project.is_none(){project=payload.get("cwd").or_else(||record.get("cwd")).and_then(Value::as_str).and_then(canonical_project);}
        if native_id.is_none(){native_id=if record["type"]=="session_meta"{payload.get("id")}else{record.get("sessionId")}.and_then(Value::as_str).map(str::to_owned);}
        for key in ["thread_name","title"] {
            if let Some(value)=record.get(key).or_else(||payload.get(key)).and_then(Value::as_str).filter(|value|!value.trim().is_empty()){title=Some(value.to_owned());}
        }
        for (key,target) in [("customTitle",&mut custom_title),("aiTitle",&mut ai_title),("lastPrompt",&mut last_prompt)] {
            if let Some(value)=record.get(key).or_else(||payload.get(key)).and_then(Value::as_str).filter(|value|!value.trim().is_empty()){*target=Some(value.to_owned());}
        }
        if record["type"]=="summary" {if let Some(value)=record["summary"].as_str().filter(|value|!value.trim().is_empty()){summary=Some(value.to_owned());}}
        if prompt.is_none(){
            let message=record.get("message").unwrap_or(payload);
            let user=message["role"]=="user"||record["type"]=="user";
            if user {prompt=message.get("content").and_then(text_content).and_then(|text|short_title(&text));}
        }
    }
    let indexed_title=native_id.as_ref().and_then(|id|titles.get(id)).cloned();
    let id=format!("{}-{:x}",app.as_str(),Sha256::digest(path.to_string_lossy().as_bytes()));
    let name=match app {
        App::Claude=>custom_title.or(ai_title).or(last_prompt).or(summary).or(title).or(prompt),
        App::Codex=>indexed_title.or(title).or(custom_title).or(ai_title).or(last_prompt).or(summary).or(prompt),
    }.unwrap_or_else(||native_id.unwrap_or_else(||path.file_stem().unwrap_or_default().to_string_lossy().into_owned()));
    let modified=fs::metadata(path).map_err(WorkspaceError::io)?.modified().ok().and_then(|time|time.duration_since(UNIX_EPOCH).ok()).map(|time|time.as_millis().min(u64::MAX as u128) as u64).unwrap_or(0);
    Ok(Session{id,name,app,file_path:path.to_path_buf(),project_path:project,updated_at:modified})
}
fn native_titles(service:&WorkspaceService)->BTreeMap<String,String>{
    let mut titles=BTreeMap::new();
    if let Ok(file)=fs::File::open(service.codex_root.join("session_index.jsonl")) {
        // The index is lightweight metadata, so visit every entry rather than
        // losing names in its middle. Per-record memory remains bounded even
        // if an index line is unexpectedly oversized.
        let _=visit_index_records(file,|record|{
            if let (Some(id),Some(name))=(record.get("id").or_else(||record.get("thread_id")).and_then(Value::as_str),record.get("thread_name").or_else(||record.get("title")).or_else(||record.get("name")).and_then(Value::as_str)) {titles.insert(id.to_owned(),name.to_owned());}
        });
    }
    titles
}

fn visit_index_records(file:fs::File,mut visit:impl FnMut(Value))->std::io::Result<()>{
    let mut reader=BufReader::new(file);let mut line=Vec::new();let mut oversized=false;
    loop {
        let buffer=reader.fill_buf()?;
        if buffer.is_empty(){
            if !oversized&&!line.is_empty(){if let Ok(record)=serde_json::from_slice(&line){visit(record);}}
            break;
        }
        let newline=buffer.iter().position(|byte|*byte==b'\n');
        let length=newline.map_or(buffer.len(),|index|index+1);
        if !oversized {
            if length>METADATA_WINDOW.saturating_sub(line.len()){oversized=true;line.clear();}
            else{line.extend_from_slice(&buffer[..length]);}
        }
        reader.consume(length);
        if newline.is_some(){
            if !oversized{if let Ok(record)=serde_json::from_slice(&line){visit(record);}}
            line.clear();oversized=false;
        }
    }
    Ok(())
}
fn visible(session:&Session,folder:Option<&Path>)->bool{
    folder.is_none_or(|folder|session.project_path.as_ref().is_some_and(|project|project.starts_with(folder)))
}
pub(super) fn all(service:&WorkspaceService,folder:Option<&Path>,app:Option<App>)->Result<Vec<Session>,WorkspaceError>{
    let titles=native_titles(service);let mut sessions=Vec::new();
    for source in [App::Claude,App::Codex] {
        if app.is_some_and(|app|app!=source){continue;}
        for root in roots(service,source) {
            if !root.exists(){continue;}
            let canonical=root.canonicalize().map_err(WorkspaceError::io)?;
            let mut files=Vec::new();collect(&root,&canonical,&mut files)?;
            for path in files {
                match metadata(source,&path,&titles) {
                    Ok(session) if visible(&session,folder)=>sessions.push(session),
                    Ok(_)=>{},
                    // A session removed during discovery should not erase the
                    // remaining list. Actual access/read failures are returned.
                    Err(error) if error.http_status==404=>{},
                    Err(error)=>return Err(error),
                }
            }
        }
    }
    sessions.sort_by(|left,right|right.updated_at.cmp(&left.updated_at).then(left.id.cmp(&right.id)));
    Ok(sessions)
}

pub(super) fn list(service:&WorkspaceService,folder_path:Option<&str>,app:Option<&str>,cursor:Option<&str>,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let folder=service.folder(folder_path)?;
    let app=app.map(App::parse).transpose()?;
    let offset=cursor.map(|value|value.parse::<usize>().map_err(|_|WorkspaceError::new(400,"INVALID_SESSION_CURSOR","Use the next_cursor returned by the session list."))).transpose()?.unwrap_or(0);
    let limit=limit.unwrap_or(50);if !(1..=200).contains(&limit){return Err(WorkspaceError::new(400,"INVALID_SESSION_LIMIT","Read between 1 and 200 session names at a time."));}
    let sessions=all(service,folder.as_deref(),app)?;
    let page:Vec<Value>=sessions.iter().skip(offset).take(limit).map(Session::value).collect();
    let end=offset.saturating_add(page.len());
    Ok(success(json!({"folder_path":folder,"sessions":page,"next_cursor":if end<sessions.len(){Some(end.to_string())}else{None}})))
}

pub(super) fn selected(service:&WorkspaceService,folder_path:Option<&str>,file_path:&str)->Result<Session,WorkspaceError>{
    let folder=service.folder(folder_path)?;let path=absolute_path(file_path)?;
    let canonical=path.canonicalize().map_err(WorkspaceError::io)?;
    if source_format(&canonical).is_none()||!canonical.is_file(){return Err(WorkspaceError::new(403,"SESSION_PATH_FORBIDDEN","Choose a recorded session log."));}
    let mut app=None;
    for source in [App::Claude,App::Codex] {
        for root in roots(service,source) {
            if root.canonicalize().is_ok_and(|root|canonical.starts_with(root)){app=Some(source);break;}
        }
    }
    let app=app.ok_or_else(||WorkspaceError::new(403,"SESSION_PATH_FORBIDDEN","This file is outside the coding apps' session folders."))?;
    let session=metadata(app,&path,&native_titles(service))?;
    if !visible(&session,folder.as_deref()){return Err(WorkspaceError::new(403,"SESSION_PROJECT_FORBIDDEN","This session does not belong to the selected project."));}
    Ok(session)
}

pub(super) fn read(service:&WorkspaceService,folder_path:Option<&str>,file_path:&str,offset:u64,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let session=selected(service,folder_path,file_path)?;
    chunk(session,offset,limit)
}
pub(super) fn read_by_id(service:&WorkspaceService,folder_path:Option<&str>,id:&str,offset:u64,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let folder=service.folder(folder_path)?;
    let session=all(service,folder.as_deref(),None)?.into_iter().find(|session|session.id==id).ok_or_else(||WorkspaceError::new(404,"SESSION_NOT_FOUND","This session was not found in the selected project."))?;
    // Recheck the selected path after lookup before opening its content.
    read(service,folder_path,&session.file_path.to_string_lossy(),offset,limit)
}

fn chunk(session:Session,offset:u64,limit:Option<usize>)->Result<Value,WorkspaceError>{
    let limit=limit.unwrap_or(DEFAULT_CHUNK_BYTES);
    if !(4..=MAX_CHUNK_BYTES).contains(&limit){return Err(WorkspaceError::new(400,"INVALID_SESSION_LIMIT","Read between 4 and 65536 bytes at a time."));}
    let mut file=fs::File::open(&session.file_path).map_err(WorkspaceError::io)?;
    let mut bytes=Vec::with_capacity(limit+1);
    if compressed(&session.file_path){
        let mut decoder=zstd::stream::read::Decoder::new(file).map_err(WorkspaceError::io)?;
        // zstd archives are streamed from their start. Offset is measured in
        // decoded bytes; skipping has constant memory but repeats prefix work.
        let skipped=std::io::copy(&mut Read::by_ref(&mut decoder).take(offset),&mut std::io::sink()).map_err(WorkspaceError::io)?;
        if skipped!=offset{return Err(WorkspaceError::new(400,"INVALID_SESSION_OFFSET","The requested offset is past the end of this session."));}
        decoder.take((limit+1) as u64).read_to_end(&mut bytes).map_err(WorkspaceError::io)?;
    }else{
        let size=file.metadata().map_err(WorkspaceError::io)?.len();
        if offset>size {return Err(WorkspaceError::new(400,"INVALID_SESSION_OFFSET","The requested offset is past the end of this session."));}
        file.seek(SeekFrom::Start(offset)).map_err(WorkspaceError::io)?;
        file.take((limit+1) as u64).read_to_end(&mut bytes).map_err(WorkspaceError::io)?;
    }
    let reached_end=bytes.len()<=limit;
    bytes.truncate(limit);
    let available=bytes.len();
    // A continuation always falls on a UTF-8 boundary. If a caller supplied an
    // interior byte offset, reject it instead of corrupting the original text.
    let consumed=match std::str::from_utf8(&bytes){
        Ok(_)=>bytes.len(),
        Err(error) if error.error_len().is_none()=>error.valid_up_to(),
        Err(_)=>return Err(WorkspaceError::new(422,"INVALID_SESSION_TEXT","The offset is not a UTF-8 boundary or this log is not UTF-8 text.")),
    };
    bytes.truncate(consumed);
    let content=String::from_utf8(bytes).map_err(|_|WorkspaceError::new(422,"INVALID_SESSION_TEXT","This session is not UTF-8 text."))?;
    let next=offset+consumed as u64;let eof=reached_end&&consumed==available;
    Ok(success(json!({"session":session.value(),"file_path":session.file_path,"content":content,"offset":offset,"next_offset":next,"eof":eof})))
}
