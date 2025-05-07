use std::collections::HashMap;
use actix_web::web;
use serde::{Deserialize, Serialize};
use actix_web::HttpRequest;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTrainingPath {
    pub platform: String,
    pub training_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StartTrainingPath {
    pub platform: String,
    pub training_id: String
}

pub struct CreateTrainingRequest {
    pub req: HttpRequest,
    pub path: web::Path<CreateTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}

pub struct StartTrainingRequest {
    pub req: HttpRequest,
    pub path: web::Path<StartTrainingPath>,
    pub query: web::Query<HashMap<String, String>>,
    pub body: web::Bytes,
}