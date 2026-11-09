//! Real widget interactions and offscreen captures. No window or API task runs.
use super::*;
use iced::advanced::{clipboard, layout, renderer::{Headless, Renderer as _}, widget::{Tree, Operation}, Layout, Shell};
use iced::{Color, Font, Pixels, Point, Rectangle, Size, Vector};

#[derive(Default)]
struct Survey {
    text: Vec<(String, Rectangle)>,
    scrolls: Vec<(String, Rectangle, Rectangle, Vector)>,
    containers: Vec<Rectangle>,
    offset: Vector,
    child_offset: Option<Vector>,
}
impl Operation for Survey {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        let previous = self.offset;
        if let Some(offset) = self.child_offset.take() { self.offset += offset; }
        operate(self);
        self.offset = previous;
    }
    fn text(&mut self, _: Option<&iced::advanced::widget::Id>, bounds: Rectangle, text: &str) {
        self.text.push((text.to_owned(), bounds - self.offset));
    }
    fn container(&mut self, _: Option<&iced::advanced::widget::Id>, bounds: Rectangle) { self.containers.push(bounds - self.offset); }
    fn scrollable(&mut self, id: Option<&iced::advanced::widget::Id>, bounds: Rectangle, content: Rectangle, translation: Vector, _: &mut dyn iced::advanced::widget::operation::Scrollable) {
        self.scrolls.push((format!("{id:?}"), bounds - self.offset, content - self.offset, translation));
        self.child_offset = Some(translation);
    }
}
pub(super) struct ViewFixture {
    renderer: iced::Renderer,
    tree: Tree,
    node: layout::Node,
    screen: Rectangle,
}
impl ViewFixture {
    pub(super) async fn new(desktop: &Desktop, size: Size) -> Self {
        {
            let mut fonts = iced::advanced::graphics::text::font_system().write().unwrap();
            fonts.load_font(std::borrow::Cow::Borrowed(include_bytes!("assets/fonts/Inter.ttf")));
            fonts.load_font(std::borrow::Cow::Borrowed(include_bytes!("assets/fonts/Inter-Medium.ttf")));
            fonts.load_font(std::borrow::Cow::Borrowed(include_bytes!("assets/fonts/Inter-SemiBold.ttf")));
            fonts.load_font(std::borrow::Cow::Borrowed(include_bytes!("assets/fonts/Inter-Bold.ttf")));
            fonts.load_font(std::borrow::Cow::Borrowed(include_bytes!("assets/fonts/RobotoMono.ttf")));
        }
        let renderer = <iced::Renderer as Headless>::new(Font::with_name("Inter"), Pixels(14.0), None).await.expect("offscreen UI renderer");
        let mut element = desktop.view(desktop.window.unwrap());
        let mut tree = Tree::new(element.as_widget());
        let node = element.as_widget_mut().layout(&mut tree, &renderer, &layout::Limits::new(size, size));
        Self { renderer, tree, node, screen: Rectangle::new(Point::ORIGIN, size) }
    }
    fn relayout(&mut self, desktop: &Desktop) {
        let mut element = desktop.view(desktop.window.unwrap());
        self.tree.diff(element.as_widget());
        self.node = element.as_widget_mut().layout(&mut self.tree, &self.renderer, &layout::Limits::new(self.screen.size(), self.screen.size()));
    }
    fn survey(&mut self, desktop: &Desktop) -> Survey {
        let mut survey = Survey::default();
        let mut element = desktop.view(desktop.window.unwrap());
        element.as_widget_mut().operate(&mut self.tree, Layout::new(&self.node), &self.renderer, &mut survey);
        survey
    }
    fn event(&mut self, desktop: &Desktop, event: iced::Event, point: Point) -> Vec<Message> {
        let mut messages = vec![];
        let mut element = desktop.view(desktop.window.unwrap());
        let mut shell = Shell::new(&mut messages);
        element.as_widget_mut().update(&mut self.tree, &event, Layout::new(&self.node), iced::mouse::Cursor::Available(point), &self.renderer, &mut clipboard::Null, &mut shell, &self.screen);
        messages
    }
    fn click(&mut self, desktop: &Desktop, label: &str) -> Vec<Message> {
        let survey = self.survey(desktop);
        let bounds = survey.text.iter().find(|(text, _)| text == label).unwrap_or_else(|| panic!("missing visible control {label}: {:?}", survey.text.iter().map(|(text, _)| text).collect::<Vec<_>>())).1;
        let point = bounds.center();
        assert!(self.screen.contains(point), "control {label} must fit on screen: {bounds:?}");
        let mut messages = self.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), point);
        messages.extend(self.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), point));
        messages
    }
    pub(super) fn pixels(&mut self, desktop: &Desktop) -> Vec<u8> {
        self.renderer.reset(self.screen);
        let mut element = desktop.view(desktop.window.unwrap());
        let mut messages = vec![];
        {
            let mut shell = Shell::new(&mut messages);
            element.as_widget_mut().update(&mut self.tree, &iced::Event::Window(iced::window::Event::RedrawRequested(std::time::Instant::now())), Layout::new(&self.node), iced::mouse::Cursor::Unavailable, &self.renderer, &mut clipboard::Null, &mut shell, &self.screen);
        }
        assert!(messages.is_empty(), "redrawing the fixture must not begin application work");
        element.as_widget().draw(&self.tree, &mut self.renderer, &theme::light(), &iced::advanced::renderer::Style{text_color:theme::INK}, Layout::new(&self.node), iced::mouse::Cursor::Unavailable, &self.screen);
        <iced::Renderer as Headless>::screenshot(&mut self.renderer, Size::new(self.screen.width as u32, self.screen.height as u32), 1.0, theme::SURFACE)
    }
    pub(super) fn export(&mut self, desktop: &Desktop, name: &str) {
        if let Some(directory) = std::env::var_os("FETCH_UI_EVIDENCE_DIR") {
            let path = std::path::PathBuf::from(directory);
            std::fs::create_dir_all(&path).unwrap();
            let started=std::time::Instant::now();
            let pixels = self.pixels(desktop);
            eprintln!("UI capture {name} {}x{}: {:.3}s",self.screen.width as u32,self.screen.height as u32,started.elapsed().as_secs_f64());
            std::fs::write(path.join(format!("{name}-{}x{}.rgba", self.screen.width as u32, self.screen.height as u32)), pixels).unwrap();
        }
    }
}

