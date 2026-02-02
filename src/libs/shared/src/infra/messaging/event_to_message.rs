use crate::application::ports::events::{
    ModelDeploymentStateDriftDetectedPayload,
    ModelDeploymentDeletedPayload,
    ModelDeploymentStartedPayload,
    ModelDeploymentStoppedPayload,
};
use crate::infra::messaging::messages;

impl From<&ModelDeploymentStateDriftDetectedPayload> for messages::ModelDeploymentStateDriftDetectedMessage {
    fn from(value: &ModelDeploymentStateDriftDetectedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            acutal_state: String::from(value.acutal_state),
            desired_state: String::from(value.desired_state),
            deployment_revision: value.deployment_revision,
            timestamp: value.timestamp.into_inner().to_string(),
        }
    }
}

impl From<&ModelDeploymentDeletedPayload> for messages::ModelDeploymentDeletedMessage {
    fn from(value: &ModelDeploymentDeletedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            timestamp: value.timestamp.into_inner().to_string(),
        }
    }
}

impl From<&ModelDeploymentStartedPayload> for messages::ModelDeploymentStartedMessage {
    fn from(value: &ModelDeploymentStartedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            timestamp: value.timestamp.into_inner().to_string(),
        }
    }
}

impl From<&ModelDeploymentStoppedPayload> for messages::ModelDeploymentStoppedMessage {
    fn from(value: &ModelDeploymentStoppedPayload) -> Self {
        Self {
            deployment_id: String::from(value.deployment_id),
            deployment_revision: value.deployment_revision,
            timestamp: value.timestamp.into_inner().to_string(),
        }
    }
}