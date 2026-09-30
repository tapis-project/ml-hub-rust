/// Defines the user-owned Model aggregate and the global ExternalModel aggregate.
pub mod external_model;

#[cfg(test)]
pub mod fixtures;

use thiserror::Error;
use uuid::Uuid;

use crate::impl_urn_generator;
use crate::shared_kernel::{
    enums::Visibility, identifiers::ExternalModelId, value_objects::TimeStamp,
};

#[derive(Debug, Clone)]
pub struct Model {
    id: Uuid,
    name: String,
    description: Option<String>,
    tenant_id: String,
    owner: String,
    artifact_id: Option<Uuid>,
    external_model_id: ExternalModelId,
    visibility: Visibility,
    updated_at: TimeStamp,
    created_at: TimeStamp,
}

impl_urn_generator!(Model, tenant_id, "model", id);

impl Model {
    pub fn create(
        tenant_id: String,
        owner: String,
        name: String,
        description: Option<String>,
        external_model_id: ExternalModelId,
        visibility: Visibility,
    ) -> Result<Self, ModelError> {
        Self::validate_name(&name)?;

        let now = TimeStamp::now();

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            description,
            tenant_id,
            owner,
            artifact_id: None,
            external_model_id,
            visibility,
            updated_at: now.clone(),
            created_at: now,
        })
    }

    pub fn reconstitute(props: ReconstituteModelProps) -> Result<Self, ModelError> {
        Self::validate_name(&props.name).map_err(|error| {
            ModelError::DataIntegrityError(format!("Model contains an invalid name: {error}"))
        })?;

        Ok(Self {
            id: props.id,
            name: props.name,
            description: props.description,
            tenant_id: props.tenant_id,
            owner: props.owner,
            artifact_id: props.artifact_id,
            external_model_id: props.external_model_id,
            visibility: props.visibility,
            updated_at: props.updated_at,
            created_at: props.created_at,
        })
    }

    pub fn associate_artifact(&mut self, artifact_id: Uuid) {
        self.artifact_id = Some(artifact_id);

        self.updated_at = TimeStamp::now();
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn artifact_id(&self) -> Option<&Uuid> {
        self.artifact_id.as_ref()
    }

    pub fn external_model_id(&self) -> &ExternalModelId {
        &self.external_model_id
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn updated_at(&self) -> &TimeStamp {
        &self.updated_at
    }

    pub fn created_at(&self) -> &TimeStamp {
        &self.created_at
    }

    fn validate_name(name: &str) -> Result<(), ModelError> {
        if name.is_empty() {
            return Err(ModelError::EmptyName);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ReconstituteModelProps {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub owner: String,
    pub artifact_id: Option<Uuid>,
    pub external_model_id: ExternalModelId,
    pub visibility: Visibility,
    pub updated_at: TimeStamp,
    pub created_at: TimeStamp,
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Model name MUST not be empty")]
    EmptyName,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
#[path = "model.test.rs"]
mod model_test;
