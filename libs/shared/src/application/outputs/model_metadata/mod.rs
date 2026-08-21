pub mod domain_to_output;
pub mod output_to_domain;

use crate::shared_kernel::enums::Task;
use platforms::Platform;
use uuid::Uuid;

/// ModelMetadata output
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    // General fields
    pub name: String,
    pub author: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub artifact_id: Option<Uuid>,
    pub canonical: Option<Canonical>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub deployment_strategy_refs: Vec<DeploymentStrategyReference>
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

#[derive(Clone, Debug)]
pub struct DeploymentStrategyReference {
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
}

pub struct ModelMetadataListOutput {
    pub models: Vec<ModelMetadata>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}

pub struct ModelMetadataOutput {
    pub model: Option<ModelMetadata>,
}