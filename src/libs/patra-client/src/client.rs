use crate::utils::deserialize_response_body;
use std::collections::hash_map::HashMap;
use reqwest::blocking::Client as ReqwestClient;
use shared::clients::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientError,
    ClientJsonResponse,
    DiscoverModelsClient,
    GetModelClient,
    ListModelsClient,
};
use shared::models::presentation::http::v1::dto::{
    DiscoverModelsRequest,
    GetModelRequest,
    ListModelsRequest,
};
use shared::logging::SharedLogger;
use serde_json::Value;

#[derive(Debug)]
pub struct PatraClient {
    client: ReqwestClient,
    logger: SharedLogger
}

impl ArtifactGenerator for PatraClient {}

impl ListModelsClient for PatraClient {
    type Data = Value;
    type Metadata = Value;
    fn list_models(&self, _request: &ListModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("List models");
        let resp = self.client.get(PatraClient::LIST_MODELS_ENDPOINT)
            .send()
            .map_err(|err| ClientError::from_str(err.to_string().as_str()))?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None
        ))
    }
}

impl GetModelClient for PatraClient {
    type Data = Value;
    type Metadata = Value;

    fn get_model(&self, request: &GetModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("Get model");

        let mut query_params = HashMap::new();
        query_params.insert("id", request.path.model_id.clone());
        let resp = self.client.get(PatraClient::GET_MODEL_ENDPOINT)
            .query(&query_params)
            .send()
            .map_err(|err| ClientError::from_str(err.to_string().as_str()))?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None
        ))
    }
}

impl DiscoverModelsClient for PatraClient {
    type Data = Value;
    type Metadata = Value;

    fn discover_models(&self, request: &DiscoverModelsRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("Discover models");
        let mut query_params = HashMap::new();
        if request.body.criteria.len() > 0 {
            // We are only taking the first critera because Patra has not implemented
            // a way to search on more than one criterion
            let name = request.body.criteria[0].name
                .clone()
                .unwrap_or(String::from(""));
            query_params.insert("q", name);
        }
        
        let resp = self.client.get(PatraClient::SEARCH_MODEL_ENDPOINT)
            .query(&query_params)
            .send()
            .map_err(|err| ClientError::from_str(err.to_string().as_str()))?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None
        ))
    }
}

impl PatraClient {
    const LIST_MODELS_ENDPOINT: &str = "https://ckn.d2i.tacc.cloud/patra/list";
    const GET_MODEL_ENDPOINT: &str = "https://ckn.d2i.tacc.cloud/patra/download_mc";
    const SEARCH_MODEL_ENDPOINT: &str = "https://ckn.d2i.tacc.cloud/patra/search";

    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            logger: SharedLogger::new(),
        }
    }
}
