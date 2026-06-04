pub mod entity_to_dto;
pub mod dto_to_input;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use std::collections::HashMap;
use crate::presentation::http::v1::requests::headers::Headers;
use crate::presentation::http::v1::requests::artifacts;
use crate::presentation::http::v1::requests::task::Task;
use crate::presentation::http::v1::requests::errors::PresentationError;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
pub struct ListModelsByPlatformPath {
    pub platform: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetModelByPlatformPath {
    pub platform: String,
    pub model_id: String
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct GetModelPath {
    pub name: String,
    pub author: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadModelPath {
    pub artifact_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestModelPath {
    pub platform: String,
    pub model_id: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestCanonicalModelPath {
    pub author: String,
    pub name: String,
}

pub struct ListModelsByPlatformRequest {
    pub headers: Headers,
    pub path: ListModelsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}

pub struct GetModelByPlatformRequest {
    pub headers: Headers,
    pub path: GetModelByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestModelRequest {
    pub headers: Headers,
    pub path: IngestModelPath,
    pub query: HashMap<String, String>,
    pub body: artifacts::IngestArtifactRequest,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestCanonicalModelRequest {
    pub headers: Headers,
    pub path: IngestCanonicalModelPath,
    pub query: HashMap<String, String>,
    pub body: artifacts::IngestArtifactRequest,
}

pub struct DownloadModelRequest {
    pub headers: Headers,
    pub path: DownloadModelPath,
}

pub struct UploadModelRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AssociateModelMetadataPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ModelMetadata {
    // General fields
    #[validate(required, length(min=1))]
    pub name: Option<String>,
    #[validate(required, length(min=1))]
    pub author: Option<String>,
    pub tenant_id: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AssociateModelMetadata {
    pub name: String,
    pub author: String,
}

pub struct CreateModelMetadata {
    metadata: ModelMetadata
}

impl CreateModelMetadata {
    pub fn new(metadata: ModelMetadata) -> Result<Self, PresentationError> {
        metadata.validate()
            .map_err(|err| PresentationError::ValidationError(err.to_string()))?;

        Ok(Self {
            metadata
        })
    }

    pub fn metadata(&self) -> ModelMetadata {
        self.metadata.clone()
    }
}