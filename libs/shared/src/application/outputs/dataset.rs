use crate::{
    domain::entities::dataset::{DatasetItem, DatasetProvider},
    shared_kernel::{enums::Visibility, value_objects::Tags},
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DatasetListOutput {
    pub datasets: Vec<DatasetQueryOutput>,
    pub cursor: Option<String>,
    pub count: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct DatasetQueryOutput {
    pub id: Uuid,
    pub tenant_id: String,
    pub owner: String,
    pub tags: Tags,
    pub provider: DatasetProvider,
    pub items: Vec<DatasetItem>,
    pub item_count: u64,
    pub size: u64,
    pub visibility: Visibility,
}
