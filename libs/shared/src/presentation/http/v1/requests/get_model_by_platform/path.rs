use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetModelByPlatformPath {
    pub platform: String,
    pub model_id: String
}