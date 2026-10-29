//! Shared values for the existing light native interface, without a visual redesign.
use iced::Theme;

pub(super) const APP_TITLE: f32 = 30.0;
pub(super) const PAGE_TITLE: f32 = 24.0;
pub(super) const SECTION_TITLE: f32 = 22.0;
pub(super) const FIELD_TITLE: f32 = 20.0;
pub(super) const SERVER_TITLE: f32 = 19.0;
pub(super) const TOOL_TITLE: f32 = 17.0;
pub(super) const TAGLINE: f32 = 16.0;
pub(super) const SMALL_TEXT: f32 = 12.0;
pub(super) const CLEANUP_TEXT: f32 = 13.0;

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
pub(super) const INPUT_PADDING: f32 = 5.0;
pub(super) const CHECKBOX_SIZE: f32 = 16.0;
pub(super) const CHECKBOX_GAP: f32 = 8.0;

pub(super) fn light() -> Theme {
    Theme::Light
}
