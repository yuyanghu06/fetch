//! Window-free tests of pane/session state; no HTTP tasks are executed.
use super::*;
use crate::ui::{auth_tests::desktop_fixture, Page};
fn opened(cursor:u64)->crate::ui::client::TerminalOpened {crate::ui::client::TerminalOpened{created:false,value:json!({"session_id":"terminal-fixture","status":"running","output_cursor":cursor,"exit_code":null,"signal":null})}}
fn output(bytes:&[u8],cursor:u64)->Value {json!({"data":STANDARD.encode(bytes),"next_cursor":cursor,"first_available_cursor":0,"gap":false,"status":"running","exit_code":null,"signal":null})}
async fn ready()->(tempfile::TempDir,Desktop) {
    let (directory,mut desktop)=desktop_fixture().await;
    drop(desktop.update_terminal(Message::Open("github".into())));assert!(desktop.terminals["github"].opening);
    drop(desktop.update_terminal(Message::Opened("github".into(),0,Ok(opened(0)))));
    (directory,desktop)
}
#[tokio::test]
async fn console_switch_collapse_and_page_changes_keep_screen_session_and_output_reads() {
    let (_directory,mut desktop)=ready().await;
    let model=desktop.terminals["github"].terminal.as_ref().unwrap().id;
    drop(desktop.update_terminal(Message::Console("github".into())));
    desktop.expanded_servers.clear();desktop.page=Page::Settings;
    let bytes=b"fixture received while console is hidden\r\n";
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(bytes,bytes.len() as u64)))));
    assert_eq!(desktop.terminals["github"].cursor,bytes.len() as u64);
    assert!(desktop.terminals["github"].pane==Pane::Console);
    drop(desktop.update_terminal(Message::Open("github".into())));
    assert!(desktop.terminals["github"].pane==Pane::Terminal);assert!(!desktop.terminals["github"].opening);
    assert_eq!(desktop.terminals["github"].terminal.as_ref().unwrap().id,model);
    desktop.terminals.get_mut("github").unwrap().status="exited".into();
    drop(desktop.update_terminal(Message::Console("github".into())));drop(desktop.update_terminal(Message::Open("github".into())));
    assert!(!desktop.terminals["github"].opening);assert_eq!(desktop.terminals["github"].session.as_deref(),Some("terminal-fixture"));assert_eq!(desktop.terminals["github"].terminal.as_ref().unwrap().id,model);
}
#[tokio::test]
async fn remote_terminal_is_disabled_and_console_requires_an_explicit_click() {
    let (_directory,mut desktop)=desktop_fixture().await;desktop.servers=json!({"servers":[{"server_id":"remote","config":{"kind":"remote"}}]});
    assert!(desktop.server_panel("remote")==Pane::Hidden);
    drop(desktop.update_terminal(Message::Open("remote".into())));assert!(!desktop.terminals.contains_key("remote"));
    drop(desktop.update_terminal(Message::Console("remote".into())));assert!(desktop.terminals["remote"].pane==Pane::Console);assert!(desktop.terminals["remote"].session.is_none());
    let _view=desktop.server_output_view("remote",&desktop.servers["servers"][0]);
    drop(_view);
    drop(desktop.update_terminal(Message::Console("remote".into())));
    assert!(desktop.server_panel("remote")==Pane::Hidden);
}
#[tokio::test]
async fn close_bypasses_blocked_input_and_stale_responses_do_not_mutate_session() {
    let (_directory,mut desktop)=ready().await;let state=desktop.terminals.get_mut("github").unwrap();state.applying=true;state.pending.push_back(json!({"action":"input","data":"private-bytes"}));
    drop(desktop.update_terminal(Message::Close("github".into())));let state=&desktop.terminals["github"];assert!(state.closing);assert!(!state.applying);assert!(state.pending.is_empty());assert_eq!(state.epoch,1);
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(b"stale",999)))));
    drop(desktop.update_terminal(Message::Applied("github".into(),"terminal-fixture".into(),0,false,Ok(json!({"status":"running"})))));
    assert!(desktop.terminals["github"].closing);assert_eq!(desktop.terminals["github"].cursor,0);
    drop(desktop.update_terminal(Message::Applied("github".into(),"terminal-fixture".into(),1,true,Ok(json!({"status":"closed","exit_code":0,"signal":null})))));
    assert_eq!(desktop.terminals["github"].status,"closed");assert!(!desktop.terminals["github"].closing);
}
#[tokio::test]
async fn window_close_keeps_model_but_invalidates_pending_requests_and_unsent_input() {
    let (_directory,mut desktop)=ready().await;let model=desktop.terminals["github"].terminal.as_ref().unwrap().id;
    desktop.terminals.get_mut("github").unwrap().pending.push_back(json!({"action":"input"}));let window=desktop.window.unwrap();drop(desktop.update(AppMessage::Closed(window)));
    let state=&desktop.terminals["github"];assert_eq!(state.session.as_deref(),Some("terminal-fixture"));assert_eq!(state.terminal.as_ref().unwrap().id,model);assert!(state.pending.is_empty());assert_eq!(state.epoch,1);assert!(desktop.client.is_none());
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(b"stale",55)))));assert_eq!(desktop.terminals["github"].cursor,0);
}
#[tokio::test]
async fn terminal_gap_is_visible_and_failed_input_is_never_replayed() {
    let (_directory,mut desktop)=ready().await;let model=desktop.terminals["github"].terminal.as_ref().unwrap().id;
    let mut value=output(b"remaining",109);value["gap"]=json!(true);value["first_available_cursor"]=json!(100);
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(value))));let state=&desktop.terminals["github"];assert!(state.gap);assert_eq!(state.cursor,109);assert_ne!(state.terminal.as_ref().unwrap().id,model);
    let state=desktop.terminals.get_mut("github").unwrap();state.applying=true;state.pending.push_back(json!({"action":"input","data":"remaining paste"}));
    drop(desktop.update_terminal(Message::Applied("github".into(),"terminal-fixture".into(),0,false,Err("actual write failure".into()))));let state=&desktop.terminals["github"];assert!(state.pending.is_empty());assert!(!state.applying);assert_eq!(state.error.as_deref(),Some("actual write failure"));
}
#[tokio::test]
async fn reused_session_does_not_answer_historical_terminal_queries_again() {
    let (_directory,mut desktop)=desktop_fixture().await;drop(desktop.update_terminal(Message::Open("github".into())));
    let query=b"\x1b[6n";drop(desktop.update_terminal(Message::Opened("github".into(),0,Ok(opened(query.len() as u64)))));
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(query,query.len() as u64)))));
    let state=&desktop.terminals["github"];assert!(!state.applying);assert!(state.pending.is_empty());
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(query,(query.len()*2) as u64)))));
    assert!(desktop.terminals["github"].applying,"new query should send its answer through the API");
}

