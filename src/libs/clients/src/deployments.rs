use shared::application::inputs::deployment::ReconcileDeploymentInput;
use shared::application::workflows::reconciliation::ReconciliationOutcome;
use crate::client::Client;
use shared::domain::entities::deployment::ModelDeployment;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelDeploymentError {
    #[error("{0}")]
    Unimplemented(String),
}

#[async_trait::async_trait]
pub trait ModelDeploymentReconciliationClient: Client {
    async fn reconcile(&self, input: &ReconcileDeploymentInput) -> Result<ReconciliationOutcome, ModelDeploymentError> {
        return Err(ModelDeploymentError::Unimplemented("Not implemented".into()));
    }

    fn capabilities(&self) -> ModelDeploymentCapabilities;
}