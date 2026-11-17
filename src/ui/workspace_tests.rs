//! Workspace interactions use fixture data and dropped Tasks. They never open
//! personal files, the system clipboard, model processes or native windows.
use super::{Desktop, Message, Page, auth_tests::desktop_fixture, neumorphic_tests::ViewFixture, workspace::{Event, Section}};
use iced::{Size, widget::text_editor};
use serde_json::{Value, json};

fn event(desktop: &mut Desktop, event: Event) { drop(desktop.update(Message::Workspace(event))); }
fn clicks(desktop: &mut Desktop, fixture: &mut ViewFixture, label: &str) {
    let messages = fixture.click(desktop, label);
    assert!(!messages.is_empty(), "{label} must be an actionable control");
    for message in messages { drop(desktop.update(message)); }
    fixture.relayout(desktop);
}
fn pick_app(desktop:&mut Desktop,view:&mut ViewFixture,app:&str){
    let survey=view.survey(desktop);
    let tabs=survey.text.iter().find(|(text,_)|text=="Instructions").unwrap().1;
    let add=survey.text.iter().find(|(text,_)|text=="Add subagent").map(|(_,rect)|rect.x+rect.width);
    let mut buttons:Vec<_>=survey.containers.iter().filter(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&(rect.center_y()-tabs.center_y()).abs()<2.0&&add.is_none_or(|right|rect.x>right)).copied().collect();
    buttons.sort_by(|a,b|a.x.total_cmp(&b.x));buttons.dedup_by(|a,b|(a.x-b.x).abs()<0.1);
    assert_eq!(buttons.len(),2,"both circular app choices share the tabs row");
    let point=buttons[usize::from(app=="codex")].center();
    let mut messages=view.event(desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),point);
    messages.extend(view.event(desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),point));
    assert!(messages.iter().any(|message|matches!(message,Message::Workspace(Event::App(value)) if value==app)));
    for message in messages{drop(desktop.update(message));}
    view.relayout(desktop);
}
fn discovered(folder: Option<&str>, files: Value, directory: &str) -> Value {
    json!({"folder_path":folder,"files":files,"agent_directories":[{"app":"codex","file_path":directory}],"next_steps":[]})
}
fn finish_names(desktop: &mut Desktop, sessions: Value) {
    let generation = desktop.workspace.generation;
    event(desktop, Event::Sessions(generation, Ok(json!({"sessions":sessions,"next_cursor":null}))));
}

fn hover_error_details(
    desktop: &Desktop,
    view: &mut ViewFixture,
    info: iced::Point,
    evidence_name: &str,
) {
    let before = view.pixels(desktop);
    let cursor = iced::mouse::Cursor::Available(info);
    let hovered = view.pixels_at(desktop, cursor, std::time::Instant::now());
    let changed_pixels = |frame: &[u8]| before.chunks_exact(4).zip(frame.chunks_exact(4))
        .filter(|(before, after)| before != after).count();
    let opened = changed_pixels(&hovered);
    assert!(opened > 500, "pointing at the info glyph opens the rendered error details");
    view.export_at(desktop, evidence_name, cursor);
    let closed = changed_pixels(&view.pixels(desktop));
    assert!(closed < opened / 10, "moving off the info glyph closes the details ({closed} of {opened} changed pixels remain)");
}

struct FixtureClipboard(String);
impl iced::advanced::Clipboard for FixtureClipboard {
    fn read(&self, _: iced::advanced::clipboard::Kind) -> Option<String> {
        Some(self.0.clone())
    }

    fn write(&mut self, _: iced::advanced::clipboard::Kind, content: String) {
        self.0 = content;
    }
}

#[tokio::test]
async fn add_subagent_opens_a_blank_editor_and_keeps_the_draft_on_filename_conflict() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.app = "codex".into();
    let agent_folder = "/fixture/project-a/.codex/agents";
    event(&mut desktop, Event::Discovered(0, Ok(discovered(Some("/fixture/project-a"), json!([]), agent_folder))));
    let size = super::window_settings().size;
    let mut view = ViewFixture::new(&desktop, size).await;
    assert!(!view.survey(&desktop).text.iter().any(|(label, _)| label == "Add subagent"));
    clicks(&mut desktop, &mut view, "Sessions");
    assert!(!view.survey(&desktop).text.iter().any(|(label, _)| label == "Add subagent"));
    clicks(&mut desktop, &mut view, "Subagents");

    let survey = view.survey(&desktop);
    let button = survey.text.iter().find(|(label, _)| label == "Add subagent").unwrap().1;
    let mut logos: Vec<_> = survey.containers.iter().filter(|rect| {
        (rect.width - 40.0).abs() < 0.1 && (rect.height - 40.0).abs() < 0.1
            && (rect.center_y() - button.center_y()).abs() < 2.0 && rect.x > button.x + button.width
    }).map(|rect| rect.x).collect();
    logos.sort_by(f32::total_cmp);
    logos.dedup();
    assert_eq!(logos.len(), 2);
    let distance = logos[0] - (button.x + button.width);
    assert!((20.0..30.0).contains(&distance), "Add subagent sits directly to the left of the logos");
    let pixels = view.pixels(&desktop);
    let face = (button.center_y() as usize * size.width as usize + (button.x - 6.0) as usize) * 4;
    assert!(pixels[face+2] >= 200 && pixels[face+2].saturating_sub(pixels[face]) > 30, "Add uses a blue button face");
    let label_has_white_text = (button.y.ceil() as usize..(button.y + button.height).floor() as usize).any(|y| {
        (button.x.ceil() as usize..(button.x + button.width).floor() as usize).any(|x| {
            let offset = (y * size.width as usize + x) * 4;
            pixels[offset..offset+3].iter().all(|&channel| channel >= 248)
        })
    });
    assert!(label_has_white_text, "Add has a white foreground label");
    view.export(&desktop, "workspace-add-subagent-toolbar");

    let mut messages = view.click(&desktop, "Add subagent");
    assert!(matches!(messages.as_slice(), [Message::Workspace(Event::AddAgent)]));
    // Add focuses the editor and starts the first catalog read. Deliver the
    // fixture reply without contacting an installed client.
    let focus = desktop.update(messages.remove(0));
    assert_eq!(focus.units(), 2);
    assert!(desktop.workspace.models_loading);
    drop(focus);
    event(&mut desktop, Event::ModelList(0, "codex".into(), Ok(json!({
        "source":"codex_cache", "models":[{"slug":"test-model","display_name":"Fixture model"}]
    }))));
    view.relayout(&desktop);
    let mut focus = iced::advanced::widget::operation::focusable::focus(super::workspace::AGENT_EDITOR_ID.into());
    view.operate(&desktop, &mut focus);
    assert!(desktop.workspace.agent_popup && desktop.workspace.agent_editing && desktop.workspace.draft);
    assert!(desktop.workspace.file_path.is_none() && desktop.workspace.destination.is_empty());
    assert!(desktop.workspace.editor.text().is_empty() && !desktop.workspace.saving);
    assert!(view.click(&desktop, "Save").is_empty(), "the blank draft cannot be saved");
    view.export(&desktop, "workspace-add-subagent-blank");
    // The test click on disabled Save can unfocus the editor.
    view.operate(&desktop, &mut focus);

    let definition = "name = \"reviewer\"\ndescription = \"Review the project\"\ndeveloper_instructions = \"Read the project instructions.\"";
    let mut clipboard = FixtureClipboard(definition.into());
    let paste = iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
        key: iced::keyboard::Key::Character("v".into()),
        modified_key: iced::keyboard::Key::Character("v".into()),
        physical_key: iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::KeyV),
        location: iced::keyboard::Location::Standard,
        modifiers: iced::keyboard::Modifiers::COMMAND,
        text: None,
        repeat: false,
    });
    let (messages, status) = view.event_with_clipboard(&desktop, paste, iced::Point::new(500.0, 400.0), &mut clipboard);
    assert_eq!(status, iced::event::Status::Captured);
    assert!(matches!(messages.as_slice(), [Message::Workspace(Event::Edit(text_editor::Action::Edit(text_editor::Edit::Paste(_))))]));
    for message in messages {
        assert_eq!(desktop.update(message).units(), 0, "pasting plain text only edits the draft");
    }
    assert_eq!(desktop.workspace.editor.text(), definition);
    assert!(!desktop.workspace.saving);
    assert!(!desktop.workspace.models_loading);

    event(&mut desktop, Event::DraftName("reviewer".into()));
    view.relayout(&desktop);
    let destination = format!("{agent_folder}/reviewer.toml");
    assert_eq!(desktop.workspace.destination, destination);
    assert!(view.survey(&desktop).text.iter().any(|(label, _)| label.contains(&destination)));
    let mut minimum = ViewFixture::new(&desktop, Size::new(760.0, 520.0)).await;
    let path = minimum.survey(&desktop).text.iter()
        .find(|(label, _)| label.contains(&destination)).unwrap().1;
    let scroll_to_save = iced::Event::Mouse(iced::mouse::Event::WheelScrolled {
        delta: iced::mouse::ScrollDelta::Pixels { x: 0.0, y: -650.0 },
    });
    minimum.event(&desktop, scroll_to_save.clone(), path.center());
    minimum.export(&desktop, "workspace-add-subagent-minimum-save");
    clicks(&mut desktop, &mut minimum, "Save");
    assert!(desktop.workspace.saving);

    let conflict = "A subagent with this filename already exists. Choose another name.";
    event(&mut desktop, Event::Saved(0, Err(conflict.into())));
    minimum.relayout(&desktop);
    let survey = minimum.survey(&desktop);
    let error = survey.text.iter().find(|(label, _)| label == "error!").unwrap().1;
    let save = survey.text.iter().find(|(label, _)| label == "Save").unwrap().1;
    let info = survey.text.iter().find(|(label, _)| label == "ⓘ").unwrap().1;
    assert!(error.x > save.x + save.width && (error.center_y() - save.center_y()).abs() < 2.0,
        "the conflict marker appears inline beside Save at minimum size");
    assert!(info.x > error.x + error.width && info.center_y() < 520.0);
    assert!(!survey.text.iter().any(|(label, _)| label.contains(conflict)),
        "the full error stays out of the form until the info icon is hovered");
    minimum.export(&desktop, "workspace-add-subagent-minimum-conflict");
    hover_error_details(&desktop, &mut minimum, info.center(), "workspace-add-subagent-save-error-hover");
    view.relayout(&desktop);
    assert!(!desktop.workspace.saving);
    assert!(desktop.workspace.agent_popup && desktop.workspace.draft);
    assert_eq!(desktop.workspace.editor.text(), definition);
    assert_eq!(desktop.workspace.destination, destination);
    assert!(view.survey(&desktop).text.iter().any(|(label, _)| label == "error!"));
    view.export(&desktop, "workspace-add-subagent-conflict");

    event(&mut desktop, Event::DraftName("project-reviewer".into()));
    view.relayout(&desktop);
    let renamed_path = view.survey(&desktop).text.iter()
        .find(|(label, _)| label.contains("project-reviewer.toml")).unwrap().1;
    super::scroll_gesture::new_test_gesture();
    view.event(&desktop, scroll_to_save, renamed_path.center());
    clicks(&mut desktop, &mut view, "Save");
    assert!(desktop.workspace.saving);
    event(&mut desktop, Event::Saved(0, Ok(json!({"saved":true}))));
    assert_eq!(desktop.workspace.file_path.as_deref(), Some("/fixture/project-a/.codex/agents/project-reviewer.toml"));
    assert!(!desktop.workspace.draft && !desktop.workspace.dirty);
}

