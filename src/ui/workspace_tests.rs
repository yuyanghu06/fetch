//! Workspace interactions use API replies and dropped Tasks; no files, clipboard,
//! model processes or native windows are opened by these UI checks.
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
    let mut buttons:Vec<_>=survey.containers.iter().filter(|rect|(rect.width-40.0).abs()<0.1&&(rect.height-40.0).abs()<0.1&&(rect.center_y()-tabs.center_y()).abs()<2.0).copied().collect();
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
    assert!(!view.survey(&desktop).text.iter().any(|(text, _)| text == missing_rollout));
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
async fn session_loading_dims_retained_rows_and_removes_loading_footer(){
    let (_directory,mut desktop)=desktop_fixture().await;
    desktop.page=Page::Workspace;desktop.workspace.section=Section::Sessions;
    finish_names(&mut desktop,json!([{"id":"previous","name":"Previous session","app":"claude","file_path":"/fixture/old.jsonl","project_path":"/fixture/project-a"}]));
    desktop.workspace.next_cursor=Some("next-page".into());
    let mut view=ViewFixture::new(&desktop,Size::new(874.0,630.0)).await;
    view.export(&desktop,"workspace-sessions-idle");
    event(&mut desktop,Event::Refresh);
    assert!(desktop.workspace.sessions_loading);
    assert_eq!(desktop.workspace.sessions.len(),1);
    view.relayout(&desktop);view.export(&desktop,"workspace-sessions-dimmed");
    let survey=view.survey(&desktop);
    assert!(survey.text.iter().any(|(text,_)|text=="Previous session"));
    assert!(!survey.text.iter().any(|(text,_)|text=="More sessions"||text.contains("Reading session names")));
    assert!(view.click(&desktop,"Previous session").is_empty(),"dimmed rows cannot open while refreshing");
    finish_names(&mut desktop,json!([{"id":"replacement","name":"Updated session","app":"claude","file_path":"/fixture/new.jsonl"}]));
    assert_eq!(desktop.workspace.sessions.len(),1);
    assert_eq!(desktop.workspace.sessions[0]["id"],"replacement");
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
    event(&mut desktop, Event::Destination("/fixture/wrong-target.md".into()));
    event(&mut desktop, Event::CloseEditor);
    assert_eq!(desktop.workspace.destination, instruction);
    assert_eq!(desktop.workspace.editor.text(), "Edited instructions");
    event(&mut desktop, Event::Saved(0, Ok(json!({"saved":true}))));
    assert!(desktop.workspace.feedback.is_empty());
    assert!(!desktop.workspace.dirty);

    event(&mut desktop, Event::App("codex".into()));
    let generation = desktop.workspace.generation;
    event(&mut desktop, Event::Discovered(generation, Ok(discovered(Some("/fixture/project-a"), json!([]), "/fixture/project-a/.codex/agents"))));
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
    event(&mut desktop, Event::ModelList(generation, "codex".into(), Ok(json!({"models":[{"slug":"fixture-model-from-app-v3","display_name":"Fixture model from app"}]}))));
    assert_eq!(desktop.workspace.models.len(), 1);
    assert_eq!(desktop.workspace.models[0].to_string(), "Fixture model from app");
    let model = desktop.workspace.models[0].clone();
    event(&mut desktop, Event::SelectModel(model));
    assert!(desktop.workspace.file_loading);
    event(&mut desktop, Event::OpenFile("/fixture/project-a/.codex/agents/other.toml".into()));
    event(&mut desktop, Event::CloseEditor);
    event(&mut desktop, Event::Destination("/fixture/wrong-target.toml".into()));
    assert_eq!(desktop.workspace.destination, destination);
    assert!(desktop.workspace.draft);
    event(&mut desktop, Event::ModelApplied(generation, Ok(json!({"content":"name = \"reviewer\"\ndescription = \"Review code\"\nmodel = \"fixture-model-from-app-v3\"\ndeveloper_instructions = \"Review carefully.\\n\"\n"}))));
    assert!(desktop.workspace.issues.is_empty());
    assert!(desktop.workspace.editor.text().contains("fixture-model-from-app-v3"));
    let mut editor_view = ViewFixture::new(&desktop, Size::new(874.0, 1000.0)).await;
    editor_view.export(&desktop, "workspace-pasted-agent-editor");
    event(&mut desktop, Event::Save);
    assert!(desktop.workspace.saving);
    event(&mut desktop, Event::OpenFile("/fixture/project-a/.codex/agents/other.toml".into()));
    event(&mut desktop, Event::Destination("/fixture/wrong-target.toml".into()));
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
        event(&mut desktop,Event::CloseAgentPopup);view.relayout(&desktop);
        assert!(!desktop.workspace.agent_popup);
        assert!(!view.survey(&desktop).text.iter().any(|(text,_)|text=="Edit file"));
    }
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
