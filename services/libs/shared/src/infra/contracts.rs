use crate::{application::ports::events::Kind, domain::entities::deployment::{DesiredState, State}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError<'a> {
    #[error("Unknown event kind: {0}")]
    UknownEventKind(&'a str),

    #[error("Unknown state: {0}")]
    UknownState(&'a str),

    #[error("Unknown desired state: {0}")]
    UknownDesiredState(&'a str),
}

impl From<&Kind> for String {
    fn from(value: &Kind) -> Self {
        match value {
            Kind::ModelDeploymentDeleted => "model_deployment.deleted".into(),
            Kind::ModelDeploymentStarted => "model_deployment.started".into(),
            Kind::ModelDeploymentStateDriftDetected => "model_deployment.state_drift_detected".into(),
            Kind::ModelDeploymentStopped => "model_deployment.stopped".into(),
        }
    }
}

impl <'a>TryFrom<&'a str> for Kind {
    type Error = ContractError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "model_deployment.deleted" => Kind::ModelDeploymentDeleted,
            "model_deployment.started" => Kind::ModelDeploymentStarted,
            "model_deployment.state_drift_detected" => Kind::ModelDeploymentStateDriftDetected,
            "model_deployment.stopped" => Kind::ModelDeploymentStopped,
            other => return Err(ContractError::UknownEventKind(other))
        })
    }
}

impl <'a>TryFrom<&'a str> for State {
    type Error = ContractError<'a>;
    
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "NotDeployed" => State::NotDeployed,
            "Running" => State::Running,
            "Stopped" => State::Stopped,
            "Failed" => State::Failed,
            "Blocked" => State::Blocked,
            "Unknown" => State::Unknown,
            other => return Err(ContractError::UknownState(other))
        })
    }
}

impl <'a>TryFrom<&'a str> for DesiredState {
    type Error = ContractError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "NotDeployed" => DesiredState::NotDeployed,
            "Running" => DesiredState::Running,
            "Stopped" => DesiredState::Stopped,
            other => return Err(ContractError::UknownDesiredState(other))
        })
    }
}