#[tokio::test]
async fn subagent_model_picker_loads_on_edit_and_scrolls_after_four_rows() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.app = "codex".into();
    desktop.workspace.section = Section::Agents;
    let path = "/fixture/project-a/.codex/agents/reviewer.toml";
    let content = "name = 'reviewer'\ndescription = 'Review changes'\ndeveloper_instructions = 'Review the diff.'\n";
    event(&mut desktop, Event::Discovered(0, Ok(discovered(Some("/fixture/project-a"), json!([
        {"app":"codex","kind":"agent","name":"reviewer","file_path":path}
    ]), "/fixture/project-a/.codex/agents"))));
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 900.0)).await;
    clicks(&mut desktop, &mut view, "reviewer");
    event(&mut desktop, Event::File(0, path.into(), Ok(json!({"content":content}))));
    view.relayout(&desktop);
    assert!(!desktop.workspace.models_loading, "read-only preview has no model picker");
    let mut edit = view.click(&desktop, "Edit");
    assert!(matches!(edit.as_slice(), [Message::Workspace(Event::EditAgent)]));
    assert_eq!(desktop.update(edit.remove(0)).units(), 1, "Edit requests the model list without a Refresh click");
    assert!(desktop.workspace.models_loading);
    let models: Vec<_> = (1..=8).map(|i| json!({"slug":format!("model-{i}"),"display_name":format!("Fixture model {i}")})).collect();
    event(&mut desktop, Event::ModelList(0, "codex".into(), Ok(json!({"source":"codex_cache","models":models}))));
    view.relayout(&desktop);
    assert_eq!(desktop.workspace.models.len(), 8);
    assert!(toml::from_str::<toml::Value>(&desktop.workspace.editor.text()).unwrap().get("model").is_none(),
        "loading the model list does not choose or write a model");

    let refresh = view.survey(&desktop).text.iter().find(|(label, _)| label == "Refresh models").unwrap().1;
    let picker = iced::Point::new(refresh.x - 140.0, refresh.center_y());
    view.event(&desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), picker);
    let (menu, _) = view.overlay_event(&desktop, None);
    let line_height = iced::widget::text::LineHeight::default().to_absolute(iced::Pixels(13.0)).0;
    let row_height = line_height + 2.0 * super::theme::INPUT_PADDING;
    assert!((menu.height - 4.0 * row_height).abs() < 1.0, "{menu:?} shows exactly four native model rows");
    view.export_at(&desktop, "workspace-model-picker-four-rows", iced::mouse::Cursor::Available(menu.center()));
    let wheel = iced::Event::Mouse(iced::mouse::Event::WheelScrolled {
        delta: iced::mouse::ScrollDelta::Pixels { x: 0.0, y: -900.0 },
    });
    view.overlay_event(&desktop, Some((wheel, menu.center())));
    view.export_at(&desktop, "workspace-model-picker-scrolled", iced::mouse::Cursor::Available(menu.center()));
    let last = iced::Point::new(menu.center_x(), menu.y + menu.height - row_height / 2.0);
    view.overlay_event(&desktop, Some((iced::Event::Mouse(iced::mouse::Event::CursorMoved { position: last }), last)));
    let (_, messages) = view.overlay_event(&desktop, Some((iced::Event::Mouse(
        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), last)));
    assert!(matches!(messages.as_slice(), [Message::Workspace(Event::SelectModel(model))]
        if model.to_string() == "Fixture model 8"), "scroll reaches the last installed-model option");
}

