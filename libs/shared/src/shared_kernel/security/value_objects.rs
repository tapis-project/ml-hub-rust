use thiserror::Error;
use nonempty::NonEmpty;

#[derive(Debug, Clone, Error)]
pub enum KeyIdError {
    #[error("Key Id MUST not be an empty string")]
    EmptyKeyId
}

#[derive(Debug, Clone)]
pub struct KeyId(String);

impl KeyId {
    pub fn new(id: &str) -> Result<Self, KeyIdError> {
        if id.len() == 0 {
            return Err(KeyIdError::EmptyKeyId)
        }

        Ok(Self(id.to_string()))
    }
}

#[derive(Debug, Clone, Error)]
pub enum NonceError {
    #[error("Nonce must not be empty")]
    EmptyNonce
}

#[derive(Debug, Clone)]
pub struct Nonce(NonEmpty<u8>);

impl Nonce {
    pub fn new(nonce: Vec<u8>) -> Result<Self, NonceError> {
        let maybe_nonce = NonEmpty::from_vec(nonce);
        match maybe_nonce {
            Some(n) => Ok(Self(n)),
            None => Err(NonceError::EmptyNonce)
        }
    }
}