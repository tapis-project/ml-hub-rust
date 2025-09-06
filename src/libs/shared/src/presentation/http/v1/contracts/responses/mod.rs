use serde_json::Value;
use utoipa::ToSchema;
use crate::presentation::http::v1::{dto::models::ModelMetadata, responses::{Artifact, ArtifactIngestion, ArtifactPublication}};

#[derive(ToSchema)]
pub struct ListPlatformsResponse {
    pub result: Vec<String>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct PublishModelArtifactResponse {
    pub result: ArtifactPublication,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelPublicationsForArtifactResponse {
    pub result: Vec<ArtifactPublication>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelPublicationsResponse {
    pub result: Vec<ArtifactPublication>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct GetModelPublicationResponse {
    pub result: ArtifactPublication,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct IngestModelArtifactResponse {
    pub result: ArtifactIngestion,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}


#[derive(ToSchema)]
pub struct GetModelIngestionResponse {
    pub result: ArtifactIngestion,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelIngestionsResponse {
    pub result: Vec<ArtifactIngestion>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}


#[derive(ToSchema)]
pub struct CreateModelMetadataResponse {
    #[schema(value_type = Object)]
    pub result: ModelMetadata,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct DiscoverModelsByPlatformResponse {
    #[schema(value_type = Object)]
    pub result: Vec<Value>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct GetModelByPlatformResponse {
    #[schema(value_type = Object)]
    pub result: Value,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelsByPlatformResponse {
    #[schema(value_type = Object)]
    pub result: Vec<Value>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct GetModelArtifactResponse {
    pub result: Artifact,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelArtifactResponse {
    pub result: Vec<Artifact>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}