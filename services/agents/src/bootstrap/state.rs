use mongodb::Client;

/// Shared infrastructure state retained by the Agents API.
#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub db_name: String,
}
