//! Presentation of complete messages returned by the native conversation API.
//! Native readers own session parsing, branch selection and injected-context filtering.
use serde_json::Value;

#[derive(Clone)]
pub(super) struct Turn {
    pub role: String,
    pub text: String,
    pub code: bool,
    pub id: String,
    pub tool_name: Option<String>,
}
#[derive(Default)]
pub(super) struct History { turns: Vec<Turn> }
impl History {
    pub(super) fn from_conversation(value: &Value) -> Result<Self, String> {
        if value["complete"] != true {
            return Err("The session reader did not return a complete conversation.".into());
        }
        let messages=value["messages"].as_array().ok_or("The session reader did not return conversation messages.")?;
        let mut turns=Vec::with_capacity(messages.len());
        for message in messages {
            let (role, code, content)=match message["kind"].as_str() {
                Some("user") => ("User",false,&message["text"]),
                Some("assistant") => ("Assistant",false,&message["text"]),
                Some("tool_result") => ("Tool result",true,if message["output"].is_null() && !message["error"].is_null() {&message["error"]} else {&message["output"]}),
                _ => return Err("The session reader returned an unknown message kind.".into()),
            };
            let text=if code {
                match content.as_str() {
                    Some(text)=>text.to_owned(),
                    None=>serde_json::to_string_pretty(content).map_err(|error|error.to_string())?,
                }
            } else {content.as_str().ok_or("The session reader returned a message without text.")?.to_owned()};
            let id=message["id"].as_str().ok_or("The session reader returned a message without an ID.")?.to_owned();
            turns.push(Turn{role:role.into(),text,code,id,tool_name:message["tool_name"].as_str().map(str::to_owned)});
        }
        Ok(Self{turns})
    }
    pub(super) fn turns(&self)->impl Iterator<Item=&Turn>{self.turns.iter()}
    #[cfg(test)]
    pub(super) fn assistant_count(&self)->usize{self.turns.iter().filter(|turn|turn.role=="Assistant").count()}
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn complete_messages_keep_full_text_and_exact_tool_output(){
        let full=format!("Start {} end", "complete paragraph\n".repeat(5000));
        let value=json!({"complete":true,"messages":[
            {"id":"u","kind":"user","text":"Check the file."},
            {"id":"t","kind":"tool_result","tool_name":"Read","output":"unchanged\n\toutput"},
            {"id":"a","kind":"assistant","text":full},
            {"id":"failed","kind":"tool_result","tool_name":"mcp.Read","output":null,"error":{"message":"Connection closed"}}
        ]});
        let history=History::from_conversation(&value).unwrap();
        let turns:Vec<_>=history.turns().collect();
        assert_eq!(turns.len(),4);assert_eq!(turns[1].text,"unchanged\n\toutput");
        assert_eq!(turns[1].tool_name.as_deref(),Some("Read"));assert_eq!(turns[2].text,full);
        assert_eq!(serde_json::from_str::<Value>(&turns[3].text).unwrap(),json!({"message":"Connection closed"}));
        assert!(History::from_conversation(&json!({"complete":false,"messages":[]})).is_err());
    }
}
