use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceFieldValueCount {
    pub tenant_id: String,
    pub resource: String,
    pub resource_owner_id: String,
    pub field: String,
    pub value: String,
    pub count: u64,
}