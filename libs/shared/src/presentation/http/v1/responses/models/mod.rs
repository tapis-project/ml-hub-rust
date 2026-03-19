use serde::Serialize;
use utoipa::ToSchema;

use crate::presentation::http::v1::requests::models::ModelMetadata;
use super::ArtifactType;

#[derive(Serialize, ToSchema)]
pub struct ModelArtifact {
    pub id: String,
    pub artifact_type: ArtifactType,
    pub created_at: String,
    pub last_modified: String,
    pub metadata: Option<ModelMetadata>
}