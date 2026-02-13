#[cfg(test)]
pub mod fixtures;

use crate::domain::entities::task::Task;
use platforms::Platform;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ModelMetadataError {
    #[error("Invalid or disallowed field path: {0:?}")]
    InvalidFieldPath(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct SystemRequirement {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Accelerator {
    pub accelerator_type: String,
    pub memory_gb: Option<i32>,
    pub cores: Option<i32>,
    /// Firmware and software
    pub system_requirements: Vec<SystemRequirement>,
}

#[derive(Debug, Clone)]
pub struct HardwareRequirements {
    pub cpus: Option<i32>,
    pub memory_gb: Option<i32>,
    pub disk_gb: Option<i32>,
    pub accelerators: Option<Vec<Accelerator>>,
    pub architectures: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ModelIO {
    pub data_type: Option<String>,
    pub shape: Option<Vec<i32>>,
}

#[derive(Debug, Clone)]
pub struct Canonical {
    pub platform: Platform,
    pub model_id: String,
    pub locator: Locator,
    pub author: Option<String>,
    pub likes: Option<u128>,
    pub downloads: Option<u128>,
    pub gated: Option<bool>,
    pub private: Option<bool>,
    pub sha: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Locator {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ModelMetadata {
    // General fields
    pub name: Option<String>,
    pub author: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub image: Option<String>,
    pub artifact_id: Option<Uuid>,
    pub canonical: Option<Canonical>,

    /// Arbitrary labels
    pub keywords: Option<Vec<String>>,
    pub annotations: Option<Value>,

    /// Architecture fields
    pub multi_modal: Option<bool>,
    pub model_inputs: Option<Vec<ModelIO>>,
    pub model_outputs: Option<Vec<ModelIO>>,

    /// Inference Fields
    pub task_types: Option<Vec<Task>>,
    pub inference_precision: Option<String>,
    pub inference_hardware: Option<HardwareRequirements>,
    pub inference_software_dependencies: Option<Vec<String>>,
    pub inference_max_energy_consumption_watts: Option<i32>,

    /// Inference performance fields
    pub inference_max_latency_ms: Option<i32>,
    pub inference_min_throughput: Option<i32>,
    pub inference_max_compute_utilization_percentage: Option<i32>,
    pub inference_max_memory_usage_mb: Option<i32>,
    pub inference_distributed: Option<bool>,

    /// Training-related Fields
    pub training_time: Option<i64>,
    pub training_precision: Option<String>,
    pub training_hardware: Option<HardwareRequirements>,
    pub pretraining_datasets: Option<Vec<String>>,
    pub finetuning_datasets: Option<Vec<String>>,
    pub edge_optimized: Option<bool>,
    pub quantization_aware: Option<bool>,
    pub supports_quantization: Option<bool>,
    pub pretrained: Option<bool>,
    pub pruned: Option<bool>,
    pub slimmed: Option<bool>,
    pub training_distributed: Option<bool>,

    /// Training performance fields
    pub training_max_energy_consumption_watts: Option<i32>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    pub bias_evaluation_score: Option<i8>,
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    Name(Option<String>),
    Author(Option<String>),
    Libraries(Option<Vec<String>>),
    Keywords(Option<Vec<String>>),
    TaskTypes(Option<Vec<Task>>),
    InferenceHardwareMemory(Option<i32>),
    AnnotationsCanonicalPrivate(Option<bool>),
    AnnotationsCanonicalGated(Option<bool>),
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
            },
            FieldValue::Libraries(libraries) => {
                match libraries {
                    Some(fws) => {
                        fws.iter().map(|fw| Value::String(fw.clone())).collect()
                    },
                    None => Value::Null,
                }
            },
            FieldValue::Keywords(keywords) => {
                match keywords {
                    Some(kws) => {
                        kws.iter().map(|kw| Value::String(kw.clone())).collect()
                    },
                    None => Value::Null,
                }
            },
            FieldValue::TaskTypes(tasks) => {
                match tasks {
                    Some(ts) => {
                        ts.iter().map(|t| Value::String(String::from(t.clone()))).collect()
                    },
                    None => Value::Null,
                }
            },
            FieldValue::InferenceHardwareMemory(memory) => {
                match memory {
                    Some(m) => Value::Number(m.into()),
                    None => Value::Null
                }
            },
            FieldValue::AnnotationsCanonicalGated(gated) => {
                match gated {
                    Some(g) => Value::Bool(g),
                    None => Value::Null
                }
            },
            FieldValue::AnnotationsCanonicalPrivate(private) => {
                match private {
                    Some(p) => Value::Bool(p),
                    None => Value::Null
                }
            }
        }
    }
}

impl ModelMetadata {
    /// Fetches the value for a select number of field paths on the metdata struct.
    pub fn get_field_value_at_field_path(
        &self,
        field_path: &Vec<String>,
    ) -> Result<FieldValue, ModelMetadataError> {
        let fp: Vec<&str> = field_path.iter().map(|v| v.as_str()).collect();
        match fp.as_slice() {
            ["name"] => Ok(FieldValue::Name(self.name.clone())),
            ["author"] => Ok(FieldValue::Author(self.author.clone())),
            ["libraries"] => Ok(FieldValue::Libraries(self.libraries.clone())),
            ["keywords"] => Ok(FieldValue::Keywords(self.keywords.clone())),
            ["task_types"] => Ok(FieldValue::TaskTypes(self.task_types.clone())),
            ["inference_hardware", "memory_gb"] => Ok(FieldValue::InferenceHardwareMemory(
                self.inference_hardware.clone().and_then(|hr| hr.memory_gb),
            )),
            ["annotations", "canonical", "gated"] => Ok(FieldValue::AnnotationsCanonicalGated(
                self.annotations
                    .as_ref()
                    .and_then(|a| a.pointer("/canonical/gated"))
                    .and_then(|g| g.clone().as_bool())
            )),
            ["annotations", "canonical", "private"] => Ok(FieldValue::AnnotationsCanonicalPrivate(
                self.annotations
                    .as_ref()
                    .and_then(|a| a.pointer("/canonical/private"))
                    .and_then(|g| g.clone().as_bool())
            )),
            other => {
                return Err(ModelMetadataError::InvalidFieldPath(
                    other.to_vec().iter().map(|s| s.to_string()).collect(),
                ))
            }
        }
    }
}

#[cfg(test)]
#[path = "model_metadata.test.rs"]
mod model_metadata_test;
