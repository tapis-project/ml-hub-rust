use crate::constants;
use crate::requests::{ListDatasetsQueryParameters, ListModelsQueryParameters};
use crate::utils::deserialize_response_body;
use async_trait;
use clients::{
    ClientError, ClientErrorScope, ClientJsonResponse, GetDatasetClient,
    GetModelClient, IngestDatasetClient, IngestModelClient, ListDatasetsClient,
    ListModelsClient, PublishDatasetClient, PublishModelClient, PublishModelMetadataClient
};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::{Client as ReqwestClient, StatusCode};
use serde_json::{Map, Value};
use shared::infra::fs::git::{
    SyncGitRepository, SyncGitRepositoryImpl, SyncLfsRepositoryParams,
};
use shared::presentation::http::v1::actix_web::helpers::param_to_string;
use shared::presentation::http::v1::dto::artifacts::PublishArtifactRequest;
use shared::presentation::http::v1::dto::headers::{AuthorizationHeaderError, Headers};
use shared::presentation::http::v1::dto::datasets::{
    GetDatasetRequest, IngestDatasetRequest, ListDatasetsRequest,
    PublishDatasetRequest
};
use shared::domain::entities::{
    artifact::Artifact,
    model_metadata::ModelMetadata
};
use shared::logging::SharedLogger;
use shared::presentation::http::v1::dto::models::{
    GetModelRequest, IngestModelRequest, ListModelsRequest,
};
use std::path::PathBuf;
use std::str::FromStr;
use std::process::Command;

struct HuggingFaceHeaders(Headers);

impl TryFrom<&HuggingFaceHeaders> for reqwest::header::HeaderMap {
    type Error = AuthorizationHeaderError;

    fn try_from(value: &HuggingFaceHeaders) -> Result<Self, Self::Error> {
        let mut header_map = HeaderMap::new();
        for (key, value) in value.0.into_inner().iter() {
            let header_name = HeaderName::try_from(key.as_str())
                .map_err(|err| AuthorizationHeaderError::HeaderNameError(err.to_string()))?;

            let header_value = HeaderValue::from_str(value.as_str())
                .map_err(|err| AuthorizationHeaderError::HeaderNameError(err.to_string()))?;
            
            header_map.insert(header_name, header_value);
        }
        Ok(header_map)
    }
}

#[derive(Debug)]
pub struct HuggingFaceClient {
    client: ReqwestClient,
    logger: SharedLogger,
}

#[async_trait::async_trait]
impl ListModelsClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_models(
        &self,
        request: &ListModelsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>
    {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10),
        };

        // Build the query parameters
        let query_params = Some(ListModelsQueryParameters {
            search: request.query.get("search").cloned(),
            author: request.query.get("author").cloned(),
            filter: request.query.get("filter").cloned(),
            sort: request.query.get("sort").cloned(),
            direction: request.query.get("direction").cloned(),
            limit: Some(limit.unwrap_or(10)),
            full: None,
            config: None,
        });

        // Construct the url for the request
        let url = Self::format_url("models");

        self.logger.debug(format!("Request url: {}", url).as_str());
        self.logger.debug(format!("Query Params: {:#?}", &query_params).as_str());

        // Make a GET request to Hugging Face to fetch the models
        let result = self.client.get(url).query(&query_params).send().await;

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response).await?;

                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            }

            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::Internal {
                    msg: err.to_string(),
                    scope: ClientErrorScope::Server,
                });
            }
        }
    }
}

#[async_trait::async_trait]
impl GetModelClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_model(
        &self,
        request: &GetModelRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>
    {
        if let Err(my_error) =
            request.headers.validate_authorization_header(None)
        {
            return Err(ClientError::Internal {
                msg: my_error.to_string(),
                scope: ClientErrorScope::Server,
            });
        }
        let fail_message = String::from_str("failed to convert to header map")
            .expect("won't fail");
        let headers = match HeaderMap::try_from(&HuggingFaceHeaders(request.headers.clone())) {
            Ok(header_map) => header_map,
            Err(_) => {
                return Err(ClientError::Internal {
                    msg: fail_message,
                    scope: ClientErrorScope::Server,
                })
            }
        };

        let result = self
            .client
            .get(Self::format_url(
                format!("{}/{}", "models", request.path.model_id).as_str(),
            ))
            .headers(headers)
            .send()
            .await;

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response).await?;

                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            }
            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::Internal {
                    msg: err.to_string(),
                    scope: ClientErrorScope::Server,
                });
            }
        }
    }
}

