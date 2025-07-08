use std::path::PathBuf;
use thiserror::Error;

use std::io::Write;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::logging::GlobalLogger;

#[derive(Debug, Error)]
pub enum StackingError {
    #[error("Error creating file: {0}")]
    StackingFileError(String),

    #[error("File I/O error: {0}")]
    IOError(String),
}

// A file compression utility that zips and compresses files and directories
pub struct FileStacker {}

impl FileStacker {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn stack(destination: &PathBuf, chunk: Vec<u8>) -> Result<(), StackingError> {
        GlobalLogger::debug(
            format!(
                "FILE STACKING (DESTINATION: {})",
                destination.clone().to_string_lossy().to_string()
            )
            .as_str(),
        );
        // 1. check if the parent directory exists, if not create it
        if let Some(parent_dir) = destination.parent() {
            tokio::fs::create_dir_all(parent_dir).await.map_err(|e| {
                StackingError::StackingFileError(format!("Fail to create dir: {}", e))
            })?;
        }

        // 2. use OpenOptions to open the file for writing
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it does not exist
            .append(true) // Append to the file if it exists
            .open(&destination)
            .await
            .map_err(|e| StackingError::StackingFileError(format!("Fail to open file: {}", e)))?;

        // 3. write the chunk to the file
        file.write_all(&chunk)
            .await
            .map_err(|e| StackingError::StackingFileError(format!("Fail to write: {}", e)))?;

        Ok(())
    }
}
