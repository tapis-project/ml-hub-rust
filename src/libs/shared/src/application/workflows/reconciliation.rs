use crate::domain::entities::deployment::State;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ReconciliationError {
    #[error("{0}")]
    Unimplemented(String)
}

#[derive(Clone, Debug)]
pub enum ReconciliationAction {
    /// Create the deployment if it doesn't exist and start it
    Start,
    /// Stop the deployment
    Stop,
    /// Observe the reason for the Blocked or Unknown state
    Observe,
    /// Delete the infra
    Undeploy,
}

/// Response info from the deployment library (e.g. FlexServ) so callers can see pod_id, volume_id, pod_url.
#[derive(Clone, Debug, Default)]
pub struct PodResultInfo {
    pub pod_id: Option<String>,
    pub volume_id: Option<String>,
    pub pod_url: Option<String>,
    pub pod_info: Option<String>,
    pub volume_info: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StartedOutcomePayload {
    pub message: Option<String>,
    /// Library response (pod_id, volume_id, pod_url) after create/start.
    pub result: Option<PodResultInfo>,
}

#[derive(Clone, Debug)]
pub struct StoppedOutcomePayload {
    pub message: Option<String>,
    pub result: Option<PodResultInfo>,
}

#[derive(Clone, Debug)]
pub struct UndeployedOutcomePayload {
    pub message: Option<String>,
    pub result: Option<PodResultInfo>,
}

#[derive(Clone, Debug)]
pub struct ObeservedOutcomePayload {
    pub message: Option<String>,
    pub state: State,
    pub result: Option<PodResultInfo>,
}

#[derive(Clone, Debug)]
pub enum ReconciliationOutcome {
    Started(StartedOutcomePayload),
    Stopped(StoppedOutcomePayload),
    Undeployed(UndeployedOutcomePayload),
    Observed(ObeservedOutcomePayload),
    NoOp,
}