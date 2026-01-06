use crate::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;

pub trait AutomatedDeploymentStrategyProvider {
    fn list_all(&self) -> &Vec<ClientStrategySet>;
}