pub trait Task<M>: Send {
    fn perform(&self, logger: &dyn Logger<M>);
    fn name(&self) -> TaskName;
}

pub trait Logger<M> {
    fn log(&self, message: M);
}

pub type TaskName = String;

pub trait View<M> {
    fn initialize(&mut self, tasks: Vec<TaskName>);
    fn show(&mut self, task_message: TaskMessage<M>);
}

pub struct TaskMessage<M> {
    pub message: M,
    pub task_name: TaskName,
}