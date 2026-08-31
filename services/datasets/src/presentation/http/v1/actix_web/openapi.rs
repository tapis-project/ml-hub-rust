use super::handlers::{
    get_dataset::__path_get_dataset, healthcheck::__path_healthcheck,
    list_datasets::__path_list_datasets, register_dataset::__path_register_dataset,
};
use crate::config::VERSION;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "MLHub Datasets API", version = VERSION), paths(register_dataset, get_dataset, list_datasets, healthcheck))]
pub struct ApiDoc;
