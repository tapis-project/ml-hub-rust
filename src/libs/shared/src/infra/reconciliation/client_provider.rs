use std::sync::Arc;
use crate::application::ports::deployment::{ModelDeploymentPlatformReconcilerProvider, ModelDeploymentPlatformReconcilerProviderError};
use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::infra::reconciliation::clients::tapis_pods::TapisPodsModelDeploymentReconciliationClient;
use platforms::Platform;

pub struct ReconciliationClientProvider;

impl ReconciliationClientProvider {
    pub fn new() -> Self { Self {} }
}

impl ModelDeploymentPlatformReconcilerProvider for ReconciliationClientProvider {
    fn provide(&self, platform: &Platform) -> Result<Arc<dyn ModelDeploymentPlatformReconciliationClient>, ModelDeploymentPlatformReconcilerProviderError> {
        match platform {
            Platform::TapisPods => Ok(Arc::new(TapisPodsModelDeploymentReconciliationClient::new())),
            _ => Err(ModelDeploymentPlatformReconcilerProviderError::PlatformClientNotFound(platform.to_string()))
        }
    }
}