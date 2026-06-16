use serde_json::Value;
use utoipa::ToSchema;
use crate::presentation::http::v1::responses::tasks::Task;
use crate::presentation::http::v1::responses::deployment::client_strategy_set::ClientStrategySet;
use crate::presentation::http::v1::responses::deployment::ModelDeployment;
use crate::presentation::http::v1::responses::models::{
    ModelMetadata,
    ModelArtifact,
};
use crate::presentation::http::v1::responses::artifacts::{
    Artifact,
    ingestions::ArtifactIngestion,
    publications::ArtifactPublication,
};
use crate::presentation::http::v1::responses::platform_details::PlatformDetails;

#[derive(ToSchema)]
pub struct ListTasksResponse {
    pub result: Vec<Task>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListPlatformsResponse {
    pub result: Vec<PlatformDetails>,
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
pub struct AssociateModelMetadataResponse {
    #[schema(value_type = Object)]
    pub result: ModelMetadata,
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
    pub result: Vec<Value>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}


#[derive(ToSchema)]
pub struct DiscoverModelsResponse {
    pub result: Vec<ModelMetadata>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct GetModelResponse {
    #[schema(value_type = Object)]
    pub result: ModelMetadata,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ListModelsResponse {
    pub result: Vec<ModelMetadata>,
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
    pub result: Vec<Value>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct GetModelArtifactResponse {
    pub result: ModelArtifact,
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

#[derive(ToSchema)]
pub struct ListDeploymentStrategiesResponse {
    pub result: Vec<ClientStrategySet>,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ModelDeploymentResponse {
    pub result: ModelDeployment,
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct BadRequestResponse {
    #[schema(default=null)]
    pub result: Value,
    #[schema(default=400)]
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct NotFoundResponse {
    #[schema(default=null)]
    pub result: Value,
    #[schema(default=404)]
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}

#[derive(ToSchema)]
pub struct ServerErrorResponse {
    #[schema(default=null)]
    pub result: Value,
    #[schema(default=500)]
    pub status: u16,
    pub message: String,
    #[schema(value_type = Object)]
    pub metadata: Value,
    pub version: String
}