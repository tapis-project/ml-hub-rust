use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::outputs::deployment_option::DeploymentOptionOutput,
    domain::entities::deployment_option as domain,
    presentation::http::v1::{
        deployment_options::{DeploymentModality, ServingRuntime},
        responses::hpc_clusters::DataCenter,
    },
    shared_kernel::enums::DeploymentModality as DomainDeploymentModality,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct DeploymentOption {
    pub id: Uuid,
    pub external_model_id: Uuid,
    pub supported_deployment_modalities: Vec<DeploymentModality>,
    pub serving_runtime: ServingRuntime,
    pub deployment_target_type: DeploymentTargetType,
    pub hpc_cluster_queue: Option<HpcClusterQueueTarget>,
    pub available: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum DeploymentTargetType {
    HpcClusterQueue,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HpcClusterQueueTarget {
    pub hpc_cluster_id: Uuid,
    pub hpc_cluster_name: String,
    pub data_center: DataCenter,
    pub hpc_cluster_enabled: bool,
    pub batch_scheduler_queue_id: Uuid,
    pub batch_scheduler_queue_name: String,
    pub batch_scheduler_queue_enabled: bool,
}

impl From<DeploymentOptionOutput> for DeploymentOption {
    fn from(value: DeploymentOptionOutput) -> Self {
        let deployment_target_type = match value.deployment_option.deployment_target() {
            domain::DeploymentTarget::HpcClusterQueue(_) => DeploymentTargetType::HpcClusterQueue,
        };

        let supported_deployment_modalities = value
            .deployment_option
            .supported_deployment_modalities()
            .iter()
            .map(|modality| match modality {
                DomainDeploymentModality::Batch => DeploymentModality::Batch,
                DomainDeploymentModality::Service => DeploymentModality::Service,
            })
            .collect();

        let serving_runtime = match value.deployment_option.serving_runtime() {
            domain::ServingRuntime::FlexServ => ServingRuntime::FlexServ,
        };

        Self {
            id: *value.deployment_option.id().as_uuid(),
            external_model_id: value.deployment_option.external_model_id().into_uuid(),
            supported_deployment_modalities,
            serving_runtime,
            deployment_target_type,
            hpc_cluster_queue: Some(HpcClusterQueueTarget {
                hpc_cluster_id: *value.target.hpc_cluster_id.as_uuid(),
                hpc_cluster_name: value.target.hpc_cluster_name,
                data_center: (&value.target.data_center).into(),
                hpc_cluster_enabled: value.target.hpc_cluster_enabled,
                batch_scheduler_queue_id: *value.target.batch_scheduler_queue_id.as_uuid(),
                batch_scheduler_queue_name: value.target.batch_scheduler_queue_name,
                batch_scheduler_queue_enabled: value.target.batch_scheduler_queue_enabled,
            }),
            available: value.available,
            created_at: value.deployment_option.created_at().clone().into(),
            updated_at: value.deployment_option.updated_at().clone().into(),
        }
    }
}

#[cfg(test)]
#[path = "deployment_options.test.rs"]
mod deployment_options_test;
