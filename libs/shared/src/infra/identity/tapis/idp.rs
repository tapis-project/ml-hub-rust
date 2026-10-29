use std::error::Error;

use jsonwebtoken::Algorithm;
use serde::Deserialize;
use tapis_tenants::{models::Tenant, TapisTenants};

// Application
use crate::application::ports::identity::{
    FederatedIdentityProvider as Port, FederatedIdentityProviderError,
};

// Domain
use crate::domain::entities::identity::{FederatedIdentity, NewFederatedIdentityProps};

// Infrastructure
use crate::infra::configuration::SiteConfiguration;
use crate::infra::error_chain::format_error_chain;
use crate::infra::identity::helpers::{token_from_string, validate_claims, Token as Jwt};
use crate::infra::identity::Idp;

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
    pub tapis_delegation: Option<bool>,

    #[serde(rename = "tapis/delegation_sub")]
    pub tapis_delegation_sub: Option<String>,

    #[serde(rename = "tapis/username")]
    pub tapis_username: String,

    #[serde(rename = "tapis/account_type")]
    pub tapis_account_type: String,

    pub exp: u64,

    #[serde(rename = "tapis/client_id")]
    pub tapis_client_id: Option<String>,

    #[serde(rename = "tapis/grant_type")]
    pub tapis_grant_type: String,
}

#[derive(Debug)]
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
        };
    }

    fn get_raw(&self) -> String {
        self.raw_token.clone()
    }
}

pub struct FederatedIdentityProvider {
    tenants: Vec<Tenant>,
}

impl FederatedIdentityProvider {
    pub async fn new(
        site_configuration: SiteConfiguration,
    ) -> Result<Self, FederatedIdentityProviderError> {
        let base_url = site_configuration.base_url;

        let client = TapisTenants::new(base_url.as_str(), None).map_err(|error| {
            Self::initialization_error(
                "constructing the Tapis Tenants client",
                &base_url,
                error.as_ref(),
            )
        })?;

        let response = client
            .tenants
            .list_tenants(None, None)
            .await
            .map_err(|error| {
                Self::initialization_error("loading the Tapis tenant catalog", &base_url, &error)
            })?;

        let tenants = response.result.ok_or_else(|| {
            let message = "Tapis tenant catalog response did not contain result data";

            log::error!(
                "Failed to initialize Tapis identity provider while loading the tenant catalog; base_url='{}'; error='{}'",
                base_url,
                message
            );

            FederatedIdentityProviderError::InitializationError(Idp::Tapis, message.into())
        })?;

        Ok(Self { tenants })
    }

    fn initialization_error(
        operation: &str,
        base_url: &str,
        error: &(dyn Error + 'static),
    ) -> FederatedIdentityProviderError {
        let error_chain = format_error_chain(error);

        log::error!(
            "Failed to initialize Tapis identity provider while {}; base_url='{}'; error_chain='{}'",
            operation,
            base_url,
            error_chain
        );

        FederatedIdentityProviderError::InitializationError(Idp::Tapis, error_chain)
    }
}

#[async_trait::async_trait]
impl Port for FederatedIdentityProvider {
    async fn authenticate(
        &self,
        token_string: String,
    ) -> Result<Option<FederatedIdentity>, FederatedIdentityProviderError> {
        let token: Token = token_from_string(&token_string)?;

        let pubkey = self
            .tenants
            .iter()
            .find(|t| t.tenant_id == token.payload.tapis_tenant_id)
            .map(|t| t.public_key.clone())
            .flatten()
            .ok_or_else(|| {
                FederatedIdentityProviderError::InternalIdpError(
                    "Tenant missing public key".to_owned(),
                )
            })?;

        let alg = token.header.alg;

        let validated_claims = validate_claims(token, pubkey, alg)?;

        return Ok(Some(FederatedIdentity::new(NewFederatedIdentityProps {
            issuer: validated_claims.iss,
            subject: validated_claims.sub,
            tenant_id: validated_claims.tapis_tenant_id,
            metadata: None,
        })));
    }

    fn authority(&self) -> Idp {
        Idp::Tapis
    }
}