#[async_trait::async_trait]
impl IngestModelClient for HuggingFaceClient {
    async fn ingest_model(
        &self,
        request: &IngestModelRequest,
        target_path: PathBuf,
    ) -> Result<(), ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authroization");

        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::Internal {
                msg: err.to_string(),
                scope: ClientErrorScope::Server,
            })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url: String::from(constants::HUGGING_FACE_BASE_URL),
            target_dir: target_path.to_string_lossy().to_string(),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone(),
        })
        .map_err(|err| ClientError::Internal {
            msg: err.to_string(),
            scope: ClientErrorScope::Server,
        })?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ListDatasetsClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_datasets(
        &self,
        request: &ListDatasetsRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>
    {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10),
        };

        // Build the query parameters
        let query_params = Some(ListDatasetsQueryParameters {
            search: request.query.get("search").cloned(),
            author: request.query.get("author").cloned(),
            filter: request.query.get("filter").cloned(),
            sort: request.query.get("sort").cloned(),
            direction: request.query.get("direction").cloned(),
            limit: Some(limit.unwrap_or(10)),
            full: None,
        });

        // Make a GET request to Hugging Face to fetch the datasets
        let result = self
            .client
            .get(Self::format_url("datasets"))
            .query(&query_params)
            .send()
            .await;

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response).await?;

                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            }

            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(
                    ClientError::Internal {
                        msg: format!("Error attempting request from HuggingFace Models API: {}", 
                        err.to_string()), scope: ClientErrorScope::Server
                    });
            }
        }
    }
}

#[async_trait::async_trait]
impl GetDatasetClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn get_dataset(
        &self,
        request: &GetDatasetRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>
    {
        let result = self
            .client
            .get(Self::format_url(
                format!("{}/{}", "datasets", request.path.dataset_id).as_str(),
            ))
            .send()
            .await;

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response).await?;

                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            }
            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(
                    ClientError::Internal {
                        msg: format!("Error attempting request from HuggingFace datasets API: {}", 
                        err.to_string()), scope: ClientErrorScope::Server
                    });
            }
        }
    }
}

