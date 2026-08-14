use crate::constants;
use crate::requests::{ListDatasetsQueryParameters, ListModelsQueryParameters};
use crate::utils::build_client_response;
use crate::model_metadata::{HFModelMetadata, CompoundTag};
use async_trait;
use clients::{
    Capability,
    Client, 
    ClientError, 
    ClientErrorScope, 
    ClientJsonResponse, 
    GetDatasetClient, 
    GetModelClient, 
    IngestDatasetClient, 
    IngestModelClient, 
    ListDatasetsClient, 
    ListModelsClient,
    PublishDatasetClient, 
    PublishModelClient, 
    PublishModelMetadataClient,
    ModelMetadataConversionClient
};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::{Client as ReqwestClient, StatusCode};
use serde_json::Value;
use shared::domain::entities;
use shared::infra::fs::git::{
    SyncGitRepository, SyncGitRepositoryImpl, SyncLfsRepositoryParams,
};
use shared::presentation::http::v1::actix_web::helpers::param_to_string;
use shared::presentation::http::v1::requests::artifacts::PublishArtifactServiceRequest;
use shared::presentation::http::v1::requests::common::headers::{AuthorizationHeaderError, Headers};
use shared::presentation::http::v1::requests::datasets::{
    GetDatasetByPlatformRequest,
    IngestDatasetRequest,
    ListDatasetsByPlatformRequest,
    PublishDatasetRequest
};
use shared::domain::entities::{
    artifact::Artifact,
};
use shared::domain::entities::model_metadata::ModelMetadata;
use shared::logging::SharedLogger;
use shared::presentation::http::v1::requests::{
    get_model_by_platform::GetModelByPlatformRequest,
    ingest_model::IngestModelRequest,
    list_models_by_platform::ListModelsByPlatformRequest,
};
use std::path::PathBuf;
use std::process::Command;
use platforms::Platform;
use heck::ToPascalCase;

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
impl Client for HuggingFaceClient {
    fn platform(&self) -> Option<Platform> {
        Some(Platform::HuggingFace)
    }

    fn capabilities(&self) -> Option<Vec<Capability>> {
        Some(vec![
            Capability::ListModels,
            Capability::GetModel,
            Capability::IngestModel,
            Capability::PublishModel,
            Capability::ListDatasets,
            Capability::GetDataset,
            // Capability::IngestDataset,
            Capability::ConvertModelMetadata,
        ])
    }
}

#[async_trait::async_trait]
impl ListModelsClient for HuggingFaceClient {
    type Data = Value;
    type Metadata = Value;

