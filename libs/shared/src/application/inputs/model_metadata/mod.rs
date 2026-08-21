pub mod inputs_to_entities;
pub mod input_to_input;
pub mod entities_to_input;
pub mod output_to_entity;

use crate::shared_kernel::enums::Task;

use platforms::Platform;
use uuid::Uuid;

use crate::application::inputs::common::Scope;

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
pub struct RegisterModelMetadataInput {
    // General fields
    pub name: String,
    pub description: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub canonical: Option<Canonical>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
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

