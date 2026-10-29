use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    application::outputs::deployment_option::{
        DeploymentOptionDetailOutput, DeploymentOptionOutput,
    },
    domain::entities::deployment_option::{
        self as domain, deployment_parameters as domain_parameters,
    },
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
pub struct DeploymentOptionDetail {
    #[serde(flatten)]
    pub deployment_option: DeploymentOption,
    pub parameters: Vec<DeploymentParameter>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeploymentParameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub parameter_type: DeploymentParameterType,
    pub choices: Option<Vec<DeploymentParameterChoice>>,
    pub default: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum DeploymentParameterType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeploymentParameterChoice {
    pub value: String,
    pub description: Option<String>,
    pub enabled: bool,
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

impl From<DeploymentOptionDetailOutput> for DeploymentOptionDetail {
    fn from(value: DeploymentOptionDetailOutput) -> Self {
        Self {
            deployment_option: value.deployment_option.into(),
            parameters: value
                .parameters
                .into_iter()
                .map(DeploymentParameter::from)
                .collect(),
        }
    }
}

impl From<domain_parameters::Parameter> for DeploymentParameter {
    fn from(value: domain_parameters::Parameter) -> Self {
        Self {
            name: value.name,
            description: value.description,
            required: value.required,
            secret: value.secret,
            parameter_type: value.r#type.into(),
            choices: value.choices.map(|choices| {
                choices
                    .into_iter()
                    .map(DeploymentParameterChoice::from)
                    .collect()
            }),
            default: value.default,
        }
    }
}

impl From<domain_parameters::ParameterType> for DeploymentParameterType {
    fn from(value: domain_parameters::ParameterType) -> Self {
        match value {
            domain_parameters::ParameterType::String => Self::String,
            domain_parameters::ParameterType::Integer => Self::Integer,
            domain_parameters::ParameterType::Float => Self::Float,
            domain_parameters::ParameterType::Boolean => Self::Boolean,
        }
    }
}

impl From<domain_parameters::Choice> for DeploymentParameterChoice {
    fn from(value: domain_parameters::Choice) -> Self {
        Self {
            value: value.value,
            description: value.description,
            enabled: value.enabled,
        }
    }
}

#[cfg(test)]
#[path = "deployment_options.test.rs"]
mod deployment_options_test;
