pub mod mappers;

#[derive(Debug, Clone)]
pub struct CreateAgentRecordInput {
    pub name: String,
    pub description: String,
    pub interfaces: Vec<AgentInterfaceInput>,
    pub capabilities: CapabilitiesInput,
    pub provider: Option<AgentProviderInput>,
    pub version: String,
    pub artifact_locators: Vec<ArtifactLocatorInput>,
    pub skills: Vec<AgentSkillInput>,
    pub icon_url: Option<String>,
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AgentInterfaceInput {
    pub name: String,
    pub description: Option<String>,
    pub protocol: ProtocolInput,
    pub message_binding: Option<MessageBindingInput>,
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
pub struct CapabilitiesInput {
    pub streaming: bool,
    pub push_notifications: bool,
}
#[derive(Debug, Clone)]
pub struct AgentProviderInput {
    pub organization: String,
    pub url: String,
}
#[derive(Debug, Clone)]
pub struct ArtifactLocatorInput {
    pub artifact_type: AgentArtifactTypeInput,
    pub url: String,
}
#[derive(Debug, Clone)]
pub enum AgentArtifactTypeInput {
    Binary,
    DockerImage,
    HelmChart,
    PythonPackage,
    SourceCode,
    Unspecified,
}
#[derive(Debug, Clone)]
pub struct AgentSkillInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
}
