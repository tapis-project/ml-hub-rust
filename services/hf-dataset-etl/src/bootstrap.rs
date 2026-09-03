use mongodb::Client;
use shared::{
    application::{ports::dataset::DatasetRepository, services::dataset_service::DatasetService},
    infra::persistence::mongo::repositories::DatasetRepository as MongoDatasetRepository,
};
use std::sync::Arc;

pub fn dataset_repository_factory(client: &Client, db_name: String) -> Arc<dyn DatasetRepository> {
    Arc::new(MongoDatasetRepository::new(client, db_name))
}

pub fn dataset_service_factory(client: &Client, db_name: String) -> DatasetService {
    DatasetService::new(dataset_repository_factory(client, db_name))
}
