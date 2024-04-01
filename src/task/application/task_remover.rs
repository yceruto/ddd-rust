use std::sync::Arc;
use uuid::Uuid;
use crate::task::application::task_finder::TaskFinder;
use crate::task::domain::error::task_not_found::TaskNotFound;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskRemover {
    finder: Arc<TaskFinder>,
    repository: Arc<dyn TaskRepository>,
}

impl TaskRemover {
    pub fn new(finder: Arc<TaskFinder>, repository: Arc<dyn TaskRepository>) -> Self {
        Self { finder, repository }
    }

    pub fn remove(&self, id: Uuid) -> Result<(), TaskNotFound> {
        let task = self.finder.find_one(id)?;
        self.repository.remove(task);

        Ok(())
    }
}