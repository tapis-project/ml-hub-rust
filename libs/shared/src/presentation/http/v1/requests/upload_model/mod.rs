use crate::errors::Error;

use crate::application::inputs;

pub struct UploadModelRequest {}

impl TryFrom<UploadModelRequest> for inputs::artifacts::UploadArtifactInput {
    type Error = Error;
    fn try_from(_value: UploadModelRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            artifact_type: inputs::artifacts::ArtifactType::Model
        })
    }
}