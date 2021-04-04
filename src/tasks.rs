use crate::common::*;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub trait Task: Send {
    fn run(&self, logger: &dyn Logger);
    fn name(&self) -> TaskName;
}

pub trait Logger {
    fn log(&self, message: String);
}

pub struct TaskRunner {
    pub thread_count: u16,
}

struct ThreadLogger {
    sender: Sender<TaskUpdate>,
    task_name: Option<TaskName>,
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.send_update(TaskChange::TaskMessage(message));
    }
}

impl ThreadLogger {
    fn new(sender: Sender<TaskUpdate>) -> ThreadLogger {
        ThreadLogger {
            sender,
            task_name: None,
        }
    }

    fn set_task_name(&mut self, task_name: TaskName) {
        self.task_name = Some(task_name);
    }

    fn set_status(&self, status: Status) {
        self.send_update(TaskChange::TaskStatus(status));
    }

    fn send_update(&self, change: TaskChange) {
        let name = match &self.task_name {
            None => panic!("No task assigned to the thread logger"),
            Some(name) => name.clone(),
        };
        let task_status = TaskUpdate {
            task_name: name,
            change,
        };
        self.sender.send(task_status).unwrap();
    }
}

impl TaskRunner {
    pub fn run(&self, tasks: Vec<Box<dyn Task>>, view: &mut dyn View) {
        if tasks.len() == 0 {
            return;
        }
        view.initialize(tasks.iter().map(|task| task.name()).collect());
        let (a_sender, receiver) = mpsc::channel();
        let senders = multiply_senders(a_sender, self.thread_count);
        let task_queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
        senders
            .into_iter()
            .for_each(|sender| run_task_in_thread(Arc::clone(&task_queue), sender));
        for received in receiver {
            view.update(received);
        }
    }
}

fn multiply_senders<T>(a_sender: Sender<T>, amount: u16) -> Vec<Sender<T>> {
    if amount == 0 {
        return Vec::new();
    } else if amount == 1 {
        return vec![a_sender];
    }

    let mut senders = Vec::new();
    for _ in 0..=amount - 2 {
        senders.push(a_sender.clone());
    }
    senders.push(a_sender);
    return senders;
}

fn run_task_in_thread(task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>, sender: Sender<TaskUpdate>) {
    let mut logger = ThreadLogger::new(sender);
    thread::spawn(move || {
        while let Some(task) = get_next_task(&task_queue) {
            logger.set_task_name(task.name());
            run_task(task, &logger);
        }
    });
}

fn get_next_task(task_queue: &Arc<Mutex<VecDeque<Box<dyn Task>>>>) -> Option<Box<dyn Task>> {
    task_queue.lock().unwrap().pop_front()
}

fn run_task(task: Box<dyn Task>, logger: &ThreadLogger) {
    logger.set_status(Status::Running);
    task.run(logger);
    logger.set_status(Status::Finished);
}
