use crate::{application::{services::deployment_argument_service::DecryptedArgument}, domain::entities::deployment::{
    DesiredState, ModelDeploymentInterfaceDelta, ModelDeploymentMetadataDelta, ReplicaGroupDelta, State
}};

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ReconcilerError {
    #[error("{0}")]
    InitializationFailed(String)
}

#[derive(Clone, Debug)]
pub enum ReconciliationAction {
    /// Create the deployment if it doesn't exist and start it
    Start { payload: Vec<DecryptedArgument> },
    /// Stop the deployment
    Stop,
    /// Observe the reason for the Blocked or Unknown state
    Observe,
    /// Delete the infra
    Undeploy,
}

#[derive(Clone, Debug)]
pub struct StartedOutcome {
    pub message: Option<String>,
    pub state: State,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
    pub replicas: Option<ReplicaGroupDelta>,
    pub interface: Option<ModelDeploymentInterfaceDelta>,
}

#[derive(Clone, Debug)]
pub struct StoppedOutcome {
    pub message: Option<String>,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
    pub replicas: Option<ReplicaGroupDelta>,
    pub interface: Option<ModelDeploymentInterfaceDelta>,
}

#[derive(Clone, Debug)]
pub struct UndeployedOutcome {
    pub message: Option<String>,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
}

#[derive(Clone, Debug)]
pub struct ObeservedOutcome {
    pub message: Option<String>,
    pub state: State,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
    pub replicas: Option<ReplicaGroupDelta>,
    pub interface: Option<ModelDeploymentInterfaceDelta>,
}

#[derive(Clone, Debug)]
pub struct FailedOutcome {
    pub message: Option<String>,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
    pub replicas: Option<ReplicaGroupDelta>,
    pub interface: Option<ModelDeploymentInterfaceDelta>,
}

#[derive(Clone, Debug)]
pub struct UnknownOutcome {
    pub desired_state: DesiredState,
    pub message: Option<String>,
    pub metadata: Option<ModelDeploymentMetadataDelta>,
    pub replicas: Option<ReplicaGroupDelta>,
    pub interface: Option<ModelDeploymentInterfaceDelta>,
}

#[derive(Clone, Debug)]
pub enum ReconciliationOutcome {
    Started(StartedOutcome),
    Stopped(StoppedOutcome),
    Undeployed(UndeployedOutcome),
    Observed(ObeservedOutcome),
    Failed(FailedOutcome),
    Unknown(UnknownOutcome),
    NoOp,
}