#[tokio::test]
async fn bulk_subagent_toolbar_copies_the_selected_scope_and_pastes_without_overwriting() {
    use super::{client::NativeClient, workspace::bulk};
    use crate::{api::AppState, authority::PendingNativeAuthority, workspace::WorkspaceService};
    use reqwest::Method;
    use std::{fs, net::TcpListener, sync::Arc, time::Duration};

    let (directory, mut desktop) = desktop_fixture().await;
    let source = directory.path().join("source");
    let target = directory.path().join("target");
    let home = directory.path().join("fixture-home");
    let source_agents = source.join(".codex/agents");
    let target_agents = target.join(".codex/agents");
    fs::create_dir_all(source_agents.join("team")).unwrap();
    fs::create_dir_all(&target_agents).unwrap();
    fs::create_dir_all(source.join(".claude/agents")).unwrap();
    fs::create_dir_all(home.join(".codex/agents")).unwrap();
    let reviewer = "name = 'reviewer'\ndescription = 'Reviews changes'\ndeveloper_instructions = 'Review the diff.'\n";
    let planner = "name = 'planner'\ndescription = 'Plans changes'\nmodel = 'fixture-model'\ndeveloper_instructions = 'Write a plan.'\n";
    let original = "name = 'planner'\ndescription = 'Keep this existing file'\n";
    fs::write(source_agents.join("team/reviewer.toml"), reviewer).unwrap();
    fs::write(source_agents.join("planner.toml"), planner).unwrap();
    fs::write(target_agents.join("planner.toml"), original).unwrap();
    fs::write(source.join(".claude/agents/other-app.md"), "Other app").unwrap();
    fs::write(home.join(".codex/agents/global.toml"), "Other scope").unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let origin = format!("https://localhost:{}", listener.local_addr().unwrap().port());
    let rcgen::CertifiedKey { cert, signing_key } =
        rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let tls = axum_server::tls_rustls::RustlsConfig::from_der(
        vec![cert.der().to_vec()], signing_key.serialize_der(),
    ).await.unwrap();
    let mut router_state = AppState::new(
        origin.clone(), desktop.host.state.repository.clone(), desktop.host.state.discovery.clone(),
        Arc::new(PendingNativeAuthority),
    ).unwrap();
    router_state.native = Some(desktop.host.native.clone());
    router_state.workspace = Some(Arc::new(WorkspaceService::new(
        home.clone(), home.join(".codex"), home.join(".claude"),
    )));
    let routes = crate::api::router(Arc::new(router_state));
    let handle = axum_server::Handle::new();
    let api = tokio::spawn(
        axum_server::from_tcp_rustls(listener, tls).unwrap().handle(handle.clone()).serve(routes.into_make_service()),
    );
    tokio::time::timeout(Duration::from_secs(3), handle.listening()).await.unwrap().unwrap();
    let client = NativeClient::new(origin, &cert.pem(), desktop.session.clone().unwrap()).unwrap();
    desktop.client = Some(client.clone());
    desktop.page = Page::Workspace;
    desktop.workspace.section = Section::Agents;
    desktop.workspace.app = "codex".into();

    let discover = |client: NativeClient, project: &std::path::Path| {
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("folder_path", project.to_str().unwrap()).finish();
        async move { client.request(Method::GET, &format!("/api/v1/workspace?{query}"), None).await }
    };
    let copy_sources = |desktop: &Desktop| {
        desktop.workspace.agents().map(|row| (
            row["file_path"].as_str().unwrap().to_owned(),
            row["name"].as_str().map(str::to_owned),
        )).collect()
    };
    event(&mut desktop, Event::Discovered(0, discover(client.clone(), &source).await));
    let mut view = ViewFixture::new(&desktop, Size::new(760.0, 520.0)).await;
    let survey = view.survey(&desktop);
    let add = survey.text.iter().find(|(label, _)| label == "Add subagent").unwrap().1;
    let mut bulk_icons: Vec<_> = survey.containers.iter().filter(|rect| {
        (rect.width - 40.0).abs() < 0.1 && (rect.height - 40.0).abs() < 0.1
            && (rect.center_y() - add.center_y()).abs() < 2.0 && rect.x + rect.width < add.x
    }).map(|rect| rect.center()).collect();
    bulk_icons.sort_by(|a,b| a.x.total_cmp(&b.x));
    bulk_icons.dedup();
    assert_eq!(bulk_icons.len(), 2, "two icon buttons precede Add subagent at minimum size");
    let [paste, copy] = bulk_icons.try_into().unwrap();
    let light_button_face = |frame: &[u8], point: iced::Point| {
        let index = ((point.y as usize * 760) + (point.x as usize + 13)) * 4;
        assert!(frame[index..index + 3].iter().all(|&channel| channel > 220),
            "clipboard button face stays light at {point:?}: {:?}", &frame[index..index + 3]);
    };
    let click = |view: &mut ViewFixture, desktop: &mut Desktop, point| {
        let down = iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left));
        let up = iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left));
        let cursor = iced::mouse::Cursor::Available(point);
        light_button_face(&view.pixels_at(desktop, cursor, std::time::Instant::now()), point);
        let (mut messages, status) = view.event_with_status(desktop, down, point);
        assert_eq!(status, iced::event::Status::Captured);
        assert!(!messages.iter().any(|message| matches!(message, Message::DragWindow)));
        light_button_face(&view.pixels_at(desktop, cursor, std::time::Instant::now()), point);
        messages.extend(view.event(desktop, up, point));
        messages.into_iter().find_map(|message| match message {
            Message::Workspace(event @ (Event::CopyAllAgents | Event::PasteAllAgents)) => Some(event),
            _ => None,
        }).expect("toolbar control published a bulk action")
    };
    assert!(matches!(click(&mut view, &mut desktop, copy), Event::CopyAllAgents));
    let copying = desktop.update(Message::Workspace(Event::CopyAllAgents));
    assert!(copying.units() > 0 && desktop.workspace.bulk_busy);
    drop(copying);
    view.relayout(&desktop);
    let (messages, status) = view.event_with_status(&desktop, iced::Event::Mouse(
        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), copy);
    assert_eq!(status, iced::event::Status::Captured, "the busy icon still owns its press");
    assert!(messages.is_empty(), "pressing a busy icon neither restarts it nor drags the window");
    let cursor = iced::mouse::Cursor::Available(copy);
    light_button_face(&view.pixels_at(&desktop, cursor, std::time::Instant::now()), copy);
    assert!(view.event(&desktop, iced::Event::Mouse(
        iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), copy).is_empty());
    assert!(!view.survey(&desktop).text.iter().any(|(text, _)| text == "Copied all!"));
    let copied = bulk::copy(
        client.clone(), desktop.workspace.folder_path.clone(), desktop.workspace.app.clone(),
        desktop.workspace.agent_directory.clone(), copy_sources(&desktop),
    ).await.unwrap();
    let bundle: Value = serde_json::from_str(&copied).unwrap();
    assert_eq!(bundle["app"], "codex");
    assert_eq!(bundle["agents"].as_array().unwrap().len(), 2, "other app and global files are excluded");
    assert!(bundle["agents"].as_array().unwrap().iter().any(|entry| {
        entry["relative_path"] == "team/reviewer.toml" && entry["content"] == reviewer
    }));
    assert!(!copied.contains(source.to_str().unwrap()), "the bundle carries no source folder");
    let write_clipboard = desktop.update(Message::Workspace(Event::CopiedAllAgents(0, Ok(copied.clone()))));
    assert!(write_clipboard.units() > 0);
    drop(write_clipboard);
    assert!(!desktop.workspace.bulk_busy);
    view.relayout(&desktop);
    let survey = view.survey(&desktop);
    let status = survey.text.iter().find(|(label, _)| label == "Copied all!").unwrap().1;
    let sessions_tab = survey.text.iter().find(|(label, _)| label == "Sessions").unwrap().1;
    assert!(status.x >= sessions_tab.x + sessions_tab.width && status.x + status.width < paste.x,
        "the copy confirmation fits between the tabs and clipboard icons");
    view.export(&desktop, "workspace-bulk-toolbar-minimum");

    event(&mut desktop, Event::FolderChosen(0, Ok(Some(target.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &target).await));
    view.relayout(&desktop);
    assert!(matches!(click(&mut view, &mut desktop, paste), Event::PasteAllAgents));
    let read_clipboard = desktop.update(Message::Workspace(Event::PasteAllAgents));
    assert!(read_clipboard.units() > 0 && desktop.workspace.bulk_busy);
    drop(read_clipboard);
    let saving = desktop.update(Message::Workspace(Event::BulkAgentsClipboard(generation, Some(copied.clone()))));
    assert!(saving.units() > 0, "clipboard delivery immediately queues a save");
    drop(saving);
    assert!(!desktop.workspace.agent_popup, "bulk paste needs no draft or additional Save");
    let prepared = bulk::prepare(client.clone(), copied.clone(), desktop.workspace.app.clone(), desktop.workspace.agent_directory.clone()).await.unwrap();
    let saved = bulk::save(client.clone(), desktop.workspace.folder_path.clone(), prepared).await;
    assert_eq!(saved.iter().filter(|entry| entry.result.is_ok()).count(), 1);
    assert_eq!(fs::read_to_string(target_agents.join("planner.toml")).unwrap(), original);
    assert_eq!(fs::read_to_string(target_agents.join("team/reviewer.toml")).unwrap(), reviewer);
    let refresh = desktop.update(Message::Workspace(Event::BulkAgentsSaved(generation, Ok(saved))));
    assert!(refresh.units() > 0, "partial success refreshes the selected list");
    drop(refresh);
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &target).await));
    view.relayout(&desktop);
    let survey = view.survey(&desktop);
    assert!(survey.text.iter().any(|(label, _)| label == "reviewer"));
    assert!(!survey.text.iter().any(|(label, _)| label == "Pasted all!"
        || label.contains("Added 1 of 2") || label.contains("already exists")));
    let error = survey.text.iter().find(|(label, _)| label == "error!").unwrap().1;
    let info = survey.text.iter().find(|(label, _)| label == "ⓘ").unwrap().1;
    assert!(error.x + error.width < info.x && info.x + info.width < paste.x,
        "the partial result stays beside the clipboard icons with an info glyph");
    assert!(desktop.workspace.error.is_none(), "bulk results stay out of the list's standard error area");
    view.export(&desktop, "workspace-bulk-partial-result-minimum");
    hover_error_details(&desktop, &mut view, info.center(), "workspace-bulk-partial-details-minimum");

    pick_app(&mut desktop, &mut view, "claude");
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &target).await));
    view.relayout(&desktop);
    let prepared = bulk::prepare(client.clone(), copied.clone(), desktop.workspace.app.clone(), desktop.workspace.agent_directory.clone()).await.unwrap();
    let saved = bulk::save(client.clone(), desktop.workspace.folder_path.clone(), prepared).await;
    let claude_agents = target.join(".claude/agents");
    let converted = fs::read_to_string(claude_agents.join("team/reviewer.md")).unwrap();
    assert!(converted.starts_with("---\n") && converted.contains("Review the diff."));
    assert!(!claude_agents.join("planner.md").exists());
    assert!(saved.iter().any(|entry| entry.destination.ends_with("planner.md")
        && entry.result.as_ref().is_err_and(|error| error.contains("review"))));

    // Paste the original bundle into a fresh project of the same app, where
    // neither name conflicts. A full success uses the requested toolbar text.
    let empty_target = directory.path().join("empty-target");
    fs::create_dir_all(&empty_target).unwrap();
    pick_app(&mut desktop, &mut view, "codex");
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::FolderChosen(generation, Ok(Some(empty_target.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &empty_target).await));
    view.relayout(&desktop);
    let paste_all = click(&mut view, &mut desktop, paste);
    event(&mut desktop, paste_all);
    event(&mut desktop, Event::BulkAgentsClipboard(generation, Some(copied.clone())));
    assert!(!view.survey(&desktop).text.iter().any(|(label, _)| label == "Pasted all!"));
    let prepared = bulk::prepare(client.clone(), copied, desktop.workspace.app.clone(), desktop.workspace.agent_directory.clone()).await.unwrap();
    let saved = bulk::save(client.clone(), desktop.workspace.folder_path.clone(), prepared).await;
    assert!(saved.iter().all(|entry| entry.result.is_ok()));
    let refresh = desktop.update(Message::Workspace(Event::BulkAgentsSaved(generation, Ok(saved))));
    assert!(refresh.units() > 0);
    drop(refresh);
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &empty_target).await));
    view.relayout(&desktop);
    let survey = view.survey(&desktop);
    let status = survey.text.iter().find(|(label, _)| label == "Pasted all!").unwrap().1;
    assert!(status.x + status.width < paste.x && desktop.workspace.error.is_none());
    assert!(survey.text.iter().any(|(label, _)| label == "reviewer"));
    view.export(&desktop, "workspace-bulk-pasted-all-minimum");

    // The row clipboard control deliberately replaces just its selected
    // definition. Any remaining names still use create-only writes.
    let row_target = directory.path().join("row-target");
    let row_agents = row_target.join(".codex/agents");
    let selected = row_agents.join("selected.toml");
    let conflict = row_agents.join("planner.toml");
    fs::create_dir_all(&row_agents).unwrap();
    fs::write(&selected, "name = 'selected'\ndescription = 'Replaced by the first clipboard item'\n").unwrap();
    fs::write(&conflict, original).unwrap();
    let row_config = row_target.join(".codex/config.toml");
    let registration = "[agents.selected]\nconfig_file = 'agents/selected.toml'\ndescription = 'Selected definition'\n";
    fs::write(&row_config, registration).unwrap();
    event(&mut desktop, Event::FolderChosen(generation, Ok(Some(row_target.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &row_target).await));
    let selected = selected.canonicalize().unwrap().to_str().unwrap().to_owned();
    let mut card_view = ViewFixture::new(&desktop, Size::new(874.0, 900.0)).await;
    let survey = card_view.survey(&desktop);
    let label = survey.text.iter().find(|(text, _)| text == "selected").unwrap().1;
    let mut card_actions: Vec<_> = survey.containers.iter().filter(|rect| {
        (rect.width - 40.0).abs() < 0.1 && (rect.height - 40.0).abs() < 0.1
            && (rect.center_y() - label.center_y()).abs() < 2.0 && rect.x > label.x + label.width
    }).map(|rect| rect.center()).collect();
    card_actions.sort_by(|a, b| a.x.total_cmp(&b.x));
    card_actions.dedup();
    assert_eq!(card_actions.len(), 3, "the row has three equal-sized Copy, Paste and Delete controls");
    let click_row_action = |view: &mut ViewFixture, desktop: &Desktop, point| {
        let mut events = view.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), point);
        events.extend(view.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), point));
        events
    };
    let mut events = click_row_action(&mut card_view, &desktop, card_actions[2]);
    assert!(matches!(events.as_slice(), [Message::Workspace(Event::RequestDeleteAgent(path))] if path == &selected));
    assert_eq!(desktop.update(events.remove(0)).units(), 0, "the card's X only opens confirmation");
    card_view.relayout(&desktop);
    assert!(card_view.survey(&desktop).text.iter().any(|(label, _)| label == "Delete selected?"));
    card_view.export(&desktop, "workspace-row-delete-confirmation");
    clicks(&mut desktop, &mut card_view, "Cancel");
    assert!(std::path::Path::new(&selected).exists(), "cancel leaves the selected file alone");
    let mut events = click_row_action(&mut card_view, &desktop, card_actions[1]);
    assert!(matches!(events.as_slice(), [Message::Workspace(Event::PasteOnAgent(path))] if path == &selected));
    assert_eq!(desktop.update(events.remove(0)).units(), 1, "row Paste reads the clipboard immediately");
    let remainder = "name = 'helper'\ndescription = 'Helps review changes'\n";
    let clipboard = json!({"fetch_agents":1,"app":"codex","agents":[
        {"relative_path":"team/reviewer.toml","content":reviewer},
        {"relative_path":"planner.toml","content":planner},
        {"relative_path":"team/helper.toml","content":remainder}
    ]}).to_string();
    assert_eq!(desktop.update(Message::Workspace(Event::RowAgentsClipboard(generation, selected.clone(), Some(clipboard.clone())))).units(), 1);
    assert!(!desktop.workspace.agent_popup, "row Paste does not leave an editor draft to save");
    let saved = bulk::paste_on_agent(
        client.clone(), desktop.workspace.folder_path.clone(), clipboard, "codex".into(),
        desktop.workspace.agent_directory.clone(), selected.clone(),
    ).await.unwrap();
    assert_eq!(saved[0].destination, selected);
    assert!(saved[0].result.is_ok());
    assert!(saved[1].result.as_ref().is_err_and(|error| error.contains("already exists")));
    assert!(saved[2].result.is_ok());
    assert_eq!(fs::read_to_string(&selected).unwrap(), reviewer);
    assert_eq!(fs::read_to_string(conflict).unwrap(), original);
    assert_eq!(fs::read_to_string(row_agents.join("team/helper.toml")).unwrap(), remainder);
    assert!(!row_agents.join("team/reviewer.toml").exists(), "the first clipboard item only replaces the selected destination");
    assert_eq!(desktop.update(Message::Workspace(Event::RowAgentsSaved(generation, selected.clone(), Ok(saved)))).units(), 1);
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &row_target).await));
    card_view.relayout(&desktop);
    assert!(card_view.survey(&desktop).text.iter().any(|(label, _)| label == "helper"));
    card_view.export(&desktop, "workspace-row-paste-result");

    let mut events = click_row_action(&mut card_view, &desktop, card_actions[2]);
    assert!(matches!(events.as_slice(), [Message::Workspace(Event::RequestDeleteAgent(path))] if path == &selected));
    drop(desktop.update(events.remove(0)));
    card_view.relayout(&desktop);
    let mut confirmation = card_view.click(&desktop, "Delete");
    assert!(matches!(confirmation.as_slice(), [Message::Workspace(Event::ConfirmDeleteAgent)]));
    assert_eq!(desktop.update(confirmation.remove(0)).units(), 1, "only confirmation issues Delete");
    let query = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("folder_path", row_target.to_str().unwrap())
        .append_pair("file_path", &selected).finish();
    let deleted = client.request(Method::DELETE, &format!("/api/v1/workspace/file?{query}"), None).await.unwrap();
    assert_eq!(deleted["deleted"], true);
    assert!(!std::path::Path::new(&selected).exists());
    assert_eq!(fs::read_to_string(&row_config).unwrap(), registration);
    assert_eq!(fs::read_to_string(row_agents.join("planner.toml")).unwrap(), original);
    assert_eq!(desktop.update(Message::Workspace(Event::AgentDeleted(generation, selected, Ok(deleted)))).units(), 1);
    card_view.relayout(&desktop);
    assert!(!desktop.workspace.delete_agent_visible());
    assert!(!card_view.survey(&desktop).text.iter().any(|(label, _)| label == "selected"));

    let refused = ["coder", "evaluator", "planner", "researcher", "tester"].into_iter().map(|role| {
        bulk::SaveResult {
            destination: format!("/fixture/project/.claude/agents/{role}.md"),
            result: Err("Copy and paste this subagent individually to review its settings.\nProvide the destination agent's name.\nProvide the destination agent's description.".into()),
        }
    }).collect();
    event(&mut desktop, Event::BulkAgentsSaved(generation, Ok(refused)));
    view.relayout(&desktop);
    let survey = view.survey(&desktop);
    assert!(!survey.text.iter().any(|(label, _)| label.contains("Added 0 of 5") || label.contains("tester.md")));
    let info = survey.text.iter().find(|(label, _)| label == "ⓘ").unwrap().1;
    hover_error_details(&desktop, &mut view, info.center(), "workspace-bulk-five-errors-details-minimum");

    // An older Codex role can take its card name from its registration,
    // while the source definition leaves the name out of the file.
    let legacy_path = source.join(".codex/roles/legacy-role.toml");
    let legacy_content = "sandbox_mode = 'read-only'\ndeveloper_instructions = 'Make the requested change.'\n";
    fs::create_dir_all(legacy_path.parent().unwrap()).unwrap();
    fs::write(&legacy_path, legacy_content).unwrap();
    let auditor_file = source.join(".codex/roles/review.v2.toml");
    std::os::unix::fs::symlink(source_agents.join("team/reviewer.toml"), &auditor_file).unwrap();
    let source_config = source.join(".codex/config.toml");
    let auditor_registration = "\n[agents.auditor]\nconfig_file = 'roles/review.v2.toml'\n";
    fs::write(&source_config, format!("[agents.coder]\nconfig_file = 'roles/legacy-role.toml'\ndescription = 'Older registration'\n{auditor_registration}")).unwrap();
    event(&mut desktop, Event::FolderChosen(generation, Ok(Some(source.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &source).await));
    view.relayout(&desktop);
    assert!(view.survey(&desktop).text.iter().any(|(label, _)| label == "coder"));
    let updated_registration = format!("[agents.coder]\nconfig_file = 'roles/legacy-role.toml'\ndescription = 'Implements the plan'\n{auditor_registration}");
    fs::write(&source_config, &updated_registration).unwrap();
    let copied = bulk::copy(
        client.clone(), desktop.workspace.folder_path.clone(), desktop.workspace.app.clone(),
        desktop.workspace.agent_directory.clone(), copy_sources(&desktop),
    ).await.unwrap();
    let bundle: Value = serde_json::from_str(&copied).unwrap();
    assert!(bundle["agents"].as_array().unwrap().iter().any(|entry| {
        entry["relative_path"] == "legacy-role.toml" && entry["name"] == "coder" && entry["description"] == "Implements the plan"
            && entry["content"] == legacy_content
    }));

    let auditor = desktop.workspace.agents().find(|agent| agent["name"] == "auditor").unwrap();
    let card_path = auditor["file_path"].as_str().unwrap();
    let file_url = format!("/api/v1/workspace/file?{}", url::form_urlencoded::Serializer::new(String::new())
        .append_pair("folder_path", source.to_str().unwrap())
        .append_pair("file_path", card_path).finish());
    let file = client.request(Method::GET, &file_url, None).await.unwrap();
    assert_ne!(file["file_path"], card_path, "the file read follows the card's alias");
    let single = bulk::copied_agent_payload("codex", auditor["name"].as_str(), card_path, &file, &desktop.workspace.agent_directory);
    let single_target = directory.path().join("single-target");
    fs::create_dir(&single_target).unwrap();
    event(&mut desktop, Event::FolderChosen(generation, Ok(Some(single_target.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &single_target).await));
    view.relayout(&desktop);
    let paste_single = click(&mut view, &mut desktop, paste);
    assert_eq!(desktop.update(Message::Workspace(paste_single)).units(), 1);
    assert_eq!(desktop.update(Message::Workspace(Event::BulkAgentsClipboard(generation, Some(single.clone())))).units(), 1);
    assert!(!desktop.workspace.agent_popup, "toolbar Paste queues the single save without opening a draft");
    let prepared = bulk::prepare(client.clone(), single, desktop.workspace.app.clone(), desktop.workspace.agent_directory.clone()).await.unwrap();
    let saved = bulk::save(client.clone(), desktop.workspace.folder_path.clone(), prepared).await;
    assert_eq!(saved.len(), 1);
    assert!(saved[0].result.is_ok(), "{:?}", saved[0].result);
    let agent_dir = single_target.join(".codex/agents");
    assert_eq!(fs::read_dir(&agent_dir).unwrap().count(), 1);
    assert_eq!(fs::read_to_string(agent_dir.join("review.v2.toml")).unwrap(), reviewer);
    event(&mut desktop, Event::BulkAgentsSaved(generation, Ok(saved)));

    pick_app(&mut desktop, &mut view, "claude");
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::FolderChosen(generation, Ok(Some(empty_target.to_str().unwrap().into()))));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, discover(client.clone(), &empty_target).await));
    let prepared = bulk::prepare(client.clone(), copied, desktop.workspace.app.clone(), desktop.workspace.agent_directory.clone()).await.unwrap();
    let saved = bulk::save(client, desktop.workspace.folder_path.clone(), prepared).await;
    assert!(saved.iter().any(|entry| entry.destination.ends_with("legacy-role.md") && entry.result.is_ok()));
    let converted = fs::read_to_string(empty_target.join(".claude/agents/legacy-role.md")).unwrap();
    let preview = crate::workspace::agent_definition_preview("claude", &converted).unwrap();
    assert_eq!(preview["fields"]["name"], "coder");
    assert_eq!(preview["fields"]["description"], "Implements the plan");
    assert_eq!(preview["fields"]["permissionMode"], "plan");
    assert_eq!(fs::read_to_string(&legacy_path).unwrap(), legacy_content);
    assert_eq!(fs::read_to_string(&source_config).unwrap(), updated_registration);

    handle.shutdown();
    api.await.unwrap().unwrap();
}

