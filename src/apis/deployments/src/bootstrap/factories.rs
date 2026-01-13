//! This module contains factories that wire together infrastructure-level concerns
//! with application level concerns
// use mongodb::Database;
use std::sync::Arc;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment::AutomatedDeploymentStrategyProvider;
use shared::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::automated_deployment_strategy_provider::AutomatedDeploymentStrategyProviderFs;

// #[cfg(feature = "mongodb")]
// pub fn inference_server_repo_factory(db: Database) -> Arc<dyn InferenceServerRepository> {
//     Arc::new(MongoInferenceServerRepository::new(db))
// }

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn AutomatedDeploymentStrategyProvider>, ApplicationError> {
    let provider = AutomatedDeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}