pub(super) fn sample_servers() -> Value {
    json!({"servers":[
        {"server_id":"filesystem","name":"Filesystem","runtime":{"status":"ready"},"authentication":{"status":"not_required","access_expires_at":null,"last_verified_at":null},"config":{"kind":"local","execution":{"default_timeout_ms":120000,"max_concurrent_calls":1},"environment":[]},"tool_execution":{"active_calls":0,"queued_calls":0,"uncertain_calls":0}},
        {"server_id":"search","name":"Search","runtime":{"status":"stopped"},"authentication":{"status":"required","access_expires_at":null,"last_verified_at":null},"config":{"kind":"remote","execution":{"default_timeout_ms":45000,"max_concurrent_calls":3},"environment":[{"name":"API_KEY","credential_ref":"search-key"}]},"tool_execution":{"active_calls":0,"queued_calls":0,"uncertain_calls":0}}
    ]})
}

pub(super) fn tool_catalog() -> Value {
    let tools: Vec<_> = ["read_file", "list_directory", "file_information", "search_files", "read_multiple_files", "directory_tree"].into_iter()
        .map(|name| json!({"name":name,"description":format!("Fixture {name}"),"inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}})).collect();
    json!({"origins":[{"source":{"server_id":"filesystem"},"status":"available","tools":tools}],"next_steps":[]})
}
fn apply_click(fixture: &mut ViewFixture, desktop: &mut Desktop, label: &str) {
    let messages = fixture.click(desktop, label);
    assert!(!messages.is_empty(), "{label} must produce its UI action");
    for message in messages { drop(desktop.update(message)); }
    fixture.relayout(desktop);
}

#[tokio::test]
async fn neumorphic_server_panels_open_from_their_own_controls_and_keep_key_ownership() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    drop(desktop.update(Message::ToggleServer("filesystem".into())));
    let mut fixture = ViewFixture::new(&desktop, Size::new(1248.0, 900.0)).await;
    let texts = fixture.survey(&desktop).text;
    assert!(!texts.iter().any(|(text, _)| text == "Available tools"));
    assert!(desktop.server_panel("filesystem") == server_terminal::Pane::Hidden);
    let survey = fixture.survey(&desktop);
    let name_y = survey.text.iter().find(|(text, _)| text == "Filesystem").unwrap().1.center_y();
    let mut actions: Vec<_> = survey.containers.iter().filter(|bounds| (bounds.width - 44.0).abs() < 0.1 && (bounds.height - 44.0).abs() < 0.1 && (bounds.center_y() - name_y).abs() < 2.0).map(Rectangle::center).collect();
    actions.sort_by(|a, b| a.x.total_cmp(&b.x));
    actions.dedup_by(|a,b| a.x == b.x);
    assert_eq!(actions.len(), 3, "server header has the three round actions");
    for shift in [false, true] {
        drop(desktop.update(Message::Modifiers(if shift { iced::keyboard::Modifiers::SHIFT } else { iced::keyboard::Modifiers::default() })));
        let mut messages = fixture.event(&desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), actions[1]);
        messages.extend(fixture.event(&desktop, iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), actions[1]));
        assert!(messages.iter().any(|message| matches!(message, Message::RestartClicked(id) if id == "filesystem")), "round restart handles the click with either modifier");
        assert_eq!(desktop.modifiers.shift(), shift);
    }
    drop(desktop.update(Message::Modifiers(Default::default())));
    apply_click(&mut fixture, &mut desktop, "Inspect tools");
    assert!(desktop.server_panel("filesystem") == server_terminal::Pane::Tools);
    drop(desktop.update(Message::Tools("filesystem".into(), desktop.window, Ok(tool_catalog()))));
    fixture.relayout(&desktop);
    assert!(fixture.survey(&desktop).text.iter().any(|(text, _)| text == "read_file"));
    assert!(desktop.server_panel("filesystem") != server_terminal::Pane::Console);
    assert_eq!(desktop.tools["origins"][0]["tools"][0], tool_catalog()["origins"][0]["tools"][0], "the visual card retains the complete original tool definition in its catalog");
    apply_click(&mut fixture, &mut desktop, "Settings");
    assert!(desktop.page == Page::Servers);
    assert_eq!(desktop.selected_server, "filesystem");
    assert_eq!((&desktop.timeout, &desktop.concurrency), (&"120".to_owned(), &"1".to_owned()));
    assert!(!fixture.survey(&desktop).text.iter().any(|(text, _)| text.contains("search-key")));
    apply_click(&mut fixture, &mut desktop, "Settings");
    assert!(desktop.server_panel("filesystem") == server_terminal::Pane::Hidden);
    drop(desktop.update(Message::Choose("search".into())));
    fixture.relayout(&desktop);
    assert!(desktop.page == Page::Servers);
    assert_eq!(desktop.selected_server, "search");
    assert_eq!((&desktop.timeout, &desktop.concurrency), (&"45".to_owned(), &"3".to_owned()));
    assert!(fixture.survey(&desktop).text.iter().any(|(text, _)| text == "API_KEY → search-key"));
    fixture.event(&desktop, iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta: iced::mouse::ScrollDelta::Pixels { x:0.0, y:-400.0 } }), Point::new(76.0, 820.0));
    apply_click(&mut fixture, &mut desktop, "API_KEY → search-key");
    assert_eq!(desktop.secret_name, "search-key");
    drop(desktop.update(Message::SecretValue("disposable-fixture-secret".into())));
    drop(desktop.update(Message::Choose("filesystem".into())));
    assert!(desktop.secret_value.is_empty(), "switching servers clears the previous key value");
}

