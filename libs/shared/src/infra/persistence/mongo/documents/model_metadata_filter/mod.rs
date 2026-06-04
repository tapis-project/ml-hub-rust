mod input_to_document;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use mongodb::bson::Document;

use crate::infra::persistence::mongo::documents::task::Task;
use crate::infra::persistence::mongo::utils::is_vec_empty;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accelerator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    /// Firmware and software
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub system_requirements: Option<Vec<SystemRequirement>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_gb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_gb: Option<i32>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub accelerators: Option<Vec<Accelerator>>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub architectures: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelIO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub shape: Option<Vec<i32>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelMetadataFilter {
    // General fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    // Tenancy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Document>,

    /// Arbitrary labels
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,

    /// Architecture fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_modal: Option<bool>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub model_inputs: Option<Vec<ModelIO>>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub task_types: Option<Vec<Task>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_hardware: Option<HardwareRequirements>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub inference_software_dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_max_energy_consumption_watts: Option<i32>,

    /// Inference performance fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_max_latency_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_min_throughput: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_max_compute_utilization_percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_max_memory_usage_mb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_distributed: Option<bool>,

    /// Training-related Fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_hardware: Option<HardwareRequirements>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub pretraining_datasets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub finetuning_datasets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_optimized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization_aware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_quantization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretrained: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slimmed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_distributed: Option<bool>,

    /// Training performance fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_max_energy_consumption_watts: Option<i32>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub regulatory: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bias_evaluation_score: Option<i8>,
}