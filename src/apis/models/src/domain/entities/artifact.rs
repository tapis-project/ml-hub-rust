use std::path::PathBuf;

use uuid::Uuid;
use crate::domain::entities::TimeStamp;

pub struct Artifact {
    pub id: Uuid,
    pub path: Option<PathBuf>, 
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

impl Artifact {
    pub fn new() -> Self {
        let now = TimeStamp::now();
        Self {
            id: Uuid::new_v4(),
            path: None,
            created_at: now.clone(),
            last_modified: now.clone()
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
        
        // Update last modified
        self.touch();
    }

    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) {
        self.last_modified = TimeStamp::now()
    }
}

