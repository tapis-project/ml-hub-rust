use crate::application::ports::events::Event;
use crate::application::ports::commands::Command;
use amqprs::channel::{
    Channel,
    ExchangeDeclareArguments,
    ExchangeType,
};
use crate::infra::messaging::rabbitmq::errors::BrokerError;
use log::info;

pub const ARTIFACT_INGESTION_EXCHANGE: &'static str = "exchange.artifact.ingest";
pub const ARTIFACT_PUBLICATION_EXCHANGE: &'static str = "exchange.artifact.publish";
pub const MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE: &'static str = "exchange.model_deployment.reconcile";
pub const DEAD_LETTER_EXCHANGE: &'static str = "exchange.dead_letter";

pub async fn declare_exchanges(channel: &Channel, exchanges: Vec<(&'static str, ExchangeType)>) -> Result<(), BrokerError> {
    for (exchange, exchange_type) in exchanges {
        let exchange_args = ExchangeDeclareArguments::new(
            exchange,
            exchange_type.to_string().as_str()
        ).finish();
        
        info!("Declaring exchange {} with type {}", exchange, exchange_type.to_string().as_str());
        channel.exchange_declare(exchange_args).await
            .map_err(|err| BrokerError::ExchangeDeclaration(err.to_string()))?
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
        _ => MODEL_DEPLOYMENT_RECONCILIATION_EXCHANGE,
    }
}