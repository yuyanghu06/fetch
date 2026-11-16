//! Real widget interactions and offscreen captures. No window or API task runs.
use super::*;
use iced::advanced::{clipboard, layout, renderer::{Headless, Renderer as _}, widget::{Tree, Operation}, Layout, Shell};
use iced::{Color, Font, Pixels, Point, Rectangle, Size, Vector};

#[derive(Default)]
pub(super) struct Survey {
    pub(super) text: Vec<(String, Rectangle)>,
    scrolls: Vec<(String, Rectangle, Rectangle, Vector)>,
    pub(super) containers: Vec<Rectangle>,
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
        super::scroll_gesture::new_test_gesture();
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
    pub(super) fn relayout(&mut self, desktop: &Desktop) {
        let mut element = desktop.view(desktop.window.unwrap());
        self.tree.diff(element.as_widget());
        self.node = element.as_widget_mut().layout(&mut self.tree, &self.renderer, &layout::Limits::new(self.screen.size(), self.screen.size()));
    }
    pub(super) fn survey(&mut self, desktop: &Desktop) -> Survey {
        let mut survey = Survey::default();
        let mut element = desktop.view(desktop.window.unwrap());
        // Fresh views must run layout to initialize size-dependent children.
        self.node = element.as_widget_mut().layout(&mut self.tree, &self.renderer, &layout::Limits::new(self.screen.size(), self.screen.size()));
        element.as_widget_mut().operate(&mut self.tree, Layout::new(&self.node), &self.renderer, &mut survey);
        survey
    }
    pub(super) fn event(&mut self, desktop: &Desktop, event: iced::Event, point: Point) -> Vec<Message> {
        self.event_with_status(desktop,event,point).0
    }
    pub(super) fn event_with_status(&mut self, desktop: &Desktop, event: iced::Event, point: Point) -> (Vec<Message>,iced::event::Status) {
        let mut messages = vec![];
        let mut element = desktop.view(desktop.window.unwrap());
        self.node = element.as_widget_mut().layout(&mut self.tree, &self.renderer, &layout::Limits::new(self.screen.size(), self.screen.size()));
        let mut shell = Shell::new(&mut messages);
        element.as_widget_mut().update(&mut self.tree, &event, Layout::new(&self.node), iced::mouse::Cursor::Available(point), &self.renderer, &mut clipboard::Null, &mut shell, &self.screen);
        let status=shell.event_status();
        (messages,status)
    }
    pub(super) fn click(&mut self, desktop: &Desktop, label: &str) -> Vec<Message> {
        let survey = self.survey(desktop);
        let bounds = survey.text.iter().find(|(text, _)| text == label).unwrap_or_else(|| panic!("missing visible control {label}: {:?}", survey.text.iter().map(|(text, _)| text).collect::<Vec<_>>())).1;
        let point = bounds.center();
        assert!(self.screen.contains(point), "control {label} must fit on screen: {bounds:?}");
        let mut messages = self.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), point);
        messages.extend(self.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), point));
        messages
    }
    pub(super) fn pixels(&mut self, desktop: &Desktop) -> Vec<u8> {
        self.pixels_at(desktop,iced::mouse::Cursor::Unavailable,std::time::Instant::now())
    }
    fn pixels_at(&mut self,desktop:&Desktop,cursor:iced::mouse::Cursor,now:std::time::Instant)->Vec<u8>{
        self.renderer.reset(self.screen);
        let mut element = desktop.view(desktop.window.unwrap());
        self.node = element.as_widget_mut().layout(&mut self.tree, &self.renderer, &layout::Limits::new(self.screen.size(), self.screen.size()));
        let mut messages = vec![];
        {
            let mut shell = Shell::new(&mut messages);
            element.as_widget_mut().update(&mut self.tree, &iced::Event::Window(iced::window::Event::RedrawRequested(now)), Layout::new(&self.node), cursor, &self.renderer, &mut clipboard::Null, &mut shell, &self.screen);
        }
        assert!(messages.is_empty(), "redrawing the fixture must not begin application work");
        element.as_widget().draw(&self.tree, &mut self.renderer, &theme::light(), &iced::advanced::renderer::Style{text_color:theme::INK}, Layout::new(&self.node), cursor, &self.screen);
        if let Some(mut overlay)=element.as_widget_mut().overlay(&mut self.tree,Layout::new(&self.node),&self.renderer,&self.screen,Vector::ZERO){
            let node=overlay.as_overlay_mut().layout(&self.renderer,self.screen.size());
            self.renderer.with_layer(self.screen,|renderer|overlay.as_overlay().draw(renderer,&theme::light(),&iced::advanced::renderer::Style{text_color:theme::INK},Layout::new(&node),cursor));
        }
        <iced::Renderer as Headless>::screenshot(&mut self.renderer, Size::new(self.screen.width as u32, self.screen.height as u32), 1.0, Color::TRANSPARENT)
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

pub(super) fn escape_key()->iced::Event{
    iced::Event::Keyboard(iced::keyboard::Event::KeyPressed{
        key:iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
        modified_key:iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
        physical_key:iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::Escape),
        location:iced::keyboard::Location::Standard,modifiers:iced::keyboard::Modifiers::default(),text:None,repeat:false,
    })
}

pub(super) fn press_escape(fixture:&mut ViewFixture,desktop:&mut Desktop,point:Point)->iced::event::Status{
    let key=escape_key();
    let (mut messages,status)=fixture.event_with_status(desktop,key.clone(),point);
    // The headless fixture has no subscription runtime: use the exact mapper
    // registered by Fetch, with the status returned by the actual widgets.
    messages.extend(subscription_event(key,status,desktop.window.unwrap()));
    for message in messages{let task=desktop.update(message);assert_eq!(task.units(),0,"Escape only dismisses the current popup");drop(task);}
    fixture.relayout(desktop);
    status
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
async fn navbar_switches_pages_without_rebuilding_shadows_on_return() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    // Exercise actual tab presses and draw the resulting pages at Retina scale.
    desktop.scale_factor = 2.0;
    let mut fixture = ViewFixture::new(&desktop, Size::new(1248.0, 900.0)).await;
    let first=fixture.pixels(&desktop);
    assert_eq!(first[3],0,"the outside corner is completely clear, without a clipped outer window shadow");
    for (x,y) in [(0,450),(1247,450),(624,0),(624,899)] {
        assert_eq!(first[(y*1248+x)*4+3],255,"the visible face reaches the real window edge so resizing has no invisible outer margin");
    }
    let title=fixture.survey(&desktop).text.iter().find(|(text,_)|text=="Fetch").unwrap().1.center();
    let drag=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),title);
    assert!(drag.iter().any(|message|matches!(message,Message::DragWindow)),"the top bar can move the frameless window");
    fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),title);
    assert!(!fixture.click(&desktop,"Servers").iter().any(|message|matches!(message,Message::DragWindow)),"navigation buttons must capture their clicks without dragging the window");
    let blank=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),Point::new(50.0,860.0));
    assert!(blank.iter().any(|message|matches!(message,Message::DragWindow)),"unused space outside the top bar also drags the window");
    fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),Point::new(50.0,860.0));
    let mut first_frames = BTreeMap::new();
    for round in 0..2 {
        for (label, expected, content) in [
            ("Install MCPs", Page::Install, "Install MCPs"),
            ("Connect apps", Page::Connect, "Claude Code"),
            ("Servers", Page::Servers, "Installed servers"),
        ] {
            let started = std::time::Instant::now();
            apply_click(&mut fixture, &mut desktop, label);
            assert!(desktop.page == expected);
            assert!(fixture.survey(&desktop).text.iter().any(|(text, _)| text.contains(content)));
            let pixels = fixture.pixels(&desktop);
            let elapsed = started.elapsed();
            eprintln!("Navbar round {round} {label} at 2x: {:.3}s", elapsed.as_secs_f64());
            if round > 0 {
                assert!(elapsed < std::time::Duration::from_millis(250), "returning to {label} must not repeat the observed half-second or longer stall: {elapsed:?}");
                assert_eq!(first_frames[label], pixels, "returning to a page must preserve its first-frame appearance");
            }
            if round == 0 {
                if let Some(directory) = std::env::var_os("FETCH_UI_EVIDENCE_DIR") {
                    std::fs::create_dir_all(&directory).unwrap();
                    std::fs::write(std::path::PathBuf::from(directory).join(format!("navbar-{label}.rgba")), &pixels).unwrap();
                }
                first_frames.insert(label, pixels);
            }
        }
    }
}

