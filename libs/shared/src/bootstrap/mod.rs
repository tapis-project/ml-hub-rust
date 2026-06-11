use std::sync::Arc;

use mongodb::Client;

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

// Infra
use crate::infra::identity::Idp;
use crate::infra::configuration::SiteConfiguration;

pub struct SharedAppContext {
    pub config: SiteConfiguration,
    pub idp_registrar: FederatedIdpRegistrar,
    pub federated_identity_service: FederatedIdentityService,
    pub principal_service: PrincipalService
}

pub async fn initialize_idps(idps: &Vec<Idp>, config: &SiteConfiguration) -> Result<Vec<Arc<dyn FederatedIdentityProvider>>, FederatedIdpRegistrarError> {
    let mut initialized_idps: Vec<Arc<dyn FederatedIdentityProvider>> = Vec::with_capacity(idps.len());
    for idp in idps {
        match idp {
            Idp::Tapis => initialized_idps.push(Arc::new(tapis::idp::FederatedIdentityProvider::new(config.clone()).await?))
        }
    }
    
    Ok(initialized_idps)
}

pub async fn build_idp_registrar(configurable_idps: &Vec<Idp>, config: &SiteConfiguration) -> Result<FederatedIdpRegistrar, FederatedIdpRegistrarError> {
    let mut registrar = FederatedIdpRegistrar::new();
    let idps = initialize_idps(configurable_idps, config).await?;
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

pub fn build_principal_service(client: Client, db_name: String) -> PrincipalService {
    let repo = build_principal_repository(client, db_name);
    PrincipalService::new(repo)
}

pub async fn build_shared_app_context(config: SiteConfiguration, client: Client, db_name: String) -> Result<SharedAppContext, FederatedIdpRegistrarError> {
    Ok(SharedAppContext {
        config: config.clone(),
        idp_registrar: build_idp_registrar(&config.idps, &config).await?,
        federated_identity_service: build_federated_identity_service(),
        principal_service: build_principal_service(client, db_name)
    })
}
