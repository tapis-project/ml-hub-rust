use std::sync::Arc;
use crate::application::ports::deployment::{ModelDeploymentPlatformReconcilerProvider, ModelDeploymentPlatformReconcilerProviderError};
use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::domain::entities::site::SiteContext;
use crate::infra::reconciliation::clients::tapis_jobs::TapisJobsModelDeploymentReconciliationClient;
// use crate::infra::reconciliation::clients::tapis_pods::TapisPodsModelDeploymentReconciliationClient;
use platforms::Platform;

pub struct ReconciliationClientProvider;

impl ReconciliationClientProvider {
    pub fn new() -> Self { Self {} }
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconcilerProvider for ReconciliationClientProvider {
    async fn provide(&self, platform: &Platform, site_context: &SiteContext) -> Result<Arc<dyn ModelDeploymentPlatformReconciliationClient>, ModelDeploymentPlatformReconcilerProviderError> {
        match platform {
            // Platform::TapisPods => Ok(Arc::new(TapisPodsModelDeploymentReconciliationClient::new())),
            Platform::TapisJobs => Ok(Arc::new(TapisJobsModelDeploymentReconciliationClient::new(site_context).await?)),
            _ => Err(ModelDeploymentPlatformReconcilerProviderError::PlatformClientNotFound(platform.to_string()))
        }
    }
}