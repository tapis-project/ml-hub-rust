use serde_json::Value;
use uuid::Uuid;

use crate::domain::entities::taxonomy::NodePath;

pub struct Annotation {
    pub name: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub payload: Value,
    pub node_path: NodePath,
    pub uuid: Uuid,
}