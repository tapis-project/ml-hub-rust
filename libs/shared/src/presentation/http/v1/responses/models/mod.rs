pub mod mappings;

use serde::Serialize;
use utoipa::ToSchema;

use platforms::Platform;

use crate::presentation::http::v1::responses::tasks::Task;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ModelArtifact {
    pub id: String,
    pub created_at: String,
    pub last_modified: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct ModelMetadata {
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

    /// Deployment strategy references
    pub deployment_strategy_refs: Vec<DeploymentStrategyReference>,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
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

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct Locator {
    pub url: String,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>
}

#[derive(Serialize,Debug, Clone, ToSchema)]
pub struct DeploymentStrategyReference {
    name: String,
    platform: Platform,
    description: Option<String>,
}

// // TODO Future
// #[derive(Serialize, Debug, Clone, ToSchema)]
// pub struct Model {
//     pub metadata: ModelMetadata,
//     pub artifact: Option<ModelArtifact>,
// }
