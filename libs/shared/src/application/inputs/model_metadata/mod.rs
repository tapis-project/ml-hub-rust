pub mod inputs_to_entities;
pub mod input_to_input;
pub mod entities_to_input;

use crate::application::inputs::task::Task;

use platforms::Platform;
use serde_json::Value;
use uuid::Uuid;

use crate::application::inputs::common::Scope;

#[derive(Debug, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Debug, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Debug, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Debug, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Debug, Clone)]
pub struct Canonical {
    pub platform: Platform,
    pub model_id: String,
    pub locator: Locator,
    pub author: Option<String>,
    pub likes: Option<u128>,
    pub downloads: Option<u128>,
    pub gated: Option<bool>,
    pub private: Option<bool>,
    pub sha: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Locator {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ModelMetadata {
    // General fields
    pub name: String,
    pub author: String,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub image: Option<String>,
    pub canonical: Option<Canonical>,

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

#[derive(Debug, Clone)]
pub struct AssociateModelMetadata {
    pub artifact_id: Uuid,
    pub name: String,
    pub author: String,
}

#[derive(Debug, Clone)]
pub struct UpdateModelMetadataArtifactId {
    pub artifact_id: Uuid,
    pub name: String,
    pub author: String,
}

#[derive(Debug, Clone)]
pub struct UpsertModelMetadata {
    pub metadata: ModelMetadata,
}

#[derive(Debug, Clone)]
pub struct GetModelMetadataByAuthorAndNameInput {
    pub author: String,
    pub name: String,
    pub tenant_id: String,
    pub principal_id: String,
    pub scope: Scope
}

#[derive(Debug, Clone)]
pub struct ListModelMetadataByAuthorInput {
    pub author: String,
    pub tenant_id: String,
    pub principal_id: String,
}