    async fn list_models(
        &self,
        request: &ListModelsByPlatformRequest,
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
                build_client_response(response).await
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
        request: &GetModelByPlatformRequest,
    ) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError>
    {
        let headers = match HeaderMap::try_from(&HuggingFaceHeaders(request.headers.clone())) {
            Ok(_header_map) => {
                // TODO Add the authorization header and value if one exists
                let map = HeaderMap::new();
                map
            },
            Err(_) => {
                return Err(ClientError::Internal {
                    msg: "failed to convert to header map".into(),
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
                build_client_response(response).await
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
        let access_token = request.headers.get_first_value("Authorization")
            .map(|t| t.replace("Bearer ", ""));

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
        request: &ListDatasetsByPlatformRequest,
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
                build_client_response(response).await
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
        request: &GetDatasetByPlatformRequest,
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
                build_client_response(response).await
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
        let access_token = request.headers.get_first_value("authorization")
            .map(|t| t.replace("Bearer ", ""));

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

    async fn publish_model(&self, extracted_artifact_path: &PathBuf, _artifact: &Artifact, maybe_metadata: Option<&ModelMetadata>, request: &PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        let metadata = match maybe_metadata {
            Some(m) => m,
            None => return Err(ClientError::BadRequest { msg: "A model metadata entry must exist for this artifact in order to publish to huggingface".into(), scope: ClientErrorScope::Client })
        };
        
        let model_name = metadata.name.clone();

        // Get the access token from the headers
        let access_token = match request.headers.get_first_value("Authorization") {
            Some(t) => t.replace("Bearer ", ""),
            None => return Err(ClientError::BadRequest { msg: "Missing Authorization header".into(), scope: ClientErrorScope::Client })
        };
        
        // Check that the repo on huggingface exists
        let base_url = Self::format_url("models");
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
        
        // Pull the large files wil git lfs then remove the existing .git directory
        if extracted_artifact_path.join(".git").is_dir() {
            std::fs::remove_dir_all(extracted_artifact_path.join(".git"))
                .map_err(|err| ClientError::Internal { msg: format!("Error removing .git directory: {}", err.to_string()), scope: ClientErrorScope::Client })?;
        }

        // Get the huggingface username from the model name
        let hf_username = model_name.split("/").collect::<Vec<&str>>()[0];
        
        // Construct remote name. Contains the auth token
        let origin = PathBuf::new()
            .join(constants::HUGGING_FACE_BASE_URL.replace("//huggingface", format!("//{}:{}@huggingface", &hf_username, &access_token).as_str()))
            .join(&model_name)
            .to_string_lossy()
            .to_string();
        
        // Initialize git repo, add all changes, commit, then add remote
        let init_output = Command::new("sh")
            .current_dir(&extracted_artifact_path)
            .arg("-c")
            .arg(format!("set -e; git init && git add -A && git -c user.name=\"MLHub HugginFace Client\" -c user.email=\"hf.client@mlhub\" commit -m \"MLHub HuggingFace Client: initial commit\" && git remote add origin {}", &origin))
            .output()
            .map_err(|err| ClientError::Internal { msg: format!("Error initializing, commiting, and adding origin: {}", err.to_string()), scope: ClientErrorScope::Client })?;

        // Check that the operation was successful
        match init_output.status.code() {
            Some(code) => {
                if code != 0 {
                    return Err(
                        ClientError::Internal {
                            msg: String::from_utf8(init_output.stderr)
                                .unwrap_or("git init operation failed. Additionally, stderr from the git rev-parse process could not be decoded".into()),
                            scope: ClientErrorScope::Client
                        }
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
                // NOTE We a trimming at the end because we are getting the newline from stdout!
                String::from_utf8(branch_name_output.stdout)
                    .map_err(|err| ClientError::Internal { msg: format!("Failed to decode stdout of command `git rev-parse ...`: {}", err.to_string()), scope: ClientErrorScope::Client })?
                    .trim()
                    .to_string()
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
            .env("GIT_CURL_VERBOSE", "1")
            .env("GIT_TRACE", "1")
            .env("GIT_TRANSFER_TRACE", "1")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg(&branch_name)
            .output()
            .map_err(|err| {
                ClientError::Internal { msg: format!("Failed to push artifact: {}", err.to_string()), scope: ClientErrorScope::Client }
            })?;

        // TODO check metadata for a version number. If provided, create a tag
        // and push it up

        // Check that the push was successful
        match push_output.status.code() {
            Some(code) => {
                if code != 0 {
                    return Err(
                        ClientError::Internal {
                            msg: String::from_utf8(push_output.stderr)
                                    .unwrap_or(format!("`git push origin {}` operation failed. Additionally, stderr from the git rev-parse process could not be decoded", &branch_name)),
                            scope: ClientErrorScope::Client }
                    )
                }
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

    async fn publish_model_metadata(&self, _metadata: &ModelMetadata, _result: &PublishArtifactServiceRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
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

impl ModelMetadataConversionClient for HuggingFaceClient {
    fn from_platform_metadata<T>(&self, client_metadata: T, author: String, tenant_id: String) -> Result<entities::model_metadata::ModelMetadata, ClientError>
        where T: serde::Serialize
    {
        let value = serde_json::to_value(client_metadata)
            .map_err(|err| ClientError::Internal { msg: format!("Failed to convert serializable client metadata into Value: {}", err.to_string()), scope: ClientErrorScope::Server })?;

        if let Ok(hf_model) = serde_json::from_value::<HFModelMetadata>(value) {
            let tags: Vec<String> = hf_model.tags.clone();
    
            // Task types derived from the tags. The "pipeline_tag"
            // property will be the authroitative soure for the task type 
            // if none are found
            let mut derived_task_types: Vec<shared::shared_kernel::enums::Task> = Vec::new();
            for tag in tags.clone() {
                match shared::shared_kernel::enums::Task::try_from(Self::normalize_string(tag).as_str()) {
                    Ok(t) => derived_task_types.push(t),
                    Err(_) => continue // Ignore as they tag cannot be interpreted as a task type
                }
            }
            
            // Compound tags are huggingface tags whose value contains the ":" char.
            // From these compund tags we can derive properties we are interested in like
            // license and task type
            let compound_tags = hf_model.parse_compound_tags();
    
            // Derive the license
            let license = compound_tags.iter()
                .filter(|ct| ct.name == "license")
                .collect::<Vec<&CompoundTag>>()
                .first()
                .and_then(|ct| Some(ct.value.clone()));
    
            // Convert pipeline tag to a variant of the task type enum.
            let mut task_types: Vec<shared::shared_kernel::enums::Task> = derived_task_types;
            match shared::shared_kernel::enums::Task::try_from(Self::normalize_string(hf_model.pipeline_tag.clone()).as_str()) {
                Ok(t) => {
                    if !task_types.contains(&t) {
                        task_types.push(t)
                    }
                },
                Err(err) => {
                    return Err(ClientError::Internal {
                        msg: format!("Failed to convert pipeline tag '{}' to Task for model {}: {}", &hf_model.pipeline_tag, &hf_model.id, err.to_string()),
                        scope: ClientErrorScope::Server
                    })
                }
            };
    
            // Determine which python libraries this model can be used with
            let mut libraries: Vec<String> = Vec::new();
            let known_libs: &[String] = &["transformers".into(), "diffusers".into(), "tensorflow".into(), "pytorch".into()];
            for lib in known_libs {
                if tags.contains(lib) && !libraries.contains(lib) {
                    libraries.push(lib.clone())
                }
            }
    
            // Parse the model name from the model's id
            let name = match hf_model.get_model_name() {
                Ok(n) => n,
                Err(err) => {
                    return Err(ClientError::Internal {
                        msg: err.to_string(),
                        scope: ClientErrorScope::Client
                    })
                }
            };
            
            return Ok(entities::model_metadata::ModelMetadata {
                name,
                artifact_id: None,
                description: None,
                author,
                tenant_id,
                model_type: None,
                canonical: Some(entities::model_metadata::Canonical {
                    platform: Platform::HuggingFace,
                    author: Some(hf_model.author.clone()),
                    model_id: hf_model.id.clone(),
                    downloads: Some(hf_model.downloads),
                    locator: entities::model_metadata::Locator {
                        url: format!("https://huggingface.co/{}", &hf_model.id.clone())
                    },
                    likes: Some(hf_model.likes),
                    gated: Some(hf_model.gated),
                    private: Some(hf_model.private),
                    sha: Some(hf_model.sha),
                }),
                libraries: Some(libraries),
                tags: Some(tags),
                task_types: Some(task_types),
                regulatory: None,
                license,
                deployment_strategy_refs: vec![],
            });
        }

        Err(ClientError::Internal { msg: "Failed to convert ".into(), scope: ClientErrorScope::Server })
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

    pub fn normalize_string(string: String) -> String {
        string.split("/")
            .into_iter()
            .map(|p| p.to_pascal_case())
            .collect::<Vec<String>>()
            .join("")
    }
}
