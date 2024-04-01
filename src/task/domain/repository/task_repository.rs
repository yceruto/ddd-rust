use rocket::serde::uuid::Uuid;
use crate::task::domain::model::task::Task;

pub trait TaskRepository: Send + Sync {
    fn add(&self, task: Task);
    fn remove(&self, task: Task);
    fn of_id(&self, id: Uuid) -> Option<Task>;
    fn all(&self) -> Vec<Task>;
}