use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};
use super::headers::Headers;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestArtifactBody {
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub webhook_url: Option<String>,
    pub params: Option<Parameters>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadArtifactBody {
    pub download_filename: Option<String>,
    pub params: Option<Parameters>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    #[strum(serialize = "zip")]
    Zip,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    #[strum(serialize = "deflated")]
    Deflated,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactBody {
    pub platform: String,
    pub platform_artifact_id: String,
    pub artifact_metadata: Option<Value>
}

pub struct PublishArtifactRequest {
    pub headers: Headers,
    pub path: PublishArtifactPath,
    pub query: HashMap<String, String>,
    pub body: PublishArtifactBody,
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}