#[tokio::test]
async fn workspace_folder_and_app_controls_keep_global_and_project_views_separate() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    event(&mut desktop, Event::Discovered(0, Ok(discovered(None, json!([
        {"name":"Global rules","kind":"instructions","app":"codex","file_path":"/fixture-home/.codex/AGENTS.md"}
    ]), "/fixture-home/.codex/agents"))));
    finish_names(&mut desktop, json!([]));
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 630.0)).await;
    view.export(&desktop, "workspace-global");

    clicks(&mut desktop, &mut view, "Choose folder…");
    let current=desktop.workspace.generation;
    event(&mut desktop, Event::FolderChosen(current,Ok(Some("/fixture/project-a".into()))));
    view.relayout(&desktop);
    assert_eq!(desktop.workspace.folder_path.as_deref(), Some("/fixture/project-a"));
    assert!(desktop.workspace.files.is_empty());
    let generation = desktop.workspace.generation;
    // An already-started global read must not reappear after selecting a project.
    event(&mut desktop, Event::Discovered(generation - 1, Ok(discovered(None, json!([{"name":"Old global reply"}]), ""))));
    assert!(desktop.workspace.files.is_empty());
    event(&mut desktop, Event::Discovered(generation, Ok(discovered(Some("/fixture/project-a"), json!([
        {"name":"Project rules","kind":"instructions","app":"shared","file_path":"/fixture/project-a/AGENTS.md"}
    ]), "/fixture/project-a/.codex/agents"))));
    finish_names(&mut desktop, json!([]));
    view.relayout(&desktop);
    view.export(&desktop, "workspace-project");

    clicks(&mut desktop, &mut view, "Sessions");
    assert_eq!(desktop.workspace.section, Section::Sessions);
    pick_app(&mut desktop,&mut view,"codex");
    assert_eq!(desktop.workspace.app, "codex");
    assert_eq!(desktop.workspace.folder_path.as_deref(), Some("/fixture/project-a"));
    assert!(desktop.workspace.sessions.is_empty());
    event(&mut desktop,Event::FolderInput(String::new()));
    event(&mut desktop,Event::OpenFolder);
    view.relayout(&desktop);
    assert!(desktop.workspace.folder_path.is_none());
    assert!(desktop.workspace.folder_input.is_empty());
    assert!(desktop.workspace.files.is_empty());
    assert!(desktop.workspace.logs.is_empty());
    let mut small = ViewFixture::new(&desktop, Size::new(760.0, 520.0)).await;
    small.export(&desktop, "workspace-global-minimum");
}

#[tokio::test]
async fn instructions_paste_button_edits_the_current_codex_or_claude_draft() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.app = "codex".into();
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 900.0)).await;
    let original = "Replace this heading\nKeep this instruction";
    let pasted = "# Pasted heading\nKeep this instruction";
    let codex_path = "/fixture-home/.codex/AGENTS.md";
    let initial_generation = desktop.workspace.generation;
    let click_at = |view: &mut ViewFixture, desktop: &Desktop, point| {
        [iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left),
            iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)]
            .into_iter().flat_map(|mouse| view.event(desktop, iced::Event::Mouse(mouse), point))
            .collect::<Vec<_>>()
    };

    for (app, path) in [("codex", codex_path), ("claude", "/fixture-home/.claude/CLAUDE.md")] {
        if app == "claude" { pick_app(&mut desktop, &mut view, app); }
        let generation = desktop.workspace.generation;
        event(&mut desktop, Event::Discovered(generation, Ok(discovered(None, json!([
            {"name":"Fixture instructions","kind":"instructions","app":app,"file_path":path}
        ]), "/fixture-home/.codex/agents"))));
        event(&mut desktop, Event::File(generation, path.into(), Ok(json!({"content":original}))));
        if app == "claude" {
            event(&mut desktop, Event::InstructionsClipboard(initial_generation, codex_path.into(), Some("Late Codex paste".into())));
            assert_eq!(desktop.workspace.editor.text(), original, "a late clipboard reply leaves the new app's editor alone");
        }
        view.relayout(&desktop);

        let survey = view.survey(&desktop);
        let title = survey.text.iter().find(|(label, _)| label == "Edit file").unwrap().1;
        let mut buttons: Vec<_> = survey.containers.iter().filter(|rect| {
            (rect.width - 40.0).abs() < 0.1 && (rect.height - 40.0).abs() < 0.1
                && rect.x > title.x && (rect.center_y() - title.center_y()).abs() < 2.0
        }).copied().collect();
        buttons.sort_by(|a, b| a.x.total_cmp(&b.x));
        buttons.dedup_by(|a, b| (a.x - b.x).abs() < 0.1);
        assert_eq!(buttons.len(), 2, "the instructions header has Paste and Copy");
        event(&mut desktop, Event::Edit(text_editor::Action::Move(text_editor::Motion::DocumentStart)));
        event(&mut desktop, Event::Edit(text_editor::Action::Select(text_editor::Motion::End)));
        view.relayout(&desktop);
        let mut messages = click_at(&mut view, &desktop, buttons[0].center());
        assert!(matches!(messages.as_slice(), [Message::Workspace(Event::PasteInstructions)]));
        let read = desktop.update(messages.remove(0));
        assert_eq!(read.units(), 1, "Paste schedules one clipboard read");
        drop(read);
        assert_eq!(desktop.workspace.editor.text(), original);

        event(&mut desktop, Event::InstructionsClipboard(generation, path.into(), Some("# Pasted heading".into())));
        assert_eq!(desktop.workspace.editor.text(), pasted);
        assert!(desktop.workspace.dirty && !desktop.workspace.saving, "pasted content stays in the draft until Save");
        assert_eq!(desktop.workspace.destination, path);
        view.relayout(&desktop);
        assert!(matches!(click_at(&mut view, &desktop, buttons[1].center()).as_slice(), [Message::Copy(text)] if text == pasted));
        view.export(&desktop, &format!("workspace-{app}-instructions-paste"));

        if app == "codex" {
            event(&mut desktop, Event::Undo);
            assert_eq!(desktop.workspace.editor.text(), original);
            assert!(!desktop.workspace.dirty, "one Undo reverts the entire paste");
        } else {
            let mut messages = view.click(&desktop, "Save");
            assert!(matches!(messages.as_slice(), [Message::Workspace(Event::Save)]));
            let save = desktop.update(messages.remove(0));
            assert_eq!(save.units(), 1, "Save schedules one write to the open instructions file");
            assert!(desktop.workspace.saving);
            assert_eq!(desktop.workspace.destination, path);
            drop(save);
            event(&mut desktop, Event::Saved(generation, Ok(json!({"saved":true}))));
            assert!(!desktop.workspace.dirty && !desktop.workspace.saving);
        }
        view.relayout(&desktop);
    }
}

#[tokio::test]
async fn session_list_failure_stays_in_sessions_while_codex_instructions_remain_editable() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.app = "codex".into();
    let path = "/fixture-home/.codex/AGENTS.md";
    let missing_rollout = "native selected rollout is missing: No such file or directory (os error 2)";
    event(&mut desktop, Event::Discovered(0, Ok(discovered(None, json!([
        {"name":"Global rules","kind":"instructions","app":"codex","file_path":path}
    ]), "/fixture-home/.codex/agents"))));
    event(&mut desktop, Event::File(0, path.into(), Ok(json!({"content":"Original instructions"}))));
    // History finishes after the instruction read, matching the reported failure.
    event(&mut desktop, Event::Sessions(0, Err(missing_rollout.into())));
    // Keep the full editor and Save control inside the scroll viewport.
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 900.0)).await;
    assert_eq!(desktop.workspace.file_path.as_deref(), Some(path));
    assert!(!view.survey(&desktop).text.iter().any(|(text, _)| text == missing_rollout));
    event(&mut desktop, Event::Edit(text_editor::Action::SelectAll));
    event(&mut desktop, Event::Edit(text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new("Updated instructions".into())))));
    view.relayout(&desktop);
    assert_eq!(desktop.workspace.editor.text(), "Updated instructions");
    assert!(desktop.workspace.dirty);
    assert!(view.click(&desktop, "Save").iter().any(|message| matches!(message, Message::Workspace(Event::Save))), "the real Save control remains available");
    let path_row=view.survey(&desktop).text.iter().find(|(text,_)|text==path).unwrap().1;
    let survey=view.survey(&desktop);
    let mut path_controls:Vec<_>=survey.containers.iter().filter(|rect|(rect.width-28.0).abs()<0.1&&(rect.height-28.0).abs()<0.1&&(rect.center_y()-path_row.center_y()).abs()<2.0).map(|rect|rect.center()).collect();
    path_controls.sort_by(|a,b|a.x.total_cmp(&b.x));path_controls.dedup_by(|a,b|(a.x-b.x).abs()<0.1&&(a.y-b.y).abs()<0.1);
    assert_eq!(path_controls.len(),2,"instruction path has compact Copy and adjacent Finder controls");
    let mut messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),path_controls[0]);
    messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),path_controls[0]));
    assert!(messages.iter().any(|message|matches!(message,Message::Copy(value) if value==path)));
    view.export(&desktop, "codex-instructions-with-unrelated-history-failure");

    clicks(&mut desktop, &mut view, "Subagents");
    let empty_agents=view.survey(&desktop);
    assert!(!empty_agents.text.iter().any(|(text, _)| text == missing_rollout));
    let empty_message=empty_agents.text.iter().find(|(text,_)|text=="No subagent definitions in this scope.").unwrap().1;
    let tab=empty_agents.text.iter().find(|(text,_)|text=="Subagents").unwrap().1;
    let panel=empty_agents.containers.iter().find(|bounds|bounds.y>tab.y+tab.height&&bounds.width>700.0&&bounds.height>300.0&&bounds.contains(empty_message.center())).unwrap();
    assert!((empty_message.center_x()-panel.center_x()).abs()<1.0&&(empty_message.center_y()-panel.center_y()).abs()<1.0,
        "the empty Subagents message is centered in its recessed panel: {empty_message:?}, {panel:?}");
    view.export(&desktop,"empty-subagents");
    clicks(&mut desktop, &mut view, "Sessions");
    assert!(view.survey(&desktop).text.iter().any(|(text, _)| text == missing_rollout));
    view.export(&desktop, "codex-sessions-with-original-history-error");

    event(&mut desktop, Event::Refresh);
    event(&mut desktop, Event::Discovered(0, Ok(discovered(None, json!([
        {"name":"Global rules","kind":"instructions","app":"codex","file_path":path}
    ]), "/fixture-home/.codex/agents"))));
    finish_names(&mut desktop, json!([{"id":"recovered","name":"Recovered session","app":"codex","file_path":"/fixture/session.jsonl"}]));
    view.relayout(&desktop);
    let survey = view.survey(&desktop);
    assert!(!survey.text.iter().any(|(text, _)| text == missing_rollout));
    assert!(survey.text.iter().any(|(text, _)| text == "Recovered session"));
    clicks(&mut desktop, &mut view, "Instructions");
    assert_eq!(desktop.workspace.editor.text(), "Updated instructions", "history reload preserves unsaved instruction edits");
}