#[tokio::test]
async fn server_expansion_opens_console_with_fetch_errors_and_server_output() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    let message="The upstream rejected its assigned provider access. Use the server's native setup or Fetch Authenticate for the online connection.";
    let mut servers=sample_servers();
    servers["servers"]=json!([servers["servers"][1].clone()]);
    let server=&mut servers["servers"][0];
    server["runtime"]=json!({"status":"failed","error":{"code":"PROVIDER_AUTH_REQUIRED","message":message,"details":{"status":401}}});
    server["authentication"]=json!({"status":"required","error":{"code":"PROVIDER_AUTH_REQUIRED","message":message,"details":{"status":401}},
        "startup":{"generation":1,"lines":["Remote service: access denied"],"exit_code":null,"signal":null}});
    let (revision,sequence)=desktop.auth_requests.begin_read();
    drop(desktop.update(Message::Servers(revision,sequence,desktop.window,Ok(servers.clone()))));
    let mut fixture=ViewFixture::new(&desktop,Size::new(1248.0,900.0)).await;
    apply_click(&mut fixture,&mut desktop,"Search");
    assert!(desktop.server_panel("search")==server_terminal::Pane::Console);
    let expected=vec!["[server] Remote service: access denied".to_owned(),format!("[Fetch] PROVIDER_AUTH_REQUIRED: {message} {{\"details\":{{\"status\":401}}}}")];
    let text=fixture.survey(&desktop).text;
    for line in &expected{assert_eq!(text.iter().filter(|(value,_)|value==line).count(),1,"each server/Fetch line appears once inside Console");}
    assert!(!text.iter().any(|(value,_)|value==message),"there is no separate card error below the tabs");
    let survey=fixture.survey(&desktop);
    let frame=survey.containers.iter().find(|bounds|(bounds.height-theme::CONSOLE_HEIGHT).abs()<0.1&&bounds.width>400.0).expect("Console frame");
    let copy=survey.containers.iter().find(|bounds|(bounds.width-40.0).abs()<0.1&&(bounds.height-40.0).abs()<0.1&&frame.contains(bounds.center())).expect("Console Copy icon").center();
    let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
    messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
    let copied=messages.into_iter().find_map(|message|if let Message::Copy(value)=message{Some(value)}else{None});
    assert_eq!(copied,Some(expected.join("\n")),"Copy includes exactly the displayed server and Fetch lines");
    let (revision,sequence)=desktop.auth_requests.begin_read();
    drop(desktop.update(Message::Servers(revision,sequence,desktop.window,Ok(servers))));
    assert_eq!(desktop.console_lines("search",&desktop.servers["servers"][0]),expected,"status polling does not duplicate either source");
    fixture.relayout(&desktop);
    apply_click(&mut fixture,&mut desktop,"Inspect tools");
    for _ in 0..2{
        apply_click(&mut fixture,&mut desktop,"Search");
        assert!(!desktop.expanded_servers.contains("search"));
        apply_click(&mut fixture,&mut desktop,"Search");
        assert!(desktop.server_panel("search")==server_terminal::Pane::Console,"re-expanding selects Console even when it was already selected");
    }
    assert!(!desktop.auth_active("search")&&!desktop.auth_requests.contains("search")&&!desktop.busy,"expansion remains a passive display action");
    fixture.export(&desktop,"expanded-error-console");
}

#[tokio::test]
async fn neumorphic_server_panels_open_from_their_own_controls_and_keep_key_ownership() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    drop(desktop.update(Message::ToggleServer("filesystem".into())));
    let mut fixture = ViewFixture::new(&desktop, Size::new(1248.0, 900.0)).await;
    let texts = fixture.survey(&desktop).text;
    assert!(!texts.iter().any(|(text, _)| text == "Available tools"));
    assert!(desktop.server_panel("filesystem") == server_terminal::Pane::Console);
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
    let inner = survey.scrolls.iter().find(|(id, _, _, _)| id.contains("fetch-tools-filesystem")).expect("tools have a bounded scrolling viewport").clone();
    assert_eq!(inner.1.height,300.0,"the tool scroll box retains its compact fixed height");
    assert!(inner.1.width<=744.0&&inner.2.height>inner.1.height+100.0);
    let before = fixture.pixels(&desktop);
    let last_card=survey.containers.iter().filter(|rect|(rect.height-200.0).abs()<0.1&&rect.y<bounds[0].y&&rect.y+rect.height>bounds[0].y).max_by(|a,b|a.x.total_cmp(&b.x)).expect("rightmost tool card");
    let shadow_x=(last_card.x+last_card.width+4.0).round() as usize;
    let shadow_y=(last_card.y+last_card.height-12.0).round() as usize;
    let shadow=&before[(shadow_y*1248+shadow_x)*4..][..3];
    assert!(shadow[0]<230,"the rightmost card shadow must extend into the tools padding instead of ending at a hard grid boundary: {shadow:?}");
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
async fn server_actions_fade_in_hover_glow_without_tooltip_text() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let heading=survey.text.iter().find(|(text,_)|text=="Search").unwrap().1;
    let action=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).min_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
    let initial=std::time::Instant::now();
    fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,initial);
    // Settle first-load cards before measuring only the hover effect.
    let now=initial+std::time::Duration::from_secs(1);
    let idle=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    let cursor=iced::mouse::Cursor::Available(action);
    let start=fixture.pixels_at(&desktop,cursor,now);
    let middle=fixture.pixels_at(&desktop,cursor,now+std::time::Duration::from_millis(75));
    let hovered=fixture.pixels_at(&desktop,cursor,now+std::time::Duration::from_millis(150));
    let difference=|pixels:&[u8]|pixels.iter().zip(&start).map(|(a,b)|a.abs_diff(*b) as u64).sum::<u64>();
    assert!(difference(&middle)>0&&difference(&hovered)>difference(&middle),"the hover glow grows over 150ms instead of appearing instantly");
    {
        let mut element=desktop.view(desktop.window.unwrap());
        let overlay=element.as_widget_mut().overlay(&mut fixture.tree,Layout::new(&fixture.node),&fixture.renderer,&fixture.screen,Vector::ZERO);
        assert!(overlay.is_none(),"hovering a server action must not create tooltip text");
    }
    assert!(fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(200))==idle,"leaving the action removes the glow");
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){
        std::fs::create_dir_all(&dir).unwrap();
        for (name,pixels) in [("hover-idle",idle),("hover-mid",middle),("hover-full",hovered)]{std::fs::write(std::path::PathBuf::from(&dir).join(format!("{name}-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),pixels).unwrap();}
    }
}

#[tokio::test]
async fn console_and_terminal_switch_without_waiting_for_server_io() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();desktop.expanded_servers.insert("filesystem".into());desktop.scale_factor=2.0;
    let mut flow=authentication::Flow::passive(&Value::Null);
    flow.log=(0..4096).map(|index|format!("[server] Fixture log line {index}: completed request")).collect();
    desktop.authentication.insert("filesystem".into(),flow);
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    fixture.pixels(&desktop);
    for round in 0..3{
        for (label,expected) in [("Console",server_terminal::Pane::Console),("Terminal",server_terminal::Pane::Terminal)]{
            let started=std::time::Instant::now();apply_click(&mut fixture,&mut desktop,label);
            assert!(desktop.server_panel("filesystem")==expected,"pane selection must update before any HTTP response");
            // The stock terminal publishes its initial size on redraw. Handle
            // that native widget event while dropping all resulting HTTP tasks.
            for message in fixture.event(&desktop,iced::Event::Window(iced::window::Event::RedrawRequested(std::time::Instant::now())),Point::ORIGIN){drop(desktop.update(message));}
            fixture.relayout(&desktop);
            let pixels=fixture.pixels(&desktop);
            let elapsed=started.elapsed();
            eprintln!("Output switch round {round} {label} at 2x: {:.3}s",elapsed.as_secs_f64());
            // Keep timing as benchmark evidence: this shared development Mac
            // can be busy compiling or drawing other windows. Correctness is
            // selection before server IO and preserved pane/input behavior.
            let survey=fixture.survey(&desktop);
            let frame=survey.containers.iter().find(|rect|(rect.height-theme::CONSOLE_HEIGHT).abs()<0.1&&rect.width>400.0).expect("shared output frame");
            let rgb_at=|x:f32,y:f32|{let i=(y.round() as usize*pixel_width+x.round() as usize)*4;pixels[i..i+3].to_vec()};
            assert_eq!(rgb_at(frame.x+24.0,frame.y+20.0),vec![36,43,48],"both output panes share the Console header/background");
            assert_eq!(rgb_at(frame.x+frame.width-24.0,frame.y+100.0),vec![36,43,48],"the terminal canvas also uses the Console background");
            if round==1&&label=="Terminal"{
                let start=Point::new(frame.x+20.0,frame.y+64.0);
                let end=Point::new(frame.x+90.0,frame.y+64.0);
                let selected=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),start);
                assert!(!selected.iter().any(|message|matches!(message,Message::DragWindow)),"terminal selection must not drag the app window");
                for message in selected{drop(desktop.update(message));}
                for message in fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::CursorMoved{position:end}),end){drop(desktop.update(message));}
                for message in fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),end){drop(desktop.update(message));}
                let copy=survey.containers.iter().find(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&frame.contains(rect.center())).expect("Terminal shares the Copy control").center();
                let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
                messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
                assert!(messages.iter().any(|message|matches!(message,Message::Copy(text) if !text.is_empty()&&"fixture-shell$ preserved interactive terminal".contains(text))),"Copy reuses the terminal's selected text");
                assert!(!messages.iter().any(|message|matches!(message,Message::DragWindow)),"Copy retains its click instead of dragging the window");
                fixture.export(&desktop,"terminal-matching-console");
            }
        }
        if round==0{
            drop(desktop.update(Message::Terminal(server_terminal::Message::Opened("filesystem".into(),0,Ok(client::TerminalOpened{created:true,value:json!({"session_id":"fixture-session","status":"running","output_cursor":0})})))));
            use base64::Engine;
            let bytes=b"fixture-shell$ preserved interactive terminal";
            drop(desktop.update(Message::Terminal(server_terminal::Message::Output("filesystem".into(),"fixture-session".into(),0,Ok(json!({"data":base64::engine::general_purpose::STANDARD.encode(bytes),"next_cursor":bytes.len(),"status":"running"}))))));
            fixture.relayout(&desktop);
        }
    }
}

