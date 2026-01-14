use std::sync::Arc;
use mongodb::Database;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;

#[derive(Clone)]
pub struct AppState {
    pub client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    pub db: Database,
}