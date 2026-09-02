mod entity_to_response;

use crate::presentation::http::v1::responses::visibility::Visibility;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Dataset {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub tenant_id: String,
    pub owner: String,
    pub tags: Vec<String>,
    pub provider: DatasetProvider,
    #[schema(required = true, nullable)]
    pub huggingface_repo_locator: Option<HuggingFaceRepoLocator>,
    #[schema(required = true, nullable)]
    pub tapis_system_locator: Option<TapisSystemLocator>,
    /// Dataset items. Retrieval operations return at most the first 50 items in persisted order;
    /// registration returns every registered item.
    pub items: Vec<DatasetItem>,
    /// Total number of items in the complete Dataset before retrieval projection.
    pub item_count: u64,
    pub size: u64,
    pub visibility: Visibility,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct DatasetItem {
    pub path: String,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct HuggingFaceRepoLocator {
    pub id: String,
    pub sha: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct TapisSystemLocator {
    pub site_id: String,
    pub tenant_id: String,
    pub system_id: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum DatasetProvider {
    HuggingFace,
    Tapis,
}

#[cfg(test)]
#[path = "datasets.test.rs"]
mod datasets_test;
