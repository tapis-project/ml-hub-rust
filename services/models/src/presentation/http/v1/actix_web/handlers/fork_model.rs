use shared::application::inputs::common::Scope;
use shared::application::inputs::model_metadata::GetModelMetadataByAuthorAndNameInput;
use shared::domain::entities::model_metadata::ModelMetadata;
use crate::presentation::http::v1::requests::ForkModelPath;
use shared::shared_kernel::context::RequestContext;
use shared::application::services::model_metadata_service::ModelMetadataService;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response,
    build_success_response,
};
use crate::application::model_metadata_inputs::RegisterModelMetadataInput;
use actix_web::{
    post,
    web, 
    Responder
};
use shared::presentation::http::v1::contracts::responses;

#[utoipa::path(
    post,
    path="/models-api/models/fork/{author}/{name}",
    tag="Models",
    params(
        ForkModelPath,
    ),
    description="Fork model metadata from a platform",
    responses(
        (status=200, description="Forked model", body=responses::ForkModelResponse),
        (status=400, description="Bad request", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Server error", body=responses::ServerErrorResponse),
    )
)]
#[post("models-api/models/fork/{author}/{name}")]
async fn fork_model(
    path: web::Path<ForkModelPath>,
    ctx: RequestContext,
    model_metadata_service: web::Data<ModelMetadataService>,
) -> impl Responder {
    let get_model_input = GetModelMetadataByAuthorAndNameInput {
        author: path.author.clone(),
        name: path.name.clone(),
        tenant_id: ctx.actor_tenant_id().clone(),
        principal_id: ctx.actor_principal_id().clone(),
        scope: Scope::Global,
    };

    let maybe_model_to_fork = match model_metadata_service.get_by_author_and_name(get_model_input).await {
        Ok(m) => m.model,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let output = match maybe_model_to_fork {
        Some(m) => m,
        None => return build_error_response(404, format!("Could not find model {}/{}", &path.author, &path.name))
    };

    let model_to_fork = match ModelMetadata::try_from(output) {
        Ok(m) => m,
        Err(_) => return build_error_response(500, "Data integrity error".into())
    };

    let register_model_metadata_input = match RegisterModelMetadataInput::try_from(model_to_fork) {
        Ok(i) => i,
        Err(e) => return build_error_response(500, e.to_string())
    };

    match model_metadata_service.register_model_metadata(register_model_metadata_input, &ctx).await {
        Ok(_) => (),
        Err(e) => return build_error_response(500, e.to_string())
    };

    build_success_response(None, Some("Successfully forked model".into()), None)
}
