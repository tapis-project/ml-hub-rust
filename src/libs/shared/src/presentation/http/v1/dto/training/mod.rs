use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bytes::Bytes;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTrainingServerPath {
    pub platform: String,
    pub training_id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StartTrainingPath {
    pub platform: String,
    pub training_id: String
}

pub struct CreateTrainingServerRequest {
    pub path: CreateTrainingServerPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}

pub struct StartTrainingRequest {
    pub path: StartTrainingPath,
    pub query: HashMap<String, String>,
    pub body: Bytes,
}