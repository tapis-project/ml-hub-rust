use std::collections::HashMap;
use actix_web::{
    web,
    post,
    HttpRequest,
    Responder,
};
use shared::logging::SharedLogger;
use crate::bootstrap::{
    state::AppState,
    factories::artifact_service_factory
};
use serde_json::to_value;
use crate::application::inputs::IngestArtifactInput;
use crate::presentation::http::v1::dto::{IngestModelPath, IngestModelRequest, Headers, IngestArtifactBody};
use crate::presentation::http::v1::actix_web::helpers::{build_error_response, build_success_response};
use crate::presentation::http::v1::responses::ArtifactIngestion;
use client_provider::ClientProvider;

#[post("models-api/platforms/{platform}/models/{model_id:.*}/artifacts")]
async fn ingest_model(
    req: HttpRequest,
    path: web::Path<IngestModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<IngestArtifactBody>,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();
    
    logger.debug("Start ingest model operation");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => {
            return build_error_response(
                400,
                String::from(err.to_string())
            )
        }
    };

    let request = IngestModelRequest{
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body: body.into_inner()
    };

    // Catch directory traversal attacks. 'model_id' may be used by clients to
    // constuct directories in the shared file system
    if request.path.model_id.contains("..") {
        return build_error_response(403, String::from("Forbidden"));
    }

    // Fail-fast: Use the client provider to determine the client for the request platform 
    // has the ability to ingest artifacts. The client will not actually be used here,
    // we are just using this check to fail fast as the client will be invoked
    // somewhere else later.
    if let Err(err) = ClientProvider::provide_ingest_model_client(&request.path.platform) {
        return build_error_response(400, err.to_string())
    }

    // Instantiate an artifact service
    let artifact_service = match artifact_service_factory(&data.db).await {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    // Convert the request dto into an input
    let input = match IngestArtifactInput::try_from(request) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string())
    };
    
    // Ingest the artifact
    let ingestion = match artifact_service.submit_artifact_ingestion(input).await {
        Ok(a) => a,
        Err(err) => return build_error_response(500, err.to_string())
    };

    // Convert to dto
    let dto = match to_value(ArtifactIngestion::from(ingestion)) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(dto), Some("success".into()), None)
}