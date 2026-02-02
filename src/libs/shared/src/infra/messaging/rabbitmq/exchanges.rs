use crate::application::ports::events::Event;
use crate::application::ports::commands::{
    CommandPublisherError,
    Command,
};
use amqprs::channel::{
    Channel,
    ExchangeDeclareArguments,
};


pub const ARTIFACT_INGESTION_EXCHANGE: &'static str = "exchange.artifact.ingest";
pub const ARTIFACT_PUBLICATION_EXCHANGE: &'static str = "exchange.artifact.publish";
pub const MODEL_DEPLOYMENT_EXCHANGE: &'static str = "exchange.model.deploy";

pub async fn delcare_exchanges(channel: &Channel, exchanges: Vec<(&'static str, &str)>) -> Result<(), CommandPublisherError> {
    for (exchange, exchange_type) in exchanges {
        let exchange_args = ExchangeDeclareArguments::new(exchange, exchange_type);
        channel.exchange_declare(exchange_args).await
            .map_err(|err| CommandPublisherError::ConnectionError(err.to_string()))?
    }
    Ok(())
}

pub fn get_exchange_for_command(command: &Command) -> &'static str {
    match command {
        Command::IngestArtifactCommand(_) => ARTIFACT_INGESTION_EXCHANGE,
        Command::PublishArtifactCommand(_) => ARTIFACT_PUBLICATION_EXCHANGE,
    }
}

pub fn get_exchange_for_event(event: &Event) -> &'static str {
    match event {
        _ => MODEL_DEPLOYMENT_EXCHANGE,
    }
}