use async_trait;
use clients::{ClientError, ClientErrorScope, IngestDatasetClient, IngestModelClient};
use shared::infra::fs::git::{
    SyncGitRepository, SyncGitRepositoryImpl, SyncLfsRepositoryParams,
};
use shared::presentation::http::v1::actix_web::helpers::param_to_string;
use shared::presentation::http::v1::dto::datasets::IngestDatasetRequest;
use shared::logging::SharedLogger;
use shared::presentation::http::v1::dto::models::IngestModelRequest;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GithubLfsClient {
    _logger: SharedLogger,
}

impl SyncGitRepository for GithubLfsClient {}

#[async_trait::async_trait]
impl IngestModelClient for GithubLfsClient {
    async fn ingest_model(
        &self,
        request: &IngestModelRequest,
        target_path: PathBuf,
    ) -> Result<(), ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch").map_err(|_| {
            ClientError::BadRequest {
                msg: "Parameter 'branch' missing from the request".into(),
                scope: ClientErrorScope::Client,
            }
        })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url: String::from("https://github.com"),
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
impl IngestDatasetClient for GithubLfsClient {
    async fn ingest_dataset(
        &self,
        request: &IngestDatasetRequest,
        target_path: PathBuf,
    ) -> Result<(), ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch").map_err(|_| {
            ClientError::BadRequest {
                msg: "Parameter 'branch' missing from the request".into(),
                scope: ClientErrorScope::Client,
            }
        })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url: String::from("https://github.com"),
            target_dir: target_path.to_string_lossy().to_string(),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone(),
        })?;

        Ok(())
    }
}

impl GithubLfsClient {
    pub fn new() -> Self {
        Self {
            _logger: SharedLogger::new(),
        }
    }
}
