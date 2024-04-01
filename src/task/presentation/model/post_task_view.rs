use rocket::serde::Serialize;
use rocket::serde::uuid::Uuid;
use crate::task::domain::model::task::Task;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostTaskView {
    id: Uuid,
    title: String,
    completed: bool,
}

impl PostTaskView {
    pub fn from(task: Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            completed: task.completed,
        }
    }
}
