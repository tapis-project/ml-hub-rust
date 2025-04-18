use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database
}