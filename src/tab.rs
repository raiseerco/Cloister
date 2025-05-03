use uuid::Uuid;
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Tab {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub dirty: bool,
    pub file_path: Option<PathBuf>,
}

impl Tab {
    pub fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: String::new(),
            dirty: false,
            file_path: None,
        }
    }
}
