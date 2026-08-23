use nonempty::NonEmpty;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AgentRecord {
    id: Uuid,
    name: String,
    tenant_id: String,
    owner: String,
    description: String,
    interfaces: NonEmpty<AgentInterface>,
}

impl AgentRecord {
    pub fn new(
        name: String,
        tenant_id: String,
        owner: String,
        description: String,
        interfaces: Vec<AgentInterface>,
    ) -> Result<Self, AgentRecordError> {
        let interfaces = Self::interfaces_from_vec(interfaces)?;
        Self::ensure_unique_interface_names(&interfaces)
            .map_err(AgentRecordError::DuplicateAgentInterfaceIdentifier)?;

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            tenant_id,
            owner,
            description,
            interfaces,
        })
    }

    pub fn reconstitute(props: ReconstituteAgentRecordProps) -> Result<Self, AgentRecordError> {
        let interfaces = Self::interfaces_from_vec(props.interfaces)?;
        Self::ensure_unique_interface_names(&interfaces).map_err(|duplicate_name| {
            AgentRecordError::DataIntegrityError(format!(
                "Agent record contains interfaces with duplicate names. Duplicate found: {duplicate_name}"
            ))
        })?;

        Ok(Self {
            id: props.id,
            name: props.name,
            tenant_id: props.tenant_id,
            owner: props.owner,
            description: props.description,
            interfaces,
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

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn interfaces(&self) -> &NonEmpty<AgentInterface> {
        &self.interfaces
    }

    fn interfaces_from_vec(
        interfaces: Vec<AgentInterface>,
    ) -> Result<NonEmpty<AgentInterface>, AgentRecordError> {
        NonEmpty::from_vec(interfaces).ok_or_else(|| {
            AgentRecordError::DataIntegrityError(
                "Agent record MUST have at least one supported interface".into(),
            )
        })
    }

    fn ensure_unique_interface_names(interfaces: &NonEmpty<AgentInterface>) -> Result<(), String> {
        let mut names = HashSet::new();

        for interface in interfaces {
            if !names.insert(interface.name()) {
                return Err(interface.name().clone());
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentRecordProps {
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub owner: String,
    pub description: String,
    pub interfaces: Vec<AgentInterface>,
}

#[derive(Clone, Debug)]
pub struct AgentInterface {
    name: String,
    description: Option<String>,
    protocol: Protocol,
    message_binding: Option<MessageBinding>,
}

impl AgentInterface {
    pub fn new(
        name: String,
        description: Option<String>,
        protocol: Protocol,
        message_binding: Option<MessageBinding>,
    ) -> Self {
        Self {
            name,
            description,
            protocol,
            message_binding,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
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
    #[error("Duplicate agent interface identifier: {0}")]
    DuplicateAgentInterfaceIdentifier(String),

    #[error("Data integrity error: {0}")]
    DataIntegrityError(String),
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
use std::collections::HashSet;
