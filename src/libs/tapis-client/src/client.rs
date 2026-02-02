// use crate::operations::files::{
//     MkdirResponse,
//     mkdir,
//     // insert
// };
// use crate::utils::token_from_headers;
// use crate::tokens::decode_jwt;
use shared::application::ports::commands::DeployModelWithStrategyCommandPayload;
use std::path::PathBuf;
use async_trait;
use platforms::Platform;
use serde_json::Value;
use clients::{
    Capability,
    Client,
    ClientError,
    ClientJsonResponse,
    ModelDeploymentClient,
    ModelDeploymentError,
    ModelDeploymentCapabilities
};
use shared::domain::entities::deployment::ModelDeployment;
use shared::logging::SharedLogger;
use shared::application::inputs::deployment;

#[derive(Debug)]
pub struct TapisClient {
    logger: SharedLogger
}

#[async_trait::async_trait]
impl Client for TapisClient {
    fn platform(&self) -> Option<Platform> {
        Some(Platform::TaccTapis)
    }
    
    fn capabilities(&self) -> Option<Vec<Capability>> {
        Some(vec![])
    }
}

#[async_trait::async_trait]
impl ModelDeploymentClient for TapisClient {
    async fn deploy_model_with_strategy(&self, _input: &DeployModelWithStrategyCommandPayload) -> Result<ModelDeployment, ModelDeploymentError> {
        return Err(ModelDeploymentError::Unimplemented("Not implemented".into()));
    }

    fn capabilities(&self) -> ModelDeploymentCapabilities {
        ModelDeploymentCapabilities { can_self_provision_model: true }
    }
}

impl TapisClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
