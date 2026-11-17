//! Convert native agent definitions without guessing app-specific policy.
use super::{App, MAX_FILE_BYTES, WorkspaceError, success};
use serde_json::{Map, Value, json};

pub(super) struct Agent {
    fields: Map<String,Value>,
    instructions: String,
}

pub(super) fn preview(app: App, content: &str) -> Result<Value, WorkspaceError> {
    let agent = parse(app, content)?;
    Ok(json!({"fields": agent.fields, "instructions": agent.instructions}))
}

fn invalid(message: impl Into<String>) -> WorkspaceError {
    WorkspaceError::new(422,"INVALID_AGENT_DEFINITION",message)
}

fn claude_definition_parts(content: &str) -> Result<(&str, &str), WorkspaceError> {
    let first_end = content.find('\n').ok_or_else(|| invalid("A Claude agent needs YAML settings between --- lines, followed by its instructions."))?;
    if content[..first_end].trim_end_matches('\r') != "---" {
        return Err(invalid("A Claude agent must begin with a --- settings delimiter."));
    }
    let start = first_end + 1;
    let mut offset = start;
    for line in content[start..].split_inclusive('\n') {
        if line.trim_end_matches(['\r', '\n']) == "---" {
            return Ok((&content[start..offset], &content[offset + line.len()..]));
        }
        offset += line.len();
    }
    Err(invalid("Close the Claude agent's YAML settings with a --- line."))
}

pub(super) fn parse(app: App, content: &str) -> Result<Agent,WorkspaceError> {
    if content.len() as u64>MAX_FILE_BYTES {return Err(WorkspaceError::new(413,"WORKSPACE_FILE_TOO_LARGE","This agent definition exceeds 1 MiB."));}
    let (mut fields,instructions)=match app {
        App::Codex=>{
            let value:toml::Value=toml::from_str(content).map_err(|error|invalid(error.to_string()))?;
            let value=serde_json::to_value(value).map_err(|error|invalid(error.to_string()))?;
            let mut fields=value.as_object().cloned().ok_or_else(||invalid("A Codex agent must be a TOML table."))?;
            let instructions=match fields.remove("developer_instructions") {
                None=>String::new(),Some(Value::String(text))=>text,
                Some(_)=>return Err(invalid("developer_instructions must be text.")),
            };
            (fields,instructions)
        },
        App::Claude=>{
            let (header, body) = claude_definition_parts(content)?;
            let value:Value=serde_saphyr::from_str(header).map_err(|error|invalid(error.to_string()))?;
            let fields=value.as_object().cloned().ok_or_else(||invalid("Claude agent settings must be a YAML mapping."))?;
            (fields,body.trim_start_matches(['\r','\n']).to_owned())
        },
    };
    for key in ["name","description","model"] {
        if fields.get(key).is_some_and(|value|!value.is_string()) {return Err(invalid(format!("{key} must be text.")));}
    }
    // Keep original setting values as parsed. Conversion consumes only fields
    // with an explicit equivalent; all other fields accompany the draft.
    Ok(Agent {fields:std::mem::take(&mut fields),instructions})
}

pub(super) fn validate_save(app:App,content:&str,new_file:bool)->Result<(),WorkspaceError>{
    let agent=parse(app,content)?;
    // Existing Codex config_file roles may inherit their name/description from
    // config.toml. A new standalone definition must identify itself.
    if app==App::Claude||new_file {
        for key in ["name","description"] {
            if agent.fields.get(key).and_then(Value::as_str).is_none_or(|value|value.trim().is_empty()) {
                return Err(invalid(format!("Provide the agent's {key} before saving.")));
            }
        }
    }
    Ok(())
}

pub(super) fn convert(
    source: App, target: App, content: &str,
    fallback_name: Option<&str>, fallback_description: Option<&str>,
) -> Result<Value, WorkspaceError> {
    let parsed=parse(source,content)?;
    if source==target {
        return Ok(success(json!({"source_app":source.as_str(),"target_app":target.as_str(),"source_content":content,"content":content,"name":parsed.fields.get("name"),"description":parsed.fields.get("description"),"issues":[],"warnings":[],"requires_review":false})));
    }
    let mut fields=Map::new();let mut issues=Vec::new();
    for (key,value) in &parsed.fields {
        if matches!(key.as_str(),"name"|"description") {fields.insert(key.clone(),value.clone());}
        else if source == App::Codex && target == App::Claude
            && key == "sandbox_mode" && value.as_str() == Some("read-only")
        {
            fields.insert("permissionMode".into(), json!("plan"));
        } else {
            issues.push(json!({"field":key,"value":value,"message":if key=="model" {"Choose a model reported by the destination app."} else {"Review this source-app setting; it has no automatic conversion."},"requires_action":true}));
        }
    }
    for (key, fallback) in [("name", fallback_name), ("description", fallback_description)] {
        if fields.get(key).and_then(Value::as_str).is_none_or(|value|value.trim().is_empty()) {
            if let Some(value) = fallback.filter(|value| !value.trim().is_empty()) {
                fields.insert(key.into(), json!(value));
                continue;
            }
            fields.insert(key.into(),json!(""));
            issues.push(json!({"field":key,"value":null,"message":format!("Provide the destination agent's {key}."),"requires_action":true}));
        }
    }
    let name=fields.get("name").cloned();let description=fields.get("description").cloned();
    let converted=encode(target,Agent{fields,instructions:parsed.instructions})?;
    let warnings:Vec<_>=issues.iter().filter_map(|issue|issue["message"].as_str()).collect();
    Ok(success(json!({"source_app":source.as_str(),"target_app":target.as_str(),"source_content":content,"content":converted,"name":name,"description":description,"issues":issues,"warnings":warnings,"requires_review":!issues.is_empty()})))
}

fn encode(app:App,mut agent:Agent)->Result<String,WorkspaceError>{
    match app {
        App::Codex=>{
            agent.fields.insert("developer_instructions".into(),json!(agent.instructions));
            toml::to_string_pretty(&agent.fields).map_err(|error|invalid(error.to_string()))
        },
        App::Claude=>encode_claude(&agent.fields, &format!("\n{}", agent.instructions)),
    }
}

fn encode_claude(fields: &Map<String, Value>, body: &str) -> Result<String, WorkspaceError> {
    let mut header = serde_saphyr::to_string(fields).map_err(|error| invalid(error.to_string()))?;
    if !header.ends_with('\n') { header.push('\n'); }
    Ok(format!("---\n{header}---\n{body}"))
}

#[derive(Clone, Copy)]
pub(crate) enum ClaudeAgentMetadataField {
    Name,
    Description,
}

pub(crate) fn edit_claude_agent_metadata(
    content: &str,
    field: ClaudeAgentMetadataField,
    value: &str,
) -> Result<String, WorkspaceError> {
    let key = match field {
        ClaudeAgentMetadataField::Name => "name",
        ClaudeAgentMetadataField::Description => "description",
    };
    let mut agent = parse(App::Claude, content)?;
    if agent.fields.get(key).and_then(Value::as_str) == Some(value) {
        return Ok(content.to_owned());
    }
    agent.fields.insert(key.into(), json!(value));
    let (_, body) = claude_definition_parts(content)?;
    encode_claude(&agent.fields, body)
}

pub(super) fn set_model(app:App,content:&str,model:&str)->Result<Value,WorkspaceError>{
    if model.trim().is_empty(){return Err(invalid("Choose a destination model."));}
    let mut agent=parse(app,content)?;
    agent.fields.insert("model".into(),json!(model));
    Ok(success(json!({"app":app.as_str(),"model":model,"content":encode(app,agent)?})))
}
