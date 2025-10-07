use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};
use utoipa::ToSchema;

/// Represents a platform for which there are clients registered for one or more of
/// the following APIs: Models, Datasets, Inference, Training. The strum(serialize="") 
/// attribute corresponds to the desired "platform" path parameter passed to the 
/// `get_client` method of a registrar.
#[derive(Clone, Eq, Hash, PartialEq, Debug, Display, EnumString, ToSchema, Serialize, Deserialize)]
pub enum Platform {
    /// This variant corresponds to the Hugging Face API client.
    #[strum(serialize="huggingface")]
    #[schema(rename="huggingface")]
    HuggingFace,
    /// This variant corresponds to the Github LFS client.
    #[strum(serialize="github")]
    #[schema(rename="github")]
    Github,
    /// This variant corresponds to the Git LFS client.
    #[strum(serialize="git")]
    #[schema(rename="git")]
    Git,
    /// This variant corresponds to the Patra client
    #[strum(serialize="patra")]
    #[schema(rename="patra")]
    Patra,
    /// This variant corresponds to the TaccTapis client
    #[strum(serialize="tacc-tapis")]
    #[schema(rename="tacc-tapis")]
    TaccTapis,
    /// This variant corresponds to the S3 client
    #[strum(serialize="s3")]
    #[schema(rename="s3")]
    S3,
}

impl Platform {
    pub fn list_all() -> Vec<Self> {
        return vec![Self::HuggingFace, Self::Git, Self::Github, Self::Patra]
    }
}