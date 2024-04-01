#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

pub mod task;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": 404,
        "title": "Not found",
        "detail": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(task::presentation::controller::stage())
        .register("/", catchers![not_found])
}