#[async_trait::async_trait]
impl IngestDatasetClient for HuggingFaceClient {
    async fn ingest_dataset(
        &self,
        request: &IngestDatasetRequest,
        target_path: PathBuf,
    ) -> Result<(), ClientError> {
        // Get the authorization token from the request
        println!("{:#?}", request.headers);
        let access_token = request.headers.get_first_value("Authorization");

        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::BadRequest {
                msg: format!("Bad request: {}", err.to_string()),
                scope: ClientErrorScope::Client,
            })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url: String::from(constants::HUGGING_FACE_BASE_URL),
            target_dir: target_path.to_string_lossy().to_string(),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone(),
        })?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl PublishModelClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_model(&self, extracted_artifact_path: &PathBuf, _artifact: &Artifact, metadata: &ModelMetadata, request: &PublishArtifactRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        // Get the repo/model name from the metadata
        let model_name = match metadata.name.clone() {
            Some(n) => n,
            None => return Err(ClientError::BadRequest { msg: "Model metadata must contain a name in order to publish to huggingface".into(), scope: ClientErrorScope::Client })
        };

        // Get the access token from the headers
        let access_token = match request.headers.get_first_value("Authroization") {
            Some(t) => t,
            None => return Err(ClientError::BadRequest { msg: "Missing Authorization header".into(), scope: ClientErrorScope::Client })
        };
        
        // Check that the repo on huggingface exists
        let base_url = Self::format_url("repos");
        let maybe_response = self.client.get(format!("{}/{}", &base_url, &model_name))
            .header("Authorization", format!("Bearer {}", &access_token))
            .send()
            .await;

        let response = match maybe_response {
            Ok(r) => r,
            Err(err) => {
                return Err(ClientError::Internal { msg: err.to_string(), scope: ClientErrorScope::Client })
            }
        };

        // Return an error if the repo doesn't exist or there is some remote
        // internal error
        match response.status() {
            StatusCode::NOT_FOUND => return Err(ClientError::NotFound { msg: format!("Repo for user/model '{}' does not exist. Repo must exist before attempting to publish to it.", &model_name), scope: ClientErrorScope::Client }),
            StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::SERVICE_UNAVAILABLE => return Err(ClientError::Internal { msg: format!("Internal error with remote server when attempting to very if repo already exists for model {}", &model_name), scope: ClientErrorScope::Server }),
            _ => {}
        };
        
        // Remove the existing .git directory
        if extracted_artifact_path.join(".git").is_dir() {
            std::fs::remove_dir_all(extracted_artifact_path.join(".git"))
                .map_err(|err| ClientError::Internal { msg: format!("Error removing .git directory: {}", err.to_string()), scope: ClientErrorScope::Client })?;
        }

        // Construct remote name
        let origin = PathBuf::new()
            .join(constants::HUGGING_FACE_BASE_URL)
            .join(&model_name)
            .join(".git")
            .to_string_lossy()
            .to_string();
        
        // Initialize git repo, add all changes, commit, then add remote
        let init_output = Command::new("sh")
            .current_dir(&extracted_artifact_path)
            .arg("-c")
            .arg(format!("set -e; git init && git add -A && git commit -m \"MLHub HuggingFace Client: initial commit\" && git remote add origin {}", &origin))
            .output()
            .map_err(|err| ClientError::Internal { msg: format!("{}", err.to_string()), scope: ClientErrorScope::Client })?;
        
        // Check that the operation was successful
        match init_output.status.code() {
            Some(code) => {
                if code != 0 {
                    return Err(
                        ClientError::Internal {
                            msg: String::from_utf8(init_output.stderr)
                                .unwrap_or("git init operation failed. Additionally, stderr from the git rev-parse process could not be decoded".into()),
                            scope: ClientErrorScope::Client }
                    )
                    
                }
            },
            None => {
                return Err(ClientError::Internal { msg: "The git init operation was terminated by an unknown signal".into(), scope: ClientErrorScope::Client })
            } 
        };

        // Get the current branch
        let mut cmd = Command::new("git");

        let branch_name_output = cmd.current_dir(&extracted_artifact_path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .map_err(|err| ClientError::Internal { msg: format!("Failed to get branch name: {}", err.to_string()), scope: ClientErrorScope::Client })?;
        
        // Check that the branch name was output successfully
        let branch_name = match branch_name_output.status.code() {
            Some(code) => {
                if code != 0 {
                    return Err(
                        ClientError::Internal {
                            msg: String::from_utf8(branch_name_output.stderr)
                                .unwrap_or("git rev-parse operation failed. Additionally, stderr from the git rev-parse process could not be decoded".into()),
                            scope: ClientErrorScope::Client }
                    )
                    
                }
                
                String::from_utf8(branch_name_output.stdout)
                    .map_err(|err| ClientError::Internal { msg: format!("Failed to decode stdout of command `git rev-parse ...`: {}", err.to_string()), scope: ClientErrorScope::Client })?
            },
            None => {
                return Err(ClientError::Internal { msg: "The git rev-parse operation was terminated by an unknown signal".into(), scope: ClientErrorScope::Client })
            } 
        };

        // Start the git push command
        let mut cmd = Command::new("git");

        // Extend the headers on the push command with the provided access token
        // and push to the branch according
        let push_output = cmd.current_dir(&extracted_artifact_path)
            .arg("-c")
            .arg(format!("http.extraHeader=\"Authorization: Bearer {}\"", access_token))
            .arg("push")
            .arg("origin")
            .arg(&branch_name)
            .output()
            .map_err(|err| ClientError::Internal { msg: format!("Failed to push artifact: {}", err.to_string()), scope: ClientErrorScope::Client })?;
        
        // TODO check metadata for a version number. If provided, create a tag
        // and push it up

        // Check that the push was successful
        match push_output.status.code() {
            Some(code) => {
                if code != 0 {
                    return Err(
                        ClientError::Internal {
                            msg: String::from_utf8(branch_name_output.stderr)
                                    .unwrap_or(format!("`git push origin {}`  operation failed. Additionally, stderr from the git rev-parse process could not be decoded", &branch_name)),
                            scope: ClientErrorScope::Client }
                    )
                    
                }
                
                String::from_utf8(push_output.stdout)
                    .map_err(|err| ClientError::Internal { msg: format!("Failed to decode stdout of command `git push ...`: {}", err.to_string()), scope: ClientErrorScope::Client })?
            },
            None => {
                return Err(ClientError::Internal { msg: "The git push operation was terminated by an unknown signal".into(), scope: ClientErrorScope::Client })
            } 
        };

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

#[async_trait::async_trait]
impl PublishModelMetadataClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_model_metadata(&self, _metadata: &ModelMetadata, _result: &PublishArtifactRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        println!("Huggingface metadata client");
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

#[async_trait::async_trait]
impl PublishDatasetClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_dataset(
        &self,
        _result: &PublishDatasetRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        Err(ClientError::Unimplemented)
    }
}

impl SyncGitRepository for HuggingFaceClient {}

impl HuggingFaceClient {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
            logger: SharedLogger::new(),
        }
    }

    fn format_url(url: &str) -> String {
        format!(
            "{}/api/{}",
            constants::HUGGING_FACE_BASE_URL,
            url.strip_prefix("/").unwrap_or(url).to_string()
        )
    }
}
