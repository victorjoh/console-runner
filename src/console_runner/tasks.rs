use super::common::*;
use lazy_static;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::panic;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use crossbeam_channel::{Sender, Receiver};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::ThreadId;
use thread_id;

pub type TaskResult = Result<Answer, Error>;

pub trait Task: Send {
    fn run(&self, logger: &dyn Logger) -> TaskResult;
    fn name(&self) -> TaskName;
}

pub trait Logger {
    fn log(&self, message: String);
}

pub struct TaskRunner {
    pub thread_count: u16,
}

struct ThreadLogger {
    sender: SyncSender<TaskUpdate>,
    task_name: Option<TaskName>,
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.send_update(TaskChange::TaskMessage(message));
    }
}

impl ThreadLogger {
    fn new(sender: SyncSender<TaskUpdate>) -> ThreadLogger {
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

lazy_static! {
    static ref LOGGERS: RwLock<HashMap<ThreadId, RwLock<ThreadLogger>>> = HashMap::new().into();
}

impl TaskRunner {
    pub fn run(&self, tasks: Vec<Box<dyn Task>>, view: &mut dyn View) {
        if tasks.len() == 0 {
            return;
        }
        view.initialize(tasks.iter().map(|task| task.name()).collect());
        let (a_sender, receiver) = mpsc::sync_channel(self.thread_count.into());
        let senders = multiply_senders(a_sender, self.thread_count);
        let task_queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
        panic::set_hook(Box::new(|info| {
            // default hook can be found here: std::panic::default_hook;
            //
            // an alternative to using set_hook could be to redirect stderr and
            // stdout specifically for each thread using io::set_output_capture
            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                },
            };

            let thread = thread::current();
            match LOGGERS.try_read() {
                Ok(loggers) => {
                    if let Some(logger) = loggers.get(&thread.id()) {
                        logger.read().unwrap().set_status(Status::Failed(format!(
                            "thread '{}' panicked at '{}', {}",
                            thread.name().unwrap_or("<unnamed>"),
                            msg,
                            info.location().unwrap()
                        )));
                    }
                }
                Err(error) => {
                    println!("failed to acquire logger: {0}", error.to_string());
                }
            }
        }));
        let (tx, rx) = crossbeam_channel::unbounded();
        let nbr_of_initilized_threads = Arc::new(Mutex::new(0));
        senders
            .into_iter()
            .for_each(|sender| run_task_in_thread(Arc::clone(&task_queue), sender, Arc::clone(&nbr_of_initilized_threads), tx.clone(), rx.clone(), self.thread_count));
        for received in receiver {
            view.update(received);
        }
    }
}

fn multiply_senders<T>(a_sender: SyncSender<T>, amount: u16) -> Vec<SyncSender<T>> {
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

fn run_task_in_thread(
    task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>,
    sender: SyncSender<TaskUpdate>,
    nbr_of_initilized_threads: Arc<Mutex<u16>>,
    all_initialized_sender: Sender<()>,
    all_initialized_receiver: Receiver<()>,
    thread_count: u16,
) {
    let logger = ThreadLogger::new(sender);
    thread::spawn(move || {
        let thread_id = thread::current().id();
        let id = thread_id::get();
        insert_logger(thread_id, logger);
        report_initialization_finished(nbr_of_initilized_threads, all_initialized_sender, thread_count);
        all_initialized_receiver.recv().unwrap();
        while let Some(task) = get_next_task(&task_queue) {
            let loggers = LOGGERS.read().unwrap();
            update_task_name(&loggers, thread_id, &task);
            let logger = loggers.get(&thread_id).unwrap().read().unwrap();
            logger.log(format!("running on thread: '{}'", id));
            run_task(task, &logger);
        }
    });
}

fn update_task_name(
    loggers: &std::sync::RwLockReadGuard<HashMap<ThreadId, RwLock<ThreadLogger>>>,
    thread_id: ThreadId,
    task: &Box<dyn Task>,
) {
    let mut logger = loggers.get(&thread_id).unwrap().write().unwrap();
    logger.set_task_name(task.name());
}

fn insert_logger(thread_id: ThreadId, logger: ThreadLogger) {
    let mut loggers = LOGGERS.write().unwrap();
    loggers.insert(thread_id, logger.into());
}

fn report_initialization_finished(nbr_of_initilized_threads: Arc<Mutex<u16>>, all_initilized: Sender<()>, thread_count: u16) {
    let mut nbr_of_initilized_threads = nbr_of_initilized_threads.lock().unwrap();
    *nbr_of_initilized_threads += 1;
    if *nbr_of_initilized_threads == thread_count {
        for _ in 0..thread_count {
            all_initilized.send(()).unwrap();
        }
    }
}

fn get_next_task(task_queue: &Arc<Mutex<VecDeque<Box<dyn Task>>>>) -> Option<Box<dyn Task>> {
    task_queue.lock().unwrap().pop_front()
}

fn run_task(task: Box<dyn Task>, logger: &ThreadLogger) {
    logger.set_status(Status::Running);
    // let unwind_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
    let result = task.run(logger);
    match result {
        Ok(answer) => logger.set_status(Status::Finished(answer)),
        Err(message) => logger.set_status(Status::Failed(message)),
    };
    // }));

    /*if let Err(panic) = unwind_result {
        match panic.downcast::<&str>() {
            Ok(panic_msg) => {
                logger.set_status(Status::Failed(format!("Task panicked: {}", panic_msg)))
            }
            Err(_) => {
                logger.set_status(Status::Failed(String::from("Task panicked")));
            }
        }
    };*/
}
