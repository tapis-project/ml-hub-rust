use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use clients::{ClientError, ClientErrorScope};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "tapis/tenant_id")]
    pub tapis_tenant_id: String
}

pub fn decode_jwt(token: &str) -> Result<Claims, ClientError> {
    // We are decoding the jwt without validation. Tapis will determine if the token
    // is valid and we will pass along the error.
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&[]), // No secret key
        &validation,
    ).map_err(|err| ClientError::BadRequest { msg: err.to_string(), scope: ClientErrorScope::Client })?;

    Ok(data.claims)
}