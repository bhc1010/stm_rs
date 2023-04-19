//! Display fields that can be filled with text.
//!
//! A [`ScientificTextInput`] has some local [`State`].

pub mod editor;
pub mod value;

pub mod cursor;

use cursor::Cursor;
use value::Value;

use iced_native::alignment;
use iced_native::event::{self, Event};
use iced_native::keyboard;
use iced_native::layout;
use iced_native::mouse;
use iced_native::renderer;
use iced_native::text::{self, Text};
use iced_native::time::{Duration, Instant};
use iced_native::touch;
use iced_native::widget;
use iced_native::widget::operation::{self, Operation};
use iced_native::widget::tree::{self, Tree};
use iced_native::window;
use iced_native::{
    Clipboard, Color, Element, Layout, Length, Padding, Pixels, Point, Rectangle, Shell,
    Size, Vector, Widget,
};

use crate::style::scientific_text_input::StyleSheet;

/// A field that can be filled with text.
///
/// # Example
/// ```
/// # pub type ScientificTextInput<'a, Message> = iced_native::widget::ScientificTextInput<'a, Message, iced_native::renderer::Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     ScientificTextInputChanged(String),
/// }
///
/// let value = "Some text";
///
/// let input = ScientificTextInput::new(
///     "This is the placeholder...",
///     value,
/// )
/// .on_input(Message::ScientificTextInputChanged)
/// .padding(10);
/// ```
/// ![Text input drawn by `iced_wgpu`](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/text_input.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct ScientificTextInput<'a, Message, Renderer>
where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    id: Option<Id>,
    placeholder: String,
    value: Value,
    is_secure: bool,
    font: Renderer::Font,
    width: Length,
    padding: Padding,
    size: Option<f32>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    icon: Option<Icon<Renderer::Font>>,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> ScientificTextInput<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`ScientificTextInput`].
    ///
    /// It expects:
    /// - a placeholder,
    /// - the current value
    pub fn new(placeholder: &str, value: &str) -> Self {
        ScientificTextInput {
            id: None,
            placeholder: String::from(placeholder),
            value: Value::new(value),
            is_secure: false,
            font: Default::default(),
            width: Length::Fill,
            padding: Padding::new(5.0),
            size: None,
            on_input: None,
            on_paste: None,
            on_submit: None,
            icon: None,
            style: Default::default(),
        }
    }

    pub fn get_value(&self) -> Value {
        self.value.clone()
    }

    /// Sets the [`Id`] of the [`ScientificTextInput`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Converts the [`ScientificTextInput`] into a secure password input.
    pub fn password(mut self) -> Self {
        self.is_secure = true;
        self
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [`ScientificTextInput`].
    ///
    /// If this method is not called, the [`ScientificTextInput`] will be disabled.
    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_input = Some(Box::new(callback));
        self
    }

    /// Sets the message that should be produced when the [`ScientificTextInput`] is
    /// focused and the enter key is pressed.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the message that should be produced when some text is pasted into
    /// the [`ScientificTextInput`].
    pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self
    }

    /// Sets the [`Font`] of the [`ScientificTextInput`].
    ///
    /// [`Font`]: text::Renderer::Font
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the [`Icon`] of the [`ScientificTextInput`].
    pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the width of the [`ScientificTextInput`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the [`Padding`] of the [`ScientificTextInput`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`ScientificTextInput`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    /// Sets the style of the [`ScientificTextInput`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Draws the [`ScientificTextInput`] with the given [`Renderer`], overriding its
    /// [`Value`] if provided.
    ///
    /// [`Renderer`]: text::Renderer
    pub fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        layout: Layout<'_>,
        cursor_position: Point,
        value: Option<&Value>,
    ) {
        draw(
            renderer,
            theme,
            layout,
            cursor_position,
            tree.state.downcast_ref::<State>(),
            value.unwrap_or(&self.value),
            &self.placeholder,
            self.size,
            &self.font,
            self.on_input.is_none(),
            self.is_secure,
            self.icon.as_ref(),
            &self.style,
        )
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ScientificTextInput<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();

        // Unfocus text input if it becomes disabled
        if self.on_input.is_none() {
            state.last_click = None;
            state.is_focused = None;
            state.is_pasting = None;
            state.is_dragging = false;
        }
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        layout(
            renderer,
            limits,
            self.width,
            self.padding,
            self.size,
            self.icon.as_ref(),
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        let state = tree.state.downcast_mut::<State>();

        operation.focusable(state, self.id.as_ref().map(|id| &id.0));
        // operation.text_input(state, self.id.as_ref().map(|id| &id.0));
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _: &Renderer,
        _: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        update(
            event,
            layout,
            cursor_position,
            shell,
            &mut self.value,
            self.on_input.as_deref(),
            &self.on_submit,
            || tree.state.downcast_mut::<State>(),
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            theme,
            layout,
            cursor_position,
            tree.state.downcast_ref::<State>(),
            &self.value,
            &self.placeholder,
            self.size,
            &self.font,
            self.on_input.is_none(),
            self.is_secure,
            self.icon.as_ref(),
            &self.style,
        )
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor_position, self.on_input.is_none())
    }
}

