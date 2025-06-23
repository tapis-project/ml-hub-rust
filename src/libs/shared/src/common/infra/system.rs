use crate::logging::GlobalLogger;
// Reexporting for continuity as this module is for code that accesses
// system software. Both git and git lfs are called via Command
pub use crate::common::infra::fs::git;
use std::process::Command;
use std::path::PathBuf;
use std::fs::create_dir_all;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Missing environment variable '{0}'")]
    MissingEnvVar(String),

    #[error("Missing system program '{0}'")]
    MissingSystemProgram(String),

    #[error("{0}")]
    CacheDirError(String),

    #[error("File system error: {0}")]
    FileSystemError(String)
}

/// The Env struct contains data from system environment variables and vaules
/// derived from them that are required by the various services
pub struct Env {
    pub shared_data_dir: String,
    pub cache_dir: String,
}

impl Env {
    pub fn new() -> Result<Self, SystemError> {
        // Shared data dir
        let shared_data_dir: String = match std::env::var("SHARED_DATA") {
            Ok(var) => var,
            Err(_) => Err(SystemError::MissingEnvVar("SHARED_DATA".into()))?
        };
        
        // Cache directory
        let cache_dir = format!("{}/{}", shared_data_dir, "cache");

        let dirs: Vec<&String> = vec![&shared_data_dir, &cache_dir];
        for dir in dirs {
            if !PathBuf::from(dir).exists() {
                create_dir_all(dir)
                    .map_err(|err| SystemError::MissingEnvVar(err.to_string()))?;
            }
        }

        return Ok(
            Self {
                shared_data_dir,
                cache_dir
            }
        )
    }
}

pub type SystemPrograms<'a> = Vec<&'a str>;

pub fn validate_system_dependencies(programs: SystemPrograms) -> Result<(), SystemError> {
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
                SystemError::FileSystemError(msg)
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
            Err(SystemError::MissingSystemProgram(err_msg))
        }
    }
}