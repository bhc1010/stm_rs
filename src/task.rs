use iced::widget::{horizontal_space, row, text};
use iced::{Element, Length};

use crate::native::taskdisplay::TaskDisplay;
use crate::style::taskdisplay::TaskDisplayStyles;
use crate::icons::*;


pub struct TaskList {
    pub tasks: Vec<Task>,
    pub current_task: Option<usize>
}

impl Default for TaskList {
    fn default() -> Self {
        Self { 
            tasks: Vec::default(), 
            current_task: None 
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    description: String,
    index: usize,
    state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed,
    Edit,
    Delete,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

impl Task {
    pub fn new(description: String, index: usize) -> Self {
        Self {
            description,
            index,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, msg: TaskMessage) {
        match msg {
            TaskMessage::Completed => {
                self.state = TaskState::Completed;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<TaskMessage> {
        match self.state {
            TaskState::Idle => TaskDisplay::new(row![
                circle_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .into(),
            TaskState::Running => TaskDisplay::new(row![
                running_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
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
            TaskState::Failed => TaskDisplay::new(row![
                failed_icon(),
                horizontal_space(Length::Fill),
                text(self.description.clone()).size(20),
                horizontal_space(Length::Fill),
                three_dots_vertical_icon(),
            ])
            .style(TaskDisplayStyles::Failed)
            .into()
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
}
