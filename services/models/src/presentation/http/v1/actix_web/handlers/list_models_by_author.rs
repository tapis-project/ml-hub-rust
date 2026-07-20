use actix_web::{
    get,
    web,
    Responder
};
use serde_json::{to_value, Value};
use shared::application::services::model_metadata_service::ModelMetadataService;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::presentation::http::v1::requests::ListModelsByAuthorPath;

use shared::shared_kernel::identity::IdentityContext;
use shared::application::inputs::model_metadata::ListModelMetadataByAuthorInput;
use shared::presentation::http::v1::contracts::responses;
use shared::presentation::http::v1::responses::models::ModelMetadata;


#[utoipa::path(
    get,
    path="/models-api/models/{author}",
    tag="Models",
    description="List models by author in the current tenant",
    params(
        ListModelsByAuthorPath,
    ),
    responses(
        (status=200, description="Successfully fetched models", body=responses::ListModelsResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[get("models-api/models/{author}")]
async fn list_models_by_author(
    path: web::Path<ListModelsByAuthorPath>,
    model_metadata_service: web::Data<ModelMetadataService>,
    identity_context: IdentityContext,
) -> impl Responder {
    let input = ListModelMetadataByAuthorInput {
        author: path.author.clone(),
        tenant_id: identity_context.actor_tenant_id().clone(),
        principal_id: identity_context.actor_principal_id().clone(),
    };

    let output = match model_metadata_service.list_by_author(input).await {
        Ok(m) => m,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let mut values: Vec<Value> = Vec::with_capacity(output.models.len());
    for model in output.models {
        let model_metadata_resp = match ModelMetadata::try_from(&model) {
            Ok(m) => m,
            Err(err) => return build_error_response(500, err.to_string())
        };

        match to_value(model_metadata_resp) {
            Ok(v) => values.push(v),
            Err(err) => return build_error_response(500, err.to_string())
        };
    }

    let resp = match to_value(values) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };

    build_success_response(Some(resp), Some("success".into()), None)
}