#[tokio::test]
async fn installed_servers_scroll_inside_a_fixed_section() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = json!({"servers":(1..=8).map(|index|json!({
        "server_id":format!("server-{index}"),"name":format!("Server {index}"),
        "runtime":{"status":"ready"},"authentication":{"status":"not_required"},
        "config":{"kind":"remote"}
    })).collect::<Vec<_>>()});
    for size in [Size::new(1248.0,900.0),Size::new(760.0,520.0)] {
        desktop.expanded_servers.clear();
        let mut fixture=ViewFixture::new(&desktop,size).await;
        let before=fixture.survey(&desktop);
        let list=before.scrolls.iter().find(|(id,_,_,_)|id.contains("fetch-installed-servers")).expect("server cards have their own scroll area").clone();
        let heading=before.text.iter().find(|(text,_)|text=="Installed servers").unwrap().1;
        assert!(heading.y+heading.height<list.1.y,"the title is above the recessed card scroll area");
        assert!(list.2.height>list.1.height+200.0,"the fixture overflows the fixed section");
        assert!(list.1.y>0.0&&list.1.y+list.1.height<=size.height,"server list stays inside the window");
        apply_click(&mut fixture,&mut desktop,"Server 1");
        let expanded=fixture.survey(&desktop);
        assert_eq!(expanded.scrolls.iter().find(|(id,_,_,_)|*id==list.0).unwrap().1,list.1,"expanding a card does not grow the section");
        fixture.export(&desktop,"installed-servers-before-scroll");
        let pixels=fixture.pixels(&desktop);
        // Scroll the outer gutter: the expanded Console now intentionally
        // owns wheel movement over its own viewport, even at its boundary.
        let list_gutter=Point::new(list.1.x+5.0,list.1.center_y());
        let messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-2000.0}}),list_gutter);
        assert!(messages.is_empty(),"scrolling the list does not start backend work");
        let after=fixture.survey(&desktop);
        assert!(after.scrolls.iter().any(|(id,bounds,_,translation)|*id==list.0&&*bounds==list.1&&translation.y>200.0));
        for label in ["Fetch","Installed servers"] {
            assert_eq!(before.text.iter().find(|(text,_)|text==label).unwrap().1,after.text.iter().find(|(text,_)|text==label).unwrap().1,"{label} stays fixed while cards scroll");
        }
        let last=after.text.iter().find(|(text,_)|text=="Server 8").unwrap().1;
        assert!(list.1.contains(last.center()),"scrolling reaches the final server card");
        assert_ne!(pixels,fixture.pixels(&desktop),"scrolling visibly changes the server cards");
        fixture.export(&desktop,"installed-servers-after-scroll");
        apply_click(&mut fixture,&mut desktop,"Server 8");
        assert!(desktop.expanded_servers.contains("server-8"),"a scrolled server still expands from its header");
        fixture.relayout(&desktop);
        assert_eq!(fixture.survey(&desktop).scrolls.iter().find(|(id,_,_,_)|*id==list.0).unwrap().1,list.1);
    }
}

#[tokio::test]
async fn neumorphic_views_render_real_screens_at_design_and_minimum_window_sizes() {
    let (_directory, mut desktop) = auth_tests::desktop_fixture().await;
    desktop.servers = sample_servers();
    for size in [window_settings().size, Size::new(760.0, 520.0)] {
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
        let popup_pixels=fixture.pixels(&desktop);
        assert_eq!(popup_pixels[3],0,"discovery shading follows the rounded window corner");
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
        assert!(survey.text.iter().any(|(text, _)| text.contains("codex mcp add fetch -- ") && text.contains("connect --client codex")), "copyable Codex commands are retained");
        assert!(!survey.text.iter().any(|(text,_)|text=="Copy Fetch MCP address"||text.starts_with("Public certificate:")),"the removed connection footer stays absent");
        assert!(!survey.text.iter().any(|(text, _)| text == "Quit Fetch" || text == "Settings"), "navigation has only the three approved tabs");
    }
}