impl<'a, Message, Renderer> From<ScientificTextInput<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + text::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(
        text_input: ScientificTextInput<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(text_input)
    }
}

/// The content of the [`Icon`].
#[derive(Debug, Clone)]
pub struct Icon<Font> {
    /// The font that will be used to display the `code_point`.
    pub font: Font,
    /// The unicode code point that will be used as the icon.
    pub code_point: char,
    /// The font size of the content.
    pub size: Option<f32>,
    /// The spacing between the [`Icon`] and the text in a [`ScientificTextInput`].
    pub spacing: f32,
    /// The side of a [`ScientificTextInput`] where to display the [`Icon`].
    pub side: Side,
}

/// The side of a [`ScientificTextInput`].
#[derive(Debug, Clone)]
pub enum Side {
    /// The left side of a [`ScientificTextInput`].
    Left,
    /// The right side of a [`ScientificTextInput`].
    Right,
}

/// The identifier of a [`ScientificTextInput`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(widget::Id);

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(widget::Id::new(id))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self(widget::Id::unique())
    }
}

impl From<Id> for widget::Id {
    fn from(id: Id) -> Self {
        id.0
    }
}

/// Computes the layout of a [`ScientificTextInput`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    padding: Padding,
    size: Option<f32>,
    icon: Option<&Icon<Renderer::Font>>,
) -> layout::Node
where
    Renderer: text::Renderer,
{
    let text_size = size.unwrap_or_else(|| renderer.default_size());

    let padding = padding.fit(Size::ZERO, limits.max());
    let limits = limits.width(width).pad(padding).height(text_size);

    let text_bounds = limits.resolve(Size::ZERO);

    if let Some(icon) = icon {
        let icon_width = renderer.measure_width(
            &icon.code_point.to_string(),
            icon.size.unwrap_or_else(|| renderer.default_size()),
            icon.font.clone(),
        );

        let mut text_node =
            layout::Node::new(text_bounds - Size::new(icon_width + icon.spacing, 0.0));

        let mut icon_node = layout::Node::new(Size::new(icon_width, text_bounds.height));

        match icon.side {
            Side::Left => {
                text_node.move_to(Point::new(
                    padding.left + icon_width + icon.spacing,
                    padding.top,
                ));

                icon_node.move_to(Point::new(padding.left, padding.top));
            }
            Side::Right => {
                text_node.move_to(Point::new(padding.left, padding.top));

                icon_node.move_to(Point::new(
                    padding.left + text_bounds.width - icon_width,
                    padding.top,
                ));
            }
        };

        layout::Node::with_children(text_bounds.pad(padding), vec![text_node, icon_node])
    } else {
        let mut text = layout::Node::new(text_bounds);
        text.move_to(Point::new(padding.left, padding.top));

        layout::Node::with_children(text_bounds.pad(padding), vec![text])
    }
}

