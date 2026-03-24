use std::sync::Arc;
use serde::Deserialize;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_idp_registrar::{FederatedIdpRegistrar, FederatedIdpRegistrarError};
use crate::infra::identity::tapis_federated_identity_provider::TapisFederatedIdentityProvider;
use strum_macros::{EnumString, Display};

#[derive(Clone, Debug, Deserialize)]
pub struct SiteConfiguration {
    pub site_id: String,
    pub base_url: String,
    pub idps: Vec<Idp>,
    pub tenancy_resolution_mode: TenancyResolutionMode,
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
}

pub async fn initialize_idps(idps: &Vec<Idp>) -> Result<Vec<Arc<dyn FederatedIdentityProvider>>, FederatedIdpRegistrarError> {
    let mut initialized_idps: Vec<Arc<dyn FederatedIdentityProvider>> = Vec::with_capacity(idps.len());
    for idp in idps {
        match idp {
            Idp::Tapis => initialized_idps.push(Arc::new(TapisFederatedIdentityProvider::new().await?))
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

pub async fn build_shared_app_context(config: SiteConfiguration) -> Result<SharedAppContext, FederatedIdpRegistrarError> {
    Ok(SharedAppContext {
        config: config.clone(),
        idp_registrar: build_idp_registrar(&config.idps).await?,
        federated_identity_service: build_federated_identity_service()
    })
}