#[tokio::test]
async fn first_server_expansion_draws_without_server_io() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();desktop.scale_factor=2.0;
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    fixture.pixels(&desktop);
    for round in 0..3 {
        let started=std::time::Instant::now();
        apply_click(&mut fixture,&mut desktop,"Filesystem");
        assert!(desktop.expanded_servers.contains("filesystem"));
        fixture.pixels(&desktop);
        eprintln!("Server expansion {round}: {:.3}s",started.elapsed().as_secs_f64());
        assert!(fixture.survey(&desktop).text.iter().any(|(label,_)|label=="Inspect tools"));
        apply_click(&mut fixture,&mut desktop,"Filesystem");fixture.pixels(&desktop);
    }
}

#[tokio::test]
async fn refresh_hover_spins_without_invoking_refresh_and_stops_on_leave() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let survey=fixture.survey(&desktop);
    let heading=survey.text.iter().find(|(text,_)|text=="Search").unwrap().1;
    let key=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).min_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
    let refresh=Point::new(key.x+54.0,key.y);
    let initial=std::time::Instant::now();
    fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,initial);
    let now=initial+std::time::Duration::from_secs(1);
    let idle=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    let cursor=iced::mouse::Cursor::Available(refresh);
    fixture.pixels_at(&desktop,cursor,now);
    let first=fixture.pixels_at(&desktop,cursor,now+std::time::Duration::from_millis(200));
    let next=fixture.pixels_at(&desktop,cursor,now+std::time::Duration::from_millis(400));
    let glyph=|pixels:&[u8]|{let mut output=vec![];for y in (refresh.y as usize-9)..(refresh.y as usize+9){let offset=(y*pixel_width+refresh.x as usize-9)*4;output.extend_from_slice(&pixels[offset..offset+72]);}output};
    assert_ne!(glyph(&first),glyph(&next),"the arrow keeps rotating after the glow finishes fading in");
    assert!(fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(500))==idle,"leaving restores the original arrow and glow");
    let messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),refresh);
    assert!(messages.is_empty());
    let messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),refresh);
    assert!(messages.iter().any(|message|matches!(message,Message::RestartClicked(id) if id=="search")),"the original restart action is preserved");
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(std::path::PathBuf::from(dir).join(format!("refresh-hover-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),first).unwrap();
    }
    desktop.busy=true;fixture.relayout(&desktop);
    let first=fixture.pixels_at(&desktop,cursor,now);
    assert!(fixture.pixels_at(&desktop,cursor,now+std::time::Duration::from_millis(400))==first,"disabled refresh does not animate");
}

#[tokio::test]
async fn manual_refresh_keeps_spinning_away_from_pointer_until_its_reply() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let heading=survey.text.iter().find(|(text,_)|text=="Installed servers").unwrap().1;
    let action=survey.containers.iter().find(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).unwrap().center();
    for failed in [false,true] {
        let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),action);
        messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),action));
        assert!(messages.iter().any(|message|matches!(message,Message::Refresh)));
        for message in messages{drop(desktop.update(message));}
        let sequence=desktop.server_refresh.pending.expect("click immediately starts its own refresh indicator");
        fixture.relayout(&desktop);
        let now=std::time::Instant::now();
        let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(200));
        let next=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(400));
        assert_ne!(first,next,"the arrow keeps rotating while waiting, without hover");
        let result=if failed{Err("Fixture refresh failed".into())}else{Ok(sample_servers())};
        drop(desktop.update(Message::Servers(desktop.auth_requests.revision(),sequence,desktop.window,result)));
        assert!(desktop.server_refresh.pending.is_none(),"both success and failure finish the request");
        drop(desktop.update(Message::RefreshFeedbackDone(sequence)));
        fixture.relayout(&desktop);
        let now=std::time::Instant::now();
        fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
        let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(300));
        let later=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(600));
        assert!(first==later,"the arrow and card entrance stop after the request and minimum feedback finish");
    }
}

#[tokio::test]
async fn server_restart_spins_through_submission_and_completion_without_success_notice() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let heading=survey.text.iter().find(|(text,_)|text=="Search").unwrap().1;
    let key=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).min_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
    let restart=Point::new(key.x+54.0,key.y);
    let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),restart);
    messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),restart));
    assert!(messages.iter().any(|message|matches!(message,Message::RestartClicked(id) if id=="search")));
    for message in messages{drop(desktop.update(message));}
    assert!(desktop.restart_submitting.contains("search"));
    for phase in 0..2 {
        fixture.relayout(&desktop);
        let now=std::time::Instant::now();
        let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(200));
        let next=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(400));
        assert_ne!(first,next,"restart arrow rotates without hover in phase {phase}");
        if phase==0{
            drop(desktop.update(Message::Changed("search".into(),Ok(json!({"restart_id":"fixture-restart","status":"pending"})))));
            assert!(!desktop.restart_submitting.contains("search"));
            assert!(desktop.restarts.contains_key("search"));
        }
    }
    drop(desktop.update(Message::RestartStatus("search".into(),Ok(json!({"servers":[{"restart":{"status":"succeeded"}}]})))));
    assert!(!desktop.restarts.contains_key("search"));assert!(desktop.notice.is_empty());
    fixture.relayout(&desktop);
    let now=std::time::Instant::now();let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    assert_eq!(first,fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(300)));
    assert!(!fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Server restart completed."));
}

#[tokio::test]
async fn nested_console_tools_and_settings_keep_wheel_input_at_both_ends() {
    for pane in [server_terminal::Pane::Console,server_terminal::Pane::Tools,server_terminal::Pane::Settings] {
        let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
        desktop.servers=sample_servers();desktop.tools=tool_catalog();
        for index in 0..5 {
            let mut server=sample_servers()["servers"][0].clone();
            server["server_id"]=json!(format!("extra-{index}"));server["name"]=json!(format!("Extra {index}"));
            desktop.servers["servers"].as_array_mut().unwrap().push(server);
        }
        let mut flow=authentication::Flow::passive(&Value::Null);
        flow.log=(0..300).map(|index|format!("[server] Fixture output line {index}")).collect();
        desktop.authentication.insert("filesystem".into(),flow);
        desktop.expanded_servers.insert("filesystem".into());desktop.selected_server="filesystem".into();
        desktop.select_server_panel("filesystem",pane);
        let mut fixture=ViewFixture::new(&desktop,Size::new(1248.0,1100.0)).await;
        let survey=fixture.survey(&desktop);
        let outer=survey.scrolls.iter().find(|(id,_,_,_)|id.contains("fetch-installed-servers")).unwrap().clone();
        assert!(outer.2.height>outer.1.height,"surrounding list must have room to move");
        let inner=survey.scrolls.iter().find(|(id,_,_,_)|*id!=outer.0).expect("nested scroll area").clone();
        let point=inner.1.intersection(&outer.1).expect("nested pane visible").center();
        for delta in [-100_000.0,100_000.0] {
            // First reach the boundary; release ends Iced's prior wheel
            // transaction so a new gesture at that boundary is also checked.
            fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:delta}}),point);
            fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),point);
            for _ in 0..3 {
                let messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:delta}}),point);
                assert!(messages.is_empty(),"scrolling is local UI work");
                let after=fixture.survey(&desktop);
                assert_eq!(after.scrolls.iter().find(|(id,_,_,_)|*id==outer.0).unwrap().3,outer.3,"extra scrolling at an inner boundary must not move the surrounding list");
            }
        }
        // The surrounding list still works when the pointer leaves the pane.
        let outside=Point::new(outer.1.x+5.0,outer.1.y+outer.1.height-5.0);
        fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),outside);
        fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-200.0}}),outside);
        assert!(fixture.survey(&desktop).scrolls.iter().find(|(id,_,_,_)|*id==outer.0).unwrap().3.y>0.0,"the list scrolls normally outside a nested pane");
    }
}

