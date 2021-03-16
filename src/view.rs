use crate::common::*;
use std::convert::TryFrom;
use std::{thread, time};
use termion;
use termion::{clear, color, cursor, style};

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
        for message in &self.messages {
            println!("  {}", message);
        }
    }

    fn nbr_of_lines(&self) -> usize {
        1 + self.messages.len()
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
        print_all_logs(&self.logs);
    }

    fn show(&mut self, task_message: TaskMessage<String>) {
        let log = self
            .logs
            .iter_mut()
            .find(|log| log.name == task_message.task_name)
            .unwrap();
        log.messages.push(task_message.message);

        clear_lines(get_nbr_of_lines(&self.logs) - 1);
        print_all_logs(&self.logs);
    }
}

fn get_nbr_of_lines(logs: &Vec<TaskLog>) -> usize {
    logs.iter().map(|log| log.nbr_of_lines()).sum()
}

fn clear_lines(nbr_of_lines: usize) {
    print!(
        "{}{}",
        cursor::Up(u16::try_from(nbr_of_lines).unwrap()),
        clear::AfterCursor
    );
}

fn print_all_logs(logs: &Vec<TaskLog>) {
    logs.iter().for_each(|log| log.print());
}

fn example() {
    let lines = [
        "Starting",
        "on going",
        "what are we doing to ourselves?",
        "work work",
        "working hard or hardly working?",
        "are we there yet",
        "yeti",
        "something is happening",
        // "this is a very long line that should require wrapping. I am sure that it will since I am adding a lot of text to it. Now I am really making sure",
        "where were you?",
        "done",
    ];

    println!(
        "{}{}Solving{} 2020 day 1",
        color::Fg(color::Blue),
        style::Bold,
        style::Reset
    );

    for line in &lines[..4] {
        println!("  {}", line);
        thread::sleep(time::Duration::from_millis(500));
    }
    for (index, _) in lines[4..].iter().enumerate() {
        print!("{}{}", cursor::Up(4), clear::AfterCursor);
        for prev in &lines[index + 1..index + 5] {
            println!("  {}", prev);
        }
        thread::sleep(time::Duration::from_millis(500));
    }

    print!("{}{}", cursor::Up(5), clear::AfterCursor);
    println!(
        "{}{}Solved{} 2020 day 1",
        color::Fg(color::Green),
        style::Bold,
        style::Reset
    );
    println!("  Part 1:{} {}{}", style::Italic, 5, style::Reset);
    println!("  Part 2:{} {}{}", style::Italic, 1235, style::Reset);
}
