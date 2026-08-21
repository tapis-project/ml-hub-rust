pub mod path;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::presentation::http::v1::requests::common::headers::Headers;
use crate::presentation::http::v1::requests::artifacts::IngestArtifactRequest;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IngestCanonicalModelRequest {
    pub headers: Headers,
    pub path: path::IngestCanonicalModelPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactRequest,
}