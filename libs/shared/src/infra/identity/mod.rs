pub mod tapis;
pub mod mongo;
mod helpers;

use strum_macros::{EnumString, Display};
use serde::Deserialize;
use thiserror::Error;

use crate::domain::entities::identity::FederatedIdentity;


#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Idp {
    Tapis
}

#[derive(Debug, Clone, Error)]
pub enum IdpError {
    #[error("Failed to resolve the principal's id from federated identity: {0}")]
    ErrorResolvingPrincipalId(String)
}

impl Idp {
    pub fn all() -> Vec<Idp> {
        vec![
            Self::Tapis
        ]
    }

    pub fn resolve_principal_id(&self, identity: &FederatedIdentity) -> Result<String, IdpError> {
        match self {
            Self::Tapis => {
                if let Some((id, _)) = identity.subject.clone().rsplit_once("@") {
                    let principal_id = String::from(id);
                    return Ok(principal_id)
                }

                return Err(IdpError::ErrorResolvingPrincipalId("Malformed subject".into()))
            }
        }
    }
}