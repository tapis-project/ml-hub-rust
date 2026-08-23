use crate::domain::entities::agent_record as entities;
use crate::presentation::http::v1::responses::agent_records as responses;

impl From<entities::AgentRecord> for responses::AgentRecord {
    fn from(value: entities::AgentRecord) -> Self {
        Self {
            id: *value.id(),
            name: value.name().clone(),
            tenant_id: value.tenant_id().clone(),
            owner: value.owner().clone(),
            description: value.description().clone(),
            interfaces: value
                .interfaces()
                .iter()
                .cloned()
                .map(responses::AgentInterface::from)
                .collect(),
        }
    }
}

impl From<entities::AgentInterface> for responses::AgentInterface {
    fn from(value: entities::AgentInterface) -> Self {
        Self {
            name: value.name().clone(),
            description: value.description().clone(),
            protocol: value.protocol().clone().into(),
            message_binding: value.message_binding().clone().map(Into::into),
        }
    }
}

impl From<entities::Protocol> for responses::Protocol {
    fn from(value: entities::Protocol) -> Self {
        match value {
            entities::Protocol::RestHttp => Self::RestHttp,
            entities::Protocol::Rpc => Self::Rpc,
            entities::Protocol::Stdio => Self::Stdio,
        }
    }
}

impl From<entities::MessageBinding> for responses::MessageBinding {
    fn from(value: entities::MessageBinding) -> Self {
        match value {
            entities::MessageBinding::HttpJson => Self::HttpJson,
            entities::MessageBinding::JsonRpc2_0 => Self::JsonRpc2_0,
            entities::MessageBinding::Grpc => Self::Grpc,
        }
    }
}
