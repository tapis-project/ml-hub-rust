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
    Ok(decode::<T::Payload>(
        &token.get_raw()[..],
        &DecodingKey::from_rsa_pem(pubkey.as_bytes())
            .map_err(|err| FederatedIdentityProviderError::InternalIdpError(format!("Tenant pubkey expected to be in PEM format: {}", err.to_string())))?,
        &Validation::new(algo)
    )
        .map_err(|err| FederatedIdentityProviderError::InvalidCredentials(err.to_string()))?
        .claims)
}

pub(crate) fn token_from_string<T>(token_string: &String) -> Result<T, FederatedIdentityProviderError> 
    where T: Token
{
    let mut parts = token_string.split(".");
    
    let parts_count = parts.clone().count();
    if parts_count != 3 {
        return Err(FederatedIdentityProviderError::MalformedCredentials(format!("Expected Tapis JWT to have 3 parts, found {} parts", &parts_count)))
    }
    
    let header = match parts.nth(0) {
        Some(h) => {
            URL_SAFE_NO_PAD.decode(h)
                .ok()
                .map(|s| from_slice::<T::Header>(&s))
                .transpose()
                .map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Header deserialization error: {}", err.to_string())))?
                .ok_or(FederatedIdentityProviderError::MalformedCredentials("Header deserialization failed".to_owned()))?     
        },
        None => return Err(FederatedIdentityProviderError::MalformedCredentials("Header is None".to_owned()))
    };

    let payload = match  parts.nth(1) {
        Some(p) => {
            URL_SAFE_NO_PAD.decode(p)
                .ok()
                .map(|s| from_slice::<T::Payload>(&s))
                .transpose()
                .map_err(|err| FederatedIdentityProviderError::MalformedCredentials(format!("Payload deserialization error: {}", err.to_string())))?
                .ok_or(FederatedIdentityProviderError::MalformedCredentials("Payload deserialization failed".to_owned()))?
        },
        None => return Err(FederatedIdentityProviderError::MalformedCredentials("Payload is None".to_owned()))
    };

    Ok(T::new(token_string.clone(), header, payload ))
}