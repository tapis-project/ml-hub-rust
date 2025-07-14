use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use actix_multipart::Multipart;
pub use crate::common::presentation::http::v1::dto::{
    Headers,
    DownloadArtifactBody,
    IngestArtifactBody
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListModelsPath {
    pub platform: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetModelPath {
    pub platform: String,
    pub model_id: String
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

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishModelPath {
    pub platform: String,
    pub model_id: String,
    pub path: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscoverModelsPath {
    pub platform: String
}

pub struct ListModelsRequest {
    pub headers: Headers,
    pub path: ListModelsPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}

pub struct DiscoverModelsRequest {
    pub headers: Headers,
    pub path: DiscoverModelsPath,
    pub query: HashMap<String, String>,
    pub body: DiscoveryCriteriaBody
}

pub struct GetModelRequest {
    pub headers: Headers,
    pub path: GetModelPath,
    pub query: HashMap<String, String>,
    pub body: bytes::Bytes,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestModelRequest {
    pub headers: Headers,
    pub path: IngestModelPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactBody,
}

pub struct DownloadModelRequest {
    pub headers: Headers,
    pub path: DownloadModelPath,
}
pub struct PublishModelRequest {
    pub headers: Headers,
    pub path: PublishModelPath,
    pub query: HashMap<String, String>,
    pub payload: Multipart,
}

pub struct UploadModelRequest {
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModelDiscoveryCriteria {
    // General fields
    pub name: Option<String>,
    pub model_type: Option<String>,
    pub version: Option<String>,
    pub framework: Option<String>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub labels: Option<Vec<String>>,
    pub label_map: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<String>>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscoveryCriteriaBody {
    pub criteria: Vec<ModelDiscoveryCriteria>,
    pub confidence_threshold: Option<Vec<String>>
}