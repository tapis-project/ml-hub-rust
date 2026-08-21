use crate::domain::entities::deployment as entities;
use crate::infra::persistence::mongo::documents::deployment as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use crate::infra::_common::mongo::ToBsonDateTime;
use crate::shared_kernel::enums::DeploymentModality;
use mongodb::bson::Uuid;

impl From<&entities::ModelDeployment> for documents::ModelDeployment {
    fn from(value: &entities::ModelDeployment) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            name: value.name.clone(),
            description: value.description.clone(),
            tenant_id: value.tenant_id.clone(),
            platform: value.platform.clone(),
            deployment_modality: documents::DeploymentModality::from(value.deployment_modality.clone()),
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
            deployment_strategy: value.deployment_strategy.clone(),
            replicas: documents::ReplicaGroup::from(value.replicas.clone()),
            last_desired_state_change: value.last_desired_state_change.to_bson(),
            last_state_change: value.last_state_change.to_bson(),
            created_at: value.created_at.to_bson(),
            last_modified: value.last_modified.to_bson(),
            metadata: value.metadata
                .clone()
                .and_then(|m| Some(m.into_inner().clone())),
        }
    }
}

impl From<entities::ReplicaGroup> for documents::ReplicaGroup {
    fn from(value: entities::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            parallelism_strategies: value.parallelism_strategies
                .iter()
                .map(|ps| documents::ParallelismStrategy::from(ps.clone()))
                .collect()
        }
    }
}

impl From<DeploymentModality> for documents::DeploymentModality {
    fn from(value: DeploymentModality) -> Self {
        match value {
            DeploymentModality::Batch => documents::DeploymentModality::Batch,
            DeploymentModality::Service => documents::DeploymentModality::Service
        }
    }
}

impl From<entities::ParallelismStrategy> for documents::ParallelismStrategy {
    fn from(value: entities::ParallelismStrategy) -> Self {
        match value {
            entities::ParallelismStrategy::PipelineParallelism => documents::ParallelismStrategy::PipelineParallelism,
            entities::ParallelismStrategy::TensorParallelism => documents::ParallelismStrategy::TensorParallelism,
            entities::ParallelismStrategy::SequenceParallelism => documents::ParallelismStrategy::SequenceParallelism,
            entities::ParallelismStrategy::ContextParallelism => documents::ParallelismStrategy::ContextParallelism,
            entities::ParallelismStrategy::ExpertParallelism => documents::ParallelismStrategy::ExpertParallelism,
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
            author: value.author,
            tenant_id: value.tenant_id,
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