// TODO Refactor: Should not be a dto. Needs mappings through to the
// infra layer
use crate::common::presentation::http::v1::dto::Compression;
use std::path::PathBuf;
use std::fs::File;
use zip::{ZipWriter, CompressionMethod};
use zip::write::SimpleFileOptions;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Error creating file: {0}")]
    CompressionFileError(String),

    #[error("File I/O error: {0}")]
    IOError(String),

    #[error("Error zipping path: {0}")]
    ZipError(String),
}

// A file compression utility that zips and compresses files and directories
pub struct FileCompressor {}

impl FileCompressor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn zip(
        source: &PathBuf,
        destination: &PathBuf,
        compression: Option<Compression>,
    ) -> Result<PathBuf, CompressionError> {
        let file = Self::create_compression_file(destination)?;

        let mut writer = ZipWriter::new(file);
    
        let compression_method = match compression.unwrap_or(Compression::Deflated) {
            Compression::Deflated => CompressionMethod::Deflated,
        };

        let options = SimpleFileOptions::default()
            .compression_method(compression_method);

        match source.is_dir() {
            true => Self::zip_dir(&mut writer, options, source)?,
            false => Self::zip_file(&mut writer, options, source)?
        }

        writer.finish().map_err(|err| CompressionError::ZipError(err.to_string()))?;
    
        Ok(destination.clone())
    }

    fn zip_dir(writer: &mut ZipWriter<File>, options: SimpleFileOptions,  path: &PathBuf) -> Result<(), CompressionError> {
        writer.add_directory_from_path(path, options)
            .map_err(|err| CompressionError::ZipError(err.to_string()))?;
        
        let entries = std::fs::read_dir(path)
            .map_err(|err| CompressionError::IOError(err.to_string()))?;
        
        for maybe_entry in entries {
            let entry = maybe_entry
                .map_err(|err| CompressionError::IOError(err.to_string()))?;

            match entry.path().is_dir() {
                true => Self::zip_dir(writer, options, &entry.path())?,
                false => Self::zip_file(writer, options, &entry.path())?
            }
        }

        Ok(())
    }

    fn zip_file(writer: &mut ZipWriter<File>, options: SimpleFileOptions,  path: &PathBuf) -> Result<(), CompressionError> {
        let mut file = File::open(path)
            .map_err(|err| CompressionError::CompressionFileError(err.to_string()))?;

        writer.start_file_from_path(path, options)
            .map_err(|err| CompressionError::ZipError(err.to_string()))?;

        std::io::copy(&mut file, writer)
            .map_err(|err| CompressionError::IOError(err.to_string()))?;

        Ok(())
    }

    fn create_compression_file(destination: &PathBuf) -> Result<File, CompressionError> {
        let file = File::create(&destination)
            .map_err(|err| {
                let msg = err.to_string();
                CompressionError::CompressionFileError(msg)
            })?;

        Ok(file)
    }
}

// Unit tests
// This test is ignored by default, as it requires a specific file structure and may not be suitable for all environments.
#[cfg(test)]#[ignore]
#[path = "compression.test.rs"]
mod compression_test;