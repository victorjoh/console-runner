use serde::{Serialize, Deserialize};

pub trait View {
    fn initialize(&mut self, tasks: Vec<TaskName>);
    fn update(&mut self, task_update: TaskUpdate);
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdate {
    pub task_name: TaskName,
    pub change: TaskChange
}

#[derive(Serialize, Deserialize)]
pub enum TaskChange {
    TaskStatus(Status),
    TaskMessage(LogMessage)
}

pub type TaskName = String;
pub type LogMessage = String;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Pending,
    Running,
    Finished(Answer),
    Failed(Error)
}

pub type Answer = Option<String>;
pub type Error = String;
