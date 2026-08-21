use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestModelPath {
    pub platform: String,
    pub model_id: String
}
