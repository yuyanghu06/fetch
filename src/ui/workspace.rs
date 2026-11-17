//! Workspace presents native API responses. It never reads or parses native files.
use super::{Desktop, Message, client::NativeClient, components, file_picker, session_history, surface, theme};
use crate::workspace::ClaudeAgentMetadataField;
use iced::{Element, Task, widget::{column, row, text, text_editor, pick_list, markdown, Space}};
use reqwest::Method;
use serde_json::{Value, json};
use std::collections::{BTreeMap,BTreeSet};
use std::path::Path;

pub(super) mod bulk;

pub(super) const AGENT_EDITOR_ID: &str = "fetch-workspace-agent-editor";
const INSTRUCTION_EDITOR_ID: &str = "fetch-workspace-instruction-editor";
pub(super) const AGENT_EDIT_SCROLL_ID: &str = "fetch-workspace-agent-edit-scroll";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(super) enum Section { #[default] Instructions, Agents, Sessions }
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Model { value: String, label: String }
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(&self.label) }
}
#[derive(Clone)]
pub(super) enum Event {
    FolderInput(String), ChooseFolder, FolderChosen(u64, Result<Option<String>, String>),
    OpenFolder, Global, Refresh, App(String), Section(Section),
    Discovered(u64, Result<Value, String>), Sessions(u64, Result<Value, String>), MoreSessions,
    OpenFile(String), OpenAgent(String), AddAgent, File(u64, String, Result<Value, String>), Edit(text_editor::Action), Undo,
    CloseAgentPopup, EditAgent, PreviewAgent, PreviewAction(text_editor::Action), IgnoreMarkdownLink, RevealFile(String), Revealed(u64,String,Result<Value,String>),
    DraftName(String), EditClaudeMetadata(ClaudeAgentMetadataField, String), Save, Saved(u64, Result<Value, String>), CloseEditor,
    CopyAgent(String), Copied(u64, String, Option<String>, String, Result<Value, String>), PasteAgent, Pasted(u64, Option<String>),
    PasteInstructions, InstructionsClipboard(u64, String, Option<String>),
    PasteOnAgent(String), RowAgentsClipboard(u64, String, Option<String>),
    RowAgentsSaved(u64, String, Result<Vec<bulk::SaveResult>, String>),
    RequestDeleteAgent(String), CancelDeleteAgent, ConfirmDeleteAgent,
    AgentDeleted(u64, String, Result<Value, String>),
    CopyAllAgents, CopiedAllAgents(u64, Result<String, String>), PasteAllAgents, BulkAgentsClipboard(u64, Option<String>),
    BulkAgentsSaved(u64, Result<Vec<bulk::SaveResult>, String>),
    Converted(u64, Result<Value, String>), ModelApplied(u64, Result<Value, String>), Models, ModelList(u64, String, Result<Value, String>), SelectModel(Model),
    OpenSession(String), CloseSession, ToggleTool(String,String),
    ConversationLoaded(u64,String,Result<Value,String>),
}

enum BulkFeedback {
    Copied,
    Pasted,
    Error(String),
}

struct DeleteAgent {
    name: String,
    path: String,
    pending: bool,
    error: Option<String>,
}

#[derive(Default)]
pub(super) struct Log {
    pub eof: bool,
    pub history: session_history::History,
    pub loading: bool,
    pub expanded_tools:BTreeSet<String>,
    pub markdown:BTreeMap<String,Vec<markdown::Item>>,
    pub tool_display:BTreeMap<String,String>,
    pub error: Option<String>,
}
struct EditorSnapshot {text:String,cursor:text_editor::Cursor}
pub(super) struct State {
    pub folder_input: String,
    pub folder_path: Option<String>,
    pub section: Section,
    pub app: String,
    pub generation: u64,
    pub files: Vec<Value>,
    pub agent_directory: String,
    pub bulk_busy: bool,
    bulk_feedback: Option<BulkFeedback>,
    delete_agent: Option<DeleteAgent>,
    pub sessions: Vec<Value>,
    pub next_cursor: Option<String>,
    pub logs: BTreeMap<String, Log>,
    pub active_session: Option<String>,
    pub sessions_appending: bool,
    pub file_path: Option<String>,
    pub destination: String,
    draft_name: String,
    pub editor: text_editor::Content,
    pub agent_popup:bool,
    pub agent_editing:bool,
    agent_fields:Vec<(String,String)>,
    agent_instructions:text_editor::Content,
    agent_preview_error:Option<String>,
    reveal_pending:BTreeSet<String>,
    undo:Vec<EditorSnapshot>,
    saved_text:String,
    pub issues: Vec<Value>,
    pub draft: bool,
    pub dirty: bool,
    pub models: Vec<Model>,
    pub selected_model: Option<Model>,
    pub loading: bool,
    pub sessions_loading: bool,
    pub file_loading: bool,
    pub models_loading: bool,
    pub saving: bool,
    pub error: Option<String>,
    pub sessions_error: Option<String>,
    pub model_error: Option<String>,
    pub catalog_source: Option<String>,
}
impl Default for State {
    fn default() -> Self {
        Self { folder_input:String::new(),folder_path:None,section:Section::Instructions,app:"claude".into(),
            generation:0,files:vec![],agent_directory:String::new(),bulk_busy:false,bulk_feedback:None,delete_agent:None,sessions:vec![],next_cursor:None,logs:BTreeMap::new(),active_session:None,sessions_appending:false,file_path:None,
            destination:String::new(),draft_name:String::new(),editor:text_editor::Content::new(),agent_popup:false,agent_editing:false,agent_fields:vec![],agent_instructions:text_editor::Content::new(),agent_preview_error:None,reveal_pending:BTreeSet::new(),undo:vec![],saved_text:String::new(),issues:vec![],
            draft:false,dirty:false,models:vec![],selected_model:None,loading:false,sessions_loading:false,
            file_loading:false,models_loading:false,saving:false,error:None,sessions_error:None,model_error:None,catalog_source:None }
    }
}

impl State {
    pub(super) fn delete_agent_visible(&self) -> bool {
        self.delete_agent.is_some()
    }

    fn can_add_agent(&self) -> bool {
        self.section == Section::Agents && !self.loading && !self.file_loading
            && !self.saving && !self.bulk_busy && !self.agent_directory.is_empty()
    }

    fn edits_claude_metadata(&self) -> bool {
        self.app == "claude" && self.agent_popup && self.agent_editing
            && !self.draft && self.file_path.is_some()
    }

    pub(super) fn agents(&self) -> impl Iterator<Item = &Value> {
        self.files.iter().filter(|file| file["kind"] == "agent" && file["app"] == self.app)
    }

    fn set_draft_name(&mut self, name: String) {
        self.draft_name = name;
        self.destination.clear();
        let name = self.draft_name.trim();
        if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\', '\0']) {
            return;
        }
        let suffix = if self.app == "claude" { ".md" } else { ".toml" };
        let filename = if name.ends_with(suffix) { name.to_owned() } else { format!("{name}{suffix}") };
        self.destination = Path::new(&self.agent_directory).join(filename).to_string_lossy().into_owned();
    }
}

fn copied_agent(value: &str) -> Option<Value> {
    serde_json::from_str::<Value>(value).ok()
        .filter(|value| value["fetch_agent"] == 1
            && matches!(value["app"].as_str(), Some("claude" | "codex"))
            && value["content"].is_string())
}

