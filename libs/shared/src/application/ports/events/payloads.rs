use uuid::Uuid;
use crate::domain::entities::deployment::{DesiredState, State};

#[derive(Debug, Clone)]
pub struct ModelDeploymentStartedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStoppedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentStateDriftDetectedPayload {
    pub deployment_revision: u32,
    pub deployment_id: Uuid,
    pub desired_state: DesiredState,
    pub actual_state: State,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelDeploymentDeletedPayload {
    pub deployment_id: Uuid,
    pub deployment_revision: u32,
    pub message: Option<String>,
}