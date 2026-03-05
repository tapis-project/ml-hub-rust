use std::sync::Arc;
use crate::application::services::federated_ipd_registrar::FederatedIdpRegistrar;

pub struct SharedState {
    pub idp_registrar: Arc<FederatedIdpRegistrar>
}