// TODO Refactor: Should not be a dto. Needs mappings through to the
// infra layer
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{Read, Seek, Cursor};
use zip::{ZipWriter, CompressionMethod, ZipArchive};
use zip::write::SimpleFileOptions;
use zip::read::ZipFile;
use thiserror::Error;

use crate::logging::GlobalLogger;


#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Error creating file: {0}")]
    CompressionFileError(String),

    #[error("File I/O error: {0}")]
    IOError(String),

    #[error("Error zipping path: {0}")]
    ZipError(String),
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Archive {
    Zip
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Compression {
    Deflated
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
        GlobalLogger::debug(format!("SOURCE: {}", source.clone().to_string_lossy().to_string()).as_str());
        GlobalLogger::debug(format!("DESTINATION: {}", destination.clone().to_string_lossy().to_string()).as_str());
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

    fn zip_dir(writer: &mut ZipWriter<File>, options: SimpleFileOptions,  path: &PathBuf) -> Result<(), CompressionError> {
        writer.add_directory_from_path(path, options)
            .map_err(|err| CompressionError::ZipError(err.to_string()))?;
        
        let entries = fs::read_dir(path)
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

            // ──────────────── 2) zip ─────────────────
            _ if name.to_ascii_lowercase().ends_with(".zip") => {
                let mut buffer = Vec::with_capacity(entry.size() as usize);
                entry.read_to_end(&mut buffer)
                    .map_err(|e| CompressionError::IOError(e.to_string()))?;

                // After one layer of unzipping, password is not applied
                Self::unzip_file(Cursor::new(buffer), &out_path, None)
                    .map_err(|e| CompressionError::ZipError(e.to_string()))
            }

            // ──────────────── 3) other ────────────────
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
#[path = "compression.test.rs"]
mod compression_test;