#[tokio::test]
async fn fresh_session_answers_early_query_even_before_open_response() {
    let (_directory,mut desktop)=desktop_fixture().await;drop(desktop.update_terminal(Message::Open("github".into())));
    let query=b"\x1b[6n";let mut fresh=opened(query.len() as u64);fresh.created=true;
    drop(desktop.update_terminal(Message::Opened("github".into(),0,Ok(fresh))));
    drop(desktop.update_terminal(Message::Output("github".into(),"terminal-fixture".into(),0,Ok(output(query,query.len() as u64)))));
    assert!(desktop.terminals["github"].applying,"new shell's early query must get its first answer");
}

#[tokio::test]
async fn embedded_terminal_renders_prompt_and_moves_cached_cells_when_scrolled() {
    use iced::advanced::{renderer::{Headless,Renderer as _},layout,widget::Tree,Layout};
    use iced::{Color,Font,Pixels,Point,Rectangle,Size,Theme};
    let mut renderer=<iced::Renderer as Headless>::new(Font::MONOSPACE,Pixels(14.0),None).await.expect("offscreen renderer for terminal display verification");
    let mut terminal=Terminal::new_external(999_999,Default::default(),50,10);
    terminal.handle(Command::ProxyToBackend(iced_term::BackendCommand::Resize(Some(Size::new(400.0,160.0)),Some(Size::new(8.0,16.0)))));
    terminal.receive_output(b"fixture-shell$ visible prompt\x1b[?25h");
    let mut element=TerminalView::show(&terminal);let mut tree=Tree::new(element.as_widget());
    let node=element.as_widget_mut().layout(&mut tree,&renderer,&layout::Limits::new(Size::new(400.0,160.0),Size::new(400.0,160.0)));
    let screen=Rectangle::new(Point::ORIGIN,Size::new(640.0,640.0));
    let mut render=|position:Point,viewport:Rectangle|{
        renderer.reset(screen);
        let located=node.clone().move_to(position);
        element.as_widget().draw(&tree,&mut renderer,&Theme::Dark,&Default::default(),Layout::new(&located),iced::mouse::Cursor::Unavailable,&viewport);
        <iced::Renderer as Headless>::screenshot(&mut renderer,Size::new(640,640),1.0,Color::BLACK)
    };
    let first=render(Point::new(40.0,430.0),screen);
    // The same cached terminal receives no new output while its parent scrolls.
    let second=render(Point::new(40.0,90.0),screen);
    let bright=|pixels:&[u8],x:usize,y:usize,width:usize,height:usize|->usize{
        (y..y+height).flat_map(|row|(x..x+width).map(move|column|(row*640+column)*4)).filter(|index|pixels[*index]>130&&pixels[*index+1]>130&&pixels[*index+2]>130).count()
    };
    let initial=bright(&first,40,430,200,40);let moved=bright(&second,40,90,200,40);
    assert!(bright(&first,40+b"fixture-shell$ visible prompt".len()*8,430,8,16)>5,"visible cursor must appear after the prompt");
    assert!(bright(&second,40+b"fixture-shell$ visible prompt".len()*8,90,8,16)>5,"cursor must move with the prompt");
    assert!(initial>100,"prompt/cursor pixels must appear in embedded pane, observed {initial}");
    assert_eq!(moved,initial,"cached cells must follow a scrolled parent without new bytes");
    assert_eq!(bright(&second,40,430,400,40),0,"old absolute location must not retain terminal text");
    // A viewport below the prompt clips it, rather than redrawing outside scroll bounds.
    let clipped=render(Point::new(40.0,90.0),Rectangle::new(Point::new(0.0,130.0),Size::new(640.0,400.0)));
    assert_eq!(bright(&clipped,40,90,400,40),0,"prompt pixels outside visible scroll bounds must be clipped");
}

