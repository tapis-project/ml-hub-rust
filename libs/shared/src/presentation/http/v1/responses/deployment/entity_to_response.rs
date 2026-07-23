use crate::domain::entities::deployment as entities;
use crate::presentation::http::v1::responses::deployment as responses;
use crate::presentation::http::v1::responses::visibility::Visibility;

impl From<entities::State> for responses::State {
    fn from(value: entities::State) -> Self {
        match value {
            entities::State::Blocked => responses::State::Blocked,
            entities::State::Failed => responses::State::Failed,
            entities::State::NotDeployed => responses::State::NotDeployed,
            entities::State::Running => responses::State::Running,
            entities::State::Stopped => responses::State::Stopped,
            entities::State::Unknown => responses::State::Unknown,
        }
    }
}

impl From<entities::DesiredState> for responses::DesiredState {
    fn from(value: entities::DesiredState) -> Self {
        match value {
            entities::DesiredState::NotDeployed => responses::DesiredState::NotDeployed,
            entities::DesiredState::Running => responses::DesiredState::Running,
            entities::DesiredState::Stopped => responses::DesiredState::Stopped,
        }
    }
}

impl From<entities::ModelReference> for responses::ModelReference {
    fn from(value: entities::ModelReference) -> Self {
        Self {
            name: value.name,
            author: value.author,
        }
    }
}

impl From<&entities::ParallelismStrategy> for responses::ParallelismStrategy {
    fn from(value: &entities::ParallelismStrategy) -> Self {
        match value {
            entities::ParallelismStrategy::PipelineParallelism => responses::ParallelismStrategy::PipelineParallelism,
            entities::ParallelismStrategy::TensorParallelism => responses::ParallelismStrategy::TensorParallelism,
            entities::ParallelismStrategy::SequenceParallelism => responses::ParallelismStrategy::SequenceParallelism,
            entities::ParallelismStrategy::ContextParallelism => responses::ParallelismStrategy::ContextParallelism,
            entities::ParallelismStrategy::ExpertParallelism => responses::ParallelismStrategy::ExpertParallelism,
        }
    }
}

impl From<entities::GpuResource> for responses::GpuResource {
    fn from(value: entities::GpuResource) -> Self {
        Self {
            memory: value.memory,
            vendor: value.vendor,
            gpu_type: value.gpu_type,
        }
    }
}

impl From<entities::ResourceRequirements> for responses::ResourceRequirements {
    fn from(value: entities::ResourceRequirements) -> Self {
        Self {
            cores: value.cores,
            disk: value.disk,
            memory: value.memory,
            gpu: value.gpu.and_then(|gpu| Some(responses::GpuResource::from(gpu)))
        }
    }
}

impl From<entities::ReplicaGroup> for responses::ReplicaGroup {
    fn from(value: entities::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            parallelism_strategies: value.parallelism_strategies
                .iter()
                .map(|s| responses::ParallelismStrategy::from(s))
                .collect(),
            resources: responses::ResourceRequirements::from(value.resources),
        }
    }
}

impl From<entities::RestApi> for responses::RestApi {
    fn from(value: entities::RestApi) -> Self {
        Self {
            spec: value.spec,
        }
    }
}

impl From<entities::ModelDeploymentInterface> for responses::ModelDeploymentInterface {
    fn from(value: entities::ModelDeploymentInterface) -> Self {
        match value {
            entities::ModelDeploymentInterface::RestApi(r) => responses::ModelDeploymentInterface::RestApi(responses::RestApi::from(r))
        }
    }
}

impl From<entities::ModelDeployment> for responses::ModelDeployment {
    fn from(value: entities::ModelDeployment) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            platform: value.platform.clone(),
            owner: value.owner.clone(),
            model: responses::ModelReference::from(value.model.clone()),
            state: responses::State::from(value.state.clone()),
            desired_state: responses::DesiredState::from(value.desired_state.clone()),
            created_at: String::from(value.created_at.clone()),
            last_desired_state_change: String::from(value.last_desired_state_change.clone()),
            last_state_change: String::from(value.last_state_change.clone()),
            last_modified: String::from(value.last_modified.clone()),
            replicas: value.replicas
                .clone()
                .and_then(|r| Some(responses::ReplicaGroup::from(r))),
            last_message: value.last_message.clone(),
            visibility: Visibility::from(value.visibility.clone()),
            revision: value.revision().clone(),
            deployment_strategy: value.deployment_strategy.clone(),
            deployment_interface: value.deployment_interface
                .clone()
                .and_then(|mdi| Some(responses::ModelDeploymentInterface::from(mdi))),
            metadata: value.metadata
                .clone()
                .and_then(|m| Some(m.into_inner().clone())),
        }
    }
}

