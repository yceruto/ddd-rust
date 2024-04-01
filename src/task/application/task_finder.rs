use std::sync::Arc;
use rocket::serde::uuid::Uuid;
use crate::task::domain::error::task_not_found::TaskNotFound;
use crate::task::domain::model::task::Task;
use crate::task::domain::repository::task_repository::TaskRepository;

pub struct TaskFinder {
    repository: Arc<dyn TaskRepository>
}

impl TaskFinder {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        Self { repository }
    }

    pub fn find_one(&self, id: Uuid) -> Result<Task, TaskNotFound> {
        match self.repository.of_id(id) {
            Some(task) => Ok(task),
            None => Err(TaskNotFound::Error(format!("Task with id {} not found", id))),
        }
    }

    pub fn find_all(&self) -> Vec<Task> {
        self.repository.all()
    }
}