/// Exercise actual widget events and rendered pixels without a native window or shell.
struct TerminalInteraction {
    terminal: Terminal,
    renderer: iced::Renderer,
    tree: iced::advanced::widget::Tree,
    node: iced::advanced::layout::Node,
    screen: iced::Rectangle,
}
impl TerminalInteraction {
    async fn new(bytes:&[u8])->Self {
        use iced::advanced::{layout,renderer::Headless,widget::Tree};
        use iced::{Font,Pixels,Point,Rectangle,Size};
        let renderer=<iced::Renderer as Headless>::new(Font::MONOSPACE,Pixels(14.0),None).await.expect("headless terminal interaction renderer");
        let mut terminal=Terminal::new_external(NEXT_WIDGET.fetch_add(1,Ordering::Relaxed),Default::default(),50,10);
        terminal.handle(Command::ProxyToBackend(iced_term::BackendCommand::Resize(Some(Size::new(400.0,160.0)),Some(Size::new(8.0,16.0)))));
        terminal.receive_output(bytes);
        let mut element=TerminalView::show(&terminal);let mut tree=Tree::new(element.as_widget());
        let node=element.as_widget_mut().layout(&mut tree,&renderer,&layout::Limits::new(Size::new(400.0,160.0),Size::new(400.0,160.0))).move_to(Point::new(40.0,90.0));
        drop(element);
        let mut fixture=Self{terminal,renderer,tree,node,screen:Rectangle::new(Point::ORIGIN,Size::new(640.0,400.0))};
        fixture.event(iced::mouse::Event::CursorMoved{position:Point::new(50.0,100.0)},Point::new(50.0,100.0));
        fixture
    }
    fn event(&mut self,event:iced::mouse::Event,position:iced::Point)->(Vec<iced_term::BackendCommand>,bool) {
        use iced::advanced::{clipboard,Layout,Shell};
        let mut messages=Vec::new();let mut element=TerminalView::show(&self.terminal);let captured;
        {
            let mut shell=Shell::new(&mut messages);
            element.as_widget_mut().update(&mut self.tree,&iced::Event::Mouse(event),Layout::new(&self.node),iced::mouse::Cursor::Available(position),&self.renderer,&mut clipboard::Null,&mut shell,&self.screen);
            captured=shell.is_event_captured();
        }
        drop(element);
        let commands:Vec<_>=messages.into_iter().map(|Event::BackendCall(_,command)|command).collect();
        for command in &commands {self.terminal.handle(Command::ProxyToBackend(command.clone()));}
        (commands,captured)
    }
    fn pixels(&mut self)->Vec<u8> {
        use iced::advanced::{Layout,renderer::{Headless,Renderer as _}};
        self.renderer.reset(self.screen);let element=TerminalView::show(&self.terminal);
        element.as_widget().draw(&self.tree,&mut self.renderer,&iced::Theme::Dark,&Default::default(),Layout::new(&self.node),iced::mouse::Cursor::Unavailable,&self.screen);
        <iced::Renderer as Headless>::screenshot(&mut self.renderer,iced::Size::new(640,400),1.0,iced::Color::BLACK)
    }
}

