use thiserror::Error;
use async_trait::async_trait;

use crate::shared_kernel::security::EncryptionEnvelope;



#[derive(Debug, Error, Clone)]
pub enum CipherError {
    #[error("{0}")]
    EncryptionError(String),

    #[error("{0}")]
    DecryptionError(String),
}

pub enum CryptoContext {
    DeploymentArgumentSecret
}

#[async_trait]
pub trait Cipher: Send + Sync {
    async fn encrypt(&self, ctx: CryptoContext, plain_text: Vec<u8>) -> Result<EncryptionEnvelope, CipherError>;
    async fn decrypt(&self, envelope: &EncryptionEnvelope) -> Result<Vec<u8>, CipherError>;
}