pub mod indexes;

use mongodb::bson::{oid::ObjectId, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::model::{Model as DomainModel, ReconstituteModelProps},
    infra::persistence::mongo::documents::visibility::Visibility,
    shared_kernel::{identifiers::ExternalModelId, value_objects::TimeStamp},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tenant_id: String,
    pub owner: String,
    pub artifact_id: Option<Uuid>,
    pub external_model_id: Uuid,
    pub visibility: Visibility,
    pub updated_at: DateTime,
    pub created_at: DateTime,
}

impl From<&DomainModel> for Model {
    fn from(value: &DomainModel) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(*value.id().as_bytes()),
            name: value.name().into(),
            description: value.description().map(Into::into),
            tenant_id: value.tenant_id().into(),
            owner: value.owner().into(),
            artifact_id: value
                .artifact_id()
                .map(|id| Uuid::from_bytes(*id.as_bytes())),
            external_model_id: Uuid::from_bytes(*value.external_model_id().as_uuid().as_bytes()),
            visibility: value.visibility().clone().into(),
            updated_at: DateTime::from_millis(value.updated_at().into_inner().timestamp_millis()),
            created_at: DateTime::from_millis(value.created_at().into_inner().timestamp_millis()),
        }
    }
}

impl TryFrom<Model> for DomainModel {
    type Error = crate::domain::entities::model::ModelError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Self::reconstitute(ReconstituteModelProps {
            id: uuid::Uuid::from_bytes(value.id.bytes()),
            name: value.name,
            description: value.description,
            tenant_id: value.tenant_id,
            owner: value.owner,
            artifact_id: value
                .artifact_id
                .map(|id| uuid::Uuid::from_bytes(id.bytes())),
            external_model_id: ExternalModelId::reconstitute(uuid::Uuid::from_bytes(
                value.external_model_id.bytes(),
            )),
            visibility: value.visibility.into(),
            updated_at: TimeStamp::from(value.updated_at.to_chrono()),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
        })
    }
}

#[cfg(test)]
#[path = "model.test.rs"]
mod model_test;
