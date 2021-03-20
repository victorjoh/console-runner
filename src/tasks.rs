use crate::common::*;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub trait Task: Send {
    fn perform(&self, logger: &dyn Logger);
    fn name(&self) -> TaskName;
}

pub trait Logger {
    fn log(&self, message: String);
}

struct ThreadLogger {
    sender: Sender<TaskUpdate>,
    task_name: TaskName,
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.send_update(TaskChange::TaskMessage(message));
    }
}

impl ThreadLogger {
    fn set_status(&self, status: Status) {
        self.send_update(TaskChange::TaskStatus(status));
    }

    fn send_update(&self, change: TaskChange) {
        let task_status = TaskUpdate {
            task_name: self.task_name.clone(),
            change,
        };
        self.sender.send(task_status).unwrap();
    }
}

pub fn perform(tasks: Vec<Box<dyn Task>>, view: &mut dyn View) {
    if tasks.len() == 0 {
        return;
    }
    view.initialize(tasks.iter().map(|task| task.name()).collect());

    let (a_sender, receiver) = mpsc::channel();
    let senders = multiply_senders(a_sender, tasks.len());

    tasks
        .into_iter()
        .zip(senders.into_iter())
        .for_each(|(task, sender)| spawn_thread(task, sender));

    for received in receiver {
        view.update(received);
    }
}

fn multiply_senders<T>(a_sender: Sender<T>, amount: usize) -> Vec<Sender<T>> {
    let mut senders = Vec::new();
    for _ in 0..=amount - 2 {
        senders.push(a_sender.clone());
    }
    senders.push(a_sender);
    return senders;
}

fn spawn_thread(task: Box<dyn Task>, sender: Sender<TaskUpdate>) {
    let logger = ThreadLogger {
        sender,
        task_name: task.name(),
    };
    thread::spawn(move || perform_thread(task, logger));
}

fn perform_thread(task: Box<dyn Task>, logger: ThreadLogger) {
    logger.set_status(Status::Running);
    task.perform(&logger);
    logger.set_status(Status::Finished);
}
