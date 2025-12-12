pub mod to_input;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use serde_json::Value;
use crate::presentation::http::v1::requests::headers::Headers;
use crate::presentation::http::v1::requests::task::Task;

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscoverModelsByPlatformPath {
    pub platform: String
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct DiscoveryCriteria {
    // Used for model discovery clients support model discovery through natural
    // language search
    pub prompt: Option<String>,
    pub criteria: Vec<DiscoveryCriterion>,
    pub confidence_threshold: Option<u8>,
}

pub struct DiscoverModelsRequest {
    pub headers: Headers,
    pub query: HashMap<String, String>,
    pub body: DiscoveryCriteria
}

pub struct DiscoverModelsByPlatformRequest {
    pub headers: Headers,
    pub path: DiscoverModelsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: DiscoveryCriteria
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct SystemRequirement {
    pub name: Option<String>,
    pub version: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct Accelerator {
    pub accelerator_type: Option<String>,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Option<Vec<SystemRequirement>>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct DiscoveryCriterion {
    // General fields
    pub name: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub keywords: Option<Vec<String>>,
    pub annotations: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<Task>>,
    pub inference_precision: Option<String>,
    pub inference_hardware: Option<HardwareRequirements>,
    pub inference_software_dependencies: Option<Vec<String>>,
    pub inference_max_energy_consumption_watts: Option<i32>,

    /// Inference performance fields
    pub inference_max_latency_ms: Option<i32>,
    pub inference_min_throughput: Option<i32>,
    pub inference_max_compute_utilization_percentage: Option<i32>,
    pub inference_max_memory_usage_mb: Option<i32>,
    pub inference_distributed: Option<bool>,

    /// Training-related Fields
    pub training_time: Option<i64>,
    pub training_precision: Option<String>,
    pub training_hardware: Option<HardwareRequirements>,
    pub pretraining_datasets: Option<Vec<String>>,
    pub finetuning_datasets: Option<Vec<String>>,
    pub edge_optimized: Option<bool>,
    pub quantization_aware: Option<bool>,
    pub supports_quantization: Option<bool>,
    pub pretrained: Option<bool>,
    pub pruned: Option<bool>,
    pub slimmed: Option<bool>,
    pub training_distributed: Option<bool>,

    /// Training performance fields
    pub training_max_energy_consumption_watts: Option<i32>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub bias_evaluation_score: Option<i8>,
}