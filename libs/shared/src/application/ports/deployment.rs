use std::sync::Arc;
use crate::application::inputs::deployment::{FilterInput, ReconcileModelDeploymentInput};
use crate::application::workflows::reconciliation::{ReconciliationError, ReconciliationOutcome};
use crate::domain::entities::deployment::ModelDeployment;
use crate::application::errors::ApplicationError;
use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use async_trait::async_trait;
use platforms::Platform;
use thiserror::Error;

pub trait DeploymentStrategyProvider {
    fn provide(&self) -> &Vec<ClientStrategySet>;
}

#[async_trait]
pub trait ModelDeploymentRepository: Send + Sync {
    async fn save(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn update(&self, deployment: &ModelDeployment) -> Result<(), ApplicationError>;
    async fn find(&self, input: &FilterInput) -> Result<Option<ModelDeployment>, ApplicationError>;
}

#[async_trait]
pub trait ModelDeploymentPlatformReconciliationClient: Send + Sync {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError>;
}

#[derive(Debug, Error)]
pub enum ModelDeploymentPlatformReconcilerProviderError {
    #[error("{0}")]
    PlatformClientNotFound(String),
}

pub trait ModelDeploymentPlatformReconcilerProvider: Send + Sync {
    fn provide(&self, platform: &Platform) -> Result<Arc<dyn ModelDeploymentPlatformReconciliationClient>, ModelDeploymentPlatformReconcilerProviderError>;
}