use crate::application::outputs::artifacts::ModelArtifactOutput;
use crate::presentation::http::v1::responses;
use crate::errors::Error;
use crate::presentation::http::v1::responses::models::ModelMetadata;


impl TryFrom<ModelArtifactOutput> for responses::models::ModelArtifact {
    type Error = Error;

    fn try_from(value: ModelArtifactOutput) -> Result<Self, Error> {
        let artifact = responses::artifacts::Artifact::from(value.artifact);
        let metadata = match value.metadata {
            Some(m) => {
                match ModelMetadata::try_from(&m) {
                    Ok(m) => Some(m),
                    Err(err) => return Err(Error::new(err.to_string()))
                }
            },
            None => None
        };

        Ok(Self {
            id: artifact.id,
            artifact_type: artifact.artifact_type,
            created_at: artifact.created_at,
            last_modified: artifact.last_modified,
            metadata,
        })
    }
}