use crate::domain::entities::deployment as entities;
use crate::infra::persistence::mongo::documents::deployment as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{Uuid, DateTime};

impl From<&entities::ModelDeployment> for documents::ModelDeployment {
    fn from(value: &entities::ModelDeployment) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            revision: value.revision().clone(),
            owner: value.owner.clone(),
            model: documents::ModelReference::from(value.model.clone()),
            state: documents::State::from(value.state.clone()),
            desired_state: documents::DesiredState::from(value.desired_state.clone()),
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
            last_desired_state_change: DateTime::from_chrono(value.last_desired_state_change.into_inner()),
            last_state_change: DateTime::from_chrono(value.last_state_change.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            metadata: value.metadata.clone(),
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

impl From<entities::State> for documents::State {
    fn from(value: entities::State) -> Self {
        match value {
            entities::State::Blocked => documents::State::Blocked,
            entities::State::Failed => documents::State::Failed,
            entities::State::NotDeployed => documents::State::NotDeployed,
            entities::State::Running => documents::State::Running,
            entities::State::Stopped => documents::State::Stopped,
            entities::State::Unknown => documents::State::Unknown,
        }
    }
}

impl From<entities::DesiredState> for documents::DesiredState {
    fn from(value: entities::DesiredState) -> Self {
        match value {
            entities::DesiredState::NotDeployed => documents::DesiredState::NotDeployed,
            entities::DesiredState::Running => documents::DesiredState::Running,
            entities::DesiredState::Stopped => documents::DesiredState::Stopped,
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