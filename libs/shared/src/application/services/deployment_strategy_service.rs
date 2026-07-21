use platforms::Platform;
use thiserror::Error;

use std::sync::Arc;

use crate::{
    application::ports::deployment_strategy::{
        DeploymentStrategyProvider,
        GetStrategyByPlatformAndNameInput as GetStrategyByPlatformAndNamePortInput
    },
    domain::entities::deployment_strategy::{
        client_strategy_set::ClientStrategySet, strategy::Strategy
    }
};

#[derive(Debug, Error)]
pub enum DeploymentStrategyServiceError {

}

pub struct DeploymentStrategyService {
    provider: Arc<dyn DeploymentStrategyProvider>
}

impl DeploymentStrategyService {
    pub fn new(provider: Arc<dyn DeploymentStrategyProvider>) -> Self {
        Self { provider }
    }

    pub async fn list_all_strategies(&self) -> Vec<ClientStrategySet> {
        self.provider.list_all().await
    }

    pub async fn get_strategy_by_platform_and_name(&self, input: GetStrategyByPlatformAndNameInput) -> Option<Strategy> {
        self.provider.get_strategy_by_platform_and_name(GetStrategyByPlatformAndNamePortInput {
            platform: input.platform.clone(),
            name: input.name.clone(),
        }).await
    }
}

pub struct GetStrategyByPlatformAndNameInput {
    platform: Platform,
    name: String,
}