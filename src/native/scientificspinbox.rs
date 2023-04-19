// use iced::{widget::text_input::move_cursor_to, Command};
use num_traits::clamp;

use crate::native::scientific_text_input::{cursor, value::Value, ScientificTextInput, State};

use iced_native::{
    event, keyboard,
    layout::{Limits, Node},
    mouse,
    widget::{
        container, text,
        tree::{self, Tree},
        Column, Container, Operation, Row, Text,
    },
    Alignment, Clipboard, Element, Event, Layout, Length, Padding, Point,
    Rectangle, Shell, Size, Widget,
};

use std::str::FromStr;

use crate::style::scientificspinbox;

const DEFAULT_PADDING: f32 = 5.0;

#[derive(Debug, Clone, Copy)]
pub struct ExponentialNumber {
    pub significand: f64,
    pub exponent: i8,
}

impl ExponentialNumber {
    pub fn new(significand: f64, exponent: i8) -> Self {
        Self {
            significand,
            exponent,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.significand * 10_f64.powf(self.exponent as f64)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    lower: ExponentialNumber,
    upper: ExponentialNumber,
}

impl Bounds {
    pub fn new(lower: ExponentialNumber, upper: ExponentialNumber) -> Self {
        Self { lower, upper }
    }

    pub fn from_f64(lower: f64, upper: f64) -> Self {
        let low_str = format!("{:.e}", lower);
        let low_val = low_str
            .split("e")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let up_str = format!("{:.e}", upper);
        let up_val = up_str
            .split("e")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        Self {
            lower: ExponentialNumber::new(low_val[0], low_val[1] as i8),
            upper: ExponentialNumber::new(up_val[0], up_val[1] as i8),
        }
    }

    pub fn clamp(&self, value: &f64) -> f64 {
        let mut lower = self.lower.to_f64();
        let mut upper = self.upper.to_f64();
        let mut val = value.clone();
        let result = clamp(&mut val, &mut lower, &mut upper);
        *result
    }

    pub fn in_bounds(&self, value: &f64) -> bool {
        *value == self.clamp(&value)
    }
}

pub struct ScientificSpinBox<'a, Message, Renderer>
where
    Renderer: iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: scientificspinbox::StyleSheet
        + crate::style::scientific_text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    value: ExponentialNumber,
    step: f64,
    bounds: Bounds,
    padding: f32,
    size: Option<f32>,
    content: ScientificTextInput<'a, Message, Renderer>,
    on_change: Box<dyn Fn(ExponentialNumber) -> Message>,
    style: <Renderer::Theme as scientificspinbox::StyleSheet>::Style,
    font: Renderer::Font,
}

impl<'a, Message, Renderer> ScientificSpinBox<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: scientificspinbox::StyleSheet
        + crate::style::scientific_text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    pub fn new<F>(value: ExponentialNumber, bounds: Bounds, unit: &str, on_changed: F) -> Self
    where
        F: 'static + Copy + Fn(ExponentialNumber) -> Message,
    {
        let convert_to_num = move |s: String| {
            on_changed(ExponentialNumber {
                significand: f64::from_str(&s).unwrap_or(if s.is_empty() {
                    0.0
                } else {
                    value.significand
                }),
                exponent: value.exponent,
            })
        };

        let prefix = get_prefix_from_exponent(value.exponent);
        let mut display = format!("{:.3} {prefix}{unit}", value.significand.abs());

        if value.significand < 0.0 {
            display = "-".to_owned() + display.as_str();
        }

        Self {
            value,
            step: 1.0,
            bounds,
            padding: DEFAULT_PADDING,
            size: None,
            content: ScientificTextInput::new("", display.as_str())
                .on_input(convert_to_num)
                .padding(DEFAULT_PADDING)
                .width(Length::Fixed(169.0)),
            on_change: Box::new(on_changed),
            style: <Renderer::Theme as scientificspinbox::StyleSheet>::Style::default(),
            font: iced_native::Font::default(),
        }
    }

    /// Sets the step of the [`NumberInput`].
    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Sets the minimum significand of the [`NumberInput`].
    #[must_use]
    pub fn min(mut self, min: ExponentialNumber) -> Self {
        if min.to_f64() <= self.bounds.upper.to_f64() {
            self.bounds.lower = min;
        }
        self
    }

    /// Sets the maximum significand of the [`NumberInput`].
    #[must_use]
    pub fn max(mut self, max: ExponentialNumber) -> Self {
        if max.to_f64() >= self.bounds.lower.to_f64() {
            self.bounds.upper = max;
        }
        self
    }

    /// Sets the minimum & maximum significand (bound) of the [`NumberInput`].
    #[must_use]
    pub fn bounds(mut self, bounds: Bounds) -> Self {
        if bounds.lower.to_f64() <= bounds.upper.to_f64() {
            self.bounds = bounds;
        }
        self
    }

