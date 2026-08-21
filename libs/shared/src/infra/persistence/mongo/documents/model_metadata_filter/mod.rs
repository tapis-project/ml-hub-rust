mod input_to_document;

use serde::{Serialize, Deserialize};
use mongodb::bson::Document;

use crate::infra::persistence::mongo::documents::task::Task;
use crate::infra::persistence::mongo::utils::is_vec_empty;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelMetadataFilter {
    // General fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<String>>,

    // Tenancy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<TenancySelector>,

    /// Arbitrary labels
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub task_types: Option<Vec<Task>>,

    /// Regulatory and Compliance Fields
    /// A vector or strings that represent regulatory standards. Ex HIPPA
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub regulatory: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

pub type TenancySelector = Document;