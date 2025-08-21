use std::collections::HashMap;
use crate::config::VERSION;
use clients::registrars::DatasetsClientRegistrar;
use actix_web::{
    web,
    get,
    HttpRequest as ActixHttpRequest,
    HttpResponse,
    Responder as ActixResponder
};
use log::debug;
use shared::responses::JsonResponse;
use shared::requests::{ListDatasetsPath, ListDatasetsRequest};

#[get("datasets-api/platforms/{platform}/datasets")]
async fn list_datasets(
    req: ActixHttpRequest,
    path: web::Path<ListDatasetsPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl ActixResponder {
    debug!("Start operation list_datasets");
    debug!("path: {:#?}", path);

    // Initialize the client registrar
    let registrar = DatasetsClientRegistrar::new();

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
    let request = ListDatasetsRequest{
        req,
        path,
        query,
        body
    };

    // Fetch the list of datasets
    match client.list_datasets(&request) {
        Ok(resp) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(200),
                    message: Some(String::from("success")),
                    result: resp.result,
                    metadata: None,
                    version: Some(VERSION.to_string())
                })
        },
        Err(err) => {
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
    }
}