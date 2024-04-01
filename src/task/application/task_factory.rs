use std::sync::Arc;
use uuid::Uuid;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskFactory {
    repository: Arc<dyn TaskRepository>
}

impl TaskFactory {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn create(&self, title: String) -> Task {
        let task = Task::new(Uuid::new_v4(), title);

        self.repository.add(task.clone());

        task
    }
}