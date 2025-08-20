// use crate::operations::files::{
//     MkdirResponse,
//     mkdir,
//     // insert
// };
// use crate::utils::token_from_headers;
// use crate::tokens::decode_jwt;
use async_trait;
use serde_json::Value;
use clients::{
    ClientError,
    ClientJsonResponse,
    PublishModelClient,
    // ClientErrorScope
};
use shared::presentation::http::v1::dto::artifacts;
use shared::logging::SharedLogger;

#[derive(Debug)]
pub struct TapisClient {
    logger: SharedLogger
}

#[async_trait::async_trait]
impl PublishModelClient for TapisClient {
    type Data = Value;
    type Metadata = Value;

    async fn publish_model(&self, _request: &artifacts::PublishArtifactRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}

impl TapisClient {
    pub fn new() -> Self {
        Self {
            logger: SharedLogger::new(),
        }
    }
}
