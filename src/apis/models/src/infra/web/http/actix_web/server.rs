use crate::presentation;
use crate::bootstrap::state::AppState;
use crate::infra::persistence::mongo::database::{ClientParams, get_db};
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use actix_web::{App, HttpServer};
use std::env;
use actix_web::middleware::Logger;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;


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
            .service(presentation::http::v1::actix_web::handlers::get_model_by_platform::get_model_by_platform)
            .service(presentation::http::v1::actix_web::handlers::list_models_by_platform::list_models_by_platform)
            .service(presentation::http::v1::actix_web::handlers::ingest_external_model::ingest_external_model)
            .service(presentation::http::v1::actix_web::handlers::discover_models_by_platform::discover_models_by_platform)
            .service(presentation::http::v1::actix_web::handlers::discover_models::discover_models)
            .service(presentation::http::v1::actix_web::handlers::publish_model_artifact::publish_model_artifact)
            .service(presentation::http::v1::actix_web::handlers::list_platforms::list_platforms)
            .service(presentation::http::v1::actix_web::handlers::download_artifact::download_artifact)
            .service(presentation::http::v1::actix_web::handlers::upload_model_artifact::upload_model_artifact)
            .service(presentation::http::v1::actix_web::handlers::associate_model_metadata_with_artifact::associate_model_metadata_with_artifact)
            .service(presentation::http::v1::actix_web::handlers::publish_model_artifact::publish_model_artifact)
            .service(presentation::http::v1::actix_web::handlers::list_model_artifacts::list_model_artifacts)
            .service(presentation::http::v1::actix_web::handlers::list_model_publications::list_model_publications)
            .service(presentation::http::v1::actix_web::handlers::list_model_ingestions::list_model_ingestions)
            .service(presentation::http::v1::actix_web::handlers::list_publications_for_artifact::list_publications_for_artifact)
            .service(presentation::http::v1::actix_web::handlers::get_model_ingestion::get_model_ingestion)
            .service(presentation::http::v1::actix_web::handlers::get_model_publication::get_model_publication)
            .service(presentation::http::v1::actix_web::handlers::get_model_artifact::get_model_artifact)
            .service(presentation::http::v1::actix_web::handlers::openapi::openapi)
            .service(
                SwaggerUi::new("models-api/swagger-ui/{_:.*}")
                    .url("/models-api/specs/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind(addrs)?
        .run()
        .await
}