use serde::{Serialize, Deserialize};
use super::SystemRequirement;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Accelerator {
    pub accelerator_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i64>,
    ///Firmware and software
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub system_requirements: Vec<SystemRequirement>,
}
impl std::fmt::Display for Accelerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
