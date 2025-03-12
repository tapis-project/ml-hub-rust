use crate::config::VERSION;
use std::collections::HashMap;
use clients::registrars::ModelsClientRegistrar;
use actix_web::{
    web,
    post,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};
use actix_files::NamedFile;
use shared::{
    logging::SharedLogger,
    responses::artifact_helpers::StagedArtifactResponseHeaders
};
use shared::requests::{DownloadModelPath, DownloadModelRequest, DownloadModelBody};
use shared::responses::JsonResponse;

#[post("models-api/platforms/{platform}/models/{model_id:.*}/files")]
async fn download_model(
    req: ActixHttpRequest,
    path: web::Path<DownloadModelPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Json<DownloadModelBody>,
) -> impl ActixResponder {
    let logger = SharedLogger::new();
    
    logger.debug("Start download model operation");

    // Initialize the client registrar
    let registrar = ModelsClientRegistrar::new();

    // Get the client for the provided platform
    let client = if let Ok(client) = registrar.get_client(&path.platform) {
        client
    } else {
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(JsonResponse {
                status: Some(500),
                message: Some(String::from(format!("Failed to find client for platform '{}'", &path.platform))),
                result: None,
                metadata: None,
                version: Some(VERSION.to_string()),
            });
    };

    // Build the request used by the client
    let request = DownloadModelRequest{
        req: req.clone(),
        path,
        query,
        body
    };
    
    // Download the model and respond with the file contents using the provided
    // MIME type
    let client_resp = match client.download_model(&request) {
        Ok(client_resp) => client_resp,
        Err(err) => {
            logger.debug(&err.to_string());
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(err.to_string()),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string())
                })
        }
    };

    let staged_artifact_path = client_resp.staged_artifact.path.clone();
    if !staged_artifact_path.exists() {
        let err_msg = String::from(
            format!("Path to the staged artifact does not exist. Path: {}", staged_artifact_path.to_string_lossy()).as_str());
        logger.error(&err_msg);
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(JsonResponse {
                status: Some(500),
                message: Some(String::from(err_msg)),
                result: None, 
                metadata: None,
                version: Some(VERSION.to_string())
            })
    }

    let download_filename = request.body
            .download_filename
            .clone();

    let staged_artifact = client_resp.staged_artifact.clone();

    logger.debug(format!("{:#?}", &staged_artifact).as_str());

    let archive = request.body.archive.clone();

    let staged_artifact_headers = match StagedArtifactResponseHeaders::new(
        &staged_artifact,
        &download_filename,
        &archive
    ) {
        Ok(headers) => headers,
        Err(err) => {
            logger.debug(&err.to_string());
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(String::from(format!("{}", &err.to_string()))),
                    result: None,
                    metadata: None,
                    version: Some(VERSION.to_string())
                })
        }
    };
    
    let mut headers: Vec<(&str, &str)> = Vec::with_capacity(2);

    let content_type_header = (
        staged_artifact_headers.content_type_header.0.as_str(),
        staged_artifact_headers.content_type_header.1.as_str(),
    );

    headers.push(content_type_header);

    match &staged_artifact_headers.content_disposition_header {
        Some((ref key, ref value)) => {
            headers.push((
                key.as_str(),
                value.as_str()
            ));
        },
        None => {}
    };
    
    logger.debug(format!("Staged artifact path: {:?}", &staged_artifact_path).as_str());

    // Handle single-file octect-stream and archived responses
    if staged_artifact_path.is_file() {
        match NamedFile::open(staged_artifact_path) {
            Ok(file) => {
                let mut resp = file.use_last_modified(true)
                    .customize();
                
                for header in headers {
                    resp = resp.insert_header(header);
                }
    
                return resp
                    .respond_to(&req)
                    .map_into_boxed_body()
            },
            Err(err) => {
                logger.debug(&err.to_string());
                return HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .json(JsonResponse {
                        status: Some(500),
                        message: Some(String::from(format!("{}", &err.to_string()))),
                        result: None,
                        metadata: None,
                        version: Some(VERSION.to_string())
                    })
            }
        }
    }

    // TODO Handle multipart/mixed responses
    return HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(501),
            message: Some(String::from("Artifact responses for MIME type multipart/mixed not yet implemented")),
            result: None,
            metadata: None,
            version: Some(VERSION.to_string())
        })
}