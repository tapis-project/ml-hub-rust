use crate::domain::entities::{
    deployment::{self as entities, ModelDeploymentMetadata},
    deployment_option::{DeploymentOptionId, ServingRuntime},
    hpc_cluster::{BatchSchedulerQueueId, HpcClusterId},
    model::external_model::ExternalModelId,
};
use crate::infra::persistence::mongo::documents::deployment as documents;
use crate::shared_kernel::enums::{DeploymentModality, Visibility};
use crate::shared_kernel::value_objects::TimeStamp;
use uuid::Uuid;

impl TryFrom<&documents::ModelDeployment> for entities::ModelDeployment {
    type Error = String;

    fn try_from(value: &documents::ModelDeployment) -> Result<Self, Self::Error> {
        let external_model_id =
            ExternalModelId::reconstitute(Uuid::from_bytes(value.external_model_id.bytes()));

        let deployment_option_id =
            DeploymentOptionId::reconstitute(Uuid::from_bytes(value.deployment_option_id.bytes()));

        let deployment_option_snapshot =
            entities::DeploymentOptionSnapshot::try_from(value.deployment_option_snapshot.clone())?;

        if deployment_option_snapshot.external_model_id != external_model_id
            || deployment_option_snapshot.deployment_option_id != deployment_option_id
        {
            return Err(
                "Deployment option snapshot identifiers do not match the deployment".into(),
            );
        }

        let props = entities::ReconstituteModelDeploymentProps {
            id: Uuid::from_bytes(value.id.bytes()),
            name: value.name.clone(),
            description: value.description.clone(),
            tenant_id: value.tenant_id.clone(),
            deployment_modality: DeploymentModality::from(value.deployment_modality.clone()),
            revision: value.revision,
            owner: value.owner.clone(),
            external_model_id,
            state: entities::State::from(value.state.clone()),
            desired_state: entities::DesiredState::from(value.desired_state.clone()),
            last_message: value.last_message.clone(),
            deployment_option_id,
            deployment_option_snapshot,
            visibility: Visibility::from(value.visibility.clone()),
            deployment_interface: value
                .deployment_interface
                .clone()
                .map(entities::ModelDeploymentInterface::try_from)
                .transpose()?,
            replicas: entities::ReplicaGroup::from(value.replicas.clone()),
            last_modified: TimeStamp::from(value.last_modified.to_chrono()),
            last_desired_state_change: TimeStamp::from(value.last_desired_state_change.to_chrono()),
            last_state_change: TimeStamp::from(value.last_state_change.to_chrono()),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
            metadata: value.metadata.clone().map(ModelDeploymentMetadata),
        };

        Ok(entities::ModelDeployment::reconstitute(props))
    }
}

impl TryFrom<documents::DeploymentOptionSnapshot> for entities::DeploymentOptionSnapshot {
    type Error = String;

    fn try_from(value: documents::DeploymentOptionSnapshot) -> Result<Self, Self::Error> {
        let target = match (value.target_type, value.hpc_cluster_queue) {
            (documents::DeploymentTargetType::HpcClusterQueue, Some(target)) => {
                entities::DeploymentTargetSnapshot::HpcClusterQueue(target.into())
            }
            (documents::DeploymentTargetType::HpcClusterQueue, None) => {
                return Err("Deployment target discriminator requires hpc_cluster_queue".into());
            }
        };

        Ok(Self {
            deployment_option_id: DeploymentOptionId::reconstitute(Uuid::from_bytes(
                value.deployment_option_id.bytes(),
            )),
            external_model_id: ExternalModelId::reconstitute(Uuid::from_bytes(
                value.external_model_id.bytes(),
            )),
            serving_runtime: ServingRuntime::from(value.serving_runtime),
            target,
        })
    }
}

impl From<documents::ServingRuntime> for ServingRuntime {
    fn from(value: documents::ServingRuntime) -> Self {
        match value {
            documents::ServingRuntime::FlexServ => Self::FlexServ,
        }
    }
}

impl From<documents::HpcClusterQueueSnapshot> for entities::HpcClusterQueueSnapshot {
    fn from(value: documents::HpcClusterQueueSnapshot) -> Self {
        Self {
            hpc_cluster_id: HpcClusterId::reconstitute(Uuid::from_bytes(
                value.hpc_cluster_id.bytes(),
            )),
            batch_scheduler_queue_id: BatchSchedulerQueueId::reconstitute(Uuid::from_bytes(
                value.batch_scheduler_queue_id.bytes(),
            )),
            cluster_host: value.cluster_host,
            queue_name: value.queue_name,
        }
    }
}

impl From<documents::ReplicaGroup> for entities::ReplicaGroup {
    fn from(value: documents::ReplicaGroup) -> Self {
        Self {
            count: value.count,
            parallelism_strategies: value
                .parallelism_strategies
                .into_iter()
                .map(entities::ParallelismStrategy::from)
                .collect(),
        }
    }
}

impl From<documents::DeploymentModality> for DeploymentModality {
    fn from(value: documents::DeploymentModality) -> Self {
        match value {
            documents::DeploymentModality::Batch => Self::Batch,
            documents::DeploymentModality::Service => Self::Service,
        }
    }
}

impl From<documents::ParallelismStrategy> for entities::ParallelismStrategy {
    fn from(value: documents::ParallelismStrategy) -> Self {
        match value {
            documents::ParallelismStrategy::PipelineParallelism => Self::PipelineParallelism,
            documents::ParallelismStrategy::TensorParallelism => Self::TensorParallelism,
            documents::ParallelismStrategy::SequenceParallelism => Self::SequenceParallelism,
            documents::ParallelismStrategy::ContextParallelism => Self::ContextParallelism,
            documents::ParallelismStrategy::ExpertParallelism => Self::ExpertParallelism,
        }
    }
}

impl From<documents::State> for entities::State {
    fn from(value: documents::State) -> Self {
        match value {
            documents::State::Blocked => Self::Blocked,
            documents::State::Failed => Self::Failed,
            documents::State::NotDeployed => Self::NotDeployed,
            documents::State::Running => Self::Running,
            documents::State::Stopped => Self::Stopped,
            documents::State::Unknown => Self::Unknown,
        }
    }
}

impl From<documents::DesiredState> for entities::DesiredState {
    fn from(value: documents::DesiredState) -> Self {
        match value {
            documents::DesiredState::NotDeployed => Self::NotDeployed,
            documents::DesiredState::Running => Self::Running,
            documents::DesiredState::Stopped => Self::Stopped,
        }
    }
}

impl From<documents::RestApi> for entities::RestApi {
    fn from(value: documents::RestApi) -> Self {
        Self { spec: value.spec }
    }
}

impl TryFrom<documents::ModelDeploymentInterface> for entities::ModelDeploymentInterface {
    type Error = String;

    fn try_from(value: documents::ModelDeploymentInterface) -> Result<Self, Self::Error> {
        match (value.interface_type, value.rest_api) {
            (documents::ModelDeploymentInterfaceType::RestApi, Some(interface)) => {
                Ok(Self::RestApi(interface.into()))
            }
            (documents::ModelDeploymentInterfaceType::RestApi, None) => {
                Err("Model deployment interface discriminator requires rest_api".into())
            }
        }
    }
}
