pub use shared::presentation::http::v1::requests::models::{
    ListModelsByPlatformPath,
    ListModelsByPlatformRequest,
    GetModelByPlatformPath,
    GetModelByPlatformRequest,
    IngestModelPath,
    IngestModelRequest,
    DownloadModelPath,
    DownloadModelRequest,
    UploadModelRequest,
    ModelMetadata,
    AssociateModelMetadataPath,
    AssociateModelMetadata,
    CreateModelMetadata,
    GetModelPath,
    IngestCanonicalModelPath,
    IngestCanonicalModelRequest,
};
pub use shared::presentation::http::v1::requests::discover_models::{
    DiscoverModelsByPlatformPath,
    DiscoverModelsByPlatformRequest,
    DiscoverModelsRequest,
    DiscoveryCriteria,
    DiscoverModelsQueryParams,
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
pub use shared::presentation::http::v1::responses::artifacts::publications::ArtifactPublication;
pub use shared::presentation::http::v1::requests::headers::Headers;