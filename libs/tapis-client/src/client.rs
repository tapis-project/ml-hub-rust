// use crate::operations::files::{
//     MkdirResponse,
//     mkdir,
//     // insert
// };
// use crate::utils::token_from_headers;
// use crate::tokens::decode_jwt;
use async_trait;
use clients::{Capability, Client, ClientError, ClientJsonResponse};
use platforms::Platform;
use serde_json::Value;
use shared::application::inputs::deployment;
use shared::domain::entities::deployment::ModelDeployment;
use shared::logging::SharedLogger;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TapisClient {
    logger: SharedLogger,
}

#[async_trait::async_trait]
impl Client for TapisClient {
    fn platform(&self) -> Option<Platform> {
        None
        // Some(Platform::TaccTapis)
    }

    fn capabilities(&self) -> Option<Vec<Capability>> {
        Some(vec![])
    }
}

impl TapisClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
