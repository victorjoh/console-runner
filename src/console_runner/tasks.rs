use super::common::*;
use logos::Lexer;
use logos::Logos;
use std::collections::VecDeque;
use std::io::set_output_capture;
use std::panic;
use std::str::from_utf8;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;

// We are locked to using this datatype to send messages since it is the
// datatype used for set_output_capture. We use the same buffer for everything
// since we want to get the order correct. For example:
// println!("Hello,");
// logger.log("World!");
// return Ok(Some("123"));
type LocalStream = Arc<Mutex<Vec<u8>>>;
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
    sender: LocalStream,
}

impl Logger for ThreadLogger {
    fn log(&self, message: String) {
        self.send_update(TaskChange::TaskMessage(message));
    }
}

impl Clone for ThreadLogger {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

const TASK_CHANGE_START_TAG: &str = "TaskChangeStart ";
const TASK_CHANGE_END_TAG: &str = " TaskChangeEnd";
const NAME_CHANGE_START_TAG: &str = "NameChangeStart ";
const NAME_CHANGE_END_TAG: &str = " NameChangeEnd";

impl ThreadLogger {
    fn new(sender: LocalStream) -> ThreadLogger {
        ThreadLogger { sender }
    }

    fn set_status(&self, status: Status) {
        self.send_update(TaskChange::TaskStatus(status));
    }

    fn send_update(&self, change: TaskChange) {
        let mut buffer = self.sender.lock().unwrap();
        buffer.extend_from_slice(TASK_CHANGE_START_TAG.as_bytes());
        buffer.append(&mut bincode::serialize(&change).unwrap());
        buffer.extend_from_slice(TASK_CHANGE_END_TAG.as_bytes());
    }

    fn switch_task(&self, name: TaskName) {
        let mut buffer = self.sender.lock().unwrap();
        buffer.extend_from_slice(NAME_CHANGE_START_TAG.as_bytes());
        buffer.extend_from_slice(name.as_bytes());
        buffer.extend_from_slice(NAME_CHANGE_END_TAG.as_bytes());
    }
}

struct ThreadSink {
    print_buffer: LocalStream,
    current_task_name: Option<TaskName>,
}

impl ThreadSink {
    fn new() -> Self {
        ThreadSink {
            print_buffer: Arc::new(Mutex::new(Vec::new())),
            current_task_name: None,
        }
    }
}

impl TaskRunner {
    pub fn run(&self, tasks: Vec<Box<dyn Task>>, view: &mut dyn View) {
        if tasks.len() == 0 {
            return;
        }
        view.initialize(tasks.iter().map(|task| task.name()).collect());
        let task_queue = Arc::new(Mutex::new(VecDeque::from(tasks)));
        let mut thread_sinks: Vec<ThreadSink> =
            (0..self.thread_count).map(|_| ThreadSink::new()).collect();

        for sink in thread_sinks.iter() {
            run_tasks_in_thread(task_queue.clone(), sink.print_buffer.clone());
        }
        loop {
            for thread_sink in thread_sinks.iter_mut() {
                let mut buffer = thread_sink.print_buffer.lock().unwrap();
                for change in get_task_updates(buffer.as_slice()) {
                    match change {
                        Change::TaskChange(task_change) => view.update(TaskUpdate {
                            task_name: thread_sink.current_task_name.as_ref().unwrap().clone(),
                            change: task_change,
                        }),
                        Change::NameChange(name) => thread_sink.current_task_name = Some(name),
                    }
                }
                buffer.clear();
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

enum Change {
    TaskChange(TaskChange),
    NameChange(TaskName),
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("TaskChangeStart ")]
    TaskChangeStart,

    #[token(" TaskChangeEnd")]
    TaskChangeEnd,

    #[token("NameChangeStart ")]
    NameChangeStart,

    #[token(" NameChangeEnd")]
    NameChangeEnd,

    // A character that is not whitespace or is whitespace. Meaning this will
    // match any single character including newline.
    #[regex("[\\S\\s]")]
    Text,

    #[error]
    Error,
}

fn get_task_updates(buffer: &[u8]) -> Vec<Change> {
    let mut lex = Token::lexer(from_utf8(buffer).unwrap());
    let mut changes = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Token::TaskChangeStart => changes.push(parse_task_change(&mut lex)),
            Token::NameChangeStart => changes.push(parse_name_change(&mut lex)),
            Token::Text => changes.push(lex_message(lex.slice(), &mut lex)),
            _ => panic!(),
        }
    }
    return changes;
}

fn parse_task_change(lex: &mut Lexer<Token>) -> Change {
    let mut text = String::new();
    while let Some(token) = lex.next() {
        match token {
            Token::Text => text.push_str(lex.slice()),
            Token::TaskChangeEnd => break,
            _ => panic!(),
        }
    }
    return Change::TaskChange(bincode::deserialize(text.as_bytes()).unwrap());
}

fn parse_name_change(lex: &mut Lexer<Token>) -> Change {
    let mut text = String::new();
    while let Some(token) = lex.next() {
        match token {
            Token::Text => text.push_str(lex.slice()),
            Token::NameChangeEnd => break,
            _ => panic!(),
        }
    }
    return Change::NameChange(text);
}

fn lex_message(start: &str, lex: &mut Lexer<Token>) -> Change {
    let mut text = String::from(start);
    while let Some(token) = lex.next() {
        match token {
            Token::Text => text.push_str(lex.slice()),
            _ => break,
        }
    }
    return Change::TaskChange(TaskChange::TaskMessage(text));
}

// TODO: Remove
// default hook can be found here: std::panic::default_hook;
//
// an alternative to using set_hook could be to redirect stderr and stdout
// specifically for each thread using io::set_output_capture
//
// thread_local! is interesting as well.

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

fn run_tasks_in_thread(task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>, sink: LocalStream) {
    thread::spawn(
        move || {
            while let Err(_) = spawn_task_thread(task_queue.clone(), sink.clone()).join() {}
        },
    );
}

fn spawn_task_thread(
    task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>,
    sink: LocalStream,
) -> JoinHandle<()> {
    let logger = ThreadLogger::new(sink.clone());
    thread::spawn(move || {
        set_output_capture(Some(sink.clone()));
        while let Some(task) = get_next_task(&task_queue) {
            logger.switch_task(task.name());
            run_task(task, &logger);
        }
    })
}

fn get_next_task(task_queue: &Arc<Mutex<VecDeque<Box<dyn Task>>>>) -> Option<Box<dyn Task>> {
    task_queue.lock().unwrap().pop_front()
}

fn run_task(task: Box<dyn Task>, logger: &ThreadLogger) {
    logger.set_status(Status::Running);
    let result = task.run(logger);
    match result {
        Ok(answer) => logger.set_status(Status::Finished(answer)),
        Err(message) => logger.set_status(Status::Failed(message)),
    };
}