#[tokio::test]
async fn terminal_selection_stops_after_mouse_release_outside_the_pane() {
    use iced::{Point,mouse::{Button,Event as Mouse}};
    let mut fixture=TerminalInteraction::new(b"fixture selectable text remains readable\x1b[?25l").await;
    let initial=fixture.pixels();
    fixture.event(Mouse::ButtonPressed(Button::Left),Point::new(50.0,100.0));
    fixture.event(Mouse::CursorMoved{position:Point::new(160.0,100.0)},Point::new(160.0,100.0));
    let selected=fixture.pixels();assert_ne!(selected,initial,"deliberate drag must still visibly select text");
    fixture.event(Mouse::ButtonReleased(Button::Left),Point::new(600.0,300.0));
    let (commands,_)=fixture.event(Mouse::CursorMoved{position:Point::new(220.0,120.0)},Point::new(220.0,120.0));
    assert!(!commands.iter().any(|command|matches!(command,iced_term::BackendCommand::SelectUpdate(_))),"moving back into the terminal after release must not extend gray selection");
    assert_eq!(fixture.pixels(),selected,"released selection must stay bounded when the mouse moves without a pressed button");
}

#[tokio::test]
async fn terminal_trackpad_scroll_reveals_earlier_output_and_keeps_small_deltas_inside() {
    use iced::{Point,mouse::{Event as Mouse,ScrollDelta}};
    let mut history=String::new();
    for line in 0..50 {history.push_str(&format!("\x1b[{}m{}-{line:02}\x1b[0m\r\n",if line<40{31}else{32},if line<40{"earlier"}else{"latest"}));}
    history.push_str("\x1b[?25l");let mut fixture=TerminalInteraction::new(history.as_bytes()).await;
    let red_pixels=|pixels:&[u8]|pixels.chunks_exact(4).filter(|pixel|pixel[0]>110&&pixel[0]>pixel[1].saturating_add(35)&&pixel[0]>pixel[2].saturating_add(35)).count();
    let latest=fixture.pixels();assert_eq!(red_pixels(&latest),0,"earlier red output should begin above the visible terminal screen");
    let (commands,captured)=fixture.event(Mouse::WheelScrolled{delta:ScrollDelta::Pixels{x:0.0,y:1.0}},Point::new(100.0,140.0));
    assert!(captured,"a fractional trackpad movement over the terminal must not scroll the surrounding page");
    assert!(!commands.iter().any(|command|matches!(command,iced_term::BackendCommand::Scroll(delta) if *delta!=0)),"sub-line motion accumulates before moving terminal rows");
    assert_eq!(fixture.pixels(),latest,"a sub-line movement must retain the current screen");
    fixture.event(Mouse::WheelScrolled{delta:ScrollDelta::Pixels{x:0.0,y:128.0}},Point::new(100.0,140.0));
    let earlier=fixture.pixels();assert!(red_pixels(&earlier)>30,"upward trackpad movement must reveal earlier terminal output");
    fixture.terminal.receive_output(b"");
    assert_eq!(fixture.pixels(),earlier,"an empty output poll must not reset the user's scroll position");
    fixture.event(Mouse::WheelScrolled{delta:ScrollDelta::Pixels{x:0.0,y:-129.0}},Point::new(100.0,140.0));
    assert_eq!(red_pixels(&fixture.pixels()),0,"scrolling down must return toward the latest output");
}

