#![cfg(test)]

use uuid::Uuid;

use super::{
    AgentInterface, AgentRecord, AgentRecordError, MessageBinding, Protocol,
    ReconstituteAgentRecordProps,
};

pub struct AgentRecordBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    tenant_id: Option<String>,
    owner: Option<String>,
    description: Option<String>,
    supported_interfaces: Option<Vec<AgentInterface>>,
}

impl AgentRecordBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            tenant_id: None,
            owner: None,
            description: None,
            supported_interfaces: None,
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_tenant_id(mut self, tenant_id: String) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    pub fn with_owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn with_supported_interfaces(mut self, supported_interfaces: Vec<AgentInterface>) -> Self {
        self.supported_interfaces = Some(supported_interfaces);
        self
    }

    pub fn build_new(&self) -> Result<AgentRecord, AgentRecordError> {
        AgentRecord::new(
            self.name
                .clone()
                .unwrap_or_else(|| "Test Agent Record".into()),
            self.tenant_id
                .clone()
                .unwrap_or_else(|| "test-tenant".into()),
            self.owner.clone().unwrap_or_else(|| "test-owner".into()),
            self.description.clone(),
            self.supported_interfaces.clone().unwrap_or_else(|| {
                vec![AgentInterface::new(
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                )]
            }),
        )
    }

    pub fn build_reconstituted(&self) -> Result<AgentRecord, AgentRecordError> {
        AgentRecord::reconstitute(ReconstituteAgentRecordProps {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            name: self
                .name
                .clone()
                .unwrap_or_else(|| "Test Agent Record".into()),
            tenant_id: self
                .tenant_id
                .clone()
                .unwrap_or_else(|| "test-tenant".into()),
            owner: self.owner.clone().unwrap_or_else(|| "test-owner".into()),
            description: self.description.clone(),
            supported_interfaces: self.supported_interfaces.clone().unwrap_or_else(|| {
                vec![AgentInterface::new(
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                )]
            }),
        })
    }
}
