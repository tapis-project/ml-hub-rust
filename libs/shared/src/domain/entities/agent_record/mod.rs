use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AgentRecord {
    id: Uuid,
    name: String,
    tenant_id: String,
    description: Option<String>,
}

impl AgentRecord {
    pub fn new(name: String, tenant_id: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            tenant_id,
            description,
        }
    }

    pub fn reconstitute(props: ReconstituteAgentRecordProps) -> Self {
        Self {
            id: props.id,
            name: props.name,
            tenant_id: props.tenant_id,
            description: props.description,
        }
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

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteAgentRecordProps {
    pub id: Uuid,
    pub name: String,
    pub tenant_id: String,
    pub description: Option<String>,
}

#[cfg(test)]
pub mod test_fixtures;

#[cfg(test)]
#[path = "agent_record.test.rs"]
mod agent_record_test;