#[tokio::test]
async fn neumorphic_tools_have_three_columns_and_scroll_without_moving_the_page() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    drop(desktop.update(Message::Inspect("filesystem".into())));
    drop(desktop.update(Message::Tools("filesystem".into(), desktop.window, Ok(tool_catalog()))));
    let mut fixture = ViewFixture::new(&desktop, Size::new(1248.0, 900.0)).await;
    let survey = fixture.survey(&desktop);
    let names = ["read_file", "list_directory", "file_information", "search_files", "read_multiple_files", "directory_tree"];
    let bounds: Vec<_> = names.iter().map(|name| survey.text.iter().find(|(text, _)| text == name).expect("tool name is rendered").1).collect();
    assert!((bounds[0].y - bounds[1].y).abs() < 2.0 && (bounds[1].y - bounds[2].y).abs() < 2.0, "first three tools share one row");
    assert!(bounds[0].x < bounds[1].x && bounds[1].x < bounds[2].x, "three columns run left to right");
    assert!(bounds[3].y > bounds[0].y + 200.0, "fourth tool starts the next row");
    let inner = survey.scrolls.iter().find(|(_, bounds, content, _)| content.height > bounds.height + 200.0 && bounds.height < 500.0).expect("tools have a bounded scrolling viewport").clone();
    let before = fixture.pixels(&desktop);
    let messages = fixture.event(&desktop, iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta: iced::mouse::ScrollDelta::Pixels { x:0.0, y:-350.0 } }), inner.1.center());
    assert!(messages.is_empty(), "scrolling tools does not invoke an API or tool");
    let after = fixture.survey(&desktop);
    assert!(after.scrolls.iter().any(|(id, _, _, translation)| *id == inner.0 && translation.y > 100.0), "wheel movement reaches the later tool row: before={:?}, after={:?}", survey.scrolls, after.scrolls);
    for (id, _, _, translation) in &after.scrolls {
        if *id != inner.0 { assert_eq!(translation.y, 0.0, "the outer page must stay still"); }
    }
    assert_ne!(fixture.pixels(&desktop), before, "scrolling visibly changes the tool contents");
    fixture.export(&desktop, "tools-scrolled");
}

