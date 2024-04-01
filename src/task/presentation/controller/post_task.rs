use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;
use rocket::State;
use crate::task::application::task_factory::TaskFactory;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostTaskBody {
    title: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostTaskView {
    id: Uuid,
    title: String,
    completed: bool,
}

#[post("/tasks", format = "json", data = "<body>")]
pub async fn handle(body: Json<PostTaskBody>, task_factory: &State<TaskFactory>) -> Json<PostTaskView> {
    let task = task_factory.create(body.title.clone());

    Json(PostTaskView {
        id: task.id,
        title: task.title,
        completed: task.completed,
    })
}