#[tokio::test]
async fn session_cards_show_complete_conversations_and_keep_them_when_closed() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.section = Section::Sessions;
    finish_names(&mut desktop, json!([{"id":"fixture-session","name":"Review feature work","app":"claude","file_path":"/fixture/log.jsonl","project_path":"/fixture/project-a"}]));
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 750.0)).await;
    view.export(&desktop,"workspace-session-blue-card");
    clicks(&mut desktop, &mut view, "Review feature work");
    assert_eq!(desktop.workspace.active_session.as_deref(),Some("fixture-session"));
    assert!(desktop.workspace.logs["fixture-session"].loading);
    let full_reply=format!("{}The change looks good.","Detailed review paragraph.\n\n".repeat(1600));
    event(&mut desktop, Event::ConversationLoaded(0,"fixture-session".into(),Ok(json!({"complete":true,"messages":[
        {"id":"u1","kind":"user","text":"Please review this change."},
        {"id":"a1","kind":"assistant","text":full_reply},
        {"id":"a2","kind":"assistant","text":"One more detail."}
    ]}))));
    assert!(!desktop.workspace.logs["fixture-session"].loading);
    view.relayout(&desktop);
    let survey=view.survey(&desktop);
    for label in ["User","Assistant"]{assert!(survey.text.iter().any(|(text,_)|text==label));}
    assert!(!survey.text.iter().any(|(text,_)|text.contains("<command-name>")||text.contains("\"type\"")));
    let assistant_labels:Vec<_>=survey.text.iter().filter(|(text,_)|text=="Assistant").collect();
    assert!(assistant_labels[1].1.y-assistant_labels[0].1.y>1000.0,"all long-message paragraphs remain inside one full-height message card before the following reply");
    assert_eq!(desktop.workspace.logs["fixture-session"].history.turns().find(|turn|turn.role=="Assistant").unwrap().text,full_reply);
    view.export(&desktop, "workspace-session-command-popup");
    let point=iced::Point::new(400.0,400.0);super::scroll_gesture::new_test_gesture();
    view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-100000.0}}),point);
    view.export(&desktop,"workspace-session-complete-message-tail");
    assert!(!view.survey(&desktop).text.iter().any(|(text,_)|text=="Read more"));
    // The shared modal closes on a click outside the dialog.
    let point=iced::Point::new(6.0,6.0);
    let messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),point);
    assert!(messages.iter().any(|message|matches!(message,Message::Workspace(Event::CloseSession))));
    for message in messages{drop(desktop.update(message));}
    view.relayout(&desktop);
    assert!(desktop.workspace.active_session.is_none());
    clicks(&mut desktop, &mut view, "Review feature work");
    let log=&desktop.workspace.logs["fixture-session"];
    assert!(!log.loading && log.eof);
    assert_eq!(log.history.assistant_count(),2);

}

#[tokio::test]
async fn session_markdown_uses_white_user_and_gray_assistant_cards_without_changing_history(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;desktop.workspace.section=Section::Sessions;
    finish_names(&mut desktop,json!([{"id":"markdown","name":"Formatted session","app":"codex","file_path":"/fixture/markdown.jsonl"}]));
    event(&mut desktop,Event::OpenSession("markdown".into()));
    let user="Please review `src/main.rs` before merging.";
    let code=format!("let ready = true;\nlet request_id = \"{}\";","0123456789abcdef".repeat(22));
    let assistant=format!("**Result:** ready.\n\nUse `safe_mode` with existing files.\n\n- First item\n- Second item\n\n> Keep the original files.\n\n```rust\n{code}\n```");
    event(&mut desktop,Event::ConversationLoaded(0,"markdown".into(),Ok(json!({"complete":true,"messages":[
        {"id":"u","kind":"user","text":user},
        {"id":"a","kind":"assistant","text":assistant}
    ]}))));
    let normal_size=super::window_settings().size;
    let normal_width=normal_size.width as usize;
    let mut view=ViewFixture::new(&desktop,normal_size).await;
    let survey=view.survey(&desktop);
    assert!(!survey.text.iter().any(|(text,_)|text==user||text==&assistant),"Markdown is rendered instead of showing the raw source as one text widget");
    let pixels=view.pixels(&desktop);
    let sample=|role:&str|{
        let bounds=survey.text.iter().find(|(text,_)|text==role).unwrap().1;
        let index=(((bounds.center_y() as usize)*normal_width)+(bounds.x as usize-8))*4;
        [pixels[index],pixels[index+1],pixels[index+2]]
    };
    let user_face=sample("User");let assistant_face=sample("Assistant");
    assert!(user_face.iter().all(|value|*value>=245),"user card face is white: {user_face:?}");
    assert!(assistant_face.iter().all(|value|*value<235),"assistant card face is gray: {assistant_face:?}");
    assert_eq!(desktop.workspace.logs["markdown"].history.turns().find(|turn|turn.role=="User").unwrap().text,user);
    assert_eq!(desktop.workspace.logs["markdown"].history.turns().find(|turn|turn.role=="Assistant").unwrap().text,assistant);
    view.export(&desktop,"session-markdown-white-user-gray-assistant");
    let (_,code_bounds)=survey.text.iter().find(|(text,_)|text.contains("let request_id =")).expect("fenced code uses the shared readable code block");
    let normal_code_height=code_bounds.height;
    super::scroll_gesture::new_test_gesture();
    view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-100000.0}}),iced::Point::new(normal_size.width/2.0,normal_size.height/2.0));
    view.export(&desktop,"session-markdown-light-code-tail");
    let tail_survey=view.survey(&desktop);
    let (_,visible_code)=tail_survey.text.iter().find(|(text,_)|text.contains("let request_id =")).unwrap();
    let tail=view.pixels(&desktop);
    let index=(((visible_code.y as usize+5)*normal_width)+(visible_code.x as usize-8))*4;
    assert!(tail[index..index+3].iter().all(|value|*value>220),"fenced code uses a light inset background");
    let mut minimum=ViewFixture::new(&desktop,Size::new(760.0,520.0)).await;
    minimum.export(&desktop,"session-markdown-inline-minimum");
    let minimum_survey=minimum.survey(&desktop);
    let (_,minimum_code)=minimum_survey.text.iter().find(|(text,_)|text.contains("let request_id =")).unwrap();
    assert!(minimum_code.width<650.0&&minimum_code.height>normal_code_height,"long fenced code wraps within the minimum popup");
    super::scroll_gesture::new_test_gesture();
    minimum.event(&desktop,iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-100000.0}}),iced::Point::new(380.0,350.0));
    minimum.export(&desktop,"session-markdown-light-code-tail-minimum");
}

#[tokio::test]
async fn session_loading_shows_centered_status_until_the_list_arrives(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;desktop.workspace.section=Section::Sessions;
    event(&mut desktop, Event::Refresh);
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,630.0)).await;
    let survey = view.survey(&desktop);
    let loading = survey.text.iter().find(|(label, _)| label == "Loading...").unwrap().1;
    let tab = survey.text.iter().find(|(label, _)| label == "Sessions").unwrap().1;
    let panel = survey.containers.iter().filter(|bounds| bounds.y > tab.y + tab.height)
        .max_by(|a, b| (a.width * a.height).total_cmp(&(b.width * b.height)))
        .expect("the Sessions panel is below the tabs");
    assert!((loading.center_x() - panel.center_x()).abs() < 1.0
        && (loading.center_y() - panel.center_y()).abs() < 1.0,
        "loading status is centered in the panel: {loading:?}, {panel:?}");
    view.export(&desktop,"workspace-sessions-loading");

    event(&mut desktop, Event::Discovered(0, Ok(discovered(None, json!([]), ""))));
    finish_names(&mut desktop,json!([{"id":"previous","name":"Previous session","app":"claude","file_path":"/fixture/old.jsonl","project_path":"/fixture/project-a"}]));
    desktop.workspace.next_cursor=Some("next-page".into());
    view.relayout(&desktop);
    assert!(!view.survey(&desktop).text.iter().any(|(label, _)| label == "Loading..."));
    view.export(&desktop,"workspace-sessions-idle");
    event(&mut desktop,Event::Refresh);
    assert!(desktop.workspace.sessions_loading);
    assert_eq!(desktop.workspace.sessions.len(),1);
    view.relayout(&desktop);view.export(&desktop,"workspace-sessions-dimmed");
    let survey=view.survey(&desktop);
    assert!(survey.text.iter().any(|(text,_)|text=="Previous session"));
    assert!(survey.text.iter().any(|(text,_)|text=="Loading..."));
    assert!(!survey.text.iter().any(|(text,_)|text=="More sessions"||text.contains("Reading session names")));
    assert!(view.click(&desktop,"Previous session").is_empty(),"dimmed rows cannot open while refreshing");
    finish_names(&mut desktop,json!([{"id":"replacement","name":"Updated session","app":"claude","file_path":"/fixture/new.jsonl"}]));
    assert_eq!(desktop.workspace.sessions.len(),1);
    assert_eq!(desktop.workspace.sessions[0]["id"],"replacement");
    view.relayout(&desktop);
    assert!(!view.survey(&desktop).text.iter().any(|(label, _)| label == "Loading..."));
}

#[tokio::test]
async fn workspace_editor_and_paste_draft_keep_the_save_target_stable_during_model_changes() {
    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    let instruction = "/fixture/project-a/AGENTS.md";
    event(&mut desktop, Event::OpenFile(instruction.into()));
    event(&mut desktop, Event::File(0, instruction.into(), Ok(json!({"content":"Original instructions"}))));
    event(&mut desktop, Event::Edit(text_editor::Action::SelectAll));
    event(&mut desktop, Event::Edit(text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new("Edited instructions".into())))));
    assert!(desktop.workspace.dirty);
    event(&mut desktop, Event::Save);
    assert!(desktop.workspace.saving);
    event(&mut desktop, Event::DraftName("wrong-target".into()));
    event(&mut desktop, Event::CloseEditor);
    assert_eq!(desktop.workspace.destination, instruction);
    assert_eq!(desktop.workspace.editor.text(), "Edited instructions");
    event(&mut desktop, Event::Saved(0, Ok(json!({"saved":true}))));
    assert!(!desktop.workspace.dirty);

    event(&mut desktop, Event::App("codex".into()));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, Ok(discovered(Some("/fixture/project-a"), json!([]), "/fixture/project-a/.codex/agents"))));
    event(&mut desktop, Event::Section(Section::Agents));
    event(&mut desktop, Event::AddAgent);
    event(&mut desktop, Event::PasteAgent);
    let source = "---\nname: reviewer\ndescription: Review code\nmodel: source-model\n---\nReview carefully.\n";
    event(&mut desktop, Event::Pasted(generation, Some(json!({"fetch_agent":1,"app":"claude","content":source}).to_string())));
    event(&mut desktop, Event::Converted(generation, Ok(json!({
        "name":"reviewer","content":"name = \"reviewer\"\ndescription = \"Review code\"\ndeveloper_instructions = \"Review carefully.\\n\"\n",
        "issues":[{"field":"model","message":"Choose a destination model.","requires_action":true}]
    }))));
    assert!(desktop.workspace.draft && desktop.workspace.dirty && desktop.workspace.models_loading);
    let destination = desktop.workspace.destination.clone();
    assert_eq!(destination, "/fixture/project-a/.codex/agents/reviewer.toml");
    assert!(!desktop.workspace.issues.is_empty());
    assert!(!desktop.workspace.saving);
    let mut editor_view = ViewFixture::new(&desktop, Size::new(874.0, 1000.0)).await;
    let survey = editor_view.survey(&desktop);
    let save = survey.text.iter().find(|(label, _)| label == "Save").unwrap().1;
    let error = survey.text.iter().find(|(label, _)| label == "error!").unwrap().1;
    let info = survey.text.iter().find(|(label, _)| label == "ⓘ").unwrap().1;
    assert!(error.x > save.x + save.width && (error.center_y() - save.center_y()).abs() < 2.0);
    assert!(!survey.text.iter().any(|(label, _)| label.contains("I reviewed the settings")
        || label.contains("Choose a destination model.")));
    hover_error_details(&desktop, &mut editor_view, info.center(), "workspace-conversion-error-hover");
    assert!(editor_view.click(&desktop, "Save").iter().any(|message| matches!(message, Message::Workspace(Event::Save))),
        "the conversion notice does not gate the Save control behind a checkbox");
    event(&mut desktop, Event::ModelList(generation, "codex".into(), Ok(json!({"models":[{"slug":"fixture-model-from-app-v3","display_name":"Fixture model from app"}]}))));
    assert_eq!(desktop.workspace.models.len(), 1);
    assert_eq!(desktop.workspace.models[0].to_string(), "Fixture model from app");
    let model = desktop.workspace.models[0].clone();
    event(&mut desktop, Event::SelectModel(model));
    assert!(desktop.workspace.file_loading);
    event(&mut desktop, Event::OpenFile("/fixture/project-a/.codex/agents/other.toml".into()));
    event(&mut desktop, Event::CloseEditor);
    event(&mut desktop, Event::DraftName("wrong-target".into()));
    assert_eq!(desktop.workspace.destination, destination);
    assert!(desktop.workspace.draft);
    event(&mut desktop, Event::ModelApplied(generation, Ok(json!({"content":"name = \"reviewer\"\ndescription = \"Review code\"\nmodel = \"fixture-model-from-app-v3\"\ndeveloper_instructions = \"Review carefully.\\n\"\n"}))));
    assert!(desktop.workspace.issues.is_empty());
    assert!(desktop.workspace.editor.text().contains("fixture-model-from-app-v3"));
    editor_view.relayout(&desktop);
    editor_view.export(&desktop, "workspace-pasted-agent-editor");
    event(&mut desktop, Event::Save);
    assert!(desktop.workspace.saving);
    event(&mut desktop, Event::OpenFile("/fixture/project-a/.codex/agents/other.toml".into()));
    event(&mut desktop, Event::DraftName("wrong-target".into()));
    assert_eq!(desktop.workspace.destination, destination);
    event(&mut desktop, Event::Saved(generation, Ok(json!({"saved":true}))));
    assert_eq!(desktop.workspace.file_path.as_deref(), Some(destination.as_str()));
    assert!(!desktop.workspace.draft);

    event(&mut desktop, Event::Global);
    event(&mut desktop, Event::ModelApplied(generation, Ok(json!({"content":"old project must not reappear"}))));
    assert!(desktop.workspace.file_path.is_none());
    assert!(desktop.workspace.editor.text().is_empty());
}


