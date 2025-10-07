use std::str::FromStr;
use std::collections::HashMap;
use platforms::Platform;
use huggingface_client::client::HuggingFaceClient;
use github_lfs_client::client::GithubLfsClient;
use git_lfs_client::client::GitLfsClient;
use patra_client::client::PatraClient;
use crate::errors::ClientProviderError;
use crate::clients::{
    ListModelsClient,
    GetModelClient,
    DiscoverModelsClient,
    PublishModelClient,
    IngestModelClient,
    IngestDatasetClient,
    PublishModelMetadataClient,
};
use clients::{Client, Capability};

/// A provider for managing clients mapped to their respective platforms.
///
/// This struct maintains a registry of model and clients, allowing retrieval 
/// of the appropriate client based on the specified `Platform`.
///
/// # Example
///
/// ```rust
/// use client_provider::ClientProvider;
/// use client_provider::Platform;
///
/// let client = ClientProvider::provide_list_models_client("huggingface");
/// ```
///
/// This struct is used for managing multiple model clients under a unified interface.
pub struct ClientProvider {}

/// Inherent implementation of ModelsClientProvider
impl ClientProvider {
    fn list_clients() -> Vec<Box<dyn Client>> {
        vec![
            Box::new(HuggingFaceClient::new()),
            Box::new(PatraClient::new()),
            Box::new(GitLfsClient::new()),
            Box::new(GithubLfsClient::new()),
        ]
    }

    pub fn get_platform_client_capabilities() -> HashMap<Platform, Vec<Capability>> {
        let mut platforms: HashMap<Platform, Vec<Capability>> = HashMap::new();
        for client in Self::list_clients() {
            if let Some((platform, capabilities)) = client.platform().zip(client.capabilities()) {
                platforms.insert(platform, capabilities);
            }
        }

        return platforms
    }

    pub fn provide_list_models_client(platform_name: &str) -> Result<ListModelsClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(ListModelsClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(ListModelsClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("listing")))
        }
    }

    pub fn provide_get_model_client(platform_name: &str) -> Result<GetModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(GetModelClient::HuggingFace(HuggingFaceClient::new())),
            Platform::Patra => Ok(GetModelClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("fetching")))
        }
    }

    pub fn provide_discover_models_client(platform_name: &str) -> Result<DiscoverModelsClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Patra => Ok(DiscoverModelsClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("discovery")))
        }
    }

    pub fn provide_ingest_model_client(platform_name: &str) -> Result<IngestModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Git => Ok(IngestModelClient::Git(GitLfsClient::new())),
            Platform::Github => Ok(IngestModelClient::Github(GithubLfsClient::new())),
            Platform::HuggingFace => Ok(IngestModelClient::HuggingFace(HuggingFaceClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("model ingesting")))
        }
    }

    pub fn provide_publish_model_client(platform_name: &str) -> Result<PublishModelClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::HuggingFace => Ok(PublishModelClient::HuggingFace(HuggingFaceClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("model publishing")))
        }
    }

    pub fn provide_publish_metadata_client(platform_name: &str) -> Result<PublishModelMetadataClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            Platform::Patra => Ok(PublishModelMetadataClient::Patra(PatraClient::new())),
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("model publishing")))
        }
    }

    pub fn provide_ingest_dataset_client(platform_name: &str) -> Result<IngestDatasetClient, ClientProviderError> {
        let platform = resolve_platform(platform_name)?;
        match platform {
            _ => Err(ClientProviderError::NotFound(String::from(platform_name), String::from("dataset ingesting")))
        }
    }
}

fn resolve_platform(platform_name: &str) -> Result<Platform, ClientProviderError> {
    Platform::from_str(platform_name)
        .map_err(|err| ClientProviderError::ParseError(err.to_string()))
        .map(|p| p)
}