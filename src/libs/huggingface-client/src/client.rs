use crate::constants;
use crate::requests::{
    ListModelsQueryParameters,
    ListDatasetsQueryParameters,
};
use crate::utils::deserialize_response_body;
use reqwest::blocking::Client as ReqwestClient;
use shared::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::git::{
   SyncGitRepository,
   SyncGitRepositoryImpl,
   SyncLfsRepositoryParams
};
use shared::requests::{
    DiscoverModelsRequest,
    DownloadDatasetRequest,
    DownloadModelRequest,
    GetDatasetRequest,
    GetModelRequest,
    ListDatasetsRequest,
    ListModelsRequest,
    PublishDatasetRequest,
    PublishModelRequest,
    utils::param_to_string
};
use shared::artifacts::{
    Artifact,
    ArtifactStager,
    ArtifactStagingParams,
};
use shared::logging::SharedLogger;
use shared::constants::{
    MODEL_DOWNLOAD_DIR_NAME,
    DATASET_DOWNLOAD_DIR_NAME,
};
use serde_json::{Value, Map};


#[derive(Debug)]
pub struct HuggingFaceClient {
    client: ReqwestClient,
    logger: SharedLogger
}

impl ArtifactGenerator for HuggingFaceClient {}

impl ModelsClient for HuggingFaceClient {
    fn list_models(
        &self,
        request: &ListModelsRequest,
    ) -> Result<ClientJsonResponse, ClientError> {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10)
        };

        // Build the query parameters
        let query_params = Some(
            ListModelsQueryParameters {
                search: request.query.get("search").cloned(),
                author: request.query.get("author").cloned(),
                filter: request.query.get("filter").cloned(),
                sort: request.query.get("sort").cloned(),
                direction: request.query.get("direction").cloned(),
                limit: Some(limit.unwrap_or(10)),
                full: None,
                config: None,
            }
        );
        
        // Construct the url for the request
        let url = Self::format_url("models");

        self.logger.debug(format!("Request url: {}", url).as_str());
        self.logger.debug(format!("Query Params: {:#?}", &query_params).as_str());

        // Make a GET request to Hugging Face to fetch the models
        let result = self.client
            .get(url)
            .query(&query_params)
            .send();
        
        match result {
            Ok(response) => {
                let body = deserialize_response_body(response)?;
                
                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new()))
                ))
            },
            
            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::new(err.to_string()))
            },
        }
    }
    
    fn get_model(&self, request: &GetModelRequest) -> Result<ClientJsonResponse, ClientError> {
        let result = self.client
            .get(Self::format_url(format!("{}/{}", "models", request.path.model_id).as_str()))
            .send();

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response)?;
                
                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            },
            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::new(err.to_string()))
            }
        }
    }

    fn download_model(&self, request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.req
            .headers()
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .map(|value| String::from(value));

        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::new(err.to_string()))?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url: String::from(constants::HUGGING_FACE_BASE_URL),
            target_dir_prefix: String::from(MODEL_DOWNLOAD_DIR_NAME),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })
            .map_err(|err| ClientError::new(String::from(err.to_string())))?;

        // Resolve the filename or set a default
        let download_filename = request.body.download_filename
            .clone();

        let artifact = Artifact {
            path: String::from(git_lfs_repo.repo.path.to_string_lossy()),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        };

        let archive_type = request.body.archive.clone();

        let compression_type = request.body.compression.clone();

        let params = ArtifactStagingParams {
            artifact: &artifact,
            staged_filename: download_filename,
            archive: archive_type.clone(),
            compression: compression_type
        };
        
        let staged_artifact = self.stage(params)
            .map_err(|err| {
                let msg = format!("Error staging artifact: {}", err.to_string());
                self.logger.error(msg.as_str());
                ClientError::new(msg)
        })?;
    
        // Create the client response
        Ok(ClientStagedArtifactResponse::new(
            staged_artifact,
            Some(200),
        ))
    }

    fn discover_models(&self, _: &DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Discover models not implemented")))
    }

    async fn publish_model(&self, _result: &PublishModelRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Not supported")))
    }
}

impl DatasetsClient for HuggingFaceClient {
    fn list_datasets(
        &self,
        request: &ListDatasetsRequest,
    ) -> Result<ClientJsonResponse, ClientError> {
        // Parse the limit from the query string
        let limit = match request.query.get("limit").cloned() {
            Some(num) => num.parse(),
            None => Ok(10)
        };

        // Build the query parameters
        let query_params = Some(
            ListDatasetsQueryParameters {
                search: request.query.get("search").cloned(),
                author: request.query.get("author").cloned(),
                filter: request.query.get("filter").cloned(),
                sort: request.query.get("sort").cloned(),
                direction: request.query.get("direction").cloned(),
                limit: Some(limit.unwrap_or(10)),
                full: None,
            }
        );

        // Make a GET request to Hugging Face to fetch the datasets
        let result = self.client
            .get(Self::format_url("datasets"))
            .query(&query_params)
            .send();

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response)?;
                
                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            },

            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::new(err.to_string()))
            },
        }
    }

    fn get_dataset(&self, request: &GetDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        let result = self.client
            .get(Self::format_url(format!("{}/{}", "datasets", request.path.dataset_id).as_str()))
            .send();

        match result {
            Ok(response) => {
                let body = deserialize_response_body(response)?;

                Ok(ClientJsonResponse::new(
                    Some(200),
                    Some(String::from("success")),
                    Some(body),
                    Some(Value::Object(Map::new())),
                ))
            },
            Err(err) => {
                self.logger.error(format!("{:#?}", err).as_str());
                return Err(ClientError::new(err.to_string()))
            },
        }
    }

    fn download_dataset(&self, request: &DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.req
            .headers()
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .map(|value| String::from(value));

        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::new(err.to_string()))?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url: String::from(constants::HUGGING_FACE_BASE_URL),
            target_dir_prefix: String::from(DATASET_DOWNLOAD_DIR_NAME),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })
            .map_err(|err| ClientError::new(String::from(err.to_string())))?;

        // Resolve the filename or set a default
        let download_filename = request.body.download_filename
            .clone();

        let artifact = Artifact {
            path: String::from(git_lfs_repo.repo.path.to_string_lossy()),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        };

        let archive_type = request.body.archive.clone();

        let compression_type = request.body.compression.clone();

        let params = ArtifactStagingParams {
            artifact: &artifact,
            staged_filename: download_filename,
            archive: archive_type.clone(),
            compression: compression_type
        };
        
        let staged_artifact = self.stage(params)
            .map_err(|err| {
                let msg = format!("Error staging artifact: {}", err.to_string());
                self.logger.error(msg.as_str());
                ClientError::new(msg)
        })?;
    
        // Create the client response
        Ok(ClientStagedArtifactResponse::new(
            staged_artifact,
            Some(200),
        ))
    }

    fn publish_dataset(&self, _result: &PublishDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Not supported")))
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
