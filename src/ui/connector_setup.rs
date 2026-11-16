//! Native connector approval. All approval and grant issuance stays in the APIs.
use super::{Desktop, Message, NativeClient, components, theme};
use iced::{
    Element, Task,
    widget::{column, row, text},
};
use reqwest::Method;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use zeroize::Zeroize;
use crate::connector::ConnectionContext;

fn request_context(request:&Value)->Option<ConnectionContext>{Some(ConnectionContext{client:request["client_name"].as_str()?.to_owned(),project_id:request["project_id"].as_str().map(str::to_owned)})}

const REQUESTS: &str = "/api/v1/connections/requests";
const DEFAULT_SCOPES: [&str; 3] = ["fetch:catalog", "fetch:tools:call", "fetch:content:read"];

#[derive(Clone)]
pub(super) enum Event {
    Start(&'static str),
    AutomaticContext(ConnectionContext),
    AutomaticStarted(String, Result<(), String>),
    RequestsAt(u64, Result<Value, String>),
    Dismiss,
    Created(String, Result<Value, String>),
    Requests(Result<Value, String>),
    Toggle(String, bool),
    Decide(bool),
    Decided(String, bool, Result<Value, String>),
    Saved(String, String, Result<(), String>),
    AutomaticSaved(String, String, Result<bool, String>),
    Reconciled(Result<(), String>),
    PendingCleared(String, Result<(), String>),
}

struct OwnedRequest {
    client: String,
    context: ConnectionContext,
    proof: String,
    expires_at: u64,
}
impl Drop for OwnedRequest {
    fn drop(&mut self) {
        self.proof.zeroize();
    }
}

#[derive(Default)]
pub(super) struct State {
    owned: BTreeMap<String, OwnedRequest>,
    request_contexts: BTreeMap<String, ConnectionContext>,
    claiming: BTreeSet<String>,
    ended: BTreeSet<String>,
    reconciled: bool,
    read_generation: u64,
    popup: Option<Value>,
    popup_hidden: bool,
    selected: BTreeSet<String>,
    polling: bool,
    pub(super) creating: bool,
    pub(super) deciding: bool,
    pub(super) status: String,
    error: Option<String>,
}

impl State {
    pub(super) fn popup_visible(&self) -> bool {
        self.popup.is_some() && !self.popup_hidden
    }
    pub(super) fn foreground_request(&mut self) {
        self.read_generation = self.read_generation.wrapping_add(1);
        self.popup_hidden = false;
        self.reconciled = false;
    }
    pub(super) fn window_closed(&mut self) {
        // Closing a window backgrounds Fetch. The setup caller still owns its
        // proof until the request ends, so reopening must not orphan approval.
        self.popup = None;
        self.popup_hidden = false;
        self.selected.clear();
        self.polling = false;
        self.reconciled = false;
        self.error = None;
    }
}

impl Desktop {
    pub(super) fn poll_connector_requests(&mut self) -> Option<Task<Message>> {
        if self.connector.polling {
            return None;
        }
        let client = self.client.clone()?;
        self.connector.polling = true;
        let generation = self.connector.read_generation;
        Some(Task::perform(
            async move { client.request(Method::GET, REQUESTS, None).await },
            move |result| Message::Connector(Event::RequestsAt(generation, result)),
        ))
    }

