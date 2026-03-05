use uuid::Uuid;
use super::PublishArtifactServiceRequest;
use crate::application::inputs::artifact_publication::PublishArtifactInput;
use crate::application::errors::ApplicationError;
use serde_json::to_vec;

impl TryFrom<PublishArtifactServiceRequest> for PublishArtifactInput {
    type Error = ApplicationError;

    fn try_from(value: PublishArtifactServiceRequest) -> Result<Self, Self::Error> {
        let artifact_id = Uuid::parse_str(&value.path.artifact_id)
            .map_err(|err| ApplicationError::ConversionError(err.to_string()))?;

        let serialized_client_request = to_vec(&value)
            .map_err(|err| ApplicationError::ConversionError(format!("Failed serialize the full client request: {}", err.to_string())))?;
        
        Ok(Self {
            artifact_id,
            webhook_url: value.body.webhook_url,
            target_platform: value.body.target_platform,
            serialized_client_request
        })
    }
}