pub mod indexes;

use mongodb::bson::{oid::ObjectId, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::{deployment_option as domain, hpc_cluster},
    infra::persistence::mongo::documents::deployment::DeploymentModality,
    shared_kernel::{identifiers::ExternalModelId, value_objects::TimeStamp},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub external_model_id: Uuid,
    pub supported_deployment_modalities: Vec<DeploymentModality>,
    pub deployment_target_type: DeploymentTargetType,
    pub hpc_cluster_queue: Option<HpcClusterQueueReference>,
    pub serving_runtime: ServingRuntime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentTargetType {
    HpcClusterQueue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpcClusterQueueReference {
    pub hpc_cluster_id: Uuid,
    pub batch_scheduler_queue_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServingRuntime {
    FlexServ,
}

impl From<&domain::DeploymentOption> for DeploymentOption {
    fn from(value: &domain::DeploymentOption) -> Self {
        let (deployment_target_type, hpc_cluster_queue) = match value.deployment_target() {
            domain::DeploymentTarget::HpcClusterQueue(reference) => (
                DeploymentTargetType::HpcClusterQueue,
                Some(HpcClusterQueueReference {
                    hpc_cluster_id: Uuid::from_bytes(
                        *reference.hpc_cluster_id().as_uuid().as_bytes(),
                    ),
                    batch_scheduler_queue_id: Uuid::from_bytes(
                        *reference.batch_scheduler_queue_id().as_uuid().as_bytes(),
                    ),
                }),
            ),
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_uuid().as_bytes()),
            external_model_id: Uuid::from_bytes(*value.external_model_id().as_uuid().as_bytes()),
            supported_deployment_modalities: value
                .supported_deployment_modalities()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            deployment_target_type,
            hpc_cluster_queue,
            serving_runtime: value.serving_runtime().into(),
            created_at: DateTime::from_chrono(value.created_at().into_inner()),
            updated_at: DateTime::from_chrono(value.updated_at().into_inner()),
        }
    }
}

impl TryFrom<DeploymentOption> for domain::DeploymentOption {
    type Error = domain::DeploymentOptionError;

    fn try_from(value: DeploymentOption) -> Result<Self, Self::Error> {
        let deployment_target = match (value.deployment_target_type, value.hpc_cluster_queue) {
            (DeploymentTargetType::HpcClusterQueue, Some(reference)) => {
                domain::DeploymentTarget::HpcClusterQueue(domain::HpcClusterQueueReference::new(
                    hpc_cluster::HpcClusterId::reconstitute(uuid::Uuid::from_bytes(
                        reference.hpc_cluster_id.bytes(),
                    )),
                    hpc_cluster::BatchSchedulerQueueId::reconstitute(uuid::Uuid::from_bytes(
                        reference.batch_scheduler_queue_id.bytes(),
                    )),
                ))
            }
            _ => {
                return Err(domain::DeploymentOptionError::DataIntegrityError(
                    "DeploymentOption target discriminator does not match exactly one target field"
                        .into(),
                ));
            }
        };

        let mut modalities = value
            .supported_deployment_modalities
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();

        let Some(first_modality) = modalities.first().cloned() else {
            return Err(domain::DeploymentOptionError::DataIntegrityError(
                "DeploymentOption supported deployment modalities MUST not be empty".into(),
            ));
        };

        modalities.remove(0);

        domain::DeploymentOption::reconstitute(domain::ReconstituteDeploymentOptionProps {
            id: domain::DeploymentOptionId::reconstitute(uuid::Uuid::from_bytes(value.id.bytes())),
            external_model_id: ExternalModelId::reconstitute(uuid::Uuid::from_bytes(
                value.external_model_id.bytes(),
            )),
            supported_deployment_modalities: nonempty::NonEmpty {
                head: first_modality,
                tail: modalities,
            },
            deployment_target,
            serving_runtime: value.serving_runtime.into(),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
            updated_at: TimeStamp::from(value.updated_at.to_chrono()),
        })
    }
}

impl From<&domain::ServingRuntime> for ServingRuntime {
    fn from(value: &domain::ServingRuntime) -> Self {
        match value {
            domain::ServingRuntime::FlexServ => Self::FlexServ,
        }
    }
}

impl From<ServingRuntime> for domain::ServingRuntime {
    fn from(value: ServingRuntime) -> Self {
        match value {
            ServingRuntime::FlexServ => Self::FlexServ,
        }
    }
}

#[cfg(test)]
#[path = "deployment_option.test.rs"]
mod deployment_option_test;