    pub(super) fn update_connector(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::RequestsAt(generation, result) => {
                self.connector.polling = false;
                if generation != self.connector.read_generation { return Task::none(); }
                return self.update_connector(Event::Requests(result));
            }
            Event::AutomaticContext(context) => {
                let name=context.client.clone();
                self.connector.foreground_request();
                self.connector.creating = true;
                let root = self.host.state.repository.root().to_owned();
                let origin = self.host.state.origin.clone();
                let port = url::Url::parse(&origin).ok().and_then(|url| url.port()).unwrap_or(crate::application::default_app_port());
                let saved_name = name.clone();
                return Task::perform(async move {
                    let store = crate::connector::profile::ProfileStore::for_context(root.clone(), context.clone())?;
                    let http = crate::connector::pinned_client(&crate::connector::Options { root, port, client: name, project_id:context.project_id })?;
                    let _ = store.ensure_pending(&http, &origin).await?;
                    Ok(())
                }, move |result| Message::Connector(Event::AutomaticStarted(saved_name.clone(), result)));
            }
            Event::AutomaticStarted(name, result) => {
                self.connector.creating = false;
                match result {
                    Ok(()) => self.connector.status = format!("Review the {name} connection request in Fetch."),
                    Err(error) => self.connector.error = Some(error),
                }
            }
            Event::Dismiss => {
                self.connector.popup_hidden = true;
            }
            Event::Start(client_name) => {
                if self.connector_matches(client_name) {
                    self.connector.popup_hidden = false;
                    return Task::none();
                }
                if self.connector.creating || self.connector.deciding {
                    return Task::none();
                }
                let Some(client) = self.client.clone() else {
                    return Task::none();
                };
                self.connector.read_generation = self.connector.read_generation.wrapping_add(1);
                self.connector.creating = true;
                self.connector.status.clear();
                self.connector.error = None;
                let name = client_name.to_owned();
                let project_id=self.projects.connection.project_id().map(str::to_owned);
                let mut scopes=DEFAULT_SCOPES.iter().map(|scope|(*scope).to_owned()).collect::<Vec<_>>();
                if project_id.is_some(){scopes.push("fetch:sessions:read".into());}
                return Task::perform(
                    async move {
                        client
                            .request(
                                Method::POST,
                                REQUESTS,
                                Some(json!({"client_name": client_name, "project_id":project_id, "scopes": scopes})),
                            )
                            .await
                    },
                    move |result| Message::Connector(Event::Created(name.clone(), result)),
                );
            }
            Event::Created(name, result) => {
                self.connector.creating = false;
                match result {
                    Ok(mut request) => {
                        let id = request["request_id"].as_str().unwrap_or("").to_owned();
                        let proof = request["claim_secret"]
                            .take()
                            .as_str()
                            .unwrap_or("")
                            .to_owned();
                        if id.is_empty() || proof.is_empty() {
                            self.connector.error =
                                Some("Fetch did not return a complete connection request.".into());
                            return Task::none();
                        }
                        let expires_at = request["expires_at"].as_u64().unwrap_or(0);
                        self.connector.owned.insert(
                            id,
                            OwnedRequest {
                                context:request_context(&request).unwrap_or_else(||ConnectionContext::global(name.clone())),
                                client: name,
                                proof,
                                expires_at,
                            },
                        );
                        if self.client.is_some() {
                            self.open_connector_popup(request);
                        }
                    }
                    Err(error) => self.connector.error = Some(error),
                }
            }
            Event::Requests(result) => {
                self.connector.polling = false;
                if self.client.is_none() {
                    return Task::none();
                }
                let mut followups = Vec::new();
                match result {
                    Ok(value) => {
                        if !self.connector.deciding {
                            let rows = value["requests"]
                                .as_array()
                                .map(Vec::as_slice)
                                .unwrap_or(&[]);
                            for request in rows {
                                if let (Some(id), Some(name)) = (request["request_id"].as_str(), request["client_name"].as_str()) {
                                    self.connector.request_contexts.insert(id.to_owned(), request_context(request).unwrap_or_else(||ConnectionContext::global(name.to_owned())));
                                }
                            }
                            for request in rows.iter().filter(|row| row["status"] == "denied") {
                                if let (Some(id), Some(name)) = (request["request_id"].as_str(), request["client_name"].as_str()) {
                                    if !self.connector.ended.contains(id) && !self.connector.claiming.contains(id) {
                                        followups.push(self.clear_automatic_request(id.to_owned(), request_context(request).unwrap_or_else(||ConnectionContext::global(name.to_owned()))));
                                    }
                                }
                            }
                            if let Some(request) = rows.iter().find(|request| request["status"] == "approved"
                                && request["request_id"].as_str().is_some_and(|id| !self.connector.owned.contains_key(id) && !self.connector.claiming.contains(id))) {
                                if let (Some(id), Some(name)) = (request["request_id"].as_str(), request["client_name"].as_str()) {
                                    return self.finish_automatic_connection(id.to_owned(), request_context(request).unwrap_or_else(||ConnectionContext::global(name.to_owned())));
                                }
                            }
                            if !self.connector.reconciled {
                                self.connector.reconciled = true;
                                let root = self.host.state.repository.root().to_owned();
                                let origin = self.host.state.origin.clone();
                                let client = self.client.clone().expect("live native session");
                                let mut profiles = self.connector.request_contexts.values().cloned().collect::<BTreeSet<_>>();
                                profiles.extend([ConnectionContext::global("claude-code".into()),ConnectionContext::global("codex".into())]);
                                followups.push(Task::perform(async move { reconcile_pending(root, &origin, &profiles, &client).await },
                                    |result| Message::Connector(Event::Reconciled(result))));
                            }
                            let before = self.connector.owned.len();
                            self.connector.owned.retain(|id, owned| {
                                owned.expires_at > crate::connector::profile::now()
                                    && !rows.iter().any(|row| {
                                        row["request_id"].as_str() == Some(id.as_str())
                                            && row["status"] == "denied"
                                    })
                            });
                            if self.connector.owned.len() != before {
                                self.connector.status =
                                    "Connection request ended. Set up the app again to retry."
                                        .into();
                            }
                            if let Some(active) = &self.connector.popup {
                                let id = active["request_id"].as_str();
                                if !rows.iter().any(|row| {
                                    row["request_id"].as_str() == id
                                        && matches!(
                                            row["status"].as_str(),
                                            Some("pending" | "confirming")
                                        )
                                }) && !id.is_some_and(|id| self.connector.owned.contains_key(id))
                                {
                                    if let Some(id) = id {
                                        if self.connector.owned.remove(id).is_some() {
                                            self.connector.status = "Connection request ended. Set up the app again to retry.".into();
                                        }
                                    }
                                    self.connector.popup = None;
                                }
                            }
                            if self.connector.popup.is_none() {
                                if let Some(request) =
                                    rows.iter().find(|row| row["status"] == "pending")
                                {
                                    self.open_connector_popup(request.clone());
                                    if let Some(id) = self.window {
                                        followups.push(iced::window::gain_focus(id));
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        if self.connector.popup.is_some() {
                            self.connector.error = Some(error);
                        }
                    }
                }
                return Task::batch(followups);
            }
            Event::Toggle(scope, selected) => {
                if self.connector.deciding {
                    return Task::none();
                }
                if selected {
                    self.connector.selected.insert(scope);
                } else {
                    self.connector.selected.remove(&scope);
                }
            }
            Event::Decide(allow) => {
                if self.connector.deciding {
                    return Task::none();
                }
                let Some(request) = self.connector.popup.as_ref() else {
                    return Task::none();
                };
                let Some(id) = request["request_id"].as_str().map(str::to_owned) else {
                    return Task::none();
                };
                let Some(client) = self.client.clone() else {
                    return Task::none();
                };
                if allow && self.connector.selected.is_empty() {
                    return Task::none();
                }
                let body = json!({"decision": if allow { "allow" } else { "deny" }, "scopes": self.connector.selected});
                self.connector.deciding = true;
                self.connector.error = None;
                let target = id.clone();
                return Task::perform(
                    async move {
                        client
                            .request(
                                Method::POST,
                                &format!("{REQUESTS}/{}/decision", super::component(&target)),
                                Some(body),
                            )
                            .await
                    },
                    move |result| Message::Connector(Event::Decided(id.clone(), allow, result)),
                );
            }
            Event::Decided(id, allow, result) => {
                self.connector.deciding = false;
                match result {
                    Ok(value) if !allow || value["status"] == "denied" => {
                        self.connector.owned.remove(&id);
                        self.connector.claiming.remove(&id);
                        self.connector.popup = None;
                        self.connector.status = "Connection denied. No access was granted.".into();
                        if let Some(name) = self.connector.request_contexts.get(&id).cloned() {
                            return self.clear_automatic_request(id, name);
                        }
                    }
                    Ok(value) if value["status"] == "approved" => {
                        self.connector.popup = None;
                        let Some(owned) = self.connector.owned.remove(&id) else {
                            if let Some(name) = self.connector.request_contexts.get(&id).cloned() {
                                return self.finish_automatic_connection(id, name);
                            }
                            self.connector.status = "Connection approved. Waiting for the requesting app to finish connecting.".into();
                            return Task::none();
                        };
                        self.connector.claiming.insert(id.clone());
                        let root = self.host.state.repository.root().to_owned();
                        let origin = self.host.state.origin.clone();
                        let name = owned.client.clone();
                        let context=owned.context.clone();
                        let saved_name = name.clone();
                        let saved_id = id.clone();
                        self.connector.status = format!("Saving the {name} connection…");
                        return Task::perform(async move {
                            let response = claim(root.clone(), &origin, &context, &id, &owned.proof).await?;
                            crate::connector::profile::save_from_response_for_context(root, context, response).await
                        }, move |result| Message::Connector(Event::Saved(saved_id.clone(), saved_name.clone(), result)));
                    }
                    Ok(_) => self.connector.error = Some("Fetch has not confirmed this approval. Review the request before retrying.".into()),
                    Err(error) => {
                        self.connector.error = Some(error);
                        // A Mac denial can arrive after the window session has
                        // closed. The durable proof can observe terminal denial
                        // without native Settings authority or a later UI poll.
                        if !self.connector.owned.contains_key(&id) {
                            if let Some(name) = self.connector.request_contexts.get(&id).cloned() {
                                return self.finish_automatic_connection(id, name);
                            }
                        }
                    }
                }
            }
            Event::Saved(id, name, result) => {
                self.connector.claiming.remove(&id);
                match result {
                    Ok(()) => self.connector.status = format!("{name} is approved. Reconnect in the coding app if it stopped waiting."),
                    Err(error) => self.connector.error = Some(format!("The connection could not be saved: {error} Retry approval in Fetch.")),
                }
            }
            Event::AutomaticSaved(id, name, result) => {
                self.connector.claiming.remove(&id);
                match result {
                    Ok(true) => self.connector.status = format!("{name} is approved. Reconnect in the coding app if it stopped waiting."),
                    Ok(false) => self.connector.status = "Fetch is still waiting for this connection to finish. Reconnect the coding app if needed.".into(),
                    Err(error) => self.connector.error = Some(error),
                }
            }
            Event::Reconciled(result) => {
                if let Err(error) = result { self.connector.error = Some(error); }
            }
            Event::PendingCleared(id, result) => {
                self.connector.claiming.remove(&id);
                match result {
                    Ok(()) => { self.connector.ended.insert(id); }
                    Err(error) => self.connector.error = Some(error),
                }
            }
        }
        Task::none()
    }

    fn finish_automatic_connection(&mut self, id: String, context: ConnectionContext) -> Task<Message> {
        if !self.connector.claiming.insert(id.clone()) { return Task::none(); }
        let root = self.host.state.repository.root().to_owned();
        let origin = self.host.state.origin.clone();
        let saved_id = id.clone();
        let saved_name = context.client.clone();
        self.connector.status = format!("Saving the {} connection…",context.client);
        Task::perform(async move { crate::connector::profile::complete_pending_for_context(root, context, origin, id).await },
            move |result| Message::Connector(Event::AutomaticSaved(saved_id.clone(), saved_name.clone(), result)))
    }

    fn clear_automatic_request(&mut self, id: String, context: ConnectionContext) -> Task<Message> {
        self.connector.claiming.insert(id.clone());
        let root = self.host.state.repository.root().to_owned();
        let origin = self.host.state.origin.clone();
        let saved_id = id.clone();
        Task::perform(async move {
            let store = crate::connector::profile::ProfileStore::for_context(root, context)?;
            store.clear_pending(&origin, &id).await.map(|_| ())
        }, move |result| Message::Connector(Event::PendingCleared(saved_id.clone(), result)))
    }

    fn open_connector_popup(&mut self, request: Value) {
        if let (Some(id), Some(name)) = (request["request_id"].as_str(), request["client_name"].as_str()) {
            self.connector.request_contexts.insert(id.to_owned(), request_context(&request).unwrap_or_else(||ConnectionContext::global(name.to_owned())));
        }
        self.connector.selected = request["scopes"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .filter(|scope| DEFAULT_SCOPES.contains(scope))
            .map(str::to_owned)
            .collect();
        self.connector.popup = Some(request);
        self.connector.popup_hidden = false;
        self.connector.error = None;
    }

    pub(super) fn connector_popup(&self) -> Element<'_, Message> {
        let request = self
            .connector
            .popup
            .as_ref()
            .expect("visible connector popup");
        let name = request["client_name"].as_str().unwrap_or("This coding app");
        let busy = self.connector.deciding || request["status"] == "confirming";
        let header=row![text(format!("Connect {name} to Fetch?")).size(26).font(theme::SEMIBOLD_FONT).width(iced::Fill),
            components::icon_button(components::Icon::Close,Some(Message::Connector(Event::Dismiss)))].spacing(16).align_y(iced::Center);
        let mut details = column![
            column![text("Request code"),components::code_block(request["code"].as_str().unwrap_or("").to_owned(),Message::Copy)].spacing(8),
            text(if request["project"].is_object(){format!("Project: {}\n{}\nAccess includes Shared servers and this project’s servers.",request["project"]["name"].as_str().unwrap_or(""),request["project"]["folder_path"].as_str().unwrap_or(""))}else{"Shared connection · Shared servers only".into()}),
            text("Choose access to current and future installed servers:"),
        ].spacing(16);
        for scope in request["scopes"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            let label = match scope {
                "fetch:catalog" => "See tool, resource and prompt descriptions",
                "fetch:tools:call" => "Call tools, including tools that change data",
                "fetch:content:read" => "Read resources and prompts",
                "fetch:manage" => "Manage installed servers",
                "fetch:sessions:read" if request["project_id"].is_string() => "Read this project’s session histories",
                _ => continue,
            };
            let key = scope.to_owned();
            let mut control =
                components::checkbox(self.connector.selected.contains(scope)).label(label);
            if !busy {
                control = control
                    .on_toggle(move |value| Message::Connector(Event::Toggle(key.clone(), value)));
            }
            details = details.push(control);
        }
        if busy {
            details = details.push(text(
                "Complete the Mac confirmation. No access is granted until it succeeds.",
            ));
        }
        if let Some(error) = &self.connector.error {
            details = details.push(components::error_message(error.as_str()));
        }
        let footer = row![
                components::text_button("Deny")
                    .on_press_maybe((!busy).then_some(Message::Connector(Event::Decide(false)))),
                components::primary_button("Allow").on_press_maybe(
                    (!busy && !self.connector.selected.is_empty())
                        .then_some(Message::Connector(Event::Decide(true)))
                ),
            ]
            .spacing(16);
        let content:Element<'_,Message>=if request["project_id"].is_string(){
            column![header,super::surface::contain_scroll(iced::widget::scrollable(details).style(theme::scrollable).height(iced::Fill)),footer].spacing(16).height(iced::Fill).into()
        }else{column![header,details,footer].spacing(16).into()};
        let popup=components::raised(iced::widget::container(content).padding(28).width(iced::Fill)).max_width(620);
        let popup=if request["project_id"].is_string(){popup.height(iced::Fill).max_height(560)}else{popup};
        components::modal(popup, Message::Connector(Event::Dismiss))
    }

    fn connector_matches(&self, client: &str) -> bool {
        self.connector.popup.as_ref().is_some_and(|request| {
            let matches_client=request["client_name"].as_str()==Some(client)
                ||request["request_id"].as_str().and_then(|id|self.connector.owned.get(id)).is_some_and(|owned|owned.client==client);
            matches_client&&request["project_id"].as_str()==self.projects.connection.project_id()
        })
    }

    pub(super) fn connector_heading(
        &self,
        client: &'static str,
        label: &str,
        logo: &'static [u8],
    ) -> iced::widget::Row<'_, Message> {
        let enabled = self.client.is_some()
            && !self.connector.creating
            && !self.connector.deciding
            && (self.connector.owned.is_empty() || self.connector_matches(client));
        components::app_heading(label, logo).push(components::app_auth_button(
            enabled.then_some(Message::Connector(Event::Start(client))),
        ))
    }

    pub(super) fn connector_feedback(&self) -> Option<Element<'_, Message>> {
        if self.connector.status.is_empty() && self.connector.error.is_none() {
            return None;
        }
        let mut feedback = column![].spacing(12);
        if !self.connector.status.is_empty() {
            feedback = feedback.push(text(self.connector.status.as_str()));
        }
        if let Some(error) = &self.connector.error {
            feedback = feedback.push(components::error_message(error.as_str()));
        }
        Some(feedback.into())
    }

