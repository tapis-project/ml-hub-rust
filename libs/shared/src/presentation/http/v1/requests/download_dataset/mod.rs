pub mod path;

use std::collections::HashMap;

use crate::presentation::http::v1::requests::{
    artifacts::DownloadArtifactBody, common::headers::Headers,
};

pub struct DownloadDatasetRequest {
    pub headers: Headers,
    pub path: path::DownloadDatasetPath,
    pub query: HashMap<String, String>,
    pub body: DownloadArtifactBody,
}
