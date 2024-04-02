use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use crate::task::application::task_factory::TaskFactory;
use crate::task::presentation::controller::get_task;
use crate::task::presentation::model::post_task_body::PostTaskBody;
use crate::task::presentation::model::post_task_view::PostTaskView;

#[post("/tasks", format = "json", data = "<body>")]
pub async fn handle(body: Json<PostTaskBody>, task_factory: &State<TaskFactory>) -> Created<Json<PostTaskView>> {
    let task = task_factory.create(
        body.id.unwrap_or(Uuid::new_v4()),
        body.0.title,
    );

    let location = uri!(_, get_task::handle(task.id)).to_string();

    Created::new(location)
        .body(Json(PostTaskView::from(task)))
}