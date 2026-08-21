#![cfg(test)]

use uuid::Uuid;

use super::{Agent, ReconstituteAgentProps};

pub struct AgentBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    tenant_id: Option<String>,
    description: Option<String>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            tenant_id: None,
            description: None,
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

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn build_new(&self) -> Agent {
        Agent::new(
            self.name.clone().unwrap_or_else(|| "Test Agent".into()),
            self.tenant_id.clone().unwrap_or_else(|| "test-tenant".into()),
            self.description.clone(),
        )
    }

    pub fn build_reconstituted(&self) -> Agent {
        Agent::reconstitute(ReconstituteAgentProps {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            name: self.name.clone().unwrap_or_else(|| "Test Agent".into()),
            tenant_id: self.tenant_id.clone().unwrap_or_else(|| "test-tenant".into()),
            description: self.description.clone(),
        })
    }
}
