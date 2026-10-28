//! Lossless catalogs and explicit modern HTTP annotation rejection.
use super::{RunError, raw};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, value::RawValue};
use sha2::{Digest,Sha256};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Serialize,Deserialize)]
pub struct Tool {
    pub name:String,
    pub definition:Box<RawValue>,
    pub header_error:bool,
    pub source_rejected:bool,
}

pub fn validate_tools(definitions:Vec<Box<RawValue>>,modern_http:bool)->Result<Vec<Tool>,RunError> {
    let mut seen=BTreeSet::new(); let mut tools=vec![];
    for definition in definitions {
        let name=raw::text(&definition,"name").ok_or_else(RunError::catalog)?;
        if !seen.insert(name.clone()) { return Err(RunError::catalog()); }
        let header_error=header_mappings(&definition).is_err();
        tools.push(Tool{name,definition,header_error,source_rejected:modern_http&&header_error});
    }
    tools.sort_by(|a,b|a.name.cmp(&b.name));
    Ok(tools)
}
pub fn origin(server_id:&str,tools:&[Tool],view:&str)->Value {
    let mut accepted=vec![]; let mut rejected=vec![];
    for tool in tools {
        if tool.source_rejected || (view==super::MODERN && tool.header_error) {
            rejected.push(json!({"name":tool.name,"code":"INVALID_MCP_HEADER_MAPPING","message":"This tool declares an invalid MCP HTTP header mapping."}));
        } else { accepted.push(tool.definition.clone()); }
    }
    json!({"source":{"kind":"server","server_id":server_id},"status":if rejected.is_empty(){"available"}else{"partial"},"observed_at":chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis,true),"tools":accepted,"error":null,"rejected_tools":rejected})
}
pub fn unavailable_origin(server_id:&str,error:&RunError)->Value {
    json!({"source":{"kind":"server","server_id":server_id},"status":"unavailable","observed_at":null,"tools":[],"error":{"code":error.code,"message":error.message},"rejected_tools":[]})
}
#[derive(Clone)]
pub struct HeaderMapping { pub name:String,pub path:Vec<String>,pub kind:String }
pub fn header_mappings(tool:&RawValue)->Result<Vec<HeaderMapping>,()> {
    let Some(schema)=raw::field(tool,"inputSchema") else { return Ok(vec![]); };
    let schema=raw::scalar(&schema)?; let mut mappings=vec![]; let mut names=BTreeSet::new();
    fn visit(value:&Value,path:&mut Vec<String>,reachable:bool,mappings:&mut Vec<HeaderMapping>,names:&mut BTreeSet<String>)->Result<(),()> {
        match value {
            Value::Object(object)=> {
                if let Some(header)=object.get("x-mcp-header") {
                    let name=header.as_str().ok_or(())?;
                    let kind=object.get("type").and_then(Value::as_str).ok_or(())?;
                    if !reachable || path.is_empty() || !matches!(kind,"string"|"integer"|"boolean") || name.is_empty() || !name.bytes().all(tchar) || !names.insert(name.to_ascii_lowercase()) { return Err(()); }
                    mappings.push(HeaderMapping{name:format!("Mcp-Param-{name}"),path:path.clone(),kind:kind.to_owned()});
                }
                for (key,child) in object {
                    if key=="properties" && reachable {
                        if let Some(properties)=child.as_object() {
                            for (name,property) in properties {
                                path.push(name.clone()); visit(property,path,true,mappings,names)?; path.pop();
                            }
                        } else { visit(child,path,false,mappings,names)?; }
                    } else if key!="x-mcp-header" { visit(child,path,false,mappings,names)?; }
                }
            }
            Value::Array(values)=> for value in values { visit(value,path,false,mappings,names)?; },
            _=>{},
        }
        Ok(())
    }
    visit(&schema,&mut vec![],true,&mut mappings,&mut names)?;
    Ok(mappings)
}
pub fn mirrored_headers(tool:&RawValue,arguments:&RawValue)->Result<BTreeMap<String,String>,RunError> {
    let mappings=header_mappings(tool).map_err(|_|RunError::unsupported(false))?;
    let values=raw::scalar(arguments).map_err(|_|RunError::invalid())?;
    let mut headers=BTreeMap::new();
    for mapping in mappings {
        let mut value=&values;
        for part in &mapping.path { value=value.get(part).unwrap_or(&Value::Null); }
        if value.is_null() { continue; }
        let text=match mapping.kind.as_str() {
            "string"=>value.as_str().map(str::to_owned),
            "boolean"=>value.as_bool().map(|v|v.to_string()),
            "integer"=>value.as_i64().filter(|v|(-9_007_199_254_740_991..=9_007_199_254_740_991).contains(v)).map(|v|v.to_string()),
            _=>None,
        }.ok_or_else(RunError::invalid)?;
        headers.insert(mapping.name,encode_header(&text));
    }
    Ok(headers)
}
fn tchar(byte:u8)->bool { byte.is_ascii_alphanumeric()||b"!#$%&'*+-.^_`|~".contains(&byte) }
pub fn encode_header(value:&str)->String {
    let plain=value.bytes().all(|b|b==b'\t'||(0x20..=0x7e).contains(&b)) && value.trim()==value && !(value.starts_with("=?base64?")&&value.ends_with("?="));
    if plain { value.to_owned() } else { format!("=?base64?{}?=",STANDARD.encode(value.as_bytes())) }
}
pub fn decode_header(value:&str)->Option<String> {
    if let Some(value)=value.strip_prefix("=?base64?").and_then(|v|v.strip_suffix("?=")) {
        String::from_utf8(STANDARD.decode(value).ok()?).ok()
    } else if value.trim()==value && value.bytes().all(|b|b==b'\t'||(0x20..=0x7e).contains(&b)) { Some(value.to_owned()) } else { None }
}

