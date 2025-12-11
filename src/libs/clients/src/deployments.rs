use shared::presentation::http::v1::requests::deployments;
use serde::Serialize;
use crate::errors::ClientError;
use crate::responses::ClientJsonResponse;
use crate::client::Client;

#[async_trait::async_trait]
pub trait CreateModelDeploymentClient: Client {
    type Data: Serialize;
    type Metadata: Serialize;

    async fn deploy_model(&self, _request: &deployments::DeployModelRequest) -> Result<ClientJsonResponse<Self::Data, Self::Metadata>, ClientError> {
        return Err(ClientError::Unimplemented);
    }
}