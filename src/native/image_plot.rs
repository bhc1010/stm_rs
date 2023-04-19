use iced::Color;
use iced_graphics::widget::canvas::{Cache, Cursor, Frame, Geometry, Path, Program};

pub struct Plot<'a, Message> {
    cache: Option<Cache>,
    // TODO: make use of Message?
    on_change: Option<Box<dyn Fn(String) -> Message + 'a>>
}

impl<'a, Message> Plot<'a, Message> {
    pub fn new() -> Self {
        Self {
            cache: None,
            on_change: None
        }
    }
}

impl<'a, Message> Program<Message> for Plot<'a, Message> {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        theme: &iced_native::Theme,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let circle = Path::circle(frame.center(), 10.0);

        frame.fill(&circle, Color::BLACK);

        vec![frame.into_geometry()]
    }
}