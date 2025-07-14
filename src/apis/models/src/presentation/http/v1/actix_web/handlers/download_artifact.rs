use actix_web::{web, get, HttpRequest, Responder, Result};
use actix_files::NamedFile;
use shared::common::application::services::artifact_service::ArtifactServiceError;
use shared::logging::SharedLogger;
use crate::bootstrap::{
    state::AppState,
    factories::artifact_service_factory
};
use crate::application::inputs::DownloadArtifactInput;
use crate::presentation::http::v1::dto::{Headers, DownloadModelPath, DownloadModelRequest};
use crate::presentation::http::v1::actix_web::helpers::{build_error_response};

#[get("models-api/artifacts/{artifact_id}")]
async fn download_artifact(
    req: HttpRequest,
    path: web::Path<DownloadModelPath>,
    _data: web::Data<AppState>,
) -> Result<impl Responder> {
    let logger = SharedLogger::new();
    
    logger.debug("Start download model operation");

    // Build the request used by the client
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => {
            return Ok(build_error_response(
                400,
                String::from(err.to_string())
            ));
        }
    };

    let request = DownloadModelRequest{
        headers,
        path: path.into_inner(),
    };

    if request.path.artifact_id.contains("..") {
        return Ok(build_error_response(403, String::from("Forbidden")));
    }

    // Instantiate an artifact service
    let artifact_service = match artifact_service_factory(&_data.db).await {
        Ok(s) => s,
        Err(err) => return Ok(build_error_response(500, err.to_string()))
    };

    // Convert the request dto into an input
    let input = match DownloadArtifactInput::try_from(request) {
        Ok(i) => i,
        Err(err) => return Ok(build_error_response(500, err.to_string()))
    };
    
    let artifact_path = match artifact_service.download_artifact(input).await {
        Ok(a) => a,
        Err(err) => {
            match err {
                ArtifactServiceError::NotFound(err) => {
                    return Ok(build_error_response(500, err.to_string()))
                },
                _ => {
                    logger.debug(&err.to_string());
                    return Ok(build_error_response(500, "Unexpected error occurred while downloading artifact".to_string()))
                }
            }
        }
    };

    let file = match NamedFile::open(artifact_path) {
        Ok(file) => file,
        Err(err) => {
            logger.debug(&err.to_string());
            return Ok(build_error_response(500, "Failed to open artifact".to_string()));
        }
    };

    let response = file
        .set_content_disposition(
            actix_web::http::header::ContentDisposition {
                disposition: actix_web::http::header::DispositionType::Attachment,
                parameters: vec![],
            }
        )
        .into_response(&req);

    Ok(response)


    // TODO Handle multipart/mixed responses
    // return build_error_response(
    //     501,
    //     String::from("Artifact responses for MIME type multipart/mixed not yet implemented")
    // );
}

// Handler tests
#[cfg(test)]
#[path = "download_artifact.test.rs"]
mod download_artifact_test;