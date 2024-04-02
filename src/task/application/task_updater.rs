use std::sync::Arc;
use uuid::Uuid;
use crate::task::application::task_finder::TaskFinder;
use crate::task::domain::error::task_not_found::TaskNotFound;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskUpdater {
    finder: Arc<TaskFinder>,
    repository: Arc<dyn TaskRepository>
}

impl TaskUpdater {
    pub fn new(finder: Arc<TaskFinder>, repository: Arc<dyn TaskRepository>) -> Self {
        Self { finder, repository }
    }

    pub fn update(&self, id: Uuid, title: Option<String>, completed: Option<bool>) -> Result<Task, TaskNotFound> {
        let task = self.finder.find_one(id)?;
        let task = task.update(
            title.unwrap_or(task.title.clone()),
            completed.unwrap_or(task.completed.clone()),
        );

        self.repository.update(task.clone());

        Ok(task)
    }
}