#[tokio::test]
async fn global_errors_and_notices_do_not_render_below_navigation() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    desktop.error=Some("The server is restarting or its waiting queue is full.".into());
    desktop.notice="Server folder open request completed.".into();
    for page in [Page::Servers,Page::Install,Page::Connect] {
        desktop.page=page;
        let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
        let survey=fixture.survey(&desktop);
        for message in [desktop.error.as_ref().unwrap(),&desktop.notice] {
            assert!(!survey.text.iter().any(|(text,_)|text==message),"global messages do not appear beneath navigation on any page");
        }
    }
}

#[tokio::test]
async fn restart_failures_show_beside_the_affected_name_and_clear_on_retry() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    desktop.servers["servers"][0]["runtime"]=json!({"status":"failed"});
    desktop.servers["servers"][0]["restart"]=json!({"status":"failed","error":{"code":"START_FAILED","message":"Fixture server could not start"}});
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let title=survey.text.iter().find(|(text,_)|text=="Filesystem").unwrap().1;
    let warning=survey.text.iter().find(|(text,_)|text=="Restart failed").expect("collapsed failed server has an inline warning").1;
    assert!((title.center_y()-warning.center_y()).abs()<2.0&&warning.x>title.x+title.width);
    assert_eq!(survey.text.iter().filter(|(text,_)|text=="Restart failed").count(),1,"other servers keep their own status");
    assert!(desktop.console_lines("filesystem",&desktop.servers["servers"][0]).iter().any(|line|line.contains("START_FAILED: Fixture server could not start")));
    drop(desktop.update(Message::Restart("filesystem".into(),false)));fixture.relayout(&desktop);
    assert!(!fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Restart failed"),"retry replaces the old failure with active progress");
    let rejected="The server is restarting or its waiting queue is full.";
    drop(desktop.update(Message::Changed("filesystem".into(),Err(rejected.into()))));fixture.relayout(&desktop);
    assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Restart failed"),"a rejected restart request is also inline");
    assert!(desktop.console_lines("filesystem",&desktop.servers["servers"][0]).iter().any(|line|line.contains(rejected)),"actual returned error remains available in Console");
    fixture.export(&desktop,"restart-failed-inline");
    drop(desktop.update(Message::Restart("filesystem".into(),false)));
    drop(desktop.update(Message::Changed("filesystem".into(),Ok(json!({"restart_id":"retry","status":"pending"})))));
    drop(desktop.update(Message::RestartStatus("filesystem".into(),Ok(json!({"servers":[{"restart":{"status":"succeeded"}}]})))));
    fixture.relayout(&desktop);
    assert!(!fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Restart failed"),"successful retry clears the old inline warning immediately");
    drop(desktop.update(Message::Restart("filesystem".into(),false)));
    drop(desktop.update(Message::Changed("filesystem".into(),Ok(json!({"restart_id":"failed-retry","status":"pending"})))));
    drop(desktop.update(Message::RestartStatus("filesystem".into(),Ok(json!({"servers":[{"restart":{"status":"failed","error":{"code":"START_FAILED","message":"Fixture retry failed"}}}]})))));
    fixture.relayout(&desktop);
    assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Restart failed"),"terminal failure updates the header before another list response");
    assert!(desktop.notice.is_empty(),"restart failure is inline, without a global notice");
}

#[tokio::test]
async fn instant_manual_reload_replays_every_card_and_keeps_brief_spin_feedback() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let first=std::time::Instant::now();
    let initial=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,first);
    let early=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,first+std::time::Duration::from_millis(30));
    let survey=fixture.survey(&desktop);
    for (name,should_change) in [("Filesystem",true),("Search",false)]{
        let title=survey.text.iter().find(|(text,_)|text==name).unwrap().1;
        let card=survey.text.iter().filter(|(text,bounds)|text=="Available in every project"&&bounds.y>title.y).min_by(|(_,a),(_,b)|a.y.total_cmp(&b.y)).unwrap().1;
        let crop=|pixels:&[u8]|{let mut result=vec![];for y in card.y.ceil() as usize..(card.y+card.height).floor() as usize{let x=card.x.ceil() as usize;let end=(card.x+card.width).floor() as usize;result.extend_from_slice(&pixels[(y*pixel_width+x)*4..(y*pixel_width+end)*4]);}result};
        assert_eq!(crop(&initial)!=crop(&early),should_change,"first-load entry progresses from the first row downward");
    }
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){
        std::fs::write(std::path::PathBuf::from(&dir).join(format!("first-load-start-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),initial).unwrap();
        std::fs::write(std::path::PathBuf::from(&dir).join(format!("first-load-stagger-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),early).unwrap();
    }

    let heading=survey.text.iter().find(|(text,_)|text=="Installed servers").unwrap().1;
    let action=survey.containers.iter().find(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).unwrap().center();
    let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),action);
    messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),action));
    for message in messages{drop(desktop.update(message));}
    let sequence=desktop.server_refresh.pending.unwrap();
    drop(desktop.update(Message::Servers(desktop.auth_requests.revision(),sequence,desktop.window,Ok(sample_servers()))));
    assert!(desktop.server_refresh.pending.is_none());assert_eq!(desktop.server_refresh.minimum,Some(sequence),"instant replies still retain minimum click feedback");
    assert_eq!(desktop.reload_generation,1);assert_eq!(desktop.servers,sample_servers());
    fixture.relayout(&desktop);
    let now=std::time::Instant::now();
    let start=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    let middle=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(120));
    let settled=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(300));
    let later=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(400));
    let survey=fixture.survey(&desktop);
    for name in ["Filesystem","Search"] {
        let title=survey.text.iter().find(|(text,_)|text==name).unwrap().1;
        let card=survey.containers.iter().find(|rect|rect.width>700.0&&rect.height>90.0&&rect.height<200.0&&rect.contains(title.center())).unwrap();
        let crop=|pixels:&[u8]|{let mut result=vec![];for y in card.y.ceil() as usize..(card.y+card.height).floor() as usize{let x=card.x.ceil() as usize;let end=(card.x+card.width).floor() as usize;result.extend_from_slice(&pixels[(y*pixel_width+x)*4..(y*pixel_width+end)*4]);}result};
        assert!(crop(&start)!=crop(&middle)&&crop(&middle)!=crop(&settled),"every visible card visibly pops back in");
        assert!(crop(&settled)==crop(&later),"card appearance settles while the arrow finishes its feedback");
    }
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){
        for (name,pixels) in [("reload-start",start),("reload-middle",middle),("reload-settled",settled)]{std::fs::write(std::path::PathBuf::from(&dir).join(format!("{name}-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),pixels).unwrap();}
    }
    drop(desktop.update(Message::RefreshFeedbackDone(sequence)));assert!(desktop.server_refresh.minimum.is_none());
    let(revision,sequence)=desktop.auth_requests.begin_read();
    drop(desktop.update(Message::Servers(revision,sequence,desktop.window,Ok(sample_servers()))));
    assert_eq!(desktop.reload_generation,1,"ordinary background reads do not replay the list");
}

#[tokio::test]
async fn remove_button_hover_has_a_red_glow() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let survey=fixture.survey(&desktop);
    let heading=survey.text.iter().find(|(text,_)|text=="Search").unwrap().1;
    let key=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).min_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
    let remove=Point::new(key.x+108.0,key.y);
    let initial=std::time::Instant::now();
    fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,initial);
    let now=initial+std::time::Duration::from_secs(1);
    let idle=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    fixture.pixels_at(&desktop,iced::mouse::Cursor::Available(remove),now);
    let hover=fixture.pixels_at(&desktop,iced::mouse::Cursor::Available(remove),now+std::time::Duration::from_millis(150));
    let index=(remove.y as usize*pixel_width+(remove.x as usize+26))*4;
    assert!((hover[index] as i16-hover[index+1] as i16)>(idle[index] as i16-idle[index+1] as i16)+8,"the halo outside the remove circle is red");
    assert!(fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(200))==idle,"leaving removes its glow");
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){std::fs::write(std::path::PathBuf::from(dir).join(format!("remove-hover-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),hover).unwrap();}
}

#[tokio::test]
async fn connect_commands_are_first_and_removed_copy_stays_absent() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;desktop.page=Page::Connect;
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let bounds=|label:&str|survey.text.iter().find(|(text,_)|text==label).unwrap().1;
    assert!(bounds("Claude Code").y<bounds("Codex").y);
    assert!(!survey.text.iter().any(|(text,_)|text=="OpenCode"||text.starts_with("Each app connects once")||text=="Set up Claude Code"||text=="Set up Codex"||text=="Copy Fetch MCP address"||text.starts_with("Public certificate:")||text.starts_with("Set up each app above")));
    assert!(survey.text.iter().any(|(text,_)|text.starts_with("claude mcp add --transport stdio fetch ")));
    assert!(survey.text.iter().any(|(text,_)|text.starts_with("codex mcp add fetch -- ")&&text.contains("connect --client codex")));
    fixture.export(&desktop,"connect-commands-first");
}

#[tokio::test]
async fn copy_control_shines_and_confirms_to_its_left_then_clears() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;desktop.page=Page::Connect;
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let survey=fixture.survey(&desktop);
    let command=survey.text.iter().find(|(text,_)|text.starts_with("claude mcp add ")).unwrap().clone();
    let button=survey.containers.iter().filter(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1).min_by(|a,b|(a.center_y()-command.1.center_y()).abs().total_cmp(&(b.center_y()-command.1.center_y()).abs())).unwrap().clone();
    let idle=fixture.pixels(&desktop);let now=std::time::Instant::now();
    let hover=fixture.pixels_at(&desktop,iced::mouse::Cursor::Available(button.center()),now);
    assert!(hover!=idle,"copy button has a slight visible hover shine");
    let outside=((button.center_y() as usize)*pixel_width+button.x as usize+4)*4;
    assert_eq!(&hover[outside..outside+4],&idle[outside..outside+4],"hover brightens the icon without a button background");
    let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),button.center());
    messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),button.center()));
    assert!(messages.iter().any(|message|matches!(message,Message::Copy(value) if value==&command.0)),"copy keeps the exact displayed command");
    for message in messages{drop(desktop.update(message));}fixture.relayout(&desktop);
    let copied=fixture.survey(&desktop);
    let label=copied.text.iter().find(|(text,_)|text=="Copied!").expect("click gives local inline feedback").1;
    assert!(label.x+label.width<button.x&&(label.center_y()-button.center_y()).abs()<2.0);
    assert_eq!(copied.text.iter().filter(|(text,_)|text=="Copied!").count(),1,"confirmation belongs only to the clicked copy button");
    fixture.export(&desktop,"copy-confirmation");
    fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,std::time::Instant::now()+std::time::Duration::from_millis(1500));
    assert!(!fixture.survey(&desktop).text.iter().any(|(text,_)|text=="Copied!"),"confirmation disappears after its brief timeout");
}

