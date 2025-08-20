use thiserror::Error;
use shared::infra::fs::git::GitError;

#[derive(Debug)]
pub enum ClientErrorScope {
    Client,
    Server
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("{scope:?} error (Internal Error): {msg}")]
    Internal {
        msg: String,
        scope: ClientErrorScope
    },
    
    #[error("{scope:?} error (Unauthorized): {msg}")]
    Unauthorized {
        msg: String,
        scope: ClientErrorScope
    },
    
    #[error("{scope:?} error (Forbidden): {msg}")]
    Forbidden {
        msg: String,
        scope: ClientErrorScope
    },
    
    #[error("{scope:?} error (Not Found): {msg}")]
    NotFound {
        msg: String,
        scope: ClientErrorScope
    },
    
    #[error("{scope:?} error (Bad Request): {msg}")]
    BadRequest {
        msg: String,
        scope: ClientErrorScope
    },

    #[error("Unimplemented")]
    Unimplemented,

    // For instances in which a client knows credentials are required to make a request
    // but the credentials are missing or invalid. This allows the client fail fast rather
    // than waiting on a response from the target service.
    #[error("Missing or invalid credentials: {0}")]
    MissingInvalidCredentials(String),

    #[error("Service unavailable: {0}")]
    Unavailable(String),
}

impl ClientError {
    pub fn status_code(&self) -> u16 {
        match self {
            ClientError::BadRequest { .. } => 400,
            ClientError::Unauthorized { .. } => 401,
            ClientError::Forbidden { .. } => 403,
            ClientError::NotFound { .. } => 404,
            ClientError::Internal { .. } => 500,
            ClientError::Unavailable(_) => 503,
            ClientError::Unimplemented => 500,
            ClientError::MissingInvalidCredentials(_) => 400,
        }
    }
}

impl From<GitError> for ClientError {
    fn from(value: GitError) -> Self {
        match value {
            GitError::SystemError(err) => ClientError::Internal { msg: err.to_string(), scope: ClientErrorScope::Client },
            err => ClientError::Internal { msg: err.to_string(), scope: ClientErrorScope::Server },
        }
    }
}

impl From<&u16> for ClientError {
    fn from(value: &u16) -> Self {
        match value {
            400 => ClientError::BadRequest { msg: value.to_string(), scope: ClientErrorScope::Server },
            401 => ClientError::Unauthorized { msg: value.to_string(), scope: ClientErrorScope::Server },
            403 => ClientError::Forbidden { msg: value.to_string(), scope: ClientErrorScope::Server },
            404 => ClientError::NotFound { msg: value.to_string(), scope: ClientErrorScope::Server },
            500 => ClientError::Internal { msg: value.to_string(), scope: ClientErrorScope::Server },
            503 => ClientError::Unavailable(value.to_string()),
            _ => ClientError::Internal { msg: format!("Invalid http status used in client: {}", value), scope: ClientErrorScope::Client }
        }
    }
}