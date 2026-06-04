use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelIo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<i64>>,
}
impl std::fmt::Display for ModelIo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
