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
const FAILED_TEXT: StatusText = StatusText {
    color: &color::Red,
    characters: "Failed",
};

pub struct Console {
    logs: Vec<TaskLog>,
}

struct TaskLog {
    name: TaskName,
    status: Status,
    lines: String,
}

impl TaskLog {
    fn new(name: TaskName) -> TaskLog {
        TaskLog {
            name,
            status: Status::Pending,
            lines: String::new(),
        }
    }

    fn print(&self) {
        println!("{}", format_status(&self.status, &self.name));
        print_messages(&self.status, &self.lines);
    }

    fn nbr_of_visible_lines(&self) -> usize {
        match &self.status {
            Status::Finished(_) => 1,
            Status::Failed(error) => 1 + get_lines(error) + get_lines(&self.lines),
            _ => floor(1 + get_lines(&self.lines), MAX_LINES_PER_LOG),
        }
    }

    fn add_message(&mut self, message: LogMessage) {
        self.lines.push_str(message.as_str());
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

fn get_lines(lines: &str) -> usize {
    lines.split_terminator('\n').count()
}

fn format_status(status: &Status, task_name: &TaskName) -> String {
    match status {
        Status::Pending => format_status_line(PENDING_TEXT, task_name),
        Status::Running => format_status_line(RUNNING_TEXT, task_name),
        Status::Finished(result) => match result {
            Some(answer) => {
                format_detailed_status_line(FINISHED_TEXT, task_name, format!(": {}", answer))
            }
            None => format_status_line(FINISHED_TEXT, task_name),
        },
        Status::Failed(_) => format_status_line(FAILED_TEXT, task_name),
    }
}

fn format_status_line(status_text: StatusText, task_name: &TaskName) -> String {
    format_detailed_status_line(status_text, task_name, String::from(""))
}

fn format_detailed_status_line(
    status_text: StatusText,
    task_name: &TaskName,
    details: String,
) -> String {
    format!(
        "{}{}{}{} {}{}",
        style::Bold,
        color::Fg(status_text.color),
        status_text.characters,
        style::Reset,
        task_name,
        details
    )
}

struct StatusText {
    color: &'static dyn Color,
    characters: &'static str,
}

fn print_messages(status: &Status, messages: &str) {
    match status {
        Status::Finished(_) => (),
        Status::Failed(error) => {
            for message in messages.split_terminator('\n') {
                println!("  {}", message);
            }
            println!("  {}", error);
        }
        _ => {
            if messages.is_empty() {
                return;
            }
            let most_recent_messages: Vec<&str> = messages
                .rsplit_terminator('\n')
                .take(MAX_LINES_PER_LOG - 1)
                .collect();
            for message in most_recent_messages.iter().rev() {
                println!("  {}", message);
            }
        }
    }
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
