use crate::application::inputs::principal::{FindByFederatedIdentity, GetOrCreateFromFederatedIdentity};
use crate::domain::entities::principal::Principal;
use crate::application::ports::principal::{PrincipalRepository, PrincipalRepositoryError};

use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, OnErrorAction, Retry, RetryPolicy};
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
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

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
        // Input for the repository call
        let find_by_identity_input = FindByFederatedIdentity { identity: input.identity.clone() };
        
        // The repo call to be passed into the retrier
        let find_by_identity_op = || self.principal_repository.find_by_identity(&find_by_identity_input);
        
        // A callback that determines which errors to retry on
        let on_error = |err: &PrincipalRepositoryError| -> OnErrorAction {
            match err {
                PrincipalRepositoryError::DomainError(_) |
                PrincipalRepositoryError::FederatedIdentityAlreadyOwned(..) |
                PrincipalRepositoryError::PrincipalAlreadyExists |
                PrincipalRepositoryError::ProgrammingError(_) => OnErrorAction::ReturnResult,
                PrincipalRepositoryError::PersistenceError { retriable, .. } => {
                    if *retriable {
                        return OnErrorAction::ContinueRetries
                    }

                    OnErrorAction::ReturnResult
                }
            }
        };

        let result_maybe_principal = retry_async(find_by_identity_op, &Self::REPO_RETRY_POLICY, on_error).await;
        // let maybe_principal = match result_maybe_principal {
        //     Ok(p) => p,
        //     Err(err) => {
        //         match err {
        //             PrincipalRepositoryError::DomainError(..) |
        //             PrincipalRepositoryError::FederatedIdentityAlreadyOwned(iss, sub) |
        //             PrincipalRepositoryError::PersistenceError { retriable, message } |
        //             PrincipalRepositoryError::PrincipalAlreadyExists |
        //         }
        //     }
        // };
        
        Err(PrincipalServiceError::InternalError("Not implemented".into()))
    }
}