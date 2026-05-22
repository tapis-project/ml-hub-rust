use crate::presentation::http::v1::actix_web::helpers::{
    build_client_error_response, build_error_response, build_success_response,
};
use crate::presentation::http::v1::requests::{GetDatasetByPlatformPath, GetDatasetByPlatformRequest, Headers};
use shared::presentation::http::v1::contracts::responses;
use actix_web::{get, web, HttpRequest, Responder};
use client_provider::ClientProvider;
use clients::GetDatasetClient;
use std::collections::HashMap;
use platforms::Platform;
use shared::presentation::http::v1::requests::models::GetModelByPlatformRequest;

#[utoipa::path(
    get,
    path="/datasets-api/platforms/{platform}/datasets/{dataset_id}",
    tag="Platforms",
    description="Fetch a dataset from an external platform by id",
    params(
        ("platform"=Platform, Path, description="Name of the platform from which to fetch the dataset"),
        ("dataset_id"=String, Path, description="Id of the dataset the fetch from the source platform"),
    ),
    responses(
        (status=200, description="Dataset fetched successfully", body=responses::GetDatasetByPlatformResponse),
        (status=400, description="Not found", body=responses::BadRequestResponse),
        (status=404, description="Not found", body=responses::NotFoundResponse),
        (status=500, description="Not found", body=responses::ServerErrorResponse),
    )
)]
#[get("datasets-api/platforms/{platform}/datasets/{dataset_id:.*}")]
async fn get_dataset_by_platform(
    req: HttpRequest,
    path: web::Path<GetDatasetByPlatformPath>,
    query: web::Query<HashMap<String, String>>,
    body: web::Bytes,
) -> impl Responder {
    let headers = match Headers::try_from(req.headers()) {
        Ok(h) => h,
        Err(err) => return build_error_response(400, String::from(err.to_string())),
    };

    let request = GetDatasetByPlatformRequest {
        headers,
        path: path.into_inner(),
        query: query.into_inner(),
        body,
    };

    // Get the client for the provided platform
    let client =
        if let Ok(client) = ClientProvider::provide_get_dataset_client(&request.path.platform) {
            client
        } else {
            return build_error_response(
                500,
                String::from(format!(
                    "Failed to find client for platform '{}'",
                    &request.path.platform
                )),
            );
        };

    // Fetch the list of models
    match client.get_dataset(&request).await {
        Ok(resp) => {
            return build_success_response(resp.result, Some(String::from("success")), resp.metadata)
        }
        Err(err) => return build_client_error_response(err),
    }
}