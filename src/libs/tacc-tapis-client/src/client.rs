use crate::operations::files::{
    MkdirResponse,
    mkdir,
    // insert
};
use crate::utils::token_from_request;
use crate::tokens::decode_jwt;

use shared::artifacts::ArtifactGenerator;
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::requests::{
    GetModelRequest,
    ListModelsRequest,
    DownloadModelRequest,
    ListDatasetsRequest,
    GetDatasetRequest,
    DownloadDatasetRequest,
    DiscoverModelsRequest,
    PublishDatasetRequest,
    PublishModelRequest,
};
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct TapisClient {
    logger: SharedLogger
}

impl ArtifactGenerator for TapisClient {}

impl ModelsClient for TapisClient {
    fn list_models(&self, _request: &ListModelsRequest,) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }
    
    fn get_model(&self, _request: &GetModelRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn download_model(&self, _request: &DownloadModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn discover_models(&self, _request: &DiscoverModelsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    /// Takes a single file uploaded to the Models API and upload it to a Tapis
    /// system
    fn publish_model(&self, request: &PublishModelRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        self.logger.debug("Publishing model");
        let token = token_from_request(&request.req)
            .ok_or_else(|| ClientError::new(String::from("Missing tapis token in 'X-Tapis-Token' header")))?;
        
        let claims = decode_jwt(&token)
            .map_err(|err| ClientError::new(format!("Failed to decode jwt: {}", err.to_string())))?;

        // Make the directories if they have not yet been made on the tapis sytem
        let path = request.path.path.clone();

        // Parse the system id from the path.
        let path = path
            .strip_prefix("/")
            .unwrap_or(&path)
            .strip_suffix("/")
            .unwrap_or(&path)
            .replace("//", "/");
            
        let parts = path.splitn(2, "/")
            .collect::<Vec<&str>>();

        if parts.len() == 0 || (parts.len() > 0 && parts[0].len() == 0) {
            return Err(ClientError::from_str("Tapis system id is missing from path. Should be the first item in the path"));
        }

        let mut rest_of_path = "/";
        if parts.len() > 1 {
            rest_of_path = parts[1];
        }

        let mkdir_resp = mkdir(
            claims.tapis_tenant_id.clone(),
            parts[0].to_string(),
            rest_of_path.to_string(),
            Some(token)
        ).map_err(|err| ClientError::new(err.to_string()))?;
        
        let mkdir_status_code = mkdir_resp.status()
            .to_string()
            .parse::<i16>()
            .unwrap_or(500);

        let deserizied_mkdir_resp: MkdirResponse = mkdir_resp.json()
            .map_err(|err| ClientError::new(err.to_string()))?;

        // Pass along the file mkdir error code
        if !(200..=299).contains(&mkdir_status_code) {
            return Err(ClientError::new(format!("Error making directories on the target system: {}", &deserizied_mkdir_resp.message)))
        }

        // Upload the model file to the system

        Err(ClientError::from_str(""))
    }
}

impl DatasetsClient for TapisClient {
    fn list_datasets(&self, _request: &ListDatasetsRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn get_dataset(&self, _request: &GetDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn download_dataset(&self, _request: &DownloadDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }

    fn publish_dataset(&self, _request: &PublishDatasetRequest) -> Result<ClientStagedArtifactResponse, ClientError> {
        Err(ClientError::new(String::from("Operation not supported")))
    }
}

impl TapisClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
