pub use shared::presentation::http::v1::requests::models::{
    ListModelsPath,
    ListModelsRequest,
    GetModelPath,
    GetModelRequest,
    DiscoverModelsPath,
    DiscoverModelsRequest,
    DiscoveryCriteria,
    IngestModelPath,
    IngestModelRequest,
    DownloadModelPath,
    DownloadModelRequest,
    UploadModelRequest,
    ModelMetadata,
    CreateModelMetadataPath,
    CreateModelMetadata
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