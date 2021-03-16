pub type TaskName = String;

pub trait View {
    fn initialize(&mut self, tasks: Vec<TaskName>);
    fn show(&mut self, task_message: TaskMessage);
}

pub struct TaskMessage {
    pub message: String,
    pub task_name: TaskName,
}