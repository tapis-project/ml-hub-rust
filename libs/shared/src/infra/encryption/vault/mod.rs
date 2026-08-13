use crate::{
    application::ports::cipher::{Cipher, CipherError, CryptoContext},
    shared_kernel::{
        security::{
        value_objects::{KeyId, Nonce},
        EncryptionEnvelope,
        EncryptionEnvelopeMetadata
    },
    value_objects::Base64EncodedString}
};

use async_trait::async_trait;

pub struct VaultCipher;

#[async_trait]
impl Cipher for VaultCipher {
    async fn encrypt(&self, ctx: CryptoContext, plain_text: Vec<u8>) ->  Result<EncryptionEnvelope, CipherError> {
        let metadata = match ctx {
            CryptoContext::DeploymentArgumentSecret => {
                // TODO actually implement
                EncryptionEnvelopeMetadata::new_aes_gcm(
                    KeyId::new("not implemented")
                        .map_err(|e| {
                            log::error!("Failed to create new KeyId: {}", e.to_string());
                            CipherError::EncryptionError(e.to_string())
                        })?,
                    Nonce::new(vec![0])
                        .map_err(|e| {
                            log::error!("Failed to create new Nonce from Vec: {}", e.to_string());
                            CipherError::EncryptionError(e.to_string())
                        })?,
                )
            }
        };

        // TODO encrypt
        let cipher_text = &plain_text;

        let payload = Base64EncodedString::encode(cipher_text);

        Ok(EncryptionEnvelope::new(payload, metadata))
    }
    
    async fn decrypt(&self, envelope: &EncryptionEnvelope) ->  Result<Vec<u8>, CipherError> {
        let cipher_text = envelope.base64_decoded_payload()
            .map_err(|e| CipherError::DecryptionError(e.to_string()))?;

        let plain_text = match envelope.metadata() {
            EncryptionEnvelopeMetadata::AesGcm { .. } => {
                // TODO Decrypt cipher text.
                cipher_text // TODO CURRENTLY UNENCRYPTED
            }
        };

        Ok(plain_text)
    }
}