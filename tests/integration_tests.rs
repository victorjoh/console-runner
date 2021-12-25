use console_runner::{common::*, tasks::*};

#[test]
pub fn the_result_of_a_task_is_passed_to_the_view() {
    let problem_runner = TaskRunner {
        thread_count: 1,
        view_update_period: 0,
    };
    let mut view = StoreToMemory::new();

    problem_runner.run(vec![Box::from(SimpleTask {})], &mut view);

    assert_eq!(
        view.task_updates,
        vec![
            new_status("SimpleTask", Status::Running),
            new_status("SimpleTask", Status::Finished(Some(String::from("5")))),
        ]
    );
}

fn new_status(name: &str, status: Status) -> TaskUpdate {
    TaskUpdate {
        task_name: String::from(name),
        change: TaskChange::TaskStatus(status),
    }
}

struct SimpleTask {}

impl Task for SimpleTask {
    fn run(&self, _: &dyn Logger) -> TaskResult {
        Ok(Some(String::from("5")))
    }

    fn name(&self) -> TaskName {
        String::from("SimpleTask")
    }
}

struct StoreToMemory {
    tasks: Vec<TaskName>,
    task_updates: Vec<TaskUpdate>,
}

impl StoreToMemory {
    pub fn new() -> StoreToMemory {
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
