use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artifact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub path: Option<String>,
    pub artifact_type: ArtifactType,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateArtifactPathRequest {
    pub path: String,
    pub last_modified: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateArtifactRequest {
    pub path: String,
    pub last_modified: DateTime,
}