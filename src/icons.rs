use iced::{Font, alignment};
use iced::widget::{text, Text};

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char, size: f32) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(size + 2.0)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(size)
}

const DEFAULT_ICON_SIZE: f32 = 25.0;
const SMALL_ICON_SIZE: f32 = 20.0;

pub fn play_icon() -> Text<'static> {
    icon('\u{e918}', DEFAULT_ICON_SIZE)
}

pub fn pause_icon() -> Text<'static> {
    icon('\u{e919}', DEFAULT_ICON_SIZE)
}

pub fn stop_icon() -> Text<'static> {
    icon('\u{e90f}', DEFAULT_ICON_SIZE)
}

pub fn completed_icon() -> Text<'static> {
    icon('\u{e904}', DEFAULT_ICON_SIZE)
}

pub fn failed_icon() -> Text<'static> {
    icon('\u{e906}', DEFAULT_ICON_SIZE)
}

pub fn running_icon() -> Text<'static> {
    icon('\u{e91d}', DEFAULT_ICON_SIZE)
}

pub fn circle_icon() -> Text<'static> {
    icon('\u{e90a}', DEFAULT_ICON_SIZE)
}

pub fn menu_icon() -> Text<'static> {
    icon('\u{e90d}', SMALL_ICON_SIZE)
}

pub fn images_icon() -> Text<'static> {
    icon('\u{e91c}', SMALL_ICON_SIZE)
}

pub fn graph_icon() -> Text<'static> {
    icon('\u{e91f}', SMALL_ICON_SIZE)
}

pub fn gear_icon() -> Text<'static> {
    icon('\u{e920}', SMALL_ICON_SIZE)
}

pub fn three_dots_vertical_icon() -> Text<'static> {
    icon('\u{e90c}', DEFAULT_ICON_SIZE)
}