#[tokio::test]
async fn starting_servers_spin_without_unknown_warning_and_success_updates_the_card_immediately() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    desktop.servers["servers"][0]["runtime"]=json!({"status":"starting"});
    desktop.servers["servers"][0]["authentication"]=json!({"status":"unknown"});
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let survey=fixture.survey(&desktop);
    assert!(!survey.text.iter().any(|(text,_)|text=="Sign-in status unknown"));
    let name=survey.text.iter().find(|(text,_)|text=="Filesystem").unwrap().1;
    let now=std::time::Instant::now();fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(200));
    let next=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(400));
    assert!(first!=next,"API-reported startup spins even without a local restart click");
    let dot_x=(name.x+name.width+14.0).round() as usize;let dot_y=name.center_y().round() as usize;
    let pixel=&next[(dot_y*pixel_width+dot_x)*4..][..3];
    assert!(pixel[0]>210&&pixel[1]>120&&pixel[1]<190&&pixel[2]<110,"startup dot is orange: {pixel:?}");
    drop(desktop.update(Message::Tick));assert!(desktop.polling.is_some(),"collapsed starting servers keep automatic status checks active");
    drop(desktop.update(Message::RestartStatus("filesystem".into(),Ok(json!({"servers":[{"server_id":"filesystem","runtime":{"status":"ready"},"authentication":{"status":"not_required"},"restart":{"status":"succeeded"}}]})))));
    assert_eq!(desktop.servers["servers"][0]["runtime"]["status"],"ready","successful response updates runtime immediately without a list refresh reply");
    assert_eq!(desktop.servers["servers"][0]["authentication"]["status"],"not_required");
    fixture.relayout(&desktop);
    let now=std::time::Instant::now();let settled=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
    assert!(settled==fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+std::time::Duration::from_millis(300)),"successful start immediately stops its spinner");
    fixture.export(&desktop,"startup-completed");
}

#[tokio::test]
async fn white_console_copy_icon_shines_without_a_background() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;desktop.servers=sample_servers();
    desktop.expanded_servers.insert("filesystem".into());desktop.select_server_panel("filesystem",server_terminal::Pane::Console);
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let pixel_width=fixture.screen.width as usize;
    let survey=fixture.survey(&desktop);
    let frame=survey.containers.iter().find(|rect|(rect.height-theme::CONSOLE_HEIGHT).abs()<0.1&&rect.width>400.0).unwrap();
    let button=survey.containers.iter().find(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&frame.contains(rect.center())).unwrap();
    let idle=fixture.pixels(&desktop);let now=std::time::Instant::now();
    let hover=fixture.pixels_at(&desktop,iced::mouse::Cursor::Available(button.center()),now);
    let outside=(button.center_y() as usize*pixel_width+button.x as usize+4)*4;
    assert_eq!(&hover[outside..outside+4],&idle[outside..outside+4],"white icon shine does not add a button background");
    assert!(idle!=hover,"the white copy glyph has visible shine");
    if let Some(dir)=std::env::var_os("FETCH_UI_EVIDENCE_DIR"){std::fs::write(std::path::PathBuf::from(dir).join(format!("white-copy-shine-{}x{}.rgba",fixture.screen.width as u32,fixture.screen.height as u32)),hover).unwrap();}
}

#[tokio::test]
async fn connector_setup_popup_shows_requested_scopes_and_requires_explicit_choice() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.page=Page::Connect;
    let request=json!({"request_id":"fixture-request","client_name":"Disposable Claude Code","code":"482193","status":"pending","scopes":["fetch:catalog","fetch:tools:call","fetch:content:read","fetch:manage"]});
    let _=desktop.update(Message::Connector(connector_setup::Event::Requests(Ok(json!({"requests":[request]})))));
    assert!(desktop.connector.popup_visible());
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    for label in ["Connect Disposable Claude Code to Fetch?","482193","See tool, resource and prompt descriptions","Call tools, including tools that change data","Read resources and prompts","Manage installed servers","Allow","Deny"] {
        assert!(survey.text.iter().any(|(text,_)|text==label),"missing popup text: {label}");
    }
    assert!(survey.text.iter().any(|(text,_)|text=="Request code"));
    assert!(!survey.text.iter().any(|(text,_)|text.starts_with("Match this code with")));
    let code=survey.text.iter().find(|(text,_)|text=="482193").unwrap().1;
    let copy=survey.containers.iter().find(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&(rect.center_y()-code.center_y()).abs()<2.0&&rect.x>code.x).unwrap().center();
    let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
    messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
    assert!(messages.iter().any(|message|matches!(message,Message::Copy(value) if value=="482193")),"Copy sends only the request code");
    fixture.export(&desktop,"connector-approval-popup");
    // Constructing/rendering the popup runs no approval task and grants no access.
    let _=desktop.update(Message::Connector(connector_setup::Event::Decided("fixture-request".into(),false,Ok(json!({"status":"denied"})))));
    assert!(!desktop.connector.popup_visible());
    assert!(desktop.connector.status.contains("denied"));
}


