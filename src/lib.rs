#![feature(internal_output_capture)]
pub mod common;
pub mod tasks;
pub mod view;
use common::*;
use std::thread;
use tasks::{Logger, Task, TaskResult, TaskRunner};
use view::*;
use rand::Rng;

use std::time::Duration;

struct Problem<'a> {
    vals: Vec<&'a str>,
    name: TaskName,
    result: TaskResult,
}

impl<'a> Task for Problem<'a> {
    fn run(&self, _: &dyn Logger) -> TaskResult {
        for val in &self.vals {
            //logger.log(val.to_string());
            println!("{}", val);
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(500..1500)));
        }
        return self.result.clone();
    }

    fn name(&self) -> TaskName {
        self.name.clone()
    }
}

struct PanicProblem {
    vals: Vec<String>,
    name: TaskName,
}

impl Task for PanicProblem {
    fn run(&self, logger: &dyn Logger) -> TaskResult {
        for val in &self.vals {
            logger.log(val);
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(500..1500)));
        }
        panic!("I don't know what to do!");
    }

    fn name(&self) -> TaskName {
        self.name.clone()
    }
}

pub fn run() {
    let p1 = PanicProblem {
        vals: vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("goodbye"),
            String::from("to"),
        ],
        name: String::from("panic"),
    };
    let p2 = Problem {
        vals: vec![
            "more", "messages", "for", "you", "less", "speaking", "with", "me",
        ],
        name: String::from("doing stuff"),
        result: Err(String::from("Something went wrong!")),
    };
    let p3 = Problem {
        vals: vec![
            // "this is a really long line that will have to be line broken, otherwise bad things will happen?",
            "messages", "for", "you", "less", "speaking", "with", "me",
        ],
        name: String::from("hello?"),
        result: Ok(Some(String::from("42"))),
    };
    let p4 = Problem {
        vals: vec![
            // "this is a really long line that will have to be line broken, otherwise bad things will happen?",
            "messages", "for", "you", "less", "speaking", "with", "me",
        ],
        name: String::from("convert format"),
        result: Ok(None),
    };
    let p01 = Problem {
        vals: vec![
            "messages", "for", "you", "less", "speaking", "with", "me", "messages", "for", "you",
            "less", "speaking", "with", "me", "messages", "for", "you", "less", "speaking", "with",
            "me",
        ],
        name: String::from("Copy file install.iso"),
        result: Ok(Some(String::from("File copied"))),
    };
    let p02 = Problem {
        vals: vec!["messages", "for", "you", "less", "speaking", "with", "me"],
        name: String::from("download happy-cow.pdf"),
        result: Ok(Some(String::from("Download complete"))),
    };
    let p03 = Problem {
        vals: vec!["messages"],
        name: String::from("quick"),
        result: Ok(Some(String::from("Yes, cookies are tasty."))),
    };
    let p04 = Problem {
        vals: vec!["messages"],
        name: String::from("get distance"),
        result: Ok(Some(String::from("532 m"))),
    };
    let p05 = Problem {
        vals: vec!["messages", "for", "you", "less", "speaking", "with", "me"],
        name: String::from("import video"),
        result: Ok(Some(String::from("Import successful."))),
    };

    let problem_runner = TaskRunner {
        thread_count: 2,
        view_update_period: 100,
    };
    problem_runner.run(
        vec![
            Box::from(p01),
            Box::from(p1),
            Box::from(p02),
            Box::from(p03),
            Box::from(p04),
            Box::from(p05),
            Box::from(p2),
            Box::from(p3),
            Box::from(p4),
        ],
        &mut Console::new(),
    );
}

// struct DebugConsole {}

// impl DebugConsole {
//     pub fn new() -> DebugConsole {
//         DebugConsole {}
//     }
// }

// impl View for DebugConsole {
//     fn initialize(&mut self, _: Vec<TaskName>) {}

//     fn update(&mut self, task_update: TaskUpdate) {
//         println!("{:?}", task_update);
//     }
// }
