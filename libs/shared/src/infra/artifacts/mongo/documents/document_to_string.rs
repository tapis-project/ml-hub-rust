use crate::infra::artifacts::mongo::documents::ArtifactType;

impl From<ArtifactType> for String {
    fn from(value: ArtifactType) -> Self {
        match value {
            ArtifactType::Dataset => "Dataset".into(),
            ArtifactType::Model => "Model".into(),
        }
    }
}