fn query(folder:Option<&str>, pairs:&[(&str, &str)]) -> String {
    let mut query=url::form_urlencoded::Serializer::new(String::new());
    if let Some(folder)=folder { query.append_pair("folder_path",folder); }
    for (key,value) in pairs { query.append_pair(key,value); }
    let query=query.finish(); if query.is_empty(){String::new()}else{format!("?{query}")}
}
fn call(client:NativeClient,method:Method,path:String,body:Option<Value>) -> impl std::future::Future<Output=Result<Value,String>>+Send {
    async move { client.request(method,&path,body).await }
}
impl Desktop {
    fn load_agent_models(&mut self) -> Task<Message> {
        if self.workspace.models.is_empty() && self.workspace.catalog_source.is_none() {
            return self.update_workspace(Event::Models);
        }
        Task::none()
    }

    fn read_workspace_files(&self) -> Task<Message> {
        let Some(client) = self.client.clone() else { return Task::none(); };
        let generation = self.workspace.generation;
        let path = format!("/api/v1/workspace{}", query(self.workspace.folder_path.as_deref(), &[]));
        Task::perform(call(client, Method::GET, path, None), move |result| Message::Workspace(Event::Discovered(generation, result)))
    }

    pub(super) fn load_workspace(&mut self) -> Task<Message> {
        if self.client.is_none() { return Task::none(); }
        self.workspace.loading=true;self.workspace.error=None;
        Task::batch([self.read_workspace_files(),self.load_workspace_sessions(false)])
    }
    fn load_workspace_sessions(&mut self, more:bool) -> Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        if self.workspace.sessions_loading{return Task::none();}
        self.workspace.sessions_error=None;
        self.workspace.sessions_loading=true;self.workspace.sessions_appending=more;
        let generation=self.workspace.generation;
        let mut pairs=vec![("app",self.workspace.app.as_str()),("view","conversation")];
        if more { if let Some(cursor)=self.workspace.next_cursor.as_deref(){pairs.push(("cursor",cursor));} }
        let path=format!("/api/v1/workspace/sessions{}",query(self.workspace.folder_path.as_deref(),&pairs));
        Task::perform(call(client,Method::GET,path,None),move|result|Message::Workspace(Event::Sessions(generation,result)))
    }
    fn read_workspace_conversation(&mut self,id:String) -> Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        let Some(session)=self.workspace.sessions.iter().find(|session|session["id"].as_str()==Some(&id)) else{return Task::none();};
        let Some(file)=session["file_path"].as_str() else{return Task::none();};
        let log=self.workspace.logs.entry(id.clone()).or_default();
        if log.loading||log.eof{return Task::none();}
        log.loading=true;log.error=None;
        let path=format!("/api/v1/workspace/sessions/conversation{}",query(self.workspace.folder_path.as_deref(),&[("file_path",file)]));
        let generation=self.workspace.generation;
        Task::perform(call(client,Method::GET,path,None),move|result|Message::Workspace(Event::ConversationLoaded(generation,id.clone(),result)))
    }
    fn refresh_agent_preview(&mut self){
        self.workspace.agent_fields.clear();self.workspace.agent_instructions=text_editor::Content::new();self.workspace.agent_preview_error=None;
        match crate::workspace::agent_definition_preview(&self.workspace.app,&self.workspace.editor.text()){
            Ok(value)=>{
                self.workspace.agent_fields=value["fields"].as_object().into_iter().flatten().map(|(key,value)|(key.clone(),value.as_str().map(str::to_owned).unwrap_or_else(||serde_json::to_string_pretty(value).unwrap_or_default()))).collect();
                self.workspace.agent_instructions=text_editor::Content::with_text(value["instructions"].as_str().unwrap_or(""));
            }
            Err(error)=>self.workspace.agent_preview_error=Some(format!("{}: {}",error.code,error.message)),
        }
    }
    fn reset_workspace_selection(&mut self) {
        let state=&mut self.workspace;
        state.generation+=1;state.files.clear();state.agent_directory.clear();state.sessions.clear();state.logs.clear();state.active_session=None;state.next_cursor=None;
        state.loading=false;state.sessions_loading=false;state.file_loading=false;state.models_loading=false;state.saving=false;
        state.file_path=None;state.destination.clear();state.draft_name.clear();state.editor=text_editor::Content::new();state.undo.clear();state.saved_text.clear();
        state.agent_popup=false;state.agent_editing=false;state.agent_fields.clear();state.agent_instructions=text_editor::Content::new();state.agent_preview_error=None;state.reveal_pending.clear();
        state.issues.clear();state.dirty=false;state.draft=false;state.models.clear();state.selected_model=None;
        state.bulk_feedback = None;
        state.delete_agent = None;
        state.error=None;state.sessions_error=None;state.model_error=None;state.catalog_source=None;
    }
    pub(super) fn update_workspace(&mut self,event:Event) -> Task<Message> {
        match event {
            Event::FolderInput(value)=>self.workspace.folder_input=value,
            Event::ChooseFolder=>{
                let generation=self.workspace.generation;
                return Task::perform(file_picker::choose_config_file(),move|result|Message::Workspace(Event::FolderChosen(generation,result)));
            }
            Event::FolderChosen(generation,result) if generation==self.workspace.generation=>match result {
                Ok(Some(path))=>{self.workspace.folder_input=path;return self.update_workspace(Event::OpenFolder);},
                Ok(None)=>{},Err(error)=>self.workspace.error=Some(error),
            },
            Event::OpenFolder=>{
                let folder=self.workspace.folder_input.trim().to_owned();
                if folder.is_empty(){return self.update_workspace(Event::Global);}
                self.reset_workspace_selection();self.workspace.folder_path=Some(folder);
                return self.load_workspace();
            }
            Event::Global=>{
                self.reset_workspace_selection();self.workspace.folder_input.clear();self.workspace.folder_path=None;
                return self.load_workspace();
            }
            Event::Refresh=>{
                if self.workspace.loading||self.workspace.sessions_loading{return Task::none();}
                self.workspace.logs.clear();self.workspace.active_session=None;self.workspace.next_cursor=None;
                return self.load_workspace();
            }
            Event::App(app)=>{
                if self.workspace.app==app{return Task::none();}
                self.reset_workspace_selection();self.workspace.app=app;
                return self.load_workspace();
            }
            Event::Section(section)=>{self.workspace.section=section;if section==Section::Instructions{return self.open_workspace_instruction();}},
            Event::Discovered(generation,result) if generation==self.workspace.generation=>{
                self.workspace.loading=false;
                match result {
                    Ok(value)=>{
                        self.workspace.files=value["files"].as_array().cloned().unwrap_or_default();
                        self.workspace.agent_directory=value["agent_directories"].as_array().into_iter().flatten()
                            .find(|directory|directory["app"]==self.workspace.app).and_then(|directory|directory["file_path"].as_str()).unwrap_or("").to_owned();
                        self.workspace.folder_path=value["folder_path"].as_str().map(str::to_owned);
                        let registration=self.register_workspace_project();
                        let editor=if self.workspace.section==Section::Instructions{self.open_workspace_instruction()}else{Task::none()};
                        return Task::batch([registration,editor]);
                    },Err(error)=>self.workspace.error=Some(error),
                }
            }
            Event::Sessions(generation,result) if generation==self.workspace.generation=>{
                self.workspace.sessions_loading=false;
                match result {
                    Ok(value)=>{
                        self.workspace.sessions_error=None;
                        if !self.workspace.sessions_appending{self.workspace.sessions.clear();}
                        for session in value["sessions"].as_array().into_iter().flatten(){
                            if !self.workspace.sessions.iter().any(|existing|existing["id"]==session["id"]){self.workspace.sessions.push(session.clone());}
                        }
                        self.workspace.next_cursor=value["next_cursor"].as_str().map(str::to_owned);
                    },Err(error)=>self.workspace.sessions_error=Some(error),
                }
            }
            Event::MoreSessions=>{if self.workspace.next_cursor.is_some(){return self.load_workspace_sessions(true);}},
            Event::OpenAgent(path)=>{
                if self.workspace.file_loading||self.workspace.saving{return Task::none();}
                self.workspace.agent_popup=true;self.workspace.agent_editing=self.workspace.dirty;
                if self.workspace.file_path.as_ref()==Some(&path){
                    self.refresh_agent_preview();
                    if self.workspace.agent_editing {return self.load_agent_models();}
                    return Task::none();
                }
                self.workspace.agent_editing=false;
                return self.update_workspace(Event::OpenFile(path));
            }
            Event::AddAgent => {
                if !self.workspace.can_add_agent() { return Task::none(); }
                let state = &mut self.workspace;
                state.file_path = None;
                state.destination.clear();
                state.draft_name.clear();
                state.editor = text_editor::Content::new();
                state.saved_text.clear();
                state.undo.clear();
                state.agent_fields.clear();
                state.agent_instructions = text_editor::Content::new();
                state.agent_preview_error = None;
                state.selected_model = None;
                state.issues.clear();
                state.dirty = false;
                state.draft = true;
                state.error = None;
                state.agent_popup = true;
                state.agent_editing = true;
                return Task::batch([
                    iced::widget::operation::focus(AGENT_EDITOR_ID),
                    self.load_agent_models(),
                ]);
            }
            Event::CloseAgentPopup=>{self.workspace.agent_popup=false;},
            Event::EditAgent=>{
                if !self.workspace.file_loading {
                    self.workspace.agent_editing=true;
                    return self.load_agent_models();
                }
            },
            Event::PreviewAgent=>{self.refresh_agent_preview();self.workspace.agent_editing=false;},
            Event::PreviewAction(action)=>{if !action.is_edit(){self.workspace.agent_instructions.perform(action);}},
            Event::IgnoreMarkdownLink=>{},
            Event::RevealFile(path)=>{
                if self.workspace.reveal_pending.contains(&path){return Task::none();}
                let Some(client)=self.client.clone()else{return Task::none();};
                self.workspace.reveal_pending.insert(path.clone());self.workspace.error=None;
                let generation=self.workspace.generation;
                let body=json!({"file_path":path,"folder_path":self.workspace.folder_path});
                return Task::perform(call(client,Method::POST,"/api/v1/workspace/file/reveal".into(),Some(body)),move|result|Message::Workspace(Event::Revealed(generation,path.clone(),result)));
            }
            Event::Revealed(generation,path,result) if generation==self.workspace.generation=>{
                self.workspace.reveal_pending.remove(&path);if let Err(error)=result{self.workspace.error=Some(error);}
            }
            Event::OpenFile(path)=>{
                if self.workspace.file_loading||self.workspace.saving{return Task::none();}
                let Some(client)=self.client.clone() else{return Task::none();};
                self.workspace.file_loading=true;self.workspace.file_path=Some(path.clone());
                self.workspace.destination=path.clone();self.workspace.draft=false;self.workspace.dirty=false;
                self.workspace.issues.clear();self.workspace.editor=text_editor::Content::new();self.workspace.undo.clear();self.workspace.saved_text.clear();
                self.workspace.error=None;
                if self.workspace.section==Section::Agents{self.workspace.agent_popup=true;self.workspace.agent_fields.clear();self.workspace.agent_instructions=text_editor::Content::new();self.workspace.agent_preview_error=None;}
                let generation=self.workspace.generation;
                let url=format!("/api/v1/workspace/file{}",query(self.workspace.folder_path.as_deref(),&[("file_path",&path)]));
                return Task::perform(call(client,Method::GET,url,None),move|result|Message::Workspace(Event::File(generation,path.clone(),result)));
            }
            Event::File(generation,path,result) if generation==self.workspace.generation&&self.workspace.file_path.as_ref()==Some(&path)=>{
                self.workspace.file_loading=false;
                match result {Ok(value)=>{self.workspace.saved_text=value["content"].as_str().unwrap_or("").into();self.workspace.editor=text_editor::Content::with_text(&self.workspace.saved_text);},Err(error)=>self.workspace.error=Some(error)}
                if self.workspace.agent_popup{self.refresh_agent_preview();}
                if self.workspace.section==Section::Instructions{return self.open_workspace_instruction();}
            }
            Event::Edit(mut action)=>{if !self.workspace.saving&&!self.workspace.file_loading{
                if self.workspace.agent_popup
                    && let text_editor::Action::Edit(text_editor::Edit::Paste(contents)) = &action
                    && let Some(source) = copied_agent(contents)
                {
                    if self.workspace.draft {
                        let Some(client) = self.client.clone() else { return Task::none(); };
                        self.workspace.file_loading = true;
                        self.workspace.error = None;
                        let generation = self.workspace.generation;
                        let body = json!({
                            "source_app": source["app"],
                            "target_app": self.workspace.app,
                            "content": source["content"],
                            "fallback_name": source["name"].as_str(),
                            "fallback_description": source["description"].as_str(),
                        });
                        return Task::perform(call(client, Method::POST, "/api/v1/workspace/agents/convert".into(), Some(body)), move |result| Message::Workspace(Event::Converted(generation, result)));
                    }
                    action = text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new(
                        source["content"].as_str().unwrap_or_default().to_owned(),
                    )));
                }
                let snapshot=action.is_edit().then(||EditorSnapshot{text:self.workspace.editor.text(),cursor:self.workspace.editor.cursor()});
                self.workspace.editor.perform(action);
                if let Some(snapshot)=snapshot{if snapshot.text!=self.workspace.editor.text(){self.workspace.undo.push(snapshot);}}
                self.workspace.dirty=self.workspace.editor.text()!=self.workspace.saved_text;
            }},
            Event::Undo=>{if !self.workspace.saving&&!self.workspace.file_loading{if let Some(snapshot)=self.workspace.undo.pop(){
                self.workspace.editor=text_editor::Content::with_text(&snapshot.text);self.workspace.editor.move_to(snapshot.cursor);
                self.workspace.dirty=self.workspace.editor.text()!=self.workspace.saved_text;
            }}},
            Event::DraftName(name) => {
                if self.workspace.draft && !self.workspace.saving && !self.workspace.file_loading {
                    self.workspace.set_draft_name(name);
                }
            }
            Event::EditClaudeMetadata(field, value) => {
                let state = &mut self.workspace;
                if !state.edits_claude_metadata() || state.saving || state.file_loading {
                    return Task::none();
                }
                let previous = state.editor.text();
                match crate::workspace::edit_claude_agent_metadata(&previous, field, &value) {
                    Ok(content) if content != previous => {
                        let cursor = state.editor.cursor();
                        state.undo.push(EditorSnapshot { text: previous, cursor });
                        state.editor = text_editor::Content::with_text(&content);
                        state.dirty = state.editor.text() != state.saved_text;
                        state.error = None;
                    }
                    Ok(_) => {}
                    Err(error) => state.error = Some(error.message),
                }
            }
            Event::CloseEditor=>{
                if self.workspace.file_loading||self.workspace.saving{return Task::none();}
                self.workspace.file_path=None;self.workspace.draft=false;self.workspace.dirty=false;
                self.workspace.editor=text_editor::Content::new();self.workspace.undo.clear();self.workspace.saved_text.clear();self.workspace.issues.clear();self.workspace.agent_popup=false;self.workspace.agent_editing=false;
            }
            Event::Save=>{
                if self.workspace.saving||self.workspace.file_loading||self.workspace.destination.trim().is_empty()
                    || (self.workspace.draft && !self.workspace.dirty) { return Task::none(); }
                let Some(client)=self.client.clone() else{return Task::none();};
                self.workspace.saving=true;self.workspace.error=None;
                let generation=self.workspace.generation;
                let body=json!({"file_path":self.workspace.destination,"folder_path":self.workspace.folder_path,"content":self.workspace.editor.text()});
                let draft = self.workspace.draft;
                return Task::perform(async move {
                    if draft {
                        client.save_new_workspace_file(body).await
                    } else {
                        call(client, Method::PUT, "/api/v1/workspace/file".into(), Some(body)).await
                    }
                },move|result|Message::Workspace(Event::Saved(generation,result)));
            }
            Event::Saved(generation,result) if generation==self.workspace.generation=>{
                self.workspace.saving=false;
                match result {
                    Ok(_)=>{
                        self.workspace.file_path=Some(self.workspace.destination.clone());self.workspace.draft=false;self.workspace.dirty=false;
                        self.workspace.issues.clear();self.workspace.saved_text=self.workspace.editor.text();
                        if self.workspace.agent_popup{self.refresh_agent_preview();}
                        // Refresh descriptors without replacing the editor or changing the folder.
                        return self.read_workspace_files();
                    },
                    Err(error) => self.workspace.error = Some(error),
                }
            }
            Event::CopyAgent(path)=>{
                let Some(client)=self.client.clone() else{return Task::none();};
                let app=self.workspace.app.clone();let generation=self.workspace.generation;
                let name = self.workspace.agents()
                    .find(|file| file["file_path"].as_str() == Some(path.as_str()))
                    .and_then(|file| file["name"].as_str())
                    .map(str::to_owned);
                let url=format!("/api/v1/workspace/file{}",query(self.workspace.folder_path.as_deref(),&[("file_path",&path)]));
                return Task::perform(call(client,Method::GET,url,None),move|result|Message::Workspace(Event::Copied(generation,app.clone(),name.clone(),path.clone(),result)));
            }
            Event::Copied(generation,app,name,path,result) if generation==self.workspace.generation=>match result {
                Ok(file)=>{
                    return iced::clipboard::write(bulk::copied_agent_payload(
                        &app, name.as_deref(), &path, &file, &self.workspace.agent_directory,
                    ));
                },Err(error)=>self.workspace.error=Some(error),
            },
            Event::PasteOnAgent(path) => {
                let state = &mut self.workspace;
                if self.client.is_none() || !state.can_add_agent()
                    || !state.agents().any(|row| row["file_path"].as_str() == Some(path.as_str()))
                {
                    return Task::none();
                }
                state.bulk_busy = true;
                state.bulk_feedback = None;
                let generation = state.generation;
                return iced::clipboard::read().map(move |text| Message::Workspace(Event::RowAgentsClipboard(generation, path.clone(), text)));
            }
            Event::RowAgentsClipboard(generation, path, text) => {
                if generation != self.workspace.generation {
                    self.workspace.bulk_busy = false;
                    return Task::none();
                }
                let (Some(client), Some(bundle)) = (self.client.clone(), text) else {
                    self.workspace.bulk_busy = false;
                    self.workspace.bulk_feedback = Some(BulkFeedback::Error(bulk::COPY_AGENT_FIRST.into()));
                    return Task::none();
                };
                let state = &self.workspace;
                let paste = bulk::paste_on_agent(client, state.folder_path.clone(), bundle, state.app.clone(), state.agent_directory.clone(), path.clone());
                return Task::perform(paste, move |result| Message::Workspace(Event::RowAgentsSaved(generation, path.clone(), result)));
            }
            Event::RowAgentsSaved(generation, path, result) => {
                if generation == self.workspace.generation
                    && self.workspace.file_path.as_deref() == Some(path.as_str())
                    && result.as_ref().is_ok_and(|entries| entries.first().is_some_and(|entry| entry.result.is_ok()))
                {
                    self.workspace.file_path = None;
                }
                return self.finish_agent_paste(generation, result, true);
            }
            Event::RequestDeleteAgent(path) => {
                let state = &mut self.workspace;
                if self.client.is_none() || state.bulk_busy || state.file_loading || state.saving || state.delete_agent.is_some() {
                    return Task::none();
                }
                let name = state.agents().find(|file| file["file_path"].as_str() == Some(path.as_str()))
                    .and_then(|file| file["name"].as_str()).map(str::to_owned);
                if let Some(name) = name {
                    state.delete_agent = Some(DeleteAgent { name, path, pending: false, error: None });
                }
            }
            Event::CancelDeleteAgent => {
                if self.workspace.delete_agent.as_ref().is_some_and(|target| !target.pending) {
                    self.workspace.delete_agent = None;
                }
            }
            Event::ConfirmDeleteAgent => {
                let Some(client) = self.client.clone() else { return Task::none(); };
                let state = &mut self.workspace;
                let Some(target) = state.delete_agent.as_mut().filter(|target| !target.pending) else {
                    return Task::none();
                };
                target.pending = true;
                target.error = None;
                let path = target.path.clone();
                let generation = state.generation;
                let request = format!("/api/v1/workspace/file{}", query(state.folder_path.as_deref(), &[("file_path", &path)]));
                return Task::perform(call(client, Method::DELETE, request, None), move |result| {
                    Message::Workspace(Event::AgentDeleted(generation, path.clone(), result))
                });
            }
            Event::AgentDeleted(generation, path, result) if generation == self.workspace.generation => {
                let state = &mut self.workspace;
                let Some(target) = state.delete_agent.as_mut().filter(|target| target.path == path && target.pending) else {
                    return Task::none();
                };
                match result {
                    Ok(_) => {
                        state.delete_agent = None;
                        state.files.retain(|file| file["file_path"].as_str() != Some(path.as_str()));
                        if state.file_path.as_deref() == Some(path.as_str()) {
                            state.file_path = None;
                        }
                        return self.read_workspace_files();
                    }
                    Err(error) => {
                        target.pending = false;
                        target.error = Some(error);
                    }
                }
            }
            Event::CopyAllAgents => {
                let state = &mut self.workspace;
                if !state.can_add_agent() { return Task::none(); }
                let Some(client) = self.client.clone() else { return Task::none(); };
                let agents: Vec<_> = state.agents().filter_map(|file| {
                    let path = file["file_path"].as_str()?.to_owned();
                    let name = file["name"].as_str().map(str::to_owned);
                    Some((path, name))
                }).collect();
                if agents.is_empty() { return Task::none(); }
                state.bulk_busy = true;
                state.bulk_feedback = None;
                let generation = state.generation;
                let copy = bulk::copy(client, state.folder_path.clone(), state.app.clone(), state.agent_directory.clone(), agents);
                return Task::perform(copy, move |result| Message::Workspace(Event::CopiedAllAgents(generation, result)));
            }
            Event::CopiedAllAgents(generation, result) => {
                self.workspace.bulk_busy = false;
                match result {
                    Ok(bundle) => {
                        self.workspace.bulk_feedback = Some(BulkFeedback::Copied);
                        return iced::clipboard::write(bundle);
                    }
                    Err(error) if generation == self.workspace.generation => self.workspace.bulk_feedback = Some(BulkFeedback::Error(error)),
                    Err(_) => {}
                }
            }
            Event::PasteAllAgents => {
                let state = &mut self.workspace;
                if self.client.is_none() || !state.can_add_agent() { return Task::none(); }
                state.bulk_busy = true;
                state.bulk_feedback = None;
                let generation = state.generation;
                return iced::clipboard::read().map(move |text| Message::Workspace(Event::BulkAgentsClipboard(generation, text)));
            }
            Event::BulkAgentsClipboard(generation, text) => {
                if generation != self.workspace.generation {
                    self.workspace.bulk_busy = false;
                    return Task::none();
                }
                let (Some(client), Some(bundle)) = (self.client.clone(), text) else {
                    self.workspace.bulk_busy = false;
                    self.workspace.bulk_feedback = Some(BulkFeedback::Error(bulk::COPY_AGENT_FIRST.into()));
                    return Task::none();
                };
                let state = &self.workspace;
                let folder = state.folder_path.clone();
                let target = state.app.clone();
                let directory = state.agent_directory.clone();
                return Task::perform(async move {
                    let drafts = bulk::prepare(client.clone(), bundle, target, directory).await?;
                    Ok(bulk::save(client, folder, drafts).await)
                }, move |result| Message::Workspace(Event::BulkAgentsSaved(generation, result)));
            }
            Event::BulkAgentsSaved(generation, result) => {
                return self.finish_agent_paste(generation, result, false);
            }
            Event::PasteInstructions => {
                let Some(path) = self.instruction_paste_target().map(str::to_owned) else { return Task::none(); };
                let generation = self.workspace.generation;
                return iced::clipboard::read().map(move |text| {
                    Message::Workspace(Event::InstructionsClipboard(generation, path.clone(), text))
                });
            }
            Event::InstructionsClipboard(generation, path, contents) => {
                if generation != self.workspace.generation || self.instruction_paste_target() != Some(path.as_str()) {
                    return Task::none();
                }
                let Some(contents) = contents.filter(|text| !text.is_empty()) else { return Task::none(); };
                let action = text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new(contents)));
                let paste = self.update_workspace(Event::Edit(action));
                return Task::batch([paste, iced::widget::operation::focus(INSTRUCTION_EDITOR_ID)]);
            }
            Event::PasteAgent=>{
                if !self.workspace.agent_popup||self.workspace.file_loading||self.workspace.saving{return Task::none();}
                self.workspace.file_loading=true;let generation=self.workspace.generation;
                return iced::clipboard::read().map(move|text|Message::Workspace(Event::Pasted(generation,text)));
            }
            Event::Pasted(generation,value) if generation==self.workspace.generation=>{
                self.workspace.file_loading = false;
                let Some(contents) = value.filter(|_| self.workspace.agent_popup) else { return Task::none(); };
                self.workspace.agent_editing = true;
                let paste = self.update_workspace(Event::Edit(text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new(contents)))));
                return Task::batch([paste, iced::widget::operation::focus(AGENT_EDITOR_ID), self.load_agent_models()]);
            }
            Event::Converted(generation,result) if generation==self.workspace.generation=>{
                self.workspace.file_loading=false;
                match result {
                    Ok(value)=>{
                        self.workspace.editor=text_editor::Content::with_text(value["content"].as_str().unwrap_or(""));self.workspace.undo.clear();
                        self.workspace.issues=value["issues"].as_array().cloned().unwrap_or_default();
                        if self.workspace.draft && self.workspace.draft_name.is_empty() {
                            if let Some(name) = value["name"].as_str() {
                                self.workspace.set_draft_name(name.to_owned());
                            }
                        }
                        self.workspace.draft=self.workspace.file_path.is_none();self.workspace.dirty=true;
                        return self.load_agent_models();
                    },Err(error)=>self.workspace.error=Some(error),
                }
            }
            Event::ModelApplied(generation,result) if generation==self.workspace.generation=>{
                self.workspace.file_loading=false;
                match result {
                    Ok(value)=>{
                        self.workspace.editor=text_editor::Content::with_text(value["content"].as_str().unwrap_or(""));self.workspace.undo.clear();
                        self.workspace.issues.retain(|issue|issue["field"]!="model");
                        self.workspace.dirty=true;
                    },Err(error)=>self.workspace.error=Some(error),
                }
            }
            Event::Models=>{
                if self.workspace.models_loading{return Task::none();}
                let Some(client)=self.client.clone() else{return Task::none();};
                self.workspace.models_loading=true;self.workspace.model_error=None;
                let app=self.workspace.app.clone();let generation=self.workspace.generation;
                let path=format!("/api/v1/workspace/models{}",query(None,&[("app",&app)]));
                return Task::perform(call(client,Method::GET,path,None),move|result|Message::Workspace(Event::ModelList(generation,app.clone(),result)));
            }
            Event::ModelList(generation,app,result) if generation==self.workspace.generation&&app==self.workspace.app=>{
                self.workspace.models_loading=false;
                match result {
                    Ok(value)=>{
                        self.workspace.catalog_source=value["source"].as_str().map(str::to_owned);
                        self.workspace.models=value["models"].as_array().into_iter().flatten().filter_map(|model|{
                            let id=model["value"].as_str().or_else(||model["model"].as_str()).or_else(||model["id"].as_str()).or_else(||model["slug"].as_str())?;
                            Some(Model{value:id.into(),label:model["displayName"].as_str().or_else(||model["display_name"].as_str()).unwrap_or(id).into()})
                        }).collect();
                    },Err(error)=>self.workspace.model_error=Some(error),
                }
            }
            Event::SelectModel(model)=>{
                let Some(client)=self.client.clone() else{return Task::none();};
                if self.workspace.saving||self.workspace.file_loading{return Task::none();}
                self.workspace.selected_model=Some(model.clone());self.workspace.file_loading=true;
                let generation=self.workspace.generation;
                let body=json!({"source_app":self.workspace.app,"target_app":self.workspace.app,"content":self.workspace.editor.text(),"target_model":model.value});
                return Task::perform(call(client,Method::POST,"/api/v1/workspace/agents/convert".into(),Some(body)),move|result|Message::Workspace(Event::ModelApplied(generation,result)));
            }
            Event::OpenSession(id)=>{
                if !self.workspace.sessions.iter().any(|session|session["id"]==id){return Task::none();}
                self.workspace.active_session=Some(id.clone());
                let log=self.workspace.logs.entry(id.clone()).or_default();
                if !log.eof&&!log.loading{return self.read_workspace_conversation(id);}
            }
            Event::CloseSession=>self.workspace.active_session=None,
            Event::ToggleTool(session,id)=>{if let Some(log)=self.workspace.logs.get_mut(&session){if !log.expanded_tools.remove(&id){log.expanded_tools.insert(id);}}},
            Event::ConversationLoaded(generation,id,result) if generation==self.workspace.generation=>{
                if let Some(log)=self.workspace.logs.get_mut(&id){
                    log.loading=false;
                    match result{
                        Ok(value)=>{
                            if value["complete"]!=true{log.error=Some("The session reader did not return a complete conversation.".into());}
                            else{match session_history::History::from_conversation(&value){Ok(history)=>{log.markdown=history.turns().filter(|turn|!turn.code).map(|turn|(turn.id.clone(),markdown::parse(&turn.text).collect())).collect();log.tool_display=history.turns().filter(|turn|turn.role=="Tool result").map(|turn|{
                                let display=serde_json::from_str::<Value>(&turn.text).ok().filter(|value|value.is_object()||value.is_array()).and_then(|value|serde_json::to_string_pretty(&value).ok()).unwrap_or_else(||turn.text.clone());
                                (turn.id.clone(),display)
                            }).collect();log.history=history;log.eof=true;log.expanded_tools.clear();},Err(error)=>log.error=Some(error)}}
                        }
                        Err(error)=>log.error=Some(error),
                    }
                }
            }
            _=>{},
        }
        Task::none()
    }

    fn finish_agent_paste(&mut self, generation: u64, result: Result<Vec<bulk::SaveResult>, String>, replaced: bool) -> Task<Message> {
        self.workspace.bulk_busy = false;
        match result {
            Ok(results) => {
                let changed = results.iter().filter(|entry| entry.result.is_ok()).count();
                self.workspace.bulk_feedback = if changed == results.len() {
                    Some(BulkFeedback::Pasted)
                } else {
                    let action = if replaced { "Pasted" } else { "Added" };
                    let mut report = vec![format!("{action} {changed} of {} subagents.", results.len())];
                    for entry in results {
                        if let Err(error) = entry.result {
                            report.push(format!("{}: {error}", entry.destination));
                        }
                    }
                    Some(BulkFeedback::Error(report.join("\n")))
                };
                if changed > 0 && generation == self.workspace.generation {
                    return self.read_workspace_files();
                }
            }
            Err(error) => self.workspace.bulk_feedback = Some(BulkFeedback::Error(error)),
        }
        Task::none()
    }
    fn workspace_instruction_path(&self)->Option<String>{
        let matches=|file:&&Value|file["kind"]=="instructions"&&(file["app"]==self.workspace.app||(self.workspace.app=="codex"&&file["app"]=="shared"));
        self.workspace.files.iter().filter(matches).find(|file|file["exists"]!=false)
            .or_else(||self.workspace.files.iter().filter(matches).next())
            .and_then(|file|file["file_path"].as_str()).map(str::to_owned)
    }
    fn instruction_paste_target(&self) -> Option<&str> {
        let state = &self.workspace;
        if self.page != super::Page::Workspace || state.section != Section::Instructions
            || state.agent_popup || state.file_loading || state.saving
        {
            return None;
        }
        let path = state.file_path.as_deref()?;
        (self.workspace_instruction_path().as_deref() == Some(path)).then_some(path)
    }
    fn open_workspace_instruction(&mut self)->Task<Message>{
        let Some(path)=self.workspace_instruction_path()else{return Task::none();};
        if self.workspace.file_path.as_ref()==Some(&path){return Task::none();}
        self.update_workspace(Event::OpenFile(path))
    }
    pub(super) fn workspace_session_popup(&self)->Element<'_,Message>{
        let id=self.workspace.active_session.as_deref().expect("visible session");
        let session=self.workspace.sessions.iter().find(|session|session["id"].as_str()==Some(id));
        let name=session.and_then(|session|session["name"].as_str()).unwrap_or("Session history");
        let log=self.workspace.logs.get(id);
        let mut turns=column![].spacing(16).width(iced::Fill);
        if let Some(log)=log{
            for turn in log.history.turns(){
                if turn.role=="Tool result"{
                    let expanded=log.expanded_tools.contains(&turn.id);
                    let name=turn.tool_name.as_deref().unwrap_or("Tool result");
                    let mut output=column![row![components::code_disclosure(name,expanded,Message::Workspace(Event::ToggleTool(id.into(),turn.id.clone()))).width(iced::Fill),components::frameless_copy(Message::Copy(format!("{name}\n\n{}",turn.text)))].spacing(12).align_y(iced::Center)].spacing(12).width(iced::Fill);
                    if expanded{output=output.push(components::code_block_without_copy(log.tool_display.get(&turn.id).unwrap_or(&turn.text).clone()));}
                    turns=turns.push(components::conversation_message(iced::widget::container(output).padding(16).width(iced::Fill),false));continue;
                }
                let body:Element<'_,Message>=if turn.code{components::code_block(turn.text.clone(),Message::Copy).into()}else{components::session_markdown(log.markdown.get(&turn.id).map(Vec::as_slice).unwrap_or_default()).map(|_|Message::Workspace(Event::IgnoreMarkdownLink))};
                turns=turns.push(components::conversation_message(iced::widget::container(column![text(&turn.role).size(14).font(theme::BOLD_FONT).color(theme::MUTED),body].spacing(8).width(iced::Fill)).padding(16).width(iced::Fill),turn.role=="User"));
            }
            if let Some(error)=&log.error{turns=turns.push(components::error_message(error.clone()));}
        }
        let content = if log.is_some_and(|log| !log.loading && log.history.turns().next().is_none() && log.error.is_none()) {
            components::empty_panel_message("No conversation messages in this session.").into()
        } else {
            surface::contain_scroll(iced::widget::scrollable(turns.padding(16)).style(theme::scrollable).height(iced::Fill))
        };
        let panel=column![row![text(name.to_owned()).size(22).font(theme::SEMIBOLD_FONT).width(iced::Fill),components::icon_button(components::Icon::Close,Some(Message::Workspace(Event::CloseSession)))].spacing(12).align_y(iced::Center),components::recessed(components::loading_panel(content,log.is_some_and(|log|log.loading))).width(iced::Fill).height(iced::Fill)].spacing(16);
        let popup=components::raised(iced::widget::container(panel).padding(24).width(iced::Fill).height(iced::Fill)).max_width(860).max_height(600);
        components::modal(popup,Message::Workspace(Event::CloseSession))
    }
    fn workspace_path(&self)->Element<'_,Message>{
        let state=&self.workspace;
        let action=state.file_path.as_ref().filter(|path|!state.file_loading&&!state.reveal_pending.contains(*path)).map(|path|Message::Workspace(Event::RevealFile(path.clone())));
        components::file_path(state.destination.clone(),action,Message::Copy(state.destination.clone()))
    }
    fn workspace_file_editor(&self,agents:bool,heading:bool)->Element<'_,Message>{
        let state=&self.workspace;let event=|event|Message::Workspace(event);
        let mut editor=column![].spacing(12).width(iced::Fill);
        if heading {
            let paste = self.instruction_paste_target().map(|_| event(Event::PasteInstructions));
            let clipboard = components::paste_and_copy(paste, Message::Copy(state.editor.text()));
            editor = editor.push(row![
                text("Edit file").size(16).font(theme::SEMIBOLD_FONT),
                Space::new().width(iced::Fill),
                clipboard,
            ].spacing(12).align_y(iced::Center));
        }
        if state.draft {
            let destination = if state.destination.is_empty() { &state.agent_directory } else { &state.destination };
            editor = editor
                .push(components::text_input("File name", &state.draft_name).on_input(|value| Message::Workspace(Event::DraftName(value))).width(iced::Fill))
                .push(text(format!("Save to {destination}")).size(12).color(theme::MUTED).wrapping(iced::widget::text::Wrapping::Glyph));
        } else {
            editor = editor.push(self.workspace_path());
        }
        if agents && state.edits_claude_metadata() {
            let preview = crate::workspace::agent_definition_preview("claude", &state.editor.text()).ok();
            let editable = preview.is_some() && !state.saving && !state.file_loading;
            for (field, key, label) in [
                (ClaudeAgentMetadataField::Name, "name", "Name"),
                (ClaudeAgentMetadataField::Description, "description", "Description"),
            ] {
                let value = preview.as_ref().and_then(|preview| preview["fields"][key].as_str()).unwrap_or_default();
                let input = components::text_input(label, value).on_input_maybe(editable.then_some(move |value| {
                    Message::Workspace(Event::EditClaudeMetadata(field, value))
                }));
                editor = editor.push(column![
                    text(label).size(12).font(theme::SEMIBOLD_FONT).color(theme::MUTED),
                    input,
                ].spacing(4));
            }
        }
        if state.file_loading{editor=editor.push(text("Reading…").size(12));}
        let editor_id = if agents { AGENT_EDITOR_ID } else { INSTRUCTION_EDITOR_ID };
        editor=editor.push(components::file_content(&state.editor,|action|Message::Workspace(Event::Edit(action)),Some(Message::Workspace(Event::Undo)),Some(editor_id)));
        if agents{
            let text_size = iced::Pixels(13.0);
            let line_height = iced::widget::text::LineHeight::default().to_absolute(text_size);
            let menu_height = if state.models.len() > 4 {
                iced::Length::Fixed(4.0 * (line_height.0 + 2.0 * theme::INPUT_PADDING))
            } else {
                iced::Shrink
            };
            let picker = pick_list(state.models.clone(), state.selected_model.clone(), |model| Message::Workspace(Event::SelectModel(model)))
                .placeholder("Model (optional)")
                .text_size(text_size)
                .padding(theme::INPUT_PADDING)
                .menu_height(menu_height)
                .style(theme::model_picker)
                .menu_style(theme::picker_menu)
                .width(iced::Fill);
            let refresh = components::text_button(if state.models_loading{"Reading models…"}else{"Refresh models"})
                .on_press_maybe((!state.models_loading).then_some(event(Event::Models)));
            editor=editor.push(row![components::input_surface(picker).width(iced::Fill),refresh].spacing(12).align_y(iced::Center));
            if state.catalog_source.as_deref()==Some("codex_cache"){editor=editor.push(text("Model list saved by Codex").size(11).color(theme::MUTED));}
        }
        let can_save=state.dirty&&!state.saving&&!state.file_loading&&!state.destination.trim().is_empty();
        let mut footer = row![components::primary_button(if state.saving{"Saving…"}else{"Save"}).style(theme::save_button).on_press_maybe(can_save.then_some(event(Event::Save)))].spacing(12).align_y(iced::Center);
        let mut details: Vec<_> = state.error.iter().cloned().collect();
        if agents {
            details.extend(state.model_error.iter().cloned());
            details.extend(state.issues.iter().map(|issue| format!("{}: {}",
                issue["field"].as_str().unwrap_or("Setting"),
                issue["message"].as_str().unwrap_or("Review this setting before saving."),
            )));
        }
        if !details.is_empty() { footer = footer.push(components::compact_error(details.join("\n"))); }
        editor=editor.push(footer);
        components::raised(iced::widget::container(editor).padding(16).width(iced::Fill)).into()
    }
    pub(super) fn workspace_delete_agent_popup(&self) -> Element<'_, Message> {
        let target = self.workspace.delete_agent.as_ref().expect("visible Delete subagent confirmation");
        let cancel = Message::Workspace(Event::CancelDeleteAgent);
        let mut actions = row![
            components::text_button("Cancel").on_press_maybe((!target.pending).then_some(cancel.clone())),
            components::primary_button(if target.pending { "Deleting…" } else { "Delete" })
                .style(|_, status| {
                    let mut style = theme::control(theme::DANGER, false, status);
                    style.text_color = iced::Color::WHITE;
                    style
                })
                .on_press_maybe((!target.pending).then_some(Message::Workspace(Event::ConfirmDeleteAgent))),
        ].spacing(12).align_y(iced::Center);
        if let Some(error) = &target.error {
            actions = actions.push(components::compact_error(error.clone()));
        }
        let body = column![
            text(format!("Delete {}?", target.name)).size(22).font(theme::SEMIBOLD_FONT),
            text("This removes the definition file from disk.").size(14).color(theme::MUTED),
            components::console_line(target.path.clone()).wrapping(iced::widget::text::Wrapping::Glyph),
            actions,
        ].spacing(16);
        let popup = components::raised(iced::widget::container(body).padding(24).width(iced::Fill)).max_width(540);
        components::modal(popup, cancel)
    }
    pub(super) fn workspace_agent_popup(&self)->Element<'_,Message>{
        let state=&self.workspace;
        let title=state.file_path.as_ref().and_then(|path|state.files.iter().find(|file|file["file_path"].as_str()==Some(path.as_str()))).and_then(|file|file["name"].as_str()).unwrap_or(if state.draft{"Add subagent"}else{"Subagent definition"});
        let paste = (!state.file_loading && !state.saving).then_some(Message::Workspace(Event::PasteAgent));
        let mut heading=row![text(title.to_owned()).size(22).font(theme::SEMIBOLD_FONT).width(iced::Fill),components::paste_and_copy(paste,Message::Copy(state.editor.text()))].spacing(12).align_y(iced::Center);
        if !state.file_loading{heading=heading.push(components::text_button(if state.agent_editing{"Preview"}else{"Edit"}).on_press_maybe((!state.saving).then_some(Message::Workspace(if state.agent_editing{Event::PreviewAgent}else{Event::EditAgent}))));}
        heading=heading.push(components::icon_button(components::Icon::Close,Some(Message::Workspace(Event::CloseAgentPopup))));
        let mut content=column![].spacing(16).width(iced::Fill);
        let body = if state.agent_editing {
            content = content.push(self.workspace_file_editor(true, false));
            surface::contain_scroll(iced::widget::scrollable(content.padding(16)).id(AGENT_EDIT_SCROLL_ID).style(theme::scrollable).height(iced::Fill))
        } else {
            if let Some(error)=&state.error{content=content.push(components::error_message(error.clone()));}
            for (key,value) in &state.agent_fields{content=content.push(column![text(key).size(12).font(theme::SEMIBOLD_FONT).color(theme::MUTED),text(value).size(14).width(iced::Fill).wrapping(iced::widget::text::Wrapping::Word)].spacing(4).width(iced::Fill));}
            if let Some(error)=&state.agent_preview_error{content=content.push(components::error_message(error.clone()));}
            if !state.agent_instructions.text().is_empty() {
                content = content.push(components::file_preview(&state.agent_instructions, |action| Message::Workspace(Event::PreviewAction(action))));
            }
            let path = iced::widget::container(self.workspace_path()).padding(16).width(iced::Fill);
            let scroll = surface::contain_scroll(
                iced::widget::scrollable(content.padding(iced::Padding { top: 0.0, right: 16.0, bottom: 16.0, left: 16.0 }))
                    .id("fetch-workspace-agent-preview")
                    .style(theme::scrollable)
                    .height(iced::Fill),
            );
            column![path, scroll].height(iced::Fill).into()
        };
        let panel=column![heading,components::recessed(components::loading_panel(body,state.file_loading)).width(iced::Fill).height(iced::Fill)].spacing(16);
        let popup=components::raised(iced::widget::container(panel).padding(24).width(iced::Fill).height(iced::Fill)).max_width(860).max_height(600);
        components::modal(popup,Message::Workspace(Event::CloseAgentPopup))
    }
    fn workspace_tabs(&self, compact: bool) -> Element<'_, Message> {
        let state=&self.workspace;
        let event=|event|Message::Workspace(event);
        let gap = if compact { 4 } else { 10 };
        let padding = if compact { [10.0, 12.0] } else { theme::BUTTON_PADDING };
        let mut tabs = row![
            components::tab("Instructions",state.section==Section::Instructions,Some(event(Event::Section(Section::Instructions)))).padding(padding),
            components::tab("Subagents",state.section==Section::Agents,Some(event(Event::Section(Section::Agents)))).padding(padding),
            components::tab("Sessions",state.section==Section::Sessions,Some(event(Event::Section(Section::Sessions)))).padding(padding),
            Space::new().width(iced::Fill),
        ].spacing(gap).align_y(iced::Center);
        if state.section == Section::Agents {
            let can_add = state.can_add_agent();
            let can_use_clipboard = can_add && self.client.is_some();
            let feedback: Element<'_, Message> = match &state.bulk_feedback {
                Some(BulkFeedback::Copied) => text("Copied all!").size(12).color(theme::MUTED).into(),
                Some(BulkFeedback::Pasted) => text("Pasted all!").size(12).color(theme::MUTED).into(),
                Some(BulkFeedback::Error(detail)) => components::compact_error(detail.clone()).into(),
                None => Space::new().into(),
            };
            tabs = tabs
                .push(iced::widget::container(feedback).width(64).align_x(iced::Right))
                .push(components::clipboard_buttons(
                    can_use_clipboard.then_some(event(Event::PasteAllAgents)),
                    (can_use_clipboard && state.agents().next().is_some()).then_some(event(Event::CopyAllAgents)),
                ).spacing(gap))
                .push(components::primary_button("Add subagent").on_press_maybe(can_add.then_some(event(Event::AddAgent))));
        }
        tabs
            .push(components::app_logo_button(include_bytes!("assets/claude-logo.svg"),state.app=="claude",event(Event::App("claude".into()))))
            .push(components::app_logo_button(include_bytes!("assets/codex-logo.svg"),state.app=="codex",event(Event::App("codex".into()))))
            .into()
    }
    pub(super) fn workspace_view(&self) -> Element<'_,Message> {
        let state=&self.workspace;
        let event=|event|Message::Workspace(event);
        let tabs: Element<'_, Message> = if state.section == Section::Agents {
            iced::widget::responsive(move |size| self.workspace_tabs(size.width < 750.0)).height(iced::Shrink).into()
        } else {
            self.workspace_tabs(false)
        };
        let mut head=column![
            row![text("Workspace").size(32).font(theme::SEMIBOLD_FONT),Space::new().width(iced::Fill),
                text(state.folder_path.as_deref().unwrap_or("Global files · all sessions")).size(12).color(theme::MUTED),
                components::refresh_button((!state.loading&&!state.sessions_loading).then_some(event(Event::Refresh)),state.loading||state.sessions_loading)].align_y(iced::Center).spacing(12),
            row![components::text_input("Project folder — leave empty for Global",&state.folder_input).on_input(|value|Message::Workspace(Event::FolderInput(value))).on_submit_maybe(Some(event(Event::OpenFolder))).width(iced::Fill),
                components::text_button("Choose folder…").on_press(event(Event::ChooseFolder))].spacing(12).align_y(iced::Center),
            tabs,
        ].spacing(14);
        if let Some(error)=self.project_registration_error(){head=head.push(components::error_message(error));}
        let mut items=column![].spacing(12).width(iced::Fill);
        let instruction_editor = state.section == Section::Instructions && state.file_path.is_some() && state.file_path == self.workspace_instruction_path();
        let error_in_editor = instruction_editor || (state.agent_popup && state.agent_editing);
        let page_error = state.error.as_ref().filter(|_| !error_in_editor);
        if let Some(error)=page_error {items=items.push(components::error_message(error.clone()));}
        let mut empty_panel = None;
        match state.section {
            Section::Instructions|Section::Agents=>{
                let agents=state.section==Section::Agents;

                if (if agents { state.agents().next().is_none() } else { self.workspace_instruction_path().is_none() }) && !state.loading {
                    let message = if agents { "No subagent definitions in this scope." } else { "No instruction files in this scope." };
                    if page_error.is_none() {
                        empty_panel = Some(message);
                    } else {
                        items = items.push(text(message).size(14).color(theme::MUTED));
                    }
                }
                if agents{for file in state.agents() {
                    let path=file["file_path"].as_str().unwrap_or("").to_owned();
                    let label=file["name"].as_str().unwrap_or(&path).to_owned();
                    let ready = self.client.is_some() && !state.loading && !state.bulk_busy && !state.file_loading && !state.saving;
                    let actions = components::agent_card_actions(
                        ready.then_some(event(Event::CopyAgent(path.clone()))),
                        (ready && state.can_add_agent()).then_some(event(Event::PasteOnAgent(path.clone()))),
                        ready.then_some(event(Event::RequestDeleteAgent(path.clone()))),
                    );
                    items=items.push(components::entry_card(label,path.clone(),(!state.file_loading&&!state.saving&&!state.bulk_busy).then_some(event(Event::OpenAgent(path.clone()))),Some(actions),true,(!state.reveal_pending.contains(&path)).then_some(event(Event::RevealFile(path.clone()))),Some(Message::Copy(path))));
                }}
                if instruction_editor {items=items.push(self.workspace_file_editor(false,true));}
            }
            Section::Sessions=>{
                if let Some(error)=&state.sessions_error{items=items.push(components::error_message(error.clone()));}
                if state.sessions.is_empty() && !state.sessions_loading && state.sessions_error.is_none() {
                    let message = "No sessions in this scope.";
                    if state.error.is_none() && state.next_cursor.is_none() {
                        empty_panel = Some(message);
                    } else {
                        items = items.push(text(message).size(14).color(theme::MUTED));
                    }
                }
                for session in &state.sessions {
                    let id=session["id"].as_str().unwrap_or("").to_owned();
                    let name=session["name"].as_str().unwrap_or("Untitled session").to_owned();
                    let project=session["project_path"].as_str().unwrap_or("No recorded project folder").to_owned();
                    items=items.push(components::entry_card(name,project,(!state.sessions_loading).then_some(event(Event::OpenSession(id))),None,true,None,None));
                }
                if state.next_cursor.is_some()&&!state.sessions_loading{items=items.push(components::text_button("More sessions").on_press(event(Event::MoreSessions)));}
            }
        }
        let body = if let Some(message) = empty_panel {
            components::empty_panel_message(message).into()
        } else {
            surface::contain_scroll(iced::widget::scrollable(items.padding(16)).height(iced::Fill).style(theme::scrollable))
        };
        column![head,components::recessed(components::loading_panel(body,state.section==Section::Sessions&&state.sessions_loading)).width(iced::Fill).height(iced::Fill)].spacing(16).height(iced::Fill).into()
    }
}
