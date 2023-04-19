//! Change the appearance of a text input.
use iced::theme::Theme;
use iced_core::{Background, Color};

/// The appearance of a text input.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The [`Background`] of the text input.
    pub background: Background,
    /// The border radius of the text input.
    pub border_radius: f32,
    /// The border width of the text input.
    pub border_width: f32,
    /// The border [`Color`] of the text input.
    pub border_color: Color,
    /// The icon [`Color`] of the text input.
    pub icon_color: Color,
}

/// A set of rules that dictate the style of a text input.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> Appearance;

    /// Produces the [`Color`] of the placeholder of a text input.
    fn placeholder_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the value of a text input.
    fn value_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the value of a disabled text input.
    fn disabled_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the selection of a text input.
    fn selection_color(&self, style: &Self::Style) -> Color;

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.focused(style)
    }

    /// Produces the style of a disabled text input.
    fn disabled(&self, style: &Self::Style) -> Appearance;
}

/// The style of a text input.
#[derive(Default)]
pub enum ScientificTextStyle {
    /// The default style.
    #[default]
    Default,
    /// A custom style.
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl StyleSheet for Theme {
    type Style = ScientificTextStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.active(self);
        }

        let palette = self.extended_palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.weak.text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.hovered(self);
        }

        let palette = self.extended_palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.base.text,
            icon_color: palette.background.weak.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.focused(self);
        }

        let palette = self.extended_palette();

        Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.primary.strong.color,
            icon_color: palette.background.weak.text,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.placeholder_color(self);
        }

        let palette = self.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.value_color(self);
        }

        let palette = self.extended_palette();

        palette.background.base.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.selection_color(self);
        }

        let palette = self.extended_palette();

        palette.primary.weak.color
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.disabled(self);
        }

        let palette = self.extended_palette();

        Appearance {
            background: palette.background.weak.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        if let ScientificTextStyle::Custom(custom) = style {
            return custom.disabled_color(self);
        }

        self.placeholder_color(style)
    }
}
