use crate::bootstrap::factories::artifact_service_factory;
use crate::bootstrap::state::AppState;
use crate::presentation::http::v1::actix_web::helpers::{
    build_error_response, build_success_response,
};
use crate::presentation::http::v1::dto::{UploadModelRequest};
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, Responder};
use futures::TryStreamExt;
use serde_json::json;
use shared::common::application::inputs::UploadArtifactInput;
use shared::logging::SharedLogger;

// Check if the field is a zip file based on its content type
fn is_zip_file(field: &actix_multipart::Field) -> bool {
    if let Some(content_type) = field.content_type() {
        return content_type.to_string() == "application/zip";
    }
    false
}

#[post("models-api/artifacts")]
async fn upload_artifacts(
    req: HttpRequest,
    bytes: web::Payload,
    data: web::Data<AppState>,
) -> impl Responder {
    let logger = SharedLogger::new();
    logger.debug("Start upload artifact operation");
    let mut multipart = Multipart::new(req.headers(), bytes);

    if let Ok(Some(mut field)) = multipart.try_next().await {
        // let mut field = field_result.unwrap();

        if !is_zip_file(&field) {
            return build_error_response(400, String::from("Content-Type must be application/zip"));
        }

        // Instantiate an artifact service
        let artifact_service = match artifact_service_factory(&data.db).await {
            Ok(s) => s,
            Err(err) => return build_error_response(500, err.to_string()),
        };

        // todo: write to a file * refactor this code to infra/app layer
        let input = match UploadArtifactInput::try_from(UploadModelRequest {}) {
            Ok(i) => i,
            Err(err) => return build_error_response(500, err.to_string()),
        };

        let (artifact_id, mut uploading) = match artifact_service.upload_artifact(&input).await {
            Ok(tuple) => tuple,
            Err(err) => return build_error_response(500, err.to_string()),
        };
        while let Ok(Some(chunk)) = field.try_next().await {
            // Convert the `bytes::Bytes` chunk into a `Vec<u8>` before passing it
            if let Err(err) = uploading(chunk.to_vec()).await {
                return build_error_response(500, err.to_string());
            }
        }

        build_success_response(Some(json!(artifact_id)), Some("success".into()), None)
    } else {
        build_error_response(400, "No file provided".to_string())
    }
}

// Handler tests
#[cfg(test)]
#[path = "upload_artifacts.test.rs"]
mod upload_artifacts_test;