#[tokio::test]
async fn workspace_logo_choices_filter_instructions_agents_and_sessions_in_one_row(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;
    let files=json!([
        {"name":"CLAUDE","kind":"instructions","app":"claude","file_path":"/fixture-home/.claude/CLAUDE.md"},
        {"name":"AGENTS","kind":"instructions","app":"codex","file_path":"/fixture-home/.codex/AGENTS.md"},
        {"name":"Claude reviewer","kind":"agent","app":"claude","file_path":"/fixture-home/.claude/agents/reviewer.md"},
        {"name":"Codex reviewer","kind":"agent","app":"codex","file_path":"/fixture-home/.codex/agents/reviewer.toml"}
    ]);
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,630.0)).await;
    for (app,instruction,other,agent,other_agent) in [("claude","CLAUDE","AGENTS","Claude reviewer","Codex reviewer"),("codex","AGENTS","CLAUDE","Codex reviewer","Claude reviewer")]{
        pick_app(&mut desktop,&mut view,app);
        let generation=desktop.workspace.generation;
        event(&mut desktop,Event::Discovered(generation,Ok(discovered(None,files.clone(),"/fixture-home/agents"))));
        finish_names(&mut desktop,json!([{"id":format!("{app}-session"),"name":format!("{app} session"),"app":app,"file_path":"/fixture/log.jsonl","project_path":null}]));
        view.relayout(&desktop);
        clicks(&mut desktop,&mut view,"Instructions");
        let path=format!("/fixture-home/.{}/{}.md",if app=="claude"{"claude"}else{"codex"},instruction);
        assert_eq!(desktop.workspace.file_path.as_deref(),Some(path.as_str()));
        event(&mut desktop,Event::File(generation,path.clone(),Ok(json!({"content":format!("{instruction} instructions body")}))));
        view.relayout(&desktop);
        let survey=view.survey(&desktop);
        assert!(!survey.text.iter().any(|(text,_)|text==instruction||text==other),"no redundant instruction selector");
        assert_eq!(desktop.workspace.editor.text(),format!("{instruction} instructions body"));
        assert!(!survey.text.iter().any(|(text,_)|matches!(text.as_str(),"Global"|"Open folder"|"Claude Code"|"Codex")),"removed button rows stay absent");
        view.export(&desktop,&format!("workspace-{app}-instructions-logo"));
        clicks(&mut desktop,&mut view,"Subagents");
        let survey=view.survey(&desktop);
        assert!(survey.text.iter().any(|(text,_)|text==agent));
        assert!(!survey.text.iter().any(|(text,_)|text==other_agent));
        view.export(&desktop,&format!("workspace-{app}-subagents-logo"));
        clicks(&mut desktop,&mut view,"Sessions");
        assert_eq!(desktop.workspace.app,app);
        assert!(view.survey(&desktop).text.iter().any(|(text,_)|text==&format!("{app} session")));
        view.export(&desktop,&format!("workspace-{app}-sessions-logo"));
        let mut small=ViewFixture::new(&desktop,Size::new(760.0,520.0)).await;
        small.export(&desktop,&format!("workspace-{app}-logo-minimum"));
    }
}

#[tokio::test]
async fn editor_undo_copy_and_save_text_use_actual_controls(){
    let (_directory,mut desktop)=desktop_fixture().await;desktop.page=Page::Workspace;
    let path="/fixture-home/.claude/CLAUDE.md";
    event(&mut desktop,Event::Discovered(0,Ok(discovered(None,json!([{"kind":"instructions","app":"claude","file_path":path}]),""))));
    event(&mut desktop,Event::File(0,path.into(),Ok(json!({"content":"Keep these instructions."}))));
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,800.0)).await;
    let survey=view.survey(&desktop);
    let input=survey.containers.iter().find(|rect|(rect.height-240.0).abs()<0.1&&rect.width>600.0).unwrap().center();
    for msg in view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),input){drop(desktop.update(msg));}
    view.relayout(&desktop);
    for modifiers in [iced::keyboard::Modifiers::LOGO,iced::keyboard::Modifiers::CTRL]{
        event(&mut desktop,Event::Edit(text_editor::Action::SelectAll));
        event(&mut desktop,Event::Edit(text_editor::Action::Edit(text_editor::Edit::Delete)));
        assert!(desktop.workspace.editor.text().is_empty());
        view.relayout(&desktop);
        let key=iced::Event::Keyboard(iced::keyboard::Event::KeyPressed{key:iced::keyboard::Key::Character("z".into()),modified_key:iced::keyboard::Key::Character("z".into()),physical_key:iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::KeyZ),location:iced::keyboard::Location::Standard,modifiers,text:None,repeat:false});
        let messages=view.event(&desktop,key,input);
        assert!(messages.iter().any(|msg|matches!(msg,Message::Workspace(Event::Undo))));
        for msg in messages{drop(desktop.update(msg));}view.relayout(&desktop);
        assert_eq!(desktop.workspace.editor.text(),"Keep these instructions.");assert!(!desktop.workspace.dirty);
    }
    let survey=view.survey(&desktop);let heading=survey.text.iter().find(|(text,_)|text=="Edit file").unwrap().1;
    let copy=survey.containers.iter().find(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&(rect.center_y()-heading.center_y()).abs()<3.0&&rect.x>600.0).unwrap().center();
    let mut messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
    messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
    assert!(messages.iter().any(|msg|matches!(msg,Message::Copy(value) if value=="Keep these instructions.")));
    assert_eq!(super::theme::save_button(&super::theme::light(),iced::widget::button::Status::Disabled).text_color,iced::Color::WHITE);
    view.export(&desktop,"workspace-editor-undo-copy-white-save");
}

#[tokio::test]
async fn continuous_outer_scroll_does_not_transfer_to_editor_until_a_new_gesture(){
    let (_directory,mut desktop)=desktop_fixture().await;desktop.page=Page::Workspace;
    let path="/fixture-home/.claude/CLAUDE.md";
    event(&mut desktop,Event::Discovered(0,Ok(discovered(None,json!([{"kind":"instructions","app":"claude","file_path":path}]),""))));
    event(&mut desktop,Event::File(0,path.into(),Ok(json!({"content":(0..100).map(|n|format!("Instruction line {n}\n")).collect::<String>()}))));
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,630.0)).await;
    let survey=view.survey(&desktop);let point=survey.text.iter().find(|(text,_)|text=="Edit file").unwrap().1.center();
    super::scroll_gesture::new_test_gesture();
    let wheel=||iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y:-180.0}});
    assert!(view.event(&desktop,wheel(),point).is_empty());
    // Iced normally resets its wheel target after a cursor update at100ms.
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::CursorMoved{position:point}),point);
    let messages=view.event(&desktop,wheel(),point);
    assert!(!messages.iter().any(|msg|matches!(msg,Message::Workspace(Event::Edit(text_editor::Action::Scroll{..})))));
    super::scroll_gesture::new_test_gesture();
    let messages=view.event(&desktop,wheel(),point);
    assert!(messages.iter().any(|msg|matches!(msg,Message::Workspace(Event::Edit(text_editor::Action::Scroll{..})))),"lifting and scrolling again targets the editor now under the pointer");
}

#[tokio::test]
async fn tool_results_start_collapsed_keep_their_names_and_copy_exact_output(){
    let (_directory,mut desktop)=desktop_fixture().await;desktop.page=Page::Workspace;desktop.workspace.section=Section::Sessions;
    finish_names(&mut desktop,json!([{"id":"tools","name":"Tool session","app":"claude","file_path":"/fixture/tools.jsonl"}]));
    event(&mut desktop,Event::OpenSession("tools".into()));
    let original_output=json!({"error":{"code":"UPSTREAM_UNAVAILABLE","message":"Cannot read the requested item","request_id":"0123456789abcdef".repeat(22)}}).to_string();
    let copied_output=format!("Read\n\n{original_output}");
    event(&mut desktop,Event::ConversationLoaded(0,"tools".into(),Ok(json!({"complete":true,"messages":[
        {"id":"read-1","kind":"tool_result","tool_name":"Read","output":original_output},
        {"id":"search-1","kind":"tool_result","tool_name":"Search","output":"Exact second tool output"},
        {"id":"a1","kind":"assistant","text":"Done."}
    ]}))));
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,750.0)).await;
    let survey=view.survey(&desktop);
    assert!(survey.text.iter().any(|(text,_)|text=="Read"));assert!(survey.text.iter().any(|(text,_)|text=="Search"));
    assert!(!survey.text.iter().any(|(text,_)|text.contains("UPSTREAM_UNAVAILABLE")||text.contains("Exact second")));
    view.export(&desktop,"session-tools-collapsed");
    let title=survey.text.iter().find(|(text,_)|text=="Read").unwrap().1;
    let copy=survey.containers.iter().find(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&(rect.center_y()-title.center_y()).abs()<3.0&&rect.x>600.0).unwrap().center();
    let mut messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
    messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
    assert!(messages.iter().any(|message|matches!(message,Message::Copy(value) if value==&copied_output)),"collapsed Copy includes the tool name and original result separated by a blank line");
    let pixels=view.pixels(&desktop);
    let visible_chevron=|pixels:&[u8],name:iced::Rectangle|{
        let mut dark=0;
        for y in (name.center_y() as usize-9)..(name.center_y() as usize+9){for x in (name.x as usize-40)..(name.x as usize-22){
            let offset=(y*874+x)*4;
            if pixels[offset..offset+3].iter().all(|value|*value<180){dark+=1;}
        }}
        dark>5
    };
    assert!(visible_chevron(&pixels,title),"collapsed tool name keeps its visible disclosure arrow");
    let sample=|x:f32,y:f32|{let offset=((y as usize)*874+x as usize)*4;[pixels[offset],pixels[offset+1],pixels[offset+2]]};
    let tool_face=sample(copy.x,copy.y-15.0);
    let assistant=survey.text.iter().find(|(text,_)|text=="Assistant").unwrap().1;
    let assistant_face=sample(assistant.x-8.0,assistant.center_y());
    assert!(tool_face.iter().zip(assistant_face).all(|(tool,assistant)|(*tool as i16-assistant as i16).abs()<=3),"tool and Assistant card faces share the same gray: {tool_face:?}, {assistant_face:?}");
    let mut collapsed_minimum=ViewFixture::new(&desktop,Size::new(760.0,520.0)).await;
    collapsed_minimum.export(&desktop,"session-tools-collapsed-minimum");
    clicks(&mut desktop,&mut view,"Read");let survey=view.survey(&desktop);
    let (display_output,output_bounds)=survey.text.iter().find(|(text,_)|text.contains("UPSTREAM_UNAVAILABLE")).expect("expanded JSON result shows the returned error details");
    assert!(display_output.starts_with("{\n")&&display_output.contains("\n  "),"minified JSON is displayed with indentation and line breaks");
    assert_eq!(serde_json::from_str::<Value>(display_output).unwrap(),serde_json::from_str::<Value>(&original_output).unwrap(),"readable formatting preserves every returned JSON value");
    let normal_output_height=output_bounds.height;
    assert_eq!(desktop.workspace.logs["tools"].history.turns().find(|turn|turn.id=="read-1").unwrap().text,original_output);
    assert!(!survey.text.iter().any(|(text,_)|text=="Exact second tool output"));
    let read=survey.text.iter().find(|(text,_)|text=="Read").unwrap().1;
    assert!(visible_chevron(&view.pixels(&desktop),read),"expanded tool name keeps its visible disclosure arrow");
    let search=survey.text.iter().find(|(text,_)|text=="Search").unwrap().1;
    let mut candidates:Vec<_>=survey.containers.iter().filter(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&rect.x>600.0&&rect.center_y()>=read.y&&rect.center_y()<search.y-12.0).map(|rect|rect.center()).collect();
    candidates.sort_by(|a,b|a.y.total_cmp(&b.y));candidates.dedup_by(|a,b|(a.x-b.x).abs()<0.1&&(a.y-b.y).abs()<0.1);
    let copies=candidates.into_iter().filter(|point|{
        let mut messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),*point);
        messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),*point));
        messages.iter().any(|message|matches!(message,Message::Copy(value) if value==&copied_output))
    }).count();
    assert_eq!(copies,1,"expanded tool card has just one actionable Copy control at its top right");
    view.export(&desktop,"session-tool-open");
    let mut minimum=ViewFixture::new(&desktop,Size::new(760.0,520.0)).await;
    let minimum_survey=minimum.survey(&desktop);
    let (_,minimum_output_bounds)=minimum_survey.text.iter().find(|(text,_)|text.contains("UPSTREAM_UNAVAILABLE")).unwrap();
    assert!(minimum_output_bounds.width<650.0,"long tool output stays inside the minimum popup");
    assert!(minimum_output_bounds.height>normal_output_height,"the long unbroken request ID wraps further in a narrower window");
    minimum.export(&desktop,"session-tool-json-minimum");
}

