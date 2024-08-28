use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PracticeSession {
    pub id: String,
    pub date: DateTime<Utc>,
    pub duration: u32,
    pub notes: Option<String>,
}

impl PracticeSession {
    pub fn new(duration: u32, notes: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            date: Utc::now(),
            duration,
            notes,
        }
    }
}
