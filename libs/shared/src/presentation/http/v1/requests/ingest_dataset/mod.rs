pub mod path;

use std::collections::HashMap;

use crate::presentation::http::v1::requests::{
    artifacts::IngestArtifactRequest, common::headers::Headers,
};

pub struct IngestDatasetRequest {
    pub headers: Headers,
    pub path: path::IngestDatasetPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactRequest,
}
