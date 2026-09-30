use amqprs::channel::Channel;
use mongodb::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub channel: Arc<Channel>,
    pub db_name: String,
    pub client: Client,
}
