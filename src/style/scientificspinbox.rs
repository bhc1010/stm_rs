use iced::Theme;

pub trait StyleSheet {
    type Style: Default;
}

impl StyleSheet for Theme {
    type Style = Theme;
}
