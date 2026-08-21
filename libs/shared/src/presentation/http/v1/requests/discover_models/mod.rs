pub mod mappings;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use crate::presentation::http::v1::requests::common::headers::Headers;
use crate::presentation::http::v1::requests::common::tasks::Task;

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscoverModelsByPlatformPath {
    pub platform: String
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct DiscoveryCriteria {
    // Used for model discovery clients support model discovery through natural
    // language search
    pub prompt: Option<String>,
    pub criteria: Vec<DiscoveryCriterion>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct DiscoverModelsQueryParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub include_count: Option<bool>,
    pub include_global_models: Option<bool>,
}

pub struct DiscoverModelsRequest {
    pub headers: Headers,
    pub query: HashMap<String, String>,
    pub body: DiscoveryCriteria
}

pub struct DiscoverModelsByPlatformRequest {
    pub headers: Headers,
    pub path: DiscoverModelsByPlatformPath,
    pub query: HashMap<String, String>,
    pub body: DiscoveryCriteria
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct DiscoveryCriterion {
    pub name: Option<String>,
    pub author: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
}