use std::sync::Arc;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_ipd_registrar::{FederatedIdpRegistrar, FederatedIdpRegistrarError};
use crate::infra::identity::tapis_federated_identity_provider::TapisFederatedIdentityProvider;

pub struct SharedAppContext {
    pub idp_registrar: FederatedIdpRegistrar,
    pub federated_identity_service: FederatedIdentityService,
}

pub async fn initialize_idps() -> Result<Vec<Arc<dyn FederatedIdentityProvider>>, FederatedIdpRegistrarError> {
    Ok(vec![
        Arc::new(TapisFederatedIdentityProvider::new().await?),
    ])
}

pub async fn build_ipd_registrar() -> Result<FederatedIdpRegistrar, FederatedIdpRegistrarError> {
    let idps = initialize_idps().await?;
    let mut registrar = FederatedIdpRegistrar::new();
    for idp in idps {
        registrar.register(idp)?;
    }

    Ok(registrar)
}

pub fn build_federated_identity_service() -> FederatedIdentityService {
    FederatedIdentityService {}
}

pub async fn build_shared_app_context() -> Result<SharedAppContext, FederatedIdpRegistrarError> {
    Ok(SharedAppContext {
        idp_registrar: build_ipd_registrar().await?,
        federated_identity_service: build_federated_identity_service()
    })
}