#[tokio::test]
async fn neumorphic_views_render_real_screens_at_design_and_minimum_window_sizes() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    for size in [Size::new(1248.0, 900.0), Size::new(760.0, 520.0)] {
        desktop.expanded_servers.clear();
        desktop.terminals.clear();
        desktop.page = Page::Servers;
        let mut fixture = ViewFixture::new(&desktop, size).await;
        fixture.export(&desktop, "servers");
        drop(desktop.update(Message::Inspect("filesystem".into())));
        drop(desktop.update(Message::Tools("filesystem".into(), desktop.window, Ok(tool_catalog()))));
        fixture.relayout(&desktop);
        fixture.export(&desktop, "tools");
        if size.width < 800.0 {
            fixture.event(&desktop, iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta: iced::mouse::ScrollDelta::Pixels { x:0.0, y:-350.0 } }), Point::new(76.0, 450.0));
            fixture.export(&desktop, "tools-minimum-scrolled");
            fixture = ViewFixture::new(&desktop, size).await;
        }
        drop(desktop.update(Message::Choose("filesystem".into())));
        fixture.relayout(&desktop);
        fixture.export(&desktop, "settings");
        let mut flow = authentication::Flow::passive(&Value::Null);
        flow.log = vec!["[server] Ready for requests".into(), "[server] Loaded the server’s own configuration".into()];
        desktop.authentication.insert("filesystem".into(), flow);
        drop(desktop.update(Message::Terminal(server_terminal::Message::Console("filesystem".into()))));
        fixture.relayout(&desktop);
        assert!(fixture.survey(&desktop).text.iter().any(|(text, _)| text == "[server] Ready for requests"));
        fixture.export(&desktop, "console");
        drop(desktop.update(Message::Page(Page::Install)));
        fixture.relayout(&desktop);
        fixture.export(&desktop, "install");
        desktop.discovery_started = true;
        desktop.discovery = json!({"configs":[{"path":"/disposable/config.toml","servers":[{"name":"Filesystem","type":"local","status":"supported","installed":false},{"name":"Search","type":"remote","status":"supported","installed":true}]}],"next_steps":[]});
        desktop.selected.insert(("/disposable/config.toml".into(), "Filesystem".into()));
        fixture.relayout(&desktop);
        fixture.export(&desktop, "install-popup");
        let pixels=fixture.pixels(&desktop);
        let survey=fixture.survey(&desktop);
        for (label,face) in [("Filesystem · local",[149,212,241]),("Search · remote · already installed",[227,230,233])] {
            let bounds=survey.text.iter().find(|(value,_)|value==label).expect("installation row is present").1;
            let left=bounds.x.round() as usize;
            let top=(bounds.center_y()-9.0).round() as usize;
            let visible=(left+2..left+16).any(|x|(top+2..top+16).any(|y|{
                let index=(y*size.width as usize+x)*4;
                pixels[index..index+3]==face
            }));
            assert!(visible,"{label} checkbox retains its filled face above the popup background, not only its checkmark");
        }
        desktop.job = Some("disposable-installation".into());
        drop(desktop.update(Message::CloseDiscovery));
        fixture.relayout(&desktop);
        apply_click(&mut fixture, &mut desktop, "View installation progress");
        assert!(desktop.discovery_started && desktop.job.as_deref() == Some("disposable-installation"), "closing the popup preserves an active installation and lets the user reopen its progress");
        desktop.job = None;
        desktop.discovery_started = false;
        drop(desktop.update(Message::Page(Page::Connect)));
        fixture.relayout(&desktop);
        fixture.export(&desktop, "connect");
        let survey = fixture.survey(&desktop);
        assert!(survey.text.iter().any(|(text, _)| text.contains("codex mcp add fetch --url") && text.contains("codex mcp login fetch")), "copyable Codex commands are retained");
        assert!(fixture.click(&desktop, "Copy Fetch MCP address").iter().any(|message| matches!(message, Message::Copy(value) if value == &format!("{}/mcp", desktop.host.state.origin))), "Copy passes the displayed address to the existing clipboard action");
        assert!(!survey.text.iter().any(|(text, _)| text == "Quit Fetch" || text == "Settings"), "navigation has only the three approved tabs");
    }
}
