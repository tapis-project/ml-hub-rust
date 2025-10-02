use serde::{Serialize, Deserialize};
use super::Accelerator;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareRequirements {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accelerators: Option<Vec<Accelerator>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_gb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i64>,
}
impl std::fmt::Display for HardwareRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