#[tokio::test]
async fn closed_conversation_finishes_in_cache_and_partial_replies_are_rejected(){
    let (_directory,mut desktop)=desktop_fixture().await;desktop.page=Page::Workspace;desktop.workspace.section=Section::Sessions;
    finish_names(&mut desktop,json!([{"id":"full","name":"Full session","app":"claude","file_path":"/fixture/full.jsonl"}]));
    event(&mut desktop,Event::OpenSession("full".into()));event(&mut desktop,Event::CloseSession);
    event(&mut desktop,Event::ConversationLoaded(0,"full".into(),Ok(json!({"complete":true,"messages":[
        {"id":"u1","kind":"user","text":"First message"},{"id":"a1","kind":"assistant","text":"Last reply"}
    ]}))));
    let task=desktop.update(Message::Workspace(Event::OpenSession("full".into())));assert_eq!(task.units(),0);drop(task);
    assert!(desktop.workspace.logs["full"].eof);assert_eq!(desktop.workspace.logs["full"].history.turns().count(),2);
    event(&mut desktop,Event::ConversationLoaded(0,"full".into(),Ok(json!({"complete":false,"messages":[]}))));
    assert!(desktop.workspace.logs["full"].error.is_some());
}

#[tokio::test]
async fn subagent_rows_open_rendered_popups_and_file_paths_offer_finder(){
    let (_directory,mut desktop)=desktop_fixture().await;desktop.page=Page::Workspace;
    desktop.workspace.section=Section::Agents;
    for (app,path,source) in [
        ("codex","/fixture-home/.codex/agents/reviewer-definition-with-clear-responsibilities-and-bounded-file-ownership-for-a-project-with-a-long-name.toml","description = \"Review changes\"\nsandbox_mode = \"read-only\"\ndeveloper_instructions = \"# Review checklist\\n\\n- Read the change\\n- Explain findings\"\n"),
        ("claude","/fixture-home/.claude/agents/reviewer.md","---\nname: reviewer\ndescription: Review changes\n---\n# Review checklist\n\n- Read the change\n- Explain findings\n")
    ]{
        desktop.workspace.app=app.into();
        event(&mut desktop,Event::Discovered(0,Ok(discovered(None,json!([{"name":"reviewer","kind":"agent","app":app,"file_path":path}]),""))));
        let mut view=ViewFixture::new(&desktop,Size::new(874.0,750.0)).await;
        assert!(!desktop.workspace.agent_popup);
        let row_label=view.survey(&desktop).text.iter().find(|(text,_)|text=="reviewer").unwrap().1;
        let row_pixels=view.pixels(&desktop);
        let face_index=((row_label.center_y() as usize)*874+(row_label.x as usize-8))*4;
        assert!(row_pixels[face_index+2]>row_pixels[face_index]+20,"the subagent name action has a blue face");
        let copied=desktop.update(Message::Workspace(Event::Copied(0,app.into(),Some("reviewer".into()),path.into(),Ok(json!({"content":source})))));
        assert!(copied.units()>0,"copying a subagent still queues a clipboard write");
        drop(copied);
        view.relayout(&desktop);
        assert!(!view.survey(&desktop).text.iter().any(|(text,_)|text=="Copied agent"));
        view.export(&desktop,&format!("{app}-subagent-blue-row-code-path"));
        clicks(&mut desktop,&mut view,"reviewer");
        event(&mut desktop,Event::File(0,path.into(),Ok(json!({"content":source}))));view.relayout(&desktop);
        assert!(desktop.workspace.agent_popup&&!desktop.workspace.agent_editing);
        let survey=view.survey(&desktop);
        assert!(survey.text.iter().any(|(text,_)|text=="description"));
        assert!(survey.text.iter().any(|(text,_)|text=="Review changes"));
        assert!(!survey.text.iter().any(|(text,_)|text=="Edit file"||text.contains("developer_instructions =")));
        let path_row=survey.text.iter().filter(|(text,_)|text==path).last().unwrap().1;
        let mut path_controls:Vec<_>=survey.containers.iter().filter(|rect|(rect.width-28.0).abs()<0.1&&(rect.height-28.0).abs()<0.1&&(rect.center_y()-path_row.center_y()).abs()<2.0).map(|rect|rect.center()).collect();
        path_controls.sort_by(|a,b|a.x.total_cmp(&b.x));path_controls.dedup_by(|a,b|(a.x-b.x).abs()<0.1&&(a.y-b.y).abs()<0.1);
        assert_eq!(path_controls.len(),2,"path Copy and Finder share a compact hit area");
        let copy=path_controls[0];let finder=path_controls[1];
        let pixels=view.pixels(&desktop);let mut blue_pixels=Vec::new();
        for y in (copy.y as usize-14)..(copy.y as usize+14){for x in (copy.x as usize-14)..(copy.x as usize+14){
            let offset=(y*874+x)*4;
            if pixels[offset+2] as i16-pixels[offset] as i16>25&&pixels[offset+2]>180{blue_pixels.push((x,y));}
        }}
        assert!(!blue_pixels.is_empty(),"the path Copy SVG remains blue");
        let glyph_width=blue_pixels.iter().map(|(x,_)|x).max().unwrap()-blue_pixels.iter().map(|(x,_)|x).min().unwrap()+1;
        let glyph_height=blue_pixels.iter().map(|(_,y)|y).max().unwrap()-blue_pixels.iter().map(|(_,y)|y).min().unwrap()+1;
        assert!(glyph_width<=14&&glyph_height<=14,"path Copy glyph is at most14px: {glyph_width}×{glyph_height}");
        let pixel_at=|x:f32,y:f32|{
            let offset=(y as usize*874+x as usize)*4;
            &pixels[offset..offset+3]
        };
        let blue=|pixel:&[u8]|pixel[2] as i16-pixel[0] as i16>20;
        assert!(blue(pixel_at(finder.x+10.0,finder.y)),"Finder has a blue button face outside the glyph");
        for side in [-1.0,1.0]{
            assert!(pixel_at(finder.x+side*4.0,finder.y-4.5).iter().all(|&channel|channel>242),"each side of the Finder glyph has a solid white fill");
            assert!(blue(pixel_at(finder.x+side*3.3,finder.y-2.1)),"each eye dot shows the button's blue through the white face");
        }
        if app=="claude"{assert!(survey.containers.iter().any(|rect|(rect.height-44.0).abs()<0.1&&rect.contains(path_row.center())&&rect.contains(copy)),"a short path code box is44px tall");}
        let mut copy_messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),copy);
        copy_messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),copy));
        assert!(copy_messages.iter().any(|message|matches!(message,Message::Copy(value) if value==path)),"path code block copies the exact file path");
        assert!(finder.x>copy.x,"Finder sits beside the path code block");
        let mut messages=view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),finder);
        messages.extend(view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),finder));
        assert!(messages.iter().any(|message|matches!(message,Message::Workspace(Event::RevealFile(value)) if value==path)));
        // Do not execute the Finder Task in this fixture.
        view.export(&desktop,&format!("{app}-subagent-rendered-popup"));
        event(&mut desktop,Event::PreviewAction(text_editor::Action::SelectAll));
        event(&mut desktop,Event::PreviewAction(text_editor::Action::Edit(text_editor::Edit::Paste(std::sync::Arc::new("must not change preview or source".into())))));
        assert_eq!(desktop.workspace.editor.text(),source,"preview actions cannot modify the native definition source");
        clicks(&mut desktop,&mut view,"Edit");assert!(desktop.workspace.agent_editing);
        assert_eq!(desktop.workspace.editor.text(),source);
        clicks(&mut desktop,&mut view,"Preview");assert!(!desktop.workspace.agent_editing);
        let mut minimum=ViewFixture::new(&desktop,Size::new(760.0,520.0)).await;
        minimum.export(&desktop,&format!("{app}-subagent-popup-minimum-code-path"));
        let minimum_survey=minimum.survey(&desktop);
        let path_bounds=minimum_survey.text.iter().filter(|(text,_)|text==path).last().unwrap().1;
        assert!(path_bounds.width<600.0,"long paths wrap within the minimum popup");
        assert!(path_bounds.x+path_bounds.width<760.0,"path text does not push controls beyond the window");
        if app == "claude" {
            let survey = view.survey(&desktop);
            let edit = survey.text.iter().find(|(label, _)| label == "Edit").unwrap().1;
            let mut buttons: Vec<_> = survey.containers.iter().filter(|rect| {
                (rect.width - 40.0).abs() < 0.1 && (rect.height - 40.0).abs() < 0.1
                    && (rect.center_y() - edit.center_y()).abs() < 2.0 && rect.x < edit.x
            }).map(|rect| rect.center()).collect();
            buttons.sort_by(|a,b| a.x.total_cmp(&b.x));
            buttons.dedup();
            let popup_click = |view: &mut ViewFixture, desktop: &Desktop, point| {
                let mut messages = view.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)), point);
                messages.extend(view.event(desktop, iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)), point));
                messages
            };
            assert_eq!(buttons.len(), 2, "Paste and Copy appear in the popup header");
            assert!((buttons[1].x - buttons[0].x - 52.0).abs() < 1.0, "Paste sits directly to the left of Copy");
            assert!(matches!(popup_click(&mut view, &desktop, buttons[1]).as_slice(), [Message::Copy(value)] if value == source));
            let mut paste = popup_click(&mut view, &desktop, buttons[0]);
            assert!(matches!(paste.as_slice(), [Message::Workspace(Event::PasteAgent)]));
            assert_eq!(desktop.update(paste.remove(0)).units(), 1, "Paste queues a clipboard read");
            let generation = desktop.workspace.generation;
            event(&mut desktop, Event::Pasted(generation, Some("Pasted note\n".into())));
            assert!(desktop.workspace.agent_editing && desktop.workspace.dirty);
            assert_eq!(desktop.workspace.file_path.as_deref(), Some(path));
            let text = desktop.workspace.editor.text();
            assert!(text.contains("Pasted note\n") && text.contains(source), "Paste inserts content into the existing draft");
            assert!(!desktop.workspace.saving);
        }
        event(&mut desktop,Event::CloseAgentPopup);view.relayout(&desktop);
        assert!(!desktop.workspace.agent_popup);
        assert!(!view.survey(&desktop).text.iter().any(|(text,_)|text=="Edit file"));
    }
}

