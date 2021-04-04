use super::common::*;
use std::convert::TryFrom;
use termion::{clear, color, color::Color, cursor, style};

const MAX_LINES_PER_LOG: usize = 5;
const PENDING_TEXT: StatusText = StatusText {
    color: &color::Blue,
    characters: "Pending",
};
const RUNNING_TEXT: StatusText = StatusText {
    color: &color::Cyan,
    characters: "Running",
};
const FINISHED_TEXT: StatusText = StatusText {
    color: &color::Green,
    characters: "Finished",
};

pub struct Console {
    logs: Vec<TaskLog>,
}

struct TaskLog {
    name: TaskName,
    status: Status,
    messages: Vec<LogMessage>,
}

impl TaskLog {
    fn new(name: TaskName) -> TaskLog {
        TaskLog {
            name,
            status: Status::Pending,
            messages: Vec::new(),
        }
    }

    fn print(&self) {
        let status_text = format(&self.status);
        println!(
            "{}{}{}{} {}",
            style::Bold,
            color::Fg(status_text.color),
            status_text.characters,
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

    fn add_message(&mut self, message: LogMessage) {
        self.messages.push(message);
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

fn format(status: &Status) -> StatusText {
    match *status {
        Status::Pending => PENDING_TEXT,
        Status::Running => RUNNING_TEXT,
        Status::Finished => FINISHED_TEXT,
    }
}

struct StatusText {
    color: &'static dyn Color,
    characters: &'static str,
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

impl View for Console {
    fn initialize(&mut self, tasks: Vec<TaskName>) {
        self.logs = tasks
            .into_iter()
            .map(|task_name| TaskLog::new(task_name))
            .collect();
        print_logs(&self.logs);
    }

    fn update(&mut self, task_update: TaskUpdate) {
        clear_lines(get_nbr_of_visible_lines(&self.logs));
        let log = get_matching_log(task_update.task_name, &mut self.logs);
        match task_update.change {
            TaskChange::TaskMessage(message) => log.add_message(message),
            TaskChange::TaskStatus(status) => log.set_status(status),
        }
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

fn get_matching_log(task_name: TaskName, logs: &mut Vec<TaskLog>) -> &mut TaskLog {
    logs.iter_mut().find(|log| log.name == task_name).unwrap()
}

fn print_logs(logs: &Vec<TaskLog>) {
    logs.iter().for_each(|log| log.print());
}
