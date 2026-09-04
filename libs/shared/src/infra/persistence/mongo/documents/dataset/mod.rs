mod document_to_entity;
mod entity_to_document;
mod query_document_to_output;

use crate::infra::persistence::mongo::documents::visibility::Visibility;
use mongodb::bson::{oid::ObjectId, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub tenant_id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub provider: DatasetProvider,
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    pub tapis_system_locator: Option<TapisSystemLocator>,
    pub items: Vec<DatasetItem>,
    pub size: u64,
    pub visibility: Visibility,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DatasetProvider {
    HuggingFace,
    Tapis,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HuggingFaceRepoLocator {
    pub id: String,
    pub sha: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TapisSystemLocator {
    pub site_id: String,
    pub tenant_id: String,
    pub system_id: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetItem {
    pub path: String,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatasetQuery {
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub tenant_id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub provider: DatasetProvider,
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    pub tapis_system_locator: Option<TapisSystemLocator>,
    pub items: Vec<DatasetItem>,
    pub item_count: i64,
    pub size: u64,
    pub visibility: Visibility,
}

#[cfg(test)]
#[path = "dataset.test.rs"]
mod dataset_test;
