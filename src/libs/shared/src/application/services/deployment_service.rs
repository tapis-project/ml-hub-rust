use std::sync::Arc;
use crate::application::errors::ApplicationError;
use crate::application::ports::deployment::AutomatedDeploymentStrategyProvider;
use crate::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;

pub struct DeploymentService {
    strategy_provider: Arc<dyn AutomatedDeploymentStrategyProvider>
}

impl AutomatedDeploymentStrategyService {
    pub fn new(strategy_provider: Arc<dyn AutomatedDeploymentStrategyProvider>) -> AutomatedDeploymentStrategyService {
        Self {
            strategy_provider
        }
    }

    pub fn get_all_valid_strategy_sets(&self) -> Result<Vec<ClientStrategySet>, ApplicationError> {
        let mut strategy_sets: Vec<ClientStrategySet> = vec![];

        match self.strategy_provider.list_all() {
            Ok(s) => strategy_sets.extend(s),
            Err(err) => return Err(err)
        };
        
        Ok(strategy_sets)
    }
}