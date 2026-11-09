//! Shared colors and control styles from the approved Fetch neumorphic design.
use iced::{Background, Border, Color, Theme, widget};

pub(super) const BODY_FONT: iced::Font = iced::Font::with_name("Inter");
pub(super) const MONO_FONT: iced::Font = iced::Font::with_name("Roboto Mono");
pub(super) const MEDIUM_FONT: iced::Font = iced::Font {weight:iced::font::Weight::Medium,..BODY_FONT};
pub(super) const SEMIBOLD_FONT: iced::Font = iced::Font {weight:iced::font::Weight::Semibold,..BODY_FONT};
pub(super) const BOLD_FONT: iced::Font = iced::Font {weight:iced::font::Weight::Bold,..BODY_FONT};
pub(super) const SURFACE: Color = Color::from_rgb8(247,246,241);
pub(super) const RECESSED: Color = Color::from_rgb8(234,230,223);
pub(super) const ACCENT: Color = Color::from_rgb8(149,212,241);
pub(super) const INK: Color = Color::from_rgb8(39,43,48);
pub(super) const MUTED: Color = Color::from_rgb8(102,107,112);
pub(super) const DANGER: Color = Color::from_rgb8(198,69,69);
pub(super) const APP_TITLE: f32 = 28.0;
pub(super) const PAGE_TITLE: f32 = 32.0;
pub(super) const SECTION_TITLE: f32 = 22.0;
pub(super) const FIELD_TITLE: f32 = 17.0;
pub(super) const SERVER_TITLE: f32 = 18.0;
pub(super) const TOOL_TITLE: f32 = 17.0;
pub(super) const SMALL_TEXT: f32 = 12.0;
pub(super) const CONTROL_GAP: f32 = 10.0;
pub(super) const NAVIGATION_GAP: f32 = 16.0;
pub(super) const PAGE_GAP: f32 = 20.0;
pub(super) const CARD_GAP: f32 = 20.0;
pub(super) const SECTION_GAP: f32 = 32.0;
pub(super) const PAGE_PADDING: f32 = 16.0;
pub(super) const CARD_PADDING: f32 = 24.0;
pub(super) const MESSAGE_PADDING: f32 = 16.0;
pub(super) const PANEL_PADDING: f32 = 24.0;
pub(super) const BUTTON_PADDING: [f32;2] = [10.0,16.0];
pub(super) const DELETE_BUTTON_PADDING: [f32;2] = [10.0,10.0];
pub(super) const DELETE_ICON_SIZE: f32 = 18.0;
pub(super) const INPUT_PADDING: f32 = 12.0;
pub(super) const CHECKBOX_SIZE: f32 = 18.0;
pub(super) const CHECKBOX_GAP: f32 = 8.0;
pub(super) const INSTALLED_LABEL_ALPHA: f32 = 0.5;
pub(super) const CONSOLE_HEIGHT: f32 = 240.0;
pub(super) const CONSOLE_TEXT_SIZE: f32 = 13.0;
pub(super) const CONSOLE_LINE_GAP: f32 = 4.0;

pub(super) fn light() -> Theme {
    Theme::custom("Fetch warm white", iced::theme::Palette {background:SURFACE,text:INK,primary:ACCENT,success:Color::from_rgb8(119,173,138),danger:DANGER,warning:Color::from_rgb8(165,125,37)})
}
pub(super) fn muted_text(_: &Theme) -> widget::text::Style {widget::text::Style {color:Some(MUTED)}}
// Figma faces are uniform colors. The shared Surface paints the actual
// outer/inner shadow stack; there is no full-face gradient or synthetic rim.
pub(super) fn convex(color:Color,_inset:bool)->Background {color.into()}
pub(super) fn control(color:Color,selected:bool,status:widget::button::Status)->widget::button::Style {
    widget::button::Style {
        background:Some(color.into()),
        text_color:if status==widget::button::Status::Disabled&&color==SURFACE{MUTED.scale_alpha(0.58)}else if status==widget::button::Status::Disabled{MUTED}else{INK},
        // Internal marker consumed by Surface; no outline is drawn.
        border:Border{radius:if status==widget::button::Status::Disabled&&color==SURFACE{12.0.into()}else{16.0.into()},width:if selected{1.0}else{0.0},color:Color::TRANSPARENT},
        ..Default::default()
    }
}
pub(super) fn button(_: &Theme,status:widget::button::Status)->widget::button::Style {
    if status==widget::button::Status::Pressed {let mut s=control(ACCENT,false,status);s.text_color=Color::WHITE;s}else{control(SURFACE,false,status)}
}
pub(super) fn selected_button(_: &Theme,status:widget::button::Status)->widget::button::Style {control(SURFACE,true,status)}
pub(super) fn output_button(_: &Theme,status:widget::button::Status)->widget::button::Style {control(Color::from_rgb8(227,230,233),false,status)}
pub(super) fn install_button(_: &Theme,status:widget::button::Status)->widget::button::Style {
    let mut style=control(ACCENT,false,status);style.text_color=if status==widget::button::Status::Disabled{MUTED}else{Color::WHITE};style
}
pub(super) fn transparent(_: &Theme,status:widget::button::Status)->widget::button::Style {
    widget::button::Style{text_color:if status==widget::button::Status::Disabled{MUTED}else{INK},..Default::default()}
}
pub(super) fn text_input(_: &Theme,status:widget::text_input::Status)->widget::text_input::Style {
    let mut style=widget::text_input::default(&light(),status);
    style.background=Color::TRANSPARENT.into();
    style.border=Border::default();
    style.value=INK;style.placeholder=MUTED;style
}
pub(super) fn checkbox(_: &Theme,status:widget::checkbox::Status)->widget::checkbox::Style {
    let mut style=widget::checkbox::primary(&light(),status);style.border.radius=5.0.into();style.border.width=0.0;style.icon_color=Color::WHITE;style
}
pub(super) fn installed_checkbox(theme:&Theme,status:widget::checkbox::Status)->widget::checkbox::Style {
    let mut style=checkbox(theme,status);style.background=Color::from_rgb8(227,230,233).into();style.icon_color=Color::from_rgb8(166,171,178);style.text_color=Some(MUTED.scale_alpha(INSTALLED_LABEL_ALPHA));style
}
pub(super) fn console(_: &Theme)->widget::container::Style {
    widget::container::Style{background:Some(Color::from_rgb8(36,43,48).into()),text_color:Some(Color::from_rgb8(222,226,232)),border:Border{radius:16.0.into(),..Default::default()},..Default::default()}
}
pub(super) fn console_action(_: &Theme,status:widget::button::Status)->widget::button::Style {
    widget::button::Style{text_color:if status==widget::button::Status::Disabled{MUTED}else{Color::WHITE},..Default::default()}
}

/// Figma's overflowing panes have no rail at rest; retain native drag feedback.
pub(super) fn scrollable(theme:&Theme,status:widget::scrollable::Status)->widget::scrollable::Style {
    let mut style=widget::scrollable::default(theme,status);
    let color=if matches!(status,widget::scrollable::Status::Active{..}){Color::TRANSPARENT}else{MUTED.scale_alpha(0.35)};
    let rail=widget::scrollable::Rail{
        background:None,border:Border::default(),
        scroller:widget::scrollable::Scroller{background:color.into(),border:Border{radius:2.0.into(),..Default::default()}},
    };
    style.vertical_rail=rail;style.horizontal_rail=rail;style.gap=None;style
}
