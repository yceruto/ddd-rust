use std::sync::Mutex;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct InMemoryTaskRepository {
    tasks: Mutex<Vec<Task>>
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self { tasks: Mutex::new(Vec::new()) }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn add(&self, task: Task) {
        self.tasks.lock().unwrap().push(task);
    }

    fn all(&self) -> Vec<Task> {
        self.tasks.lock().unwrap().clone()
    }
}