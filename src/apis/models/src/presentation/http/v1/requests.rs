pub use shared::presentation::http::v1::requests::models::{
    ListModelsPath,
    ListModelsRequest,
    GetModelPath,
    GetModelRequest,
    IngestModelPath,
    IngestModelRequest,
    DownloadModelPath,
    DownloadModelRequest,
    UploadModelRequest,
    ModelMetadata,
    AssociateModelMetadataPath,
    AssociateModelMetadata
};
pub use shared::presentation::http::v1::requests::discover_models::{
    DiscoverModelsByPlatformPath,
    DiscoverModelsByPlatformRequest,
    DiscoverModelsRequest,
    DiscoveryCriteria,
};
pub use shared::presentation::http::v1::requests::artifact_ingestions::GetArtifactIngestionPath;
pub use shared::presentation::http::v1::requests::artifact_publications::GetArtifactPublicationPath;
pub use shared::presentation::http::v1::requests::artifacts::{
    PublishArtifactPath,
    PublishArtifactRequest,
    PublishArtifactServiceRequest,
    IngestArtifactRequest,
    ListArtifactIngestionsPath,
    ListArtifactPublicationsPath,
};
pub use shared::presentation::http::v1::responses::ArtifactPublication;
pub use shared::presentation::http::v1::requests::headers::Headers;