//! Native API-backed interface. No workflow reads next_steps as executable state.
pub mod client;
mod file_picker;
mod authentication;
mod auth_requests;
mod components;
mod theme;
#[cfg(test)]
mod auth_tests;
use components::{button, checkbox, text_input};
use authentication::{Flow as AuthFlow, Phase as AuthPhase};
use crate::{application::ApplicationHost, native_auth::NativeSettingsSession};
use client::{NativeClient, component};
use iced::{Element, Subscription, Task, window, widget::{column, container, row, scrollable, text}};
use reqwest::Method;
use serde_json::{Value, json};
use zeroize::Zeroize;
use std::{collections::{BTreeMap,BTreeSet}, sync::Arc, time::Duration};

#[derive(Clone,Copy,PartialEq,Eq)]
enum Page { Servers, Install, Connect, Settings }
#[derive(Clone,Copy,PartialEq,Eq)]
enum Deletion { Pending(u64), Refreshing }
fn installable(row:&Value)->bool { row["status"]=="supported" && row["installed"]!=true }
#[derive(Clone)]
enum Message {
    Page(Page), Opened(window::Id), Closed(window::Id), Tick, Show, Quit, Stopped,
    Refresh, Servers(u64,u64,Option<window::Id>,Result<Value,String>), Discover, ConfigPath(String), CustomConfig(bool), ChooseConfigFile, ConfigFileChosen(window::Id,Result<Option<String>,String>), Discovery(Result<Value,String>),
    Select(String,String,bool), SelectAllServers, Install, Installed(Result<Value,String>), Progress(Result<Value,String>),
    Inspect(String), Tools(String,Option<window::Id>,Result<Value,String>), Authenticate(String), Restart(String,bool), Delete(String), Deleted(u64,String,Result<Value,String>),
    AuthFinished(u64,String,Result<Value,String>), AuthDismiss(String),
    CredentialFinished(u64,String,bool,Result<Value,String>), AuthName(String,String), AuthValue(String,String), AuthSave(String),
    OpenFolder(String), FolderOpened(String,Option<window::Id>,Result<Value,String>),
    Changed(String,Result<Value,String>), RestartStatus(String,Result<Value,String>),
    Choose(String), Timeout(String), Concurrency(String), SaveExecution,
    SecretName(String), SecretValue(String), SaveSecret, RemoveSecret,
    Copy(String), CleanupConfirmed(bool), ToggleServer(String), ToggleTool(String),
}
impl std::fmt::Debug for Message { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("FetchNativeMessage") } }
struct Desktop {
    host: Arc<ApplicationHost>, client: Option<NativeClient>, session: Option<Arc<NativeSettingsSession>>, window:Option<window::Id>,
    page:Page, servers:Value, discovery:Value, tools:Value, selected:BTreeSet<(String,String)>,
    config_path:String, custom_config:bool, discovery_started:bool, picker_window:Option<window::Id>, selected_server:String, timeout:String, concurrency:String, secret_name:String, secret_value:String,
    job:Option<String>, progress:Value, restarts:BTreeMap<String,String>, error:Option<String>, notice:String,
    busy:bool, polling:Option<u64>, installation_polling:bool, restart_polling:BTreeSet<String>, cleanup_confirmed:bool, expanded_servers:BTreeSet<String>, expanded_tools:BTreeSet<String>,
    authentication:BTreeMap<String,AuthFlow>, auth_requests:auth_requests::Requests,
    deletions:BTreeMap<String,Deletion>, opening_folders:BTreeSet<String>,
}
impl Desktop {
    fn boot(host:Arc<ApplicationHost>)->(Self,Task<Message>) {
        let session=Arc::new(host.native.open_settings());
        let client=NativeClient::new(host.state.origin.clone(),&host.certificate_pem,session.clone());
        let (id,open)=window::open(window::Settings { size:iced::Size::new(1080.0,760.0),min_size:Some(iced::Size::new(760.0,520.0)),..Default::default() });
        let mut state=Self {host,client:None,session:Some(session),window:Some(id),page:Page::Servers,servers:json!({"servers":[]}),discovery:Value::Null,tools:Value::Null,selected:BTreeSet::new(),config_path:String::new(),custom_config:false,discovery_started:false,picker_window:None,selected_server:String::new(),timeout:"120".into(),concurrency:"1".into(),secret_name:String::new(),secret_value:String::new(),job:None,progress:Value::Null,restarts:BTreeMap::new(),error:None,notice:String::new(),busy:false,polling:None,installation_polling:false,restart_polling:BTreeSet::new(),cleanup_confirmed:false,expanded_servers:BTreeSet::new(),expanded_tools:BTreeSet::new(),authentication:BTreeMap::new(),auth_requests:auth_requests::Requests::default(),deletions:BTreeMap::new(),opening_folders:BTreeSet::new()};
        match client {Ok(client)=>state.client=Some(client),Err(error)=>state.error=Some(error)}
        let read=state.refresh();
        (state,Task::batch([open.map(Message::Opened),read]))
    }
    fn refresh(&mut self)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        let (revision,sequence)=self.auth_requests.begin_read();let window=self.window;
        Task::perform(async move{client.servers().await},move|result|Message::Servers(revision,sequence,window,result))
    }
    fn subscription(&self)->Subscription<Message> {
        Subscription::batch([iced::time::every(Duration::from_secs(1)).map(|_|Message::Tick),window::close_events().map(Message::Closed)])
    }
    fn operation(&mut self,id:String,method:Method,suffix:&str,body:Value)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        if self.busy||self.auth_active(&id){return Task::none();} self.busy=true;self.error=None;self.cleanup_confirmed=false;
        let path=format!("/api/v1/servers/{}{suffix}",component(&id));
        Task::perform(async move{client.request(method,&path,Some(body)).await},move |result|Message::Changed(id.clone(),result))
    }
    fn auth_active(&self,id:&str)->bool {
        self.deletions.contains_key(id) || self.auth_requests.contains(id) || self.authentication.get(id).is_some_and(|flow|flow.active()||flow.reconcile) || self.servers["servers"].as_array().into_iter().flatten()
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
        self.error=None;self.notice.clear();self.cleanup_confirmed=false;
        let path=format!("/api/v1/servers/{}",component(&id));
        Task::perform(async move{client.request(Method::DELETE,&path,None).await},move|result|Message::Deleted(token,id.clone(),result))
    }
    fn forget_server(&mut self,id:&str) {
        if let Some(rows)=self.servers["servers"].as_array_mut(){rows.retain(|row|row["server_id"].as_str()!=Some(id));}
        if let Some(origins)=self.tools.get_mut("origins").and_then(Value::as_array_mut){origins.retain(|origin|origin["source"]["server_id"].as_str()!=Some(id));if origins.is_empty(){self.tools=Value::Null;}}
        self.authentication.remove(id);
        self.opening_folders.remove(id);
        self.expanded_servers.remove(id);self.expanded_tools.retain(|key|!key.starts_with(&format!("{id}:")));
        self.restarts.remove(id);self.restart_polling.remove(id);
        if self.selected_server==id{self.secret_value.zeroize();self.secret_name.clear();self.selected_server.clear();self.timeout="120".into();self.concurrency="1".into();}
        if let Some(groups)=self.discovery["configs"].as_array_mut(){for group in groups{
            let path=group["path"].as_str().unwrap_or("").to_owned();
            if let Some(rows)=group["servers"].as_array_mut(){for row in rows{if row["installed_server_id"].as_str()==Some(id){
                if let Some(name)=row["name"].as_str(){self.selected.remove(&(path.clone(),name.to_owned()));}
                row["installed"]=json!(false);row["installed_server_id"]=Value::Null;row["selected_by_default"]=json!(false);
            }}}
        }}
        if let Some(rows)=self.progress["servers"].as_array_mut(){rows.retain(|row|row["server_id"].as_str()!=Some(id));}
        self.cleanup_confirmed=false;
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
        self.clear_secrets();self.authentication.insert(id.clone(),AuthFlow::starting());
        self.cleanup_confirmed=false;
        self.expanded_servers.insert(id.clone());
        let path=format!("/api/v1/servers/{}/authenticate",component(&id));
        Task::perform(async move{client.request(Method::POST,&path,Some(json!({"interactive":true}))).await},move|result|Message::AuthFinished(token,id.clone(),result))
    }
    fn submit_credential_request(&mut self,id:String,body:Value,settings:bool)->Task<Message>{
        let Some(client)=self.client.clone() else{return Task::none();};
        let Some(token)=self.auth_requests.begin(&id) else{return Task::none();};
        self.cleanup_confirmed=false;self.expanded_servers.insert(id.clone());
        let path=format!("/api/v1/servers/{}/credentials",component(&id));
        Task::perform(async move{client.request(Method::PATCH,&path,Some(body)).await},move|result|Message::CredentialFinished(token,id.clone(),settings,result))
    }
    fn save_inline_key(&mut self,id:String)->Task<Message>{
        if self.client.is_none()||self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
        let Some(flow)=self.authentication.get_mut(&id)else{return Task::none();};
        if flow.entry.trim().is_empty()||flow.secret.is_empty(){flow.error=Some("Choose an assigned key entry and enter its value.".into());return Task::none();}
        let body=authentication::manual_patch(flow.entry.clone(),Some(std::mem::take(&mut flow.secret)));
        flow.phase=AuthPhase::Saving;flow.storage_result=true;flow.reconcile=false;flow.error=None;
        self.submit_credential_request(id,body,false)
    }
    fn settings_credential_operation(&mut self,remove:bool)->Task<Message>{
        let id=self.selected_server.clone();
        if self.client.is_none()||self.busy||self.auth_active(&id)||self.restarts.contains_key(&id){return Task::none();}
        if id.is_empty()||self.secret_name.trim().is_empty()||(!remove&&self.secret_value.is_empty()){self.error=Some("Choose an assigned key entry and enter its value.".into());return Task::none();}
        let body=authentication::manual_patch(self.secret_name.clone(),(!remove).then(||std::mem::take(&mut self.secret_value)));
        self.secret_value.zeroize();
        let mut flow=AuthFlow::starting();flow.phase=AuthPhase::Saving;flow.storage_result=true;flow.entry=self.secret_name.clone();
        self.authentication.insert(id.clone(),flow);self.error=None;self.notice.clear();
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
            Message::Page(page)=>{if self.page!=page{self.clear_secrets();}self.page=page;self.error=None;if page==Page::Install&&!self.discovery_started{return self.update(Message::Discover);}}
            Message::Opened(id)=>{self.window=Some(id);crate::native_auth::install_app_menu();}
            Message::Closed(id)=>{if self.window==Some(id){self.window=None;self.picker_window=None;file_picker::cancel();self.host.native.invalidate_all();self.client=None;self.session=None;self.close_authentication();}}
            Message::Show=>{
                if let Some(id)=self.window{return window::gain_focus(id);}
                if self.window.is_none(){let session=Arc::new(self.host.native.open_settings());match NativeClient::new(self.host.state.origin.clone(),&self.host.certificate_pem,session.clone()){Ok(client)=>self.client=Some(client),Err(error)=>self.error=Some(error)}self.session=Some(session);let(id,open)=window::open(window::Settings{size:iced::Size::new(1080.0,760.0),min_size:Some(iced::Size::new(760.0,520.0)),..Default::default()});self.window=Some(id);return Task::batch([open.map(Message::Opened),self.refresh()]);}
            }
            Message::Quit=>{self.picker_window=None;file_picker::cancel();self.close_authentication();self.client=None;self.session=None;let host=self.host.clone();return Task::perform(async move{host.shutdown().await;},|_|Message::Stopped);}
            Message::Stopped=>return iced::exit(),
            Message::Refresh=>return self.refresh(),
            Message::Servers(revision,sequence,window,result)=>{
                if window!=self.window{return Task::none();}
                if self.polling==Some(sequence){self.polling=None;}
                if revision!=self.auth_requests.revision(){return self.refresh();}
                if !self.auth_requests.accept_read(revision,sequence){return Task::none();}
                match result {Ok(value)=>{
                    if let Some(servers)=value["servers"].as_array(){for server in servers{
                        if let Some(flow)=server["server_id"].as_str().and_then(|id|self.authentication.get_mut(id)){flow.observe(&server["authentication"]);}
                    }}
                    self.servers=value;
                    if self.servers["servers"].is_array(){
                        let refreshed:Vec<_>=self.deletions.iter().filter_map(|(id,state)|(*state==Deletion::Refreshing).then_some(id.clone())).collect();
                        for id in refreshed{self.deletions.remove(&id);if !self.servers["servers"].as_array().unwrap().iter().any(|row|row["server_id"].as_str()==Some(id.as_str())){self.forget_server(&id);}}
                    }
                },Err(error)=>{
                    let mut inline=false;
                    for flow in self.authentication.values_mut().filter(|flow|flow.reconcile){inline=true;flow.read_error=Some(format!("Could not read sign-in status: {error} Fetch will keep checking; do not start another login."));}
                    if !inline{self.error=Some(error);}
                }}
            }
            Message::ConfigPath(value)=>self.config_path=value,
            Message::CustomConfig(custom)=>{
                if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}
                self.custom_config=custom;
                if !custom{self.config_path.clear();self.discovery_started=false;return self.update(Message::Discover);}
            }
            Message::ChooseConfigFile=>{
                if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}
                let Some(id)=self.window else{return Task::none();};
                self.picker_window=Some(id);self.error=None;
                return Task::perform(file_picker::choose_config_file(),move|result|Message::ConfigFileChosen(id,result));
            }
            Message::ConfigFileChosen(id,result)=>{
                if self.window!=Some(id)||self.picker_window!=Some(id){return Task::none();}
                self.picker_window=None;
                match result{Ok(Some(path))=>self.config_path=path,Ok(None)=>{},Err(error)=>self.error=Some(error)}
            }
            Message::Discover=>{if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}if let Some(client)=self.client.clone(){self.busy=true;self.discovery_started=true;let path=if self.custom_config{self.config_path.clone()}else{String::new()};return Task::perform(async move{client.discovery(&path).await},Message::Discovery);}}
            Message::Discovery(result)=>{self.busy=false;match result {Ok(value)=>{
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
            Message::Install=>{self.cleanup_confirmed=false;let rows=self.selected_for_install();if self.busy||self.picker_window.is_some()||self.job.is_some()||rows.is_empty(){return Task::none();}if let Some(client)=self.client.clone(){self.busy=true;return Task::perform(async move{client.install(rows).await},Message::Installed);}}
            Message::Installed(result)=>{self.busy=false;match result {Ok(value)=>{self.job=value["installation_id"].as_str().map(str::to_owned);let existing=value["error"]["code"]=="INSTALLATION_IN_PROGRESS";self.progress=value;self.notice=if existing{"An installation is already running. Monitoring that existing operation.".into()}else{"Installation accepted. Fetch will connect prepared servers automatically; required login remains separate.".into()};},Err(error)=>self.error=Some(error)}}
            Message::Progress(result)=>{self.installation_polling=false;match result {Ok(value)=>{self.mark_installed_from_progress(&value);let terminal=matches!(value["status"].as_str(),Some("succeeded"|"failed"));if terminal {
                    self.selected.clear();
                    let eligible=self.supported_selections();
                    if let Some(rows)=value["servers"].as_array(){for item in rows {if item["status"]=="failed" {if let (Some(path),Some(name))=(item["config_path"].as_str(),item["server_name"].as_str()){let key=(path.to_owned(),name.to_owned());if eligible.contains(&key){self.selected.insert(key);}}}}}
                }
                self.progress=value;if terminal{self.job=None;return self.refresh();}},Err(error)=>self.error=Some(error)}}
            Message::Inspect(id)=>{if self.deletions.contains_key(&id){return Task::none();}if let Some(client)=self.client.clone(){let query=url::form_urlencoded::Serializer::new(String::new()).append_pair("server_id",&id).finish();let window=self.window;return Task::perform(async move{client.request(Method::GET,&format!("/api/v1/tools?{query}"),None).await},move|result|Message::Tools(id.clone(),window,result));}}
            Message::Tools(id,window,result)=>{if window!=self.window||self.deletions.contains_key(&id)||!self.servers["servers"].as_array().into_iter().flatten().any(|row|row["server_id"].as_str()==Some(id.as_str())){return Task::none();}match result{Ok(value)=>self.tools=value,Err(error)=>self.error=Some(error)}}
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
            Message::AuthFinished(token,id,result)=>{
                if !self.auth_requests.finish(&id,token){return Task::none();}
                if let Some(flow)=self.authentication.get_mut(&id){match result{
                    Ok(value)=>flow.result(&value),
                    Err(error)=>{flow.phase=AuthPhase::Failed;flow.error=Some(error);flow.reconcile=true;}
                }}
                return self.refresh();
            }
            Message::CredentialFinished(token,id,settings,result)=>{
                if !self.auth_requests.finish(&id,token){return Task::none();}
                if let Some(flow)=self.authentication.get_mut(&id){match &result{
                    Ok(value)=>flow.stored(value),
                    Err(error)=>{flow.phase=AuthPhase::Failed;flow.storage_result=true;flow.reconcile=false;flow.secret.zeroize();flow.error=Some(error.clone());}
                }}
                if settings&&self.page==Page::Settings&&self.selected_server==id{match result{
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

            Message::Restart(id,force)=>return self.operation(id,Method::POST,"/restart",json!({"force_restart":force})),
            Message::Changed(id,result)=>{self.busy=false;match result{Ok(value)=>{if let Some(restart)=value["restart_id"].as_str(){self.restarts.insert(id,restart.into());}self.notice=if value["status"]=="authenticating"{"Complete the provider's browser sign-in, then Fetch will show the result.".into()}else{"Saved. Review server state for any required next action.".into()};return self.refresh();},Err(error)=>{self.error=Some(error);return self.refresh();}}}
            Message::RestartStatus(id,result)=>{self.restart_polling.remove(&id);match result{Ok(value)=>{let restart=value["servers"].as_array().and_then(|records|records.first()).map(|record|&record["restart"]);if let Some(restart)=restart{if matches!(restart["status"].as_str(),Some("succeeded"|"failed")){self.restarts.remove(&id);self.notice=if restart["status"]=="succeeded"{"Server restart completed.".into()}else{"Server restart failed. Review its state before retrying.".into()};return self.refresh();}}},Err(error)=>{self.error=Some(error);}}}
            Message::Choose(id)=>{if self.deletions.contains_key(&id){return Task::none();}self.clear_secrets();self.selected_server=id;self.secret_name.clear();self.page=Page::Settings;
                if let Some(record)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|row|row["server_id"].as_str()==Some(self.selected_server.as_str()))){self.timeout=(record["config"]["execution"]["default_timeout_ms"].as_u64().unwrap_or(120000)/1000).to_string();self.concurrency=record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1).to_string();}
            }
            Message::Timeout(value)=>self.timeout=value,
            Message::Concurrency(value)=>self.concurrency=value,
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
            Message::CleanupConfirmed(value)=>self.cleanup_confirmed=value,
            Message::ToggleServer(id)=>{if self.expanded_servers.remove(&id){if let Some(flow)=self.authentication.get_mut(&id){flow.secret.zeroize();}}else{self.clear_secrets();self.expanded_servers.insert(id);}}
            Message::ToggleTool(id)=>{if !self.expanded_tools.remove(&id){self.expanded_tools.insert(id);}}
            Message::Tick=>{
                if self.host.is_stopped(){return iced::exit();}
                match crate::native_auth::take_app_action(){1=>return self.update(Message::Show),2=>return self.update(Message::Quit),_=>{}}
                if self.host.take_show_request(){return self.update(Message::Show);}
                let Some(client)=self.client.clone() else{return Task::none();};
                let mut reads=Vec::new();
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
                if self.polling.is_none()&&(self.deletions.values().any(|state|*state==Deletion::Refreshing)||self.authentication.values().any(|flow|flow.reconcile)||self.servers["servers"].as_array().is_some_and(|records|records.iter().any(|record|record["authentication"]["status"]=="authenticating"))){let read=self.refresh();self.polling=Some(self.auth_requests.latest_sequence());reads.push(read);}
                return Task::batch(reads);
            }
        }
        Task::none()
    }
    fn view(&self,_window:window::Id)->Element<'_,Message>{
        let nav=components::navigation([Message::Page(Page::Servers),Message::Page(Page::Install),Message::Page(Page::Connect),Message::Page(Page::Settings),Message::Quit]);
        let mut body=column![nav,text("Your MCP servers, shared through one local connection.").size(theme::TAGLINE)].spacing(theme::SECTION_GAP);
        if self.page==Page::Install{
            // Give the list a finite share of the window; its action footer never scrolls.
            return components::page(body.push(self.install_view()).height(iced::Fill)).into();
        }
        if let Some(error)=&self.error{body=body.push(components::error_message(error.as_str()));}
        if !self.notice.is_empty(){body=body.push(components::message(self.notice.as_str()));}
        body=body.push(match self.page{Page::Servers=>self.servers_view(),Page::Install=>self.install_view(),Page::Connect=>self.connect_view(),Page::Settings=>self.settings_view()});
        components::page(scrollable(body)).into()
    }
    fn servers_view(&self)->Element<'_,Message>{
        let mut items=column![row![text("Installed servers").size(theme::PAGE_TITLE),button("Refresh").on_press(Message::Refresh)].spacing(theme::SECTION_GAP)].spacing(theme::CARD_GAP);
        if let Some(servers)=self.servers["servers"].as_array(){if servers.is_empty(){items=items.push(text("No servers installed yet. Open Install MCPs to discover your existing definitions."));}for server in servers{
            let id=server["server_id"].as_str().unwrap_or("").to_owned();let runtime=server["runtime"]["status"].as_str().unwrap_or("unknown");let auth=server["authentication"]["status"].as_str().unwrap_or("unknown");
            let title=format!("{}  ·  {}  ·  authentication: {}",server["name"].as_str().unwrap_or(&id),runtime,auth);
            let can_change=!self.busy&&!self.auth_active(&id)&&!self.restarts.contains_key(&id);
            let mut controls=row![button("Inspect tools").on_press_maybe((!self.deletions.contains_key(&id)).then_some(Message::Inspect(id.clone()))),button("Settings").on_press_maybe(can_change.then_some(Message::Choose(id.clone())))].spacing(theme::CONTROL_GAP);
            if matches!(auth,"required"|"failed"|"unknown"){controls=controls.push(button("Authenticate").on_press_maybe(can_change.then_some(Message::Authenticate(id.clone()))));}
            if server["restart_required"]==true||matches!(runtime,"stopped"|"failed"|"unknown"){controls=controls.push(button("Restart / Connect").on_press_maybe(can_change.then_some(Message::Restart(id.clone(),false))));}
            if server["tool_execution"]["uncertain_calls"].as_u64().unwrap_or(0)>0{controls=controls.push(button("Force restart (may interrupt work)").on_press_maybe(can_change.then_some(Message::Restart(id.clone(),true))));}
            let expanded=self.expanded_servers.contains(&id);
            let delete_hint=match self.deletions.get(&id){Some(Deletion::Pending(_))=>"Deleting server…",Some(Deletion::Refreshing)=>"Refreshing server status…",None if !self.can_delete(&id)=>"Server is busy; wait before deleting",None=>"Delete server"};
            let mut card=column![components::server_header(&title,expanded,Message::ToggleServer(id.clone()),self.can_delete(&id).then_some(Message::Delete(id.clone())),delete_hint)].spacing(theme::CONTROL_GAP);
            if expanded { card=card.push(text(id.clone()).size(theme::SMALL_TEXT)).push(controls);
            if auth!="not_required"{
                if let Some(flow)=self.authentication.get(&id){card=card.push(self.authentication_view(&id,server,flow));}
                else if auth=="authenticating"{card=card.push(text("Complete sign-in in your browser. Fetch is checking authentication status…"));}
            }

            card=card.push(text(format!("Connection access expires: {}",server["authentication"]["access_expires_at"].as_str().unwrap_or("unknown"))))
                .push(text(format!("Last verified connection access: {}",server["authentication"]["last_verified_at"].as_str().unwrap_or("unknown"))));
            if let Some(error)=server["authentication"]["error"]["message"].as_str(){card=card.push(components::message(error));}
            if let Some(error)=server["runtime"]["error"]["message"].as_str(){card=card.push(components::message(error));}
            if let Some(execution)=server.get("tool_execution"){card=card.push(text(format!("Active: {}   Waiting: {}   Uncertain: {}",execution["active_calls"],execution["queued_calls"],execution["uncertain_calls"])));}
            if self.restarts.contains_key(&id){card=card.push(text("Restarting… waiting for this attempt to finish."));}
            }
            items=items.push(components::server_card(card));
        }}
        if !self.tools.is_null(){items=items.push(text("Tool definitions").size(theme::SECTION_TITLE));if let Some(origins)=self.tools["origins"].as_array(){for origin in origins{items=items.push(text(format!("{} · {}",origin["source"]["server_id"].as_str().unwrap_or("server"),origin["status"].as_str().unwrap_or("unknown"))));if let Some(tools)=origin["tools"].as_array(){for tool in tools{let name=tool["name"].as_str().unwrap_or("Unnamed tool");let key=format!("{}:{name}",origin["source"]["server_id"].as_str().unwrap_or(""));let expanded=self.expanded_tools.contains(&key);items=items.push(components::tool_disclosure(name,expanded,expanded.then(||serde_json::to_string_pretty(tool).unwrap_or_default()),Message::ToggleTool(key)));}}if origin["status"]!="available"{items=items.push(text("This catalog is incomplete. Review its server state and rejected-tool details."));if let Some(rejected)=origin["rejected_tools"].as_array(){for rejection in rejected{items=items.push(text(format!("{}: {}",rejection["name"].as_str().unwrap_or("Tool"),rejection["message"].as_str().unwrap_or("Compatibility issue"))));}}}}}}
        items.into()
    }
    fn folder_button(&self,id:String)->Element<'_,Message>{
        let enabled=!self.deletions.contains_key(&id)&&!self.opening_folders.contains(&id);
        button(if self.opening_folders.contains(&id){"Opening server folder…"}else{"Open server folder"})
            .on_press_maybe(enabled.then_some(Message::OpenFolder(id))).into()
    }
    fn authentication_view<'a>(&'a self,id:&str,server:&'a Value,flow:&'a AuthFlow)->Element<'a,Message>{
        let mut panel=column![text(flow.title()).size(theme::FIELD_TITLE)].spacing(authentication::SPACING);
        if let Some(error)=&flow.error{panel=panel.push(components::message(error));}
        if let Some(error)=&flow.read_error{panel=panel.push(components::message(error));}
        for instruction in &flow.guidance{panel=panel.push(components::message(instruction.as_str()));}
        if matches!(flow.phase,AuthPhase::Required|AuthPhase::Unknown|AuthPhase::Failed|AuthPhase::NeedsVerification){
            panel=panel.push(text("Use the server’s own configuration files and supported login process. Fetch cannot verify every server’s provider login."))
                .push(self.folder_button(id.to_owned()));
        }
        let references=authentication::credential_references(server);
        if !references.is_empty(){
            for (binding,reference) in references{
                panel=panel.push(button(text(format!("{binding} → {reference}"))).on_press_maybe((!flow.active()&&!self.auth_active(id)).then_some(Message::AuthName(id.to_owned(),reference))));
            }
            let name_id=id.to_owned();let value_id=id.to_owned();
            panel=panel.push(authentication::credential_editor(&flow.entry,&flow.secret,!self.busy&&!self.auth_active(id),
                move|value|Message::AuthName(name_id.clone(),value),move|value|Message::AuthValue(value_id.clone(),value),Message::AuthSave(id.to_owned()),None));
        }
        if matches!(flow.phase,AuthPhase::Authenticated|AuthPhase::NotRequired){panel=panel.push(text("Use Restart / Connect to start this server or apply changes made in its files."));}
        if !flow.active()&&!flow.reconcile{panel=panel.push(button("Hide authentication details").on_press(Message::AuthDismiss(id.to_owned())));}
        container(panel).padding(authentication::SPACING).into()
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
        let eligible=self.supported_selections();
        let all_selected=!eligible.is_empty()&&eligible.is_subset(&self.selected);
        let can_select_all=!self.busy&&self.picker_window.is_none()&&self.job.is_none()&&!eligible.is_empty();
        let available=!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let mut header=column![text("Discover and install").size(theme::PAGE_TITLE),text("Fetch automatically finds your global Claude Code, Codex and OpenCode configuration files. Select the servers you want to install. Imported credentials are never copied.")].spacing(theme::PAGE_GAP);
        if self.custom_config {
            header=header.push(text("Choose a home folder, a .codex folder, or one configuration file to use instead."))
                .push(row![text_input("Absolute folder or configuration file path",&self.config_path).on_input(Message::ConfigPath),button("Choose file or folder…").on_press_maybe(available.then_some(Message::ChooseConfigFile))].spacing(theme::CONTROL_GAP).align_y(iced::Center))
                .push(button("Use default locations").on_press_maybe(available.then_some(Message::CustomConfig(false))));
        }else{
            header=header.push(button("Use a different file or folder…").on_press_maybe(available.then_some(Message::CustomConfig(true))));
        }
        header=header.push(row![button(if self.busy{"Working…"}else{"Refresh servers"}).on_press_maybe(available.then_some(Message::Discover)),button(if all_selected{"Unselect all"}else{"Select all"}).on_press_maybe(can_select_all.then_some(Message::SelectAllServers))].spacing(theme::CONTROL_GAP));
        let mut items=column![].spacing(theme::PAGE_GAP);
        if let Some(error)=&self.error{items=items.push(components::error_message(error.as_str()));}
        if !self.notice.is_empty(){items=items.push(components::message(self.notice.as_str()));}
        let has_rows=self.discovery["configs"].as_array().is_some_and(|groups|groups.iter().any(|group|group["servers"].as_array().is_some_and(|rows|!rows.is_empty())));
        if !has_rows{items=items.push(text(if self.busy{"Looking for server definitions…"}else{"No server definitions found. Review the source messages below, refresh, or choose a different file or folder."}));}
        if let Some(groups)=self.discovery["configs"].as_array(){for group in groups{let path=group["path"].as_str().unwrap_or("").to_owned();items=items.push(text(path.clone()).size(theme::TOOL_TITLE));if let Some(rows)=group["servers"].as_array(){for server in rows{let name=server["name"].as_str().unwrap_or("").to_owned();let eligible=installable(server);let installed=server["installed"]==true;let selected=eligible&&self.selected.contains(&(path.clone(),name.clone()));let control=if installed{components::installed_checkbox()}else{checkbox(selected)};let mut control=control.label(format!("{} · {}{}",name,server["type"].as_str().unwrap_or("unsupported"),if installed{" · already installed"}else if server["source_enabled"]==false{" · disabled in source; opt in explicitly"}else{""}));if eligible{let path=path.clone();let name=name.clone();control=control.on_toggle(move|value|Message::Select(path.clone(),name.clone(),value));}items=items.push(control);}}if let Some(error)=group["error"]["message"].as_str(){items=items.push(components::message(error));}}}
        if !self.progress.is_null(){items=items.push(text(format!("Installation: {}",self.progress["status"].as_str().unwrap_or("pending"))));if let Some(rows)=self.progress["servers"].as_array().or_else(||self.progress["items"].as_array()){for item in rows{items=items.push(text(format!("{} · {}",item["server_name"].as_str().unwrap_or("server"),item["status"].as_str().unwrap_or("pending"))));if let Some(error)=item["error"]["message"].as_str(){items=items.push(components::message(error));}}}}
        let selected_count=self.selected_for_install().len();
        let can_install=selected_count>0&&!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let footer=row![button("Install selected servers").on_press_maybe(can_install.then_some(Message::Install)),text(format!("{} selected",selected_count))].spacing(theme::PAGE_GAP).align_y(iced::Center);
        column![header,scrollable(items).id("fetch-install-discovery").width(iced::Fill).height(iced::Fill),footer].spacing(theme::PAGE_GAP).height(iced::Fill).into()
    }
    fn connect_view(&self)->Element<'_,Message>{
        let endpoint=format!("{}/mcp",self.host.state.origin);
        let mut items=column![text("Connect your coding apps").size(theme::PAGE_TITLE),text("Each app connects once to Fetch and keeps its own renewable token. Review and approve its access in your browser, then confirm on your Mac. Provider logins remain separate."),text(endpoint.clone()).size(theme::FIELD_TITLE),button("Copy Fetch MCP address").on_press(Message::Copy(endpoint.clone())),text(format!("Public certificate: {}",self.host.certificate_path.display())),text("The certificate must be explicitly trusted before the browser and clients can connect normally. Fetch never changes trust or client configurations automatically."),text("Claude Code"),text(format!("claude mcp add --transport http fetch {endpoint}")),text("Codex"),text(format!("codex mcp add fetch --url {endpoint}\ncodex mcp login fetch")),text("OpenCode"),text(format!("Configure a remote MCP named fetch with URL {endpoint}, then use its MCP authentication flow.")),checkbox(self.cleanup_confirmed).label("I have connected my apps, completed required provider authentication and verified the imported tools work.").on_toggle(Message::CleanupConfirmed)].spacing(theme::PAGE_GAP);
        if self.cleanup_confirmed{let prompt=self.cleanup_prompt();items=items.push(text("After verification, paste this into your coding agent to review cleanup:")).push(text(prompt.clone()).size(theme::CLEANUP_TEXT)).push(button("Copy cleanup prompt").on_press(Message::Copy(prompt)));}
        items.into()
    }
    fn cleanup_prompt(&self)->String{
        let mut names=Vec::new();if let Some(servers)=self.servers["servers"].as_array(){for server in servers{names.push(server["name"].as_str().unwrap_or("unnamed").to_owned());}}
        format!("I have verified Fetch's connection, provider authentication and imported tools. Review my original MCP configuration entries for these migrated servers: {}. Identify the exact original file/name pairs and show me the intended cleanup. Remove only those original entries after matching them to successful Fetch imports. Preserve the Fetch connection, unrelated entries and all other settings. Do not delete credentials, server data or packages. If the mapping or verification is unclear, ask me before changing it.",names.join(", "))
    }
    fn settings_view(&self)->Element<'_,Message>{
        let mut items=column![text("Server settings").size(theme::PAGE_TITLE),
            text("Choose Settings on an installed server. Use the server’s own files and authentication process; use Restart / Connect after changes when needed."),
            text(format!("Selected server: {}",if self.selected_server.is_empty(){"None"}else{&self.selected_server}))].spacing(theme::PAGE_GAP);
        if self.servers["servers"].as_array().is_some_and(|rows|rows.iter().any(|server|server["server_id"].as_str()==Some(self.selected_server.as_str()))){
            items=items.push(self.folder_button(self.selected_server.clone()))
                .push(text("Use the server’s normal setup first. For a required manual API key or fixed token, use an existing assigned entry below."))
                .push(text("Fetch call limits").size(theme::FIELD_TITLE))
                .push(text("Seconds allowed for each call. A longer tool-specific limit can override this default."))
                .push(text_input("120",&self.timeout).on_input(Message::Timeout))
                .push(text("Calls allowed at once. Additional calls wait in the server’s queue."))
                .push(text_input("1",&self.concurrency).on_input(Message::Concurrency))
                .push(button("Save call limits").on_press_maybe((!self.busy&&!self.auth_active(&self.selected_server)).then_some(Message::SaveExecution)));
            if let Some(server)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|server|server["server_id"].as_str()==Some(self.selected_server.as_str()))){
                let references=authentication::credential_references(server);
                if !references.is_empty(){
                    for (binding,reference) in references{items=items.push(button(text(format!("{binding} → {reference}"))).on_press_maybe((!self.busy&&!self.auth_active(&self.selected_server)).then_some(Message::SecretName(reference))));}
                    items=items.push(authentication::credential_editor(&self.secret_name,&self.secret_value,!self.busy&&!self.auth_active(&self.selected_server),Message::SecretName,Message::SecretValue,Message::SaveSecret,Some(Message::RemoveSecret)));
                }
            }
        }
        items.into()
    }

}

pub fn run(host:Arc<ApplicationHost>)->iced::Result{
    iced::daemon(move||Desktop::boot(host.clone()),Desktop::update,Desktop::view)
        .title(|_:&Desktop,_:window::Id|"Fetch".to_owned()).theme(|_: &Desktop, _: window::Id| theme::light())
        .subscription(Desktop::subscription).run()
}
