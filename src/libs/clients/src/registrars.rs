use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FormatResult};
use strum_macros::{EnumString, Display};
use shared::clients::{
    ModelsClient,
    DatasetsClient,
};
use huggingface_client::client::HuggingFaceClient;
use github_lfs_client::client::GithubLfsClient;
use git_lfs_client::client::GitLfsClient;
use patra_client::client::PatraClient;

/// Represents an error that occurs within the registrar. This error is returned
/// from registrar method calls as the `Err` variant of the `Result` enum
/// 
/// # Fields
/// 
/// - `message`: A `String` of human readable error message
#[derive(Debug)]
pub struct RegistrarError {
    message: String
}

/// Inherent implementation for RegistrarError
impl RegistrarError {
    /// Creates a new `RegistrarError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string describing the error.
    ///
    /// # Returns
    ///
    /// A new instance of `RegistrarError`.
    pub fn new(message: String) -> Self {
        RegistrarError {
            message,
        }
    }
}

/// Implementation of std::fmt::Display for RegisrarError
impl Display for RegistrarError {
    /// Formats the error message for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to display the error message.
    ///
    /// # Returns
    ///
    /// A FormatResutl which == Result<(), std::fmt::Error>
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}

/// Represents a platform for which there are clients registered for one or more of
/// the following APIs: Models, Datasets, Inference, Training. The strum(serialize="") 
/// attribute corresponds to the desired "platform" path parameter passed to the 
/// `get_client` method of a registrar.
#[derive(Eq, Hash, PartialEq, Debug, Display, Serialize, Deserialize, EnumString)]
pub enum Platform {
    /// This variant corresponds to the Hugging Face API client.
    #[strum(serialize="huggingface")]
    HuggingFace,
    /// This variant corresponds to the Github LFS client.
    #[strum(serialize="github")]
    Github,
    /// This variant corresponds to the Git LFS client.
    #[strum(serialize="git")]
    Git,
    /// This variant corresponds to the Patra client
    #[strum(serialize="patra")]
    Patra,
    Default
}

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
pub struct ModelsClientRegistrar {
    pub clients: HashMap<Platform, Box<dyn ModelsClient>>,
}

/// Inherent implementation of ModelsClientRegistrar
impl ModelsClientRegistrar {
    /// Creates a new ModelsClientRegistrar with pre-instantiated implementations
    /// of `ModelsClient`s
    ///
    /// # Returns
    ///
    /// A new instance of ModelsClientRegistrar
    pub fn new() -> Self {
        // Initialize a mutable clients hashmap
        let mut clients: HashMap<Platform, Box<dyn ModelsClient>> = HashMap::new();

        // Add the initialized modles clients into the clients hashmap by the
        // specific enum variant designated for the platform
        clients.insert(Platform::HuggingFace, Box::new(HuggingFaceClient::new()));
        clients.insert(Platform::Github, Box::new(GithubLfsClient::new()));
        clients.insert(Platform::Git, Box::new(GitLfsClient::new()));
        clients.insert(Platform::Patra, Box::new(PatraClient::new()));
        clients.insert(Platform::Default, Box::new(GithubLfsClient::new()));

        return Self {
            clients
        }
    }

    pub fn get_client<'a>(&'a self, platform_name: &str) -> Result<&'a Box<dyn ModelsClient>, RegistrarError> {
        let platform = Platform::from_str(platform_name)
            .map_err(|err| RegistrarError::new(err.to_string()))
            .map(|p| p)?;

        self.clients.get(&platform)
            .ok_or(RegistrarError::new(String::from(format!("No models client registered for platform '{}'", platform_name))))
    }
}

/// A registrar for managing pre-registered dataset clients mapped to their respective
/// platforms.
///
/// This struct maintains a registry of dataset clients, allowing retrieval 
/// of the appropriate client based on the specified `Platform`.
///
/// # Fields
/// 
/// - `clients`: A `HashMap` storing dataset client instances associated with a 
///   `Platform` key. The values are boxed trait objects implementing `DatasetsClient`, 
///   enabling dynamic dispatch for different dataset client implementations.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use clients::registrars::{DatasetsClientRegistrar, Platform};
/// use shared::clients::DatasetsClient
///
/// let registrar = DatasetsClientRegistrar::new();
///
/// // Assume `MyDatasetsClient` implements `DatasetsClient`
/// // Get the HuggingFace datasets client from the DatasetsClientRegistrar
/// let client = registrar.get_client(Platform::HuggingFace);
/// ```
///
/// This struct is used for managing multiple dataset clients under a unified interface.
pub struct DatasetsClientRegistrar {
    clients: HashMap<Platform, Box<dyn DatasetsClient>>,
}

/// Inherent implementation of DatasestsClientRegistrar
impl DatasetsClientRegistrar {
    /// Creates a new DatasetsClientRegistrar with pre-instantiated implementations
    /// of `DatasetsClient`s
    ///
    /// # Returns
    ///
    /// A new instance of DatasetsClientRegistrar
    pub fn new() -> Self {
        // Initialize a mutable clients HashMap
        let mut clients: HashMap<Platform, Box<dyn DatasetsClient>> = HashMap::new();

        // Add the initialized datasets clients into the clients hashmap by the
        // specific enum variant designated for the platform.
        clients.insert(Platform::HuggingFace, Box::new(HuggingFaceClient::new()));
        clients.insert(Platform::Github, Box::new(GithubLfsClient::new()));
        clients.insert(Platform::Git, Box::new(GitLfsClient::new()));
        clients.insert(Platform::Default, Box::new(GithubLfsClient::new()));
        return Self {
            clients
        }
    }

    pub fn get_client<'a>(&'a self, platform_name: &str) -> Result<&'a Box<dyn DatasetsClient>, RegistrarError> {
        let platform = Platform::from_str(platform_name)
            .map_err(|err| RegistrarError::new(err.to_string()))
            .map(|p| p)?;

        self.clients.get(&platform)
            .ok_or(RegistrarError::new(String::from(format!("No datasets client registered for platform '{}'", platform_name))))
    }
}