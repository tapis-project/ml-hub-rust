use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct QueryParameters {
    pub search: Option<String>,
    pub author: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<u128>,
    pub full: Option<String>,
    pub config: Option<String>,
}

#[derive(Debug)]
pub struct ListModelsRequest {
    pub query_params: Option<QueryParameters>,
}

#[derive(Debug)]
pub struct GetModelRequest {
    pub path: String,
    pub query_params: Option<QueryParameters>,
}

#[derive(Debug)]
pub struct ListDatasetsRequest {
    pub query_params: Option<QueryParameters>,
}

#[derive(Debug)]
pub struct GetDatasetRequest {
    pub path: String,
    pub query_params: Option<QueryParameters>,
}
