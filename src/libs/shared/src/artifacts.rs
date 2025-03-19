use crate::errors::Error;
use crate::logging::GlobalLogger;
use crate::system::Env;
use crate::archive::zip;
use std::path::PathBuf;
use std::fs;
use strum_macros::{EnumString, Display};
use serde::Deserialize;
// Reexport to create a unified api for all artifact-related functionality
pub use crate::responses::artifact_helpers;

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum Archive {
    #[strum(serialize="zip")]
    Zip
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Deserialize, Display, EnumString)]
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

pub trait ArtifactGenerator {}

pub trait ArtifactStager {
    fn stage(&self, params: ArtifactStagingParams) -> Result<StagedArtifact, Error>;
}

impl<T: ArtifactGenerator> ArtifactStager for T {
    fn stage(&self, params: ArtifactStagingParams) -> Result<StagedArtifact, Error> {
        let env = Env::new()?;
        let cache_dir = PathBuf::from(env.cache_dir);
        let artifact = params.artifact.clone();
        let source = PathBuf::new().join(artifact.path.as_str());
        
        let mut destination = cache_dir;
        if params.archive.is_some() {
            destination = destination.join(params.staged_filename.unwrap_or(String::from("artifact")))
        }

        // Shadowing here because the archive utilities expect a reference to some
        // path, not a mutable reference
        let destination = destination;

        let staged_path = match params.archive {
            Some(Archive::Zip) => {
                zip(
                    &source,
                    &destination,
                    params.compression,
                )?
            },
            None => {
                fs::rename(&source, &destination)
                    .map_err(|err| Error::new(err.to_string()))?;

                PathBuf::from(&destination)
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