    pub(super) fn connector_command(&self, client: &str) -> String {
        let executable = std::env::current_exe().unwrap_or_else(|_| "fetch".into());
        let quote = |value: &str| format!("'{}'", value.replace('\'', "'\\''"));
        let root=self.host.state.repository.root().to_string_lossy();
        let port=url::Url::parse(&self.host.state.origin).ok().and_then(|url|url.port()).unwrap_or(crate::application::default_app_port());
        let project=self.projects.connection.project_id();
        if client=="codex" {if let Some(id)=project {
            let value=json!({"mcp_servers":{"fetch":{"command":executable.to_string_lossy(),"args":["connect","--client",client,"--project",id,"--data-dir",root,"--port",port.to_string()]}}});
            return toml::to_string_pretty(&value).unwrap_or_default();
        }}
        let project_arg=project.map(|id|format!(" --project {}",quote(id))).unwrap_or_default();
        let launcher=format!("{} connect --client {client}{project_arg} --data-dir {} --port {port}",quote(&executable.to_string_lossy()),quote(&root));
        match client {"claude-code"=>format!("claude mcp add {}--transport stdio fetch -- {launcher}",if project.is_some(){"--scope project "}else{""}),_=>format!("codex mcp add fetch -- {launcher}")}
    }

}

async fn claim(
    root: std::path::PathBuf,
    origin: &str,
    context: &ConnectionContext,
    id: &str,
    proof: &str,
) -> Result<Value, String> {
    // Claim needs its private proof, not a live native-window capability. An
    // already approved request can finish saving after the window backgrounds.
    let port = url::Url::parse(origin)
        .ok()
        .and_then(|url| url.port())
        .ok_or("Fetch's connection address is unavailable.")?;
    let http = crate::connector::pinned_client(&crate::connector::Options {
        root,
        port,
        client: context.client.clone(),
        project_id:context.project_id.clone(),
    })?;
    let reply = http
        .post(format!("{origin}{REQUESTS}/{}/claim", super::component(id)))
        .json(&json!({"claim_secret": proof}))
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|_| {
            "Fetch could not deliver the approved connection. Set it up again before connecting."
                .to_owned()
        })?;
    let status = reply.status();
    let response: Value = reply
        .json()
        .await
        .map_err(|_| "Fetch returned unreadable connection credentials.".to_owned())?;
    if !status.is_success() {
        return Err(response["error"]["message"]
            .as_str()
            .unwrap_or("Fetch could not claim the approved connection.")
            .to_owned());
    }
    if response["status"] != "approved" {
        return Err("Fetch did not deliver approved connection credentials.".into());
    }
    Ok(response)
}


