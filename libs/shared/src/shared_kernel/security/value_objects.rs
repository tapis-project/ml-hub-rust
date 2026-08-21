use thiserror::Error;
use nonempty::NonEmpty;

use std::fmt;

use crate::shared_kernel::value_objects::{Base64EncodedString, Base64EncodedStringError};

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

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

// 1. Convert Nonce -> Base64EncodedString
// This infallible conversion uses your existing .encode() method
impl TryFrom<Nonce> for Base64EncodedString {
    type Error = Base64EncodedStringError;

    fn try_from(value: Nonce) -> Result<Self, Self::Error> {
        let bytes_vec: Vec<u8> = value.0.into_iter().collect();
        
        Ok(Self::encode(&bytes_vec))
    }
}

// 2. Convert Base64EncodedString -> Nonce
// This fallible conversion handles invalid Base64 string decoding and empty vectors
impl TryFrom<Base64EncodedString> for Nonce {
    type Error = Base64EncodedStringError;

    fn try_from(value: Base64EncodedString) -> Result<Self, Self::Error> {
        // 1. Decode the base64 string using your existing implementation
        let raw_bytes = value.decode()?;
        
        // 2. Map the constructor's empty check error into your expected return type
        Self::new(raw_bytes).map_err(|_empty_err| {
            Base64EncodedStringError::InvalidBase64Encoding(
                "Decoded nonce byte sequence cannot be empty".into()
            )
        })
    }
}