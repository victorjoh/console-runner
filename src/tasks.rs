use crate::common::*;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

struct ThreadLogger<M> {
    sender: Sender<TaskMessage<M>>,
    task_name: TaskName,
}

impl<M> Logger<M> for ThreadLogger<M> {
    fn log(&self, message: M) {
        let task_message = TaskMessage {
            message,
            task_name: self.task_name.clone(),
        };
        self.sender.send(task_message).unwrap();
    }
}

pub fn perform<M>(tasks: Vec<Box<dyn Task<M>>>, view: &mut dyn View<M>)
where
    M: Send + 'static,
{
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
        view.show(received);
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

fn spawn_thread<M>(task: Box<dyn Task<M>>, sender: Sender<TaskMessage<M>>)
where
    M: Send + 'static,
{
    let logger = ThreadLogger {
        sender,
        task_name: task.name(),
    };
    thread::spawn(move || task.perform(&logger));
}
