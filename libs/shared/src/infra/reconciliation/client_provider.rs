use std::sync::Arc;

use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::domain::entities::deployment::DeploymentReconciliationProvider;
use crate::domain::entities::site::SiteContext;
use crate::infra::configuration::TapisJobsConfiguration;
use crate::infra::reconciliation::clients::tapis_jobs::TapisJobsModelDeploymentReconciliationClient;

// use crate::infra::reconciliation::clients::tapis_pods::TapisPodsModelDeploymentReconciliationClient;

pub struct ReconciliationClientProvider {
    tapis_jobs_configuration: TapisJobsConfiguration,
}

impl ReconciliationClientProvider {
    pub fn new(tapis_jobs_configuration: TapisJobsConfiguration) -> Self {
        Self {
            tapis_jobs_configuration,
        }
    }
}

#[async_trait::async_trait]
impl ModelDeploymentReconcilerProvider for ReconciliationClientProvider {
    async fn provide(
        &self,
        provider: &DeploymentReconciliationProvider,
        site_context: &SiteContext,
    ) -> Result<
        Arc<dyn ModelDeploymentPlatformReconciliationClient>,
        ModelDeploymentReconcilerProviderError,
    > {
        match provider {
            DeploymentReconciliationProvider::TapisJobs => Ok(Arc::new(
                TapisJobsModelDeploymentReconciliationClient::new(
                    site_context,
                    self.tapis_jobs_configuration.clone(),
                )
                .await?,
            )),
        }
    }
}
