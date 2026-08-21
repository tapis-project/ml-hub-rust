use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::domain::entities::deployment_strategy::strategy::Strategy;
use async_trait::async_trait;
use platforms::Platform;

#[async_trait]
pub trait DeploymentStrategyProvider: Send + Sync {
    async fn get_strategy_by_platform_and_name(&self, input: GetStrategyByPlatformAndNameInput) -> Option<Strategy>;
    async fn list_all(&self) -> Vec<ClientStrategySet>;
}

pub struct GetStrategyByPlatformAndNameInput {
    pub platform: Platform,
    pub name: String
}
