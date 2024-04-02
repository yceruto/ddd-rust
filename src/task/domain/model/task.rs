use chrono::{DateTime, Utc};
use rocket::serde::uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: Uuid, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(&self, title: String, completed: bool) -> Self {
        Self {
            id: self.id,
            title,
            completed,
            created_at: self.created_at,
            updated_at: Some(Utc::now()),
        }
    }
}