//! Shared native controls; callers keep state, permissions and backend operations.
use super::{surface,theme};
use surface::{Button,TextInput};
use iced::{Element,Color,widget::{self,Checkbox,Container,Row,Text,column,row,text,svg}};

pub(super) fn button<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>)->Button<'a,M>{Button::new(content).padding(theme::BUTTON_PADDING).style(theme::button)}
pub(super) fn text_input<'a,M:Clone+'a>(placeholder:&str,value:&str)->TextInput<'a,M>{TextInput::new(placeholder,value)}
pub(super) fn checkbox<'a,M:Clone+'a>(selected:bool)->Checkbox<'a,M>{widget::checkbox(selected).size(theme::CHECKBOX_SIZE).spacing(theme::CHECKBOX_GAP).style(theme::checkbox)}
pub(super) fn installed_checkbox<'a,M:Clone+'a>()->Checkbox<'a,M>{checkbox(true).style(theme::installed_checkbox)}
pub(super) fn tab<'a,M:Clone+'a>(label:&str,selected:bool,action:Option<M>)->Button<'a,M>{button(text(label.to_owned()).size(14).font(theme::MEDIUM_FONT).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).style(if selected{theme::selected_button}else{theme::button}).on_press_maybe(action)}
pub(super) fn output_tab<'a,M:Clone+'a>(label:&str,selected:bool,action:Option<M>)->Button<'a,M>{button(text(label.to_owned()).size(14).font(theme::MEDIUM_FONT).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).style(move|_,status|theme::control(Color::from_rgb8(227,230,233),selected,status)).on_press_maybe(action)}
pub(super) fn navigation<'a,M:Clone+'a>(actions:[M;3],active:usize)->Container<'a,M>{
    let labels=["Servers","Install MCPs","Connect apps"];
    let mut content=row![text("Fetch").size(theme::APP_TITLE).font(theme::SEMIBOLD_FONT).line_height(iced::widget::text::LineHeight::Absolute(40.0.into()))].spacing(theme::NAVIGATION_GAP).align_y(iced::Center);
    for (index,action) in actions.into_iter().enumerate(){
        content=content.push(button(text(labels[index]).size(14).font(if active==index{theme::BOLD_FONT}else{theme::MEDIUM_FONT}).line_height(iced::widget::text::LineHeight::Absolute(20.0.into()))).on_press(action).style(move|_,status|{
            let mut style=theme::control(if active==index{theme::ACCENT}else{theme::SURFACE},active==index,status);
            style.text_color=if active==index{Color::WHITE}else{theme::INK};style.border.radius=12.0.into();style
        }));
    }
    raised(widget::container(content).padding([16,16]).width(iced::Fill)).width(iced::Fill)
}
pub(super) fn raised<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,theme::SURFACE,24.0,false))}
pub(super) fn raised_with_radius<'a,M:'a>(content:impl Into<Element<'a,M>>,radius:f32)->Container<'a,M>{widget::container(surface::wrap(content,theme::SURFACE,radius,false))}
pub(super) fn input_surface<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,Color::from_rgb8(238,239,239),12.0,true))}
pub(super) fn recessed<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(surface::wrap(content,theme::RECESSED,24.0,true))}
pub(super) fn read_only<'a,M:'a>(value:String,height:f32)->Container<'a,M>{
    widget::container(surface::wrap(widget::container(text(value).size(14).width(iced::Fill).wrapping(widget::text::Wrapping::Glyph)).padding([if height==96.0{16.0}else{12.0},12.0]).width(iced::Fill).height(height),Color::from_rgb8(234,245,251),12.0,false)).width(iced::Fill)
}
pub(super) fn page<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{
    widget::container(surface::wrap(widget::container(content).padding(theme::PAGE_PADDING).width(iced::Fill).height(iced::Fill),theme::SURFACE,32.0,false)).padding(8).width(iced::Fill).height(iced::Fill)
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
pub(super) fn icon<'a,M:'a>(kind:Icon)->Element<'a,M>{
    let color=match kind{Icon::Key|Icon::Refresh|Icon::Close=>Color::WHITE,Icon::Copy=>theme::ACCENT,_=>theme::MUTED};
    let svg=svg(svg::Handle::from_memory(asset(kind))).style(move|_,_|svg::Style{color:Some(color)});
    match kind{Icon::Refresh=>svg.width(18).height(18).content_fit(iced::ContentFit::Contain).into(),Icon::Close=>svg.width(18.2).height(18.2).content_fit(iced::ContentFit::Contain).into(),_=>svg.width(iced::Shrink).height(iced::Shrink).into()}
}
pub(super) fn icon_button<'a,M:Clone+'a>(kind:Icon,action:Option<M>)->Button<'a,M>{
    let color=match kind{Icon::Refresh=>theme::ACCENT,Icon::Key if action.is_none()=>Color::from_rgb8(184,189,195),Icon::Key=>Color::from_rgb8(29,32,35),Icon::Close=>theme::DANGER,_=>theme::SURFACE};
    let glyph=widget::container(icon(kind)).center_x(44).center_y(44);
    let glyph=if matches!(kind,Icon::Close){surface::bold_on_hover(glyph)}else{glyph.into()};
    Button::new(glyph).padding(0).width(44).height(44).on_press_maybe(action).style(move|_,status|{
        let mut style=theme::control(color,false,status);style.border.radius=99.0.into();style.text_color=Color::WHITE;
        if matches!(kind,Icon::Close)&&status==widget::button::Status::Disabled{style.background=Some(theme::convex(theme::DANGER,false));}
        style
    })
}
fn copy_button<'a,M:Clone+'a>(copy:M,dark:bool)->Button<'a,M>{
    let glyph=svg(svg::Handle::from_memory(asset(Icon::Copy))).width(iced::Shrink).height(iced::Shrink).style(move|_,_|svg::Style{color:Some(if dark{Color::from_rgb8(222,226,232)}else{theme::ACCENT})});
    Button::new(widget::container(glyph).center_x(40).center_y(40)).padding(0).style(theme::transparent).on_press(copy)
}
pub(super) fn code_block<'a,M:Clone+'a>(commands:String,copy:impl FnOnce(String)->M)->Container<'a,M>{
    let message=copy(commands.clone());
    widget::container(surface::wrap(widget::container(row![console_line(commands).width(iced::Fill),copy_button(message,false)].spacing(theme::CONTROL_GAP).align_y(iced::Center)).padding(16).width(iced::Fill),Color::from_rgb8(238,239,239),16.0,true)).width(iced::Fill)
}
pub(super) fn console<'a,M:Clone+'a>(content:impl Into<Element<'a,M>>,copy:M)->Container<'a,M>{
    let header=row![widget::Space::new().width(iced::Fill),copy_button(copy,true)];
    widget::container(column![header,widget::scrollable(content).style(theme::scrollable).height(iced::Fill).anchor_bottom()]).padding(theme::MESSAGE_PADDING).width(iced::Fill).height(theme::CONSOLE_HEIGHT).style(theme::console)
}
pub(super) fn output_controls<'a,M:Clone+'a>(console:M,terminal:Option<M>)->Row<'a,M>{row![button("Console").style(theme::output_button).on_press(console),button("Terminal").style(theme::output_button).on_press_maybe(terminal)].spacing(theme::CONTROL_GAP)}
pub(super) fn terminal_pane<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{widget::container(content).width(iced::Fill).height(theme::CONSOLE_HEIGHT).style(theme::console)}
pub(super) fn server_card<'a,M:'a>(content:impl Into<Element<'a,M>>)->Container<'a,M>{raised(widget::container(content).padding(theme::CARD_PADDING).width(iced::Fill)).width(iced::Fill)}
pub(super) fn disclosure<'a,M:Clone+'a>(title:&str,expanded:bool,size:f32,toggle:M)->Button<'a,M>{
    button(row![icon(if expanded{Icon::ChevronDown}else{Icon::ChevronRight}),text(title.to_owned()).size(size)].spacing(10).align_y(iced::Center)).style(theme::transparent).on_press(toggle)
}
pub(super) fn tool_disclosure<'a,M:Clone+'a>(name:&str,expanded:bool,raw_definition:Option<String>,toggle:M)->Container<'a,M>{
    let mut content=column![disclosure(name,expanded,theme::TOOL_TITLE,toggle)];
    if let Some(definition)=raw_definition{content=content.push(text(definition).size(theme::SMALL_TEXT).font(theme::MONO_FONT));}
    widget::container(content).padding(theme::PANEL_PADDING)
}
pub(super) fn server_identity<'a,M:Clone+'a>(name:&str,runtime:&str,auth:&str,expanded:bool,toggle:M)->Button<'a,M>{
    let dot_color=match runtime{"ready"=>Color::from_rgb8(119,173,138),"starting"|"restarting"=>Color::from_rgb8(226,184,79),"failed"=>Color::from_rgb8(199,107,107),_=>Color::from_rgb8(156,163,170)};
    let dot_asset=if runtime=="ready"{include_bytes!("assets/67c29.svg").as_slice()}else{include_bytes!("assets/97eb5.svg").as_slice()};
    let dot=svg(svg::Handle::from_memory(dot_asset)).width(iced::Shrink).height(iced::Shrink).style(move|_,_|svg::Style{color:Some(dot_color)});
    let mut content=row![widget::container(icon(if expanded{Icon::ChevronDown}else{Icon::ChevronRight})).center_x(28).center_y(44),text(name.to_owned()).size(theme::SERVER_TITLE).font(theme::BOLD_FONT).line_height(iced::Pixels(24.0)),dot].spacing(10).align_y(iced::Center);
    if !matches!(auth,"authenticated"|"not_required"){
        let warning=match auth{"required"=>"Sign-in required","failed"=>"Sign-in failed","authenticating"=>"Signing in…",_=>"Sign-in status unknown"};
        content=content.push(svg(svg::Handle::from_memory(include_bytes!("assets/2a1d5.svg").as_slice())).width(18).height(18).content_fit(iced::ContentFit::Contain).style(|_,_|svg::Style{color:Some(Color::from_rgb8(165,125,37))})).push(text(warning).size(13).line_height(iced::Pixels(20.0)).color(Color::from_rgb8(165,125,37)));
    }
    button(content).padding(0).width(iced::Fill).style(theme::transparent).on_press(toggle)
}
// Kept for callers outside the server list while the same shared header is reused.
pub(super) fn server_header<'a,M:Clone+'a>(title:&str,expanded:bool,toggle:M,delete:Option<M>,delete_hint:&str)->Row<'a,M>{
    row![disclosure(title,expanded,theme::SERVER_TITLE,toggle).width(iced::Fill),widget::tooltip(icon_button(Icon::Close,delete),text(delete_hint.to_owned()),widget::tooltip::Position::Top)].spacing(theme::CONTROL_GAP).align_y(iced::Center)
}
