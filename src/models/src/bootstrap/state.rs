use std::sync::Arc;
use amqprs::channel::Channel;
use mongodb::Database;
use shared::{application::services::federated_ipd_registrar::FederatedIdpRegistrar, domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet, presentation::http::v1::actix_web::state::SharedState};

#[derive(Clone)]
pub struct AppState {
    pub client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    pub channel: Arc<Channel>,
    pub db: Database,
    pub idp_registrar: Arc<FederatedIdpRegistrar>,
}