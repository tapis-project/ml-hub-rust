use crate::application::outputs::artifacts::ModelArtifactOutput;
use crate::presentation::http::v1::responses;
use crate::errors::Error;

impl TryFrom<ModelArtifactOutput> for responses::models::ModelArtifact {
    type Error = Error;

    fn try_from(value: ModelArtifactOutput) -> Result<Self, Error> {
        let artifact = responses::artifacts::Artifact::from(value.artifact);

        Ok(Self {
            id: artifact.id,
            created_at: artifact.created_at,
            last_modified: artifact.last_modified,
        })
    }
}