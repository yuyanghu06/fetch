//! A display/IO wrapper around iced_term and the native-only terminal API.
//! The backend owns the shell, its environment, process lifetime and credentials.
use super::{Desktop, Message as AppMessage, components, theme};
use super::client::TerminalOpened;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use iced::{Element, Subscription, Task, widget::{column, row, text}};
use iced_term::{Command, Event, ExternalAction, Terminal, TerminalView};
use serde_json::{Value, json};
use std::{collections::VecDeque, sync::atomic::{AtomicU64, Ordering}, time::Duration};

static NEXT_WIDGET: AtomicU64 = AtomicU64::new(1);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum Pane { #[default] Hidden, Tools, Settings, Console, Terminal }

#[derive(Clone)]
pub(super) enum Message {
    Console(String), Open(String), Start(String), Close(String), Tick,
    Event(String, Event),
    Opened(String, u64, Result<TerminalOpened, String>),
    Output(String, String, u64, Result<Value, String>),
    Applied(String, String, u64, bool, Result<Value, String>),
}

#[derive(Default)]
pub(super) struct State {
    pane: Pane,
    epoch: u64,
    terminal: Option<Terminal>,
    session: Option<String>,
    status: String,
    cursor: u64,
    replay_until: u64,
    opening: bool,
    closing: bool,
    reading: bool,
    read_stopped: bool,
    drained: bool,
    applying: bool,
    pending: VecDeque<Value>,
    columns: u16,
    rows: u16,
    error: Option<String>,
    gap: bool,
    exit_code: Option<i64>,
    signal: Option<Value>,
}

impl State {
    pub(super) fn hide_settings(&mut self){if self.pane==Pane::Settings{self.pane=Pane::Hidden;}}
    fn emulator(&self) -> Terminal {
        Terminal::new_external(NEXT_WIDGET.fetch_add(1, Ordering::Relaxed), Default::default(), self.columns.max(1), self.rows.max(1))
    }
    fn collect_actions(&mut self, allow_write: bool) {
        let Some(terminal) = self.terminal.as_mut() else { return; };
        let Some(session) = self.session.as_ref() else { return; };
        for action in terminal.take_external_actions() {
            if self.status != "running" || self.closing { continue; }
            match action {
                ExternalAction::Write(bytes) => {
                    if !allow_write { continue; }
                    for bytes in bytes.chunks(64 * 1024) {
                        self.pending.push_back(json!({"action":"input","session_id":session,"data":STANDARD.encode(bytes)}));
                    }
                }
                ExternalAction::Resize { columns, rows } => {
                    self.columns = columns.clamp(1, 1000);
                    self.rows = rows.clamp(1, 1000);
                    self.pending.push_back(json!({"action":"resize","session_id":session,"columns":self.columns,"rows":self.rows}));
                }
            }
        }
    }
    fn observe_status(&mut self, value: &Value) {
        if let Some(status) = value["status"].as_str() { self.status = status.to_owned(); }
        self.exit_code = value["exit_code"].as_i64();
        self.signal = value.get("signal").filter(|value| !value.is_null()).cloned();
    }
}

impl Desktop {
    pub(super) fn server_panel(&self,id:&str)->Pane {
        self.terminals.get(id).map(|state|state.pane).unwrap_or_default()
    }
    pub(super) fn select_server_panel(&mut self,id:&str,pane:Pane)->bool {
        let state=self.terminals.entry(id.to_owned()).or_default();
        state.pane=if state.pane==pane{Pane::Hidden}else{pane};
        state.pane!=Pane::Hidden
    }

    pub(super) fn pause_terminals(&mut self) {
        for state in self.terminals.values_mut() {
            // Responses belonging to the invalidated window must not alter a
            // newly authorized view. Submitted input is never retried.
            state.epoch = state.epoch.wrapping_add(1);
            state.opening = false; state.closing = false; state.reading = false; state.applying = false;
            state.pending.clear();
        }
    }

    pub(super) fn resume_terminals(&mut self) {
        for state in self.terminals.values_mut() { state.read_stopped = false; }
    }

    pub(super) fn terminal_subscription(&self) -> Subscription<AppMessage> {
        if self.window.is_none() || !self.terminals.values().any(|state| state.session.is_some() && !state.drained && !state.read_stopped) {
            return Subscription::none();
        }
        // Only reads output. No shell or input is started by this timer.
        iced::time::every(Duration::from_millis(50)).map(|_| AppMessage::Terminal(Message::Tick))
    }

    fn apply_terminal_input(&mut self, id: &str) -> Task<AppMessage> {
        let Some(client) = self.client.clone() else { return Task::none(); };
        let Some(state) = self.terminals.get_mut(id) else { return Task::none(); };
        if state.applying { return Task::none(); }
        let Some(session) = state.session.clone() else { return Task::none(); };
        let Some(body) = state.pending.pop_front() else { return Task::none(); };
        let closing = body["action"] == "close";
        let epoch = state.epoch;
        state.applying = true;
        let id = id.to_owned();
        let result_id = id.clone();
        Task::perform(async move { client.terminal_action(&id, body).await }, move |result| AppMessage::Terminal(Message::Applied(result_id.clone(), session.clone(), epoch, closing, result)))
    }

    fn open_terminal(&mut self, id: String, start_new: bool) -> Task<AppMessage> {
        if self.deletions.contains_key(&id) || !self.servers["servers"].as_array().into_iter().flatten().any(|server| server["server_id"].as_str() == Some(id.as_str()) && server["config"]["kind"] != "remote") {
            return Task::none();
        }
        let Some(client) = self.client.clone() else { return Task::none(); };
        let state = self.terminals.entry(id.clone()).or_default();
        state.pane = Pane::Terminal;
        if state.opening || state.closing || (start_new && state.status == "running") { return Task::none(); }
        if state.terminal.is_some() && !start_new {
            state.read_stopped = false;
            state.error = None;
            return state.terminal.as_ref().map(|terminal| TerminalView::focus(terminal.widget_id().clone())).unwrap_or_else(Task::none);
        }
        let epoch = state.epoch;
        state.opening = true;
        state.error = None;
        let result_id = id.clone();
        Task::perform(async move { client.terminal_open(&id).await }, move |result| AppMessage::Terminal(Message::Opened(result_id.clone(), epoch, result)))
    }

    pub(super) fn update_terminal(&mut self, message: Message) -> Task<AppMessage> {
        match message {
            Message::Console(id) => {
                self.clear_secrets();self.expanded_servers.insert(id.clone());
                self.select_server_panel(&id,Pane::Console); Task::none()
            }
            Message::Open(id) => {
                if self.deletions.contains_key(&id)||!self.servers["servers"].as_array().into_iter().flatten().any(|server|server["server_id"].as_str()==Some(id.as_str())&&server["config"]["kind"]!="remote"){return Task::none();}
                self.clear_secrets();self.expanded_servers.insert(id.clone());
                if !self.select_server_panel(&id,Pane::Terminal){return Task::none();}
                self.open_terminal(id, false)
            },
            Message::Start(id) => self.open_terminal(id, true),
            Message::Opened(id, epoch, result) => {
                let Some(state) = self.terminals.get_mut(&id) else { return Task::none(); };
                if state.epoch != epoch { return Task::none(); }
                state.opening = false;
                match result {
                    Ok(TerminalOpened { created, value }) => {
                        let Some(session) = value["session_id"].as_str() else { state.error = Some("Fetch terminal: the open response has no session ID.".into()); return Task::none(); };
                        state.session = Some(session.to_owned());
                        state.columns = 80; state.rows = 24;
                        state.terminal = Some(state.emulator());
                        // Only a reused session contains already handled output.
                        // A new shell may issue a query before POSTopen returns.
                        state.cursor = 0; state.replay_until = if created { 0 } else { value["output_cursor"].as_u64().unwrap_or(0) }; state.reading = false; state.read_stopped = false; state.drained = false;
                        state.pending.clear(); state.applying = false; state.gap = false;
                        state.observe_status(&value);
                        let focus = state.terminal.as_ref().map(|terminal| TerminalView::focus(terminal.widget_id().clone())).unwrap_or_else(Task::none);
                        return Task::batch([focus, self.update_terminal(Message::Tick)]);
                    }
                    Err(error) => state.error = Some(error),
                }
                Task::none()
            }
            Message::Event(id, Event::BackendCall(widget, command)) => {
                let Some(state) = self.terminals.get_mut(&id) else { return Task::none(); };
                let Some(terminal) = state.terminal.as_mut() else { return Task::none(); };
                if terminal.id != widget { return Task::none(); }
                // After exit the screen remains selectable, but no new bytes go to a shell.
                terminal.handle(Command::ProxyToBackend(command));
                state.collect_actions(true);
                self.apply_terminal_input(&id)
            }
            Message::Close(id) => {
                let Some(client) = self.client.clone() else { return Task::none(); };
                let Some(state) = self.terminals.get_mut(&id) else { return Task::none(); };
                if state.closing { return Task::none(); }
                let Some(session) = state.session.clone() else { return Task::none(); };
                // Closing must remain available even while a shell has stopped
                // reading a pending paste. It is independent of that writer.
                state.epoch = state.epoch.wrapping_add(1);
                let epoch = state.epoch;
                state.pending.clear(); state.applying = false; state.reading = false; state.closing = true;
                let result_id = id.clone(); let result_session = session.clone();
                Task::perform(async move { client.terminal_action(&id, json!({"action":"close","session_id":session})).await }, move |result| AppMessage::Terminal(Message::Applied(result_id.clone(), result_session.clone(), epoch, true, result)))
            }
            Message::Tick => {
                let Some(client) = self.client.clone() else { return Task::none(); };
                let mut tasks = Vec::new();
                for (id, state) in &mut self.terminals {
                    if state.reading || state.read_stopped || state.drained || state.closing { continue; }
                    let Some(session) = state.session.clone() else { continue; };
                    state.reading = true;
                    let id = id.clone(); let client = client.clone(); let after = state.cursor; let epoch = state.epoch;
                    let result_id = id.clone(); let result_session = session.clone();
                    tasks.push(Task::perform(async move { client.terminal_output(&id, &session, after).await }, move |result| AppMessage::Terminal(Message::Output(result_id.clone(), result_session.clone(), epoch, result))));
                }
                Task::batch(tasks)
            }
            Message::Output(id, session, epoch, result) => {
                let Some(state) = self.terminals.get_mut(&id) else { return Task::none(); };
                if state.epoch != epoch || state.session.as_deref() != Some(session.as_str()) { return Task::none(); }
                state.reading = false;
                match result {
                    Ok(value) => {
                        let bytes = value["data"].as_str().ok_or("Fetch terminal: output response has no data.")
                            .and_then(|data| STANDARD.decode(data).map_err(|_| "Fetch terminal: output response contains invalid base64."));
                        let bytes = match bytes { Ok(bytes) => bytes, Err(error) => { state.error = Some(error.into()); state.read_stopped = true; return Task::none(); } };
                        if value["gap"] == true {
                            state.gap = true;
                            state.terminal = Some(state.emulator());
                        }
                        // A reused session replays past output to reconstruct its
                        // screen. Do not answer old terminal queries a second time.
                        let start = if value["gap"] == true { value["first_available_cursor"].as_u64().unwrap_or(state.cursor) } else { state.cursor };
                        let historical = state.replay_until.saturating_sub(start).min(bytes.len() as u64) as usize;
                        if historical > 0 {
                            if let Some(terminal) = &mut state.terminal { terminal.receive_output(&bytes[..historical]); }
                            state.collect_actions(false);
                        }
                        if let Some(terminal) = &mut state.terminal { terminal.receive_output(&bytes[historical..]); }
                        if let Some(cursor) = value["next_cursor"].as_u64() { state.cursor = cursor; }
                        state.observe_status(&value);
                        state.drained = state.status != "running" && bytes.is_empty();
                        state.collect_actions(true);
                        self.apply_terminal_input(&id)
                    }
                    Err(error) => { state.error = Some(error); state.read_stopped = true; Task::none() }
                }
            }
            Message::Applied(id, session, epoch, closing, result) => {
                let Some(state) = self.terminals.get_mut(&id) else { return Task::none(); };
                if state.epoch != epoch || state.session.as_deref() != Some(session.as_str()) { return Task::none(); }
                state.applying = false;
                if closing { state.closing = false; }
                match result {
                    Ok(value) => {
                        state.observe_status(&value);
                        if closing { state.status = "closed".into(); }
                        self.apply_terminal_input(&id)
                    }
                    Err(error) => {
                        // A write might already have happened. Do not replay it or
                        // automatically send the rest of a queued paste.
                        state.pending.clear(); state.error = Some(error); Task::none()
                    }
                }
            }
        }
    }

    pub(super) fn server_output_view<'a>(&'a self, id: &str, _server: &Value) -> Element<'a, AppMessage> {
        let state = self.terminals.get(id);
        let terminal_selected = state.is_some_and(|state| state.pane == Pane::Terminal);
        let mut pane = column![].spacing(theme::CONTROL_GAP).width(iced::Fill);
        if !terminal_selected { return pane.push(self.authentication_view(id, self.authentication.get(id))).into(); }
        if let Some(state) = state {
            if let Some(error) = &state.error { pane = pane.push(components::message(error.as_str())); }
            if state.gap { pane = pane.push(text("Some earlier terminal output is no longer available; this screen may be incomplete.").size(theme::SMALL_TEXT)); }
            if let Some(terminal) = &state.terminal {
                let id = id.to_owned();
                pane = pane.push(components::terminal_pane(TerminalView::show(terminal).map(move |event| AppMessage::Terminal(Message::Event(id.clone(), event)))));
            } else { pane = pane.push(components::terminal_pane(column![])); }
            if state.status == "running" {
                pane = pane.push(row![components::button("Close terminal").on_press_maybe((!state.closing).then(|| AppMessage::Terminal(Message::Close(id.to_owned()))))]);
            } else if !state.status.is_empty() {
                let detail = state.exit_code.map(|code| format!(" (exit code {code})"))
                    .or_else(|| state.signal.as_ref().map(|signal| format!(" (signal {signal})"))).unwrap_or_default();
                pane = pane.push(row![
                    text(format!("Shell {}{detail}", state.status)),
                    components::button("Start terminal").on_press_maybe((!state.opening && !state.closing).then(|| AppMessage::Terminal(Message::Start(id.to_owned())))),
                ].spacing(theme::CONTROL_GAP));
            }
        }
        pane.into()
    }
}

#[cfg(test)]
#[path = "server_terminal_tests.rs"]
mod tests;
