use mongodb::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub db_name: String,
}
