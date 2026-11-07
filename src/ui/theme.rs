//! Shared values for the existing light native interface, without a visual redesign.
use iced::Theme;

pub(super) const APP_TITLE: f32 = 30.0;
pub(super) const PAGE_TITLE: f32 = 24.0;
pub(super) const SECTION_TITLE: f32 = 22.0;
pub(super) const FIELD_TITLE: f32 = 20.0;
pub(super) const SERVER_TITLE: f32 = 19.0;
pub(super) const TOOL_TITLE: f32 = 17.0;
pub(super) const SMALL_TEXT: f32 = 12.0;

pub(super) const CONTROL_GAP: f32 = 10.0;
pub(super) const NAVIGATION_GAP: f32 = 12.0;
pub(super) const PAGE_GAP: f32 = 14.0;
pub(super) const CARD_GAP: f32 = 16.0;
pub(super) const SECTION_GAP: f32 = 20.0;
pub(super) const PAGE_PADDING: f32 = 28.0;
pub(super) const CARD_PADDING: f32 = 16.0;
pub(super) const MESSAGE_PADDING: f32 = 12.0;
pub(super) const PANEL_PADDING: f32 = 10.0;

// Match Iced's current primary button, full-width input and checkbox defaults.
pub(super) const BUTTON_PADDING: [f32; 2] = [5.0, 10.0];
pub(super) const DELETE_BUTTON_PADDING: [f32; 2] = [3.0, 7.0];
pub(super) const DELETE_ICON_SIZE: f32 = 18.0;
pub(super) const INPUT_PADDING: f32 = 5.0;
pub(super) const CHECKBOX_SIZE: f32 = 16.0;
pub(super) const CHECKBOX_GAP: f32 = 8.0;
pub(super) const INSTALLED_LABEL_ALPHA: f32 = 0.5;

pub(super) fn installed_checkbox(theme: &Theme, status: iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
    let mut style = iced::widget::checkbox::primary(theme, status);
    style.text_color = Some(theme.palette().text.scale_alpha(INSTALLED_LABEL_ALPHA));
    style
}

pub(super) fn light() -> Theme {
    Theme::Light
}

pub(super) fn muted_text(theme: &Theme) -> iced::widget::text::Style {
    iced::widget::text::Style { color: Some(theme.extended_palette().background.base.text.scale_alpha(0.55)) }
}

pub(super) fn install_button(theme: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    match status {
        iced::widget::button::Status::Disabled => iced::widget::button::secondary(theme, status),
        _ => iced::widget::button::primary(theme, status),
    }
}

pub(super) const CONSOLE_HEIGHT: f32 = 200.0;
pub(super) const CONSOLE_TEXT_SIZE: f32 = 13.0;
pub(super) const CONSOLE_LINE_GAP: f32 = 4.0;

pub(super) fn console(_theme: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background:Some(iced::Color::from_rgb8(23,25,30).into()),
        text_color:Some(iced::Color::from_rgb8(222,226,232)),
        border:iced::Border {color:iced::Color::from_rgb8(67,72,82),width:1.0,radius:4.0.into()},
        ..Default::default()
    }
}

pub(super) fn console_action(theme: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let mut style=iced::widget::button::text(theme,status);
    style.text_color=if status==iced::widget::button::Status::Disabled {
        iced::Color::from_rgb8(113,122,136)
    }else{
        iced::Color::from_rgb8(148,189,255)
    };
    style
}
