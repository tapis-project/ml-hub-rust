use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::{
    application::{
        inputs::model::AssociateModelWithArtifactInput,
        services::{
            model_artifact_association_service::{
                ModelArtifactAssociationService, ModelArtifactAssociationServiceError,
            },
            model_query_service::ModelQueryService,
        },
    },
    presentation::http::v1::{
        contracts::responses,
        requests::associate_model::{body::AssociateModelBody, path::AssociateModelPath},
        responses::models::Model,
    },
    shared_kernel::context::RequestContext,
};
use uuid::Uuid;

use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};

#[utoipa::path(
    post,
    path = "/models-api/artifacts/{artifact_id}/model",
    tag = "Artifacts",
    summary = "Associate an owned model with an artifact",
    params(
        (
            "artifact_id" = String,
            Path,
            format = "uuid"
        ),
    ),
    request_body = AssociateModelBody,
    responses(
        (
            status = 200,
            description = "Model associated",
            body = responses::AssociateModelResponse
        ),
        (
            status = 400,
            description = "Invalid identifier",
            body = responses::BadRequestResponse
        ),
        (
            status = 404,
            description = "Model or artifact not found",
            body = responses::NotFoundResponse
        ),
        (
            status = 409,
            description = "Artifact already associated",
            body = responses::ConflictResponse
        ),
        (
            status = 500,
            description = "Unable to associate model",
            body = responses::ServerErrorResponse
        ),
    ),
)]
#[post("models-api/artifacts/{artifact_id}/model")]
pub async fn associate_model_with_artifact(
    path: web::Path<AssociateModelPath>,
    body: web::Json<AssociateModelBody>,
    ctx: RequestContext,
    association_service: web::Data<ModelArtifactAssociationService>,
    query_service: web::Data<ModelQueryService>,
) -> impl Responder {
    let artifact_id = match Uuid::parse_str(&path.artifact_id) {
        Ok(id) => id,
        Err(_) => return build_error_response(400, "artifact_id must be a UUID".into()),
    };

    let model_id = body.model_id;
    let input = AssociateModelWithArtifactInput {
        artifact_id,
        model_id,
    };

    match association_service.associate(&ctx, input).await {
        Ok(()) => {}
        Err(ModelArtifactAssociationServiceError::ArtifactNotFound)
        | Err(ModelArtifactAssociationServiceError::ModelNotFound) => {
            return build_error_response(404, "Model or artifact not found".into())
        }
        Err(ModelArtifactAssociationServiceError::ModelRepository(
            shared::application::ports::model::ModelRepositoryError::ArtifactAlreadyAssociated,
        )) => {
            return build_error_response(409, "Artifact is already associated with a Model".into())
        }
        Err(error) => return build_error_response(500, error.to_string()),
    }

    let output = match query_service.get_model(&ctx, model_id).await {
        Ok(output) => output,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    let result = match to_value(Model::from(output)) {
        Ok(result) => result,
        Err(error) => return build_error_response(500, error.to_string()),
    };

    build_success_response(
        Some(result),
        Some("Successfully associated model with artifact".into()),
        None,
    )
}
