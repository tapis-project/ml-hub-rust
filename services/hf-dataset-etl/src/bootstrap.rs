use mongodb::Client;
use shared::{
    application::{
        ports::dataset::DatasetRepository,
        services::{
            dataset_query_service::DatasetQueryService,
            dataset_registration_service::DatasetRegistrationService,
        },
    },
    infra::persistence::mongo::repositories::DatasetRepository as MongoDatasetRepository,
};
use std::sync::Arc;

pub fn dataset_repository_factory(client: &Client, db_name: String) -> Arc<dyn DatasetRepository> {
    Arc::new(MongoDatasetRepository::new(client, db_name))
}

pub fn dataset_registration_service_factory(
    dataset_repository: Arc<dyn DatasetRepository>,
) -> DatasetRegistrationService {
    DatasetRegistrationService::new(dataset_repository)
}

pub fn dataset_query_service_factory(
    dataset_repository: Arc<dyn DatasetRepository>,
) -> DatasetQueryService {
    DatasetQueryService::new(dataset_repository)
}
