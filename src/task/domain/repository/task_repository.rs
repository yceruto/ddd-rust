use crate::task::domain::model::task::Task;

pub trait TaskRepository: Send + Sync {
    fn add(&self, task: Task);
    fn all(&self) -> Vec<Task>;
}