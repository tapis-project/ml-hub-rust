use crate::model_metadata::PatraModelMetadata;
use crate::utils::deserialize_response_body;
use async_trait;
use clients::{
    Capability, Client, ClientError, ClientErrorScope, ClientJsonResponse, DiscoverModelsClient,
    GetModelClient, ListModelsClient, ModelMetadataConversionClient, PublishModelMetadataClient,
};
use platforms::Platform;
use reqwest::blocking::Client as ReqwestClient;
use serde_json::Value;
use shared::domain::entities;
use shared::domain::entities::model_metadata::ModelMetadata;
use shared::logging::SharedLogger;
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::presentation::http::v1::requests::discover_models::DiscoverModelsByPlatformRequest;
use shared::presentation::http::v1::requests::{
    get_model_by_platform::GetModelByPlatformRequest,
    list_models_by_platform::ListModelsByPlatformRequest,
};
use std::collections::hash_map::HashMap;

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
            Capability::DiscoverModels,
            Capability::ConvertModelMetadata,
        ])
    }
}

#[async_trait::async_trait]
impl ListModelsClient for PatraClient {
    type Data = Value;
    type Metadata = Value;
    async fn list_models(
        &self,
        _request: &ListModelsByPlatformRequest,
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
        request: &GetModelByPlatformRequest,
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
        request: &DiscoverModelsByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        self.logger.debug("Discover models");
        let mut query_params = HashMap::new();

        let prompt = match request.body.prompt.clone() {
            Some(p) => p,
            None => return Err(ClientError::BadRequest { msg: "Missing field 'prompt': Model discovery with Patra requires a natural language prompt support via the 'prompt' field of the DiscoverModelsRequest".into(), scope: ClientErrorScope::Client })
        };

        query_params.insert("q", prompt);

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
        return Ok(ClientJsonResponse::new(None, None, None, None));
    }
}

impl ModelMetadataConversionClient for PatraClient {
    fn from_platform_metadata<T>(
        &self,
        client_metadata: T,
        author: String,
        tenant_id: String,
    ) -> Result<ModelMetadata, ClientError>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_value(client_metadata).map_err(|err| ClientError::Internal {
            msg: format!(
                "Failed to convert serializable client metadata into Value: {}",
                err
            ),
            scope: ClientErrorScope::Server,
        })?;

        let patra_model =
            serde_json::from_value::<PatraModelMetadata>(value.clone()).map_err(|err| {
                ClientError::Internal {
                    msg: format!("Failed to convert Patra model metadata: {}", err),
                    scope: ClientErrorScope::Server,
                }
            })?;

        let tags = patra_model.parse_tags();

        let raw_libraries = patra_model.parse_libs();
        let known_libraries = &[
            "transformers".into(),
            "diffusers".into(),
            "tensorflow".into(),
            "pytorch".into(),
        ];
        let libraries: Vec<String> = known_libraries
            .iter()
            .filter(|lib| raw_libraries.contains(*lib))
            .cloned()
            .collect();

        let model_inputs = patra_model.input_type.clone().map(|input_type| {
            vec![entities::model_metadata::ModelIO {
                data_type: Some(input_type),
                shape: None,
            }]
        });

        Ok(entities::model_metadata::ModelMetadata {
            name: patra_model.model_name(),
            artifact_id: None,
            annotations: Some(value),
            description: patra_model.description(),
            author,
            tenant_id,
            canonical: Some(entities::model_metadata::Canonical {
                platform: Platform::Patra,
                author: patra_model.author.clone(),
                model_id: patra_model.uuid.clone(),
                downloads: None,
                locator: entities::model_metadata::Locator {
                    url: patra_model.locator_url(),
                },
                likes: None,
                gated: patra_model.is_gated,
                private: patra_model.is_private,
                sha: None,
            }),
            model_inputs,
            model_outputs: None,
            model_type: patra_model.model_type(),
            libraries: if libraries.is_empty() {
                None
            } else {
                Some(libraries)
            },
            image: None,
            tags: if tags.is_empty() { None } else { Some(tags) },
            multi_modal: None,
            task_types: None,
            inference_distributed: None,
            inference_hardware: None,
            inference_max_compute_utilization_percentage: None,
            inference_max_energy_consumption_watts: None,
            inference_max_latency_ms: None,
            inference_max_memory_usage_mb: None,
            inference_min_throughput: None,
            inference_precision: None,
            inference_software_dependencies: None,
            training_distributed: None,
            training_hardware: None,
            training_max_energy_consumption_watts: None,
            training_precision: None,
            training_time: None,
            pretrained: None,
            pretraining_datasets: None,
            finetuning_datasets: None,
            edge_optimized: None,
            quantization_aware: None,
            supports_quantization: None,
            pruned: None,
            slimmed: None,
            regulatory: None,
            license: patra_model.license(),
            bias_evaluation_score: None,
            deployment_strategy_refs: vec![],
        })
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
