pub use shared::presentation::http::v1::dto::models::{
    ListModelsPath,
    ListModelsRequest,
    GetModelPath,
    GetModelRequest,
    DiscoverModelsPath,
    DiscoverModelsRequest,
    DiscoveryCriteriaBody,
    IngestModelPath,
    IngestModelRequest,
    DownloadModelPath,
    DownloadModelRequest,
    UploadModelRequest,
    ModelMetadata,
    CreateModelMetadataPath,
    CreateModelMetadata
};
pub use shared::presentation::http::v1::dto::artifacts::{
    PublishArtifactPath,
    PublishArtifactRequest,
    PublishArtifactBody,
    IngestArtifactBody,
};
pub use shared::presentation::http::v1::dto::headers::Headers;