use crate::domain::entities::deployment::State;

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

#[derive(Clone, Debug)]
struct Reason(String);

impl Reason {
    pub fn into_inner(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StartedOutcomePayload {
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StoppedOutcomePayload {
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct UndeployedOutcomePayload {
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ObeservedOutcomePayload {
    pub message: Option<String>,
    pub state: State,
}

#[derive(Clone, Debug)]
pub enum ReconciliationOutcome {
    Started(StartedOutcomePayload),
    Stopped(StoppedOutcomePayload),
    Undeployed(UndeployedOutcomePayload),
    Observed(ObeservedOutcomePayload),
    NoOp,
}