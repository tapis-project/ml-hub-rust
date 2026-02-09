use amqprs::{
    channel::Channel,
    connection::{
        Connection, 
        OpenConnectionArguments,
    },
};
use crate::infra::messaging::rabbitmq::errors::BrokerError;

pub async fn open_channel(connection_args: OpenConnectionArguments) -> Result<Channel, BrokerError> {
    let conn = match Connection::open(&connection_args).await {
        Ok(conn) => conn,
        Err(err) => return Err(BrokerError::Connection(err.to_string()))
    };

    conn.open_channel(None).await.map_err(|err| BrokerError::Channel(err.to_string()))
}

