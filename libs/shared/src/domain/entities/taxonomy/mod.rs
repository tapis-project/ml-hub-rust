use derive_more::AsRef;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::domain::entities::annotations::Annotation;

pub struct Taxonomy {
    nodes: Vec<Node>
}

#[derive(AsRef)]
pub struct NodePath(Uuid);

pub struct Node {
    node_path: NodePath,
    schmea_id: Option<SchemaId>,
    children: Vec<NodePath>,
    annotation: Vec<Annotation>
}

pub struct Schema {
    id: SchemaId,
    name: String,
    description: Option<String>,
    tenant_id: String,
    schema_type: SchemaType,
    schema: Map<String, Value>,
    revision: u32,
    validation_policy: SchemaValidationPolicy,
}

pub struct SchemaId(String);

pub enum SchemaType {
    JsonSchema,
    // JsonStructure, // TODO support
}

pub enum SchemaValidationPolicy {
    ValidateOnWrite,
    SkipValidation,
}