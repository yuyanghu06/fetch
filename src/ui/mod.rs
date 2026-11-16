//! Native API-backed interface. No workflow reads next_steps as executable state.
pub mod client;
mod file_picker;
mod authentication;
mod auth_requests;
mod components;
mod copy_feedback;
mod refresh_feedback;
mod connector_setup;
mod workspace;
mod session_history;
mod project_scope;
#[cfg(test)]
mod project_scope_tests;
mod server_terminal;
mod theme;
mod surface;
mod scroll_gesture;
#[cfg(test)]
mod auth_tests;
#[cfg(test)]
mod neumorphic_tests;
#[cfg(test)]
mod fidelity_tests;
#[cfg(test)]
mod workspace_tests;
use components::{button, checkbox, text_input};
use authentication::{Flow as AuthFlow, Phase as AuthPhase};
use crate::{application::ApplicationHost, native_auth::NativeSettingsSession};
use client::{NativeClient, component};
use iced::{Element, Subscription, Task, window, widget::{column, row, scrollable, text}};
use reqwest::Method;
use serde_json::{Value, json};
use zeroize::Zeroize;
use std::{collections::{BTreeMap,BTreeSet}, sync::Arc, time::Duration};

#[derive(Clone,Copy,PartialEq,Eq)]
enum Page { Servers, Install, Connect, Workspace, Settings }
#[derive(Clone,Copy,PartialEq,Eq)]
enum Deletion { Pending(u64), Refreshing }
fn installable(row:&Value)->bool { row["status"]=="supported" && row["installed"]!=true }
#[derive(Clone)]
enum Message {
    Workspace(workspace::Event),
    Projects(project_scope::Event),
    Terminal(server_terminal::Message),
    Connector(connector_setup::Event),
    Page(Page), Opened(window::Id), Closed(window::Id), Escape(window::Id), DragWindow, Tick, Show, Quit, Stopped,
    Refresh, RefreshFeedbackDone(u64), DiscoveryFeedbackDone(u64), AutomaticRefresh, Servers(u64,u64,Option<window::Id>,Result<Value,String>), Discover, ConfigPath(String), ChooseConfigFile, ConfigFileChosen(window::Id,Result<Option<String>,String>), Discovery(Result<Value,String>),
    Select(String,String,bool), SelectAllServers, Install, Installed(Result<Value,String>), Progress(Result<Value,String>),
    Inspect(String), Tools(String,Option<window::Id>,Result<Value,String>), Authenticate(String), Restart(String,bool), Delete(String), Deleted(u64,String,Result<Value,String>),
    AuthFinished(u64,String,Result<Value,String>), AuthDismiss(String), AuthOpenSignIn(String),
    CredentialFinished(u64,String,bool,Result<Value,String>), AuthName(String,String), AuthValue(String,String), AuthSave(String),
    OpenFolder(String), FolderOpened(String,Option<window::Id>,Result<Value,String>),
    PowerClicked(String,bool), PowerFinished(String,bool,Result<Value,String>),
    Changed(String,Result<Value,String>), RestartStatus(String,Result<Value,String>),
    Choose(String), Timeout(String), Concurrency(String), StepTimeout(i64), StepConcurrency(i64), SaveExecution,
    ScaleFactor(window::Id,f32), Modifiers(iced::keyboard::Modifiers), RestartClicked(String), CloseDiscovery, OpenDiscovery,
    SecretName(String), SecretValue(String), SaveSecret, RemoveSecret,
    Copy(String), ToggleServer(String), ToggleTool(String),
}
impl std::fmt::Debug for Message { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("FetchNativeMessage") } }
fn window_settings()->window::Settings{
    window::Settings{size:iced::Size::new(1058.0,789.0),min_size:Some(iced::Size::new(760.0,520.0)),decorations:false,transparent:true,..Default::default()}
}
/// Window-wide shortcuts also receive keys captured by an editor or control.
fn subscription_event(event:iced::Event,_status:iced::event::Status,id:window::Id)->Option<Message>{
    match event{
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed{key:iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),..})=>Some(Message::Escape(id)),
        iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(value))=>Some(Message::Modifiers(value)),
        iced::Event::Window(window::Event::Rescaled(value))=>Some(Message::ScaleFactor(id,value)),
        _=>None,
    }
}
struct Desktop {
    workspace: workspace::State,
    projects: project_scope::State,
    connector: connector_setup::State,
    host: Arc<ApplicationHost>, client: Option<NativeClient>, session: Option<Arc<NativeSettingsSession>>, window:Option<window::Id>,
    page:Page, servers:Value, discovery:Value, tools:Value, selected:BTreeSet<(String,String)>,
    config_path:String, discovery_started:bool, picker_window:Option<window::Id>, selected_server:String, timeout:String, concurrency:String, secret_name:String, secret_value:String,
    job:Option<String>, progress:Value, restarts:BTreeMap<String,String>, error:Option<String>, notice:String,
    busy:bool, server_refresh:refresh_feedback::RefreshFeedback, discovery_refresh:refresh_feedback::RefreshFeedback, reload_generation:u64, restart_submitting:BTreeSet<String>, restart_failures:BTreeMap<String,String>, polling:Option<u64>, installation_polling:bool, restart_polling:BTreeSet<String>, expanded_servers:BTreeSet<String>, expanded_tools:BTreeSet<String>,
    authentication:BTreeMap<String,AuthFlow>, auth_requests:auth_requests::Requests,
    power_pending:BTreeMap<String,bool>, power_errors:BTreeMap<String,String>,
    deletions:BTreeMap<String,Deletion>, opening_folders:BTreeSet<String>,
    terminals:BTreeMap<String,server_terminal::State>,
    modifiers:iced::keyboard::Modifiers, scale_factor:f32,
}
impl Desktop {
    fn boot(host:Arc<ApplicationHost>)->(Self,Task<Message>) {
        let session=Arc::new(host.native.open_settings());
        let client=NativeClient::new(host.state.origin.clone(),&host.certificate_pem,session.clone());
        let (id,open)=window::open(window_settings());
        let mut state=Self {projects:project_scope::State::default(),workspace:workspace::State::default(),connector:connector_setup::State::default(),host,client:None,session:Some(session),window:Some(id),page:Page::Servers,servers:json!({"servers":[]}),discovery:Value::Null,tools:Value::Null,selected:BTreeSet::new(),config_path:String::new(),discovery_started:false,picker_window:None,selected_server:String::new(),timeout:"120".into(),concurrency:"1".into(),secret_name:String::new(),secret_value:String::new(),job:None,progress:Value::Null,restarts:BTreeMap::new(),error:None,notice:String::new(),busy:false,server_refresh:refresh_feedback::RefreshFeedback::default(),discovery_refresh:refresh_feedback::RefreshFeedback::default(),reload_generation:0,restart_submitting:BTreeSet::new(),restart_failures:BTreeMap::new(),polling:None,installation_polling:false,restart_polling:BTreeSet::new(),expanded_servers:BTreeSet::new(),expanded_tools:BTreeSet::new(),authentication:BTreeMap::new(),auth_requests:auth_requests::Requests::default(),power_pending:BTreeMap::new(),power_errors:BTreeMap::new(),deletions:BTreeMap::new(),opening_folders:BTreeSet::new(),terminals:BTreeMap::new(),modifiers:iced::keyboard::Modifiers::default(),scale_factor:1.0};
        match client {Ok(client)=>state.client=Some(client),Err(error)=>state.error=Some(error)}
        let read=state.refresh();
        let projects=state.load_projects();
        (state,Task::batch([open.map(Message::Opened),read,projects]))
    }
    fn refresh(&mut self)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        let (revision,sequence)=self.auth_requests.begin_read();let window=self.window;
        let project=self.projects.server_filter.query().map(str::to_owned);
        Task::perform(async move{client.servers_for(project.as_deref()).await},move|result|Message::Servers(revision,sequence,window,result))
    }
    fn apply_server_record(&mut self,id:&str,incoming:&Value){
        if incoming["server_id"].as_str().is_some_and(|returned|returned!=id){return;}
        if let Some(record)=self.servers["servers"].as_array_mut().and_then(|rows|rows.iter_mut().find(|row|row["server_id"].as_str()==Some(id))){
            if let (Some(target),Some(fields))=(record.as_object_mut(),incoming.as_object()){target.extend(fields.iter().map(|(key,value)|(key.clone(),value.clone())));}
        }
        if incoming["authentication"].is_object(){
            self.authentication.entry(id.to_owned()).and_modify(|flow|flow.observe(&incoming["authentication"])).or_insert_with(||AuthFlow::passive(&incoming["authentication"]));
        }
    }
    fn subscription(&self)->Subscription<Message> {
        Subscription::batch([iced::time::every(Duration::from_secs(1)).map(|_|Message::Tick),iced::time::every(Duration::from_secs(60)).map(|_|Message::AutomaticRefresh),window::close_events().map(Message::Closed),iced::event::listen_with(subscription_event),self.terminal_subscription()])
    }
    fn operation(&mut self,id:String,method:Method,suffix:&str,body:Value)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        if self.busy||self.auth_active(&id){return Task::none();} self.busy=true;self.error=None;
        let path=format!("/api/v1/servers/{}{suffix}",component(&id));
        Task::perform(async move{client.request(method,&path,Some(body)).await},move |result|Message::Changed(id.clone(),result))
    }
    fn auth_active(&self,id:&str)->bool {
        self.deletions.contains_key(id) || self.power_pending.contains_key(id) || self.auth_requests.contains(id) || self.authentication.get(id).is_some_and(|flow|flow.active()||flow.reconcile) || self.servers["servers"].as_array().into_iter().flatten()
            .any(|server|server["server_id"].as_str()==Some(id)&&server["authentication"]["status"]=="authenticating")
    }
    fn can_delete(&self,id:&str)->bool {
        if self.busy||self.job.is_some()||self.auth_active(id)||self.restarts.contains_key(id){return false;}
        self.servers["servers"].as_array().into_iter().flatten().find(|server|server["server_id"].as_str()==Some(id)).is_some_and(|server|{
            !matches!(server["runtime"]["status"].as_str(),Some("starting"|"restarting"))
                &&server["restart"]["status"]!="restarting"
                &&["active_calls","queued_calls","uncertain_calls"].into_iter().all(|key|server["tool_execution"][key].as_u64().unwrap_or(0)==0)
                &&server["unresolved_prior_calls"].as_u64().unwrap_or(0)==0
        })
    }
    fn delete_server(&mut self,id:String)->Task<Message> {
        if !self.can_delete(&id){return Task::none();}
        let Some(client)=self.client.clone() else{return Task::none();};
        let Some(token)=self.auth_requests.begin(&id) else{return Task::none();};
        self.deletions.insert(id.clone(),Deletion::Pending(token));
        self.error=None;self.notice.clear();
        let path=format!("/api/v1/servers/{}",component(&id));
        Task::perform(async move{client.request(Method::DELETE,&path,None).await},move|result|Message::Deleted(token,id.clone(),result))
    }
    fn forget_server(&mut self,id:&str) {
        if let Some(rows)=self.servers["servers"].as_array_mut(){rows.retain(|row|row["server_id"].as_str()!=Some(id));}
        if let Some(origins)=self.tools.get_mut("origins").and_then(Value::as_array_mut){origins.retain(|origin|origin["source"]["server_id"].as_str()!=Some(id));if origins.is_empty(){self.tools=Value::Null;}}
        self.authentication.remove(id);
        self.terminals.remove(id);
        self.opening_folders.remove(id);
        self.expanded_servers.remove(id);self.expanded_tools.retain(|key|!key.starts_with(&format!("{id}:")));
        self.power_pending.remove(id);self.power_errors.remove(id);self.restarts.remove(id);self.restart_failures.remove(id);self.restart_submitting.remove(id);self.restart_polling.remove(id);
        if self.selected_server==id{self.secret_value.zeroize();self.secret_name.clear();self.selected_server.clear();self.timeout="120".into();self.concurrency="1".into();}
        if let Some(groups)=self.discovery["configs"].as_array_mut(){for group in groups{
            let path=group["path"].as_str().unwrap_or("").to_owned();
            if let Some(rows)=group["servers"].as_array_mut(){for row in rows{if row["installed_server_id"].as_str()==Some(id){
                if let Some(name)=row["name"].as_str(){self.selected.remove(&(path.clone(),name.to_owned()));}
                row["installed"]=json!(false);row["installed_server_id"]=Value::Null;row["selected_by_default"]=json!(false);
            }}}
        }}
        if let Some(rows)=self.progress["servers"].as_array_mut(){rows.retain(|row|row["server_id"].as_str()!=Some(id));}
    }
    fn clear_secrets(&mut self) {
        self.secret_value.zeroize();
        for flow in self.authentication.values_mut(){flow.secret.zeroize();}
    }
    fn clear_other_secrets(&mut self,id:&str) {
        self.secret_value.zeroize();
        for (server,flow) in &mut self.authentication{if server!=id{flow.secret.zeroize();}}
    }
    fn close_authentication(&mut self) {
        self.clear_secrets();self.authentication.clear();self.auth_requests.clear();self.opening_folders.clear();
        self.polling=None;self.installation_polling=false;self.restart_polling.clear();
    }
    fn auth_operation(&mut self,id:String)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        if self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
        let Some(token)=self.auth_requests.begin(&id) else{return Task::none();};
        self.clear_secrets();self.authentication.entry(id.clone()).or_insert_with(||AuthFlow::passive(&Value::Null)).begin_authentication();
        self.expanded_servers.insert(id.clone());
        let path=format!("/api/v1/servers/{}/restart",component(&id));
        Task::perform(async move{client.request(Method::POST,&path,Some(json!({"interactive":true}))).await},move|result|Message::AuthFinished(token,id.clone(),result))
    }
    fn can_open_sign_in(&self,id:&str)->bool {
        !self.busy && !self.deletions.contains_key(id) && !self.restarts.contains_key(id)
            && !self.auth_requests.contains(id)
            && self.authentication.get(id).is_some_and(|flow|flow.phase==AuthPhase::Pending
                && flow.sign_in.as_ref().is_some_and(|sign_in|!sign_in.opened))
    }
    fn open_sign_in(&mut self,id:String)->Task<Message> {
        if !self.can_open_sign_in(&id){return Task::none();}
        let Some(client)=self.client.clone() else{return Task::none();};
        let Some(token)=self.auth_requests.begin(&id) else{return Task::none();};
        let path=format!("/api/v1/servers/{}/restart",component(&id));
        Task::perform(async move{client.request(Method::POST,&path,Some(json!({"interactive":true}))).await},move|result|Message::AuthFinished(token,id.clone(),result))
    }
    fn submit_credential_request(&mut self,id:String,body:Value,settings:bool)->Task<Message>{
        let Some(client)=self.client.clone() else{return Task::none();};
        let Some(token)=self.auth_requests.begin(&id) else{return Task::none();};
        self.expanded_servers.insert(id.clone());
        let path=format!("/api/v1/servers/{}/credentials",component(&id));
        Task::perform(async move{client.request(Method::PATCH,&path,Some(body)).await},move|result|Message::CredentialFinished(token,id.clone(),settings,result))
    }
    fn save_inline_key(&mut self,id:String)->Task<Message>{
        if self.client.is_none()||self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
        let Some(flow)=self.authentication.get_mut(&id)else{return Task::none();};
        if flow.entry.trim().is_empty()||flow.secret.is_empty(){flow.error=Some("Choose an assigned key entry and enter its value.".into());flow.record_state();return Task::none();}
        let body=authentication::manual_patch(flow.entry.clone(),Some(std::mem::take(&mut flow.secret)));
        flow.phase=AuthPhase::Saving;flow.storage_result=true;flow.reconcile=false;flow.error=None;flow.record_state();
        self.submit_credential_request(id,body,false)
    }
    fn settings_credential_operation(&mut self,remove:bool)->Task<Message>{
        let id=self.selected_server.clone();
        if self.client.is_none()||self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
        if id.is_empty()||self.secret_name.trim().is_empty()||(!remove&&self.secret_value.is_empty()){self.error=Some("Choose an assigned key entry and enter its value.".into());return Task::none();}
        let body=authentication::manual_patch(self.secret_name.clone(),(!remove).then(||std::mem::take(&mut self.secret_value)));
        self.secret_value.zeroize();
        let flow=self.authentication.entry(id.clone()).or_insert_with(||AuthFlow::passive(&Value::Null));
        flow.begin_authentication();flow.phase=AuthPhase::Saving;flow.storage_result=true;flow.entry=self.secret_name.clone();flow.record_state();
        self.error=None;self.notice.clear();
        self.submit_credential_request(id,body,true)
    }
    fn open_server_folder(&mut self,id:String)->Task<Message> {
        if self.deletions.contains_key(&id)||self.opening_folders.contains(&id)||!self.servers["servers"].as_array().into_iter().flatten().any(|server|server["server_id"].as_str()==Some(id.as_str())){return Task::none();}
        let Some(client)=self.client.clone() else{return Task::none();};
        self.opening_folders.insert(id.clone());self.error=None;
        let window=self.window;
        let target=id.clone();
        Task::perform(async move{client.open_server_folder(&target).await},move|result|Message::FolderOpened(id.clone(),window,result))
    }
    fn update(&mut self,message:Message)->Task<Message> {
        match message {
            Message::Workspace(event)=>return self.update_workspace(event),
            Message::Projects(event)=>return self.update_projects(event),
            Message::Terminal(message)=>return self.update_terminal(message),
            Message::Connector(event)=>return self.update_connector(event),
            Message::Escape(id)=>{
                if self.window!=Some(id){return Task::none();}
                if self.connector.popup_visible(){return self.update_connector(connector_setup::Event::Dismiss);}
                if self.page==Page::Workspace{
                    if self.workspace.agent_popup{return self.update_workspace(workspace::Event::CloseAgentPopup);}
                    if self.workspace.active_session.is_some(){return self.update_workspace(workspace::Event::CloseSession);}
                }
                if self.page==Page::Install&&self.discovery_started{return self.update(Message::CloseDiscovery);}
            }
            Message::Page(page)=>{if self.page!=page{self.clear_secrets();}self.page=page;self.error=None;if page==Page::Workspace{return self.load_workspace();}}
            Message::DragWindow=>{if let Some(id)=self.window{return window::drag(id);}}
            Message::Opened(id)=>{file_picker::prepare();self.window=Some(id);crate::native_auth::install_app_menu();return Task::batch([window::run(id,crate::native_auth::configure_window).discard(),window::scale_factor(id).map(move|value|Message::ScaleFactor(id,value))]);}
            Message::Closed(id)=>{if self.window==Some(id){self.server_refresh=refresh_feedback::RefreshFeedback::default();self.discovery_refresh=refresh_feedback::RefreshFeedback::default();self.restart_submitting.clear();self.power_pending.clear();self.window=None;self.picker_window=None;file_picker::cancel();self.host.native.invalidate_all();self.client=None;self.session=None;self.connector.window_closed();self.pause_terminals();self.close_authentication();}}
            Message::Show=>{
                self.connector.foreground_request();
                if let Some(id)=self.window{return window::gain_focus(id);}
                if self.window.is_none(){let session=Arc::new(self.host.native.open_settings());match NativeClient::new(self.host.state.origin.clone(),&self.host.certificate_pem,session.clone()){Ok(client)=>self.client=Some(client),Err(error)=>self.error=Some(error)}self.session=Some(session);let(id,open)=window::open(window_settings());self.window=Some(id);self.resume_terminals();return Task::batch([open.map(Message::Opened),self.refresh(),self.load_projects()]);}
            }
            Message::Quit=>{self.connector=connector_setup::State::default();self.picker_window=None;file_picker::cancel();self.close_authentication();self.client=None;self.session=None;let host=self.host.clone();return Task::perform(async move{host.shutdown().await;},|_|Message::Stopped);}
            Message::Stopped=>return iced::exit(),
            Message::Refresh=>{
                if self.client.is_none()||self.server_refresh.running(){return Task::none();}
                let read=self.refresh();let sequence=self.auth_requests.latest_sequence();
                let feedback=self.server_refresh.begin(sequence,Duration::from_millis(1500)).map(Message::RefreshFeedbackDone);
                return Task::batch([read,feedback]);
            },
            Message::RefreshFeedbackDone(sequence)=>self.server_refresh.elapsed(sequence),
            Message::DiscoveryFeedbackDone(sequence)=>self.discovery_refresh.elapsed(sequence),
            Message::ScaleFactor(id,value)=>{if self.window==Some(id){self.scale_factor=value;}},
            Message::Modifiers(value)=>self.modifiers=value,
            Message::RestartClicked(id)=>return self.update(Message::Restart(id,self.modifiers.shift())),
            Message::CloseDiscovery=>self.discovery_started=false,
            Message::OpenDiscovery=>self.discovery_started=true,
            Message::AutomaticRefresh=>{
                if self.client.is_none()||self.polling.is_some(){return Task::none();}
                let read=self.refresh();self.polling=Some(self.auth_requests.latest_sequence());return read;
            }
            Message::Servers(revision,sequence,window,result)=>{
                if window!=self.window{return Task::none();}
                if self.polling==Some(sequence){self.polling=None;}
                let manual=self.server_refresh.pending==Some(sequence);
                self.server_refresh.finish(sequence);
                if revision!=self.auth_requests.revision(){
                    let read=self.refresh();if manual{self.server_refresh.pending=Some(self.auth_requests.latest_sequence());}return read;
                }
                if !self.auth_requests.accept_read(revision,sequence){return Task::none();}
                match result {Ok(value)=>{
                    if manual{self.reload_generation+=1;}
                    if let Some(servers)=value["servers"].as_array(){for server in servers{
                        if let Some(id)=server["server_id"].as_str(){
                            // Every server has passive console state. A read can
                            // restore logs/consent without starting authentication.
                            self.authentication.entry(id.to_owned())
                                .and_modify(|flow|flow.observe(&server["authentication"]))
                                .or_insert_with(||AuthFlow::passive(&server["authentication"]));
                        }
                    }}
                    self.servers=value;
                    if self.servers["servers"].is_array(){
                        let refreshed:Vec<_>=self.deletions.iter().filter_map(|(id,state)|(*state==Deletion::Refreshing).then_some(id.clone())).collect();
                        for id in refreshed{self.deletions.remove(&id);if !self.servers["servers"].as_array().unwrap().iter().any(|row|row["server_id"].as_str()==Some(id.as_str())){self.forget_server(&id);}}
                    }
                },Err(error)=>{
                    let mut inline=false;
                    for flow in self.authentication.values_mut().filter(|flow|flow.reconcile){inline=true;flow.read_error=Some(format!("Could not read sign-in status: {error} Fetch will keep checking; do not start another login."));flow.record_state();}
                    if !inline{self.error=Some(error);}
                }}
            }
            Message::ConfigPath(value)=>self.config_path=value,
            Message::ChooseConfigFile=>{
                if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}
                let Some(id)=self.window else{return Task::none();};
                self.picker_window=Some(id);self.error=None;
                return Task::perform(file_picker::choose_config_file(),move|result|Message::ConfigFileChosen(id,result));
            }
            Message::ConfigFileChosen(id,result)=>{
                if self.window!=Some(id)||self.picker_window!=Some(id){return Task::none();}
                self.picker_window=None;
                match result{Ok(Some(path))=>{self.config_path=path;self.discovery_refresh.minimum=None;return self.update(Message::Discover);},Ok(None)=>{},Err(error)=>self.error=Some(error)}
            }
            Message::Discover=>{if (self.config_path.trim().is_empty()&&self.projects.install.project_id().is_none())||self.busy||self.discovery_refresh.running()||self.picker_window.is_some()||self.job.is_some(){return Task::none();}if let Some(client)=self.client.clone(){self.busy=true;self.discovery_started=true;let path=self.config_path.clone();let project=self.projects.install.project_id().map(str::to_owned);let feedback=self.discovery_refresh.begin(self.discovery_refresh.generation+1,Duration::from_secs(3)).map(Message::DiscoveryFeedbackDone);return Task::batch([Task::perform(async move{client.discovery_for(&path,project.as_deref()).await},Message::Discovery),feedback]);}}
            Message::Discovery(result)=>{self.busy=false;self.discovery_refresh.pending=None;match result {Ok(value)=>{
                self.discovery=value;
                // A discovery read may finish after an in-flight installation succeeds.
                self.mark_installed_from_progress(&self.progress.clone());
                self.selected.clear();
                if let Some(groups)=self.discovery["configs"].as_array(){for group in groups {if let Some(rows)=group["servers"].as_array(){for item in rows{if installable(item)&&item["selected_by_default"]==true{if let(Some(path),Some(name))=(group["path"].as_str(),item["name"].as_str()){self.selected.insert((path.into(),name.into()));}}}}}}
            },Err(error)=>self.error=Some(error)}}
            Message::Select(path,name,selected)=>{let key=(path,name);if selected&&self.supported_selections().contains(&key){self.selected.insert(key);}else{self.selected.remove(&key);}}
            Message::SelectAllServers=>{
                if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}
                let eligible=self.supported_selections();
                if eligible.is_subset(&self.selected){self.selected.clear();}else{self.selected=eligible;}
            }
            Message::Install=>{let rows=self.selected_for_install();if self.busy||self.picker_window.is_some()||self.job.is_some()||rows.is_empty(){return Task::none();}if let Some(client)=self.client.clone(){self.notice.clear();self.busy=true;let project=self.projects.install.project_id().map(str::to_owned);return Task::perform(async move{client.install_for(rows,project.as_deref()).await},Message::Installed);}}
            Message::Installed(result)=>{self.busy=false;match result {Ok(value)=>{self.job=value["installation_id"].as_str().map(str::to_owned);let existing=value["error"]["code"]=="INSTALLATION_IN_PROGRESS";self.progress=value;self.notice=if existing{"An installation is already running. Monitoring that existing operation.".into()}else{String::new()};},Err(error)=>self.error=Some(error)}}
            Message::Progress(result)=>{self.installation_polling=false;match result {Ok(value)=>{self.mark_installed_from_progress(&value);let terminal=matches!(value["status"].as_str(),Some("succeeded"|"failed"));if terminal {
                    self.selected.clear();
                    let eligible=self.supported_selections();
                    if let Some(rows)=value["servers"].as_array(){for item in rows {if item["status"]=="failed" {if let (Some(path),Some(name))=(item["config_path"].as_str(),item["server_name"].as_str()){let key=(path.to_owned(),name.to_owned());if eligible.contains(&key){self.selected.insert(key);}}}}}
                }
                self.progress=value;if terminal{self.job=None;return self.refresh();}},Err(error)=>self.error=Some(error)}}
            Message::Inspect(id)=>{
                if self.deletions.contains_key(&id){return Task::none();}
                self.clear_secrets(); self.expanded_servers.insert(id.clone());
                if !self.select_server_panel(&id,server_terminal::Pane::Tools){return Task::none();}
                if let Some(client)=self.client.clone(){let query=url::form_urlencoded::Serializer::new(String::new()).append_pair("server_id",&id).finish();let window=self.window;return Task::perform(async move{client.request(Method::GET,&format!("/api/v1/tools?{query}"),None).await},move|result|Message::Tools(id.clone(),window,result));}
            }
            Message::Tools(id,window,result)=>{
                if window!=self.window||self.deletions.contains_key(&id)||!self.servers["servers"].as_array().into_iter().flatten().any(|row|row["server_id"].as_str()==Some(id.as_str())){return Task::none();}
                match result {Ok(value)=>{
                    if !self.tools["origins"].is_array(){self.tools=json!({"origins":[]});}
                    let origins=self.tools["origins"].as_array_mut().unwrap();
                    origins.retain(|origin|origin["source"]["server_id"].as_str()!=Some(id.as_str()));
                    if let Some(incoming)=value["origins"].as_array(){origins.extend(incoming.iter().filter(|origin|origin["source"]["server_id"].as_str()==Some(id.as_str())).cloned());}
                },Err(error)=>self.error=Some(error)}
            }
            Message::Delete(id)=>return self.delete_server(id),
            Message::Deleted(token,id,result)=>{
                if self.deletions.get(&id)!=Some(&Deletion::Pending(token)){return Task::none();}
                self.auth_requests.finish(&id,token);self.auth_requests.invalidate_reads();self.polling=None;
                match result{
                    Ok(value) if value["status"]=="deleted"&&value["server_id"].as_str()==Some(id.as_str())=>{self.deletions.remove(&id);self.forget_server(&id);self.notice="Server registration removed. Its files and credentials were preserved.".into();}
                    result=>{self.deletions.insert(id,Deletion::Refreshing);let error=match result{Err(error)=>error,Ok(_)=>"The server did not confirm deletion.".into()};self.error=Some(format!("{error} Refreshing server status before another delete attempt; deletion will not be retried automatically."));}
                }
                return self.refresh();
            }
            Message::Authenticate(id)=>return self.auth_operation(id),
            Message::AuthOpenSignIn(id)=>return self.open_sign_in(id),
            Message::AuthFinished(token,id,result)=>{
                if !self.auth_requests.finish(&id,token){return Task::none();}
                if let Ok(value)=&result{
                    if matches!(value["status"].as_str(),Some("authenticated"|"not_required"|"required"|"unknown"|"authenticating")){
                        if let Some(record)=self.servers["servers"].as_array_mut().and_then(|rows|rows.iter_mut().find(|row|row["server_id"].as_str()==Some(id.as_str()))){record["authentication"]["status"]=value["status"].clone();record["authentication"]["error"]=Value::Null;}
                    }
                }
                if let Some(flow)=self.authentication.get_mut(&id){match result{
                    Ok(value)=>flow.result(&value),
                    Err(error)=>{flow.phase=AuthPhase::Failed;flow.error=Some(error);flow.reconcile=true;flow.record_state();}
                }}
                return self.refresh();
            }
            Message::CredentialFinished(token,id,settings,result)=>{
                if !self.auth_requests.finish(&id,token){return Task::none();}
                if let Some(flow)=self.authentication.get_mut(&id){match &result{
                    Ok(value)=>flow.stored(value),
                    Err(error)=>{flow.phase=AuthPhase::Failed;flow.storage_result=true;flow.reconcile=false;flow.secret.zeroize();flow.error=Some(error.clone());flow.record_state();}
                }}
                if settings&&self.page==Page::Servers&&self.selected_server==id{match result{
                    Ok(value) if value["status"]=="stored"&&value["verified"]==false=>self.notice="Manual key changes saved in macOS Keychain. Account access has not been verified.".into(),
                    Ok(_)=>self.error=Some("Fetch did not confirm storing the manual key.".into()),
                    Err(error)=>self.error=Some(error),
                }}
                return self.refresh();
            }
            Message::AuthName(id,value)=>{self.clear_other_secrets(&id);if let Some(flow)=self.authentication.get_mut(&id).filter(|flow|!flow.active()){flow.secret.zeroize();flow.entry=value;}}
            Message::AuthValue(id,mut value)=>{self.clear_other_secrets(&id);if let Some(flow)=self.authentication.get_mut(&id).filter(|flow|!flow.active()){flow.secret.zeroize();flow.secret=std::mem::take(&mut value);}value.zeroize();}
            Message::AuthSave(id)=>return self.save_inline_key(id),
            Message::AuthDismiss(id)=>{if self.authentication.get(&id).is_some_and(|flow|flow.active()||flow.reconcile){return Task::none();}self.authentication.remove(&id);}

            Message::PowerClicked(id,enabled)=>{
                if self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
                let Some(client)=self.client.clone()else{return Task::none();};
                self.power_pending.insert(id.clone(),enabled);self.power_errors.remove(&id);self.auth_requests.invalidate_reads();
                let path=format!("/api/v1/servers/{}/restart",component(&id));
                return Task::perform(async move{client.request(Method::POST,&path,Some(if enabled{json!({})}else{json!({"action":"stop"})})).await},move|result|Message::PowerFinished(id.clone(),enabled,result));
            }
            Message::PowerFinished(id,enabled,result)=>{
                if self.power_pending.get(&id)!=Some(&enabled){return Task::none();}
                self.auth_requests.invalidate_reads();
                match result{
                    Ok(value)=>{
                        if let Some(record)=self.servers["servers"].as_array_mut().and_then(|rows|rows.iter_mut().find(|row|row["server_id"].as_str()==Some(id.as_str()))){
                            record["config"]["enabled"]=json!(enabled);
                            if !enabled{record["runtime"]=json!({"status":"stopped","error":null});record["restart"]=Value::Null;}
                        }
                        if let Some(restart)=value["restart_id"].as_str(){self.restarts.insert(id.clone(),restart.into());}
                        else{self.power_pending.remove(&id);}
                    }
                    Err(error)=>{self.power_pending.remove(&id);self.power_errors.insert(id.clone(),error);self.expanded_servers.insert(id.clone());self.select_server_panel(&id,server_terminal::Pane::Console);}
                }
                return self.refresh();
            }
            Message::Restart(id,force)=>{
                if self.client.is_none()||self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
                self.restart_submitting.insert(id.clone());self.restart_failures.remove(&id);self.notice.clear();
                return self.operation(id,Method::POST,"/restart",json!({"force_restart":force}));
            },
            Message::Changed(id,result)=>{self.busy=false;let restarting=self.restart_submitting.remove(&id);match result{Ok(value)=>{if let Some(restart)=value["restart_id"].as_str(){self.restarts.insert(id,restart.into());}self.notice=if restarting{String::new()}else if value["status"]=="authenticating"{"Complete the provider's browser sign-in, then Fetch will show the result.".into()}else{"Saved. Review server state for any required next action.".into()};return self.refresh();},Err(error)=>{if restarting{self.restart_failures.insert(id,error.clone());}self.error=Some(error);return self.refresh();}}}
            Message::RestartStatus(id,result)=>{self.restart_polling.remove(&id);match result{Ok(value)=>{
                if let Some(record)=value["servers"].as_array().and_then(|records|records.first()){self.apply_server_record(&id,record);}
                let restart=value["servers"].as_array().and_then(|records|records.first()).map(|record|&record["restart"]);if let Some(restart)=restart{if matches!(restart["status"].as_str(),Some("succeeded"|"failed")){
                    if let Some(record)=self.servers["servers"].as_array_mut().and_then(|rows|rows.iter_mut().find(|row|row["server_id"].as_str()==Some(id.as_str()))){record["restart"]=restart.clone();}
                    if self.power_pending.remove(&id).is_some(){self.auth_requests.invalidate_reads();if restart["status"]=="failed"{self.expanded_servers.insert(id.clone());self.select_server_panel(&id,server_terminal::Pane::Console);}}
                    self.restarts.remove(&id);self.notice.clear();return self.refresh();}}},Err(error)=>{self.error=Some(error);}}}
            Message::Choose(id)=>{if self.deletions.contains_key(&id){return Task::none();}self.clear_secrets();for (server,state) in self.terminals.iter_mut(){if server!=&id{state.hide_settings();}}self.selected_server=id;self.secret_name.clear();self.page=Page::Servers;self.expanded_servers.insert(self.selected_server.clone());self.select_server_panel(&self.selected_server.clone(),server_terminal::Pane::Settings);
                if let Some(record)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|row|row["server_id"].as_str()==Some(self.selected_server.as_str()))){self.timeout=(record["config"]["execution"]["default_timeout_ms"].as_u64().unwrap_or(120000)/1000).to_string();self.concurrency=record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1).to_string();}
            }
            Message::Timeout(value)=>self.timeout=value,
            Message::Concurrency(value)=>self.concurrency=value,
            Message::StepTimeout(step)=>{if let Ok(value)=self.timeout.parse::<i64>(){self.timeout=(value.saturating_add(step).clamp(1,86400)).to_string();}}
            Message::StepConcurrency(step)=>{if let Ok(value)=self.concurrency.parse::<i64>(){self.concurrency=(value.saturating_add(step).clamp(1,64)).to_string();}}
            Message::SaveExecution=>{
                let values=self.timeout.parse::<u64>().ok().and_then(|seconds|seconds.checked_mul(1000)).zip(self.concurrency.parse::<u64>().ok());
                match values{Some((timeout,concurrency)) if timeout>0&&timeout<=86400000&&concurrency>0&&concurrency<=64=>return self.operation(self.selected_server.clone(),Method::PATCH,"/config",json!({"execution":{"default_timeout_ms":timeout,"max_concurrent_calls":concurrency}})),_=>self.error=Some("Enter 1–86400 seconds and 1–64 concurrent calls.".into())}
            }
            Message::SecretName(value)=>{self.secret_value.zeroize();self.secret_name=value;}
            Message::SecretValue(mut value)=>{self.secret_value.zeroize();self.secret_value=std::mem::take(&mut value);value.zeroize();}
            Message::SaveSecret|Message::RemoveSecret=>return self.settings_credential_operation(matches!(message,Message::RemoveSecret)),
            Message::OpenFolder(id)=>return self.open_server_folder(id),
            Message::FolderOpened(id,window,result)=>{
                if window!=self.window||!self.opening_folders.remove(&id){return Task::none();}
                match result{
                    Ok(value) if value["status"]=="opened"&&value["server_id"].as_str()==Some(id.as_str())=>self.notice="Server folder open request completed.".into(),
                    Ok(_)=>self.error=Some("Fetch did not confirm opening the server folder.".into()),
                    Err(error)=>self.error=Some(error),
                }
            }
            Message::Copy(value)=>return iced::clipboard::write(value),
            Message::ToggleServer(id)=>{self.clear_secrets();if !self.expanded_servers.remove(&id){self.show_server_console(&id);self.expanded_servers.insert(id);}}
            Message::ToggleTool(id)=>{if !self.expanded_tools.remove(&id){self.expanded_tools.insert(id);}}
            Message::Tick=>{
                if self.host.is_stopped(){return iced::exit();}
                match crate::native_auth::take_app_action(){1=>return self.update(Message::Show),2=>return self.update(Message::Quit),_=>{}}
                if self.host.state.oauth.as_ref().is_some_and(|service| service.take_connection_request()) { return self.update(Message::Show); }
                if self.host.take_show_request(){return self.update(Message::Show);}
                if let Some(context)=self.host.take_connection_context(){return self.update_connector(connector_setup::Event::AutomaticContext(context));}
                let Some(client)=self.client.clone() else{return Task::none();};
                let mut reads=Vec::new();
                if let Some(read)=self.poll_connector_requests(){reads.push(read);}
                if !self.installation_polling{if let Some(id)=self.job.clone(){
                    self.installation_polling=true;let client=client.clone();
                    reads.push(Task::perform(async move{client.request(Method::GET,&format!("/api/v1/setup/installations/{}",component(&id)),None).await},Message::Progress));
                }}
                for (id,restart) in &self.restarts{if !self.restart_polling.contains(id){
                    let id=id.clone();let client=client.clone();
                    let query=url::form_urlencoded::Serializer::new(String::new()).append_pair("id",&id).append_pair("restart_id",restart).finish();
                    self.restart_polling.insert(id.clone());
                    reads.push(Task::perform(async move{client.request(Method::GET,&format!("/api/v1/servers?{query}"),None).await},move|result|Message::RestartStatus(id.clone(),result)));
                }}
                if self.polling.is_none()&&((self.page==Page::Servers&&!self.expanded_servers.is_empty())||self.deletions.values().any(|state|*state==Deletion::Refreshing)||self.authentication.values().any(|flow|flow.reconcile)||self.servers["servers"].as_array().is_some_and(|records|records.iter().any(|record|record["authentication"]["status"]=="authenticating"||matches!(record["runtime"]["status"].as_str(),Some("starting"|"restarting"))||record["restart"]["status"]=="restarting"))){let read=self.refresh();self.polling=Some(self.auth_requests.latest_sequence());reads.push(read);}
                return Task::batch(reads);
            }
        }
        Task::none()
    }
    fn view(&self,_window:window::Id)->Element<'_,Message>{
        let base=self.view_content();
        let mut layers=iced::widget::stack![base];
        if self.page==Page::Workspace&&self.workspace.active_session.is_some(){layers=layers.push(self.workspace_session_popup());}
        if self.page==Page::Workspace&&self.workspace.agent_popup{layers=layers.push(self.workspace_agent_popup());}
        if self.connector.popup_visible(){layers=layers.push(self.connector_popup());}
        let content:Element<'_,Message>=layers.into();
        surface::with_scale(iced::widget::mouse_area(content).on_press(Message::DragWindow),self.scale_factor)
    }
    fn view_content(&self)->Element<'_,Message>{
        let active=match self.page{Page::Servers|Page::Settings=>0,Page::Install=>1,Page::Connect=>2,Page::Workspace=>3};
        let nav=components::navigation([Message::Page(Page::Servers),Message::Page(Page::Install),Message::Page(Page::Connect),Message::Page(Page::Workspace)],active);
        let mut body=column![nav].spacing(if self.page==Page::Connect{16}else{32}).padding(16);
        if self.page==Page::Install{
            let base=components::page(body.push(self.install_view()).height(iced::Fill));
            return if self.discovery_started{iced::widget::stack![base,self.install_popup_view()].into()}else{base.into()};
        }
        body=body.push(match self.page{Page::Servers|Page::Settings=>self.servers_view(),Page::Install=>self.install_view(),Page::Connect=>self.connect_view(),Page::Workspace=>self.workspace_view()});
        components::page(body.height(iced::Fill)).into()
    }
    fn servers_view(&self)->Element<'_,Message>{
        use components::Icon;
        let heading=row![text("Installed servers").size(32).line_height(iced::Pixels(45.0)).font(theme::SEMIBOLD_FONT),components::refresh_button(self.client.is_some().then_some(Message::Refresh),self.server_refresh.running())].spacing(20).align_y(iced::Center).height(45);
        let mut items=column![].spacing(20).width(iced::Fill);
        if let Some(servers)=self.servers["servers"].as_array(){
            if servers.is_empty(){items=items.push(text(if self.projects.server_filter==project_scope::Choice::All{"No servers installed yet. Open Install MCPs to discover your existing definitions."}else{"No servers in this view. Choose another project or open Install MCPs."}));}
            for (index,server) in servers.iter().enumerate(){
                let id=server["server_id"].as_str().unwrap_or("").to_owned();
                let runtime=server["runtime"]["status"].as_str().unwrap_or("unknown");
                let starting=matches!(runtime,"starting"|"restarting")||server["authentication"]["status"]=="authenticating"||server["restart"]["status"]=="restarting"||self.restart_submitting.contains(&id)||self.restarts.contains_key(&id)||self.authentication.get(&id).is_some_and(|flow|flow.phase==AuthPhase::Starting);
                let runtime=if starting{"starting"}else{runtime};
                let auth=server["authentication"]["status"].as_str().unwrap_or("unknown");
                let name=server["name"].as_str().unwrap_or(&id);
                let can_change=!self.busy&&!self.auth_active(&id)&&!self.restarts.contains_key(&id);
                let expanded=self.expanded_servers.contains(&id);
                let enabled=server["config"]["enabled"].as_bool().unwrap_or(true);
                let toggle_id=id.clone();let power=components::server_toggle(enabled,(can_change&&!starting).then_some(move|enabled|Message::PowerClicked(toggle_id.clone(),enabled)));
                let key=components::icon_button(Icon::Key,(can_change&&matches!(auth,"required"|"failed"|"unknown")).then_some(Message::Authenticate(id.clone())));
                let restart=components::refresh_button(can_change.then_some(Message::RestartClicked(id.clone())),starting);
                let remove=components::icon_button(Icon::Close,self.can_delete(&id).then_some(Message::Delete(id.clone())));
                let restarting=starting;
                let restart_failed=!restarting&&(self.restart_failures.contains_key(&id)||server["restart"]["status"]=="failed");
                let identity=components::server_identity(name,runtime,auth,restart_failed,expanded,Message::ToggleServer(id.clone()));
                let header=row![identity,power,key,restart,remove].spacing(10).align_y(iced::Center);
                let mut card=column![header,text(self.server_scope_label(server)).size(12).color(theme::MUTED)].spacing(14).width(iced::Fill);
                if expanded{
                    let pane=self.server_panel(&id);
                    let local=server["config"]["kind"]!="remote";
                    let controls=row![
                        components::tab("Inspect tools",pane==server_terminal::Pane::Tools,(!self.deletions.contains_key(&id)).then_some(Message::Inspect(id.clone()))),
                        components::tab("Settings",pane==server_terminal::Pane::Settings,can_change.then_some(Message::Choose(id.clone()))),
                        iced::widget::Space::new().width(iced::Fill),
                        components::output_tab("Console",pane==server_terminal::Pane::Console,Some(Message::Terminal(server_terminal::Message::Console(id.clone())))),
                        components::output_tab("Terminal",pane==server_terminal::Pane::Terminal,(local&&self.client.is_some()&&!self.deletions.contains_key(&id)).then_some(Message::Terminal(server_terminal::Message::Open(id.clone())))),
                    ].spacing(10).align_y(iced::Center);
                    card=card.push(controls);
                    match pane{
                        server_terminal::Pane::Tools=>card=card.push(self.server_tools_view(&id)),
                        server_terminal::Pane::Settings=>card=card.push(self.settings_view()),
                        server_terminal::Pane::Console|server_terminal::Pane::Terminal=>card=card.push(self.server_output_view(&id,server)),
                        server_terminal::Pane::Hidden=>{},
                    }
                    if self.restarts.contains_key(&id){card=card.push(text("Restarting… waiting for this attempt to finish."));}
                    let count=|key:&str|server["tool_execution"][key].as_u64().unwrap_or(0);
                    card=card.push(row![text(format!("Active: {}   Waiting: {}   Uncertain: {}",count("active_calls"),count("queued_calls"),count("uncertain_calls"))).size(12).line_height(iced::Pixels(18.0)).color(theme::MUTED),iced::widget::Space::new().width(iced::Fill),text(format!("Connection access expires: {}",server["authentication"]["access_expires_at"].as_str().unwrap_or("unknown"))).size(12).line_height(iced::Pixels(18.0)).color(theme::MUTED),text(format!("Last verified connection access: {}",server["authentication"]["last_verified_at"].as_str().unwrap_or("unknown"))).size(12).line_height(iced::Pixels(18.0)).color(theme::MUTED)].spacing(16).height(22).align_y(iced::Center));
                }
                items=items.push(surface::pop_in(components::server_card(card).width(iced::Fill),self.reload_generation.wrapping_add(1),index));
            }
        }
        let cards=surface::contain_scroll(scrollable(iced::widget::container(items).padding(24)).id("fetch-installed-servers").style(theme::scrollable).width(iced::Fill).height(iced::Fill));
        column![iced::widget::container(heading).padding(iced::Padding{top:24.0,right:0.0,bottom:0.0,left:0.0}),self.project_picker("Show servers",&self.projects.server_filter,true,true,project_scope::Event::Filter),components::recessed(cards).width(iced::Fill).height(iced::Fill)].spacing(20).height(iced::Fill).into()
    }
    fn server_tools_view<'a>(&'a self,id:&str)->Element<'a,Message>{
        let mut cards=Vec::new();
        let mut notes=column![].spacing(10);
        let mut found=false;
        if let Some(origins)=self.tools["origins"].as_array(){for origin in origins.iter().filter(|origin|origin["source"]["server_id"].as_str()==Some(id)){
            found=true;
            if let Some(tools)=origin["tools"].as_array(){for tool in tools{
                let name=tool["name"].as_str().unwrap_or("Unnamed tool");
                let required=tool["inputSchema"]["required"].as_array().map(|values|values.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")).filter(|value|!value.is_empty()).unwrap_or_else(||"None".into());
                let detail=column![
                    column![text("Name").size(12).line_height(iced::Pixels(16.0)).font(theme::BOLD_FONT).color(theme::MUTED),components::read_only(name.to_owned(),52.0)].spacing(6),
                    column![text("Required fields").size(12).line_height(iced::Pixels(16.0)).font(theme::BOLD_FONT).color(theme::MUTED),components::read_only(required,64.0)].spacing(6),
                ].spacing(12);
                cards.push(components::raised_with_radius(iced::widget::container(detail).padding(14).height(200).width(iced::Fill),32.0).width(iced::Fill).into());
            }}
            if origin["status"]!="available"{notes=notes.push(text("This catalog is incomplete. Review its server state and rejected-tool details."));if let Some(rejected)=origin["rejected_tools"].as_array(){for rejection in rejected{notes=notes.push(text(format!("{}: {}",rejection["name"].as_str().unwrap_or("Tool"),rejection["message"].as_str().unwrap_or("Compatibility issue"))));}}}
        }}
        let mut content=column![].spacing(16).padding(24).width(iced::Fill);
        if !found{content=content.push(text("Loading tools…"));}else if cards.is_empty(){content=content.push(text("No tools available."));}else{content=content.push(components::tool_grid(cards));}
        content=content.push(notes);
        column![text("Available tools").size(22).line_height(iced::Pixels(31.0)).font(theme::SEMIBOLD_FONT),components::recessed(surface::contain_scroll(scrollable(content).style(theme::scrollable).id(format!("fetch-tools-{id}")).height(300))).width(iced::Fill).max_width(744)].spacing(14).into()
    }
    fn folder_button(&self,id:String)->Element<'_,Message>{
        let enabled=!self.deletions.contains_key(&id)&&!self.opening_folders.contains(&id);
        components::text_button(if self.opening_folders.contains(&id){"Opening server folder…"}else{"Open server folder"})
            .on_press_maybe(enabled.then_some(Message::OpenFolder(id))).into()
    }
    fn console_lines(&self,id:&str,server:&Value)->Vec<String>{
        let flow=self.authentication.get(id);
        let mut lines:Vec<_>=flow.into_iter().flat_map(|flow|&flow.log).map(|line|{
            line.strip_prefix("[error] ").map(|error|format!("[Fetch] {error}")).unwrap_or_else(||line.clone())
        }).collect();
        let mut errors=Vec::new();
        for error in [&server["authentication"]["error"],&server["runtime"]["error"],&server["restart"]["error"]]{
            if let Some(message)=error["message"].as_str(){
                let code=error["code"].as_str().map(|code|format!("{code}: ")).unwrap_or_default();
                let mut line=format!("[Fetch] {code}{message}");
                if let Some(fields)=error.as_object(){
                    let details:serde_json::Map<_,_>=fields.iter().filter(|(key,_)|!matches!(key.as_str(),"code"|"message")).map(|(key,value)|(key.clone(),value.clone())).collect();
                    if !details.is_empty(){line.push_str(&format!(" {}",Value::Object(details)));}
                }
                errors.push((message.to_owned(),line));
            }
        }
        if let Some(flow)=flow{for message in [&flow.error,&flow.read_error].into_iter().flatten(){
            if !errors.iter().any(|(existing,_)|existing==message){errors.push((message.clone(),format!("[Fetch] {message}")));}
        }}
        if let Some(message)=self.restart_failures.get(id){
            if !errors.iter().any(|(existing,_)|existing==message){errors.push((message.clone(),format!("[Fetch] {message}")));}
        }
        if let Some(message)=self.power_errors.get(id){if !errors.iter().any(|(existing,_)|existing==message){errors.push((message.clone(),format!("[Fetch] {message}")));}}
        for (message,line) in errors{
            // Replace the message-only fallback with its complete returned error.
            lines.retain(|existing|existing!=&format!("[Fetch] {message}"));
            if !lines.contains(&line){lines.push(line);}
        }
        lines
    }
    fn authentication_view<'a>(&'a self,id:&str,server:&Value)->Element<'a,Message>{
        let flow=self.authentication.get(id);
        let mut output=column![].spacing(theme::CONSOLE_LINE_GAP).width(iced::Fill);
        let lines=self.console_lines(id,server);
        for line in &lines {output=output.push(components::console_line(line.clone()));}
        if let Some(sign_in)=flow.and_then(|flow|flow.sign_in.as_ref()){
            if !sign_in.opened{
                output=output.push(button(components::console_line(format!("> Open sign-in at {}",sign_in.host)))
                    .style(theme::console_action)
                    .on_press_maybe(self.can_open_sign_in(id).then_some(Message::AuthOpenSignIn(id.to_owned()))));
            }
        }
        components::console(output,Message::Copy(lines.join("\n"))).into()
    }
    fn supported_selections(&self)->BTreeSet<(String,String)>{
        let mut selections=BTreeSet::new();
        if let Some(groups)=self.discovery["configs"].as_array(){for group in groups{
            if let (Some(path),Some(rows))=(group["path"].as_str(),group["servers"].as_array()){for server in rows{
                if installable(server){if let Some(name)=server["name"].as_str(){selections.insert((path.to_owned(),name.to_owned()));}}
            }}
        }}
        selections
    }
    fn selected_for_install(&self)->Vec<(String,String)>{
        self.selected.intersection(&self.supported_selections()).cloned().collect()
    }
    fn mark_installed_from_progress(&mut self,progress:&Value){
        let mut succeeded=BTreeMap::new();
        if let Some(rows)=progress["servers"].as_array(){for item in rows{
            if item["status"]=="succeeded"{if let(Some(path),Some(name),Some(id))=(item["config_path"].as_str(),item["server_name"].as_str(),item["server_id"].as_str()){
                succeeded.insert((path,name),id);
            }}
        }}
        if let Some(groups)=self.discovery["configs"].as_array_mut(){for group in groups{
            let path=group["path"].as_str().unwrap_or("").to_owned();
            if let Some(rows)=group["servers"].as_array_mut(){for row in rows{
                if let Some(id)=row["name"].as_str().and_then(|name|succeeded.get(&(path.as_str(),name))).copied(){
                    row["installed"]=json!(true);row["installed_server_id"]=json!(id);row["selected_by_default"]=json!(false);
                }
            }}
        }}
        let eligible=self.supported_selections();
        self.selected.retain(|key|eligible.contains(key));
    }
    fn install_view(&self)->Element<'_,Message>{
        let available=!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let header=column![
            text("Discover and install").size(32).line_height(iced::Pixels(45.0)).font(iced::Font{weight:iced::font::Weight::Semibold,..iced::Font::with_name("Inter")}),
            self.project_picker("Install into",&self.projects.install,false,available&&!self.discovery_refresh.running(),project_scope::Event::Install),
            row![
                text_input("Folder or configuration file path",&self.config_path).on_input_maybe(available.then_some(Message::ConfigPath)).on_submit_maybe((available&&!self.discovery_refresh.running()&&(!self.config_path.trim().is_empty()||self.projects.install.project_id().is_some())).then_some(Message::Discover)),
                components::refresh_button((self.client.is_some()&&available).then_some(Message::Discover),self.discovery_refresh.running()),
                components::text_button("Choose file or folder…").on_press_maybe(available.then_some(Message::ChooseConfigFile)),
            ].spacing(16).align_y(iced::Center),
        ].spacing(20);
        let mut base=column![header].spacing(24).width(iced::Fill).height(iced::Fill);
        if !self.discovery.is_null()||self.job.is_some(){base=base.push(components::text_button(if self.job.is_some(){"View installation progress"}else{"View discovered MCPs"}).on_press(Message::OpenDiscovery));}
        base.into()
    }
    fn install_popup_view(&self)->Element<'_,Message>{
        let eligible=self.supported_selections();
        let all_selected=!eligible.is_empty()&&eligible.is_subset(&self.selected);
        let can_select_all=!self.busy&&self.picker_window.is_none()&&self.job.is_none()&&!eligible.is_empty();
        let available=!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let mut items=column![].spacing(theme::PAGE_GAP);
        if let Some(error)=&self.error{items=items.push(components::error_message(error.as_str()));}
        if !self.notice.is_empty(){items=items.push(components::message(self.notice.as_str()));}
        let has_rows=self.discovery["configs"].as_array().is_some_and(|groups|groups.iter().any(|group|group["servers"].as_array().is_some_and(|rows|!rows.is_empty())));
        if !has_rows{items=items.push(text(if !self.discovery_started{"Choose a folder to discover MCP servers."}else if self.busy{"Looking for server definitions…"}else{"No server definitions found. Review the source messages below, refresh, or choose a different file or folder."}));}
        if let Some(groups)=self.discovery["configs"].as_array(){for group in groups{let path=group["path"].as_str().unwrap_or("").to_owned();items=items.push(text(path.clone()).size(theme::TOOL_TITLE));if let Some(rows)=group["servers"].as_array(){for server in rows{let name=server["name"].as_str().unwrap_or("").to_owned();let eligible=installable(server);let installed=server["installed"]==true;let selected=eligible&&self.selected.contains(&(path.clone(),name.clone()));let control=if installed{components::installed_checkbox()}else{checkbox(selected)};let mut control=control.label(format!("{} · {}{}",name,server["type"].as_str().unwrap_or("unsupported"),if installed{" · already installed"}else if server["source_enabled"]==false{" · disabled in source; opt in explicitly"}else{""}));if eligible{let path=path.clone();let name=name.clone();control=control.on_toggle(move|value|Message::Select(path.clone(),name.clone(),value));}items=items.push(control);if let Some(error)=server["error"]["message"].as_str(){items=items.push(components::message(error));}}}if let Some(error)=group["error"]["message"].as_str(){items=items.push(components::message(error));}}}
        if !self.progress.is_null(){items=items.push(text(format!("Installation: {}",self.progress["status"].as_str().unwrap_or("pending"))));if let Some(rows)=self.progress["servers"].as_array().or_else(||self.progress["items"].as_array()){for item in rows{items=items.push(text(format!("{} · {}",item["server_name"].as_str().unwrap_or("server"),item["status"].as_str().unwrap_or("pending"))));if let Some(error)=item["error"]["message"].as_str(){items=items.push(components::message(error));}}}}
        let selected_count=self.selected_for_install().len();
        let can_install=selected_count>0&&!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let footer=row![components::primary_button("Install selected servers").on_press_maybe(can_install.then_some(Message::Install)),text(format!("{} selected",selected_count))].spacing(theme::PAGE_GAP).align_y(iced::Center);
        let title=if self.busy&&self.discovery.is_null(){"Looking for MCP servers…"}else if self.job.is_some(){"Installing MCP servers"}else if self.progress["status"]=="failed"{"Installation needs attention"}else{"Choose MCPs to install"};
        let popup=components::raised(iced::widget::container(column![
            row![text(title).size(28).font(iced::Font{weight:iced::font::Weight::Semibold,..iced::Font::with_name("Inter")}),iced::widget::Space::new().width(iced::Fill),components::icon_button(components::Icon::Close,Some(Message::CloseDiscovery))].spacing(16).align_y(iced::Center),
            components::text_button(if all_selected{"Unselect all"}else{"Select all"}).on_press_maybe(can_select_all.then_some(Message::SelectAllServers)),
            surface::contain_scroll(scrollable(items).style(theme::scrollable).id("fetch-install-discovery").width(iced::Fill).height(iced::Fill)),footer,
        ].spacing(24)).padding(32).width(iced::Fill).height(iced::Fill)).width(iced::Fill).max_width(900).height(iced::Fill).max_height(600);
        components::modal(popup,Message::CloseDiscovery)
    }
    fn connect_view(&self)->Element<'_,Message>{
        let mut items=column![self.project_picker("Connection",&self.projects.connection,false,!self.connector.creating&&!self.connector.deciding,project_scope::Event::Connection)].spacing(20).width(iced::Fill);
        if let Some(project)=self.selected_connection_project(){items=items.push(text(format!("Project folder: {}",project["folder_path"].as_str().unwrap_or(""))).size(12)).push(text("Run the Claude command from this folder. Add the Codex entry to this folder’s .codex/config.toml.").size(12).color(theme::MUTED));}
        if let Some(feedback)=self.connector_feedback(){items=items.push(feedback);}
        let items=items
            .push(self.connector_heading("claude-code","Claude Code",include_bytes!("assets/claude-logo.svg")))
            .push(components::code_block(self.connector_command("claude-code"),Message::Copy))
            .push(self.connector_heading("codex","Codex",include_bytes!("assets/codex-logo.svg")))
            .push(components::code_block(self.connector_command("codex"),Message::Copy));
        let apps=surface::contain_scroll(scrollable(iced::widget::container(items).padding(24)).id("fetch-connect-apps").style(theme::scrollable).width(iced::Fill).height(iced::Fill));
        let heading=text("Connect your coding apps").size(32).line_height(iced::Pixels(45.0)).font(theme::SEMIBOLD_FONT);
        column![iced::widget::container(heading).padding(iced::Padding{top:12.0,right:24.0,bottom:0.0,left:24.0}),components::recessed(apps).width(iced::Fill).height(iced::Fill)].spacing(20).height(iced::Fill).into()
    }

    fn settings_view(&self)->Element<'_,Message>{
        let mut items=column![self.folder_button(self.selected_server.clone()),self.server_scope_editor(&self.selected_server)].spacing(20).padding(24).width(iced::Fill);
        let enabled=!self.busy&&!self.auth_active(&self.selected_server);
        let number=|value:&str,placeholder:&str,on_input:fn(String)->Message,up:Message,down:Message|{
            let arrow=|kind,action|button(iced::widget::container(components::icon(kind)).center_x(24).center_y(18)).padding(0).style(theme::transparent).on_press_maybe(enabled.then_some(action));
            let arrows=iced::widget::container(column![arrow(components::Icon::Up,up),arrow(components::Icon::Down,down)].spacing(0)).width(24).height(36).style(|_|iced::widget::container::Style{background:Some(iced::Color::WHITE.into()),..Default::default()});
            components::input_surface(iced::widget::container(row![text_input(placeholder,value).unframed().padding(0).on_input(on_input),arrows].align_y(iced::Center)).padding(iced::Padding{top:4.0,right:4.0,bottom:4.0,left:12.0}).height(44).width(iced::Fill)).width(iced::Fill)
        };
        let bold=iced::Font{weight:iced::font::Weight::Bold,..iced::Font::with_name("Inter")};
        items=items.push(row![
            column![text("Time until timeout").font(bold).size(17).line_height(iced::Pixels(24.0)),number(&self.timeout,"120",Message::Timeout,Message::StepTimeout(1),Message::StepTimeout(-1))].spacing(20).width(iced::Fill),
            column![text("Max concurrent calls").font(bold).size(17).line_height(iced::Pixels(24.0)),number(&self.concurrency,"1",Message::Concurrency,Message::StepConcurrency(1),Message::StepConcurrency(-1))].spacing(20).width(iced::Fill),
        ].spacing(24)).push(components::primary_button("Save call limits").on_press_maybe(enabled.then_some(Message::SaveExecution)));
        if let Some(server)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|server|server["server_id"].as_str()==Some(self.selected_server.as_str()))){
            let references=authentication::credential_references(server);
            if !references.is_empty(){
                for (binding,reference) in references{items=items.push(components::text_button(format!("{binding} → {reference}")).on_press_maybe(enabled.then_some(Message::SecretName(reference))));}
                items=items.push(authentication::credential_editor(&self.secret_name,&self.secret_value,enabled,Message::SecretName,Message::SecretValue,Message::SaveSecret,Some(Message::RemoveSecret)));
            }
        }
        column![text("Server settings").size(22).line_height(iced::Pixels(31.0)).font(theme::SEMIBOLD_FONT),components::recessed(surface::contain_scroll(scrollable(items).style(theme::scrollable).id("fetch-inline-settings").height(440))).width(iced::Fill)].spacing(14).into()
    }

}

pub fn run(host:Arc<ApplicationHost>)->iced::Result{
    iced::daemon(move||Desktop::boot(host.clone()),Desktop::update,Desktop::view)
        .title(|_:&Desktop,_:window::Id|"Fetch".to_owned()).theme(|_: &Desktop, _: window::Id| theme::light())
        .style(|_,_|iced::theme::Style{background_color:iced::Color::TRANSPARENT,text_color:theme::INK})
        .settings(iced::Settings{default_font:iced::Font::with_name("Inter"),default_text_size:iced::Pixels(14.0),..Default::default()})
        .font(include_bytes!("assets/fonts/Inter.ttf").as_slice())
        .font(include_bytes!("assets/fonts/Inter-Medium.ttf").as_slice())
        .font(include_bytes!("assets/fonts/Inter-SemiBold.ttf").as_slice())
        .font(include_bytes!("assets/fonts/Inter-Bold.ttf").as_slice())
        .font(include_bytes!("assets/fonts/RobotoMono.ttf").as_slice())
        .subscription(Desktop::subscription).run()
}
