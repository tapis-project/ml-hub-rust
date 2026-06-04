use std::sync::Arc;
use amqprs::channel::Channel;
use mongodb::Client;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;

#[derive(Clone)]
pub struct AppState {
    pub client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    pub channel: Arc<Channel>,
    pub db_name: String,
    pub client: Client,
}