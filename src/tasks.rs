use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub trait Task: Send {
    fn perform(&self, logger: &dyn Logger);
}

pub trait Logger {
    fn log(&self, message: String);
}

struct ThreadLogger {
    sender: Sender<String>,
}

impl ThreadLogger {
    fn new(sender: Sender<String>) -> ThreadLogger {
        ThreadLogger { sender }
    }
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.sender.send(message).unwrap();
    }
}

pub fn perform(tasks: Vec<Box<dyn Task>>, logger: &dyn Logger) {
    if tasks.len() == 0 {
        return;
    }

    let (first_sender, receiver) = mpsc::channel();
    let senders = multiply_senders(first_sender, tasks.len());

    tasks
        .into_iter()
        .zip(senders.into_iter())
        .for_each(|(task, sender)| spawn_thread(task, sender));

    for received in receiver {
        logger.log(received);
    }
}

fn multiply_senders<T>(original_sender: Sender<T>, amount: usize) -> Vec<Sender<T>> {
    let mut senders = Vec::new();
    for _ in 0..=amount - 2 {
        senders.push(original_sender.clone());
    }
    senders.push(original_sender);
    return senders;
}

fn spawn_thread(task: Box<dyn Task>, sender: Sender<String>) {
    let logger = ThreadLogger::new(sender);
    thread::spawn(move || task.perform(&logger));
}