    /// Sets the [ `Font`] of the [`Text`].
    ///
    /// [`Font`]: crate::widget::text::Renderer::Font
    /// [`Text`]: crate::widget::Text
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self.content = self.content.font(font);
        self
    }

    /// Sets the width of the [`NumberInput`].
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.content = self.content.width(width);
        self
    }

    /// Sets the padding of the [`NumberInput`].
    #[must_use]
    pub fn padding(mut self, units: f32) -> Self {
        self.padding = units;
        self.content = self.content.padding(units);
        self
    }

    /// Sets the text size of the [`NumberInput`].
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self.content = self.content.size(size);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.content = self.content.on_submit(message);
        self
    }

    /// Sets the style of the [`NumberInput`].
    #[must_use]
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as scientificspinbox::StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the input style of the [`NumberInput`].
    #[must_use]
    pub fn input_style(
        mut self,
        style: impl Into<<Renderer::Theme as crate::style::scientific_text_input::StyleSheet>::Style>,
    ) -> Self {
        self.content = self.content.style(style);
        self
    }

    /// Decrease current significand by step of the [`NumberInput`].
    fn decrease_val(&mut self, shell: &mut Shell<Message>, child: &mut Tree, value: &mut Value) {
        let (start, end) = child
            .state
            .downcast_ref::<State>()
            .cursor()
            .selection(&value)
            .unwrap_or_else(|| (0, 1));
        let pos = start.min(end) as i32;
        let sig = self.value.significand;
        let mut exp = self.value.exponent;

        if value.graphemes[pos as usize]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
        {
            let mut new_sig = sig - get_step(pos, value);
            if new_sig <= -1000.0 {
                new_sig = new_sig / 1000.0;
                exp = exp + 3;
            } else if new_sig < 1.0 && new_sig > 0.0 && exp - 3 != -12 {
                new_sig = new_sig * 1000.0;
                exp = exp - 3;

                // Move cursor for selection continuity
                let new_value = Value::new(new_sig.to_string().as_str());
                child.state.downcast_mut::<State>().select_left(&new_value);
                child.state.downcast_mut::<State>().select_left(&new_value);
            }

            let new_val = ExponentialNumber::new(new_sig, exp);

            if self.bounds.in_bounds(&new_val.to_f64()) {
                shell.publish((self.on_change)(new_val));
            } else {
                shell.publish((self.on_change)(self.bounds.lower));
            }

            if sig >= 0.0 && new_sig < 0.0 {
                let new_value = Value::new(new_sig.to_string().as_str());
                child.state.downcast_mut::<State>().select_right(&new_value);
            }
        } else {
            let new_exp = exp - 3;
            let mut new_val = ExponentialNumber::new(sig, new_exp);

            if !self.bounds.in_bounds(&new_val.to_f64()) {
                new_val = self.bounds.lower;
            } else if new_exp < -12 {
                new_val.exponent = -12;
            }

            shell.publish((self.on_change)(new_val));
        }
    }

    /// Increase current significand by step of the [`NumberInput`].
    fn increase_val(&mut self, shell: &mut Shell<Message>, child: &mut Tree, value: &mut Value) {
        let (start, end) = child
            .state
            .downcast_ref::<State>()
            .cursor()
            .selection(&value)
            .unwrap_or_else(|| (0, 1));
        let pos = start.min(end) as i32;
        let sig = self.value.significand;
        let mut exp = self.value.exponent;

        if value.graphemes[pos as usize]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
        {
            let mut new_sig = sig + get_step(pos, value);
            if new_sig >= 1000.0 {
                new_sig = new_sig / 1000.0;
                exp = exp + 3;
            } else if (-1.0 < new_sig && new_sig < 0.0) | (0.0 < new_sig && new_sig < 1.0) {
                new_sig = new_sig * 1000.0;
                exp = exp - 3;

                // Move cursor for selection continuity
                let new_value = Value::new(new_sig.to_string().as_str());
                child.state.downcast_mut::<State>().select_left(&new_value);
                child.state.downcast_mut::<State>().select_left(&new_value);
            }

            let new_val = ExponentialNumber::new(new_sig, exp);

            if self.bounds.in_bounds(&new_val.to_f64()) {
                shell.publish((self.on_change)(new_val));
            } else {
                shell.publish((self.on_change)(self.bounds.upper));
            }

            if sig < 0.0 && new_sig >= 0.0 {
                let new_value = Value::new(new_sig.to_string().as_str());
                child.state.downcast_mut::<State>().select_left(&new_value);
            }
        } else {
            let new_exp = exp + 3;
            let mut new_val = ExponentialNumber::new(sig, new_exp);

            if !self.bounds.in_bounds(&new_val.to_f64()) {
                new_val = self.bounds.upper;
            } else if new_exp > self.bounds.upper.exponent {
                new_val.exponent = self.bounds.upper.exponent;
            }

            shell.publish((self.on_change)(new_val));
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ScientificSpinBox<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: scientificspinbox::StyleSheet
        + crate::style::scientific_text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ModifierState>()
    }
    fn state(&self) -> tree::State {
        tree::State::new(ModifierState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree {
            tag: self.content.tag(),
            state: self.content.state(),
            children: self.content.children(),
        }]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &[&self.content],
            |state, content| content.diff(state),
            |&content| Tree {
                tag: content.tag(),
                state: content.state(),
                children: content.children(),
            },
        );
    }

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = Padding::from(self.padding);
        let limits = limits
            .width(self.width())
            .height(Length::Shrink)
            .pad(padding);
        let content = self.content.layout(renderer, &limits.loose());
        let txt_size = self.size.unwrap_or_else(|| renderer.default_size());
        let icon_size = txt_size * 3.0 / 4.0;
        let btn_mod = |c| {
            Container::<(), Renderer>::new(Text::new(format!(" {c} ")).size(icon_size))
                .center_y()
                .center_x()
        };
        let mut modifier = if self.padding < DEFAULT_PADDING {
            Row::<(), Renderer>::new()
                .spacing(1)
                .width(Length::Shrink)
                .push(btn_mod('+'))
                .push(btn_mod('-'))
                .layout(renderer, &limits.loose())
        } else {
            Column::<(), Renderer>::new()
                .spacing(1)
                .width(Length::Shrink)
                .push(btn_mod('▲'))
                .push(btn_mod('▼'))
                .layout(renderer, &limits.loose())
        };
        let intrinsic = Size::new(
            content.size().width - 3.0,
            content.size().height.max(modifier.size().height),
        );
        modifier.align(Alignment::End, Alignment::Center, intrinsic);
        let size = limits.resolve(intrinsic);
        Node::with_children(size, vec![content, modifier])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.content.operate(
                &mut tree.children[0],
                layout
                    .children()
                    .next()
                    .expect("NumberInput inner child Textbox was not created."),
                renderer,
                operation,
            );
        });
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        let mut children = layout.children();
        let content = children.next().expect("fail to get content layout");
        let mut mod_children = children
            .next()
            .expect("fail to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("fail to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("fail to get decreate mod layout")
            .bounds();
        let mouse_over_inc = inc_bounds.contains(cursor_position);
        let mouse_over_dec = dec_bounds.contains(cursor_position);
        let modifiers = state.state.downcast_mut::<ModifierState>();
        let mut child = &mut state.children[0];

        if self.bounds.lower.to_f64() == self.bounds.upper.to_f64() {
            return event::Status::Ignored;
        }

        if child.state.downcast_mut::<State>().is_focused() {
            if mouse_over_inc || mouse_over_dec {
                let mut event_status = event::Status::Captured;
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        if mouse_over_dec {
                            modifiers.decrease_pressed = true;
                            self.decrease_val(shell, &mut child, &mut self.content.get_value());
                        } else if mouse_over_inc {
                            modifiers.increase_pressed = true;
                            self.increase_val(shell, &mut child, &mut self.content.get_value());
                        } else {
                            event_status = event::Status::Ignored;
                        }
                    }
                    Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                        if mouse_over_dec {
                            modifiers.decrease_pressed = false;
                        } else if mouse_over_inc {
                            modifiers.increase_pressed = false;
                        } else {
                            event_status = event::Status::Ignored;
                        }
                    }
                    _ => event_status = event::Status::Ignored,
                }
                event_status
            } else {
                match event {
                    Event::Keyboard(keyboard::Event::CharacterReceived(c)) if c.is_numeric() => {
                        let mut new_val = self.value.significand.to_string();
                        match child
                            .state
                            .downcast_mut::<State>()
                            .cursor()
                            .state(&Value::new(&new_val))
                        {
                            cursor::State::Index(idx) => {
                                if self.value.significand == 0.0 {
                                    new_val = c.to_string();
                                } else {
                                    new_val.insert(idx, c);
                                }
                            }
                            cursor::State::Selection { start, end } => {
                                if (0..new_val.len()).contains(&start)
                                    && (0..new_val.len()).contains(&end)
                                {
                                    new_val.replace_range(
                                        if start > end { end..start } else { start..end },
                                        &c.to_string(),
                                    );
                                }
                            }
                        }

                        match f64::from_str(&new_val) {
                            Ok(val) => {
                                if (self.bounds.lower.significand..=self.bounds.upper.significand)
                                    .contains(&val)
                                {
                                    self.value.significand = val;
                                    shell.publish((self.on_change)(self.value));
                                    self.content.on_event(
                                        child,
                                        event.clone(),
                                        content,
                                        cursor_position,
                                        renderer,
                                        clipboard,
                                        shell,
                                    )
                                } else {
                                    event::Status::Ignored
                                }
                            }
                            Err(_) => event::Status::Ignored,
                        }
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. })
                        if child.state.downcast_mut::<State>().is_focused() =>
                    {
                        match key_code {
                            keyboard::KeyCode::Up => {
                                self.increase_val(shell, &mut child, &mut self.content.get_value());
                                event::Status::Captured
                            }
                            keyboard::KeyCode::Down => {
                                self.decrease_val(shell, &mut child, &mut self.content.get_value());
                                event::Status::Captured
                            }
                            _ => self.content.on_event(
                                child,
                                event.clone(),
                                content,
                                cursor_position,
                                renderer,
                                clipboard,
                                shell,
                            ),
                        }
                    }
                    // This section from line 502 to 516 was owned by 13r0ck (https://github.com/13r0ck).
                    Event::Mouse(mouse::Event::WheelScrolled { delta })
                        if layout.bounds().contains(cursor_position) =>
                    {
                        let negative = match delta {
                            mouse::ScrollDelta::Lines { y, .. }
                            | mouse::ScrollDelta::Pixels { y, .. } => y.is_sign_negative(),
                        };
                        if negative {
                            self.increase_val(shell, &mut child, &mut self.content.get_value());
                        } else {
                            self.decrease_val(shell, &mut child, &mut self.content.get_value());
                        }
                        event::Status::Captured
                    }
                    _ => self.content.on_event(
                        child,
                        event,
                        content,
                        cursor_position,
                        renderer,
                        clipboard,
                        shell,
                    ),
                }
            }
        } else {
            match event {
                Event::Keyboard(_) => event::Status::Ignored,
                _ => self.content.on_event(
                    child,
                    event,
                    content,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                ),
            }
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let _content_layout = children.next().expect("fail to get content layout");
        let mut mod_children = children
            .next()
            .expect("fail to get modifiers layout")
            .children();
        let inc_bounds = mod_children
            .next()
            .expect("fail to get increase mod layout")
            .bounds();
        let dec_bounds = mod_children
            .next()
            .expect("fail to get decreate mod layout")
            .bounds();
        let is_mouse_over = bounds.contains(cursor_position);
        let is_decrease_disabled = self.value.to_f64() <= self.bounds.lower.to_f64()
            || self.bounds.lower.to_f64() == self.bounds.upper.to_f64();
        let is_increase_disabled = self.value.to_f64() >= self.bounds.upper.to_f64()
            || self.bounds.lower.to_f64() == self.bounds.upper.to_f64();
        let mouse_over_decrease = dec_bounds.contains(cursor_position);
        let mouse_over_increase = inc_bounds.contains(cursor_position);

        if (mouse_over_decrease && !is_decrease_disabled)
            || (mouse_over_increase && !is_increase_disabled)
        {
            mouse::Interaction::Pointer
        } else if is_mouse_over {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        _viewport: &iced_graphics::Rectangle,
    ) {
        let mut children = layout.children();
        let content_layout = children.next().expect("fail to get content layout");

        self.content.draw(
            &state.children[0],
            renderer,
            theme,
            content_layout,
            cursor_position,
            None,
        );
    }
}

