mod dto_to_input;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use super::headers::Headers;
use crate::presentation::http::v1::requests::Parameters;

#[derive(Serialize, Deserialize)]
pub struct GetArtifactPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct IngestArtifactRequest {
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub webhook_url: Option<String>,
    #[schema(value_type = Object)]
    pub params: Option<Parameters>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadArtifactBody {
    pub download_filename: Option<String>,
    pub params: Option<Parameters>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListArtifactPublicationsPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListArtifactIngestionsPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct PublishArtifactRequest {
    pub target_platform: String,
    pub webhook_url: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactServiceRequest {
    pub headers: Headers,
    pub path: PublishArtifactPath,
    pub query: HashMap<String, String>,
    pub body: PublishArtifactRequest,
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteArtifactRequest{
    pub id: String,
    pub artifact_id: String,
    pub webhook_url: String,
}