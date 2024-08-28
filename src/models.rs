use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PracticeSession {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Utc::now")]
    pub date: DateTime<Utc>,
    pub duration: u32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetMusic {
    pub name: String,
    pub content: String,
}
