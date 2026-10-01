use std::sync::Arc;

use crate::application::ports::deployment::ModelDeploymentReconciliationClient;
use crate::application::ports::deployment::{
    ModelDeploymentReconcilerProvider, ModelDeploymentReconcilerProviderError,
};
use crate::domain::entities::deployment::{DeploymentOptionSnapshot, DeploymentTargetSnapshot};
use crate::domain::entities::deployment_option::ServingRuntime;
use crate::domain::entities::site::SiteContext;
use crate::infra::reconciliation::clients::tapis_jobs::TapisJobsModelDeploymentReconciliationClient;

// use crate::infra::reconciliation::clients::tapis_pods::TapisPodsModelDeploymentReconciliationClient;

pub struct ReconciliationClientProvider;

impl ReconciliationClientProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModelDeploymentReconcilerProvider for ReconciliationClientProvider {
    async fn provide(
        &self,
        deployment_option_snapshot: &DeploymentOptionSnapshot,
        site_context: &SiteContext,
    ) -> Result<Arc<dyn ModelDeploymentReconciliationClient>, ModelDeploymentReconcilerProviderError>
    {
        match (
            &deployment_option_snapshot.target,
            deployment_option_snapshot.serving_runtime,
        ) {
            (DeploymentTargetSnapshot::HpcClusterQueue(_), ServingRuntime::FlexServ) => Ok(
                Arc::new(TapisJobsModelDeploymentReconciliationClient::new(site_context).await?),
            ),
        }
    }
}
