use strum_macros::{EnumString, Display};

/// Represents a platform for which there are clients registered for one or more of
/// the following APIs: Models, Datasets, Inference, Training. The strum(serialize="") 
/// attribute corresponds to the desired "platform" path parameter passed to the 
/// `get_client` method of a registrar.
#[derive(Eq, Hash, PartialEq, Debug, Display, EnumString)]
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
    Patra
}

impl Platform {
    pub fn list_all() -> Vec<Self> {
        return vec![Self::HuggingFace, Self::Git, Self::Github, Self::Patra]
    } 
}