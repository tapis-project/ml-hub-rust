use crate::domain::entities::{dataset_metadata::{HardwareRequirements, DatasetIO, DatasetMetadata, Accelerator, SystemRequirement}, task::Task};
use serde_json::json;

pub fn full_dataset_metadata() -> DatasetMetadata {
    DatasetMetadata {
        name: Some("foo".into()),
        author: Some("bar".into()),
    }
}