#[tokio::test]
async fn terminal_click_and_drag_stay_on_the_clicked_row_after_outer_page_scroll() {
    use iced::advanced::{clipboard,layout,renderer::{Headless,Renderer as _},widget::{Tree,operation},Layout,Shell};
    use iced::{Element,Point,Size,mouse::{Button,Event as Mouse},widget::{column,container,scrollable,space}};
    const PAGE:&str="terminal-pointer-scrolled-page";
    fn page(terminal:&Terminal)->Element<'_,Event> {
        scrollable(column![space().height(600.0),container(TerminalView::show(terminal)).height(160.0),space().height(400.0)])
            .id(PAGE).width(500.0).height(320.0).into()
    }
    fn event(fixture:&mut TerminalInteraction,event:Mouse,position:Point)->Vec<iced_term::BackendCommand> {
        let mut messages=Vec::new();let mut element=page(&fixture.terminal);
        {
            let mut shell=Shell::new(&mut messages);
            element.as_widget_mut().update(&mut fixture.tree,&iced::Event::Mouse(event),Layout::new(&fixture.node),iced::mouse::Cursor::Available(position),&fixture.renderer,&mut clipboard::Null,&mut shell,&fixture.screen);
        }
        drop(element);
        let commands:Vec<_>=messages.into_iter().map(|Event::BackendCall(_,command)|command).collect();
        for command in &commands {fixture.terminal.handle(Command::ProxyToBackend(command.clone()));}
        commands
    }
    fn pixels(fixture:&mut TerminalInteraction)->Vec<u8> {
        fixture.renderer.reset(fixture.screen);let element=page(&fixture.terminal);
        element.as_widget().draw(&fixture.tree,&mut fixture.renderer,&iced::Theme::Dark,&Default::default(),Layout::new(&fixture.node),iced::mouse::Cursor::Unavailable,&fixture.screen);
        <iced::Renderer as Headless>::screenshot(&mut fixture.renderer,Size::new(640,400),1.0,iced::Color::BLACK)
    }
    let mut output=String::new();for row in 0..8 {output.push_str(&format!("visible fixture row {row}: selectable text\r\n"));}output.push_str("\x1b[?25l");
    let mut fixture=TerminalInteraction::new(output.as_bytes()).await;
    {
        let mut element=page(&fixture.terminal);fixture.tree=Tree::new(element.as_widget());
        fixture.node=element.as_widget_mut().layout(&mut fixture.tree,&fixture.renderer,&layout::Limits::new(Size::new(500.0,320.0),Size::new(500.0,320.0))).move_to(Point::new(40.0,30.0));
        let mut scroll=operation::scrollable::scroll_to::<()>(PAGE.into(),scrollable::AbsoluteOffset{x:None,y:Some(500.0)});
        element.as_widget_mut().operate(&mut fixture.tree,Layout::new(&fixture.node),&fixture.renderer,&mut scroll);
    }
    // The real parent scrollable translates Cursor for its child but leaves raw
    // CursorMoved event coordinates in window space. A tiny move must not jump rows.
    event(&mut fixture,Mouse::CursorMoved{position:Point::new(58.0,155.0)},Point::new(58.0,155.0));
    let baseline=pixels(&mut fixture);
    event(&mut fixture,Mouse::ButtonPressed(Button::Left),Point::new(58.0,155.0));
    let commands=event(&mut fixture,Mouse::CursorMoved{position:Point::new(59.0,156.0)},Point::new(59.0,156.0));
    event(&mut fixture,Mouse::ButtonReleased(Button::Left),Point::new(59.0,156.0));
    let locations:Vec<_>=commands.iter().filter_map(|command|if let iced_term::BackendCommand::SelectUpdate((_,y))=command{Some(*y)}else{None}).collect();
    assert_eq!(locations,vec![26.0],"the scrolled terminal must use parent-translated pointer coordinates");
    assert_eq!(pixels(&mut fixture),baseline,"a normal click with tiny movement must not select other terminal rows");
    // Deliberate selection still follows a short drag on the actual visible row.
    event(&mut fixture,Mouse::ButtonPressed(Button::Left),Point::new(90.0,200.0));
    event(&mut fixture,Mouse::CursorMoved{position:Point::new(150.0,200.0)},Point::new(150.0,200.0));
    event(&mut fixture,Mouse::ButtonReleased(Button::Left),Point::new(150.0,200.0));
    let selected=pixels(&mut fixture);let mut changed=0;
    for (index,(before,after)) in baseline.chunks_exact(4).zip(selected.chunks_exact(4)).enumerate() {
        if before!=after {changed+=1;let y=index/640;assert!((175..220).contains(&y),"short drag must not highlight unrelated rows: y={y}");}
    }
    assert!(changed>30,"a deliberate short drag must still visibly select text");
    assert!(changed<2000,"a short drag must not create a screen-sized gray selection");
    // A plain click elsewhere removes the intentional selection again.
    event(&mut fixture,Mouse::ButtonPressed(Button::Left),Point::new(250.0,240.0));
    event(&mut fixture,Mouse::ButtonReleased(Button::Left),Point::new(250.0,240.0));
    assert_eq!(pixels(&mut fixture),baseline,"plain click should clear the old highlight");
}
