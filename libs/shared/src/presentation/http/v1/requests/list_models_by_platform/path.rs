use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListModelsByPlatformPath {
    pub platform: String
}