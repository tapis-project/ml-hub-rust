use std::sync::Arc;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_ipd_registrar::{FederatedIdpRegistrar, FederatedIdpRegistrarError};
use crate::infra::identity::tapis_federated_identity_provider::TapisFederatedIdentityProvider;
use crate::presentation::http::v1::actix_web::state::SharedState;

pub struct SharedAppContext {
    pub idp_registrar: Arc<FederatedIdpRegistrar>,
    pub federated_identity_service: Arc<FederatedIdentityService>,
    pub state: Arc<SharedState>,
}

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

pub fn build_federated_identity_service() -> Arc<FederatedIdentityService> {
    Arc::new(FederatedIdentityService {})
}

pub async fn build_shared_app_context() -> Result<SharedAppContext, FederatedIdpRegistrarError> {
    let idp_registrar = Arc::new(build_ipd_registrar().await?);
    Ok(SharedAppContext {
        state: Arc::new(SharedState {
            idp_registrar: idp_registrar.clone()
        }),
        idp_registrar,
        federated_identity_service: build_federated_identity_service()
    })
}
