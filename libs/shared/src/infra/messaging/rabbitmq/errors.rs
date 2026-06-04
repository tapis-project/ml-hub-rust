use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrokerError {
    #[error("Failed to open channel: {0}")]
    Channel(String),

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Queue bind failed: {0}")]
    QueueBind(String),

    #[error("Exchange declaration failed: {0}")]
    ExchangeDeclaration(String),
    
    #[error("Ack error: {0}")]
    Ack(String),

    #[error("Nack error: {0}")]
    Nack(String),
}