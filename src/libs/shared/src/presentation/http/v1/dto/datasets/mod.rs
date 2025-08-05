use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_multipart::Multipart;
use bytes::Bytes;
use crate::presentation::http::v1::dto::{
    Headers,
    DownloadArtifactBody,
    IngestArtifactBody
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListDatasetsPath {
    pub platform: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetDatasetPath {
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

pub struct ListDatasetsRequest {
    pub headers: Headers,
    pub path: ListDatasetsPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct GetDatasetRequest {
    pub headers: Headers,
    pub path: GetDatasetPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct IngestDatasetRequest {
    pub headers: Headers,
    pub path: IngestDatasetPath,
    pub query: HashMap<String, String>,
    pub body: IngestArtifactBody,
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