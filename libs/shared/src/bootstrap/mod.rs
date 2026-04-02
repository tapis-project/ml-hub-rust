use std::sync::Arc;

use mongodb::Client;
use strum_macros::{EnumString, Display};
use serde::Deserialize;
use thiserror::Error;

// Ports
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::application::ports::principal::PrincipalRepository;

// Services
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_idp_registrar::{FederatedIdpRegistrar, FederatedIdpRegistrarError};
use crate::application::services::principal_service::PrincipalService;

// Adapters
use crate::infra::identity::tapis;
use crate::infra::principal::mongo::principal_repository::PrincipalRepository as MongoPrincipalRepository;

// Domain
use crate::domain::entities::identity::FederatedIdentity;

#[derive(Clone, Debug, Deserialize)]
pub struct SiteConfiguration {
    pub site_id: String,
    pub base_url: String,
    pub idps: Vec<Idp>,
    pub tenancy_resolution_mode: TenancyResolutionMode,
}

#[derive(Debug, Clone, Error)]
pub enum IdpError {
    #[error("Failed to resolve the principal's id from federated identity: {0}")]
    ErrorResolvingPrincipalId(String)
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Idp {
    Tapis
}

impl Idp {
    pub fn all() -> Vec<Idp> {
        vec![
            Self::Tapis
        ]
    }

    pub fn resolve_principal_id(&self, identity: &FederatedIdentity) -> Result<String, IdpError> {
        match self {
            Self::Tapis => {
                if let Some((id, _)) = identity.subject.clone().rsplit_once("@") {
                    let principal_id = String::from(id);
                    return Ok(principal_id)
                }

                return Err(IdpError::ErrorResolvingPrincipalId("Malformed subject".into()))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Display, EnumString)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TenancyResolutionMode {
    Subdomain
}

pub struct SharedAppContext {
    pub config: SiteConfiguration,
    pub idp_registrar: FederatedIdpRegistrar,
    pub federated_identity_service: FederatedIdentityService,
    pub principal_service: Arc<PrincipalService>
}

pub async fn initialize_idps(idps: &Vec<Idp>) -> Result<Vec<Arc<dyn FederatedIdentityProvider>>, FederatedIdpRegistrarError> {
    let mut initialized_idps: Vec<Arc<dyn FederatedIdentityProvider>> = Vec::with_capacity(idps.len());
    for idp in idps {
        match idp {
            Idp::Tapis => initialized_idps.push(Arc::new(tapis::idp::FederatedIdentityProvider::new().await?))
        }
    }
    
    Ok(initialized_idps)
}

pub async fn build_idp_registrar(configurable_idps: &Vec<Idp>) -> Result<FederatedIdpRegistrar, FederatedIdpRegistrarError> {
    let mut registrar = FederatedIdpRegistrar::new();
    let idps = initialize_idps(configurable_idps).await?;
    for idp in idps {
        registrar.register(idp)?;
    }

    Ok(registrar)
}

pub fn build_federated_identity_service() -> FederatedIdentityService {
    FederatedIdentityService {}
}

pub fn build_principal_repository(client: Client, db_name: String) -> Arc<dyn PrincipalRepository> {
    Arc::new(MongoPrincipalRepository::new(client, db_name))
}

pub fn build_principal_service(client: Client, db_name: String) -> Arc<PrincipalService> {
    let repo = build_principal_repository(client, db_name);
    Arc::new(PrincipalService::new(repo))
}

pub async fn build_shared_app_context(config: SiteConfiguration, client: Client, db_name: String) -> Result<SharedAppContext, FederatedIdpRegistrarError> {
    Ok(SharedAppContext {
        config: config.clone(),
        idp_registrar: build_idp_registrar(&config.idps).await?,
        federated_identity_service: build_federated_identity_service(),
        principal_service: build_principal_service(client, db_name)
    })
}
