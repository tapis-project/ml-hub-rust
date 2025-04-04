use crate::artifacts::{Archive, Compression};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use actix_multipart::Multipart;
use actix_web::web;
// Re-export so clients can use this struct
pub use actix_web::HttpRequest;

#[derive(Deserialize, Debug)]
pub struct ListModelsPath {
    pub platform: String
}

#[derive(Deserialize, Debug)]
pub struct GetModelPath {
    pub platform: String,
    pub model_id: String
}

#[derive(Deserialize, Debug)]
pub struct DownloadModelPath {
    pub platform: String,
    pub model_id: String
}

#[derive(Deserialize, Debug)]
pub struct PublishModelPath {
    pub platform: String,
    pub model_id: String,
    pub path: String
}

#[derive(Deserialize, Debug)]
pub struct DiscoverModelsPath {
    pub platform: String
}

#[derive(Deserialize, Debug)]
pub struct ListDatasetsPath {
    pub platform: String,
}

#[derive(Deserialize, Debug)]
pub struct GetDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Debug)]
pub struct DownloadDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Debug)]
pub struct PublishDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Debug)]
pub struct StartInferenceServerPath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Debug)]
pub struct RunInferencePath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Debug)]
pub struct CreateInferenceServerPath {
    pub platform: String
}

#[derive(Deserialize, Debug)]
pub struct CreateInferencePath {
    pub inference_service_id: String
}

#[derive(Deserialize, Debug)]
pub struct CreateTrainingPath {
    pub platform: String,
    pub training_id: String
}

#[derive(Deserialize, Debug)]
pub struct StartTrainingPath {
    pub platform: String,
    pub training_id: String
}

pub struct ListModelsRequest {
    pub req: HttpRequest,
    pub path: web::Path<ListModelsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

#[derive(Deserialize, Debug)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Deserialize, Debug)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Deserialize, Debug)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Deserialize, Debug)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Deserialize, Debug)]
pub struct ModelDiscoveryCriteria {
    // General fields
    pub name: Option<String>,
    pub model_type: Option<String>,
    pub version: Option<String>,
    pub framework: Option<String>,

    /// Arbitrary labels
    pub labels: Option<Vec<String>>,

    /// Architecture fileds
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

#[derive(Deserialize, Debug)]
pub struct DiscoveryCriteriaBody {
    pub criteria: Vec<ModelDiscoveryCriteria>,
    pub confidence_threshold: Option<Vec<String>>
}


pub type Parameters = std::collections::hash_map::HashMap<String, Value>;

#[derive(Deserialize, Debug)]
pub struct DownloadArtifactBody {
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>,
    pub download_filename: Option<String>,
    pub params: Option<Parameters>,
}

pub struct DiscoverModelsRequest {
    pub req: HttpRequest,
    pub path: web::Path<DiscoverModelsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Json<DiscoveryCriteriaBody>
}

pub struct GetModelRequest {
    pub req: HttpRequest,
    pub path: web::Path<GetModelPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct DownloadModelRequest {
    pub req: HttpRequest,
    pub path: web::Path<DownloadModelPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Json<DownloadArtifactBody>,
}

pub struct PublishModelRequest {
    pub req: HttpRequest,
    pub path: web::Path<PublishModelPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub payload: Multipart,
}

pub struct ListDatasetsRequest {
    pub req: HttpRequest,
    pub path: web::Path<ListDatasetsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct GetDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<GetDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct DownloadDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<DownloadDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Json<DownloadArtifactBody>,
}

pub struct PublishDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<PublishDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub payload: Multipart,
}

pub struct CreateInferenceServerRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateInferenceServerPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}
pub struct CreateInferenceRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartInferenceServerRequest {
    pub req: HttpRequest,
    pub path: web::Path<StartInferenceServerPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct RunInferenceRequest {
    pub req: HttpRequest,
    pub path: web::Path<RunInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct CreateTrainingRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartTrainingRequest {
    pub req: HttpRequest,
    pub path: web::Path<StartTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub mod utils {
    use actix_web::HttpRequest;

    use super::Parameters;
    use crate::errors::Error;
    use std::collections::hash_map::HashMap;

    pub fn param_to_string(params: Option<Parameters>, prop: &str) -> Result<Option<String>, Error> {
        return params.unwrap_or_else(HashMap::new)
            .get(prop)
            .map(|value| {
                if value.is_string() {
                    return Ok(value.to_string())
                }

                Err(Error::new(String::from("Parameter 'branch' must be a string")))
            })
            .transpose();
    }

    pub fn get_header_value(header_key: &str, request: &HttpRequest) -> Option<String> {
        request
            .headers()
            .get(header_key)
            .and_then(|value| value.to_str().ok())
            .map(|value| String::from(value))
    }
}