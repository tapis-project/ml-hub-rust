pub mod git;

use crate::errors::Error;
use crate::presentation::http::v1::dto::Compression;
use crate::logging::GlobalLogger;
use std::path::PathBuf;
use std::fs::File;
use zip::{ZipWriter, CompressionMethod};
use zip::write::SimpleFileOptions;
use zip_extensions::write::ZipWriterExtensions;

pub fn zip(
    source: &PathBuf,
    destination: &PathBuf,
    compression: Option<Compression>,
) -> Result<PathBuf, Error> {
    let file = File::create(&destination)
        .map_err(|err| {
            let msg = err.to_string();
            GlobalLogger::error(format!("Error creating file to zip at path '{}'", &destination.to_string_lossy()).as_str());
            Error::new(msg)
        })?;

    let zip = ZipWriter::new(file);

    let compression_method = match compression.unwrap_or(Compression::Deflated) {
        Compression::Deflated => CompressionMethod::Deflated,
    };
    let options = SimpleFileOptions::default().compression_method(compression_method);
    zip.create_from_directory_with_options(&source, |_| options)
        .map_err(|err| Error::new(err.to_string()))?;

    Ok(destination.clone())
}