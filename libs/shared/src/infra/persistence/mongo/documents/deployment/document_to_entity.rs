use crate::domain::entities::deployment::{self as entities, ModelDeploymentMetadata};
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;
use crate::infra::persistence::mongo::documents::deployment as documents;
use uuid::Uuid;

impl From<&documents::ModelDeployment> for entities::ModelDeployment {
    fn from(value: &documents::ModelDeployment) -> Self {
        let props = entities::RehydrateModelDeploymentProps {
            id: Uuid::from_bytes(value.id.bytes()),
            platform: value.platform.clone(),
            revision: value.revision.clone(),
            owner: value.owner.clone(),
            tenant_id: value.tenant_id.clone(),
            model: entities::ModelReference::from(value.model.clone()),
            state: entities::State::from(value.state.clone()),
            desired_state: entities::DesiredState::from(value.desired_state.clone()),
            last_message: value.last_message.clone(),
            visibility: Visibility::from(value.visibility.clone()),
            deployment_interface: value.deployment_interface
                .clone()
                .and_then(|di| Some(entities::ModelDeploymentInterface::from(di))),
            deployment_strategy: value.deployment_strategy.clone(),
            replicas: value.replicas
                .clone()
                .and_then(|rg| Some(entities::ReplicaGroup::from(rg))),
            last_modified: TimeStamp::from(value.last_modified.to_chrono()),
            last_desired_state_change: TimeStamp::from(value.last_desired_state_change.to_chrono()),
            last_state_change: TimeStamp::from(value.last_state_change.to_chrono()),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
            metadata: value.metadata
                .clone()
                .and_then(|m| Some(ModelDeploymentMetadata(m))),
        };
        
        entities::ModelDeployment::rehydrate(props)
    }
}

impl From<documents::ReplicaGroup> for entities::ReplicaGroup {
    fn from(value: documents::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            resources: entities::ResourceRequirements::from(value.resources),
            parallelism_strategies: value.parallelism_strategies
                .iter()
                .map(|ps| entities::ParallelismStrategy::from(ps.clone()))
                .collect()
        }
    }
}

impl From<documents::ResourceRequirements> for entities::ResourceRequirements {
    fn from(value: documents::ResourceRequirements) -> Self {
        Self {
            cores: value.cores,
            disk: value.disk,
            memory: value.memory,
            gpu: value.gpu
                .and_then(|gr| Some(entities::GpuResource::from(gr))),
        }
    }
}

impl From<documents::GpuResource> for entities::GpuResource {
    fn from(value: documents::GpuResource) -> Self {
        Self {
            gpu_type: value.gpu_type,
            memory: value.memory,
            vendor: value.vendor,
        }
    }
}

impl From<documents::ParallelismStrategy> for entities::ParallelismStrategy {
    fn from(value: documents::ParallelismStrategy) -> Self {
        match value {
            documents::ParallelismStrategy::DataSharding => entities::ParallelismStrategy::DataSharding,
            documents::ParallelismStrategy::ModelSharding => entities::ParallelismStrategy::ModelSharding,
            documents::ParallelismStrategy::TensorSharding => entities::ParallelismStrategy::TensorSharding,
            documents::ParallelismStrategy::PipelineSharding => entities::ParallelismStrategy::PipelineSharding,
        }
    }
}

impl From<documents::State> for entities::State {
    fn from(value: documents::State) -> Self {
        match value {
            documents::State::Blocked => entities::State::Blocked,
            documents::State::Failed => entities::State::Failed,
            documents::State::NotDeployed => entities::State::NotDeployed,
            documents::State::Running => entities::State::Running,
            documents::State::Stopped => entities::State::Stopped,
            documents::State::Unknown => entities::State::Unknown,
        }
    }
}

impl From<documents::DesiredState> for entities::DesiredState {
    fn from(value: documents::DesiredState) -> Self {
        match value {
            documents::DesiredState::NotDeployed => entities::DesiredState::NotDeployed,
            documents::DesiredState::Running => entities::DesiredState::Running,
            documents::DesiredState::Stopped => entities::DesiredState::Stopped,
        }
    }
}

impl From<documents::ModelReference> for entities::ModelReference {
    fn from(value: documents::ModelReference) -> Self {
        Self {
            name: value.name,
            author: value.author
        }
    }
}

impl From<documents::RestApi> for entities::RestApi {
    fn from(value: documents::RestApi) -> Self {
        Self {
            spec: value.spec
        }
    }
}

impl From<documents::ModelDeploymentInterface> for entities::ModelDeploymentInterface {
    fn from(value: documents::ModelDeploymentInterface) -> Self {
        match value {
            documents::ModelDeploymentInterface::RestApi(i) => entities::ModelDeploymentInterface::RestApi(entities::RestApi::from(i))
        }
    }
}