mod console_runner;

use console_runner::common::*;
use std::thread;
use console_runner::tasks::{Logger, Task, TaskRunner};
use console_runner::view::*;

use std::time::Duration;

struct Problem {
    vals: Vec<String>,
    name: TaskName,
}

impl Task for Problem {
    fn run(&self, logger: &dyn Logger) {
        for val in &self.vals {
            logger.log(val.to_string());
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn name(&self) -> TaskName {
        self.name.clone()
    }
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
            String::from("all"),
            String::from("needles"),
        ],
        name: String::from("p1"),
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
    };

    let problem_runner = TaskRunner { thread_count: 2 };
    problem_runner.run(
        vec![Box::from(p1), Box::from(p2), Box::from(p3)],
        &mut Console::new(),
    );
}
