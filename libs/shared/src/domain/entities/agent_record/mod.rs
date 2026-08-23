use nonempty::NonEmpty;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AgentRecord {
    id: Uuid,
    name: String,
    tenant_id: String,
    owner: String,
    description: Option<String>,
    supported_interfaces: NonEmpty<AgentInterface>,
}

impl AgentRecord {
    pub fn new(
        name: String,
        tenant_id: String,
        owner: String,
        description: Option<String>,
        supported_interfaces: Vec<AgentInterface>,
    ) -> Result<Self, AgentRecordError> {
        let supported_interfaces = Self::supported_interfaces_from_vec(supported_interfaces)?;

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            tenant_id,
            owner,
            description,
            supported_interfaces,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentRecordProps) -> Result<Self, AgentRecordError> {
        let supported_interfaces = Self::supported_interfaces_from_vec(props.supported_interfaces)?;

        Ok(Self {
            id: props.id,
            name: props.name,
            tenant_id: props.tenant_id,
            owner: props.owner,
            description: props.description,
            supported_interfaces,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn tenant_id(&self) -> &String {
        &self.tenant_id
    }

    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn supported_interfaces(&self) -> &NonEmpty<AgentInterface> {
        &self.supported_interfaces
    }

    fn supported_interfaces_from_vec(
        supported_interfaces: Vec<AgentInterface>,
    ) -> Result<NonEmpty<AgentInterface>, AgentRecordError> {
        NonEmpty::from_vec(supported_interfaces).ok_or_else(|| {
            AgentRecordError::DataIntegrityError(
                "Agent record MUST have at least one supported interface".into(),
            )
        })
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentRecordProps {
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: Option<String>,
    pub supported_interfaces: Vec<AgentInterface>,
}

#[derive(Clone, Debug)]
pub struct AgentInterface {
    protocol: Protocol,
    message_binding: Option<MessageBinding>,
}

impl AgentInterface {
    pub fn new(protocol: Protocol, message_binding: Option<MessageBinding>) -> Self {
        Self {
            protocol,
            message_binding,
        }
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn message_binding(&self) -> &Option<MessageBinding> {
        &self.message_binding
    }
}

#[derive(Clone, Debug)]
pub enum Protocol {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Clone, Debug)]
pub enum MessageBinding {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Debug, Error, Clone)]
pub enum AgentRecordError {
    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
