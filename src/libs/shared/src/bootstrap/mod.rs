use std::sync::Arc;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::application::services::federated_ipd_registrar::{FederatedIdpRegistrar, FederatedIdpRegistrarError};
use crate::infra::identity::tapis_federated_identity_provider::TapisFederatedIdentityProvider;

pub async fn initialize_idps() -> Vec<Arc<dyn FederatedIdentityProvider>> {
    vec![
        Arc::new(TapisFederatedIdentityProvider {}),
    ]
}

pub async fn build_ipd_registrar() -> Result<FederatedIdpRegistrar, FederatedIdpRegistrarError> {
    let idps = initialize_idps().await;
    let mut registrar = FederatedIdpRegistrar::new();
    for idp in idps {
        registrar.register(idp)?;
    }

    Ok(registrar)
}
