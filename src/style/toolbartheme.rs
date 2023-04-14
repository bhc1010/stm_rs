use iced::{Theme, Color, color};
use iced::widget::{container, button};
use iced_core::Vector;

pub struct ToolBarTheme;

impl container::StyleSheet for ToolBarTheme {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(94, 124, 226).into(),
            border_radius: 20.0,
            ..Default::default()
        }
    }
}

impl button::StyleSheet for ToolBarTheme {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: color!(94, 124, 226).into(),
            border_radius: 32.0,
            border_width: 32.0,
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    // Produces the hovered [`Appearance`] of a button.
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: color!(219, 84, 97).into(),
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }

    /// Produces the pressed [`Appearance`] of a button.
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: color!(219, 84, 97).into(),
            shadow_offset: Vector::default(),
            ..self.active(style)
        }
    }

}