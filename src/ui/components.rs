//! Shared native controls; callers keep state, permissions and backend operations.
use super::{surface,theme};
use surface::{Button,TextInput};
use iced::{Element,Color,widget::{self,Checkbox,Container,Row,Text,column,row,text,svg,text_editor,markdown}};

pub(super) fn button<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>)->Button<'a,M>{Button::new(content).padding(theme::BUTTON_PADDING).style(theme::button)}
/// Text actions share typography as well as native hover/press/disabled behavior.
pub(super) fn text_button<'a,M:Clone+'a>(label:impl Into<String>)->Button<'a,M>{
    button(text(label.into()).size(14).font(theme::BODY_FONT).line_height(iced::Pixels(22.0)))
}
pub(super) fn primary_button<'a,M:Clone+'a>(label:impl Into<String>)->Button<'a,M>{
    button(text(label.into()).size(14).font(theme::BOLD_FONT).line_height(iced::Pixels(22.0))).style(theme::install_button)
}
pub(super) fn text_input<'a,M:Clone+'a>(placeholder:&str,value:&str)->TextInput<'a,M>{TextInput::new(placeholder,value)}
pub(super) fn checkbox<'a,M:Clone+'a>(selected:bool)->Checkbox<'a,M>{widget::checkbox(selected).size(theme::CHECKBOX_SIZE).spacing(theme::CHECKBOX_GAP).style(theme::checkbox)}
pub(super) fn server_toggle<'a,M:Clone+'a>(enabled:bool,action:Option<impl Fn(bool)->M+'a>)->Container<'a,M>{
    widget::container(widget::toggler(enabled).size(22).style(theme::server_toggle).on_toggle_maybe(action)).width(44).height(22)
}
pub(super) fn installed_checkbox<'a,M:Clone+'a>()->Checkbox<'a,M>{checkbox(true).style(theme::installed_checkbox)}
pub(super) fn tab<'a,M:Clone+'a>(label:&str,selected:bool,action:Option<M>)->Button<'a,M>{button(text(label.to_owned()).size(14).font(theme::MEDIUM_FONT).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).style(if selected{theme::selected_button}else{theme::button}).on_press_maybe(action)}
pub(super) fn output_tab<'a,M:Clone+'a>(label:&str,selected:bool,action:Option<M>)->Button<'a,M>{button(text(label.to_owned()).size(14).font(theme::MEDIUM_FONT).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).style(move|_,status|theme::control(Color::from_rgb8(227,230,233),selected,status)).on_press_maybe(action)}
pub(super) fn navigation<'a,M:Clone+'a>(actions:[M;4],active:usize)->Container<'a,M>{
    let labels=["Servers","Install MCPs","Connect apps","Workspace"];
    let mut content=row![text("Fetch").size(theme::APP_TITLE).font(theme::SEMIBOLD_FONT).line_height(iced::widget::text::LineHeight::Absolute(40.0.into()))].spacing(theme::NAVIGATION_GAP).align_y(iced::Center);
    for (index,action) in actions.into_iter().enumerate(){
        content=content.push(button(text(labels[index]).size(14).font(if active==index{theme::BOLD_FONT}else{theme::MEDIUM_FONT}).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).on_press(action).style(move|_,status|{
            let mut style=theme::control(if active==index{theme::ACCENT}else{theme::SURFACE},active==index,status);
            style.text_color=if active==index{Color::WHITE}else{theme::INK};style.border.radius=12.0.into();style
        }));
    }
    raised(widget::container(content).padding([16,16]).width(iced::Fill)).width(iced::Fill)
}
/// Equal columns without Grid's viewport clamp, so card shadows reach the scroll padding.
pub(super) fn tool_grid<'a,M:'a>(cards:Vec<Element<'a,M>>)->widget::Column<'a,M>{
    let mut cards=cards.into_iter();
    let mut rows=column![].spacing(24).width(iced::Fill);
    while let Some(first)=cards.next(){
        let mut line=row![first].spacing(24).width(iced::Fill);
        for _ in 0..2{line=line.push(cards.next().unwrap_or_else(||widget::Space::new().width(iced::Fill).into()));}
        rows=rows.push(line);
    }
    rows
}
pub(super) fn raised<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,theme::SURFACE,24.0,false))}
pub(super) fn conversation_message<'a,M:'a>(content:impl Into<Element<'a,M>>,user:bool)->Container<'a,M>{
    let color=if user{theme::SESSION_USER_BACKGROUND}else{theme::SESSION_ASSISTANT_BACKGROUND};
    widget::container(surface::wrap(content,color,24.0,false))
}
pub(super) fn raised_with_radius<'a,M:'a>(content:impl Into<Element<'a,M>>,radius:f32)->Container<'a,M>{widget::container(surface::wrap(content,theme::SURFACE,radius,false))}
pub(super) fn input_surface<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,theme::CODE_BACKGROUND,12.0,true))}
/// Native file text shares typography, padding and scrolling in edit and preview.
pub(super) fn file_content<'a,M:Clone+'a>(content:&'a text_editor::Content,on_action:impl Fn(text_editor::Action)->M+'a,undo:Option<M>)->Container<'a,M>{
    input_surface(surface::contain_scroll(text_editor(content).font(theme::MONO_FONT).size(12).padding(theme::INPUT_PADDING).style(theme::text_editor).height(240).key_binding(move|press|{
        if let Some(undo)=undo.as_ref(){
            if matches!(press.status,text_editor::Status::Focused{..})&&press.key.to_latin(press.physical_key)==Some('z')&&(press.modifiers.command()||press.modifiers.control())&&!press.modifiers.shift()&&!press.modifiers.alt(){return Some(text_editor::Binding::Custom(undo.clone()));}
        }
        text_editor::Binding::from_key_press(press)
    }).on_action(on_action))).width(iced::Fill)
}
pub(super) fn recessed<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,theme::RECESSED,24.0,true))}
pub(super) fn read_only<'a,M:'a>(value:String,height:f32)->Container<'a,M>{
    widget::container(surface::wrap(widget::container(text(value).size(12).width(iced::Fill).wrapping(widget::text::Wrapping::Glyph)).padding(8).width(iced::Fill).height(height),Color::from_rgb8(234,245,251),12.0,false)).width(iced::Fill)
}
pub(super) fn page<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{
    widget::container(surface::window(widget::container(content).padding(theme::PAGE_PADDING+8.0).width(iced::Fill).height(iced::Fill))).width(iced::Fill).height(iced::Fill)
}
/// Shared popup backdrop follows the rounded app face; only outside clicks dismiss.
pub(super) fn modal<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,dismiss:M)->Element<'a,M>{
    let shade=widget::mouse_area(widget::container(widget::Space::new().width(iced::Fill).height(iced::Fill))
        .width(iced::Fill).height(iced::Fill).style(|_|widget::container::Style{
            background:Some(Color::from_rgba(0.15,0.17,0.18,0.18).into()),
            border:iced::Border{radius:theme::WINDOW_RADIUS.into(),..Default::default()},..Default::default()
        })).on_press(dismiss);
    let dialog=widget::container(widget::opaque(content)).padding(iced::Padding{top:48.0,right:32.0,bottom:32.0,left:32.0}).center_x(iced::Fill).center_y(iced::Fill);
    widget::opaque(widget::stack![shade,dialog]).into()
}
/// Dim a pending list while keeping its existing rows in place.
pub(super) fn loading_panel<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,loading:bool)->Element<'a,M>{
    let mut layers=widget::stack![content.into()];
    if loading{layers=layers.push(widget::opaque(widget::container(widget::Space::new().width(iced::Fill).height(iced::Fill)).style(|_|widget::container::Style{background:Some(Color::from_rgb8(190,190,188).scale_alpha(0.30).into()),border:iced::Border{radius:24.0.into(),..Default::default()},..Default::default()})));}
    layers.into()
}
pub(super) fn entry_card<'a,M:Clone+'a>(label:String,detail:String,open:Option<M>,copy:Option<M>,primary:bool,reveal:Option<M>,copy_path:Option<M>)->Container<'a,M>{
    let action=if primary{primary_button(label)}else{text_button(label)};
    let mut actions=row![action.on_press_maybe(open).width(iced::Fill)].spacing(12).align_y(iced::Center);
    if let Some(copy)=copy{actions=actions.push(copy_button(copy,false));}
    let detail:Element<'a,M>=if let Some(copy_path)=copy_path{file_path(detail,reveal,copy_path)}else{text(detail).size(11).color(theme::MUTED).into()};
    raised(widget::container(column![actions,detail].spacing(6)).padding(14).width(iced::Fill))
}
/// A compact raised Finder-face control; callers invoke the authorized file API.
pub(super) fn finder_button<'a,M:Clone+'a>(action:Option<M>)->Element<'a,M>{
    const FINDER:&[u8]=br#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><g fill='none' stroke='currentColor' stroke-width='1.6' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='3' width='18' height='18' rx='3'/><path d='M13 3l-2 10h4l-1 8M7 8v2m10-2v2M7 16c3 2 7 2 10-1'/></g></svg>"#;
    let glyph=svg(svg::Handle::from_memory(FINDER)).width(17).height(17).style(|_,_|svg::Style{color:Some(theme::ACCENT)});
    Button::new(widget::container(glyph).center_x(28).center_y(28)).padding(0).style(theme::button).on_press_maybe(action).into()
}
pub(super) fn file_path<'a,M:Clone+'a>(path:String,reveal:Option<M>,copy:M)->Element<'a,M>{
    widget::responsive(move|size|{
        let text_width=(size.width-28.0-8.0-24.0-28.0-theme::CONTROL_GAP).max(1.0);
        let path_text=widget::container(console_line(path.clone()).wrapping(widget::text::Wrapping::Glyph)).max_width(text_width);
        let code=code_box(row![path_text,frameless_copy_at(copy.clone(),14.0,28.0)].spacing(theme::CONTROL_GAP).align_y(iced::Center),[8.0,12.0]);
        row![code,finder_button(reveal.clone())].spacing(8).align_y(iced::Center).into()
    }).height(iced::Shrink).into()
}
pub(super) fn message<'a>(content:impl widget::text::IntoFragment<'a>)->Text<'a>{text(content).size(14)}
pub(super) fn error_message<'a,M:'a>(content:impl widget::text::IntoFragment<'a>)->Container<'a,M>{widget::container(message(content).color(theme::DANGER)).padding(theme::MESSAGE_PADDING)}
pub(super) fn console_line<'a>(content:impl widget::text::IntoFragment<'a>)->Text<'a>{text(content).font(theme::MONO_FONT).size(theme::CONSOLE_TEXT_SIZE)}