/// Processes an [`Event`] and updates the [`State`] of a [`ScientificTextInput`]
/// accordingly.
pub fn update<'a, Message>(
    event: Event,
    layout: Layout<'_>,
    cursor_position: Point,
    // renderer: &Renderer,
    // clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
    value: &mut Value,
    // size: Option<f32>,
    // font: &Renderer::Font,
    // is_secure: bool,
    on_input: Option<&dyn Fn(String) -> Message>,
    // on_paste: Option<&dyn Fn(String) -> Message>,
    on_submit: &Option<Message>,
    state: impl FnOnce() -> &'a mut State,
) -> event::Status
where
    Message: Clone,
    // Renderer: text::Renderer,
{
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            let state = state();
            let is_clicked = layout.bounds().contains(cursor_position) && on_input.is_some();

            state.is_focused = if is_clicked {
                state.is_focused.or_else(|| {
                    let now = Instant::now();

                    Some(Focus {
                        updated_at: now,
                        now,
                    })
                })
            } else {
                None
            };
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
            let state = state();

            if let Some(focus) = &mut state.is_focused {
                let Some(_) = on_input else { return event::Status::Ignored };

                focus.updated_at = Instant::now();

                match key_code {
                    keyboard::KeyCode::Enter | keyboard::KeyCode::NumpadEnter => {
                        if let Some(on_submit) = on_submit.clone() {
                            shell.publish(on_submit);
                        }
                    }
                    keyboard::KeyCode::Left => state.cursor.select_left(value),
                    keyboard::KeyCode::Right => state.cursor.select_right(value),
                    keyboard::KeyCode::Escape => {
                        state.is_focused = None;
                        state.is_dragging = false;
                        state.is_pasting = None;

                        state.keyboard_modifiers = keyboard::Modifiers::default();
                    }
                    keyboard::KeyCode::Tab => {}
                    keyboard::KeyCode::Up | keyboard::KeyCode::Down => {
                        return event::Status::Ignored;
                    }
                    _ => {}
                }

                return event::Status::Captured;
            }
        }
        Event::Window(window::Event::RedrawRequested(now)) => {
            let state = state();

            if let Some(focus) = &mut state.is_focused {
                focus.now = now;

                let millis_until_redraw = CURSOR_BLINK_INTERVAL_MILLIS
                    - (now - focus.updated_at).as_millis() % CURSOR_BLINK_INTERVAL_MILLIS;

                shell.request_redraw(window::RedrawRequest::At(
                    now + Duration::from_millis(millis_until_redraw as u64),
                ));
            }
        }
        _ => {}
    }

    event::Status::Ignored
}

/// Draws the [`ScientificTextInput`] with the given [`Renderer`], overriding its
/// [`Value`] if provided.
///
/// [`Renderer`]: text::Renderer
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    theme: &Renderer::Theme,
    layout: Layout<'_>,
    cursor_position: Point,
    state: &State,
    value: &Value,
    placeholder: &str,
    size: Option<f32>,
    font: &Renderer::Font,
    is_disabled: bool,
    is_secure: bool,
    icon: Option<&Icon<Renderer::Font>>,
    style: &<Renderer::Theme as StyleSheet>::Style,
) where
    Renderer: text::Renderer,
    Renderer::Theme: StyleSheet,
{
    let secure_value = is_secure.then(|| value.secure());
    let value = secure_value.as_ref().unwrap_or(value);

    let bounds = layout.bounds();

    let mut children_layout = layout.children();
    let text_bounds = children_layout.next().unwrap().bounds();

    let is_mouse_over = bounds.contains(cursor_position);

    let appearance = if is_disabled {
        theme.disabled(style)
    } else if state.is_focused() {
        theme.focused(style)
    } else if is_mouse_over {
        theme.hovered(style)
    } else {
        theme.active(style)
    };

    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border_radius: appearance.border_radius.into(),
            border_width: appearance.border_width,
            border_color: appearance.border_color,
        },
        appearance.background,
    );

    if let Some(icon) = icon {
        let icon_layout = children_layout.next().unwrap();

        renderer.fill_text(Text {
            content: &icon.code_point.to_string(),
            size: icon.size.unwrap_or_else(|| renderer.default_size()),
            font: icon.font.clone(),
            color: appearance.icon_color,
            bounds: icon_layout.bounds(),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
        });
    }

    let text = value.to_string();
    let size = size.unwrap_or_else(|| renderer.default_size());

    let (cursor, offset) = if let Some(focus) = &state.is_focused {
        match state.cursor.state(value) {
            cursor::State::Index(position) => {
                let (text_value_width, offset) = measure_cursor_and_scroll_offset(
                    renderer,
                    text_bounds,
                    value,
                    size,
                    position,
                    font.clone(),
                );

                let is_cursor_visible =
                    ((focus.now - focus.updated_at).as_millis() / CURSOR_BLINK_INTERVAL_MILLIS) % 2
                        == 0;

                let cursor = if is_cursor_visible {
                    Some((
                        renderer::Quad {
                            bounds: Rectangle {
                                x: text_bounds.x + text_value_width,
                                y: text_bounds.y,
                                width: 1.0,
                                height: text_bounds.height,
                            },
                            border_radius: 0.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                        },
                        theme.value_color(style),
                    ))
                } else {
                    None
                };

                (cursor, offset)
            }
            cursor::State::Selection { start, end } => {
                let left = start.min(end);
                let right = end.max(start);

                let (left_position, left_offset) = measure_cursor_and_scroll_offset(
                    renderer,
                    text_bounds,
                    value,
                    size,
                    left,
                    font.clone(),
                );

                let (right_position, right_offset) = measure_cursor_and_scroll_offset(
                    renderer,
                    text_bounds,
                    value,
                    size,
                    right,
                    font.clone(),
                );

                let width = right_position - left_position;

                (
                    Some((
                        renderer::Quad {
                            bounds: Rectangle {
                                x: text_bounds.x + left_position,
                                y: text_bounds.y,
                                width,
                                height: text_bounds.height,
                            },
                            border_radius: 0.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                        },
                        theme.selection_color(style),
                    )),
                    if end == right {
                        right_offset
                    } else {
                        left_offset
                    },
                )
            }
        }
    } else {
        (None, 0.0)
    };

    let text_width = renderer.measure_width(
        if text.is_empty() { placeholder } else { &text },
        size,
        font.clone(),
    );

    let render = |renderer: &mut Renderer| {
        if let Some((cursor, color)) = cursor {
            renderer.fill_quad(cursor, color);
        }

        renderer.fill_text(Text {
            content: if text.is_empty() { placeholder } else { &text },
            color: if text.is_empty() {
                theme.placeholder_color(style)
            } else if is_disabled {
                theme.disabled_color(style)
            } else {
                theme.value_color(style)
            },
            font: font.clone(),
            bounds: Rectangle {
                y: text_bounds.center_y(),
                width: f32::INFINITY,
                ..text_bounds
            },
            size,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Center,
        });
    };

    if text_width > text_bounds.width {
        renderer.with_layer(text_bounds, |renderer| {
            renderer.with_translation(Vector::new(-offset, 0.0), render)
        });
    } else {
        render(renderer);
    }
}

