use crate::application::ports::identity::{FederatedIdentityProvider, FederatedIdentityProviderError};
use crate::domain::entities::identity::{FederatedIdentity, NewFederatedIdentityProps, Authority};
use jsonwebtoken::Algorithm;
use serde::Deserialize;
use tapis_tenants::{TapisTenants, models::Tenant};
use crate::infra::identity::helpers::{token_from_string, validate_claims, Token as Jwt};

#[derive(Debug, Clone, Deserialize)]
pub struct Header {
    pub alg: Algorithm,
    pub kid: String,
    pub typ: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Payload {
    pub jti: String,
    pub iss: String,
    pub sub: String,

    #[serde(rename = "tapis/tenant_id")]
    pub tapis_tenant_id: String,

    #[serde(rename = "tapis/token_type")]
    pub tapis_token_type: String,

    #[serde(rename = "tapis/delegation")]
    pub tapis_delegation: bool,

    #[serde(rename = "tapis/delegation_sub")]
    pub tapis_delegation_sub: Option<String>,

    #[serde(rename = "tapis/username")]
    pub tapis_username: String,

    #[serde(rename = "tapis/account_type")]
    pub tapis_account_type: String,

    pub exp: u64,

    #[serde(rename = "tapis/client_id")]
    pub tapis_client_id: String,

    #[serde(rename = "tapis/grant_type")]
    pub tapis_grant_type: String,
}

pub struct Token {
    pub raw_token: String,
    pub header: Header,
    pub payload: Payload,
}

impl Jwt for Token {
    type Header = Header;
    type Payload = Payload;
    
    fn new(raw_token: String, header: Self::Header, payload: Self::Payload) -> Self {
        return Self {
            header,
            payload,
            raw_token,
        }
    }

    fn get_raw(&self) -> String {
        self.raw_token.clone()
    }
}

pub struct TapisFederatedIdentityProvider {
    tenants: Vec<Tenant>,
}

impl TapisFederatedIdentityProvider {
    pub async fn new() -> Result<Self, FederatedIdentityProviderError> {
        let base_url = std::env::var(&"TAPIS_IPD_BASE_URL".to_string())
            .unwrap_or(String::from("https://admin.tapis.io"));
        
        Ok(Self {
            tenants: TapisTenants::new(base_url.as_str(), None)
                .map_err(|err| FederatedIdentityProviderError::InitializationError(Authority::Tapis, err.to_string()))?
                .tenants
                .list_tenants(None, None)
                .await
                .map_err(|err| FederatedIdentityProviderError::InitializationError(Authority::Tapis, err.to_string()))?
                .result
                .unwrap_or_else(|| vec![])
        })
    }
}

#[async_trait::async_trait]
impl FederatedIdentityProvider for TapisFederatedIdentityProvider {    
    async fn authenticate(&self, token_string: String) -> Result<Option<FederatedIdentity>, FederatedIdentityProviderError> {
        let token: Token = token_from_string(&token_string)?;
        
        let pubkey = self.tenants
            .iter()
            .find(|t| t.tenant_id == token.payload.tapis_tenant_id)
            .map(|t| t.public_key.clone())
            .flatten()
            .ok_or_else(|| FederatedIdentityProviderError::InternalIdpError("Tenant missing public key".to_owned()))?;

        let alg = token.header.alg;
        
        let validated_claims = validate_claims(token, pubkey, alg)?;

        return Ok(Some(
            FederatedIdentity::new(
                NewFederatedIdentityProps {
                    authority: Authority::Tapis,
                    issuer: validated_claims.iss,
                    subject: validated_claims.sub,
                    metadata: None,
                }
            )
        ))
    }

    fn authority(&self) -> Authority {
        Authority::Tapis
    }
}