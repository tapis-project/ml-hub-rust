use std::sync::Arc;
use amqprs::channel::Channel;
use mongodb::Client;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;

#[derive(Clone)]
pub struct MessagePublisherConnectionArgs {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    pub client: Client,
    pub db_name: String,
    pub channel: Arc<Channel>,
}