use console_runner::{common::*, tasks::*};
use spectral::prelude::*;

const TASK_RUNNER: TaskRunner = TaskRunner {
    thread_count: 1,
    view_update_period: 0,
};

#[test]
fn the_result_of_a_task_is_passed_to_the_view() {
    let mut view = StoreToMemory::new();
    let task = SimpleTask {
        name: "my name",
        run_task: || Ok(Some(String::from("5"))),
    };

    TASK_RUNNER.run(vec![Box::from(task)], &mut view);

    assert_that(&view.task_updates).is_equal_to(vec![
        a_status("my name", Status::Running),
        a_status("my name", Status::Finished(Some(String::from("5")))),
    ]);
}

#[test]
fn when_a_task_prints_something_it_is_passed_to_the_view() {
    let mut view = StoreToMemory::new();
    let task = SimpleTask {
        name: "my name",
        run_task: || {
            print!("Hello!");
            Ok(None)
        },
    };

    TASK_RUNNER.run(vec![Box::from(task)], &mut view);

    assert_that(&view.task_updates).is_equal_to(vec![
        a_status("my name", Status::Running),
        a_message("my name", "Hello!"),
        a_status("my name", Status::Finished(None)),
    ]);
}

#[test]
fn when_a_task_panics_it_is_passed_to_the_view() {
    let mut view = StoreToMemory::new();
    let task = SimpleTask {
        name: "my name",
        run_task: || panic!("Aargh!"),
    };

    TASK_RUNNER.run(vec![Box::from(task)], &mut view);

    assert_that(&view.task_updates).has_length(3);
    let panic_message = extract_message(&view.task_updates[1]);
    assert_that(&panic_message).starts_with("thread '<unnamed>' panicked at 'Aargh!'");
    assert_that(&view.task_updates[2]).is_equal_to(a_status(
        "my name",
        Status::Failed(String::from("Aborting task since thread panicked")),
    ))
}

fn extract_message(update: &TaskUpdate) -> &str {
    match &update.change {
        TaskChange::TaskMessage(message) => return &message,
        _ => panic!("The update was not a log message: <{:?}>", update),
    }
}

#[test]
fn many_tasks_are_run_in_order() {
    let mut view = StoreToMemory::new();
    let first_task = SimpleTask {
        name: "first task",
        run_task: || Err(String::from("failure")),
    };
    let second_task = SimpleTask {
        name: "second task",
        run_task: || Ok(None),
    };

    TASK_RUNNER.run(
        vec![Box::from(first_task), Box::from(second_task)],
        &mut view,
    );

    assert_that(&view.task_updates).is_equal_to(vec![
        a_status("first task", Status::Running),
        a_status("first task", Status::Failed(String::from("failure"))),
        a_status("second task", Status::Running),
        a_status("second task", Status::Finished(None)),
    ]);
}

fn a_status(name: &str, status: Status) -> TaskUpdate {
    TaskUpdate {
        task_name: String::from(name),
        change: TaskChange::TaskStatus(status),
    }
}

fn a_message(name: &str, message: &str) -> TaskUpdate {
    TaskUpdate {
        task_name: String::from(name),
        change: TaskChange::TaskMessage(String::from(message)),
    }
}

struct SimpleTask<'a> {
    name: &'a str,
    run_task: fn() -> TaskResult,
}

impl<'a> Task for SimpleTask<'a> {
    fn run(&self, _: &dyn Logger) -> TaskResult {
        let run_task = self.run_task;
        run_task()
    }

    fn name(&self) -> TaskName {
        String::from(self.name)
    }
}

enum ViewMethod {
    Initialize,
    Update,
}

struct StoreToMemory {
    tasks: Vec<TaskName>,
    task_updates: Vec<TaskUpdate>,
}

impl StoreToMemory {
    fn new() -> StoreToMemory {
        StoreToMemory {
            tasks: Vec::new(),
            task_updates: Vec::new(),
        }
    }
}

impl View for StoreToMemory {
    fn initialize(&mut self, tasks: Vec<TaskName>) {
        self.tasks = tasks;
    }

    fn update(&mut self, task_update: TaskUpdate) {
        self.task_updates.push(task_update);
    }
}
