use amqprs::{
    channel::{BasicAckArguments, BasicNackArguments, Channel},
    Deliver
};
use crate::infra::messaging::rabbitmq::errors::BrokerError;

pub async fn ack(channel: &Channel, deliver: &Deliver, multiple: Option<bool>) -> Result<(), BrokerError> {
    let args = BasicAckArguments {
        delivery_tag: deliver.delivery_tag(),
        multiple: multiple.unwrap_or(false)
    };

    match channel.basic_ack(args).await {
        Ok(_) => Ok(()),
        Err(err) => Err(BrokerError::Ack(err.to_string()))
    }
}

pub async fn nack(channel: &Channel, deliver: &Deliver, requeue: Option<bool>, multiple: Option<bool>) -> Result<(), BrokerError> {
    let args = BasicNackArguments {
        delivery_tag: deliver.delivery_tag(),
        requeue: requeue.unwrap_or(false),
        multiple: multiple.unwrap_or(false)
    };

    match channel.basic_nack(args).await {
        Ok(_) => Ok(()),
        Err(err) => Err(BrokerError::Nack(err.to_string()))
    }
}