use rocket::serde::json::Json;
use rocket::State;
use crate::task::application::task_finder::TaskFinder;
use crate::task::presentation::view::task_view::TaskView;

#[get("/tasks", format = "json")]
pub async fn handle(task_finder: &State<TaskFinder>) -> Json<Vec<TaskView>> {
    let tasks = task_finder.find_all();

    Json(TaskView::from_many(tasks))
}