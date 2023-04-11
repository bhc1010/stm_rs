use iced::widget::{
    column, row, container, horizontal_rule, horizontal_space, vertical_rule, vertical_space,
    pick_list, text, button, text_input,
    PickList, Text, Button, TextInput,
};
use iced::alignment;
use iced::{Alignment, Element, Length, Renderer, Sandbox, Settings, Font};
use iced_audio::{Normal, NormalParam, XYPad};
use iced_aw::native::NumberInput;
use iced_aw::style::NumberInputStyles;

fn main() -> iced::Result {
    R9Control::run(Settings::default())
}

#[derive(Default)]
struct R9Control {
    lines: Option<u32>,
    size: f64,
    ph_x_pad: NormalParam,
    ph_y_pad: NormalParam,
    x_offset: f64,
    y_offset: f64,
    line_time: f64,
    scan_speed: f64,
    start_voltage: f64,
    stop_voltage: f64,
    step_voltage: f64,
    total_images: u16,
    time_to_finish: f64,
    name: String,
}

#[derive(Debug, Clone)]
enum Message {
    ScanAreaChanged(Normal, Normal),
    LinesChanged(u32),
    SizeChanged(f64),
    XOffsetChanged(f64),
    YOffsetChanged(f64),
    LineTimeChanged(f64),
    TipSpeedChanged(f64),
    StartVoltageChanged(f64),
    StopVoltageChanged(f64),
    StepVoltageChanged(f64),
    AddToQueue,
    NameChanged(String),
}

impl Sandbox for R9Control {
    type Message = Message;

    fn new() -> Self {
        R9Control::default()
    }

    fn title(&self) -> String {
        String::from("R9 External Control")
    }

    fn update(&mut self, msg: Message) {

    }

    fn view(&self) -> Element<Message> {
        let placeholder_plot = XYPad::new(self.ph_x_pad, self.ph_y_pad, Message::ScanAreaChanged);

        let lines_list: PickList<u32, Message, Renderer> =
            pick_list(&LinesOptions::ALL[..], self.lines, Message::LinesChanged)
                .placeholder("Pick a resolution...");

        let size_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.size, 6e-6, Message::SizeChanged)
                .style(NumberInputStyles::Default);

        let x_offset_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.x_offset, 3e-6, Message::XOffsetChanged)
                .style(NumberInputStyles::Default);

        let y_offset_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.y_offset, 3e-6, Message::YOffsetChanged)
                .style(NumberInputStyles::Default);

        let line_time_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.line_time, 10.0, Message::LineTimeChanged)
                .style(NumberInputStyles::Default);

        let scan_speed_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.scan_speed, 10.0, Message::TipSpeedChanged)
                .style(NumberInputStyles::Default);

        let total_images_display: Text<'static, Renderer> = text(self.total_images);

        let time_to_finish_display: Text<'static, Renderer> = text(self.time_to_finish);

        let spacing = 5;
        let scan_area_params = column![
            row!["Lines per frame:", horizontal_space(Length::Fill), lines_list]
                .align_items(Alignment::Center),

            row!["Size:", horizontal_space(Length::Fill), size_input]
                .align_items(Alignment::Center),

            row!["X offset:", horizontal_space(Length::Fill), x_offset_input]
                .align_items(Alignment::Center),

            row!["Y offset:", horizontal_space(Length::Fill), y_offset_input]
                .align_items(Alignment::Center),

            row!["Scan speed:", horizontal_space(Length::Fill), scan_speed_input]
                .align_items(Alignment::Center),

            row!["Line time:", horizontal_space(Length::Fill), line_time_input]
                .align_items(Alignment::Center),
        ]
        .spacing(spacing);

        let start_voltage_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.start_voltage, 10.0, Message::StartVoltageChanged)
                .style(NumberInputStyles::Default);

        let stop_voltage_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.stop_voltage, 10.0, Message::StopVoltageChanged)
                .style(NumberInputStyles::Default);

        let step_voltage_input: NumberInput<'static, f64, Message, Renderer> =
            NumberInput::new(self.step_voltage, 10.0, Message::StepVoltageChanged)
                .style(NumberInputStyles::Default);

        let name: TextInput<'static, Message, Renderer> = 
            text_input(
                "Image set alias..", 
                &self.name, 
                Message::NameChanged)
                .size(30)
                .width(Length::Fill);

        let add_to_queue_button: Button<'static, Message, Renderer> = 
            button("Add to queue")
                .width(Length::Fill)
                .padding(10)
                .on_press(Message::AddToQueue);

        let voltage_params = column![
            row!["Start voltage:", horizontal_space(Length::Fill), start_voltage_input]
                .align_items(Alignment::Center),

            row!["Stop voltage:", horizontal_space(Length::Fill), stop_voltage_input]
                .align_items(Alignment::Center),

            row!["Step voltage:", horizontal_space(Length::Fill), step_voltage_input]
                .align_items(Alignment::Center),

            vertical_space(5),

            row!["Total images:", horizontal_space(Length::Fill), total_images_display]
                .align_items(Alignment::Center),

            vertical_space(4),

            row!["Time to finish:", horizontal_space(Length::Fill), time_to_finish_display]
                .align_items(Alignment::Center),
        ]
        .spacing(spacing);

        let content = row![
            column![
                placeholder_plot,
            ]
                .align_items(Alignment::Center)
                .spacing(10),
            container(column![
                row![
                    play_icon(), pause_icon(), stop_icon()
                ]
                    .spacing(10),
                horizontal_rule(20),
                scan_area_params, 
                horizontal_rule(20), 
                voltage_params,
                vertical_space(Length::Fill),
                name,
                vertical_space(10),
                add_to_queue_button,
                ]
                .align_items(Alignment::Center)
            )
            .max_width(400),
            vertical_rule(20),
            column![
                "Test",
                horizontal_rule(100),
                vertical_space(Length::Fill),
            ]
            .spacing(10),
        ]
        .spacing(20);

        container(content)
            .padding(20)
            .into()
    }
}

// Options for resolution by line count
#[derive(Debug, Clone, Copy)]
enum LinesOptions {}

impl LinesOptions {
    const ALL: [u32; 10] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
}

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}

fn play_icon() -> Text<'static> {
    icon('\u{0041}')
}

fn pause_icon() -> Text<'static> {
    icon('\u{0042}')
}
fn stop_icon() -> Text<'static> {
    icon('\u{0043}')
}