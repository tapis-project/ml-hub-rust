use serde::{Serialize, Deserialize};
use super::{HardwareRequirements, ModelIo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bias_evaluation_score: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_optimized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finetuning_datasets: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_distributed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_hardware: Option<HardwareRequirements>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_max_compute_utilization_percentage: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_max_energy_consumption_watts: Option<i64>,
    ///Inference performance fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_max_latency_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_max_memory_usage_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_min_throughput: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_precision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_software_dependencies: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_map: Option<serde_json::Value>,
    ///Arbitrary labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_inputs: Option<Vec<ModelIo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_outputs: Option<Vec<ModelIo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    ///Architecture fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_modal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pretrained: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pretraining_datasets: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pruned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantization_aware: Option<bool>,
    /**Regulatory and Compliance Fields
A vector or strings that represent regulatory standards. Ex HIPPA*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regulatory: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slimmed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_quantization: Option<bool>,
    ///Inference Fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_distributed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_hardware: Option<HardwareRequirements>,
    ///Training performance fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_max_energy_consumption_watts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_precision: Option<String>,
    ///Training-related Fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ModelMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
