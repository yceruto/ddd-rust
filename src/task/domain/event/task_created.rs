use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskCreated {
    pub id: String,
    pub created_at: String,
}

impl TaskCreated {
    pub fn new(id: Uuid) -> Self {
        Self {
            id: id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}