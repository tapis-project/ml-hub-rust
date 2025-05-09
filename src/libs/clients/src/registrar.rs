use crate::errors::ClientProviderError;
use crate::platform::Platform;
use std::str::FromStr;
use huggingface_client::client::HuggingFaceClient;
use github_lfs_client::client::GithubLfsClient;
use git_lfs_client::client::GitLfsClient;
use patra_client::client::PatraClient;
use crate::clients::{
    ListModelsClient,
    GetModelClient,
    DiscoverModelsClient,
    DownloadModelClient,
    PublishModelClient
};

/// A provider for managing clients mapped to their respective platforms.
///
/// This struct maintains a registry of model and clients, allowing retrieval 
/// of the appropriate client based on the specified `Platform`.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use clients::providers::{ModelsClientProvider, Platform};
/// use shared::clients::ModelsClient
///
/// let provider = ModelsClientProvider::new();
///
/// // Assume `MyModelsClient` implements `ModelsClient`
/// // Get the HuggingFace models client from the ModelsClientProvider
/// let client = provider.get_client(Platform::HuggingFace);
/// ```
///
/// This struct is used for managing multiple model clients under a unified interface.
pub struct ClientProvider {}

/// Inherent implementation of ModelsClientProvider
impl ClientProvider {
    pub fn provide_list_models_client(platform_name: &str) -> Result<ListModelsClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(ListModelsClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(ListModelsClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn provide_get_model_client(platform_name: &str) -> Result<GetModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(GetModelClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(GetModelClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn provide_discover_models_client(platform_name: &str) -> Result<DiscoverModelsClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Patra => Ok(DiscoverModelsClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn provide_download_models_client(platform_name: &str) -> Result<DownloadModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Git => Ok(DownloadModelClient::Git(GitLfsClient::new())),
            Platform::Github => Ok(DownloadModelClient::Github(GithubLfsClient::new())),
            Platform::HuggingFace => Ok(DownloadModelClient::HuggingFace(HuggingFaceClient::new())),
            _ => Err(ClientProviderError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn provide_publish_model_client(platform_name: &str) -> Result<PublishModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            _ => Err(ClientProviderError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }
}

fn resolve_platform(platform_name: &str) -> Result<Platform, ClientProviderError> {
    Platform::from_str(platform_name)
        .map_err(|err| ClientProviderError::new(err.to_string()))
        .map(|p| p)
}