#[tokio::test]
async fn claude_subagent_popup_edits_name_and_description_in_the_existing_file() {
    use crate::workspace::ClaudeAgentMetadataField;
    use iced::advanced::widget::{Id, Operation, operation::TextInput};
    use iced::keyboard::{Event as KeyEvent, Key, Location, Modifiers, key::{Code, Physical}};

    #[derive(Default)]
    struct InputBounds(Vec<iced::Rectangle>);
    impl Operation for InputBounds {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text_input(&mut self, _: Option<&Id>, bounds: iced::Rectangle, _: &mut dyn TextInput) {
            self.0.push(bounds);
        }
    }

    let (_directory, mut desktop) = desktop_fixture().await;
    desktop.page = Page::Workspace;
    desktop.workspace.section = Section::Agents;
    let path = "/fixture-home/.claude/agents/reviewer.md";
    let body = "\n# Review checklist\n\n- Read the change\n- Explain findings\n";
    let source = format!("---\nname: reviewer\ndescription: Review changes\ntools: [Read, Grep]\n---\n{body}");
    event(&mut desktop, Event::Discovered(0, Ok(discovered(None, json!([
        {"name":"reviewer","kind":"agent","app":"claude","file_path":path}
    ]), ""))));
    let mut view = ViewFixture::new(&desktop, Size::new(874.0, 750.0)).await;
    clicks(&mut desktop, &mut view, "reviewer");
    event(&mut desktop, Event::File(0, path.into(), Ok(json!({"content":source}))));
    view.relayout(&desktop);
    clicks(&mut desktop, &mut view, "Edit");

    let key = |character: &'static str, code| iced::Event::Keyboard(KeyEvent::KeyPressed {
        key: Key::Character(character.into()),
        modified_key: Key::Character(character.into()),
        physical_key: Physical::Code(code),
        location: Location::Standard,
        modifiers: Modifiers::COMMAND,
        text: None,
        repeat: false,
    });
    for (label, value) in [
        ("Name", "quality-reviewer"),
        ("Description", "Look for regressions: review each new path."),
    ] {
        let survey = view.survey(&desktop);
        let label_bounds = survey.text.iter().find(|(text, _)| text == label).unwrap().1;
        let mut inputs = InputBounds::default();
        view.operate(&desktop, &mut inputs);
        let point = inputs.0.iter()
            .filter(|bounds| bounds.y >= label_bounds.y + label_bounds.height
                && bounds.x <= label_bounds.x && bounds.x + bounds.width >= label_bounds.x + label_bounds.width)
            .min_by(|a, b| a.y.total_cmp(&b.y))
            .expect("each metadata label has an input directly below it")
            .center();
        for action in [iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left),
            iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)] {
            for message in view.event(&desktop, iced::Event::Mouse(action), point) {
                drop(desktop.update(message));
            }
        }
        assert!(view.event(&desktop, iced::Event::Keyboard(KeyEvent::ModifiersChanged(Modifiers::COMMAND)), point).is_empty());
        assert!(view.event(&desktop, key("a", Code::KeyA), point).is_empty());
        let mut clipboard = FixtureClipboard(value.into());
        let (mut messages, status) = view.event_with_clipboard(&desktop, key("v", Code::KeyV), point, &mut clipboard);
        assert_eq!(status, iced::event::Status::Captured);
        let [Message::Workspace(Event::EditClaudeMetadata(field, text))] = messages.as_slice() else {
            panic!("{label} must emit exactly one metadata edit");
        };
        let target_label = match field {
            ClaudeAgentMetadataField::Name => "Name",
            ClaudeAgentMetadataField::Description => "Description",
        };
        assert_eq!(target_label, label);
        assert_eq!(text, value);
        assert_eq!(desktop.update(messages.remove(0)).units(), 0, "editing only changes the draft");
        view.relayout(&desktop);
    }

    let draft = desktop.workspace.editor.text();
    let preview = crate::workspace::agent_definition_preview("claude", &draft).unwrap();
    assert_eq!(preview["fields"]["name"], "quality-reviewer");
    assert_eq!(preview["fields"]["description"], "Look for regressions: review each new path.");
    assert_eq!(preview["fields"]["tools"], json!(["Read", "Grep"]));
    assert_eq!(draft.split_once("\n---\n").unwrap().1, body);
    assert!(desktop.workspace.dirty);
    assert!(!desktop.workspace.saving && !desktop.workspace.draft);
    assert_eq!(desktop.workspace.destination, path);
    view.export(&desktop, "workspace-claude-subagent-editable-metadata");

    let path_bounds = view.survey(&desktop).text.into_iter().filter(|(text, _)| text == path).last().unwrap().1;
    view.event(&desktop, iced::Event::Mouse(iced::mouse::Event::WheelScrolled {
        delta: iced::mouse::ScrollDelta::Pixels { x: 0.0, y: -650.0 },
    }), path_bounds.center());
    let mut messages = view.click(&desktop, "Save");
    assert!(matches!(messages.as_slice(), [Message::Workspace(Event::Save)]));
    assert_eq!(desktop.update(messages.remove(0)).units(), 1, "Save queues the existing definition write");
    assert!(desktop.workspace.saving);
    assert_eq!(desktop.workspace.file_path.as_deref(), Some(path));
    assert_eq!(desktop.workspace.destination, path);
    event(&mut desktop, Event::Saved(0, Ok(json!({"file_path":path,"saved":true}))));
    view.relayout(&desktop);
    assert!(!desktop.workspace.dirty && !desktop.workspace.saving);

    clicks(&mut desktop, &mut view, "Preview");
    let survey = view.survey(&desktop);
    assert!(survey.text.iter().any(|(text, _)| text == "quality-reviewer"));
    assert!(survey.text.iter().any(|(text, _)| text == "Look for regressions: review each new path."));
    assert_eq!(desktop.workspace.file_path.as_deref(), Some(path));
}

#[tokio::test]
async fn subagent_preview_wheel_reaches_the_end_without_moving_the_header_or_path(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;
    desktop.workspace.section=Section::Agents;
    let path="/fixture-home/.claude/agents/reviewer.md";
    event(&mut desktop,Event::Discovered(0,Ok(discovered(None,json!([{"name":"reviewer","kind":"agent","app":"claude","file_path":path}]),""))));
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,750.0)).await;
    clicks(&mut desktop,&mut view,"reviewer");
    let instructions=(0..55).map(|line|format!("Review instruction {line:02}\n")).collect::<String>();
    let source=format!("---\nname: reviewer\ndescription: Review changes\n---\n{instructions}Final review instruction\n");
    event(&mut desktop,Event::File(0,path.into(),Ok(json!({"content":source}))));
    view.relayout(&desktop);

    let before=view.survey(&desktop);
    let scroll=|survey:&super::neumorphic_tests::Survey|{
        let (_,bounds,content,offset)=survey.scrolls.iter().find(|(id,_,_,_)|id.contains("fetch-workspace-agent-preview")).unwrap();
        (*bounds,*content,*offset)
    };
    let pinned=|survey:&super::neumorphic_tests::Survey|["Edit",path].map(|label|survey.text.iter().filter(|(text,_)|text==label).last().unwrap().1);
    let (viewport,content,initial_offset)=scroll(&before);
    assert_eq!(initial_offset.y,0.0);
    assert!(content.height>viewport.height+300.0,"the full instructions exceed the preview viewport");
    view.export(&desktop,"subagent-preview-top");

    let wheel=|y:f32|iced::Event::Mouse(iced::mouse::Event::WheelScrolled{delta:iced::mouse::ScrollDelta::Pixels{x:0.0,y}});
    super::scroll_gesture::new_test_gesture();
    let point=viewport.center();
    assert!(view.event(&desktop,wheel(-180.0),point).is_empty(),"wheel over instructions scrolls the panel without sending an editor action");
    let middle=view.survey(&desktop);
    assert!(scroll(&middle).2.y>100.0);
    assert_eq!(pinned(&middle),pinned(&before));

    assert!(view.event(&desktop,wheel(-1500.0),point).is_empty());
    let end=view.survey(&desktop);
    let (viewport,content,offset)=scroll(&end);
    assert!((offset.y-(content.height-viewport.height)).abs()<1.0,"the same scroll area reaches the final line");
    assert_eq!(pinned(&end),pinned(&before),"the popup actions and file path stay visible throughout the scroll");
    view.export(&desktop,"subagent-preview-end");
}

#[tokio::test]
async fn escape_closes_focused_workspace_popups_and_preserves_drafts_and_cached_history(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;desktop.workspace.app="codex".into();desktop.workspace.section=Section::Agents;
    let path="/fixture-home/.codex/agents/reviewer.toml";
    let source="developer_instructions = \"Review carefully.\"\n";
    event(&mut desktop,Event::Discovered(0,Ok(discovered(None,json!([{"name":"reviewer","kind":"agent","app":"codex","file_path":path}]),""))));
    let mut view=ViewFixture::new(&desktop,super::window_settings().size).await;
    clicks(&mut desktop,&mut view,"reviewer");
    event(&mut desktop,Event::File(0,path.into(),Ok(json!({"content":source}))));view.relayout(&desktop);
    clicks(&mut desktop,&mut view,"Edit");
    let editor=view.survey(&desktop).containers.iter().filter(|rect|(rect.height-240.0).abs()<0.1&&rect.width>600.0).last().unwrap().center();
    for message in view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)),editor){drop(desktop.update(message));}
    view.event(&desktop,iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)),editor);
    let typed=iced::Event::Keyboard(iced::keyboard::Event::KeyPressed{key:iced::keyboard::Key::Character(" ".into()),modified_key:iced::keyboard::Key::Character(" ".into()),physical_key:iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::Space),location:iced::keyboard::Location::Standard,modifiers:iced::keyboard::Modifiers::default(),text:Some(" ".into()),repeat:false});
    let (messages,status)=view.event_with_status(&desktop,typed,editor);
    assert_eq!(status,iced::event::Status::Captured,"the actual editor has keyboard focus");
    for message in messages{drop(desktop.update(message));}
    assert!(desktop.workspace.dirty);
    let draft=desktop.workspace.editor.text();
    assert!(matches!(super::subscription_event(super::neumorphic_tests::escape_key(),iced::event::Status::Captured,desktop.window.unwrap()),Some(Message::Escape(_))),"the production subscription also routes Escape captured by a widget");
    super::neumorphic_tests::press_escape(&mut view,&mut desktop,editor);
    assert!(!desktop.workspace.agent_popup);
    assert!(desktop.workspace.dirty&&desktop.workspace.editor.text()==draft,"dismissing preserves unsaved source edits");
    clicks(&mut desktop,&mut view,"reviewer");
    assert!(desktop.workspace.agent_popup&&desktop.workspace.agent_editing);
    assert_eq!(desktop.workspace.editor.text(),draft);
    super::neumorphic_tests::press_escape(&mut view,&mut desktop,editor);

    clicks(&mut desktop,&mut view,"Sessions");
    finish_names(&mut desktop,json!([{"id":"escape-session","name":"Cached session","app":"codex","file_path":"/fixture/log.jsonl"}]));view.relayout(&desktop);
    clicks(&mut desktop,&mut view,"Cached session");
    event(&mut desktop,Event::ConversationLoaded(0,"escape-session".into(),Ok(json!({"complete":true,"messages":[{"id":"answer","kind":"assistant","text":"Keep this cached reply."}]}))));view.relayout(&desktop);
    super::neumorphic_tests::press_escape(&mut view,&mut desktop,iced::Point::new(400.0,350.0));
    assert!(desktop.workspace.active_session.is_none());
    assert!(desktop.workspace.logs["escape-session"].eof);
    assert_eq!(desktop.workspace.logs["escape-session"].history.turns().next().unwrap().text,"Keep this cached reply.");
    super::neumorphic_tests::press_escape(&mut view,&mut desktop,iced::Point::new(400.0,350.0));
    assert!(desktop.workspace.active_session.is_none()&&!desktop.workspace.agent_popup,"Escape without a popup does nothing");
    assert_eq!(desktop.workspace.editor.text(),draft);
}
