use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListModelsQueryParameters {
    pub search: Option<String>,
    pub author: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<u64>,
    pub full: Option<bool>,
    pub config: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsRequest {
    pub query_params: Option<ListModelsQueryParameters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetModelRequest {
    pub model_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListDatasetsQueryParameters {
    pub search: Option<String>,
    pub author: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<u64>,
    pub full: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDatasetsRequest {
    pub query_params: Option<ListDatasetsQueryParameters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDatasetRequest {
    pub dataset_id: String,
}
