use std::sync::Arc;
use mongodb::Database;
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
    pub db: Database,
    pub message_publisher_connection_args: MessagePublisherConnectionArgs,
}