#[tokio::test]
async fn install_refresh_shares_request_spin_minimum_feedback_and_retry_behavior() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.page=Page::Install;
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let action=fixture.survey(&desktop).containers.iter().find(|rect|
        (rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1
    ).unwrap().center();
    let now=std::time::Instant::now();let cursor=iced::mouse::Cursor::Available(action);
    fixture.pixels_at(&desktop,cursor,now);
    let first=fixture.pixels_at(&desktop,cursor,now+Duration::from_millis(200));
    assert_ne!(first,fixture.pixels_at(&desktop,cursor,now+Duration::from_millis(400)),"empty-path refresh keeps the shared hover rotation");
    fixture.export(&desktop,"install-refresh-idle");
    drop(desktop.update(Message::Discover));
    assert!(!desktop.busy&&!desktop.discovery_started&&!desktop.discovery_refresh.running(),"empty path never scans defaults or starts a request");
    desktop.config_path="/fixture/chosen/.codex".into();
    fixture.relayout(&desktop);
    for slow_failure in [false,true] {
        let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),action);
        messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),action));
        assert!(messages.iter().any(|message|matches!(message,Message::Discover)),"click uses the real discovery action");
        for message in messages { drop(desktop.update(message)); }
        let sequence=desktop.discovery_refresh.pending.expect("discovery starts its own indicator");
        assert!(desktop.busy);
        assert!(desktop.discovery_started,"discovery still opens its existing results popup");
        drop(desktop.update(Message::CloseDiscovery));
        fixture.relayout(&desktop);
        let now=std::time::Instant::now();
        let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
        let next=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+Duration::from_millis(200));
        assert_ne!(first,next,"the pending arrow spins without hover, just like Servers");
        let mut repeated=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),action);
        repeated.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),action));
        assert!(!repeated.iter().any(|message|matches!(message,Message::Discover)),"a running refresh cannot submit twice");
        if slow_failure {
            drop(desktop.update(Message::DiscoveryFeedbackDone(sequence)));
            assert!(desktop.discovery_refresh.running(),"a slow read outlasts minimum feedback");
            drop(desktop.update(Message::Discovery(Err("Fixture source could not be read".into()))));
            assert_eq!(desktop.error.as_deref(),Some("Fixture source could not be read"));
        } else {
            let result=json!({"configs":[],"next_steps":[]});
            drop(desktop.update(Message::Discovery(Ok(result.clone()))));
            assert_eq!(desktop.discovery,result,"fast results apply before animation finishes");
            assert!(!desktop.busy,"animation does not keep the rest of installation busy");
            assert!(desktop.discovery_refresh.running(),"fast replies retain minimum feedback");
            fixture.relayout(&desktop);
            fixture.export(&desktop,"install-refresh-feedback");
            let now=std::time::Instant::now();
            let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
            assert_ne!(first,fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+Duration::from_millis(200)));
            let generation=desktop.discovery_refresh.generation;
            drop(desktop.update(Message::Discover));
            assert_eq!(desktop.discovery_refresh.generation,generation,"keyboard and click actions share the same feedback guard");
            drop(desktop.update(Message::DiscoveryFeedbackDone(sequence)));
        }
        assert!(!desktop.discovery_refresh.running(),"both success and failure allow retry once feedback and work finish");
        fixture.relayout(&desktop);
        let now=std::time::Instant::now();
        let first=fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now);
        assert_eq!(first,fixture.pixels_at(&desktop,iced::mouse::Cursor::Unavailable,now+Duration::from_millis(200)),"finished refresh stops spinning");
    }
}


#[tokio::test]
async fn inline_app_keys_open_and_reopen_dismissible_approval_with_rounded_shading() {
    for (client,label,size) in [("claude-code","Claude Code",window_settings().size),("codex","Codex",Size::new(760.0,520.0))] {
        let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
        desktop.page=Page::Connect;
        let mut fixture=ViewFixture::new(&desktop,size).await;
        let survey=fixture.survey(&desktop);
        let heading=survey.text.iter().find(|(text,_)|text==label).unwrap().1;
        let viewport=survey.scrolls.iter().find(|(id,_,_,_)|id.contains("fetch-connect-apps")).unwrap().1;
        if !viewport.contains(heading.center()){
            fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-300.0}}),viewport.center());
        }
        let survey=fixture.survey(&desktop);
        let heading=survey.text.iter().find(|(text,_)|text==label).unwrap().1;
        let key=survey.containers.iter().find(|rect|(rect.width-22.0).abs()<0.1&&(rect.height-22.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<2.0).unwrap().center();
        assert!(key.x>heading.x+heading.width,"authentication is inline to the right of the app heading");
        fixture.export(&desktop,"connect-inline-auth");
        let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),key);
        messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),key));
        assert!(messages.iter().any(|message|matches!(message,Message::Connector(connector_setup::Event::Start(name)) if *name==client)),"each key requests its own app");
        for message in messages{drop(desktop.update(message));}
        let request=json!({"request_id":"inline-fixture","client_name":client,"code":"482193","claim_secret":"disposable-fixture-proof","expires_at":crate::connector::profile::now()+600,"status":"pending","scopes":["fetch:catalog","fetch:tools:call","fetch:content:read"]});
        drop(desktop.update(Message::Connector(connector_setup::Event::Created(client.into(),Ok(request.clone())))));
        assert!(desktop.connector.popup_visible());
        fixture.relayout(&desktop);
        fixture.export(&desktop,"inline-auth-popup");
        let pixels=fixture.pixels(&desktop);
        assert_eq!(pixels[3],0,"popup dimming leaves the rounded app corner transparent");
        let last=((size.width as usize)*(size.height as usize)-1)*4+3;
        assert_eq!(pixels[last],0,"the bottom-right corner is also outside the shade");
        let survey=fixture.survey(&desktop);
        let title=survey.text.iter().find(|(text,_)|text==&format!("Connect {client} to Fetch?")).unwrap().1;
        let close=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-title.center_y()).abs()<2.0).max_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
        let inside=survey.text.iter().find(|(text,_)|text=="482193").unwrap().1.center();
        assert!(!fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),inside).iter().any(|message|matches!(message,Message::Connector(connector_setup::Event::Dismiss))),"inside clicks do not dismiss");
        fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),inside);
        for point in [close,Point::new(18.0,120.0)] {
            let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),point);
            messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),point));
            assert!(messages.iter().any(|message|matches!(message,Message::Connector(connector_setup::Event::Dismiss))));
            assert!(!messages.iter().any(|message|matches!(message,Message::Connector(connector_setup::Event::Decide(_))|Message::DragWindow)),"hiding only changes popup visibility");
            for message in messages{drop(desktop.update(message));}
            assert!(!desktop.connector.popup_visible());
            drop(desktop.update(Message::Connector(connector_setup::Event::Requests(Ok(json!({"requests":[request.clone()]}))))));
            assert!(!desktop.connector.popup_visible(),"polling must not reopen a dismissed popup");
            fixture.relayout(&desktop);
            let mut messages=fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),key);
            messages.extend(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),key));
            assert!(messages.iter().any(|message|matches!(message,Message::Connector(connector_setup::Event::Start(name)) if *name==client)));
            for message in messages{drop(desktop.update(message));}
            assert!(desktop.connector.popup_visible(),"the same key reopens the existing request");
            fixture.relayout(&desktop);
            assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text=="482193"));
        }
    }
}

