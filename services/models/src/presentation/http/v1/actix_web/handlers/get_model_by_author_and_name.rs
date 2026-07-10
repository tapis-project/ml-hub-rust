use actix_web::{
    get,
    web,
    Responder
};
use serde_json::to_value;
use shared::application::services::model_metadata_service::ModelMetadataService;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::{
    GetModelByAuthorAndNamePath,
    GetModelByAuthorAndNameQueryParams,
};

use shared::shared_kernal::identity::IdentityContext;
use shared::application::inputs::common::Scope as ScopeInput;
use shared::application::inputs::model_metadata::GetModelMetadataByAuthorAndNameInput;
use shared::presentation::http::v1::contracts::responses;
use shared::presentation::http::v1::responses::models::ModelMetadata;


#[utoipa::path(
    get,
    path="/models-api/models/{author}/{name}",
    tag="Models",
    description="Get model by the author and name",
    params(
        GetModelByAuthorAndNamePath,
        GetModelByAuthorAndNameQueryParams,
    ),
    responses(
        (status=200, description="Successfully fetched model", body=responses::GetModelResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[get("models-api/models/{author}/{name}")]
async fn get_model_by_author_and_name(
    path: web::Path<GetModelByAuthorAndNamePath>,
    params: web::Query<GetModelByAuthorAndNameQueryParams>,
    model_metadata_service: web::Data<ModelMetadataService>,
    identity_context: IdentityContext,
) -> impl Responder {
    let input = GetModelMetadataByAuthorAndNameInput {
        author: path.author.clone(),
        name: path.name.clone(),
        tenant_id: identity_context.actor_tenant_id().clone(),
        principal_id: identity_context.actor_principal_id().clone(),
        scope: ScopeInput::from(params.into_inner().scope.clone()),
    };

    let output = match model_metadata_service.get_by_author_and_name(input).await {
        Ok(m) => m,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let output_model = match output.model {
        Some(m) => m,
        None => return build_error_response(404, format!("No model metadata found for author {} and name {}", &path.author, &path.name))
    };

    let metadata_resp = match ModelMetadata::try_from(&output_model) {
        Ok(m) => m,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let value = match to_value(metadata_resp) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(value), Some("success".into()), None)
}
