mod core;
mod core_async;
mod native;
mod style;

use iced::keyboard;
use iced_native::subscription;
use iced_native::Event;

use iced::{
    executor, theme,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable,
        text, text_input, vertical_rule, vertical_space, Button, PickList, Text, TextInput,
    },
    Alignment, Application, Command, Element, Length, Renderer, Settings, Subscription, Theme,
};
use iced_graphics::widget::canvas::Canvas;

use crate::core::{
    icons::*,
    stmimage::STMImage,
    task::{Task, TaskList, TaskMessage, TaskState},
    vector2::Vector2,
    jlcontext::JuliaContext
};
use native::image_plot::Plot;
use native::scientificspinbox::{Bounds, ExponentialNumber, ScientificSpinBox};
use style::toolbartheme::ToolBarTheme;

use itertools_num::linspace;
use std::cmp::min;
use crossbeam_channel;

fn main() -> iced::Result {

    R9Control::run(Settings {
        ..Settings::default()
    })

}

struct R9Control {
    lines: Option<u32>,
    size: ExponentialNumber,
    x_offset: ExponentialNumber,
    y_offset: ExponentialNumber,
    line_time: ExponentialNumber,
    // scan_speed: ExponentialNumber,
    start_voltage: ExponentialNumber,
    stop_voltage: ExponentialNumber,
    step_voltage: ExponentialNumber,
    total_images: u16,
    time_to_finish: String,
    name: String,
    tasklist: TaskList<STMImage>,
    jlcontext: JuliaContext
}

