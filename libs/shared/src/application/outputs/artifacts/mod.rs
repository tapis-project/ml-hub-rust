use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::model::Model;

pub struct ModelArtifactOutput {
    pub artifact: Artifact,
    pub model: Option<Model>
}
