//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::ports::artifacts::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    ArtifactPublicationRepository,
};
use shared::application::ports::model::ModelRepository;
use shared::application::services::artifact_service::ArtifactService;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::persistence::mongo::repositories::{
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
    ModelRepository as MongoModelRepository,
};
use shared::infra::messaging::rabbitmq::artifact_op_message_publisher::RabbitMQArtifactOpMessagePublisher;
use std::sync::Arc;

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name.clone()))
}

pub fn artifact_ingestion_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactIngestionRepository> {
    Arc::new(MongoArtifactIngestionRepository::new(client, db_name.clone()))
}

pub fn artifact_publication_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactPublicationRepository> {
    Arc::new(MongoArtifactPublicationRepository::new(client, db_name.clone()))
}

pub fn model_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelRepository> {
    Arc::new(MongoModelRepository::new(client, db_name.clone()))
}

pub fn artifact_service_factory(client: &Client, db_name: String, channel: Arc<Channel>) -> ArtifactService {    
    ArtifactService::new(
        artifact_repo_factory(client, db_name.clone()),
        artifact_ingestion_repo_factory(client, db_name.clone()),
        artifact_publication_repo_factory(client, db_name.clone()),
        model_repo_factory(client, db_name.clone()),
        Arc::new(RabbitMQArtifactOpMessagePublisher::new(channel.clone()))
    )
}