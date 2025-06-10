use shared::constants;
use clients::{
    ClientError,
    ClientErrorScope,
    ClientStagedArtifactResponse,
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
use clients::artifacts::{
    ArtifactStager,
    ArtifactGenerator,
};
use shared::logging::SharedLogger;
use shared::common::presentation::http::v1::dto::{
    Artifact,
    ArtifactStagingParams
};
use shared::common::presentation::http::v1::actix_web::helpers::param_to_string;


#[derive(Debug)]
pub struct GitLfsClient {
    _logger: SharedLogger
}

impl ArtifactGenerator for GitLfsClient {}

impl SyncGitRepository for GitLfsClient {}

impl IngestModelClient for GitLfsClient {
    fn ingest_model(&self, request: &IngestModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
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

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url,
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

impl IngestDatasetClient for GitLfsClient {
    fn ingest_dataset(&self, request: &IngestDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
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

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url,
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

impl GitLfsClient {
    pub fn new() -> Self {
        Self {
            _logger: SharedLogger::new(),
        }
    }
}
