use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::uuid::Uuid;
use crate::task::application::task_finder::TaskFinder;
use crate::task::presentation::model::task_view::TaskView;

#[get("/tasks/<id>", format = "json")]
pub async fn handle(id: Uuid, task_finder: &State<TaskFinder>) -> Json<TaskView> {
    let task = task_finder.find_one(id);

    Json(TaskView::from(task.unwrap()))
}