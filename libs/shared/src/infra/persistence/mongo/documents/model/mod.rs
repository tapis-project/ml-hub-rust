mod document_to_domain;
mod domain_to_document;
pub mod indexes;

use crate::infra::persistence::mongo::documents::task::Task;
use mongodb::bson::{oid::ObjectId, Uuid};
use platforms::Platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canonical {
    pub platform: String,
    pub model_id: String,
    pub locator: Locator,
    pub author: Option<String>,
    pub likes: Option<u64>,
    pub downloads: Option<u64>,
    pub gated: Option<bool>,
    pub private: Option<bool>,
    pub sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locator {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentStrategyReference {
    pub name: String,
    pub platform: Platform,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    // Unique ID of the artifact associated with this model
    pub artifact_id: Option<Uuid>,

    // General fields
    pub name: String,
    pub author: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub canonical: Option<Canonical>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,

    /// Deployment-strategy related data
    pub deployment_strategy_refs: Vec<DeploymentStrategyReference>,
}
