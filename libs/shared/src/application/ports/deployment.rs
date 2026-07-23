use std::sync::Arc;

// Application layer
use crate::application::inputs::deployment::{FilterInput, ReconcileModelDeploymentInput};
use crate::application::workflows::reconciliation::{ReconciliationError, ReconciliationOutcome};

// Domain layer
use crate::application::ports::errors::CommonRepositoryError;
use crate::domain::entities::deployment::ModelDeployment;

use async_trait::async_trait;
use platforms::Platform;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ModelDeploymentRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait ModelDeploymentRepository: Send + Sync {
    async fn save(&self, deployment: &ModelDeployment) -> Result<(), ModelDeploymentRepositoryError>;
    async fn update(&self, deployment: &ModelDeployment) -> Result<(), ModelDeploymentRepositoryError>;
    async fn find(&self, input: &FilterInput) -> Result<Option<ModelDeployment>, ModelDeploymentRepositoryError>;
    async fn find_by_owner(&self, tenant_id: &str, owner: &str) -> Result<Vec<ModelDeployment>, ModelDeploymentRepositoryError>;
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