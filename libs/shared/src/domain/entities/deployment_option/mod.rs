pub mod parameter_set;
pub mod traits;

use std::fmt;

use nonempty::NonEmpty;
use thiserror::Error;
use traits::ProvideDeploymentParameters;
use parameter_set::{Parameter, ParameterType, Choice};
use uuid::Uuid;

use crate::domain::entities::hpc_cluster::{BatchSchedulerQueueId, HpcClusterId};
use crate::shared_kernel::{
    enums::DeploymentModality, identifiers::ExternalModelId, value_objects::TimeStamp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeploymentOptionId(Uuid);

impl DeploymentOptionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn reconstitute(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DeploymentOptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeploymentOptionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone)]
pub struct DeploymentOption {
    id: DeploymentOptionId,
    external_model_id: ExternalModelId,
    supported_deployment_modalities: NonEmpty<DeploymentModality>,
    deployment_target: DeploymentTarget,
    serving_runtime: ServingRuntime,
    created_at: TimeStamp,
    updated_at: TimeStamp,
}

impl DeploymentOption {
    pub fn new(props: NewDeploymentOptionProps) -> Result<Self, DeploymentOptionError> {
        let now = TimeStamp::now();

        Self::build(
            ReconstituteDeploymentOptionProps {
                id: DeploymentOptionId::new(),
                external_model_id: props.external_model_id,
                supported_deployment_modalities: props.supported_deployment_modalities,
                deployment_target: props.deployment_target,
                serving_runtime: props.serving_runtime,
                created_at: now.clone(),
                updated_at: now,
            },
            false,
        )
    }

    pub fn reconstitute(
        props: ReconstituteDeploymentOptionProps,
    ) -> Result<Self, DeploymentOptionError> {
        Self::build(props, true)
    }

    fn build(
        props: ReconstituteDeploymentOptionProps,
        persisted: bool,
    ) -> Result<Self, DeploymentOptionError> {
        if let Err(error) = Self::validate(&props) {
            if persisted {
                return Err(DeploymentOptionError::DataIntegrityError(error.to_string()));
            }

            return Err(error);
        }

        Ok(Self {
            id: props.id,
            external_model_id: props.external_model_id,
            supported_deployment_modalities: props.supported_deployment_modalities,
            deployment_target: props.deployment_target,
            serving_runtime: props.serving_runtime,
            created_at: props.created_at,
            updated_at: props.updated_at,
        })
    }

    fn validate(props: &ReconstituteDeploymentOptionProps) -> Result<(), DeploymentOptionError> {
        let modalities = props
            .supported_deployment_modalities
            .iter()
            .collect::<Vec<_>>();

        for (index, modality) in modalities.iter().enumerate() {
            if modalities[index + 1..].contains(modality) {
                return Err(DeploymentOptionError::DuplicateDeploymentModality(
                    (*modality).clone(),
                ));
            }
        }

        if props.updated_at.into_inner() < props.created_at.into_inner() {
            return Err(DeploymentOptionError::InvalidTimestamps);
        }

        Ok(())
    }

    pub fn replace_supported_deployment_modalities(
        &mut self,
        supported_deployment_modalities: NonEmpty<DeploymentModality>,
    ) -> Result<(), DeploymentOptionError> {
        let props = ReconstituteDeploymentOptionProps {
            id: self.id,
            external_model_id: self.external_model_id,
            supported_deployment_modalities: supported_deployment_modalities.clone(),
            deployment_target: self.deployment_target.clone(),
            serving_runtime: self.serving_runtime,
            created_at: self.created_at.clone(),
            updated_at: TimeStamp::now(),
        };

        Self::validate(&props)?;

        self.supported_deployment_modalities = supported_deployment_modalities;
        self.updated_at = props.updated_at;

        Ok(())
    }

    pub fn has_same_semantic_identity(&self, other: &Self) -> bool {
        self.external_model_id == other.external_model_id
            && self.serving_runtime == other.serving_runtime
            && self.deployment_target == other.deployment_target
    }

    pub fn id(&self) -> &DeploymentOptionId {
        &self.id
    }

    pub fn external_model_id(&self) -> &ExternalModelId {
        &self.external_model_id
    }

    pub fn supported_deployment_modalities(&self) -> &NonEmpty<DeploymentModality> {
        &self.supported_deployment_modalities
    }

    pub fn deployment_target(&self) -> &DeploymentTarget {
        &self.deployment_target
    }

    pub fn serving_runtime(&self) -> &ServingRuntime {
        &self.serving_runtime
    }

    pub fn created_at(&self) -> &TimeStamp {
        &self.created_at
    }

    pub fn updated_at(&self) -> &TimeStamp {
        &self.updated_at
    }
}

#[derive(Debug, Clone)]
pub struct NewDeploymentOptionProps {
    pub external_model_id: ExternalModelId,
    pub supported_deployment_modalities: NonEmpty<DeploymentModality>,
    pub deployment_target: DeploymentTarget,
    pub serving_runtime: ServingRuntime,
}

#[derive(Debug, Clone)]
pub struct ReconstituteDeploymentOptionProps {
    pub id: DeploymentOptionId,
    pub external_model_id: ExternalModelId,
    pub supported_deployment_modalities: NonEmpty<DeploymentModality>,
    pub deployment_target: DeploymentTarget,
    pub serving_runtime: ServingRuntime,
    pub created_at: TimeStamp,
    pub updated_at: TimeStamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeploymentTarget {
    HpcClusterQueue(HpcClusterQueueReference),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HpcClusterQueueReference {
    hpc_cluster_id: HpcClusterId,
    batch_scheduler_queue_id: BatchSchedulerQueueId,
}

impl HpcClusterQueueReference {
    pub fn new(
        hpc_cluster_id: HpcClusterId,
        batch_scheduler_queue_id: BatchSchedulerQueueId,
    ) -> Self {
        Self {
            hpc_cluster_id,
            batch_scheduler_queue_id,
        }
    }

    pub fn hpc_cluster_id(&self) -> &HpcClusterId {
        &self.hpc_cluster_id
    }

    pub fn batch_scheduler_queue_id(&self) -> &BatchSchedulerQueueId {
        &self.batch_scheduler_queue_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServingRuntime {
    FlexServ,
}

impl ProvideDeploymentParameters for ServingRuntime {
    fn provide_parameters(&self) -> Vec<Parameter> {
        match self {
            Self::FlexServ => vec![
                Parameter {
                    name: "FlexServ Version".into(),
                    description: Some("The version of the FlexServ model serving software to deploy".into()),
                    required: false,
                    default: Some("1.5".into()),
                    secret: false,
                    r#type: ParameterType::String,
                    choices: Some(vec![
                        Choice {
                            value: "1.4".into(),
                            description: Some("FlexServ v1.4.0".into()),
                            enabled: true,
                        },
                        Choice {
                            value: "1.5".into(),
                            description: Some("FlexServ v1.4.0".into()),
                            enabled: true,
                        },
                    ])
                }
            ]
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum DeploymentOptionError {
    #[error("DeploymentOption contains duplicate deployment modality {0}")]
    DuplicateDeploymentModality(DeploymentModality),

    #[error("DeploymentOption updated_at MUST not precede created_at")]
    InvalidTimestamps,

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
#[path = "deployment_option.test.rs"]
mod deployment_option_test;
