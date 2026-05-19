use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_multipart::Multipart;
use bytes::Bytes;
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;
use crate::presentation::http::v1::requests::headers::Headers;
use crate::presentation::http::v1::requests::artifacts::{
    DownloadArtifactBody,
    IngestArtifactRequest
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListDatasetsByPlatformPath {
    pub platform: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDatasetByPlatformPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IngestDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishDatasetPath {
    pub platform: String,
    pub dataset_id: String
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
pub struct DatasetMetadata {
    // General fields
    #[validate(required, length(min=1))]
    pub name: Option<String>,
    #[validate(required, length(min=1))]
    pub author: Option<String>,
    pub dataset_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub image: Option<String>,

    /// Arbitrary labels
    pub keywords: Option<Vec<String>>,
    pub annotations: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub bias_evaluation_score: Option<i8>,
}

pub struct ListDatasetsByPlatformRequest {
    pub headers: Headers,
    pub path: ListDatasetsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct GetDatasetByPlatformRequest {
    pub headers: Headers,
    pub path: GetDatasetByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct IngestDatasetRequest {
    pub headers: Headers,
    pub path: IngestDatasetPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactRequest,
}

pub struct DownloadDatasetRequest {
    pub headers: Headers,
    pub path: DownloadDatasetPath,
    pub query: HashMap<String, String>,
    pub body: DownloadArtifactBody,
}

pub struct PublishDatasetRequest {
    pub headers: Headers,
    pub path: PublishDatasetPath,
    pub query: HashMap<String, String>,
    pub payload: Multipart,
}