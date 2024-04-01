use rocket::response::status::{NoContent, NotFound};
use rocket::State;
use uuid::Uuid;
use crate::task::application::task_remover::TaskRemover;

#[delete("/tasks/<id>", format = "json")]
pub async fn handle(id: Uuid, task_remover: &State<TaskRemover>) -> Result<NoContent, NotFound<String>> {
    match task_remover.remove(id) {
        Ok(_) => Ok(NoContent),
        Err(_) => return Err(NotFound("Task not found".to_string())),
    }
}