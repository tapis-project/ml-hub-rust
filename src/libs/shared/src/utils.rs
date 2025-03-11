use crate::errors::Error;
use crate::artifacts::Compression;
use crate::logging::GlobalLogger;
use std::process::Command;
use std::path::PathBuf;
use std::fs::File;
use zip::{ZipWriter, CompressionMethod};
use zip::write::SimpleFileOptions;
use zip_extensions::write::ZipWriterExtensions;

/// The Env struct contains data from system environment variables and vaules
/// derived from them that are required by the various services
pub struct Env {
    pub shared_data_dir: String,
    pub cache_dir: String
}

impl Env {
    pub fn new() -> Result<Self, Error> {
        // Shared data dir
        let shared_data_dir: String = match std::env::var("SHARED_DATA") {
            Ok(var) => var,
            Err(err) => Err(Error::new(err.to_string()))?
        };
        
        // Cache directory
        let cache_dir = format!("{}/{}", shared_data_dir, "cache");

        return Ok(
            Self {
                shared_data_dir,
                cache_dir
            }
        )
    }
}

pub type SystemPrograms<'a> = Vec<&'a str>;

pub fn validate_system_dependencies(programs: SystemPrograms) -> Result<(), Error> {
    let mut missing_programs: Vec<&str> = Vec::with_capacity(10);
    for program in programs {
        GlobalLogger::debug(format!("Checking system program '{}'", program).as_str());
        match Command::new(program).output() {
            Ok(_) => continue,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                GlobalLogger::error(format!("Missing system program '{}'", program).as_str());
                missing_programs.push(program);
                continue
            },
            Err(err) => {
                let msg = format!("An error has occured checking for program '{}': {}", program, err.to_string());
                GlobalLogger::error(&msg.as_str());
                Error::new(msg)
            }
        };
    }

    match missing_programs.len() > 0 {
        false => Ok(()),
        true => {
            let mut err_msg = String::from("Missing required system programs:");
            for program in missing_programs {
                err_msg.push_str(program)
            };
            GlobalLogger::error(&err_msg.as_str());
            Err(Error::new(err_msg))
        }
    }
}

pub fn zip(source: &PathBuf, destination: &PathBuf, compression: Option<Compression>) -> Result<PathBuf, Error> {
    let file = File::create(&destination)
        .map_err(|err| Error::new(err.to_string()))?;

    let zip = ZipWriter::new(file);

    let compression_method = match compression.unwrap_or(Compression::Deflated) {
        Compression::Deflated => CompressionMethod::Deflated,
    };
    let options = SimpleFileOptions::default().compression_method(compression_method);
    zip.create_from_directory_with_options(&source, |_| options)
        .map_err(|err| Error::new(err.to_string()))?;

    Ok(destination.clone())
}