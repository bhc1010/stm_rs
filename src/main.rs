mod native;
mod style;
mod task;
mod icons;

use std::cmp::min;

use iced::{theme, executor, window};
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, text, text_input, scrollable,
    vertical_rule, vertical_space, Button, PickList, Text, TextInput,
};
use iced::{Alignment, Element, Length, Renderer, Application, Settings, Command, Theme};
use iced_audio::{Normal, NormalParam, XYPad};
use iced_aw::native::NumberInput;
use iced_aw::style::NumberInputStyles;

use style::toolbartheme::ToolBarTheme;
use task::{Task, TaskMessage, TaskState, TaskList};
use icons::*;

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
    tasklist: TaskList
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
    MenuPressed,
    ImagesButtonPressed,
    GraphButtonPressed,
    SettingsButtonPressed,
    TaskMessage(TaskMessage),
    TaskRunning(usize),
    TaskCompleted(usize),
    TaskFailed(usize),
}

impl Application for R9Control {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            R9Control::default(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("STM External Controller")
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            Message::AddToQueue => {
                let id = self.tasklist.tasks.len();
                self.tasklist.tasks.push(Task::new(self.name.clone(), id));
                if self.tasklist.current_task.is_none(){
                    self.tasklist.current_task = Some(0);
                }
                Command::none()
            },
            Message::TaskRunning(idx) => {
                self.tasklist.tasks[idx].state(TaskState::Running);
                Command::none()
            },
            Message::PlayPressed => {
                self.tasklist.current_task.is_some().then(|| {
                    let id = self.tasklist.current_task.unwrap();
                    // send async command to Julia to run the task
                    if self.tasklist.tasks[id].is_idle() {
                        self.tasklist.tasks[id].state(TaskState::Running);
                    }
                });
                Command::none()
            },
            Message::StopPressed => {
                self.tasklist.current_task.is_some().then(|| {
                    let id = self.tasklist.current_task.unwrap();
                    // send async command to Julia to run the task
                    self.tasklist.tasks[id].state(TaskState::Failed);
                    self.tasklist.current_task = Some(min(id + 1, self.tasklist.tasks.len() - 1));
                });
                Command::none()
            },
            Message::LinesChanged(lines) => {
                self.lines = Some(lines);
                Command::none()
            },
            Message::SizeChanged(size) => {
                self.size = size;
                Command::none()
            }
            _ => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let toolbar = container(
            row![
                horizontal_space(2),
                button(menu_icon()).on_press(Message::MenuPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                button(images_icon()).on_press(Message::ImagesButtonPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                button(graph_icon()).on_press(Message::GraphButtonPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                horizontal_space(Length::Fill),
                row![
                    button(play_icon()).on_press(Message::PlayPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                    button(pause_icon()).on_press(Message::PausePressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                    button(stop_icon()).on_press(Message::StopPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
                ],
                horizontal_space(Length::Fill),
                button(gear_icon()).on_press(Message::SettingsButtonPressed).style(theme::Button::Custom(Box::from(ToolBarTheme))),
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

        let name: TextInput<'static, Message, Renderer> = text_input(
            "Choose an alias for the image set...",
            &self.name,
            Message::NameChanged,
        )
        .size(20)
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

        let tasks: Element<_> = column(
            self.tasklist.tasks
                .iter()
                .enumerate()
                .map(|(_, task)| {
                    task.view()
                        .map(move |message| Message::TaskMessage(message))
                })
                .collect(),
        )
        .spacing(10)
        .into();

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
                scrollable(container(tasks).padding(10)),
            ]
            .spacing(20)
        ]
        .align_items(Alignment::Start)
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