use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<i64>,
    pub user_id: i64,
    pub path: String,
    pub content: Option<String>,
    pub size: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Resource {
    pub fn new(path: String, content: String) -> Self {
        let now = Utc::now();
        let size = content.len() as i64;
        
        Self {
            id: None,
            user_id: 1, // Default user_id as per requirements
            path,
            content: Some(content),
            size,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_folder_path(&self) -> String {
        if let Some(last_slash_pos) = self.path.rfind('/') {
            if last_slash_pos == 0 {
                "/".to_string()
            } else {
                self.path[..last_slash_pos].to_string()
            }
        } else {
            "/".to_string()
        }
    }
}

#[derive(Debug)]
pub struct FolderInfo {
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub resources: Vec<String>,
}
