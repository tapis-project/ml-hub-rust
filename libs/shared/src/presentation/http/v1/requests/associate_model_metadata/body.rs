use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::Error;
use crate::application::inputs;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AssociateModelMetadataBody {
    pub name: String,
    pub author: String,
}

impl TryFrom<(&String, AssociateModelMetadataBody)> for inputs::model_metadata::AssociateModelMetadata {
    type Error = Error;

    fn try_from(value: (&String, AssociateModelMetadataBody)) -> Result<Self, Self::Error> {  
        let artifact_id= match Uuid::parse_str(&value.0) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Error::new("Value provided for artifact_id is not a UUID".into()))
        };

        Ok(
            Self {
                artifact_id,
                name: value.1.name,
                author: value.1.author,
            }
        )
    }
}