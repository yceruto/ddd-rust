use std::sync::Arc;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::uuid::Uuid;
use crate::task::application::task_finder::TaskFinder;
use crate::task::presentation::model::task_view::TaskView;

#[get("/tasks/<id>", format = "json")]
pub async fn handle(id: Uuid, task_finder: &State<Arc<TaskFinder>>) -> Result<Json<TaskView>, NotFound<String>> {
    match task_finder.find_one(id) {
        Ok(task) => Ok(Json(TaskView::from(task))),
        Err(_) => Err(NotFound("Task not found".to_string())),
    }
}