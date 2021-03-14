use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub trait Task<M>: Send {
    fn perform(&self, logger: &dyn Logger<M>);
    fn name(&self) -> TaskName;
}

pub trait Logger<M> {
    fn log(&self, message: M);
}

pub type TaskName = String;

pub trait View<M> {
    fn initialize(&self, tasks: Vec<TaskName>);
    fn show(&self, task_message: TaskMessage<M>);
}

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

pub struct TaskMessage<M> {
    pub message: M,
    pub task_name: TaskName,
}

pub fn perform<M>(tasks: Vec<Box<dyn Task<M>>>, view: &dyn View<M>)
where
    M: Send + 'static,
{
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
        view.show(received);
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
