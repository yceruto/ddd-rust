use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: Uuid, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
        }
    }
}