async fn reconcile_pending(root: std::path::PathBuf, origin: &str, profiles: &BTreeSet<ConnectionContext>, client: &NativeClient) -> Result<(), String> {
    for profile in profiles {
        let store = crate::connector::profile::ProfileStore::for_context(root.clone(), profile.clone())?;
        if let Some(pending) = store.pending(origin).await? {
            // Read backend state after this exact proof. A pre-creation list
            // snapshot must never erase a request persisted by another startup.
            let value = client.request(Method::GET, REQUESTS, None).await?;
            let live = value["requests"].as_array().ok_or("Fetch returned an unreadable connection list.")?;
            if !live.iter().any(|request| request["request_id"].as_str() == Some(pending.request_id.as_str())) {
                store.clear_pending(origin, &pending.request_id).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn project_popup_shows_its_folder_and_history_requires_an_explicit_click(){
        let (_directory,mut desktop)=crate::ui::auth_tests::desktop_fixture().await;
        desktop.page=super::super::Page::Connect;
        desktop.connector.reconciled=true;
        let id="project-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let project=json!({"id":id,"name":"Project A","folder_path":"/fixture/project-a"});
        let request=json!({"request_id":"project-request","client_name":"codex","project_id":id,"project":project,"resource":format!("{}/mcp/projects/{id}",desktop.host.state.origin),"code":"219843","expires_at":u64::MAX,"status":"pending","scopes":["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:sessions:read"]});
        drop(desktop.update_connector(Event::Requests(Ok(json!({"requests":[request]})))));
        assert!(desktop.connector.popup_visible());
        assert!(!desktop.connector.selected.contains("fetch:sessions:read"));
        assert_eq!(desktop.connector.popup.as_ref().unwrap()["project"]["folder_path"],"/fixture/project-a");
        let mut view=crate::ui::neumorphic_tests::ViewFixture::new(&desktop,iced::Size::new(874.0,750.0)).await;
        view.export(&desktop,"project-history-approval");
        let messages=view.click(&desktop,"Read this project’s session histories");
        assert!(messages.iter().any(|message|matches!(message,Message::Connector(Event::Toggle(scope,true)) if scope=="fetch:sessions:read")));
        for message in messages{drop(desktop.update(message));}
        assert!(desktop.connector.selected.contains("fetch:sessions:read"));
        assert_eq!(desktop.connector.request_contexts["project-request"].project_id.as_deref(),Some(id));
        let mut view=crate::ui::neumorphic_tests::ViewFixture::new(&desktop,iced::Size::new(760.0,520.0)).await;
        view.export(&desktop,"project-history-approval-minimum");
        for message in view.click(&desktop,"Allow"){drop(desktop.update(message));}
        assert!(desktop.connector.deciding);
    }

    #[tokio::test]
    async fn automatic_requests_show_immediately_and_denial_schedules_exact_pending_cleanup() {
        let (_directory, mut desktop) = crate::ui::auth_tests::desktop_fixture().await;
        let request=json!({"request_id":"automatic","client_name":"codex","code":"219843","expires_at":u64::MAX,"status":"pending","scopes":DEFAULT_SCOPES});
        let task=desktop.update_connector(Event::Requests(Ok(json!({"requests":[request.clone()]}))));
        assert!(desktop.connector.popup_visible(),"reconciliation must not delay the first popup");
        assert!(desktop.connector.owned.is_empty(),"automatic proof belongs to its durable profile");
        drop(task); // Production Keychain tasks are never executed by this fixture.
        let task=desktop.update_connector(Event::Decided("automatic".into(),false,Ok(json!({"status":"denied"}))));
        assert_eq!(task.units(),1);
        assert!(desktop.connector.claiming.contains("automatic"));
        drop(task);
        let _=desktop.update_connector(Event::PendingCleared("automatic".into(),Ok(())));
        assert!(desktop.connector.ended.contains("automatic"));
        assert!(!desktop.connector.popup_visible());
        // An OS confirmation denial is returned as an error. The safe-list read
        // must still schedule cleanup for that exact denied profile/request.
        let mut denied=request;
        denied["request_id"]=json!("mac-denied");
        denied["status"]=json!("denied");
        let task=desktop.update_connector(Event::Requests(Ok(json!({"requests":[denied]}))));
        assert_eq!(task.units(),1);
        assert!(desktop.connector.claiming.contains("mac-denied"));
        drop(task);
        desktop.connector.request_contexts.insert("closed-denied".into(),ConnectionContext::global("codex".into()));
        let window=desktop.window.unwrap();
        let _=desktop.update(Message::Closed(window));
        assert!(desktop.client.is_none());
        let task=desktop.update_connector(Event::Decided("closed-denied".into(),true,Err("The Mac confirmation was denied.".into())));
        assert_eq!(task.units(),1,"terminal denial must be observed even after window/session close");
        assert!(desktop.connector.claiming.contains("closed-denied"));
        assert!(desktop.connector.error.as_deref().unwrap().contains("denied"));
        drop(task);
    }
    #[tokio::test]
    async fn closing_and_reopening_the_window_preserves_the_setup_claim_owner() {
        let (_directory, mut desktop) = crate::ui::auth_tests::desktop_fixture().await;
        desktop.connector.reconciled = true;
        let request = json!({"request_id":"pending","client_name":"claude-code","code":"219843","claim_secret":"disposable-proof","expires_at":u64::MAX,"status":"pending","scopes":DEFAULT_SCOPES});
        let _ = desktop.update_connector(Event::Created("claude-code".into(), Ok(request.clone())));
        let window = desktop.window.unwrap();
        let _ = desktop.update(Message::Closed(window));
        assert!(!desktop.connector.popup_visible());
        assert_eq!(desktop.connector.owned["pending"].proof, "disposable-proof");
        // Reopen the native API session without executing a real window task.
        let session = std::sync::Arc::new(desktop.host.native.open_settings());
        desktop.client = Some(
            NativeClient::new(
                desktop.host.state.origin.clone(),
                &desktop.host.certificate_pem,
                session.clone(),
            )
            .unwrap(),
        );
        desktop.session = Some(session);
        desktop.window = Some(iced::window::Id::unique());
        let _ = desktop.update_connector(Event::Requests(Ok(json!({"requests":[request]}))));
        assert!(desktop.connector.popup_visible());
        assert_eq!(desktop.connector.owned["pending"].client, "claude-code");
        assert_eq!(desktop.connector.owned["pending"].proof, "disposable-proof");
    }

    #[tokio::test]
    async fn approved_reply_after_window_close_still_schedules_proof_claim_and_save() {
        let (_directory, mut desktop) = crate::ui::auth_tests::desktop_fixture().await;
        desktop.connector.reconciled = true;
        let _=desktop.update_connector(Event::Created("claude-code".into(),Ok(json!({"request_id":"pending","client_name":"claude-code","code":"219843","claim_secret":"disposable-proof","expires_at":u64::MAX,"status":"pending","scopes":DEFAULT_SCOPES}))));
        let window = desktop.window.unwrap();
        let _ = desktop.update(Message::Closed(window));
        assert!(desktop.client.is_none());
        let task = desktop.update_connector(Event::Decided(
            "pending".into(),
            true,
            Ok(json!({"status":"approved"})),
        ));
        assert_eq!(
            task.units(),
            1,
            "proof-only claim/save must survive loss of the window session"
        );
        assert!(desktop.connector.status.starts_with("Saving"));
        // Do not execute the task: this fixture must not write real Keychain.
        drop(task);
    }

    #[tokio::test]
    async fn approval_expiring_while_window_is_closed_releases_setup_for_retry() {
        let (_directory, mut desktop) = crate::ui::auth_tests::desktop_fixture().await;
        desktop.connector.reconciled = true;
        let _=desktop.update_connector(Event::Created("claude-code".into(),Ok(json!({"request_id":"pending","client_name":"claude-code","code":"219843","claim_secret":"disposable-proof","expires_at":u64::MAX,"status":"pending","scopes":DEFAULT_SCOPES}))));
        desktop.connector.window_closed();
        desktop
            .connector
            .owned
            .get_mut("pending")
            .unwrap()
            .expires_at = 0;
        let _ = desktop.update_connector(Event::Requests(Ok(json!({"requests":[]}))));
        assert!(desktop.connector.owned.is_empty());
        assert!(!desktop.connector.popup_visible());
    }

    #[tokio::test]
    async fn denied_confirmation_releases_ui_setup_for_a_fresh_request() {
        let (_directory, mut desktop) = crate::ui::auth_tests::desktop_fixture().await;
        desktop.connector.reconciled = true;
        let _=desktop.update_connector(Event::Created("claude-code".into(),Ok(json!({"request_id":"pending","client_name":"claude-code","code":"219843","claim_secret":"disposable-proof","expires_at":u64::MAX,"status":"pending","scopes":DEFAULT_SCOPES}))));
        assert!(desktop.connector.popup_visible());
        let _ = desktop.update_connector(Event::Decided(
            "pending".into(),
            true,
            Err("The Mac confirmation was denied.".into()),
        ));
        let _ = desktop.update_connector(Event::Requests(Ok(
            json!({"requests":[{"request_id":"pending","status":"denied"}]}),
        )));
        assert!(!desktop.connector.popup_visible());
        assert!(desktop.connector.owned.is_empty());
        assert!(!desktop.connector.deciding);
    }
}
