//! Provide progress feedback to your users.
use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::widget::{tree::Tree, Widget};
use iced_native::{Color, Element, Length, Padding, Point, Rectangle};

use crate::style::taskdisplay::StyleSheet;

use std::ops::RangeInclusive;

pub struct TaskDisplay<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    content: Element<'a, Message, Renderer>,
    range: RangeInclusive<f32>,
    value: f32,
    width: Length,
    height: Option<Length>,
    padding: Padding,
    border_radius: f32,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> TaskDisplay<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub const DEFAULT_HEIGHT: f32 = 40.0;

    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        TaskDisplay {
            content: content.into(),
            range: 0.0..=100.0,
            value: 0.0,
            width: Length::Fill,
            height: Some(Length::Shrink),
            padding: Padding::new(15.0),
            border_radius: 10.0,
            style: Default::default(),
        }
    }

    /// Sets the style of the [`ProgressBar`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TaskDisplay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height.unwrap_or(Length::Fixed(Self::DEFAULT_HEIGHT))
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        layout(
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            |renderer, limits| self.content.as_widget().layout(renderer, limits),
        )
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let (range_start, range_end) = self.range.clone().into_inner();
        let content_layout = layout.children().next().unwrap();

        let active_progress_width = if range_start >= range_end {
            0.0
        } else {
            bounds.width * (self.value - range_start) / (range_end - range_start)
        };

        let style = theme.appearance(&self.style);

        // Draw task background quad
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle { ..bounds },
                border_radius: self.border_radius.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            style.background,
        );

        // Draw task progress quad
        if active_progress_width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        width: active_progress_width,
                        ..bounds
                    },
                    border_radius: self.border_radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                style.bar,
            );
        }

        // Draw content on top of task bar
        self.content.as_widget().draw(
            &_state.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: _style.text_color,
            },
            content_layout,
            _cursor_position,
            &bounds,
        );
    }
}

impl<'a, Message, Renderer> From<TaskDisplay<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(task: TaskDisplay<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(task)
    }
}

/// Computes the layout of a [`TaskDisplay`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Option<Length>,
    padding: Padding,
    layout_content: impl FnOnce(&Renderer, &layout::Limits) -> layout::Node,
) -> layout::Node {
    let limits = limits.width(width).height(height.unwrap());

    let mut content = layout_content(renderer, &limits.pad(padding));
    let padding = padding.fit(content.size(), limits.max());
    let size = limits.pad(padding).resolve(content.size()).pad(padding);

    content.move_to(Point::new(padding.left, padding.top));

    layout::Node::with_children(size, vec![content])
}
