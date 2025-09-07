use uuid::Uuid;
use super::PublishArtifactRequest;
use crate::application::inputs::artifact_publication::PublishArtifactInput;
use crate::application::errors::ApplicationError;
use serde_json::to_vec;

impl TryFrom<PublishArtifactRequest> for PublishArtifactInput {
    type Error = ApplicationError;

    fn try_from(value: PublishArtifactRequest) -> Result<Self, Self::Error> {
        let artifact_id = Uuid::parse_str(&value.path.artifact_id)
            .map_err(|err| ApplicationError::ConvesionError(err.to_string()))?;

        let serialized_client_request = to_vec(&value)
            .map_err(|err| ApplicationError::ConvesionError(format!("Failed serialize the full client request: {}", err.to_string())))?;
        
        Ok(Self {
            artifact_id,
            webhook_url: value.body.webhook_url,
            target_platform: value.body.target_platform,
            serialized_client_request
        })
    }
}