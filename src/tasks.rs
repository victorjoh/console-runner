use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::Sender;

pub trait Task : Send {
    fn perform(&self, logger: &dyn Logger);
}

pub trait Logger {
    fn log(&self, message: String);
}

struct ThreadLogger {
    sender: Sender<String>
}

impl ThreadLogger {
    fn new(sender: Sender<String>) -> ThreadLogger {
        ThreadLogger {sender}
    }
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.sender.send(message).unwrap();
    }
}

pub fn perform(tasks: Vec<Box<dyn Task>>) {
    let (tx, rx) = mpsc::channel();

    for task in tasks {
        let logger = ThreadLogger::new(tx.clone());
        thread::spawn(move || task.perform(&logger));
    }

    for received in rx {
        println!("Got: {}", received);
    }
}
