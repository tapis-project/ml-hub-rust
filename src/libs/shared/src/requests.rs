use serde::Deserialize;
use std::collections::HashMap;
use actix_web::{web, HttpRequest as ActixHttpRequest};

#[derive(Deserialize, Debug)]
pub struct ListModelsPath {
    pub platform: String
}

#[derive(Deserialize, Debug)]
pub struct GetModelPath {
    pub platform: String,
    pub  model_id: String
}

#[derive(Deserialize, Debug)]
pub struct DownloadModelPath {
    pub platform: String,
    pub dmodel_id: String
}

#[derive(Deserialize, Debug)]
pub struct ListDatasetsPath {
    pub platform: String,
}

#[derive(Deserialize, Debug)]
pub struct GetDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Debug)]
pub struct DownloadDatasetPath {
    pub platform: String,
    pub dataset_id: String
}

#[derive(Deserialize, Debug)]
pub struct StartInferenceServerPath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Debug)]
pub struct RunInferencePath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Debug)]
pub struct CreateInferencePath {
    pub platform: String,
    pub inference_id: String
}

#[derive(Deserialize, Debug)]
pub struct CreateTrainingPath {
    pub platform: String,
    pub training_id: String
}

#[derive(Deserialize, Debug)]
pub struct StartTrainingPath {
    pub platform: String,
    pub training_id: String
}

pub struct ListModelsRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<ListModelsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct GetModelRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<GetModelPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct DownloadModelRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<DownloadModelPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct ListDatasetsRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<ListDatasetsPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct GetDatasetRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<GetDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct DownloadDatasetRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<DownloadDatasetPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct CreateInferenceRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<CreateInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartInferenceServerRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<StartInferenceServerPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct RunInferenceRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<RunInferencePath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct CreateTrainingRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<CreateTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartTrainingRequest {
    pub req: ActixHttpRequest,
    pub path: web::Path<StartTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}