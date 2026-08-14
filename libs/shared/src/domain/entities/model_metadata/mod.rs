/// Defines and manages the lifecycle of the ModelMetadata entity and all of its
/// sub-components. The ModelMetadata enety is a metadata representation of a machine learning
/// models.

#[cfg(test)]
pub mod fixtures;

use crate::shared_kernel::enums::Task;
use platforms::Platform;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;


/// ModelMetadata entity
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    // General fields
    pub name: String,
    pub author: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub artifact_id: Option<Uuid>,
    pub canonical: Option<Canonical>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
    
    // Viable deployment strategy references
    pub deployment_strategy_refs: Vec<DeploymentStrategyReference>
}

impl ModelMetadata {
    /// Fetches the value for a select number of field paths on the metadata struct.
    pub fn get_field_value_at_field_path(
        &self,
        field_path: &Vec<String>,
    ) -> Result<FieldValue, ModelMetadataError> {
        let fp: Vec<&str> = field_path.iter().map(|v| v.as_str()).collect();
        match fp.as_slice() {
            ["name"] => Ok(FieldValue::Name(Some(self.name.clone()))),
            ["author"] => Ok(FieldValue::Author(Some(self.author.clone()))),
            ["libraries"] => Ok(FieldValue::Libraries(self.libraries.clone())),
            ["tags"] => Ok(FieldValue::Tags(self.tags.clone())),
            ["task_types"] => Ok(FieldValue::TaskTypes(self.task_types.clone())),
            ["canonical", "gated"] => Ok(FieldValue::CanonicalGated(
                self.canonical
                    .as_ref()
                    .and_then(|c| c.gated)
            )),
            ["canonical", "private"] => Ok(FieldValue::CanonicalPrivate(
                self.canonical
                    .as_ref()
                    .and_then(|c| c.private)
            )),
            other => {
                return Err(ModelMetadataError::InvalidFieldPath(
                    other.to_vec().iter().map(|s| s.to_string()).collect(),
                ))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ModelMetadataError {
    #[error("Invalid or disallowed field path: {0:?}")]
    InvalidFieldPath(Vec<String>),
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

#[derive(Clone, Debug)]
pub struct DeploymentStrategyReference {
    pub name: String,
    pub platform: Platform,
}


#[derive(Clone, Debug)]
pub enum FieldValue {
    Name(Option<String>),
    Author(Option<String>),
    Libraries(Option<Vec<String>>),
    Tags(Option<Vec<String>>),
    TaskTypes(Option<Vec<Task>>),
    CanonicalPrivate(Option<bool>),
    CanonicalGated(Option<bool>),
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
            FieldValue::Tags(tags) => {
                match tags {
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
            FieldValue::CanonicalGated(gated) => {
                match gated {
                    Some(g) => Value::Bool(g),
                    None => Value::Null
                }
            },
            FieldValue::CanonicalPrivate(private) => {
                match private {
                    Some(p) => Value::Bool(p),
                    None => Value::Null
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "model_metadata.test.rs"]
mod model_metadata_test;
