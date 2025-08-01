// TODO Refactor: Should not be a dto. Needs mappings through to the
// infra layer
use crate::common::presentation::http::v1::dto::Compression;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{Read, Seek};
use zip::{ZipWriter, CompressionMethod, ZipArchive};
use zip::write::SimpleFileOptions;
use zip::read::ZipFile;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Error creating file: {0}")]
    CompressionFileError(String),

    #[error("Error stripping prefix `{0}` from path: {1}")]
    BasePathError(String, String),

    #[error("File I/O error: {0}")]
    IOError(String),

    #[error("Error zipping path: {0}")]
    ZipError(String),
}

// A utility that archives and optionally compresses files and directories
pub struct Archiver {}

impl Archiver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn zip(
        source: &PathBuf,
        destination: &PathBuf,
        compression: Option<Compression>,
        base_path: Option<&str>
    ) -> Result<PathBuf, CompressionError> {
        let file = Self::create_compression_file(destination)?;

        let mut writer = ZipWriter::new(file);
    
        let compression_method = match compression.unwrap_or(Compression::Deflated) {
            Compression::Deflated => CompressionMethod::Deflated,
        };

        let options = SimpleFileOptions::default()
            .compression_method(compression_method);

        match source.is_dir() {
            true => Self::zip_dir(&mut writer, options, source, base_path)?,
            false => Self::zip_file(&mut writer, options, source, base_path)?
        }

        writer.finish().map_err(|err| CompressionError::ZipError(err.to_string()))?;
    
        Ok(destination.clone())
    }

    fn zip_dir(writer: &mut ZipWriter<File>, options: SimpleFileOptions,  path: &PathBuf, base_path: Option<&str>) -> Result<(), CompressionError> {
        // The prefix that will be stripped from the directory name being written
        let prefix = base_path.unwrap_or_else(|| "");

        // Strip the prefix. Will strip nothing if no base_path provided. In the
        // event nothing is stripped, the modified_path will be equal to the
        // provided path
        let modified_path = path.strip_prefix(prefix)
            .map_err(|err| CompressionError::BasePathError(prefix.into(), err.to_string()))?;
        
        let dir_path = modified_path.to_string_lossy().into_owned();
        
        writer.add_directory_from_path(dir_path, options)
            .map_err(|err| CompressionError::ZipError(err.to_string()))?;
        
        let entries = fs::read_dir(path)
            .map_err(|err| CompressionError::IOError(err.to_string()))?;
        
        for maybe_entry in entries {
            let entry = maybe_entry
                .map_err(|err| CompressionError::IOError(err.to_string()))?;

            match entry.path().is_dir() {
                true => Self::zip_dir(writer, options, &entry.path(), base_path)?,
                false => Self::zip_file(writer, options, &entry.path(), base_path)?
            }
        }

        Ok(())
    }

    fn zip_file(writer: &mut ZipWriter<File>, options: SimpleFileOptions,  path: &PathBuf, base_path: Option<&str>) -> Result<(), CompressionError> {
        let mut file = File::open(path)
            .map_err(|err| CompressionError::CompressionFileError(err.to_string()))?;

        // The prefix that will be stripped from the file name being written
        let prefix = base_path.unwrap_or_else(|| "");

        // Strip the prefix. Will strip nothing if no base_path provided. In the
        // event nothing is stripped, the modified_path will be equal to the
        // provided path
        let modified_path = path.strip_prefix(prefix)
            .map_err(|err| CompressionError::BasePathError(prefix.into(), err.to_string()))?;
        
        let file_path = modified_path.to_string_lossy().into_owned();

        writer.start_file(file_path, options)
            .map_err(|err| CompressionError::ZipError(err.to_string()))?;

        std::io::copy(&mut file, writer)
            .map_err(|err| CompressionError::IOError(err.to_string()))?;

        Ok(())
    }

    pub fn unzip(
        source: &PathBuf,
        destination: &PathBuf,
        password: Option<&str>,
    ) -> Result<PathBuf, CompressionError> {
        // if the destination directory does not exist, create it
        if !destination.exists() {
            fs::create_dir_all(destination)
                .map_err(|e| CompressionError::IOError(e.to_string()))?;
        }

        let file   = File::open(source)
            .map_err(|e| CompressionError::IOError(e.to_string()))?;

        Self::unzip_file(file, destination, password)
            .map_err(|e| CompressionError::ZipError(e.to_string()))?;

        Ok(destination.clone())
    }

    fn unzip_file<R: Read + Seek>(
        file: R,
        destination: &PathBuf,
        password: Option<&str>,
    ) -> Result<(), CompressionError> {
        let mut archive = ZipArchive::new(file)
            .map_err(|e| CompressionError::ZipError(e.to_string()))?;

        (0..archive.len()).try_for_each(|i| -> Result<(), CompressionError> {
            let mut entry = match password {
                Some(pw) => archive.by_index_decrypt(i, pw.as_bytes())
                    .map_err(|e| CompressionError::ZipError(e.to_string()))?,
                None => archive.by_index(i)
                    .map_err(|e| CompressionError::ZipError(e.to_string()))?,
            };

            Self::extract_entry(&mut entry, destination)
        })
    }

    fn extract_entry<R: Read + Seek>(
        entry: &mut ZipFile<R>,
        destination: &PathBuf,
    ) -> Result<(), CompressionError>{
        let out_path = destination.join(entry.name());
        let name = entry.name();
        // with end of the name, we can check if it is a directory or zip or just a file
        match () {
            // ──────────────── 1) dir ────────────────
            _ if name.ends_with('/') => {
                fs::create_dir_all(&out_path)
                    .map_err(|e| CompressionError::IOError(e.to_string()))
            }

            // ──────────────── 3) zip ────────────────
            _ => {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| CompressionError::IOError(e.to_string()))?;
                }
                let mut outfile = File::create(&out_path)
                    .map_err(|e| CompressionError::IOError(e.to_string()))?;
                std::io::copy(entry, &mut outfile)
                    .map_err(|e| CompressionError::IOError(e.to_string()))?;
                Ok(())
            }
        }?;
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
#[cfg(test)]
#[path = "archiver.test.rs"]
mod archiver_test;