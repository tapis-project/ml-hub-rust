use crate::utils::deserialize_response_body;
use async_trait;
use clients::{
    Capability, Client, ClientError, ClientErrorScope, ClientJsonResponse, DiscoverModelsClient, GetModelClient, ListModelsClient, PublishModelMetadataClient
};
use reqwest::blocking::Client as ReqwestClient;
use serde_json::Value;
use shared::logging::SharedLogger;
use shared::presentation::http::v1::requests::models::{
    DiscoverModelsRequest, GetModelRequest, ListModelsRequest,
};
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::domain::entities:: model_metadata::ModelMetadata;
use std::collections::hash_map::HashMap;
use platforms::Platform;

#[derive(Debug)]
pub struct PatraClient {
    client: ReqwestClient,
    logger: SharedLogger,
}

#[async_trait::async_trait]
impl Client for PatraClient {
    fn platform(&self) -> Option<Platform> {
        Some(Platform::Patra)
    }
    
    fn capabilities(&self) -> Option<Vec<Capability>> {
        Some(vec![
            Capability::ListModels,
            Capability::GetModel,
            Capability::DiscoverModels
        ])
    }
}

#[async_trait::async_trait]
impl ListModelsClient for PatraClient {
    type Data = Value;
    type Metadata = Value;
    async fn list_models(
        &self,
        _request: &ListModelsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("List models");
        let resp = self
            .client
            .get(PatraClient::LIST_MODELS_ENDPOINT)
            .send()
            .map_err(|err| {
                let msg = err.to_string();
                if err.is_body() {
                    ClientError::BadRequest {
                        msg,
                        scope: ClientErrorScope::Client,
                    }
                } else if err.is_connect() {
                    ClientError::Unavailable(err.to_string())
                } else {
                    ClientError::Internal {
                        msg: "An unknown error occurred".into(),
                        scope: ClientErrorScope::Client,
                    }
                }
            })?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None,
        ));
    }
}

#[async_trait::async_trait]
impl GetModelClient for PatraClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_model(
        &self,
        request: &GetModelRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("Get model");

        let mut query_params = HashMap::new();
        query_params.insert("id", request.path.model_id.clone());
        let resp = self
            .client
            .get(PatraClient::GET_MODEL_ENDPOINT)
            .query(&query_params)
            .send()
            .map_err(|err| {
                let msg = err.to_string();
                if err.is_body() {
                    ClientError::BadRequest {
                        msg,
                        scope: ClientErrorScope::Client,
                    }
                } else if err.is_connect() {
                    ClientError::Unavailable(err.to_string())
                } else {
                    ClientError::Internal {
                        msg: "An unknown error occurred".into(),
                        scope: ClientErrorScope::Client,
                    }
                }
            })?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None,
        ));
    }
}

#[async_trait::async_trait]
impl DiscoverModelsClient for PatraClient {
    type Data = Value;
    type Metadata = Value;

    async fn discover_models(
        &self,
        request: &DiscoverModelsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("Discover models");
        let mut query_params = HashMap::new();

        let prompt = match request.body.prompt.clone() {
            Some(p) => p,
            None => return Err(ClientError::BadRequest { msg: "Missing field 'prompt': Model discovery on Patra requires a natural language prompt support via the 'prompt' field of the DiscoverModelsRequest".into(), scope: ClientErrorScope::Client })
        };
        
        if request.body.criteria.len() > 0 {
            query_params.insert("q", prompt);
        };

        let resp = self
            .client
            .get(PatraClient::SEARCH_MODEL_ENDPOINT)
            .query(&query_params)
            .send()
            .map_err(|err| {
                let msg = err.to_string();
                if err.is_body() {
                    ClientError::BadRequest {
                        msg,
                        scope: ClientErrorScope::Client,
                    }
                } else if err.is_connect() {
                    ClientError::Unavailable(err.to_string())
                } else {
                    ClientError::Internal {
                        msg: "An unknown error occurred".into(),
                        scope: ClientErrorScope::Client,
                    }
                }
            })?;

        let status_code = resp.status().as_u16();

        let deserialized_resp = deserialize_response_body(resp)?;

        return Ok(ClientJsonResponse::new(
            Some(status_code),
            Some(String::from("success")),
            Some(deserialized_resp),
            None,
        ));
    }
}

#[async_trait::async_trait]
impl PublishModelMetadataClient for PatraClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_model_metadata(
        &self,
        _metadata: &ModelMetadata,
        _request: &PublishArtifactServiceRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Ok(
            ClientJsonResponse::new(
                None,
                None,
                None,
                None
            )
        )
    }
}

impl PatraClient {
    const LIST_MODELS_ENDPOINT: &str = "https://patraserver.pods.icicleai.tapis.io/list";
    const GET_MODEL_ENDPOINT: &str = "https://patraserver.pods.icicleai.tapis.io/download_mc";
    const SEARCH_MODEL_ENDPOINT: &str = "https://patraserver.pods.icicleai.tapis.io/search";

    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            logger: SharedLogger::new(),
        }
    }
}
