use crate::application::inputs::principal::{FindByFederatedIdentity, GetOrCreateFromFederatedIdentity};
use crate::domain::entities::principal::{NewUserPrincipalProps, Principal, PrincipalError};
use crate::application::ports::principal::{PrincipalRepository, PrincipalRepositoryError};

use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, RetryStrategyAction, Retry, RetryPolicy};
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

    pub fn new(principal_repository: Arc<dyn PrincipalRepository>) -> Self {
        Self {
            principal_repository
        }
    }

    async fn save(&self, principal: Principal) -> Result<(), PrincipalServiceError> {
        self.principal_repository.save(&principal).await
            .map_err(|err| PrincipalServiceError::from(err))
    }

    pub async fn get_or_create_from_identity(&self, input: GetOrCreateFromFederatedIdentity) -> Result<Principal, PrincipalServiceError> {
        // Input for the repository call
        let find_by_identity_input = FindByFederatedIdentity { identity: input.identity.clone() };
        
        // The repo call to be passed into the retrier
        let find_by_identity_op = || self.principal_repository.find_by_identity(&find_by_identity_input);
        
        // A closure that determines how to handle errors for each retry attempt
        let retry_strategy = |err: &PrincipalRepositoryError, _attempt| -> RetryStrategyAction {
            match err {
                PrincipalRepositoryError::DomainError(_) |
                PrincipalRepositoryError::FederatedIdentityAlreadyOwned(..) |
                PrincipalRepositoryError::PrincipalAlreadyExists |
                PrincipalRepositoryError::ProgrammingError(_) => RetryStrategyAction::ReturnResult,
                PrincipalRepositoryError::PersistenceError { retriable, .. } => {
                    if *retriable {
                        return RetryStrategyAction::ContinueRetries
                    }

                    RetryStrategyAction::ReturnResult
                }
            }
        };

        let maybe_principal = retry_async(find_by_identity_op, &Self::REPO_RETRY_POLICY, retry_strategy).await?;

        if let Some(p) = maybe_principal {
            return Ok(p)
        };

        let props = NewUserPrincipalProps {
            id: input.principal_id,
            tenant_id: input.identity.tenant_id.clone(),
            identity: input.identity
        };

        let new_principal = Principal::new_user(props)?;
        
        self.save(new_principal.clone()).await?;

        return Ok(new_principal)
    }
}

impl From<PrincipalError> for PrincipalServiceError {
    fn from(value: PrincipalError) -> Self {
        match value {
            PrincipalError::TenantMismatch => PrincipalServiceError::InternalError(value.to_string())
        }
    }
}

impl From<PrincipalRepositoryError> for PrincipalServiceError {
    fn from(value: PrincipalRepositoryError) -> Self {
        match value {
            PrincipalRepositoryError::DomainError(err) => {
                PrincipalServiceError::InternalError(err.to_string())
            },
            PrincipalRepositoryError::ProgrammingError(msg) |
            PrincipalRepositoryError::PersistenceError { message: msg, .. } => PrincipalServiceError::InternalError(msg),
            PrincipalRepositoryError::FederatedIdentityAlreadyOwned(sub, iss) => PrincipalServiceError::FederatedIdentityConflict(iss, sub),
            PrincipalRepositoryError::PrincipalAlreadyExists => PrincipalServiceError::PrincipalConflict,
        }
    }
}