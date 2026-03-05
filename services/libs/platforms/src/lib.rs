use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};
use utoipa::ToSchema;

/// Represents a platform for which there are clients registered for one or more of
/// the following APIs: Models, Datasets, Inference, Training. The strum(serialize="") 
/// attribute corresponds to the desired "platform" path parameter passed to the 
/// `get_client` method of a registrar.
#[derive(Clone, Eq, Hash, PartialEq, Debug, ToSchema, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Platform {
    HuggingFace,
    Github,
    Git,
    Patra,
    TapisPods,
    TapisJobs,
    S3,
}

impl Platform {
    pub fn list_all() -> Vec<Self> {
        return vec![Self::HuggingFace, Self::Git, Self::Github, Self::Patra, Self::TapisPods]
    }
}