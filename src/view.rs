use crate::common::*;
use std::{thread, time};
use termion;
use termion::{clear, color, cursor, style};

pub struct Console {
    tasks: Vec<TaskName>,
}

fn print_pending(task_name: &TaskName) {
    println!(
        "{}{}Pending{} {}",
        color::Fg(color::Blue),
        style::Bold,
        style::Reset,
        task_name
    );
}

impl Console {
    pub fn new() -> Console {
        Console { tasks: Vec::new() }
    }
}

impl View<String> for Console {
    fn initialize(&mut self, tasks: Vec<TaskName>) {
        self.tasks = tasks;
        self.tasks
            .iter()
            .for_each(|task_name| print_pending(task_name));
    }

    fn show(&mut self, task_message: TaskMessage<String>) {
        println!("{}: {}", task_message.task_name, task_message.message);
    }
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