#[derive(Default,Clone,Serialize,Deserialize)]
pub struct Aliases {
    by_alias:BTreeMap<String,(String,String)>,
}
impl Aliases {
    pub fn assign(&mut self,server:&str,original:&str)->Result<String,RunError> {
        if let Some((alias,_))=self.by_alias.iter().find(|(_,target)|target.0==server&&target.1==original) { return Ok(alias.clone()); }
        let server_hash=format!("{:x}",Sha256::digest(server.as_bytes()));
        let part=if !original.is_empty()&&original.len()<=88&&original.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_'||b==b'-') { original.to_owned() }
            else { format!("t_{:x}",Sha256::digest(original.as_bytes())) };
        let prefix=format!("mcp_{}__",&server_hash[..32]);
        let mut alias=format!("{prefix}{part}");
        for attempt in 0..1024u32 {
            match self.by_alias.get(&alias) {
                None=>{self.by_alias.insert(alias.clone(),(server.to_owned(),original.to_owned()));return Ok(alias);},
                Some(target) if target.0==server&&target.1==original=>return Ok(alias),
                _=>{ let digest=format!("{:x}",Sha256::digest(format!("{server}\0{original}\0{attempt}").as_bytes())); alias=format!("{prefix}t_{digest}"); },
            }
        }
        Err(RunError::catalog())
    }
    pub fn resolve(&self,alias:&str)->Option<(String,String)> { self.by_alias.get(alias).cloned() }
}

pub fn qualified_uri(server:&str,uri:&str)->String {
    // The original tail is literal and never URL-normalized or percent-decoded.
    // RFC6570 braces remain visible to clients expanding a template.
    format!("fetch-resource://{:x}/{}",Sha256::digest(server.as_bytes()),uri)
}
pub fn unqualify_uri<'a>(server:&str,uri:&'a str)->Option<&'a str> {
    uri.strip_prefix(&format!("fetch-resource://{:x}/",Sha256::digest(server.as_bytes())))
}
