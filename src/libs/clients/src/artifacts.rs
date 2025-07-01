use shared::common::infra::fs::compression::CompressionError;
use shared::common::infra::system::SystemError;
use std::fs::File;
use std::io::Write;
use std::future::Future;
use bytes::Bytes;
use uuid::Uuid;
// TODO Should these be presentation-layer dtos? idk
use shared::common::presentation::http::v1::dto::{
    ArtifactStagingParams,
    StagedArtifact,
    MultipartStagingParams,
    Compression,
    Archive,
    Artifact
};
use futures_util::stream::StreamExt;
use thiserror::Error;
use std::io::Error as IOError;

// Reexport to create a unified api for all artifact-related functionality
pub use shared::common::presentation::http::v1::responses::artifact_helpers;

#[derive(Debug, Error)]
pub enum ArtifactStagingError {
    #[error("I/O error while staging artifact: {0}")]
    IOError(#[from] IOError),

    #[error("Error streaming artifact bytes: {0}")]
    ByteStreamError(String),

    #[error("Error compressing artifact: {0}")]
    CompressionError(#[from] CompressionError),

    #[error("System error while staging artifact: {0}")]
    SystemError(#[from] SystemError),
}

pub trait ArtifactGenerator {}

pub trait ArtifactStager {
    fn stage(&self, params: ArtifactStagingParams) -> Result<StagedArtifact, ArtifactStagingError>;
    fn stage_multipart(&self, params: MultipartStagingParams) -> impl Future<Output = Result<StagedArtifact, ArtifactStagingError>>;
}

impl<T: ArtifactGenerator> ArtifactStager for T {
    async fn stage_multipart(&self, params: MultipartStagingParams<'_>) -> Result<StagedArtifact, ArtifactStagingError> {
        let tmp_dir = format!("tmp/artifacts/{}", Uuid::now_v7());
        while let Some(item) = params.payload.next().await {
            let mut field = item
                .map_err(|err| ArtifactStagingError::ByteStreamError(err.to_string()))?;

            // Get filename from content disposition
            let default_filename = String::from(Uuid::now_v7().to_string());
            let mut filename = default_filename.clone();
            let maybe_content_disposition = field.content_disposition();
            if let Some(contend_disposition) = maybe_content_disposition {
                filename = contend_disposition.get_filename()
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| default_filename);
            }

            let file_path = format!("{}/{}", tmp_dir, filename);

            let mut file = File::create(file_path)?;

            // Write each chunk to the file
            while let Some(chunk) = field.next().await {
                let data: Bytes = chunk
                    .map_err(|err| ArtifactStagingError::ByteStreamError(err.to_string()))?;
                
                file.write_all(&data)?;
            }
        }

        let artifact = Artifact {
            path: tmp_dir,
            include_paths: None,
            exclude_paths: None
        };

        let params = ArtifactStagingParams {
            artifact: &artifact,
            staged_filename: Some(params.staged_filename),
            archive: Some(Archive::Zip),
            compression: Some(Compression::Deflated)
        };
        
        self.stage(params)
    }

    // TODO remove
    fn stage(&self, _params: ArtifactStagingParams) -> Result<StagedArtifact, ArtifactStagingError> {
        unimplemented!("Staging with the ArtifactStage is no longer available")
        // let env = Env::new()?;
        // let cache_dir = PathBuf::from(env.artifacts_cache_dir);
        // let artifact = params.artifact.clone();
        // let source = PathBuf::new().join(artifact.path.as_str());
        
        // let mut target = cache_dir;
        // if params.archive.is_some() {
        //     target = target.join(params.staged_filename.unwrap_or(String::from("artifact")))
        // }

        // // Shadowing here because the archive utilities expect a reference to some
        // // path, not a mutable reference
        // let target = target;

        // let staged_path = match params.archive {
        //     Some(Archive::Zip) => {
        //         FileCompressor::zip(
        //             &source,
        //             &target,
        //             params.compression,
        //         )?
        //     },
        //     None => {
        //         fs::rename(&source, &target)?;

        //         PathBuf::from(&target)
        //     }
        // };
        
        // // Clean up all of the workdir
        // fs::remove_dir_all(&source)?;

        // Ok(StagedArtifact {
        //     path: staged_path,
        //     artifact,
        // })
    }
}