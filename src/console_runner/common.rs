pub trait View {
    fn initialize(&mut self, tasks: Vec<TaskName>);
    fn update(&mut self, task_update: TaskUpdate);
}

pub struct TaskUpdate {
    pub task_name: TaskName,
    pub change: TaskChange
}

pub enum TaskChange {
    TaskStatus(Status),
    TaskMessage(LogMessage)
}

pub type TaskName = String;
pub type LogMessage = String;

pub type Answer = String;

pub enum Status {
    Pending,
    Running,
    Finished(Option<Answer>),
}
