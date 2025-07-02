use crate::presentation;
use crate::bootstrap::state::AppState;
use crate::infra::persistence::mongo::database::{ClientParams, get_db};
use actix_web::{App, HttpServer};
use std::env;
use actix_web::middleware::Logger;

pub async fn run_server() -> std::io::Result<()> {
    pub const DEFAULT_PORT: u16 = 8000;
    pub const DEFAULT_HOST: &str = "0.0.0.0";
    
    // Initialize the logger
    env_logger::init();

    // Set the address from env vars HOST and PORT, fallback to default values
    // if values for these env vars are not defined
    let addrs = (
        env::var("HOST").unwrap_or(DEFAULT_HOST.into()),
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT)
    );

    // Initialize AppState
    let state = AppState {
        db: get_db(ClientParams{
            username: env::var("ARTIFACTS_DB_USERNAME").expect("ARTIFACTS_DB_USERNAME env var not set"),
            password: env::var("ARTIFACTS_DB_PASSWORD").expect("ARTIFACTS_DB_PASSWORD env var not set"),
            host: env::var("ARTIFACTS_DB_HOST").expect("ARTIFACTS_DB_HOST env var not set"),
            port: env::var("ARTIFACTS_DB_PORT").expect("ARTIFACTS_DB_PORT env var not set"),
            db: env::var("ARTIFACTS_DB_NAME").expect("ARTIFACTS_DB_NAME env var not set"),
        })
            .await
            .map_err(|err| {
                panic!("Database initialization error: {}", err.to_string().as_str()); 
            })
            .expect("Datbase initialization error")
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(state.clone()))
            .service(presentation::http::v1::actix_web::handlers::index::index)
            .service(presentation::http::v1::actix_web::handlers::health_check::health_check)
            .service(presentation::http::v1::actix_web::handlers::get_model::get_model)
            .service(presentation::http::v1::actix_web::handlers::list_models::list_models)
            .service(presentation::http::v1::actix_web::handlers::ingest_model::ingest_model)
            .service(presentation::http::v1::actix_web::handlers::discover_models::discover_models)
            .service(presentation::http::v1::actix_web::handlers::publish_model::publish_model)
            .service(presentation::http::v1::actix_web::handlers::list_platforms::list_platforms)
            .service(presentation::http::v1::actix_web::handlers::upload_artifacts::upload_artifacts)
    })
        .bind(addrs)?
        .run()
        .await
}