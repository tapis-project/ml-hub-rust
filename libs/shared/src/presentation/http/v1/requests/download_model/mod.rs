pub mod path;

use uuid::Uuid;

use crate::presentation::http::v1::requests::common::headers::Headers;
use crate::application::inputs;
use crate::errors::Error;

pub struct DownloadModelRequest {
    pub headers: Headers,
    pub path: path::DownloadModelPath,
}

impl TryFrom<DownloadModelRequest> for inputs::artifacts::DownloadArtifactInput {
    type Error = Error;
    fn try_from(value: DownloadModelRequest) -> Result<Self, Self::Error> {
        let artifact_id= match Uuid::parse_str(&value.path.artifact_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Error::new("Value provided for artifact_id is not a UUID".into()))
        };
        
        Ok(Self {
            artifact_type: inputs::artifacts::ArtifactType::Model,
            artifact_id
        })
    }
}