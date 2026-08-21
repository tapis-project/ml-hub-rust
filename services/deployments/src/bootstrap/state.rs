use std::sync::Arc;
use amqprs::channel::Channel;
use mongodb::Client;

#[derive(Clone)]
pub struct MessagePublisherConnectionArgs {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub db_name: String,
    pub channel: Arc<Channel>,
}