use crate::errors::RegistrarError;
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

/// A registrar for managing pre-registered model clients mapped to their respective
/// platforms.
///
/// This struct maintains a registry of model clients, allowing retrieval 
/// of the appropriate client based on the specified `Platform`.
///
/// # Fields
/// 
/// - `clients`: A `HashMap` storing model clients instances associated with a `Platform` key. 
///   The values are boxed trait objects implementing `ModelsClient`, enabling 
///   dynamic dispatch for different model client implementations.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use clients::registrars::{ModelsClientRegistrar, Platform};
/// use shared::clients::ModelsClient
///
/// let registrar = ModelsClientRegistrar::new();
///
/// // Assume `MyModelsClient` implements `ModelsClient`
/// // Get the HuggingFace models client from the ModelsClientRegistrar
/// let client = registrar.get_client(Platform::HuggingFace);
/// ```
///
/// This struct is used for managing multiple model clients under a unified interface.
pub struct ClientRegistrar {}

/// Inherent implementation of ModelsClientRegistrar
impl ClientRegistrar {
    pub fn resolve_list_models_client(platform_name: &str) -> Result<ListModelsClient, RegistrarError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(ListModelsClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(ListModelsClient::Patra(PatraClient::new())),
            _ => Err(RegistrarError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn resolve_get_model_client(platform_name: &str) -> Result<GetModelClient, RegistrarError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(GetModelClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(GetModelClient::Patra(PatraClient::new())),
            _ => Err(RegistrarError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn resolve_discover_models_client(platform_name: &str) -> Result<DiscoverModelsClient, RegistrarError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Patra => Ok(DiscoverModelsClient::Patra(PatraClient::new())),
            _ => Err(RegistrarError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn resolve_download_models_client(platform_name: &str) -> Result<DownloadModelClient, RegistrarError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Git => Ok(DownloadModelClient::Git(GitLfsClient::new())),
            Platform::Github => Ok(DownloadModelClient::Github(GithubLfsClient::new())),
            Platform::HuggingFace => Ok(DownloadModelClient::HuggingFace(HuggingFaceClient::new())),
            _ => Err(RegistrarError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }

    pub fn resolve_publish_model_client(platform_name: &str) -> Result<PublishModelClient, RegistrarError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            _ => Err(RegistrarError::new(format!("No client registered with name '{}' for list_models", platform_name)))
        }
    }
}

fn resolve_platform(platform_name: &str) -> Result<Platform, RegistrarError> {
    Platform::from_str(platform_name)
        .map_err(|err| RegistrarError::new(err.to_string()))
        .map(|p| p)
}