use crate::domain::entities::deployment::{DesiredState, ModelDeploymentId, State};

#[derive(Debug, Clone)]
pub struct ModelDeploymentStartedPayload {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStoppedPayload {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStateDriftDetectedPayload {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
    pub desired_state: DesiredState,
    pub actual_state: State,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentDeletedPayload {
    pub deployment_id: ModelDeploymentId,
    pub deployment_revision: u32,
    pub message: Option<String>,
}
