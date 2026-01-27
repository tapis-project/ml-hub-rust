use crate::domain::entities::deployment as entities;
use crate::infra::persistence::mongo::documents::deployment as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{Uuid, DateTime};

impl From<&entities::ModelDeployment> for documents::ModelDeployment {
    fn from(value: &entities::ModelDeployment) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            owner: value.owner.clone(),
            model: documents::ModelReference::from(value.model.clone()),
            status: documents::ModelDeploymentStatus::from(value.status.clone()),
            last_message: value.last_message.clone(),
            visibility: Visibility::from(value.visibility.clone()),
            deployment_interface: value.deployment_interface
                .clone()
                .and_then(|di| Some(documents::ModelDeploymentInterface::from(di))),
            deployment_strategy: value.deployment_strategy
                .clone()
                .and_then(|dsr| Some(documents::DeploymentStrategyReference::from(dsr))),
            parallelism: value.parallelism
                .clone()
                .and_then(|rg| Some(documents::ReplicaGroup::from(rg))),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
        }
    }
}

impl From<entities::ReplicaGroup> for documents::ReplicaGroup {
    fn from(value: entities::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            resources: documents::ResourceRequirements::from(value.resources),
            parallelism_strategies: value.parallelism_strategies
                .iter()
                .map(|ps| documents::ParallelismStrategy::from(ps.clone()))
                .collect()
        }
    }
}

impl From<entities::ResourceRequirements> for documents::ResourceRequirements {
    fn from(value: entities::ResourceRequirements) -> Self {
        Self {
            cores: value.cores,
            disk: value.disk,
            memory: value.memory,
            gpu: value.gpu
                .and_then(|gr| Some(documents::GpuResource::from(gr))),
        }
    }
}

impl From<entities::GpuResource> for documents::GpuResource {
    fn from(value: entities::GpuResource) -> Self {
        Self {
            gpu_type: value.gpu_type,
            memory: value.memory,
            vendor: value.vendor,
        }
    }
}

impl From<entities::ParallelismStrategy> for documents::ParallelismStrategy {
    fn from(value: entities::ParallelismStrategy) -> Self {
        match value {
            entities::ParallelismStrategy::DataSharding => documents::ParallelismStrategy::DataSharding,
            entities::ParallelismStrategy::ModelSharding => documents::ParallelismStrategy::ModelSharding,
            entities::ParallelismStrategy::TensorSharding => documents::ParallelismStrategy::TensorSharding,
            entities::ParallelismStrategy::PipelineSharding => documents::ParallelismStrategy::PipelineSharding,
        }
    }
}

impl From<entities::ModelDeploymentStatus> for documents::ModelDeploymentStatus {
    fn from(value: entities::ModelDeploymentStatus) -> Self {
        match value {
            entities::ModelDeploymentStatus::Failed => documents::ModelDeploymentStatus::Failed,
            entities::ModelDeploymentStatus::Submitted => documents::ModelDeploymentStatus::Submitted,
            entities::ModelDeploymentStatus::Queued => documents::ModelDeploymentStatus::Queued,
            entities::ModelDeploymentStatus::Provisioning => documents::ModelDeploymentStatus::Provisioning,
            entities::ModelDeploymentStatus::Starting => documents::ModelDeploymentStatus::Starting,
            entities::ModelDeploymentStatus::Running => documents::ModelDeploymentStatus::Running,
            entities::ModelDeploymentStatus::Stopping => documents::ModelDeploymentStatus::Stopping,
            entities::ModelDeploymentStatus::Stopped => documents::ModelDeploymentStatus::Stopped,
        }
    }
}

impl From<entities::ModelReference> for documents::ModelReference {
    fn from(value: entities::ModelReference) -> Self {
        Self {
            name: value.name,
            author: value.author
        }
    }
}

impl From<entities::DeploymentStrategyReference> for documents::DeploymentStrategyReference {
    fn from(value: entities::DeploymentStrategyReference) -> Self {
        Self {
            client: value.client,
            name: value.name
        }
    }
}

impl From<entities::RestApi> for documents::RestApi {
    fn from(value: entities::RestApi) -> Self {
        Self {
            spec: value.spec
        }
    }
}

impl From<entities::ModelDeploymentInterface> for documents::ModelDeploymentInterface {
    fn from(value: entities::ModelDeploymentInterface) -> Self {
        match value {
            entities::ModelDeploymentInterface::RestApi(i) => documents::ModelDeploymentInterface::RestApi(documents::RestApi::from(i))
        }
    }
}