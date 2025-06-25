use std::path::PathBuf;

use shared::constants;
use clients::{
    ClientError, ClientErrorScope, ClientStagedArtifactResponse, IngestDatasetClient, IngestModelClient
};
use shared::common::infra::fs::git::{
    SyncGitRepository,
    SyncLfsRepositoryParams,
    SyncGitRepositoryImpl
};
use shared::models::presentation::http::v1::dto::IngestModelRequest;
use shared::datasets::presentation::http::v1::dto::IngestDatasetRequest;
use shared::common::presentation::http::v1::actix_web::helpers::param_to_string;
use shared::common::presentation::http::v1::dto::{
    Artifact,
    ArtifactStagingParams,

};
use clients::artifacts::{
    ArtifactStager,
    ArtifactGenerator,

};
use shared::logging::SharedLogger;


#[derive(Debug)]
pub struct GithubLfsClient {
    _logger: SharedLogger
}

impl ArtifactGenerator for GithubLfsClient {}

impl SyncGitRepository for GithubLfsClient {}

impl IngestModelClient for GithubLfsClient {
    fn ingest_model(&self, request: &IngestModelRequest, _target_path: PathBuf) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|_| {
                ClientError::BadRequest { msg: "Parameter 'branch' missing from the request".into(), scope: ClientErrorScope::Client }
            })?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url: String::from("https://github.com"),
            target_dir_prefix: String::from(constants::MODEL_INGEST_DIR_NAME),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })?;

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
        
        let staged_artifact = self.stage(params)?;
    
        // Create the client response
        Ok(ClientStagedArtifactResponse::new(
            staged_artifact,
            Some(200),
        ))
    }
}

impl IngestDatasetClient for GithubLfsClient {
    fn ingest_dataset(&self, request: &IngestDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|_| {
                ClientError::BadRequest { msg: "Parameter 'branch' missing from the request".into(), scope: ClientErrorScope::Client }
            })?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url: String::from("https://github.com"),
            target_dir_prefix: String::from(constants::MODEL_INGEST_DIR_NAME),
            branch,
            access_token: access_token.clone(),
            include_paths: request.body.include_paths.clone(),
            exclude_paths: request.body.exclude_paths.clone()
        })?;
        
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
        
        let staged_artifact = self.stage(params)?;
    
        // Create the client response
        Ok(ClientStagedArtifactResponse::new(
            staged_artifact,
            Some(200),
        ))
    }
}

impl GithubLfsClient {
    pub fn new() -> Self {
        Self {
            _logger: SharedLogger::new(),
        }
    }
}
