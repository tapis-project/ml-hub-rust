use crate::infra::configuration::{SiteConfiguration, DEFAULT_SITE_CONFGIURATION_PATH};
use log::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SiteConfigurationLoaderError {
    #[error("Failed to load the site configuration: {0}")]
    LoadingError(String),

    #[error("Failed to deserialize configuration")]
    DeserializationError(#[from] serde_json::Error)
}

pub struct SiteConfigurationLoader {
    config: SiteConfiguration
}

impl SiteConfigurationLoader {
    pub fn new() -> Result<Self, SiteConfigurationLoaderError> {
        let config_path = std::env::var("SITE_CONFIG_PATH")
                .unwrap_or(DEFAULT_SITE_CONFGIURATION_PATH.into());

        // Load the client strategy set from the file
        let contents = std::fs::read_to_string(std::path::PathBuf::from(config_path))
            .map_err(|e| {
                error!("Error reading contents of site configuration file {}", e.to_string());
                SiteConfigurationLoaderError::LoadingError(format!("Failed to read config file: {}", e.to_string()))
            })?;
        
        let config = serde_json::from_str::<SiteConfiguration>(&contents)
            .map_err(|e| {
                error!("Failed to deserialize site configuration: {}", e.to_string());
                SiteConfigurationLoaderError::DeserializationError(e)
            })?;

        Ok(Self { config })
    }

    pub fn get_config(&self) -> SiteConfiguration {
        return self.config.clone()
    }
}