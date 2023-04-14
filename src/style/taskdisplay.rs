use iced_core::{Background, Color};
use iced_style::theme::Theme;

/// The appearance of a task.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The [`Background`] of the task bar.
    pub background: Background,
    /// The [`Background`] of the bar of the task bar.
    pub bar: Background,
    /// The border radius of the task bar.
    pub border_radius: f32,
    /// Test color that overlays on task bar
    pub text_color: Color,
}

/// A set of rules that dictate the style of a progress bar.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of the progress bar.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

pub enum TaskDisplayStyles {
    Waiting,
    Running,
    Completed,
    Failed,
}

impl Default for TaskDisplayStyles {
    fn default() -> Self {
        TaskDisplayStyles::Waiting
    }
}

impl StyleSheet for Theme {
    type Style = TaskDisplayStyles;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        match style {
            TaskDisplayStyles::Waiting => Appearance {
                background: palette.background.weak.color.into(),
                bar: palette.background.strong.color.into(),
                border_radius: 0.0,
                text_color: Color::BLACK,
            },
            TaskDisplayStyles::Running => Appearance {
                background: palette.primary.weak.color.into(),
                bar: palette.primary.strong.color.into(),
                border_radius: 0.0,
                text_color: Color::BLACK,
            },
            TaskDisplayStyles::Completed => Appearance {
                background: palette.success.weak.color.into(),
                bar: palette.success.strong.color.into(),
                border_radius: 0.0,
                text_color: Color::BLACK,
            },
            TaskDisplayStyles::Failed => Appearance {
                background: palette.danger.weak.color.into(),
                bar: palette.danger.strong.color.into(),
                border_radius: 0.0,
                text_color: Color::BLACK,
            },
        }
    }
}
