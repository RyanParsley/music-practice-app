use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetMusic {
    pub name: String,
    pub content: String,
}

impl SheetMusic {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}
