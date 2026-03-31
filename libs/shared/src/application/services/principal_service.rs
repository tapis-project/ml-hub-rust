use crate::application::inputs::principal::GetOrCreateFromFederatedIdentity;
use crate::domain::entities::identity::FederatedIdentity;
use crate::domain::entities::principal::Principal;
use crate::application::ports::principal::{PrincipalRepository, PrincipalRepositoryError};

use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PrincipalServiceError {
    #[error("Principal already exists")]
    PrincipalConflict,

    #[error("Federated Identity {1} from issuer {0} already belongs to another principal")]
    FederatedIdentityConflict(String, String),

    #[error("Unexpected internal error: {0}")]
    InternalError(String),
}

pub struct PrincipalService {
    principal_repository: Arc<dyn PrincipalRepository>
}

impl PrincipalService {
    pub async fn save(&self, principal: Principal) -> Result<(), PrincipalServiceError> {
        match self.principal_repository.save(&principal).await {
            Ok(_) => Ok(()),
            Err(err) => {
                match err {
                    PrincipalRepositoryError::PrincipalAlreadyExists => Err(PrincipalServiceError::PrincipalConflict),
                    PrincipalRepositoryError::FederatedIdentityAlreadyOwned(iss, sub) => Err(PrincipalServiceError::FederatedIdentityConflict(iss, sub)),
                    PrincipalRepositoryError::PersistenceError { message, .. } |
                    PrincipalRepositoryError::ProgrammingError(message) => Err(PrincipalServiceError::InternalError(message)),
                    PrincipalRepositoryError::DomainError(err) => Err(PrincipalServiceError::InternalError(err.to_string())),
                }
            }
        }
    }

    pub async fn get_or_create_from_identity(&self, input: GetOrCreateFromFederatedIdentity) -> Result<Principal, PrincipalServiceError> {
        Err(PrincipalServiceError::InternalError("Not implemented".into()))
    }
}