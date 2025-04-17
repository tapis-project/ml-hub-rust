use crate::errors::Error;
use crate::logging::GlobalLogger;
use crate::system::Env;
use crate::archive::zip;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::future::Future;
use strum_macros::{EnumString, Display};
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt;
use actix_web::web::Bytes;
use actix_multipart::Multipart;
use uuid::Uuid;
// Reexport to create a unified api for all artifact-related functionality
pub use crate::responses::artifact_helpers;

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    #[strum(serialize="zip")]
    Zip
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    #[strum(serialize="deflated")]
    Deflated
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub path: String,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct StagedArtifact {
    pub path: PathBuf,
    pub artifact: Artifact
}

pub struct ArtifactStagingParams<'a> {
    pub artifact: &'a Artifact,
    pub staged_filename: Option<String>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>
}

pub struct MultipartStagingParams<'payload> {
    pub payload: &'payload mut Multipart,
    pub staged_filename: String,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>
}

pub trait ArtifactGenerator {}

pub trait ArtifactStager {
    fn stage(&self, params: ArtifactStagingParams) -> Result<StagedArtifact, Error>;
    fn stage_multipart(&self, params: MultipartStagingParams) -> impl Future<Output = Result<StagedArtifact, Error>>;
}

impl<T: ArtifactGenerator> ArtifactStager for T {
    async fn stage_multipart(&self, params: MultipartStagingParams<'_>) -> Result<StagedArtifact, Error> {
        let tmp_dir = format!("tmp/artifacts/{}", Uuid::now_v7());
        while let Some(item) = params.payload.next().await {
            let mut field = item
                .map_err(|err| Error::new(err.to_string()))?;

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

            let mut file = File::create(file_path)
                .map_err(|err| Error::new(err.to_string()))?;

            // Write each chunk to the file
            while let Some(chunk) = field.next().await {
                let data: Bytes = chunk
                    .map_err(|err| Error::new(err.to_string()))?;
                
                file.write_all(&data)
                    .map_err(|err| Error::new(err.to_string()))?;
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

    fn stage(&self, params: ArtifactStagingParams) -> Result<StagedArtifact, Error> {
        let env = Env::new()?;
        let cache_dir = PathBuf::from(env.cache_dir);
        let artifact = params.artifact.clone();
        let source = PathBuf::new().join(artifact.path.as_str());
        
        let mut target = cache_dir;
        if params.archive.is_some() {
            target = target.join(params.staged_filename.unwrap_or(String::from("artifact")))
        }

        // Shadowing here because the archive utilities expect a reference to some
        // path, not a mutable reference
        let target = target;

        let staged_path = match params.archive {
            Some(Archive::Zip) => {
                zip(
                    &source,
                    &target,
                    params.compression,
                )?
            },
            None => {
                fs::rename(&source, &target)
                    .map_err(|err| Error::new(err.to_string()))?;

                PathBuf::from(&target)
            }
        };
        
        // Clean up all of the workdir
        fs::remove_dir_all(&source)
            .map_err(|err| {
                let msg = format!("Error occured while attempting to remove the prestaged artifact data from path: {}", err.to_string());
                GlobalLogger::error(&msg.as_str());
                Error::new(msg)
            })?;

        Ok(StagedArtifact {
            path: staged_path,
            artifact,
        })
    }
}