use crate::application::artifact_inputs::IngestArtifactInput;
use crate::bootstrap::factories::{artifact_service_factory, model_metadata_repo_factory, model_metadata_service_factory};
use crate::bootstrap::state::AppState;
use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::{
    Headers, IngestModelRequest, IngestArtifactRequest, IngestModelPath, IngestCanonicalModelPath,
};
use crate::presentation::http::v1::responses::ArtifactIngestion;
use actix_web::{post, web, HttpRequest, Responder};
use client_provider::ClientProvider;
use serde_json::to_value;
use shared::application::identity_context::IdentityContext;
use shared::application::inputs::common::Scope;
use shared::application::inputs::model_metadata::{GetModelMetadataByAuthorAndNameInput, UpdateModelMetadataArtifactId};
use shared::presentation::http::v1::contracts;
use std::collections::HashMap;

#[utoipa::path(
    post,
    path="/models-api/models/{author}/{name}",
    tag="Models",
    description="Ingest canonical model artifact",
    params(
        ("author" = String, Path, description = "The author of the model"),
        ("name" = String, Path, description = "The name of the model"),
    ),
    request_body=contracts::requests::artifacts::IngestArtifactRequest,
    responses(
        (status=200, description="Ingest canonical models", body=contracts::responses::IngestModelArtifactResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[post("models-api/models/{author}/{name}")]
async fn ingest_canonical_model(
    req: HttpRequest,
    path: web::Path<IngestCanonicalModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<IngestArtifactRequest>,
    data: web::Data<AppState>,
    identity_context: IdentityContext,
) -> impl Responder {
    let model_metadata_service = match model_metadata_service_factory(
        &data.client,
        data.db_name.clone(),
        data.client_strategy_sets.clone()
    ).await {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let input = GetModelMetadataByAuthorAndNameInput {
        author: path.author.clone(),
        name: path.name.clone(),
        tenant_id: identity_context.actor_tenant_id().clone(),
        principal_id: identity_context.actor_principal_id().clone(),
        scope: Scope::Global,
    };

    let maybe_metadata = match model_metadata_service.get_by_author_and_name(input).await {
        Ok(m) => m,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let metadata = match maybe_metadata {
        Some(m) => m,
        None => return build_error_response(404, format!("No model metadata found for author {} and name {}", &path.author, &path.name))
    };

    let canonical = match metadata.canonical {
        Some(c) => c,
        None => return build_error_response(404, format!("No canonical model for model {}/{}", &path.author, &path.name)),
    };

    // Fail-fast: Use the client provider to determine the client for the request platform
    // has the ability to ingest artifacts. The client will not actually be used here,
    // we are just using this check to fail fast as the client will be invoked
    // somewhere else later.
    if let Err(err) = ClientProvider::provide_ingest_model_client(&canonical.platform.to_string().as_str()) {
        return build_error_response(400, err.to_string());
    };

    // Instantiate an artifact service
    let artifact_service = artifact_service_factory(&data.client, data.db_name.clone(), data.channel.clone());

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = IngestModelRequest {
        headers,
        path: IngestModelPath {
            platform: canonical.platform.to_string(),
            model_id: canonical.model_id,
        },
        query: query.into_inner(),
        body: body.into_inner(),
    };

    // Convert the request requests into an input
    let submit_input = match IngestArtifactInput::try_from(request) {
        Ok(i) => i,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    // Ingest the artifact
    let ingestion = match artifact_service.submit_artifact_ingestion(submit_input).await {
        Ok(a) => a,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    let update_input = UpdateModelMetadataArtifactId {
        artifact_id: ingestion.artifact_id,
        name: path.name.clone(),
        author: path.author.clone()
    };

    // TODO Refactor! No repos called directly in the presentation layer.
    // Put this in the model metadata service.
    let metadata_repo = model_metadata_repo_factory(&data.client, data.db_name.clone());

    // Update model metadata with the artifact id
    match metadata_repo.update_artifact_id(&update_input).await {
        Ok(_) => {},
        Err(err) => return build_error_response(500, err.to_string())
    };

    // Convert to requests
    let requests = match to_value(ArtifactIngestion::from(ingestion)) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string()),
    };

    build_success_response(Some(requests), Some("success".into()), None)
}
