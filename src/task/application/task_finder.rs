use std::sync::Arc;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskFinder {
    repository: Arc<dyn TaskRepository>
}

impl TaskFinder {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn find_all(&self) -> Vec<Task> {
        self.repository.all()
    }
}