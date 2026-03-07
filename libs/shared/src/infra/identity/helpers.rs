use crate::application::ports::identity::FederatedIdentityProviderError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use serde_json::from_slice;

pub(crate) trait Token {
    type Header: Clone + for<'de> Deserialize<'de>;
    type Payload: Clone + for<'de> Deserialize<'de>;

    fn new(raw_token: String, header: Self::Header, payload: Self::Payload) -> Self;
    fn get_raw(&self) -> String;
}

pub(crate) fn validate_claims<T: Token>(token: T, pubkey: String, algo: Algorithm) -> Result<T::Payload, FederatedIdentityProviderError> {
    let raw_token = &token.get_raw()[..];
    
    let decoding_key = &DecodingKey::from_rsa_pem(pubkey.as_bytes())
            .map_err(|err| FederatedIdentityProviderError::InternalIdpError(format!("Tenant pubkey expected to be in PEM format: {}", err.to_string())))?;
    
    Ok(
        decode::<T::Payload>(
            raw_token,
            decoding_key,
            &Validation::new(algo)
        )
            .map_err(|err| FederatedIdentityProviderError::InvalidCredentials(err.to_string()))?
            .claims
    )
}

pub(crate) fn token_from_string<T>(token_string: &String) -> Result<T, FederatedIdentityProviderError> 
    where T: Token
{
    let mut parts = token_string.split(".");
    
    let parts_count = parts.clone().count();
    if parts_count != 3 {
        return Err(FederatedIdentityProviderError::MalformedCredentials(format!("Expected Tapis JWT to have 3 parts, found {} parts", &parts_count)))
    }

    let header_string = parts.next()
        .ok_or_else(|| FederatedIdentityProviderError::MalformedCredentials("Header is None".to_owned()))?;
    
    let header = from_slice::<T::Header>(
        URL_SAFE_NO_PAD.decode(header_string)
            .map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Failed to base64 decode header: {}", err.to_string())))?
            .as_slice()
    ).map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Header deserialization error: {}", err.to_string())))?;

    let payload_string = parts.next()
        .ok_or_else(|| FederatedIdentityProviderError::MalformedCredentials("Payload is None".to_owned()))?;

    let payload = from_slice::<T::Payload>(
        URL_SAFE_NO_PAD.decode(payload_string)
            .map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Failed to base64 decode payload: {}", err.to_string())))?
            .as_slice()
    ).map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Payload deserialization error: {}", err.to_string())))?;

    Ok(T::new(token_string.clone(), header, payload ))
}