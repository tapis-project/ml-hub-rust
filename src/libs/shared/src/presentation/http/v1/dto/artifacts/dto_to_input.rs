use uuid::Uuid;
use super::CreateArtifactPublication;
use crate::application::inputs::artifact_publication::PublishArtifactInput;
use crate::application::errors::ApplicationError;

impl TryFrom<CreateArtifactPublication> for PublishArtifactInput {
    type Error = ApplicationError;

    fn try_from(value: CreateArtifactPublication) -> Result<Self, Self::Error> {
        let artifact_id = Uuid::parse_str(&value.artifact_id)
            .map_err(|err| ApplicationError::ConvesionError(err.to_string()))?;
        
        Ok(Self {
            artifact_id,
            platform: value.platform,
        })
    }
}