use crate::{application::errors::ApplicationError, bootstrap::SiteConfiguration, constants::DEFAULT_SITE_CONFGIURATION_PATH};
use log::error;

pub struct SiteConfigurationRepository {
    config: SiteConfiguration
}

impl SiteConfigurationRepository {
    pub fn new() -> Result<Self, ApplicationError> {
        let config_path = std::env::var("SITE_CONFIG_PATH")
                .unwrap_or(DEFAULT_SITE_CONFGIURATION_PATH.into());

        // Load the client strategy set from the file
        let contents = std::fs::read_to_string(std::path::PathBuf::from(config_path))
            .map_err(|err| {
                error!("Error reading contents of site configuration file {}", err.to_string());
                ApplicationError::SiteConfigLoaderInitialization(format!("Failed to read config file: {}", err.to_string()))
            })?;
        
        let config = serde_json::from_str::<SiteConfiguration>(&contents)
            .map_err(|err| {
                error!("Failed to deserialize site configuration: {}", err.to_string());
                ApplicationError::SiteConfigLoaderInitialization(format!("Failed to deserialize configuration file contents: {}", err.to_string()))
            })?;

        Ok(Self { config })
    }

    pub fn get_config(&self) -> SiteConfiguration {
        return self.config.clone()
    }
}