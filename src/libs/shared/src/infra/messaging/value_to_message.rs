use serde_json::{Value, from_value};
use crate::infra::messaging::messages::{
    ModelDeploymentStateDriftDetectedPayload,
    ModelDeploymentStartedPayload,
    ModelDeploymentDeletedPayload,
    ModelDeploymentStoppedPayload,
    EventEnvelope,
};
use crate::infra::messaging::errors::SerializationError;


impl TryFrom<Value> for EventEnvelope {
    type Error = SerializationError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(from_value::<EventEnvelope>(value)
            .map_err(|err| Self::Error::DeserializationFailed(err.to_string()))?)
    }
}

impl TryFrom<Value> for ModelDeploymentStateDriftDetectedPayload {
    type Error = SerializationError;
    
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(from_value::<ModelDeploymentStateDriftDetectedPayload>(value)
            .map_err(|err| Self::Error::DeserializationFailed(err.to_string()))?)
    }
}

impl TryFrom<Value> for ModelDeploymentStartedPayload {
    type Error = SerializationError;
    
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(from_value::<ModelDeploymentStartedPayload>(value)
            .map_err(|err| Self::Error::DeserializationFailed(err.to_string()))?)
    }
}

impl TryFrom<Value> for ModelDeploymentDeletedPayload {
    type Error = SerializationError;
    
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(from_value::<ModelDeploymentDeletedPayload>(value)
            .map_err(|err| Self::Error::DeserializationFailed(err.to_string()))?)
    }
}

impl TryFrom<Value> for ModelDeploymentStoppedPayload {
    type Error = SerializationError;
    
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(from_value::<ModelDeploymentStoppedPayload>(value)
            .map_err(|err| Self::Error::DeserializationFailed(err.to_string()))?)
    }
}