use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_multipart::Multipart;
use actix_web::web;
use crate::presentation::http::v1::dto::DownloadArtifactBody;
// Re-export so clients can use this struct
pub use actix_web::HttpRequest;

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
    pub req: HttpRequest,
    pub path: web::Path<ListDatasetsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct GetDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<GetDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct DownloadDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<DownloadDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Json<DownloadArtifactBody>,
}

pub struct PublishDatasetRequest {
    pub req: HttpRequest,
    pub path: web::Path<PublishDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub payload: Multipart,
}