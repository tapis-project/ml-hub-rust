use crate::operations::files::{
    MkdirResponse,
    mkdir,
    // insert
};
use crate::utils::token_from_request;
use crate::tokens::decode_jwt;

use shared::artifacts::{
    // Archive,
    // Artifact,
    ArtifactGenerator,
    // ArtifactStager,
    // ArtifactStagingParams,
    // Compression
};
use shared::clients::{
    ClientStagedArtifactResponse,
    ClientError,
    ClientJsonResponse,
    DatasetsClient,
    ModelsClient,
};
use shared::models::web::v1::dto::{
    DiscoverModelsRequest,
    DownloadModelRequest,
    GetModelRequest,
    ListModelsRequest,
    PublishModelRequest,
};
use shared::datasets::web::v1::dto::{
    DownloadDatasetRequest,
    GetDatasetRequest,
    ListDatasetsRequest,
    PublishDatasetRequest,
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

    // fn publish_model(&self, _request: &PublishModelRequest) -> Result<ClientJsonResponse, ClientError> {
    //     Err(ClientError::new(String::from("Operation not supported")))
    // }

    /// Takes a single file uploaded to the Models API and upload it to a Tapis
    /// system
    fn publish_model(&self, request: &PublishModelRequest) -> Result<ClientJsonResponse, ClientError> {
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

        let system_id = parts[0].to_string();

        let model_id = request.path.model_id.clone();

        let mut target_path = "/";
        if parts.len() > 1 {
            target_path = parts[1];
        }
        let formatted_target_path = format!("{}/{}", target_path, model_id);
        target_path = formatted_target_path.as_str();

        let mkdir_resp = mkdir(
            claims.tapis_tenant_id.clone(),
            system_id.clone(),
            target_path.to_string().clone(),
            Some(token.clone())
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

        // Package the files of upload
        

        // Upload the model file to the system
        // let resp_insert = insert(
        //     claims.tapis_tenant_id.clone(),
        //     system_id.clone(),
        //     staged_artifact.path.to_string_lossy().to_string(),
        //     target_path.to_string().clone(),
        //     Some(token.clone())
        // ).await;
        
        Err(ClientError::from_str("Placholder not implemented"))

        // match resp_insert {
        //     Ok(_) => {
        //         Ok(ClientJsonResponse::new(
        //             Some(200),
        //             Some(String::from("Successfully uploaded model")),
        //             None,
        //             None
        //         ))
        //     },
        //     Err(err) => Err(ClientError::new(err.to_string()))
        // }
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

    fn publish_dataset(&self, _request: &PublishDatasetRequest) -> Result<ClientJsonResponse, ClientError> {
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