/// The modifier state of a [`NumberInput`].
#[derive(Default, Clone, Debug)]
pub struct ModifierState {
    /// The state of decrease button on a [`NumberInput`].
    pub decrease_pressed: bool,
    /// The state of increase button on a [`NumberInput`].
    pub increase_pressed: bool,
}

fn get_prefix_from_exponent(exp: i8) -> String {
    let mu = "\u{00b5}";

    match exp {
        -12 => String::from("p"),
        -9 => String::from("n"),
        -6 => String::from(mu),
        -3 => String::from("m"),
        0 => String::from(" "),
        3 => String::from("k"),
        6 => String::from("M"),
        9 => String::from("G"),
        12 => String::from("T"),
        _ => String::from("woops"),
    }
}

fn get_step(pos: i32, value: &Value) -> f64 {
    let mut str_val = value.graphemes.join("");
    for c in [" ", "."] {
        str_val = str_val.split(c).next().unwrap().to_string();
    }
    let mut decimal_pos = str_val.len() as i32;
    if pos < decimal_pos {
        decimal_pos = decimal_pos - 1;
    }
    let step = 10_f64.powf((decimal_pos - pos) as f64);

    step
}

impl<'a, Message, Renderer> From<ScientificSpinBox<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: scientificspinbox::StyleSheet
        + crate::style::scientific_text_input::StyleSheet
        + container::StyleSheet
        + text::StyleSheet,
{
    fn from(num_input: ScientificSpinBox<'a, Message, Renderer>) -> Self {
        Element::new(num_input)
    }
}
