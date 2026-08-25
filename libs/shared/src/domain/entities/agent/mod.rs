use nonempty::NonEmpty;
use uuid::Uuid;

use crate::domain::entities::agent_record::{AgentRecord, MessageBinding, Protocol};
use crate::shared_kernel::enums::Visibility;
use crate::shared_kernel::value_objects::TimeStamp;

pub struct Agent {
    id: Uuid,
    tenant_id: String,
    name: String,
    owner: String,
    description: Option<String>,
    deployment_modality: AgentDeploymentModality,
    liveness: AgentLiveness,
    target_endpoints: NonEmpty<AgentEndpoint>,
    created_at: TimeStamp,
    visibility: Visibility,
    last_modified: TimeStamp,
    last_failed_heartbeat: Option<TimeStamp>,
}

impl Agent {
    pub async fn register_from_agent_record(props: RegisterAgentFromAgentRecordProps) -> Self {
        unimplemented!("Unimplemented")
    }
}

pub struct RegisterAgentFromAgentRecordProps {
    pub agent_record: AgentRecord,
}

pub struct AgentEndpoint {
    protocol: Protocol,
    message_binding: Option<MessageBinding>,
    url: Option<String>,
}

pub struct LivenessProbeEndpoint {
    url: String,
}

pub enum AgentLiveness {
    Alive,
    Dead,
}

pub enum AgentDeploymentModality {
    Persistent,
    OnDemand,
}
