use std::fs;
use serde_json;
use crate::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::infra::deployment::fs::dtos::{
    client_strategy_set::ClientStrategySet as Config
};
use crate::application::errors::ApplicationError;
use crate::application::ports::deployment::AutomatedDeploymentStrategyProvider;
use crate::constants::DEFAULT_CLIENT_DEPLOYMENT_STRATEGIES_DIR;

pub struct AutomatedDeploymentStrategyProviderFs {
    pub client_strategy_sets: Vec<ClientStrategySet>
}

impl AutomatedDeploymentStrategyProviderFs {
    pub fn new() -> Result<Self, ApplicationError> {
        let config_dir = std::env::var("CLIENT_DEPLOYMENT_STRATEGIES_DIR")
            .unwrap_or(DEFAULT_CLIENT_DEPLOYMENT_STRATEGIES_DIR.into());

        let mut client_strategy_sets: Vec<ClientStrategySet> = vec![];

        let dir_entries = fs::read_dir(config_dir)
            .map_err(|err| {
                ApplicationError::DeploymentStrategyProviderInitialization(err.to_string())
            })?;

        
        for maybe_entry in dir_entries {
            let entry = match maybe_entry {
                Ok(e) => e,
                Err(err) => return Err(ApplicationError::DeploymentStrategyProviderInitialization(err.to_string()))
            };

            match entry.file_type() {
                Ok(t) => {
                    // Ignore if entry is a dir
                    if !t.is_file() {
                        continue;
                    }
                },
                Err(err) => return Err(ApplicationError::DeploymentStrategyProviderInitialization(err.to_string()))
            };
            
            // Load the client strategy set from the file
            let contents = fs::read_to_string(entry.path())
                .map_err(|err| ApplicationError::DeploymentStrategyProviderInitialization(format!("Failed to read config file: {}", err.to_string())))?;
        
            let config: Config = serde_json::from_str(&contents)
                .map_err(|err| ApplicationError::DeploymentStrategyProviderInitialization(format!("Failed to deserialize configuration file contents: {}", err.to_string())))?;
            
            let strategy_set = match ClientStrategySet::try_from(config) {
                Ok(s) => s,
                Err(err) => return Err(ApplicationError::DeploymentStrategyProviderInitialization(err.to_string()))
            };
            client_strategy_sets.push(strategy_set);
        }

        Ok(Self {
            client_strategy_sets
        }) 
    }
}

impl AutomatedDeploymentStrategyProvider for AutomatedDeploymentStrategyProviderFs {
    fn list_all(&self) -> &Vec<ClientStrategySet> {
        &self.client_strategy_sets
    }
}