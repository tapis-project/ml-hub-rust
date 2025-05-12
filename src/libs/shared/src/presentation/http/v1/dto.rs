use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use strum_macros::{EnumString, Display};
use actix_multipart::Multipart;
use serde_json::Value;

// Reexport to create a unified api for all artifact-related functionality
pub use crate::presentation::http::v1::responses::artifact_helpers;
pub use actix_web::HttpRequest;

pub type Parameters = std::collections::hash_map::HashMap<String, Value>;

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadArtifactBody {
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>,
    pub download_filename: Option<String>,
    pub params: Option<Parameters>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    #[strum(serialize="zip")]
    Zip
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    #[strum(serialize="deflated")]
    Deflated
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct StagedArtifact {
    pub path: PathBuf,
    pub artifact: Artifact
}

pub struct ArtifactStagingParams<'a> {
    pub artifact: &'a Artifact,
    pub staged_filename: Option<String>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>
}

pub struct MultipartStagingParams<'payload> {
    pub payload: &'payload mut Multipart,
    pub staged_filename: String,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum FilterOperation {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    In,
    Nin,
    Pattern
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Filter {
    pub field: String,
    pub operation: FilterOperation,
    pub value: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Order {
    Asc,
    Desc
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListAll {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub fields: Option<Vec<String>>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub order_by: Option<Order>
}