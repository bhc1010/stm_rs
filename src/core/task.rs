use iced::widget::{horizontal_space, row, text};
use iced::{Element, Length};

use crate::core::icons::*;
use crate::native::taskdisplay::TaskDisplay;
use crate::style::taskdisplay::TaskDisplayStyles;

pub struct TaskList<T> {
    pub tasks: Vec<Task<T>>,
    pub current_task: Option<usize>,
}

impl<T> Default for TaskList<T> {
    fn default() -> Self {
        Self {
            tasks: Vec::default(),
            current_task: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task<T> {
    content: Vec<T>,
    description: String,
    index: usize,
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Finished,
    Edit,
    Delete,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T> Task<T> {
    pub fn new(content: Vec<T>, description: String, index: usize) -> Self {
        Self {
            content,
            description,
            index,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, msg: TaskMessage) {
        match msg {
            TaskMessage::Finished => {
                self.state = TaskState::Completed;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => TaskDisplay::new(row![
                circle_icon(),
                horizontal_space(Length::Fill),
                text(&self.description).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .value(0.0)
            .into(),
            TaskState::Running => TaskDisplay::new(row![
                running_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .value(50.0)
            .style(TaskDisplayStyles::Running)
            .into(),
            TaskState::Completed => TaskDisplay::new(row![
                completed_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .style(TaskDisplayStyles::Completed)
            .into(),
            TaskState::Failed(error) => TaskDisplay::new(row![
                failed_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .value(66.0)
            .style(TaskDisplayStyles::Failed)
            .into(),
        }
    }

    pub fn state(&mut self, state: TaskState) {
        self.state = state
    }

    pub fn is_idle(&self) -> bool {
        match self.state {
            TaskState::Idle => true,
            _ => false,
        }
    }

    pub fn content(&self) -> &Vec<T> {
        &self.content
    }
}