#[tokio::test]
async fn escape_hides_only_the_top_pending_popup_without_canceling_approval_or_discovery(){
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.page=Page::Install;desktop.config_path="/fixture/config.json".into();
    drop(desktop.update(Message::Discover));
    let sequence=desktop.discovery_refresh.pending.expect("discovery request is pending");
    assert!(desktop.discovery_started&&desktop.busy);
    let request=json!({"request_id":"escape-fixture","client_name":"codex","code":"482193","claim_secret":"disposable-fixture-proof","expires_at":crate::connector::profile::now()+600,"status":"pending","scopes":["fetch:catalog","fetch:tools:call"]});
    drop(desktop.update(Message::Connector(connector_setup::Event::Created("codex".into(),Ok(request.clone())))));
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    assert!(desktop.connector.popup_visible());
    assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text=="482193"));

    press_escape(&mut fixture,&mut desktop,Point::new(400.0,350.0));
    assert!(!desktop.connector.popup_visible(),"Escape hides the top approval popup");
    assert!(desktop.discovery_started&&desktop.busy,"the lower installation popup and pending discovery remain");
    assert_eq!(desktop.discovery_refresh.pending,Some(sequence));
    assert!(!desktop.connector.deciding,"Escape does not approve or deny the request");
    drop(desktop.update(Message::Connector(connector_setup::Event::Requests(Ok(json!({"requests":[request.clone()]}))))));
    assert!(!desktop.connector.popup_visible(),"polling preserves the person's dismissal");

    press_escape(&mut fixture,&mut desktop,Point::new(400.0,350.0));
    assert!(!desktop.discovery_started,"the next Escape hides the installation popup");
    assert!(desktop.busy&&desktop.discovery_refresh.pending==Some(sequence),"hiding does not cancel the ongoing discovery");
    let task=desktop.update(Message::Connector(connector_setup::Event::Start("codex")));
    assert_eq!(task.units(),0,"the same pending approval reopens without creating another request");drop(task);
    fixture.relayout(&desktop);
    assert!(desktop.connector.popup_visible());
    assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text=="482193"));
}

#[tokio::test]
async fn server_power_toggle_waits_for_api_preserves_rejected_state_and_turns_on_again() {
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,window_settings().size).await;
    let survey=fixture.survey(&desktop);
    let title=survey.text.iter().find(|(text,_)|text=="Filesystem").unwrap().1;
    let key=survey.containers.iter().filter(|rect|(rect.width-44.0).abs()<0.1&&(rect.height-44.0).abs()<0.1&&(rect.center_y()-title.center_y()).abs()<2.0).min_by(|a,b|a.x.total_cmp(&b.x)).unwrap().center();
    let toggle=Point::new(key.x-54.0,key.y);
    let click=|fixture:&mut ViewFixture,desktop:&Desktop|{
        let mut messages=fixture.event(desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),toggle);
        messages.extend(fixture.event(desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),toggle));messages
    };
    let messages=click(&mut fixture,&desktop);
    assert!(messages.iter().any(|message|matches!(message,Message::PowerClicked(id,false) if id=="filesystem")),"On server's shared toggle asks the backend to turn it off");
    for message in messages{drop(desktop.update(message));}
    assert_eq!(desktop.power_pending.get("filesystem"),Some(&false));
    assert_ne!(desktop.servers["servers"][0]["config"]["enabled"],false,"the switch does not claim Off before the API reply");
    fixture.relayout(&desktop);
    assert!(!click(&mut fixture,&desktop).iter().any(|message|matches!(message,Message::PowerClicked(..))),"pending power requests cannot be submitted twice");
    let failure="SERVER_BUSY: Fixture tool call is still running.";
    drop(desktop.update(Message::PowerFinished("filesystem".into(),false,Err(failure.into()))));
    assert!(!desktop.power_pending.contains_key("filesystem"));
    assert_ne!(desktop.servers["servers"][0]["config"]["enabled"],false,"rejected Off preserves the prior switch state");
    assert_eq!(desktop.power_errors.get("filesystem").map(String::as_str),Some(failure));
    assert!(desktop.expanded_servers.contains("filesystem"));
    assert!(desktop.server_panel("filesystem")==server_terminal::Pane::Console);
    fixture.relayout(&desktop);
    assert!(fixture.survey(&desktop).text.iter().any(|(text,_)|text.contains(failure)),"the real power error is visible in that server's Console");
    fixture.export(&desktop,"server-power-busy-console");

    for message in click(&mut fixture,&desktop){drop(desktop.update(message));}
    drop(desktop.update(Message::PowerFinished("filesystem".into(),false,Ok(json!({"server_id":"filesystem","status":"stopped","enabled":false,"changed":true,"runtime":{"status":"stopped","error":null},"next_steps":[]})))));
    assert_eq!(desktop.servers["servers"][0]["config"]["enabled"],false);
    assert_eq!(desktop.servers["servers"][0]["runtime"]["status"],"stopped");
    assert!(!desktop.power_pending.contains_key("filesystem"));
    fixture.relayout(&desktop);
    fixture.export(&desktop,"server-power-off");
    let messages=click(&mut fixture,&desktop);
    assert!(messages.iter().any(|message|matches!(message,Message::PowerClicked(id,true) if id=="filesystem")),"Off server's toggle asks the backend to turn it on");
    for message in messages{drop(desktop.update(message));}
    drop(desktop.update(Message::PowerFinished("filesystem".into(),true,Ok(json!({"server_id":"filesystem","status":"restarting","enabled":true,"restart_id":"power-on","next_steps":[]})))));
    assert_eq!(desktop.power_pending.get("filesystem"),Some(&true),"On waits for actual startup completion");
    drop(desktop.update(Message::RestartStatus("filesystem".into(),Ok(json!({"servers":[{"server_id":"filesystem","config":{"enabled":true},"runtime":{"status":"ready","error":null},"restart":{"restart_id":"power-on","status":"succeeded"}}],"next_steps":[]})))));
    assert!(!desktop.power_pending.contains_key("filesystem"));
    assert_eq!(desktop.servers["servers"][0]["config"]["enabled"],true);
    assert_eq!(desktop.servers["servers"][0]["runtime"]["status"],"ready");
    fixture.relayout(&desktop);
    fixture.export(&desktop,"server-power-on");
}

#[tokio::test]
async fn installed_heading_aligns_with_filter_and_project_menu_uses_app_surface(){
    let (_directory,mut desktop)=auth_tests::desktop_fixture().await;
    desktop.page=Page::Servers;desktop.servers=sample_servers();
    let mut fixture=ViewFixture::new(&desktop,Size::new(874.0,630.0)).await;
    let survey=fixture.survey(&desktop);
    let left=|label:&str|survey.text.iter().find(|(text,_)|text==label).unwrap().1.x;
    assert!((left("Installed servers")-left("Show servers")).abs()<0.1);
    fixture.export(&desktop,"installed-heading-aligned");
    let label=survey.text.iter().find(|(text,_)|text=="Show servers").unwrap().1;
    let point=Point::new(label.x+170.0,label.center_y());
    assert!(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),point).is_empty());
    assert!(fixture.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),point).is_empty());
    {
        let mut element=desktop.view(desktop.window.unwrap());
        let mut overlay=element.as_widget_mut().overlay(&mut fixture.tree,Layout::new(&fixture.node),&fixture.renderer,&fixture.screen,Vector::ZERO).expect("project menu opens");
        let node=overlay.as_overlay_mut().layout(&fixture.renderer,fixture.screen.size());
        assert!(node.bounds().width>200.0);
    }
    fixture.export(&desktop,"project-menu-app-surface");
    assert_eq!(project_scope::Choice::Shared.to_string(),"Available in every project");
    assert_eq!(project_scope::Choice::Shared.query(),Some("global"));
}