/// Computes the current [`mouse::Interaction`] of the [`ScientificTextInput`].
pub fn mouse_interaction(
    layout: Layout<'_>,
    cursor_position: Point,
    is_disabled: bool,
) -> mouse::Interaction {
    if layout.bounds().contains(cursor_position) {
        if is_disabled {
            mouse::Interaction::NotAllowed
        } else {
            mouse::Interaction::Text
        }
    } else {
        mouse::Interaction::default()
    }
}

/// The state of a [`ScientificTextInput`].
#[derive(Debug, Default, Clone)]
pub struct State {
    is_focused: Option<Focus>,
    is_dragging: bool,
    is_pasting: Option<Value>,
    last_click: Option<mouse::Click>,
    cursor: Cursor,
    keyboard_modifiers: keyboard::Modifiers,
    // TODO: Add stateful horizontal scrolling offset
}

#[derive(Debug, Clone, Copy)]
struct Focus {
    updated_at: Instant,
    now: Instant,
}

impl State {
    /// Creates a new [`State`], representing an unfocused [`ScientificTextInput`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the [`ScientificTextInput`] is currently focused or not.
    pub fn is_focused(&self) -> bool {
        self.is_focused.is_some()
    }

    /// Returns the [`Cursor`] of the [`ScientificTextInput`].
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn select_left(&mut self, value: &Value) {
        self.cursor.select_left(value)
    }

    pub fn select_right(&mut self, value: &Value) {
        self.cursor.select_right(value)
    }

    /// Focuses the [`ScientificTextInput`].
    pub fn focus(&mut self) {
        let now = Instant::now();

        self.is_focused = Some(Focus {
            updated_at: now,
            now,
        });
    }

    /// Unfocuses the [`ScientificTextInput`].
    pub fn unfocus(&mut self) {
        self.is_focused = None;
    }

}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        State::is_focused(self)
    }

    fn focus(&mut self) {
        State::focus(self)
    }

    fn unfocus(&mut self) {
        State::unfocus(self)
    }
}

fn measure_cursor_and_scroll_offset<Renderer>(
    renderer: &Renderer,
    text_bounds: Rectangle,
    value: &Value,
    size: f32,
    cursor_index: usize,
    font: Renderer::Font,
) -> (f32, f32)
where
    Renderer: text::Renderer,
{
    let text_before_cursor = value.until(cursor_index).to_string();

    let text_value_width = renderer.measure_width(&text_before_cursor, size, font);

    let offset = ((text_value_width + 5.0) - text_bounds.width).max(0.0);

    (text_value_width, offset)
}

const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;
