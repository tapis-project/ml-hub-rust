use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::dto::{Headers, PublishArtifactPath, PublishArtifactBody, PublishArtifactRequest};
use crate::presentation::http::v1::dto::ArtifactPublication as ArtifactPublicationDto;
use crate::application::artifact_publication_inputs::PublishArtifactInput;
use client_provider::ClientProvider;
use actix_web::{post, web, HttpRequest, Responder};
use shared::logging::SharedLogger;
use std::collections::HashMap;
use serde_json::to_value;

#[post("models-api/artifacts/{artifact_id}/publications")]
async fn publish_model(
    req: HttpRequest,
    path: web::Path<PublishArtifactPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<PublishArtifactBody>,
    data: web::Data<AppState>
) -> impl Responder {
    let logger = SharedLogger::new();

    logger.debug("Start publish model operation");
    
    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = PublishArtifactRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body: body.into_inner(),
    };

    // Fail-fast: Use the client provider to determine the client for the request platform
    // has the ability to publish artifacts. The client will not actually be used here,
    // we are just using this check to fail fast as the client will be invoked
    // somewhere else later.
    if let Err(err) = ClientProvider::provide_publish_model_client(&request.body.target_platform) {
        return build_error_response(400, err.to_string());
    }

    let artifact_service = match artifact_service_factory(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let input = match PublishArtifactInput::try_from(request) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let publication = match artifact_service.submit_artifact_publication(input).await {
        Ok(p) => p,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let resp = match to_value(ArtifactPublicationDto::from(publication)) {
        Ok(r) => r,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(resp), Some("Successfully submitted artifact publication".into()), None)
}
