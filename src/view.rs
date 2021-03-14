use crate::common::*;

pub struct Console {}

impl View<String> for Console {
    fn initialize(&self, tasks: Vec<TaskName>) {}

    fn show(&self, task_message: TaskMessage<String>) {
        println!("{}: {}", task_message.task_name, task_message.message);
    }
}
