use rocket::serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostTaskBody {
    pub id: Option<Uuid>,
    pub title: String,
}
