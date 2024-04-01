#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::{Value, json};

pub mod task;

#[catch(default)]
fn error_catcher(status: Status, _req: &Request) -> Value {
    json!({
        "status": status.code,
        "title": "Not found",
        "detail": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(task::presentation::controller::stage())
        .register("/", catchers![error_catcher])
}