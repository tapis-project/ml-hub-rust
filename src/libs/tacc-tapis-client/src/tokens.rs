use jsonwebtoken::{decode, DecodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "tapis/tenant_id")]
    pub tapis_tenant_id: String
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    // We are decoding the jwt without validation. Tapis will determine if the token
    // is valid and we will pass along the error.
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&[]), // No secret key
        &validation,
    )?;

    Ok(data.claims)
}