mod console_runner;

use console_runner::common::*;
use console_runner::tasks::{Logger, Task, TaskResult, TaskRunner};
use console_runner::view::*;
use std::thread;

use std::time::Duration;

struct Problem {
    vals: Vec<String>,
    name: TaskName,
    result: TaskResult,
}

impl Task for Problem {
    fn run(&self, logger: &dyn Logger) -> TaskResult {
        for val in &self.vals {
            logger.log(val.to_string());
            thread::sleep(Duration::from_secs(1));
        }
        return self.result.clone();
    }

    fn name(&self) -> TaskName {
        self.name.clone()
    }
}

struct ErrorProblem {
    name: TaskName,
}

pub fn run(day: Option<usize>, session: Option<String>) {
    let p1 = Problem {
        vals: vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("goodbye"),
            String::from("to"),
        ],
        name: String::from("p1"),
        result: Err(String::from("Something went wrong!")),
    };
    let p2 = Problem {
        vals: vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            String::from("less"),
            String::from("speaking"),
            String::from("with"),
            String::from("me"),
        ],
        name: String::from("p2"),
        result: Ok(Some(String::from("5"))),
    };
    let p3 = Problem {
        vals: vec![
            // String::from("this is a really long line that will have to be line broken, otherwise bad things will happen?"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            String::from("less"),
            String::from("speaking"),
            String::from("with"),
            String::from("me"),
        ],
        name: String::from("p3"),
        result: Ok(None),
    };

    let problem_runner = TaskRunner { thread_count: 2 };
    problem_runner.run(
        vec![Box::from(p1), Box::from(p2), Box::from(p3)],
        &mut Console::new(),
    );
}
