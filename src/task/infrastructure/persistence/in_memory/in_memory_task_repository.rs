use std::sync::Mutex;
use rocket::serde::uuid::Uuid;
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

    fn remove(&self, task: Task) {
        self.tasks.lock().unwrap().retain(|t| t.id != task.id);
    }

    fn of_id(&self, id: Uuid) -> Option<Task> {
        self.tasks.lock().unwrap().iter()
            .find(|task| task.id == id)
            .cloned()
    }

    fn all(&self) -> Vec<Task> {
        self.tasks.lock().unwrap().clone()
    }
}