impl Default for R9Control {
    fn default() -> Self {

        let jlcontext = JuliaContext::default();
        jlcontext.load::<STMImage>();

        Self {
            lines: None,
            size: ExponentialNumber::new(50.0, -9),
            x_offset: ExponentialNumber::new(0.0, -9),
            y_offset: ExponentialNumber::new(0.0, -9),
            line_time: ExponentialNumber::new(0.0, 0),
            // scan_speed: ExponentialNumber::new(0.0, -9),
            start_voltage: ExponentialNumber::new(0.0, 0),
            stop_voltage: ExponentialNumber::new(0.0, 0),
            step_voltage: ExponentialNumber::new(0.0, 0),
            total_images: 0,
            time_to_finish: String::from(""),
            name: String::from(""),
            tasklist: TaskList::default(),
            jlcontext
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ScanAreaChanged(Vector2<f64>),
    LinesChanged(u32),
    SizeChanged(ExponentialNumber),
    XOffsetChanged(ExponentialNumber),
    YOffsetChanged(ExponentialNumber),
    LineTimeChanged(ExponentialNumber),
    // ScanSpeedChanged(ExponentialNumber),
    StartVoltageChanged(ExponentialNumber),
    StopVoltageChanged(ExponentialNumber),
    StepVoltageChanged(ExponentialNumber),
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
    FocusNext,
    FocusPrevious,
}

impl Application for R9Control {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (R9Control::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("STM External Controller")
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            Message::AddToQueue => {
                let id = self.tasklist.tasks.len();

                let start = self.start_voltage.to_f64();
                let stop = self.stop_voltage.to_f64();
                let step = self.step_voltage.to_f64();
                let n = ((start - stop).abs() / step).floor() as usize;

                let mut images: Vec<STMImage> = vec![];

                for bias in linspace(start, stop, n) {
                    images.push(STMImage::new(
                        self.lines.unwrap_or(256),
                        self.size.to_f64(),
                        self.x_offset.to_f64(),
                        self.y_offset.to_f64(),
                        self.line_time.to_f64(),
                        bias,
                        None,
                    ));
                }

                self.tasklist
                    .tasks
                    .push(Task::new(images, self.name.clone(), id));
                if self.tasklist.current_task.is_none() {
                    self.tasklist.current_task = Some(0);
                }
                Command::none()
            }
            Message::TaskRunning(idx) => {
                self.tasklist.tasks[idx].state(TaskState::Running);
                Command::none()
            }
            Message::PlayPressed => {
                self.tasklist.current_task.is_some().then(|| {
                    let id = self.tasklist.current_task.unwrap();
                    if self.tasklist.tasks[id].is_idle() {
                        self.tasklist.tasks[id].state(TaskState::Running);
                        // send async command to Julia to run the task
                        self.jlcontext.receiver = {
                            let (sender, receiver) = crossbeam_channel::bounded(1);
                            self.jlcontext.julia.try_task(self.tasklist.tasks[id].content()[0].clone(), sender).unwrap();
                            Some(receiver)
                        };

                        let result = self.jlcontext.receiver.as_ref().unwrap().recv().unwrap().unwrap();
                        println!("{:?}", result);
                    }
                });
                Command::none()
            }
            Message::StopPressed => {
                self.tasklist.current_task.is_some().then(|| {
                    let id = self.tasklist.current_task.unwrap();
                    // send async command to Julia to run the task
                    self.tasklist.tasks[id]
                        .state(TaskState::Failed(String::from("Interrupted by user.")));
                    self.tasklist.current_task = Some(min(id + 1, self.tasklist.tasks.len() - 1));
                });
                Command::none()
            }
            Message::LinesChanged(lines) => {
                self.lines = Some(lines);
                self.time_to_finish = calculate_time_remaining(
                    self.lines.unwrap_or(0) as f64,
                    self.line_time.to_f64(),
                    self.total_images as f64,
                );
                Command::none()
            }
            Message::SizeChanged(size) => {
                self.size = size;
                Command::none()
            }
            Message::XOffsetChanged(x_offset) => {
                self.x_offset = x_offset;
                Command::none()
            }
            Message::YOffsetChanged(y_offset) => {
                self.y_offset = y_offset;
                Command::none()
            }
            Message::LineTimeChanged(line_time) => {
                self.line_time = line_time;
                self.time_to_finish = calculate_time_remaining(
                    self.lines.unwrap_or(0) as f64,
                    self.line_time.to_f64(),
                    self.total_images as f64,
                );
                Command::none()
            }
            // Message::ScanSpeedChanged(scan_speed) => {
            //     self.scan_speed = scan_speed;
            //     Command::none()
            // }
            Message::StartVoltageChanged(start_voltage) => {
                self.start_voltage = start_voltage;
                self.total_images = calculate_total_images(
                    self.start_voltage.to_f64(),
                    self.stop_voltage.to_f64(),
                    self.step_voltage.to_f64(),
                );
                self.time_to_finish = calculate_time_remaining(
                    self.lines.unwrap_or(0) as f64,
                    self.line_time.to_f64(),
                    self.total_images as f64,
                );
                Command::none()
            }
            Message::StopVoltageChanged(stop_voltage) => {
                self.stop_voltage = stop_voltage;
                self.total_images = calculate_total_images(
                    self.start_voltage.to_f64(),
                    self.stop_voltage.to_f64(),
                    self.step_voltage.to_f64(),
                );
                self.time_to_finish = calculate_time_remaining(
                    self.lines.unwrap_or(0) as f64,
                    self.line_time.to_f64(),
                    self.total_images as f64,
                );
                Command::none()
            }
            Message::StepVoltageChanged(step_voltage) => {
                self.step_voltage = step_voltage;
                self.total_images = calculate_total_images(
                    self.start_voltage.to_f64(),
                    self.stop_voltage.to_f64(),
                    self.step_voltage.to_f64(),
                );
                self.time_to_finish = calculate_time_remaining(
                    self.lines.unwrap_or(0) as f64,
                    self.line_time.to_f64(),
                    self.total_images as f64,
                );
                Command::none()
            }
            Message::NameChanged(value) => {
                self.name = value;
                Command::none()
            }
            Message::FocusNext => iced::widget::focus_next(),
            Message::FocusPrevious => iced::widget::focus_previous(),
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                } => Some(if modifiers.shift() {
                    Message::FocusPrevious
                } else {
                    Message::FocusNext
                }),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let toolbar = container(
            row![
                horizontal_space(2),
                button(menu_icon())
                    .on_press(Message::MenuPressed)
                    .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                button(images_icon())
                    .on_press(Message::ImagesButtonPressed)
                    .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                button(graph_icon())
                    .on_press(Message::GraphButtonPressed)
                    .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                horizontal_space(Length::Fill),
                row![
                    button(play_icon())
                        .on_press(Message::PlayPressed)
                        .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                    button(pause_icon())
                        .on_press(Message::PausePressed)
                        .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                    button(stop_icon())
                        .on_press(Message::StopPressed)
                        .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                ],
                horizontal_space(Length::Fill),
                horizontal_space(92.0),
                button(gear_icon())
                    .on_press(Message::SettingsButtonPressed)
                    .style(theme::Button::Custom(Box::from(ToolBarTheme))),
                horizontal_space(2)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        )
        .padding(8)
        .style(theme::Container::Custom(Box::from(ToolBarTheme)));

        let scan_area = Canvas::new(Plot::<Message>::new())
            .width(Length::Fill)
            .height(Length::Fill);

        let lines_list: PickList<u32, Message, Renderer> =
            pick_list(&LinesOptions::ALL[..], self.lines, Message::LinesChanged)
                .placeholder("Pick a resolution...");

        let size_input = ScientificSpinBox::new(
            self.size,
            Bounds::new(
                ExponentialNumber::new(210.0, -12),
                ExponentialNumber::new(2.1, -6),
            ),
            "m",
            Message::SizeChanged,
        );

        let x_offset_input = ScientificSpinBox::new(
            self.x_offset,
            Bounds::new(
                ExponentialNumber::new(-1.05, -6),
                ExponentialNumber::new(1.05, -6),
            ),
            "m",
            Message::XOffsetChanged,
        );

        let y_offset_input = ScientificSpinBox::new(
            self.y_offset,
            Bounds::new(
                ExponentialNumber::new(-1.05, -6),
                ExponentialNumber::new(1.06, -6),
            ),
            "m",
            Message::YOffsetChanged,
        );

        let line_time_input = ScientificSpinBox::new(
            self.line_time,
            Bounds::new(
                ExponentialNumber::new(102.4, -3),
                ExponentialNumber::new(100.0, 0),
            ),
            "s",
            Message::LineTimeChanged,
        );

        // let scan_speed_input = ScientificSpinBox::new(
        //     self.scan_speed,
        //     Bounds::new(
        //         ExponentialNumber::new(2.1, -12),
        //         ExponentialNumber::new(2.051, -9),
        //     ),
        //     "m/s",
        //     Message::ScanSpeedChanged,
        // );

        let total_images_display: Text<'static, Renderer> = text(self.total_images);

        let time_to_finish_display: Text<'static, Renderer> = text(&self.time_to_finish);

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
            // row![
            //     "Scan speed:",
            //     horizontal_space(Length::Fill),
            //     scan_speed_input
            // ]
            // .align_items(Alignment::Center),
            row![
                "Line time:",
                horizontal_space(Length::Fill),
                line_time_input
            ]
            .align_items(Alignment::Center),
        ]
        .spacing(spacing);

        let start_voltage_input = ScientificSpinBox::new(
            self.start_voltage,
            Bounds::new(
                ExponentialNumber::new(-5.0, 0),
                ExponentialNumber::new(5.0, 0),
            ),
            "V",
            Message::StartVoltageChanged,
        );

        let stop_voltage_input = ScientificSpinBox::new(
            self.stop_voltage,
            Bounds::new(
                ExponentialNumber::new(-5.0, 0),
                ExponentialNumber::new(5.0, 0),
            ),
            "V",
            Message::StopVoltageChanged,
        );

        let step_voltage_input = ScientificSpinBox::new(
            self.step_voltage,
            Bounds::new(
                ExponentialNumber::new(-5.0, 0),
                ExponentialNumber::new(5.0, 0),
            ),
            "V",
            Message::StepVoltageChanged,
        );

        let name: TextInput<'static, Message, Renderer> =
            text_input("Choose an alias for the image set...", &self.name)
                .on_input(Message::NameChanged)
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
            self.tasklist
                .tasks
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
                container(scan_area).max_width(1000),
                container(
                    column![
                        scrollable(column![
                            scan_area_params,
                            horizontal_rule(20),
                            voltage_params
                        ]),
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

impl Drop for R9Control {
    fn drop(&mut self) {
        std::mem::drop(&self.jlcontext.julia);
        std::mem::drop(&self.jlcontext.handle);
    }
}

fn calculate_total_images(start: f64, stop: f64, step: f64) -> u16 {
    if step != 0.0 {
        ((start - stop) / step).abs() as u16
    } else {
        0 as u16
    }
}

fn calculate_time_remaining(lines_per_frame: f64, line_time: f64, num_images: f64) -> String {
    let mut secs = lines_per_frame * line_time * num_images;

    let days = (secs / (60. * 60. * 24.)).floor();
    secs = secs - days * (60. * 60. * 24.);

    let hrs = (secs / (60. * 60.)).floor();
    secs = secs - hrs * (60. * 60.);

    let mins = (secs / 60.0).floor();
    secs = (secs - mins * 60.0).floor();

    if days > 0.0 {
        format!("{:02}:{:02}:{:02}:{:02}", days, hrs, mins, secs)
    } else {
        format!("{:02}:{:02}:{:02}", hrs, mins, secs)
    }
}

// Options for resolution by line count
#[derive(Debug, Clone, Copy)]
enum LinesOptions {}

impl LinesOptions {
    const ALL: [u32; 10] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
}
