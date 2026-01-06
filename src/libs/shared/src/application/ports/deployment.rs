use crate::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;

pub trait AutomatedDeploymentStrategyProvider {
    fn provide(&self) -> &Vec<ClientStrategySet>;
}