//! Native rendering shared across pages. Callers retain state and backend decisions.
use super::theme;
use iced::{
    Element,
    widget::{self, Button, Checkbox, Container, Row, Text, TextInput, column, row, text},
};

pub(super) fn button<'a, M: Clone + 'a>(
    content: impl Into<Element<'a, M>>,
) -> Button<'a, M> {
    widget::button(content).padding(theme::BUTTON_PADDING)
}

pub(super) fn text_input<'a, M: Clone + 'a>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, M> {
    widget::text_input(placeholder, value)
        .padding(theme::INPUT_PADDING)
        .width(iced::Fill)
}

pub(super) fn checkbox<'a, M: Clone + 'a>(selected: bool) -> Checkbox<'a, M> {
    widget::checkbox(selected)
        .size(theme::CHECKBOX_SIZE)
        .spacing(theme::CHECKBOX_GAP)
}

pub(super) fn installed_checkbox<'a, M: Clone + 'a>() -> Checkbox<'a, M> {
    checkbox(true).style(theme::installed_checkbox)
}

pub(super) fn navigation<'a, M: Clone + 'a>(actions: [M; 5]) -> Row<'a, M> {
    let [servers, install, connect, settings, quit] = actions;
    row![
        text("Fetch").size(theme::APP_TITLE),
        button("Servers").on_press(servers),
        button("Install MCPs").on_press(install),
        button("Connect apps").on_press(connect),
        button("Settings").on_press(settings),
        button("Quit Fetch").on_press(quit),
    ]
    .spacing(theme::NAVIGATION_GAP)
    .align_y(iced::Center)
}

pub(super) fn page<'a, M: Clone + 'a>(
    content: impl Into<Element<'a, M>>,
) -> Container<'a, M> {
    widget::container(content)
        .padding(theme::PAGE_PADDING)
        .width(iced::Fill)
        .height(iced::Fill)
}

pub(super) fn message<'a>(content: impl widget::text::IntoFragment<'a>) -> Text<'a> {
    text(content)
}

pub(super) fn error_message<'a, M: Clone + 'a>(
    content: impl widget::text::IntoFragment<'a>,
) -> Container<'a, M> {
    widget::container(message(content)).padding(theme::MESSAGE_PADDING)
}

pub(super) fn server_card<'a, M: Clone + 'a>(
    content: impl Into<Element<'a, M>>,
) -> Container<'a, M> {
    widget::container(content)
        .padding(theme::CARD_PADDING)
        .style(widget::container::rounded_box)
}

pub(super) fn disclosure<'a, M: Clone + 'a>(
    title: &str,
    expanded: bool,
    size: f32,
    toggle: M,
) -> Button<'a, M> {
    button(text(format!("{} {title}", if expanded { "▾" } else { "▸" })).size(size))
        .on_press(toggle)
}

pub(super) fn tool_disclosure<'a, M: Clone + 'a>(
    name: &str,
    expanded: bool,
    raw_definition: Option<String>,
    toggle: M,
) -> Container<'a, M> {
    let mut content = column![disclosure(name, expanded, theme::TOOL_TITLE, toggle)];
    if let Some(definition) = raw_definition {
        content = content.push(text(definition).size(theme::SMALL_TEXT));
    }
    widget::container(content).padding(theme::PANEL_PADDING)
}

pub(super) fn server_header<'a, M: Clone + 'a>(
    title: &str,
    expanded: bool,
    toggle: M,
    delete: Option<M>,
    delete_hint: &str,
) -> Row<'a, M> {
    row![
        disclosure(title, expanded, theme::SERVER_TITLE, toggle).width(iced::Fill),
        widget::tooltip(
            button(text("×").size(theme::DELETE_ICON_SIZE))
                .padding(theme::DELETE_BUTTON_PADDING)
                .on_press_maybe(delete),
            text(delete_hint.to_owned()),
            widget::tooltip::Position::Top,
        ),
    ]
    .spacing(theme::CONTROL_GAP)
    .align_y(iced::Center)
}
