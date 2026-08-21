pub mod value_objects;

use value_objects::{KeyId, Nonce};

use crate::shared_kernel::value_objects::{Base64EncodedString, Base64EncodedStringError};

#[derive(Debug, Clone)]
pub enum EncryptionEnvelopeMetadata {
    /// Symmetric encryption with AES-GCM
    AesGcm {
        /// The identifier for the encryption key
        key_id: value_objects::KeyId,

        /// A value that ensure encryption of the same data results in different values
        nonce: value_objects::Nonce,
    }
}

impl EncryptionEnvelopeMetadata {
    pub fn new_aes_gcm(key_id: KeyId, nonce: Nonce) -> Self {
        EncryptionEnvelopeMetadata::AesGcm { key_id, nonce }
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionEnvelope {
    payload: Base64EncodedString,
    metadata: EncryptionEnvelopeMetadata,
}

impl EncryptionEnvelope {
    pub fn new(payload: Base64EncodedString, metadata: EncryptionEnvelopeMetadata) -> Self {
        Self {
            payload,
            metadata,
        }
    }

    pub fn payload(&self) -> &Base64EncodedString {
        &self.payload
    }

    pub fn base64_decoded_payload(&self) -> Result<Vec<u8>, Base64EncodedStringError> {
        self.payload.decode()
    }

    pub fn metadata(&self) -> &EncryptionEnvelopeMetadata {
        &self.metadata
    }
}

