mod dto_to_input;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::headers::Headers;
use crate::presentation::http::v1::dto::Parameters;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactPath {
    pub artifact_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishArtifactBody {
    pub platform: String,
}

pub struct PublishArtifactRequest {
    pub headers: Headers,
    pub path: PublishArtifactPath,
    pub query: HashMap<String, String>,
    pub body: PublishArtifactBody,
}

pub struct CreateArtifactPublication {
    pub platform: String,
    pub artifact_id: String,
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}