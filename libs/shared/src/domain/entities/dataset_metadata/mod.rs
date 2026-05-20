#[cfg(test)]
pub mod fixtures;

use crate::domain::entities::task::Task;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatasetMetadataError {
    #[error("Invalid or disallowed field path: {0:?}")]
    InvalidFieldPath(Vec<String>),
}

#[derive(Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String,
}

#[derive(Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>,
}

#[derive(Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct DatasetIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>,
}

#[derive(Clone)]
pub struct DatasetMetadata {
    // General fields
    pub name: Option<String>,
    pub author: Option<String>,
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    Name(Option<String>),
    Author(Option<String>),
}

impl Into<Value> for FieldValue {
    fn into(self) -> Value {
        match self {
            FieldValue::Name(name) => {
                match name {
                    Some(n) => Value::String(n),
                    None => Value::Null
                }
            },
            FieldValue::Author(author) => {
                match author {
                    Some(a) => Value::String(a),
                    None => Value::Null
                }
            }
        }
    }
}

impl DatasetMetadata {
    /// Fetches the value for a select number of field paths on the metdata struct.
    pub fn get_field_value_at_field_path(
        &self,
        field_path: &Vec<String>,
    ) -> Result<FieldValue, DatasetMetadataError> {
        let fp: Vec<&str> = field_path.iter().map(|v| v.as_str()).collect();
        match fp.as_slice() {
            ["name"] => Ok(FieldValue::Name(self.name.clone())),
            ["author"] => Ok(FieldValue::Author(self.author.clone())),
            other => {
                return Err(DatasetMetadataError::InvalidFieldPath(
                    other.to_vec().iter().map(|s| s.to_string()).collect(),
                ))
            }
        }
    }
}

#[cfg(test)]
#[path = "dataset_metadata.test.rs"]
mod dataset_metadata_test;
