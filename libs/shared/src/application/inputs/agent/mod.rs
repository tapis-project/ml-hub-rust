pub mod mappers;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RegisterAgentInput {
    pub name: String,
    pub description: String,
    pub deployment_modality: AgentDeploymentModalityInput,
    pub endpoints: Vec<AgentEndpointInput>,
    pub tags: Vec<String>,
    pub agent_record_id: Option<Uuid>,
    pub visibility: VisibilityInput,
}

#[derive(Debug, Clone)]
pub struct AgentEndpointInput {
    pub name: Option<String>,
    pub protocol: ProtocolInput,
    pub message_binding: Option<MessageBindingInput>,
    pub base_url: Option<String>,
    pub liveness_probe: Option<LivenessProbeConfigurationInput>,
}

#[derive(Debug, Clone)]
pub enum ProtocolInput {
    RestHttp,
    Rpc,
    Stdio,
}

#[derive(Debug, Clone)]
pub enum MessageBindingInput {
    HttpJson,
    JsonRpc2_0,
    Grpc,
}

#[derive(Debug, Clone)]
pub enum LivenessProbeConfigurationInput {
    RestHttp {
        route: String,
        interval_seconds: u32,
        timeout_seconds: u32,
        missed_heartbeat_threshold: u16,
        initial_delay_seconds: u32,
    },
}

#[derive(Debug, Clone)]
pub enum AgentDeploymentModalityInput {
    Persistent,
    OnDemand,
}

#[derive(Debug, Clone)]
pub enum VisibilityInput {
    Public,
    Private,
}
