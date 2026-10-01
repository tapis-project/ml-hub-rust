use std::sync::Arc;

// Application layer
use crate::application::inputs::deployment::{FilterInput, ReconcileModelDeploymentInput};
use crate::application::workflows::reconciliation::{ReconcilerError, ReconciliationOutcome};

// Domain layer
use crate::application::ports::errors::InfrastructureError;
use crate::domain::entities::deployment::argument::Argument;
use crate::domain::entities::deployment::{DeploymentOptionSnapshot, ModelDeployment};
use crate::domain::entities::site::SiteContext;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelDeploymentRepositoryError {
    #[error(transparent)]
    Persistence(#[from] InfrastructureError),
}

#[async_trait]
pub trait ModelDeploymentRepository: Send + Sync {
    async fn save(
        &self,
        deployment: &ModelDeployment,
    ) -> Result<(), ModelDeploymentRepositoryError>;

    async fn save_with_arguments(
        &self,
        deployment: &ModelDeployment,
        arguments: &[Argument],
    ) -> Result<(), ModelDeploymentRepositoryError>;

    async fn update(
        &self,
        deployment: &ModelDeployment,
    ) -> Result<(), ModelDeploymentRepositoryError>;

    async fn find(
        &self,
        input: &FilterInput,
    ) -> Result<Option<ModelDeployment>, ModelDeploymentRepositoryError>;

    async fn find_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<ModelDeployment>, ModelDeploymentRepositoryError>;
}

#[async_trait]
pub trait ModelDeploymentReconciliationClient: Send + Sync {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> ReconciliationOutcome;

    fn get_site_context(&self) -> &SiteContext;
}

#[derive(Debug, Error)]
pub enum ModelDeploymentReconcilerProviderError {
    #[error("{0}")]
    ClientNotFound(String),

    #[error("{0}")]
    ClientInitializationError(#[from] ReconcilerError),
}

#[async_trait]
pub trait ModelDeploymentReconcilerProvider: Send + Sync {
    async fn provide(
        &self,
        deployment_option_snapshot: &DeploymentOptionSnapshot,
        site_context: &SiteContext,
    ) -> Result<Arc<dyn ModelDeploymentReconciliationClient>, ModelDeploymentReconcilerProviderError>;
}
