use std::time::Duration;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, Clone)]
pub enum CommonRepositoryError {
    #[error("Temporary infrastructure failure: {reason}")]
    Transient {
        error_id: Uuid,
        reason: String,
        retry_after: Option<Duration>
    },

    #[error("An an unexpected internal error occurred. Error ID: {error_id}")]
    InternalError { error_id: Uuid },
}

impl CommonRepositoryError {
    pub fn new_internal() -> Self {
        Self::InternalError {
            error_id: Self::gen_error_id(),
        }
    }

    pub fn new_transient() -> Self {
        Self::InternalError {
            error_id: Self::gen_error_id(),
        }
    }

    pub fn error_id(&self) -> uuid::Uuid {
        match self {
            Self::InternalError { error_id } |
            Self::Transient { error_id, .. } => *error_id,
        }
    }

    fn gen_error_id() -> Uuid {
        uuid::Uuid::now_v7()
    }
}

