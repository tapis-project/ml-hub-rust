use crate::presentation::http::v1::actix_web::response_helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, web, HttpRequest, Responder};
use shared::application::inputs::artifacts::GetModelArtifactInput;
use shared::application::services::artifact_service::ArtifactServiceError;
use shared::logging::SharedLogger;
use serde_json::to_value;
use shared::presentation::http::v1::requests::artifacts::GetArtifactPath;
use uuid::Uuid;
use shared::presentation::http::v1::contracts;

#[utoipa::path(
    get,
    tag="Artifacts",
    path = "/models-api/artifacts/{artifact_id}",
    description="Fetches the model artifact by the provided id",
    params(
        ("artifact_id" = String, Path, description = "Artifact id")
    ),
    responses(
        (status=200, description="Found model artifact", body=contracts::responses::GetModelArtifactResponse),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse),
    )
)]
#[get("models-api/artifacts/{artifact_id}")]
async fn get_model_artifact(
    _req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<GetArtifactPath>
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("get_model_artifact operation");
    
    let artifact_service = artifact_service_factory(&data.client, data.db_name.clone(), data.channel.clone());

    let artifact_id = match Uuid::parse_str(&path.artifact_id) {
        Ok(id) => id,
        Err(err) => return build_error_response(400, err.to_string())
    };

    let input = GetModelArtifactInput {
        artifact_id
    };

    let model_artifact_output = match artifact_service.get_model_artifact(input).await {
        Ok(o) => o,
        Err(err) => {
            match err {
                ArtifactServiceError::NotFound(msg) => return build_error_response(404, msg),
                _ => return build_error_response(500, format!("Error fetching artifact {}", &artifact_id))
            }
        }
    };

    let model_artifact = match responses::ModelArtifact::try_from(model_artifact_output) {
        Ok(ma) => ma,
        Err(err) => return build_error_response(500, err.to_string())
    };

    let resp = match to_value(model_artifact) {
        Ok(v) => v,
        Err(err) => return build_error_response(500, err.to_string())
    };
    
    build_success_response(Some(resp), None, None)
}