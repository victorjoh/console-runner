use crate::common::*;
use std::convert::TryFrom;
use termion::{clear, color, cursor, style};

const MAX_LINES_PER_LOG: usize = 5;

pub struct Console {
    logs: Vec<TaskLog>,
}

struct TaskLog {
    name: TaskName,
    messages: Vec<String>,
}

impl TaskLog {
    fn new(name: TaskName) -> TaskLog {
        TaskLog {
            name,
            messages: Vec::new(),
        }
    }

    fn print(&self) {
        println!(
            "{}{}Pending{} {}",
            color::Fg(color::Blue),
            style::Bold,
            style::Reset,
            self.name
        );
        for message in get_last_n(&self.messages, MAX_LINES_PER_LOG - 1) {
            println!("  {}", message);
        }
    }

    fn nbr_of_visible_lines(&self) -> usize {
        floor(1 + self.messages.len(), MAX_LINES_PER_LOG)
    }

    fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }
}

fn get_last_n<'a, T>(vector: &'a Vec<T>, n: usize) -> &'a [T] {
    let start_index = vector.len().saturating_sub(n);
    let (_, last_4) = vector.split_at(start_index);
    return last_4;
}

fn floor(x: usize, y: usize) -> usize {
    if x < y {
        x
    } else {
        y
    }
}

impl Console {
    pub fn new() -> Console {
        Console { logs: Vec::new() }
    }
}

impl View<String> for Console {
    fn initialize(&mut self, tasks: Vec<TaskName>) {
        self.logs = tasks
            .into_iter()
            .map(|task_name| TaskLog::new(task_name))
            .collect();
        print_logs(&self.logs);
    }

    fn show(&mut self, task_message: TaskMessage<String>) {
        clear_lines(get_nbr_of_visible_lines(&self.logs));
        add_message_to_matching_log(task_message, &mut self.logs);
        print_logs(&self.logs);
    }
}

fn clear_lines(nbr_of_lines: usize) {
    print!(
        "{}{}",
        cursor::Up(u16::try_from(nbr_of_lines).unwrap()),
        clear::AfterCursor
    );
}

fn get_nbr_of_visible_lines(logs: &Vec<TaskLog>) -> usize {
    logs.iter().map(|log| log.nbr_of_visible_lines()).sum()
}

fn add_message_to_matching_log(task_message: TaskMessage<String>, logs: &mut Vec<TaskLog>) {
    let log = logs
        .iter_mut()
        .find(|log| log.name == task_message.task_name)
        .unwrap();
    log.add_message(task_message.message);
}

fn print_logs(logs: &Vec<TaskLog>) {
    logs.iter().for_each(|log| log.print());
}
