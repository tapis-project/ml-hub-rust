use std::path::PathBuf;
use clients::{
    ClientError,
    ClientErrorScope,
    IngestDatasetClient,
    IngestModelClient
};
use shared::common::infra::fs::git::{
    SyncGitRepository,
    SyncLfsRepositoryParams,
    SyncGitRepositoryImpl
};
use shared::models::presentation::http::v1::dto::IngestModelRequest;
use shared::datasets::presentation::http::v1::dto::IngestDatasetRequest;
use clients::artifacts::ArtifactGenerator;
use shared::logging::SharedLogger;
use shared::common::presentation::http::v1::actix_web::helpers::param_to_string;


#[derive(Debug)]
pub struct GitLfsClient {
    _logger: SharedLogger
}

impl ArtifactGenerator for GitLfsClient {}

impl SyncGitRepository for GitLfsClient {}

impl IngestModelClient for GitLfsClient {
    fn ingest_model(&self, request: &IngestModelRequest, target_path: PathBuf) -> Result<(), ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the remote base url from the request
        let remote_base_url = param_to_string(request.body.params.clone(), "remote_base_url")
            .map_err(|err| {
                ClientError::BadRequest { msg: err.to_string(), scope: ClientErrorScope::Client }
            })?
            .ok_or(ClientError::BadRequest { msg: "Parameter 'remote_base_url' missing from the request".into(), scope: ClientErrorScope::Client })?
            .trim_end_matches("/")
            .to_string();

        // Get branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| {
                ClientError::BadRequest { msg: err.to_string(), scope: ClientErrorScope::Client }
            })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url,
            target_dir: target_path.to_string_lossy().to_string(),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })?;

        Ok(())
    }
}

impl IngestDatasetClient for GitLfsClient {
    fn ingest_dataset(&self, request: &IngestDatasetRequest, target_path: PathBuf) -> Result<(), ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the remote base url from the request
        let remote_base_url = param_to_string(request.body.params.clone(), "remote_base_url")
            .map_err(|err| {
                ClientError::BadRequest { msg: err.to_string(), scope: ClientErrorScope::Client }
            })?
            .ok_or(ClientError::BadRequest { msg: "Parameter 'remote_base_url' missing from the request".into(), scope: ClientErrorScope::Client })?
            .trim_end_matches("/")
            .to_string();

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| {
                ClientError::BadRequest { msg: err.to_string(), scope: ClientErrorScope::Client }
            })?;

        self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url,
            target_dir: target_path.to_string_lossy().to_string(),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })?;

        Ok(())
    }
}

impl GitLfsClient {
    pub fn new() -> Self {
        Self {
            _logger: SharedLogger::new(),
        }
    }
}
