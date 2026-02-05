use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::workflows::reconciliation::{ReconciliationError, ReconciliationOutcome};
pub struct TapisPodsModelDeploymentReconciliationClient {}

impl TapisPodsModelDeploymentReconciliationClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconciliationClient for TapisPodsModelDeploymentReconciliationClient {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        Err(ReconciliationError::Unimplemented("".into()))
    }
}