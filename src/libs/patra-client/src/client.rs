use crate::utils::deserialize_response_body;
use std::collections::hash_map::HashMap;
use reqwest::blocking::Client as ReqwestClient;
use shared::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::requests::{
    GetModelRequest,
    ListModelsRequest,
    DownloadModelRequest,
    ListDatasetsRequest,
    GetDatasetRequest,
    DownloadDatasetRequest,
    DiscoverModelsRequest,
    PublishDatasetRequest,
    PublishModelRequest,
};
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct PatraClient {
    client: ReqwestClient,
    logger: SharedLogger
}

impl ArtifactGenerator for PatraClient {}

impl ModelsClient for PatraClient {
    fn list_models(&self, _request: &ListModelsRequest) -> Result<ClientJsonResponse, ClientError> {
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
    
    fn get_model(&self, request: &GetModelRequest) -> Result<ClientJsonResponse, ClientError> {
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

    fn download_model(&self, _request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn discover_models(&self, request: &DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError> {
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

    async fn publish_model(&self, _request: &PublishModelRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::from_str(""))
    }
}

impl DatasetsClient for PatraClient {
    fn list_datasets(&self, _request: &ListDatasetsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn get_dataset(&self, _request: &GetDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn download_dataset(&self, _request: &DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn publish_dataset(&self, _request: &PublishDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
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
