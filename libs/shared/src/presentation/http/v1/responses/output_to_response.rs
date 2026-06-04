use crate::application::outputs::artifacts::ModelArtifactOutput;
use crate::presentation::http::v1::responses;
use crate::errors::Error;

// TODO We need a response dto of the ModelMetadata. Reusing the request dto
// is fine for now though, it's just confusing.
use crate::presentation::http::v1::requests::models::ModelMetadata;


impl TryFrom<ModelArtifactOutput> for responses::models::ModelArtifact {
    type Error = Error;

    fn try_from(value: ModelArtifactOutput) -> Result<Self, Error> {
        let artifact = responses::Artifact::from(value.artifact);
        let metadata = match value.metadata {
            Some(m) => {
                match ModelMetadata::try_from(m) {
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