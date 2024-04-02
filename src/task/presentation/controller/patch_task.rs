use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use crate::task::application::task_updater::TaskUpdater;
use crate::task::presentation::model::patch_task_body::PatchTaskBody;
use crate::task::presentation::model::task_view::TaskView;

#[patch("/tasks/<id>", format = "json", data = "<body>")]
pub async fn handle(id: Uuid, body: Json<PatchTaskBody>, task_updater: &State<TaskUpdater>) -> Result<Json<TaskView>, NotFound<String>> {
    let result = task_updater.update(
        id,
        body.0.title,
        body.0.completed,
    );

    match result {
        Ok(task) => Ok(Json(TaskView::from(task))),
        Err(_) => return Err(NotFound("Task not found".to_string())),
    }
}