//! Independent native view, installation and connection choices. APIs own project paths and scope changes.
use super::{components,theme,Desktop,Message};
use iced::{Element,Task,widget::{column,row,text,pick_list}};
use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Debug,PartialEq,Eq)]
pub(super) enum Choice { All,Shared,Project{id:String,name:String} }
impl std::fmt::Display for Choice {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self{Self::All=>f.write_str("All projects"),Self::Shared=>f.write_str("Available in every project"),Self::Project{name,..}=>f.write_str(name)}}
}
impl Choice {
    pub(super) fn project_id(&self)->Option<&str>{match self{Self::Project{id,..}=>Some(id),_=>None}}
    pub(super) fn query(&self)->Option<&str>{match self{Self::All=>None,Self::Shared=>Some("global"),Self::Project{id,..}=>Some(id)}}
}
pub(super) struct State {
    pub(super) records:Vec<Value>,pub(super) server_filter:Choice,pub(super) install:Choice,pub(super) connection:Choice,
    loading:bool,registering:bool,pending:BTreeSet<String>,errors:BTreeMap<String,String>,error:Option<String>,feedback:String,
}
impl Default for State {fn default()->Self{Self{records:vec![],server_filter:Choice::All,install:Choice::Shared,connection:Choice::Shared,loading:false,registering:false,pending:BTreeSet::new(),errors:BTreeMap::new(),error:None,feedback:String::new()}}}
#[derive(Clone)]
pub(super) enum Event {Loaded(Result<Value,String>),Register,Registered(Result<Value,String>),Filter(Choice),Install(Choice),Connection(Choice),Scope(String,Choice),Saved(String,Result<Value,String>)}
impl Desktop {
    pub(super) fn load_projects(&mut self)->Task<Message>{
        if self.projects.loading{return Task::none();}let Some(client)=self.client.clone()else{return Task::none();};
        self.projects.loading=true;
        Task::perform(async move{client.projects().await},|r|Message::Projects(Event::Loaded(r)))
    }
    pub(super) fn update_projects(&mut self,event:Event)->Task<Message>{match event{
        Event::Loaded(result)=>{self.projects.loading=false;match result{Ok(value)=>{self.projects.records=value["projects"].as_array().cloned().unwrap_or_default();self.projects.error=None;return self.register_workspace_project();},Err(error)=>self.projects.error=Some(error)}}
        Event::Register=>{
            if self.projects.loading||self.projects.registering{return Task::none();}
            let Some(folder)=self.workspace.folder_path.clone()else{return Task::none();};
            let Some(client)=self.client.clone()else{return Task::none();};self.projects.registering=true;self.projects.error=None;self.projects.feedback.clear();
            return Task::perform(async move{client.register_project(&folder).await},|r|Message::Projects(Event::Registered(r)));
        }
        Event::Registered(result)=>{self.projects.registering=false;match result{
            Ok(value)=>{let project=value["project"].clone();if let Some(id)=project["id"].as_str(){self.projects.records.retain(|row|row["id"].as_str()!=Some(id));self.projects.records.push(project);self.projects.feedback.clear();return self.register_workspace_project();}else{self.projects.error=Some("Fetch did not return the registered project.".into());}},Err(error)=>self.projects.error=Some(error)}}
        Event::Filter(choice)=>{
            if choice==self.projects.server_filter{return Task::none();}self.projects.server_filter=choice;
            self.auth_requests.invalidate_reads();self.polling=None;self.servers=json!({"servers":[]});self.selected_server.clear();self.tools=Value::Null;
            return self.refresh();
        }
        Event::Install(choice)=>{
            if self.busy||self.picker_window.is_some()||self.job.is_some()||self.discovery_refresh.running(){return Task::none();}
            self.projects.install=choice;self.discovery=Value::Null;self.selected.clear();self.progress=Value::Null;self.discovery_started=false;self.error=None;
        }
        Event::Connection(choice)=>{if !self.connector.creating&&!self.connector.deciding{self.projects.connection=choice;}}
        Event::Scope(id,choice)=>{
            if self.busy||self.auth_active(&id)||self.restarts.contains_key(&id)||self.projects.pending.contains(&id){return Task::none();}
            let Some(client)=self.client.clone()else{return Task::none();};
            if !self.servers["servers"].as_array().into_iter().flatten().any(|row|row["server_id"]==id){return Task::none();}
            self.projects.pending.insert(id.clone());self.projects.errors.remove(&id);
            let path=format!("/api/v1/servers/{}/config",super::component(&id));let body=json!({"scope":{"project_id":choice.project_id()}});
            return Task::perform(async move{client.request(reqwest::Method::PATCH,&path,Some(body)).await},move|r|Message::Projects(Event::Saved(id.clone(),r)));
        }
        Event::Saved(id,result)=>{self.projects.pending.remove(&id);match result{Ok(_)=>return self.refresh(),Err(error)=>{self.projects.errors.insert(id,error);}}}
    }Task::none()}
    fn project_choices(&self,all:bool)->Vec<Choice>{let mut choices=vec![];if all{choices.push(Choice::All);}choices.push(Choice::Shared);choices.extend(self.projects.records.iter().filter_map(|row|Some(Choice::Project{id:row["id"].as_str()?.into(),name:row["name"].as_str()?.into()})));choices}
    pub(super) fn project_picker(&self,label:&'static str,selected:&Choice,all:bool,enabled:bool,on_select:fn(Choice)->Event)->Element<'_,Message>{
        let picker=pick_list(self.project_choices(all),Some(selected.clone()),move|choice|Message::Projects(on_select(choice))).style(theme::model_picker).menu_style(theme::picker_menu).padding(theme::INPUT_PADDING).width(iced::Fill);
        // A disabled selector keeps the current choice visible without accepting a change.
        let control:Element<'_,Message>=if enabled&&!self.projects.loading{components::raised_with_radius(picker,12.0).width(iced::Fill).into()}else{components::raised_with_radius(iced::widget::container(text(selected.to_string())).padding(theme::INPUT_PADDING).width(iced::Fill),12.0).into()};
        let mut content=column![row![text(label).width(120),control].spacing(12).align_y(iced::Center)].spacing(8);
        if let Some(error)=&self.projects.error{content=content.push(components::error_message(error.clone()));}
        content.into()
    }
    pub(super) fn selected_connection_project(&self)->Option<&Value>{let id=self.projects.connection.project_id()?;self.projects.records.iter().find(|row|row["id"].as_str()==Some(id))}
    pub(super) fn register_workspace_project(&mut self)->Task<Message>{
        if self.workspace.loading||self.workspace.error.is_some()||self.projects.loading||self.projects.registering{return Task::none();}
        let Some(folder)=self.workspace.folder_path.as_deref()else{return Task::none();};
        if self.projects.records.iter().any(|project|project["folder_path"].as_str()==Some(folder)){return Task::none();}
        self.update_projects(Event::Register)
    }
    pub(super) fn project_registration_error(&self)->Option<&str>{self.workspace.folder_path.as_ref()?;self.projects.error.as_deref()}
    pub(super) fn server_scope_label(&self,server:&Value)->String{match server["config"]["scope"]["project_id"].as_str(){None=>"Available in every project".into(),Some(id)=>self.projects.records.iter().find(|row|row["id"]==id).and_then(|row|row["name"].as_str()).unwrap_or(id).into()}}
    pub(super) fn server_scope_editor(&self,id:&str)->Element<'_,Message>{
        let server=self.servers["servers"].as_array().into_iter().flatten().find(|row|row["server_id"].as_str()==Some(id));
        let selected=server.and_then(|row|row["config"]["scope"]["project_id"].as_str()).map(|project_id|Choice::Project{id:project_id.into(),name:server.map(|row|self.server_scope_label(row)).unwrap_or_default()}).unwrap_or(Choice::Shared);
        let enabled=!self.busy&&!self.auth_active(id)&&!self.restarts.contains_key(id)&&!self.projects.pending.contains(id);
        let target=id.to_owned();let control:Element<'_,Message>=if enabled{let picker=pick_list(self.project_choices(false),Some(selected),move|choice|Message::Projects(Event::Scope(target.clone(),choice))).style(theme::model_picker).menu_style(theme::picker_menu).padding(theme::INPUT_PADDING).width(iced::Fill);components::raised_with_radius(picker,12.0).width(iced::Fill).into()}else{text(format!("{}{}",selected,if self.projects.pending.contains(id){" · saving…"}else{""})).into()};
        let mut content=column![text("Available to"),control].spacing(10);if let Some(error)=self.projects.errors.get(id){content=content.push(components::error_message(error.clone()));}content.into()
    }
}
