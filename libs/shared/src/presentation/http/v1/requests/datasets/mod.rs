use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_multipart::Multipart;
use bytes::Bytes;
use crate::presentation::http::v1::requests::headers::Headers;
use crate::presentation::http::v1::requests::artifacts::{
    DownloadArtifactBody,
    IngestArtifactRequest
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListDatasetsByPlatformPath {
    pub platform: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDatasetByPlatformPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IngestDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

pub struct ListDatasetsByPlatformRequest {
    pub headers: Headers,
    pub path: ListDatasetsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct GetDatasetByPlatformRequest {
    pub headers: Headers,
    pub path: GetDatasetByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct IngestDatasetRequest {
    pub headers: Headers,
    pub path: IngestDatasetPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactRequest,
}

pub struct DownloadDatasetRequest {
    pub headers: Headers,
    pub path: DownloadDatasetPath,
    pub query: HashMap<String, String>,
    pub body: DownloadArtifactBody,
}

pub struct PublishDatasetRequest {
    pub headers: Headers,
    pub path: PublishDatasetPath,
    pub query: HashMap<String, String>,
    pub payload: Multipart,
}