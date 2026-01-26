use crate::presentation::http::v1::responses;
use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use actix_web::{get, post, delete, web, HttpRequest, Responder};
use shared::application::services::artifact_service::ArtifactServiceError;
use shared::logging::SharedLogger;
use serde_json::to_value;
use shared::presentation::http::v1::requests::artifacts::GetArtifactPath;
use uuid::Uuid;
use shared::presentation::http::v1::contracts;




#[utoipa::path(
    post,
    tag="Artifacts",
    path="/models-api/artifacts/{artifact_id}",
    description="delete model artifact by provided id",
    params(
        ("artifact_id"=String, Path, description="Artifact id")
    ),
    responses(
        (status=200, description="Deleted model artifact", body=contracts::responses::DeleteArtifact),
        (status=400, description="Not found", body=contracts::responses::BadRequestResponse),
        (status=404, description="Not found", body=contracts::responses::NotFoundResponse),
        (status=500, description="Not found", body=contracts::responses::ServerErrorResponse)
    )
)]

#[delete("/models-api/artifacts/{artifact_id}")]
async fn delete_artifact (
    _req: HttpRequest,
    path: web::Path<GetArtifactPath>,
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug;

    let delete_service = match delete_resource_from_db(&data.db) {
        Ok(s) => s,
        Err(err) => return build_error_response(500, err.to_string()) 
    };

    let artifact_id = match Uuid::parse_str(&path.artifact_id) {
        Ok(id) => id,
        Err(err) => return build_error_response(400, err.to_string())
    };

    let artifact_response = match artifact_service.delete_artifact(input).await{
        Ok(o) => o,
        Err(err) => {
            match err {
                ArtifactServiceError::NotFound(msg) => return build_error_response(400, msg),
                _ => return build_error_response(500, format!("Error fetching artifact {}", &artifact_id))
            }
        }

    };
    let resp = match to_value(model_artifact) {
        Ok(v) => v,
        Err(err) => return 
    };
    
    delete_success_response(Some(resp), None, None)
}

