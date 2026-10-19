use crate::domain::entities::deployment as entities;
use crate::domain::entities::deployment_option::ServingRuntime;
use crate::infra::_common::mongo::ToBsonDateTime;
use crate::infra::persistence::mongo::documents::deployment as documents;
use crate::infra::persistence::mongo::documents::visibility::Visibility;
use crate::shared_kernel::enums::DeploymentModality;
use mongodb::bson::Uuid;

impl From<&entities::ModelDeployment> for documents::ModelDeployment {
    fn from(value: &entities::ModelDeployment) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_uuid().as_bytes()),
            name: value.name.clone(),
            description: value.description.clone(),
            tenant_id: value.tenant_id.clone(),
            deployment_modality: documents::DeploymentModality::from(
                value.deployment_modality.clone(),
            ),
            revision: value.revision().clone(),
            owner: value.owner.clone(),
            external_model_id: Uuid::from_bytes(*value.external_model_id.as_uuid().as_bytes()),
            state: documents::State::from(value.state.clone()),
            batch_execution_outcome: value
                .batch_execution_outcome()
                .cloned()
                .map(documents::BatchExecutionOutcome::from),
            desired_state: documents::DesiredState::from(value.desired_state.clone()),
            last_message: value.last_message.clone(),
            visibility: Visibility::from(value.visibility.clone()),
            target_endpoint: value
                .target_endpoint()
                .cloned()
                .map(documents::ModelDeploymentEndpoint::from),
            deployment_option_id: Uuid::from_bytes(
                *value.deployment_option_id.as_uuid().as_bytes(),
            ),
            deployment_option_snapshot: documents::DeploymentOptionSnapshot::from(
                value.deployment_option_snapshot.clone(),
            ),
            replicas: documents::ReplicaGroup::from(value.replicas.clone()),
            last_desired_state_change: value.last_desired_state_change.to_bson(),
            last_state_change: value.last_state_change.to_bson(),
            last_observed: value.last_observed().map(ToBsonDateTime::to_bson),
            maximum_runtime_seconds: value
                .maximum_runtime()
                .map(|maximum_runtime| maximum_runtime.as_secs()),
            expires_at: value.expires_at().map(ToBsonDateTime::to_bson),
            created_at: value.created_at.to_bson(),
            last_modified: value.last_modified.to_bson(),
            metadata: value
                .metadata
                .clone()
                .and_then(|m| Some(m.into_inner().clone())),
            reconciliation_status: value
                .reconciliation_status()
                .cloned()
                .map(documents::ReconciliationStatus::from),
            retries: value.retries(),
            retry_after: value.retry_after().map(ToBsonDateTime::to_bson),
        }
    }
}

impl From<entities::BatchExecutionOutcome> for documents::BatchExecutionOutcome {
    fn from(value: entities::BatchExecutionOutcome) -> Self {
        match value {
            entities::BatchExecutionOutcome::Succeeded => Self::Succeeded,
            entities::BatchExecutionOutcome::Cancelled => Self::Cancelled,
            entities::BatchExecutionOutcome::Failed => Self::Failed,
        }
    }
}

impl From<entities::ReconciliationStatus> for documents::ReconciliationStatus {
    fn from(value: entities::ReconciliationStatus) -> Self {
        match value {
            entities::ReconciliationStatus::Pending => Self::Pending,
            entities::ReconciliationStatus::Starting => Self::Starting,
            entities::ReconciliationStatus::Stopping => Self::Stopping,
            entities::ReconciliationStatus::Undeploying => Self::Undeploying,
            entities::ReconciliationStatus::Retrying => Self::Retrying,
            entities::ReconciliationStatus::Synchronized => Self::Synchronized,
            entities::ReconciliationStatus::Failed => Self::Failed,
        }
    }
}

impl From<entities::ReplicaGroup> for documents::ReplicaGroup {
    fn from(value: entities::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            parallelism_strategies: value
                .parallelism_strategies
                .iter()
                .map(|ps| documents::ParallelismStrategy::from(ps.clone()))
                .collect(),
        }
    }
}

impl From<DeploymentModality> for documents::DeploymentModality {
    fn from(value: DeploymentModality) -> Self {
        match value {
            DeploymentModality::Batch => documents::DeploymentModality::Batch,
            DeploymentModality::Service => documents::DeploymentModality::Service,
        }
    }
}

impl From<entities::ParallelismStrategy> for documents::ParallelismStrategy {
    fn from(value: entities::ParallelismStrategy) -> Self {
        match value {
            entities::ParallelismStrategy::PipelineParallelism => {
                documents::ParallelismStrategy::PipelineParallelism
            }
            entities::ParallelismStrategy::TensorParallelism => {
                documents::ParallelismStrategy::TensorParallelism
            }
            entities::ParallelismStrategy::SequenceParallelism => {
                documents::ParallelismStrategy::SequenceParallelism
            }
            entities::ParallelismStrategy::ContextParallelism => {
                documents::ParallelismStrategy::ContextParallelism
            }
            entities::ParallelismStrategy::ExpertParallelism => {
                documents::ParallelismStrategy::ExpertParallelism
            }
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

impl From<entities::DeploymentOptionSnapshot> for documents::DeploymentOptionSnapshot {
    fn from(value: entities::DeploymentOptionSnapshot) -> Self {
        let (target_type, hpc_cluster_queue) = match value.target {
            entities::DeploymentTargetSnapshot::HpcClusterQueue(target) => (
                documents::DeploymentTargetType::HpcClusterQueue,
                Some(documents::HpcClusterQueueSnapshot::from(target)),
            ),
        };

        Self {
            deployment_option_id: Uuid::from_bytes(
                *value.deployment_option_id.as_uuid().as_bytes(),
            ),
            external_model_id: Uuid::from_bytes(*value.external_model_id.as_uuid().as_bytes()),
            serving_runtime: documents::ServingRuntime::from(value.serving_runtime),
            target_type,
            hpc_cluster_queue,
        }
    }
}

impl From<ServingRuntime> for documents::ServingRuntime {
    fn from(value: ServingRuntime) -> Self {
        match value {
            ServingRuntime::FlexServ => Self::FlexServ,
        }
    }
}

impl From<entities::HpcClusterQueueSnapshot> for documents::HpcClusterQueueSnapshot {
    fn from(value: entities::HpcClusterQueueSnapshot) -> Self {
        Self {
            hpc_cluster_id: Uuid::from_bytes(*value.hpc_cluster_id.as_uuid().as_bytes()),
            batch_scheduler_queue_id: Uuid::from_bytes(
                *value.batch_scheduler_queue_id.as_uuid().as_bytes(),
            ),
            cluster_host: value.cluster_host,
            queue_name: value.queue_name,
        }
    }
}

impl From<entities::ModelDeploymentEndpoint> for documents::ModelDeploymentEndpoint {
    fn from(value: entities::ModelDeploymentEndpoint) -> Self {
        Self {
            name: value.name().map(String::from),
            protocol: documents::Protocol::from(value.protocol().clone()),
            base_url: value.base_url().map(String::from),
        }
    }
}

impl From<entities::Protocol> for documents::Protocol {
    fn from(value: entities::Protocol) -> Self {
        match value {
            entities::Protocol::RestHttp => Self::RestHttp,
        }
    }
}
