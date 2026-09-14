use std::sync::Arc;

use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::ports::{
    artifacts::{ArtifactIngestionRepository, ArtifactPublicationRepository, ArtifactRepository},
    model::{ExternalModelRepository, ModelRepository},
};
use shared::application::services::{
    artifact_service::ArtifactService,
    external_model_discovery_service::ExternalModelDiscoveryService,
    model_artifact_association_service::ModelArtifactAssociationService,
    model_creation_service::ModelCreationService, model_query_service::ModelQueryService,
};
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::messaging::rabbitmq::artifact_op_message_publisher::RabbitMQArtifactOpMessagePublisher;
use shared::infra::persistence::mongo::repositories::{
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
    ExternalModelRepository as MongoExternalModelRepository,
    ModelRepository as MongoModelRepository,
};

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name))
}

pub fn artifact_ingestion_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ArtifactIngestionRepository> {
    Arc::new(MongoArtifactIngestionRepository::new(client, db_name))
}

pub fn artifact_publication_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ArtifactPublicationRepository> {
    Arc::new(MongoArtifactPublicationRepository::new(client, db_name))
}

pub fn model_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelRepository> {
    Arc::new(MongoModelRepository::new(client, db_name))
}

pub fn external_model_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ExternalModelRepository> {
    Arc::new(MongoExternalModelRepository::new(client, db_name))
}

pub fn artifact_service_factory(
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
) -> ArtifactService {
    ArtifactService::new(
        artifact_repo_factory(client, db_name.clone()),
        artifact_ingestion_repo_factory(client, db_name.clone()),
        artifact_publication_repo_factory(client, db_name.clone()),
        model_repo_factory(client, db_name),
        Arc::new(RabbitMQArtifactOpMessagePublisher::new(channel)),
    )
}

pub fn model_creation_service_factory(client: &Client, db_name: String) -> ModelCreationService {
    ModelCreationService::new(
        model_repo_factory(client, db_name.clone()),
        external_model_repo_factory(client, db_name),
    )
}

pub fn model_query_service_factory(client: &Client, db_name: String) -> ModelQueryService {
    ModelQueryService::new(
        model_repo_factory(client, db_name.clone()),
        external_model_repo_factory(client, db_name),
    )
}

pub fn model_artifact_association_service_factory(
    client: &Client,
    db_name: String,
) -> ModelArtifactAssociationService {
    ModelArtifactAssociationService::new(
        model_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name),
    )
}

pub fn external_model_discovery_service_factory(
    client: &Client,
    db_name: String,
) -> ExternalModelDiscoveryService {
    ExternalModelDiscoveryService::new(external_model_repo_factory(client, db_name))
}
