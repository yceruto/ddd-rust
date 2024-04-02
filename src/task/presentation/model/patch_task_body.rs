use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PatchTaskBody {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
