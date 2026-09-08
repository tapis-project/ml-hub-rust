use crate::presentation::http::v1::{
    actix_web::helpers::{build_error_response, build_success_response},
    contracts::responses::RegisterDatasetResponse,
    requests::RegisterDatasetBody,
    responses::Dataset,
};
use actix_web::{post, web, Responder};
use serde_json::to_value;
use shared::{
    application::{
        inputs::dataset::RegisterDatasetInput,
        services::dataset_registration_service::{
            DatasetRegistrationService, DatasetRegistrationServiceError,
        },
    },
    shared_kernel::context::RequestContext,
};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/datasets-api/datasets",
    tag = "Datasets",
    summary = "Register a dataset",
    request_body = RegisterDatasetBody,
    responses(
        (status = 200, description = "Dataset registered", body = RegisterDatasetResponse),
        (status = 400, description = "Invalid dataset registration"),
        (status = 500, description = "Unable to register dataset"),
    )
)]
#[post("datasets-api/datasets")]
pub async fn register_dataset(
    body: web::Json<RegisterDatasetBody>,
    ctx: RequestContext,
    service: web::Data<DatasetRegistrationService>,
) -> impl Responder {
    let body = body.into_inner();

    if let Err(e) = body.validate() {
        return build_error_response(400, e.to_string());
    }

    let input = match RegisterDatasetInput::try_from(body) {
        Ok(v) => v,
        Err(e) => return build_error_response(400, e.to_string()),
    };

    let dataset = match service.register_dataset(&ctx, input).await {
        Ok(v) => v,
        Err(DatasetRegistrationServiceError::Repository(e)) => {
            return build_error_response(500, e.to_string())
        }
        Err(e) => return build_error_response(400, e.to_string()),
    };

    let response = match to_value(Dataset::from(dataset)) {
        Ok(v) => v,
        Err(e) => return build_error_response(500, e.to_string()),
    };

    build_success_response(
        Some(response),
        Some("Successfully registered dataset".into()),
        None,
    )
}
