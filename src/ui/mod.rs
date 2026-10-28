//! Native API-backed interface. No workflow reads next_steps as executable state.
pub mod client;
mod file_picker;
use crate::{application::ApplicationHost, native_auth::NativeSettingsSession};
use client::{NativeClient, component};
use iced::{Element, Subscription, Task, Theme, window, widget::{button, checkbox, column, container, row, scrollable, text, text_input}};
use reqwest::Method;
use serde_json::{Value, json};
use std::{collections::{BTreeMap,BTreeSet}, sync::Arc, time::Duration};
use zeroize::Zeroize;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Page { Servers, Install, Connect, Settings }
#[derive(Clone)]
enum Message {
    Page(Page), Opened(window::Id), Closed(window::Id), Tick, Show, Quit, Stopped,
    Refresh, Servers(Result<Value,String>), Discover, ConfigPath(String), ChooseConfigFile, ConfigFileChosen(window::Id,Result<Option<String>,String>), Discovery(Result<Value,String>),
    Select(String,String,bool), SelectAllServers, Install, Installed(Result<Value,String>), Progress(Result<Value,String>),
    Inspect(String), Tools(Result<Value,String>), Authenticate(String), Restart(String,bool),
    Changed(String,Result<Value,String>), RestartStatus(String,Result<Value,String>),
    Choose(String), Config(String), SaveConfig, Timeout(String), Concurrency(String), SaveExecution, Advanced(bool), SecretName(String), SecretValue(String), SaveSecret, RemoveSecret,
    Copy(String), CleanupConfirmed(bool), ToggleServer(String), ToggleTool(String),
}
impl std::fmt::Debug for Message { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("FetchNativeMessage") } }
struct Desktop {
    host: Arc<ApplicationHost>, client: Option<NativeClient>, session: Option<Arc<NativeSettingsSession>>, window:Option<window::Id>,
    page:Page, servers:Value, discovery:Value, tools:Value, selected:BTreeSet<(String,String)>,
    config_path:String, picker_window:Option<window::Id>, selected_server:String, config:String, timeout:String, concurrency:String, advanced:bool, secret_name:String, secret_value:String,
    job:Option<String>, progress:Value, restarts:BTreeMap<String,String>, error:Option<String>, notice:String,
    busy:bool, polling:bool, cleanup_confirmed:bool, expanded_servers:BTreeSet<String>, expanded_tools:BTreeSet<String>,
}
impl Desktop {
    fn boot(host:Arc<ApplicationHost>)->(Self,Task<Message>) {
        let session=Arc::new(host.native.open_settings());
        let client=NativeClient::new(host.state.origin.clone(),&host.certificate_pem,session.clone());
        let (id,open)=window::open(window::Settings { size:iced::Size::new(1080.0,760.0),min_size:Some(iced::Size::new(760.0,520.0)),..Default::default() });
        let mut state=Self {host,client:None,session:Some(session),window:Some(id),page:Page::Servers,servers:json!({"servers":[]}),discovery:Value::Null,tools:Value::Null,selected:BTreeSet::new(),config_path:String::new(),picker_window:None,selected_server:String::new(),config:"{}".into(),timeout:"120".into(),concurrency:"1".into(),advanced:false,secret_name:String::new(),secret_value:String::new(),job:None,progress:Value::Null,restarts:BTreeMap::new(),error:None,notice:String::new(),busy:false,polling:false,cleanup_confirmed:false,expanded_servers:BTreeSet::new(),expanded_tools:BTreeSet::new()};
        match client {Ok(client)=>state.client=Some(client),Err(error)=>state.error=Some(error)}
        let read=state.refresh();
        (state,Task::batch([open.map(Message::Opened),read]))
    }
    fn refresh(&self)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        Task::perform(async move{client.servers().await},Message::Servers)
    }
    fn subscription(&self)->Subscription<Message> {
        Subscription::batch([iced::time::every(Duration::from_secs(1)).map(|_|Message::Tick),window::close_events().map(Message::Closed)])
    }
    fn operation(&mut self,id:String,method:Method,suffix:&str,body:Value)->Task<Message> {
        let Some(client)=self.client.clone() else{return Task::none();};
        if self.busy{return Task::none();} self.busy=true;self.error=None;self.cleanup_confirmed=false;
        let path=format!("/api/v1/servers/{}{suffix}",component(&id));
        Task::perform(async move{client.request(method,&path,Some(body)).await},move |result|Message::Changed(id.clone(),result))
    }
    fn update(&mut self,message:Message)->Task<Message> {
        match message {
            Message::Page(page)=>{self.page=page;self.error=None;}
            Message::Opened(id)=>{self.window=Some(id);crate::native_auth::install_app_menu();}
            Message::Closed(id)=>{if self.window==Some(id){self.window=None;self.picker_window=None;file_picker::cancel();self.host.native.invalidate_all();self.client=None;self.session=None;self.secret_value.zeroize();}}
            Message::Show=>{
                if let Some(id)=self.window{return window::gain_focus(id);}
                if self.window.is_none(){let session=Arc::new(self.host.native.open_settings());match NativeClient::new(self.host.state.origin.clone(),&self.host.certificate_pem,session.clone()){Ok(client)=>self.client=Some(client),Err(error)=>self.error=Some(error)}self.session=Some(session);let(id,open)=window::open(window::Settings{size:iced::Size::new(1080.0,760.0),min_size:Some(iced::Size::new(760.0,520.0)),..Default::default()});self.window=Some(id);return Task::batch([open.map(Message::Opened),self.refresh()]);}
            }
            Message::Quit=>{self.picker_window=None;file_picker::cancel();self.secret_value.zeroize();self.client=None;self.session=None;let host=self.host.clone();return Task::perform(async move{host.shutdown().await;},|_|Message::Stopped);}
            Message::Stopped=>return iced::exit(),
            Message::Refresh=>return self.refresh(),
            Message::Servers(result)=>{self.polling=false;match result {Ok(value)=>{self.servers=value;},Err(error)=>self.error=Some(error)}}
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
                match result{Ok(Some(path))=>self.config_path=path,Ok(None)=>{},Err(error)=>self.error=Some(error)}
            }
            Message::Discover=>{if self.busy||self.picker_window.is_some(){return Task::none();}if let Some(client)=self.client.clone(){self.busy=true;let path=self.config_path.clone();return Task::perform(async move{client.discovery(&path).await},Message::Discovery);}}
            Message::Discovery(result)=>{self.busy=false;match result {Ok(value)=>{self.selected.clear();if let Some(groups)=value["configs"].as_array(){for group in groups {if let Some(rows)=group["servers"].as_array(){for item in rows{if item["selected_by_default"]==true{if let(Some(path),Some(name))=(group["path"].as_str(),item["name"].as_str()){self.selected.insert((path.into(),name.into()));}}}}}}self.discovery=value;},Err(error)=>self.error=Some(error)}}
            Message::Select(path,name,selected)=>{if selected{self.selected.insert((path,name));}else{self.selected.remove(&(path,name));}}
            Message::SelectAllServers=>{
                if self.busy||self.picker_window.is_some()||self.job.is_some(){return Task::none();}
                self.selected=self.supported_selections();
            }
            Message::Install=>{self.cleanup_confirmed=false;if self.busy||self.picker_window.is_some()||self.job.is_some()||self.selected.is_empty(){return Task::none();}if let Some(client)=self.client.clone(){self.busy=true;let rows=self.selected.iter().cloned().collect();return Task::perform(async move{client.install(rows).await},Message::Installed);}}
            Message::Installed(result)=>{self.busy=false;match result {Ok(value)=>{self.job=value["installation_id"].as_str().map(str::to_owned);let existing=value["error"]["code"]=="INSTALLATION_IN_PROGRESS";self.progress=value;self.notice=if existing{"An installation is already running. Monitoring that existing operation.".into()}else{"Installation accepted. Preparation and authentication are separate stages.".into()};},Err(error)=>self.error=Some(error)}}
            Message::Progress(result)=>{self.polling=false;match result {Ok(value)=>{let terminal=matches!(value["status"].as_str(),Some("succeeded"|"failed"));if terminal {
                    self.selected.clear();
                    if let Some(rows)=value["servers"].as_array(){for item in rows {if item["status"]=="failed" {if let (Some(path),Some(name))=(item["config_path"].as_str(),item["server_name"].as_str()){self.selected.insert((path.into(),name.into()));}}}}
                }
                self.progress=value;if terminal{self.job=None;return self.refresh();}},Err(error)=>self.error=Some(error)}}
            Message::Inspect(id)=>{if let Some(client)=self.client.clone(){let query=url::form_urlencoded::Serializer::new(String::new()).append_pair("server_id",&id).finish();return Task::perform(async move{client.request(Method::GET,&format!("/api/v1/tools?{query}"),None).await},Message::Tools);}}
            Message::Tools(result)=>match result{Ok(value)=>self.tools=value,Err(error)=>self.error=Some(error)},
            Message::Authenticate(id)=>return self.operation(id,Method::POST,"/authenticate",json!({})),
            Message::Restart(id,force)=>return self.operation(id,Method::POST,"/restart",json!({"force_restart":force})),
            Message::Changed(id,result)=>{self.busy=false;match result{Ok(value)=>{if let Some(restart)=value["restart_id"].as_str(){self.restarts.insert(id,restart.into());}self.notice=if value["status"]=="authenticating"{"Complete the provider's browser sign-in, then Fetch will show the result.".into()}else{"Saved. Review server state for any required next action.".into()};return self.refresh();},Err(error)=>{self.error=Some(error);return self.refresh();}}}
            Message::RestartStatus(id,result)=>{self.polling=false;match result{Ok(value)=>{let restart=value["servers"].as_array().and_then(|records|records.first()).map(|record|&record["restart"]);if let Some(restart)=restart{if matches!(restart["status"].as_str(),Some("succeeded"|"failed")){self.restarts.remove(&id);self.notice=if restart["status"]=="succeeded"{"Server restart completed.".into()}else{"Server restart failed. Review its state before retrying.".into()};return self.refresh();}}},Err(error)=>{self.error=Some(error);}}}
            Message::Choose(id)=>{self.selected_server=id;self.config="{}".into();self.secret_value.zeroize();self.page=Page::Settings;self.advanced=false;
                if let Some(record)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|row|row["server_id"].as_str()==Some(self.selected_server.as_str()))){self.timeout=(record["config"]["execution"]["default_timeout_ms"].as_u64().unwrap_or(120000)/1000).to_string();self.concurrency=record["config"]["execution"]["max_concurrent_calls"].as_u64().unwrap_or(1).to_string();}
            }
            Message::Config(value)=>self.config=value,
            Message::Timeout(value)=>self.timeout=value,
            Message::Concurrency(value)=>self.concurrency=value,
            Message::Advanced(value)=>self.advanced=value,
            Message::SaveExecution=>{
                let values=self.timeout.parse::<u64>().ok().and_then(|seconds|seconds.checked_mul(1000)).zip(self.concurrency.parse::<u64>().ok());
                match values{Some((timeout,concurrency)) if timeout>0&&timeout<=86400000&&concurrency>0&&concurrency<=64=>return self.operation(self.selected_server.clone(),Method::PATCH,"/config",json!({"execution":{"default_timeout_ms":timeout,"max_concurrent_calls":concurrency}})),_=>self.error=Some("Enter 1–86400 seconds and 1–64 concurrent calls.".into())}
            }
            Message::SaveConfig=>match serde_json::from_str::<Value>(&self.config){Ok(body) if body.is_object()=>return self.operation(self.selected_server.clone(),Method::PATCH,"/config",body),_=>self.error=Some("Enter a JSON object containing the settings you want to change.".into())},
            Message::SecretName(value)=>self.secret_name=value,
            Message::SecretValue(value)=>{self.secret_value.zeroize();self.secret_value=value;}
            Message::SaveSecret|Message::RemoveSecret=>{let remove=matches!(message,Message::RemoveSecret);if self.busy{return Task::none();}if self.selected_server.is_empty()||self.secret_name.is_empty(){self.error=Some("Select a server and enter the credential entry name.".into());return Task::none();}let body=if remove{json!({"set":[],"remove":[self.secret_name.clone()]})}else{json!({"set":[{"name":self.secret_name.clone(),"value":self.secret_value.clone()}],"remove":[]})};self.secret_value.zeroize();return self.operation(self.selected_server.clone(),Method::PATCH,"/credentials",body);}
            Message::Copy(value)=>return iced::clipboard::write(value),
            Message::CleanupConfirmed(value)=>self.cleanup_confirmed=value,
            Message::ToggleServer(id)=>{if !self.expanded_servers.remove(&id){self.expanded_servers.insert(id);}}
            Message::ToggleTool(id)=>{if !self.expanded_tools.remove(&id){self.expanded_tools.insert(id);}}
            Message::Tick=>{
                if self.host.is_stopped(){return iced::exit();}
                match crate::native_auth::take_app_action(){1=>return self.update(Message::Show),2=>return self.update(Message::Quit),_=>{}}
                if self.host.take_show_request(){return self.update(Message::Show);}
                if self.client.is_none()||self.polling{return Task::none();}
                let client=self.client.clone().expect("live client");
                if let Some(id)=self.job.clone(){self.polling=true;return Task::perform(async move{client.request(Method::GET,&format!("/api/v1/setup/installations/{}",component(&id)),None).await},Message::Progress);}
                if let Some((id,restart))=self.restarts.iter().next(){let id=id.clone();let query=url::form_urlencoded::Serializer::new(String::new()).append_pair("id",&id).append_pair("restart_id",restart).finish();self.polling=true;return Task::perform(async move{client.request(Method::GET,&format!("/api/v1/servers?{query}"),None).await},move|result|Message::RestartStatus(id.clone(),result));}
                if self.servers["servers"].as_array().is_some_and(|records|records.iter().any(|record|record["authentication"]["status"]=="authenticating")){self.polling=true;return self.refresh();}
            }
        }
        Task::none()
    }
    fn view(&self,_window:window::Id)->Element<'_,Message>{
        let nav=row![text("Fetch").size(30),button("Servers").on_press(Message::Page(Page::Servers)),button("Install MCPs").on_press(Message::Page(Page::Install)),button("Connect apps").on_press(Message::Page(Page::Connect)),button("Settings").on_press(Message::Page(Page::Settings)),button("Quit Fetch").on_press(Message::Quit)].spacing(12).align_y(iced::Center);
        let mut body=column![nav,text("Your MCP servers, shared through one local connection.").size(16)].spacing(20);
        if self.page==Page::Install{
            // Give the list a finite share of the window; its action footer never scrolls.
            return container(body.push(self.install_view()).height(iced::Fill)).padding(28).width(iced::Fill).height(iced::Fill).into();
        }
        if let Some(error)=&self.error{body=body.push(container(text(error)).padding(12));}
        if !self.notice.is_empty(){body=body.push(text(&self.notice));}
        body=body.push(match self.page{Page::Servers=>self.servers_view(),Page::Install=>self.install_view(),Page::Connect=>self.connect_view(),Page::Settings=>self.settings_view()});
        container(scrollable(body)).padding(28).width(iced::Fill).height(iced::Fill).into()
    }
    fn servers_view(&self)->Element<'_,Message>{
        let mut items=column![row![text("Installed servers").size(24),button("Refresh").on_press(Message::Refresh)].spacing(20)].spacing(16);
        if let Some(servers)=self.servers["servers"].as_array(){if servers.is_empty(){items=items.push(text("No servers installed yet. Open Install MCPs to discover your existing definitions."));}for server in servers{
            let id=server["server_id"].as_str().unwrap_or("").to_owned();let runtime=server["runtime"]["status"].as_str().unwrap_or("unknown");let auth=server["authentication"]["status"].as_str().unwrap_or("unknown");
            let title=format!("{}  ·  {}  ·  authentication: {}",server["name"].as_str().unwrap_or(&id),runtime,auth);
            let mut controls=row![button("Inspect tools").on_press(Message::Inspect(id.clone())),button("Edit settings").on_press(Message::Choose(id.clone()))].spacing(10);
            if matches!(auth,"required"|"failed"|"unknown"){controls=controls.push(button("Authenticate").on_press(Message::Authenticate(id.clone())));}
            if server["restart_required"]==true||matches!(runtime,"stopped"|"failed"|"unknown"){controls=controls.push(button("Restart / Connect").on_press(Message::Restart(id.clone(),false)));}
            if server["tool_execution"]["uncertain_calls"].as_u64().unwrap_or(0)>0{controls=controls.push(button("Force restart (may interrupt work)").on_press(Message::Restart(id.clone(),true)));}
            let expanded=self.expanded_servers.contains(&id);
            let mut card=column![button(text(format!("{} {title}",if expanded{"▾"}else{"▸"})).size(19)).on_press(Message::ToggleServer(id.clone()))].spacing(10);
            if expanded { card=card.push(text(id.clone()).size(12)).push(controls);
            if let Some(error)=server["authentication"]["error"]["message"].as_str(){card=card.push(text(error));}
            if let Some(error)=server["runtime"]["error"]["message"].as_str(){card=card.push(text(error));}
            if let Some(execution)=server.get("tool_execution"){card=card.push(text(format!("Active: {}   Waiting: {}   Uncertain: {}",execution["active_calls"],execution["queued_calls"],execution["uncertain_calls"])));}
            if self.restarts.contains_key(&id){card=card.push(text("Restarting… waiting for this attempt to finish."));}
            }
            items=items.push(container(card).padding(16).style(container::rounded_box));
        }}
        if !self.tools.is_null(){items=items.push(text("Tool definitions").size(22));if let Some(origins)=self.tools["origins"].as_array(){for origin in origins{items=items.push(text(format!("{} · {}",origin["source"]["server_id"].as_str().unwrap_or("server"),origin["status"].as_str().unwrap_or("unknown"))));if let Some(tools)=origin["tools"].as_array(){for tool in tools{let name=tool["name"].as_str().unwrap_or("Unnamed tool");let key=format!("{}:{name}",origin["source"]["server_id"].as_str().unwrap_or(""));let expanded=self.expanded_tools.contains(&key);let mut detail=column![button(text(format!("{} {name}",if expanded{"▾"}else{"▸"})).size(17)).on_press(Message::ToggleTool(key))];if expanded{detail=detail.push(text(serde_json::to_string_pretty(tool).unwrap_or_default()).size(12));}items=items.push(container(detail).padding(10));}}if origin["status"]!="available"{items=items.push(text("This catalog is incomplete. Review its server state and rejected-tool details."));if let Some(rejected)=origin["rejected_tools"].as_array(){for rejection in rejected{items=items.push(text(format!("{}: {}",rejection["name"].as_str().unwrap_or("Tool"),rejection["message"].as_str().unwrap_or("Compatibility issue"))));}}}}}}
        items.into()
    }
    fn supported_selections(&self)->BTreeSet<(String,String)>{
        let mut selections=BTreeSet::new();
        if let Some(groups)=self.discovery["configs"].as_array(){for group in groups{
            if let (Some(path),Some(rows))=(group["path"].as_str(),group["servers"].as_array()){for server in rows{
                if server["status"]=="supported"{if let Some(name)=server["name"].as_str(){selections.insert((path.to_owned(),name.to_owned()));}}
            }}
        }}
        selections
    }
    fn install_view(&self)->Element<'_,Message>{
        let can_select_all=!self.busy&&self.picker_window.is_none()&&self.job.is_none()&&!self.supported_selections().is_empty();
        let header=column![text("Discover and install").size(24),text("Read your global Claude Code, Codex and OpenCode definitions, or choose one absolute configuration path. Imported credentials are never copied."),row![text_input("Optional absolute configuration path",&self.config_path).on_input(Message::ConfigPath),button("Choose file…").on_press_maybe((!self.busy&&self.picker_window.is_none()&&self.job.is_none()).then_some(Message::ChooseConfigFile))].spacing(10).align_y(iced::Center),row![button(if self.busy{"Working…"}else{"Discover servers"}).on_press_maybe((!self.busy&&self.picker_window.is_none()).then_some(Message::Discover)),button("Select all").on_press_maybe(can_select_all.then_some(Message::SelectAllServers))].spacing(10)].spacing(14);
        let mut items=column![].spacing(14);
        if let Some(error)=&self.error{items=items.push(container(text(error)).padding(12));}
        if !self.notice.is_empty(){items=items.push(text(&self.notice));}
        let has_rows=self.discovery["configs"].as_array().is_some_and(|groups|groups.iter().any(|group|group["servers"].as_array().is_some_and(|rows|!rows.is_empty())));
        if !has_rows{items=items.push(text("No server definitions to select. Discover servers or review the source messages below."));}
        if let Some(groups)=self.discovery["configs"].as_array(){for group in groups{let path=group["path"].as_str().unwrap_or("").to_owned();items=items.push(text(path.clone()).size(17));if let Some(rows)=group["servers"].as_array(){for server in rows{let name=server["name"].as_str().unwrap_or("").to_owned();let eligible=server["status"]=="supported";let selected=self.selected.contains(&(path.clone(),name.clone()));let mut control=checkbox(selected).label(format!("{} · {}{}",name,server["type"].as_str().unwrap_or("unsupported"),if server["source_enabled"]==false{" · disabled in source; opt in explicitly"}else{""}));if eligible{let path=path.clone();let name=name.clone();control=control.on_toggle(move|value|Message::Select(path.clone(),name.clone(),value));}items=items.push(control);}}if let Some(error)=group["error"]["message"].as_str(){items=items.push(text(error));}}}
        if !self.progress.is_null(){items=items.push(text(format!("Installation: {}",self.progress["status"].as_str().unwrap_or("pending"))));if let Some(rows)=self.progress["servers"].as_array().or_else(||self.progress["items"].as_array()){for item in rows{items=items.push(text(format!("{} · {}",item["server_name"].as_str().unwrap_or("server"),item["status"].as_str().unwrap_or("pending"))));if let Some(error)=item["error"]["message"].as_str(){items=items.push(text(error));}}}}
        let can_install=!self.selected.is_empty()&&!self.busy&&self.picker_window.is_none()&&self.job.is_none();
        let footer=row![button("Install selected servers").on_press_maybe(can_install.then_some(Message::Install)),text(format!("{} selected",self.selected.len()))].spacing(14).align_y(iced::Center);
        column![header,scrollable(items).id("fetch-install-discovery").width(iced::Fill).height(iced::Fill),footer].spacing(14).height(iced::Fill).into()
    }
    fn connect_view(&self)->Element<'_,Message>{
        let endpoint=format!("{}/mcp",self.host.state.origin);
        let mut items=column![text("Connect your coding apps").size(24),text("Each app connects once to Fetch and keeps its own renewable token. Review and approve its access in your browser, then confirm on your Mac. Provider logins remain separate."),text(endpoint.clone()).size(20),button("Copy Fetch MCP address").on_press(Message::Copy(endpoint.clone())),text(format!("Public certificate: {}",self.host.certificate_path.display())),text("The certificate must be explicitly trusted before the browser and clients can connect normally. Fetch never changes trust or client configurations automatically."),text("Claude Code"),text(format!("claude mcp add --transport http fetch {endpoint}")),text("Codex"),text(format!("codex mcp add fetch --url {endpoint}\ncodex mcp login fetch")),text("OpenCode"),text(format!("Configure a remote MCP named fetch with URL {endpoint}, then use its MCP authentication flow.")),checkbox(self.cleanup_confirmed).label("I have connected my apps, completed required provider authentication and verified the imported tools work.").on_toggle(Message::CleanupConfirmed)].spacing(14);
        if self.cleanup_confirmed{let prompt=self.cleanup_prompt();items=items.push(text("After verification, paste this into your coding agent to review cleanup:")).push(text(prompt.clone()).size(13)).push(button("Copy cleanup prompt").on_press(Message::Copy(prompt)));}
        items.into()
    }
    fn cleanup_prompt(&self)->String{
        let mut names=Vec::new();if let Some(servers)=self.servers["servers"].as_array(){for server in servers{names.push(server["name"].as_str().unwrap_or("unnamed").to_owned());}}
        format!("I have verified Fetch's connection, provider authentication and imported tools. Review my original MCP configuration entries for these migrated servers: {}. Identify the exact original file/name pairs and show me the intended cleanup. Remove only those original entries after matching them to successful Fetch imports. Preserve the Fetch connection, unrelated entries and all other settings. Do not delete credentials, server data or packages. If the mapping or verification is unclear, ask me before changing it.",names.join(", "))
    }
    fn settings_view(&self)->Element<'_,Message>{
        let mut items=column![text("Server settings").size(24),text("Choose Edit settings on an installed server. Saved launch changes need an explicit Restart or Reconnect."),text(format!("Selected server: {}",if self.selected_server.is_empty(){"None"}else{&self.selected_server})),text("Call limits").size(20),text("Seconds allowed for each call. A longer tool-specific limit can override this default."),text_input("120",&self.timeout).on_input(Message::Timeout),text("Calls allowed at once. Additional calls wait in the server's queue."),text_input("1",&self.concurrency).on_input(Message::Concurrency),button("Save call limits").on_press(Message::SaveExecution),text("Private credential entry").size(20),text("Save a secret assigned to this server. Each change needs fresh Mac confirmation. Fetch verifies the newly stored credential with the provider before reporting success."),text_input("Entry name",&self.secret_name).on_input(Message::SecretName),text_input("Secret value",&self.secret_value).secure(true).on_input(Message::SecretValue),row![button("Save credential").on_press(Message::SaveSecret),button("Remove entry").on_press(Message::RemoveSecret)].spacing(10)].spacing(14);
        if let Some(server)=self.servers["servers"].as_array().and_then(|rows|rows.iter().find(|r|r["server_id"].as_str()==Some(self.selected_server.as_str()))){
            if let Some(credentials)=server["credentials"].as_array(){for entry in credentials{let name=entry["name"].as_str().unwrap_or("").to_owned();items=items.push(row![button(text(name.clone())).on_press(Message::SecretName(name)),text(entry["status"].as_str().unwrap_or("unknown"))].spacing(12));}}
            items=items.push(checkbox(self.advanced).label("Show advanced launch and per-tool settings").on_toggle(Message::Advanced));
            if self.advanced{items=items.push(text("Saved non-secret configuration")).push(text(serde_json::to_string_pretty(&server["config"]).unwrap_or_default()).size(12)).push(text("Configuration patch (JSON). Omitted fields keep their current values."))
                .push(text_input("{}",&self.config).on_input(Message::Config))
                .push(text("Use args, working_directory, environment references, remote header references or execution.tool_timeouts_ms. Enter actual secret values only in the private field above."))
                .push(button("Save configuration patch").on_press(Message::SaveConfig));}
        }
        items.into()
    }
}

pub fn run(host:Arc<ApplicationHost>)->iced::Result{
    iced::daemon(move||Desktop::boot(host.clone()),Desktop::update,Desktop::view)
        .title(|_:&Desktop,_:window::Id|"Fetch".to_owned()).theme(|_: &Desktop, _: window::Id| Theme::Light)
        .subscription(Desktop::subscription).run()
}