#[derive(Clone,Copy)]
pub(super) enum Icon{Refresh,Key,Close,ChevronRight,ChevronDown,Copy,Up,Down}
fn asset(icon:Icon)->&'static [u8]{match icon{
    Icon::Refresh=>include_bytes!("assets/refresh.svg"),Icon::Key=>include_bytes!("assets/66300.svg"),Icon::Close=>include_bytes!("assets/close.svg"),
    Icon::ChevronRight=>include_bytes!("assets/bed01.svg"),Icon::ChevronDown=>include_bytes!("assets/11619.svg"),Icon::Copy=>include_bytes!("assets/ccd0d.svg"),
    Icon::Up=>include_bytes!("assets/6ef1b.svg"),Icon::Down=>include_bytes!("assets/83aeb.svg"),
}}
pub(super) fn icon<'a,M:'a>(kind:Icon)->Element<'a,M>{icon_at_size(kind,None)}
fn icon_at_size<'a,M:'a>(kind:Icon,size:Option<f32>)->Element<'a,M>{
    let color=match kind{Icon::Key|Icon::Refresh|Icon::Close=>Color::WHITE,Icon::Copy=>theme::ACCENT,_=>theme::MUTED};
    let svg=svg(svg::Handle::from_memory(asset(kind))).style(move|_,_|svg::Style{color:Some(color)});
    if let Some(size)=size{return svg.width(size).height(size).content_fit(iced::ContentFit::Contain).into();}
    match kind{Icon::Refresh=>svg.width(18).height(18).content_fit(iced::ContentFit::Contain).into(),Icon::Close=>svg.width(18.2).height(18.2).content_fit(iced::ContentFit::Contain).into(),_=>svg.width(iced::Shrink).height(iced::Shrink).into()}
}
pub(super) fn icon_button<'a,M:Clone+'a>(kind:Icon,action:Option<M>)->Button<'a,M>{
    icon_button_with_activity(kind,action,false,44.0)
}
pub(super) fn refresh_button<'a,M:Clone+'a>(action:Option<M>,running:bool)->Button<'a,M>{
    icon_button_with_activity(Icon::Refresh,action.filter(|_|!running),running,44.0)
}
pub(super) fn app_auth_button<'a,M:Clone+'a>(action:Option<M>)->Button<'a,M>{
    icon_button_with_activity(Icon::Key,action,false,theme::APP_LOGO_SIZE).style(|_,status|{
        let mut style=theme::control(Color::BLACK,false,status);style.border.radius=99.0.into();style.text_color=Color::WHITE;style
    })
}
fn icon_button_with_activity<'a,M:Clone+'a>(kind:Icon,action:Option<M>,running:bool,size:f32)->Button<'a,M>{
    let color=match kind{Icon::Refresh=>theme::ACCENT,Icon::Key=>Color::from_rgb8(184,189,195),Icon::Close=>theme::DANGER,_=>theme::SURFACE};
    let glyph=widget::container(if matches!(kind,Icon::Refresh){surface::refresh_glyph(action.is_some(),running)}else{icon_at_size(kind,(size!=44.0).then_some(20.0*size/44.0))}).center_x(size).center_y(size);
    let glyph=if matches!(kind,Icon::Close){surface::bold_on_hover(glyph)}else{glyph.into()};
    let control=Button::new(glyph);
    let control=if matches!(kind,Icon::Refresh|Icon::Key){control.strong_hover()}else{control};
    control.padding(0).width(size).height(size).on_press_maybe(action).style(move|_,status|{
        let mut style=theme::control(color,false,status);style.border.radius=99.0.into();style.text_color=Color::WHITE;
        if matches!(kind,Icon::Close)&&status==widget::button::Status::Disabled{style.background=Some(theme::convex(theme::DANGER,false));}
        style
    })
}
pub(super) fn copy_button<'a,M:Clone+'a>(copy:M,dark:bool)->Element<'a,M>{
    let glyph=svg(svg::Handle::from_memory(asset(Icon::Copy))).width(iced::Shrink).height(iced::Shrink).style(move|_,status|svg::Style{color:Some(if dark{if status==svg::Status::Hovered{Color::WHITE}else{Color::from_rgb8(222,226,232)}}else if status==svg::Status::Hovered{Color::from_rgb8(194,232,250)}else{theme::ACCENT})});
    super::copy_feedback::wrap(Button::new(widget::container(glyph).center_x(40).center_y(40)).padding(0).style(if dark{theme::transparent}else{theme::button}).on_press(copy),dark)
}
/// A blue Copy glyph with a transparent hit area and no raised face.
pub(super) fn frameless_copy<'a,M:Clone+'a>(copy:M)->Element<'a,M>{
    frameless_copy_at(copy,20.0,40.0)
}
fn frameless_copy_at<'a,M:Clone+'a>(copy:M,glyph_size:f32,hit_size:f32)->Element<'a,M>{
    let glyph=svg(svg::Handle::from_memory(asset(Icon::Copy))).width(glyph_size).height(glyph_size).style(|_,_|svg::Style{color:Some(theme::ACCENT)});
    widget::button(widget::container(glyph).center_x(hit_size).center_y(hit_size)).padding(0).style(theme::transparent).on_press(copy).into()
}
pub(super) fn app_heading<'a,M:'a>(label:&str,logo:&'static [u8])->Row<'a,M>{
    row![text(label.to_owned()).size(18).font(theme::BOLD_FONT).color(theme::MUTED).line_height(iced::Pixels(24.0)),svg(svg::Handle::from_memory(logo)).width(theme::APP_LOGO_SIZE).height(theme::APP_LOGO_SIZE).content_fit(iced::ContentFit::Contain)].spacing(8).align_y(iced::Center)
}
/// Circular app choice: raised when idle, inset when selected or pressed.
pub(super) fn app_logo_button<'a,M:Clone+'a>(logo:&'static [u8],selected:bool,action:M)->Button<'a,M>{
    let glyph=svg(svg::Handle::from_memory(logo)).width(theme::APP_LOGO_SIZE).height(theme::APP_LOGO_SIZE).content_fit(iced::ContentFit::Contain);
    button(widget::container(glyph).center_x(40).center_y(40)).padding(0).width(40).height(40).on_press(action).style(move|_,status|{
        let pressed=selected||status==widget::button::Status::Pressed;
        let mut style=theme::control(if pressed{theme::RECESSED}else{theme::SURFACE},pressed,status);style.border.radius=99.0.into();style
    })
}
pub(super) fn code_block<'a,M:Clone+'a>(commands:String,copy:impl FnOnce(String)->M)->Container<'a,M>{
    let message=copy(commands.clone());
    code_box(row![console_line(commands).width(iced::Fill),copy_button(message,false)].spacing(theme::CONTROL_GAP).align_y(iced::Center).width(iced::Fill),16).width(iced::Fill)
}
pub(super) fn code_block_without_copy<'a,M:'a>(value:String)->Container<'a,M>{
    code_box(console_line(value).width(iced::Fill).wrapping(widget::text::Wrapping::Glyph),16).width(iced::Fill)
}
fn code_box<'a,M:'a>(content:impl Into<Element<'a,M>>,padding:impl Into<iced::Padding>)->Container<'a,M>{
    widget::container(surface::wrap(widget::container(content).padding(padding),theme::CODE_BACKGROUND,16.0,true))
}
/// Preserve Markdown structure while sharing the application's light code style.
pub(super) fn session_markdown<'a>(items:&'a [markdown::Item])->Element<'a,markdown::Uri>{
    let mut style=markdown::Style::from(theme::light());
    style.font=theme::BODY_FONT;
    style.inline_code_font=theme::BODY_FONT;
    style.inline_code_color=theme::INK;
    style.inline_code_highlight.background=theme::CODE_BACKGROUND.into();
    markdown::view_with(items,markdown::Settings::with_text_size(14,style),&SessionMarkdown)
}
struct SessionMarkdown;
impl<'a> markdown::Viewer<'a,markdown::Uri> for SessionMarkdown {
    fn on_link_click(url:markdown::Uri)->markdown::Uri{url}
    fn code_block(&self,_settings:markdown::Settings,_language:Option<&'a str>,code:&'a str,_lines:&'a [markdown::Text])->Element<'a,markdown::Uri>{
        code_block_without_copy(code.to_owned()).into()
    }
}
pub(super) fn console<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,copy:M)->Container<'a,M>{
    output_frame(widget::scrollable(content).style(theme::scrollable).height(iced::Fill).anchor_bottom(),Some(copy))
}
fn output_frame<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,copy:Option<M>)->Container<'a,M>{
    let mut header=row![widget::Space::new().width(iced::Fill)].height(40);
    if let Some(copy)=copy{header=header.push(copy_button(copy,true));}
    widget::container(surface::contain_scroll(widget::container(column![header,widget::container(content).width(iced::Fill).height(iced::Fill)]).padding(theme::MESSAGE_PADDING).width(iced::Fill).height(theme::CONSOLE_HEIGHT).style(theme::console)))
}
pub(super) fn output_controls<'a,M:Clone+'a>(console:M,terminal:Option<M>)->Row<'a,M>{row![text_button("Console").style(theme::output_button).on_press(console),text_button("Terminal").style(theme::output_button).on_press_maybe(terminal)].spacing(theme::CONTROL_GAP)}
pub(super) fn terminal_pane<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,copy:Option<M>)->Container<'a,M>{output_frame(content,copy)}
pub(super) fn server_card<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{raised(widget::container(content).padding(theme::CARD_PADDING).width(iced::Fill)).width(iced::Fill)}
pub(super) fn disclosure<'a,M:Clone+'a>(title:&str,expanded:bool,size:f32,toggle:M)->Button<'a,M>{
    button(row![icon(if expanded{Icon::ChevronDown}else{Icon::ChevronRight}),text(title.to_owned()).size(size)].spacing(10).align_y(iced::Center)).style(theme::transparent).on_press(toggle)
}
pub(super) fn code_disclosure<'a,M:Clone+'a>(title:&str,expanded:bool,toggle:M)->Button<'a,M>{
    let name=code_box(console_line(title.to_owned()).width(iced::Fill).wrapping(widget::text::Wrapping::Glyph),[8.0,12.0]).width(iced::Fill);
    let content=row![icon_at_size(if expanded{Icon::ChevronDown}else{Icon::ChevronRight},Some(18.0)),name].spacing(10).align_y(iced::Center);
    button(content).padding(0).style(theme::transparent).on_press(toggle)
}
pub(super) fn tool_disclosure<'a,M:Clone+'a>(name:&str,expanded:bool,raw_definition:Option<String>,toggle:M)->Container<'a,M>{
    let mut content=column![disclosure(name,expanded,theme::TOOL_TITLE,toggle)];
    if let Some(definition)=raw_definition{content=content.push(text(definition).size(theme::SMALL_TEXT).font(theme::MONO_FONT));}
    widget::container(content).padding(theme::PANEL_PADDING)
}
pub(super) fn server_identity<'a,M:Clone+'a>(name:&str,runtime:&str,auth:&str,restart_failed:bool,expanded:bool,toggle:M)->Button<'a,M>{
    let dot_color=match runtime{"ready"=>Color::from_rgb8(119,173,138),"starting"|"restarting"=>Color::from_rgb8(229,163,72),"failed"=>Color::from_rgb8(199,107,107),_=>Color::from_rgb8(156,163,170)};
    let dot_asset=if runtime=="ready"{include_bytes!("assets/67c29.svg").as_slice()}else{include_bytes!("assets/97eb5.svg").as_slice()};
    let dot=svg(svg::Handle::from_memory(dot_asset)).width(iced::Shrink).height(iced::Shrink).style(move|_,_|svg::Style{color:Some(dot_color)});
    let mut content=row![widget::container(icon(if expanded{Icon::ChevronDown}else{Icon::ChevronRight})).center_x(28).center_y(44),text(name.to_owned()).size(theme::SERVER_TITLE).font(theme::BOLD_FONT).line_height(iced::Pixels(24.0)),dot].spacing(10).align_y(iced::Center);
    let warning=if restart_failed{Some("Restart failed")}else if !matches!(auth,"authenticated"|"not_required")&&!(auth=="unknown"&&matches!(runtime,"starting"|"restarting")){
        Some(match auth{"required"=>"Sign-in required","failed"=>"Sign-in failed","authenticating"=>"Signing in…",_=>"Sign-in status unknown"})
    }else{None};
    if let Some(warning)=warning{
        content=content.push(svg(svg::Handle::from_memory(include_bytes!("assets/2a1d5.svg").as_slice())).width(18).height(18).content_fit(iced::ContentFit::Contain).style(|_,_|svg::Style{color:Some(Color::from_rgb8(165,125,37))})).push(text(warning).size(13).line_height(iced::Pixels(20.0)).color(Color::from_rgb8(165,125,37)));
    }
    button(content).padding(0).width(iced::Fill).style(theme::transparent).on_press(toggle)
}
// Kept for callers outside the server list while the same shared header is reused.
pub(super) fn server_header<'a,M:Clone+'a>(title:&str,expanded:bool,toggle:M,delete:Option<M>,_delete_hint:&str)->Row<'a,M>{
    row![disclosure(title,expanded,theme::SERVER_TITLE,toggle).width(iced::Fill),icon_button(Icon::Close,delete)].spacing(theme::CONTROL_GAP).align_y(iced::Center)
}
