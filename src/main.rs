mod native;
mod style;

use iced::subscription::run;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, text, text_input,
    vertical_rule, vertical_space, Button, PickList, Text, TextInput,
};
use iced::{alignment, color, theme, Theme};
use iced::{Alignment, Element, Font, Length, Renderer, Sandbox, Settings};
use iced_audio::{Normal, NormalParam, XYPad};
use iced_aw::native::NumberInput;
use iced_aw::style::NumberInputStyles;
use native::task::Task;
use style::task::TaskStyles;

fn main() -> iced::Result {
    R9Control::run(Settings {
        ..Settings::default()
    })
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
    PlayPressed,
    PausePressed,
    StopPressed,
}

impl Sandbox for R9Control {
    type Message = Message;

    fn new() -> Self {
        R9Control::default()
    }

    fn title(&self) -> String {
        String::from("STM External Controller")
    }

    fn update(&mut self, msg: Message) {}

    fn view(&self) -> Element<Message> {
        let toolbar = container(
            row![
                horizontal_space(2),
                menu_icon(),
                images_icon(),
                graph_icon(),
                horizontal_space(Length::Fill),
                row![
                    button(play_icon()).on_press(Message::PlayPressed),
                    button(pause_icon()).on_press(Message::PausePressed),
                    button(stop_icon()).on_press(Message::StopPressed),
                ],
                horizontal_space(Length::Fill),
                gear_icon(),
                horizontal_space(2)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        )
        .padding(8)
        .style(theme::Container::Custom(Box::from(ToolBarTheme)));

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
            row![
                "Lines per frame:",
                horizontal_space(Length::Fill),
                lines_list
            ]
            .align_items(Alignment::Center),
            row!["Size:", horizontal_space(Length::Fill), size_input]
                .align_items(Alignment::Center),
            row!["X offset:", horizontal_space(Length::Fill), x_offset_input]
                .align_items(Alignment::Center),
            row!["Y offset:", horizontal_space(Length::Fill), y_offset_input]
                .align_items(Alignment::Center),
            row![
                "Scan speed:",
                horizontal_space(Length::Fill),
                scan_speed_input
            ]
            .align_items(Alignment::Center),
            row![
                "Line time:",
                horizontal_space(Length::Fill),
                line_time_input
            ]
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
            text_input("Choose an alias for the image set...", &self.name, Message::NameChanged)
                .size(25)
                .width(Length::Fill);

        let add_to_queue_button: Button<'static, Message, Renderer> = button("Add to queue")
            .width(Length::Fill)
            .padding(10)
            .on_press(Message::AddToQueue);

        let voltage_params = column![
            row![
                "Start voltage:",
                horizontal_space(Length::Fill),
                start_voltage_input
            ]
            .align_items(Alignment::Center),
            row![
                "Stop voltage:",
                horizontal_space(Length::Fill),
                stop_voltage_input
            ]
            .align_items(Alignment::Center),
            row![
                "Step voltage:",
                horizontal_space(Length::Fill),
                step_voltage_input
            ]
            .align_items(Alignment::Center),
            vertical_space(5),
            row![
                "Total images:",
                horizontal_space(Length::Fill),
                total_images_display
            ]
            .align_items(Alignment::Center),
            vertical_space(4),
            row![
                "Time to finish:",
                horizontal_space(Length::Fill),
                time_to_finish_display
            ]
            .align_items(Alignment::Center),
        ]
        .spacing(spacing);

        let finished_task = Task::new(
            row![
                finished_icon(),
                horizontal_space(Length::Fill),
                text("TaS2 2.5V - 2.9V, 100nm").size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon()
            ]
            .align_items(Alignment::Center),
            100.0
        )
        .style(TaskStyles::Finished);

        let error_task = Task::new(
            row![
                error_icon(),
                horizontal_space(Length::Fill),
                text("TaS2 2.5V - 2.9V, 50nm").size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon()
            ]
            .align_items(Alignment::Center),
            23.0
        )
        .style(TaskStyles::Error);

        let running_task = Task::new(
            row![
                running_icon(),
                horizontal_space(Length::Fill),
                text("TaS2 2.5V - 2.9V, 50nm").size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon()
            ]
            .align_items(Alignment::Center),
            67.0
        )
        .style(TaskStyles::Running);

        let waiting_task = Task::new(
            row![
                circle_icon(),
                horizontal_space(Length::Fill),
                text("TaS2 2.5V - 2.9V, 10nm").size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon()
            ]
            .align_items(Alignment::Center),
            0.0
        )
        .style(TaskStyles::Waiting);


        let content = column![
            toolbar,
            row![
                column![placeholder_plot,]
                    .align_items(Alignment::Center)
                    .spacing(10),
                container(
                    column![
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
                container(
                    column![
                        finished_task,
                        error_task,
                        running_task,
                        waiting_task,
                        vertical_space(Length::Fill),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                )
                .padding(10),
            ]
            .spacing(20)
        ]
        .align_items(Alignment::Center)
        .spacing(20);

        container(content).padding(20).into()
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
    bytes: include_bytes!("../fonts/icomoon.ttf"),
};

fn icon(unicode: char, size: f32) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(size)
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(size)
}

const DEFAULT_ICON_SIZE: f32 = 25.0;

fn play_icon() -> Text<'static> {
    icon('\u{e918}', 32.0)
}

fn pause_icon() -> Text<'static> {
    icon('\u{e919}', 32.0)
}

fn stop_icon() -> Text<'static> {
    icon('\u{e90f}', 32.0)
}

fn finished_icon() -> Text<'static> {
    icon('\u{e904}', 32.0)
}

fn error_icon() -> Text<'static> {
    icon('\u{e906}', 32.0)
}

fn running_icon() -> Text<'static> {
    icon('\u{e91d}', DEFAULT_ICON_SIZE)
}

fn circle_icon() -> Text<'static> {
    icon('\u{e90a}', 32.0)
}

fn menu_icon() -> Text<'static> {
    icon('\u{e90d}', DEFAULT_ICON_SIZE)
}

fn images_icon() -> Text<'static> {
    icon('\u{e91c}', DEFAULT_ICON_SIZE)
}

fn graph_icon() -> Text<'static> {
    icon('\u{e91f}', DEFAULT_ICON_SIZE)
}

fn gear_icon() -> Text<'static> {
    icon('\u{e920}', DEFAULT_ICON_SIZE)
}

fn three_dots_vertical_icon() -> Text<'static> {
    icon('\u{e90c}', DEFAULT_ICON_SIZE)
}

struct ToolBarTheme;

impl container::StyleSheet for ToolBarTheme {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: color!(94, 124, 226, 0.05).into(),
            border_radius: 20.0,
            ..Default::default()
        }
    }
}
