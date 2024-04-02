use rocket::serde::Serialize;
use rocket::serde::uuid::Uuid;
use crate::task::domain::model::task::Task;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskView {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: String,
    updated_at: String,
}

impl TaskView {
    pub fn from_many(tasks: Vec<Task>) -> Vec<TaskView> {
        tasks.into_iter().map(Self::from).collect()
    }

    pub fn from(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            completed: task.completed,
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.map(|date| date.to_rfc3339()).unwrap_or_default(),
        }
    }
}