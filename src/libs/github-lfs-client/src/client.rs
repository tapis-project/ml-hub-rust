use shared::constants;
use shared::clients::{
    ClientError,
    ClientStagedArtifactResponse,
    DownloadDatasetClient,
    DownloadModelClient
};
use shared::common::infra::fs::git::{
    SyncGitRepository,
    SyncLfsRepositoryParams,
    SyncGitRepositoryImpl
};
use shared::models::presentation::http::v1::dto::DownloadModelRequest;
use shared::datasets::presentation::http::v1::dto::DownloadDatasetRequest;
use shared::common::presentation::http::v1::helpers::param_to_string;
use shared::common::presentation::http::v1::dto::{
    Artifact,
    ArtifactStagingParams,

};
use shared::clients::artifacts::{
    ArtifactStager,
    ArtifactGenerator,

};
use shared::logging::SharedLogger;


#[derive(Debug)]
pub struct GithubLfsClient {
    logger: SharedLogger
}

impl ArtifactGenerator for GithubLfsClient {}

impl SyncGitRepository for GithubLfsClient {}

impl DownloadModelClient for GithubLfsClient {
    fn download_model(&self, request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::new(err.to_string()))?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.model_id.clone(),
            remote_base_url: String::from("https://github.com"),
            target_dir_prefix: String::from(constants::MODEL_DOWNLOAD_DIR_NAME),
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
}

impl DownloadDatasetClient for GithubLfsClient {
    fn download_dataset(&self, request: &DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        // Get the authorization token from the request
        let access_token = request.headers.get_first_value("Authorization");

        // Get the branch from the request
        let branch = param_to_string(request.body.params.clone(), "branch")
            .map_err(|err| ClientError::new(err.to_string()))?;

        let git_lfs_repo = self.sync_lfs_repo(SyncLfsRepositoryParams {
            name: request.path.dataset_id.clone(),
            remote_base_url: String::from("https://github.com"),
            target_dir_prefix: String::from(constants::MODEL_DOWNLOAD_DIR_NAME),
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
}

impl GithubLfsClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
