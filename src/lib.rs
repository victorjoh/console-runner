mod common;
mod tasks;
mod view;

use common::*;
use std::thread;
use view::*;

use std::time::Duration;

struct Problem {
    vals: Vec<String>,
    name: TaskName,
}

impl Task<String> for Problem {
    fn perform(&self, logger: &dyn Logger<String>) {
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
        ],
        name: String::from("p1"),
    };
    let p2 = Problem {
        vals: vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ],
        name: String::from("p2"),
    };

    tasks::perform(vec![Box::from(p1), Box::from(p2)], &mut Console::new());
}

fn ceiling(dividend: usize, divisor: usize) -> usize {
    (dividend + divisor - 1) / divisor
}
