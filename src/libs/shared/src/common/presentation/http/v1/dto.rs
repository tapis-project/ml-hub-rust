use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use strum_macros::{Display, EnumString};
use thiserror::Error;
// Reexport to create a unified api for all artifact-related functionality
pub use crate::common::presentation::http::v1::responses::artifact_helpers;

// TODO Refactor Multipart into actix_web and create an adapter that prevents
// consumer from needing to know about this framework-speicifc implemntation
use actix_multipart::Multipart;

pub type Header = (String, String);

pub type Boundry = String;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Headers(Vec<Header>);

impl Headers {
    pub fn new(headers: Vec<Header>) -> Self {
        Self(headers)
    }
    // By the http standard, some headers can be set more than once, so
    // we return an optional vector of strings
    pub fn get_all_values(&self, name: &str) -> Option<Vec<String>> {
        let header_values = self
            .0
            .iter()
            .filter(|(k, _)| k == &name)
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();

        if header_values.len() < 1 {
            return None;
        }

        Some(header_values)
    }

    pub fn get_first_value(&self, name: &str) -> Option<String> {
        let values = self.get_all_values(name);
        if let Some(v) = values {
            return Some(v[0].clone());
        };

        return None;
    }

    pub fn validate_authorization_header(
        &self,
        auth_header_prefix: Option<&str>,
    ) -> Result<(), AuthorizationHeaderError> {
        // get authorization header if there isn't 1 return
        let header_value = match self.get_first_value(AUTHORIZATION.as_str()) {
            Some(value) => value,
            None => {
                return Ok(());
            }
        };

        // use auth_header_prefix if one is given else use 'Bearer '
        let prefix = match auth_header_prefix {
            Some(value) => value,
            None => "Bearer ",
        };

        // returns PrefixNotFound error if prefix string not found
        let header_index = match header_value.find(prefix) {
            Some(index) => index,
            None => {
                return Err(AuthorizationHeaderError::PrefixNotFound);
            }
        };

        // prefix should be at index 0 malformed error
        if header_index != 0 {
            return Err(AuthorizationHeaderError::MalformedHeader(header_value));
        }

        if prefix.len() == header_value.len() {
            return Err(AuthorizationHeaderError::NoValue);
        }

        return Ok(());
    }
}

impl TryFrom<&Headers> for reqwest::header::HeaderMap {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: &Headers) -> Result<Self, Self::Error> {
        let mut header_map = HeaderMap::new();
        for (key, value) in value.0.iter() {
            let header_name = HeaderName::try_from(key.as_str())?;
            let header_value = HeaderValue::from_str(value.as_str())?;
            header_map.insert(header_name, header_value);
        }
        Ok(header_map)
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationHeaderError {
    #[error("malformed header: {0}")]
    MalformedHeader(String),

    #[error("no header value was provided")]
    NoValue,

    #[error("provided prefix not found")]
    PrefixNotFound,
}

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

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct StagedArtifact {
    pub path: PathBuf,
    pub artifact: Artifact,
}

pub struct ArtifactStagingParams<'a> {
    pub artifact: &'a Artifact,
    pub staged_filename: Option<String>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>,
}

pub struct MultipartStagingParams<'payload> {
    pub payload: &'payload mut Multipart,
    pub staged_filename: String,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>,
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
    Pattern,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Filter {
    pub field: String,
    pub operation: FilterOperation,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListAll {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub fields: Option<Vec<String>>,
    pub filters: Option<Vec<Filter>>,
    pub sort_by: Option<String>,
    pub order_by: Option<Order>,
}
