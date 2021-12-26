use super::common::*;
use logos::Lexer;
use logos::Logos;
use std::collections::VecDeque;
use std::io::set_output_capture;
use std::panic;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
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
    fn log(&self, message: &str);
}

pub struct TaskRunner {
    pub thread_count: u16,
    pub view_update_period: u64,
}

struct ThreadLogger {
    sink: LocalStream,
}

impl Logger for ThreadLogger {
    fn log(&self, message: &str) {
        let mut msg = String::from(message);
        msg.push('\n');
        self.send_update(TaskChange::TaskMessage(msg));
    }
}

impl Clone for ThreadLogger {
    fn clone(&self) -> Self {
        Self {
            sink: self.sink.clone(),
        }
    }
}

const TASK_CHANGE_START_TAG: &str = "{TaskChangeStart ";
const TASK_CHANGE_END_TAG: &str = " TaskChangeEnd}";
const NAME_CHANGE_START_TAG: &str = "{NameChangeStart ";
const NAME_CHANGE_END_TAG: &str = " NameChangeEnd}";
const CLOSE_SINK_TAG: &str = "{CloseSink}";

impl ThreadLogger {
    fn new(sink: LocalStream) -> ThreadLogger {
        ThreadLogger { sink }
    }

    fn set_status(&self, status: Status) {
        self.send_update(TaskChange::TaskStatus(status));
    }

    fn send_update(&self, change: TaskChange) {
        let mut buffer = self.sink.lock().unwrap();
        buffer.extend_from_slice(TASK_CHANGE_START_TAG.as_bytes());
        buffer.append(&mut bincode::serialize(&change).unwrap());
        buffer.extend_from_slice(TASK_CHANGE_END_TAG.as_bytes());
    }

    fn switch_task(&self, name: TaskName) {
        let mut buffer = self.sink.lock().unwrap();
        buffer.extend_from_slice(NAME_CHANGE_START_TAG.as_bytes());
        buffer.extend_from_slice(name.as_bytes());
        buffer.extend_from_slice(NAME_CHANGE_END_TAG.as_bytes());
    }

    fn close_sink(&self) {
        let mut buffer = self.sink.lock().unwrap();
        buffer.extend_from_slice(CLOSE_SINK_TAG.as_bytes());
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
        while !thread_sinks.is_empty() {
            thread::sleep(Duration::from_millis(self.view_update_period));
            thread_sinks.retain_mut(|sink| send_changes_to_view(sink, view));
        }
    }
}

fn send_changes_to_view(thread_sink: &mut ThreadSink, view: &mut dyn View) -> bool {
    let mut buffer = thread_sink.print_buffer.lock().unwrap();
    for change in get_task_updates(buffer.as_slice()) {
        match change {
            Change::TaskChange(task_change) => view.update(TaskUpdate {
                task_name: thread_sink.current_task_name.as_ref().unwrap().clone(),
                change: task_change,
            }),
            Change::NameChange(name) => thread_sink.current_task_name = Some(name),
            Change::CloseSink => return false,
        }
    }
    buffer.clear();
    true
}

enum Change {
    TaskChange(TaskChange),
    NameChange(TaskName),
    CloseSink,
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("{TaskChangeStart ")]
    TaskChangeStart,

    #[token(" TaskChangeEnd}")]
    TaskChangeEnd,

    #[token("{NameChangeStart ")]
    NameChangeStart,

    #[token(" NameChangeEnd}")]
    NameChangeEnd,

    #[token("{CloseSink}")]
    CloseSink,

    // A character that is not whitespace or is whitespace. Meaning this will
    // match any single character including newline.
    #[regex("[\\S\\s]")]
    Text,

    #[error]
    Error,
}

fn get_task_updates(buffer: &[u8]) -> Vec<Change> {
    let mut lex = Token::lexer(from_utf8(buffer).unwrap());
    let mut changes: Vec<Change> = Vec::new();

    let mut text = String::new();
    let mut on_text = false;
    while let Some(token) = lex.next() {
        match token {
            Token::TaskChangeStart => {
                if on_text {
                    changes.push(Change::TaskChange(TaskChange::TaskMessage(text.clone())));
                    text.clear();
                    on_text = false;
                }
                changes.push(parse_task_change(&mut lex));
            }
            Token::NameChangeStart => changes.push(parse_name_change(&mut lex)),
            Token::Text => {
                on_text = true;
                text.push_str(lex.slice());
            }
            Token::CloseSink => changes.push(Change::CloseSink),
            _ => panic!(),
        }
    }
    if !text.is_empty() {
        changes.push(Change::TaskChange(TaskChange::TaskMessage(text.clone())));
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

fn run_tasks_in_thread(task_queue: Arc<Mutex<VecDeque<Box<dyn Task>>>>, sink: LocalStream) {
    thread::spawn(move || {
        let logger = ThreadLogger::new(sink.clone());
        while let Some(task) = get_next_task(&task_queue) {
            logger.switch_task(task.name());
            if let Err(_) = spawn_task_thread(task, logger.clone()).join() {
                logger.set_status(Status::Failed(String::from(
                    "Aborting task since thread panicked",
                )));
            }
        }
        logger.close_sink();
    });
}

fn spawn_task_thread(task: Box<dyn Task>, logger: ThreadLogger) -> JoinHandle<()> {
    thread::spawn(move || {
        set_output_capture(Some(logger.sink.clone()));
        run_task(task, &logger);
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
