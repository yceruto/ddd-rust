use std::sync::Arc;
use rocket::serde::uuid::Uuid;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskFactory {
    repository: Arc<dyn TaskRepository>
}

impl TaskFactory {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn create(&self, id: Uuid, title: String) -> Task {
        let task = Task::new(id, title);

        self.repository.add(task.clone());

        task
    }
}