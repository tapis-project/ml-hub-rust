#![cfg(test)]

use uuid::Uuid;

use super::{
    AgentInterface, AgentProvider, AgentRecord, AgentRecordError, AgentSkill, ArtifactLocator,
    Capabilities, MessageBinding, Protocol, ReconstituteAgentRecordProps,
};
use crate::shared_kernel::enums::Visibility;

pub struct AgentRecordBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    tenant_id: Option<String>,
    owner: Option<String>,
    description: Option<String>,
    interfaces: Option<Vec<AgentInterface>>,
    capabilities: Option<Capabilities>,
    provider: Option<AgentProvider>,
    version: Option<String>,
    artifact_locators: Option<Vec<ArtifactLocator>>,
    skills: Option<Vec<AgentSkill>>,
    icon_url: Option<String>,
    documentation_url: Option<String>,
    visibility: Option<Visibility>,
}

impl AgentRecordBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            tenant_id: None,
            owner: None,
            description: None,
            interfaces: None,
            capabilities: None,
            provider: None,
            version: None,
            artifact_locators: None,
            skills: None,
            icon_url: None,
            documentation_url: None,
            visibility: None,
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

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_interfaces(mut self, interfaces: Vec<AgentInterface>) -> Self {
        self.interfaces = Some(interfaces);
        self
    }

    pub fn with_capabilities(mut self, capabilities: Capabilities) -> Self {
        self.capabilities = Some(capabilities);
        self
    }

    pub fn with_provider(mut self, provider: AgentProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_artifact_locators(mut self, artifact_locators: Vec<ArtifactLocator>) -> Self {
        self.artifact_locators = Some(artifact_locators);
        self
    }

    pub fn with_skills(mut self, skills: Vec<AgentSkill>) -> Self {
        self.skills = Some(skills);
        self
    }

    pub fn with_icon_url(mut self, icon_url: String) -> Self {
        self.icon_url = Some(icon_url);
        self
    }

    pub fn with_documentation_url(mut self, documentation_url: String) -> Self {
        self.documentation_url = Some(documentation_url);
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
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
            self.description
                .clone()
                .unwrap_or_else(|| "Test agent record description".into()),
            self.interfaces.clone().unwrap_or_else(|| {
                vec![AgentInterface::new(
                    "default".into(),
                    Some("Default test interface".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    None,
                )]
            }),
            self.capabilities
                .clone()
                .unwrap_or_else(|| Capabilities::new(false, false)),
            self.provider.clone(),
            self.version.clone().unwrap_or_else(|| "0.1.0".into()),
            self.artifact_locators.clone().unwrap_or_default(),
            self.skills.clone().unwrap_or_default(),
            self.icon_url.clone(),
            self.documentation_url.clone(),
            self.visibility.clone().unwrap_or(Visibility::Private),
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
            description: self
                .description
                .clone()
                .unwrap_or_else(|| "Test agent record description".into()),
            interfaces: self.interfaces.clone().unwrap_or_else(|| {
                vec![AgentInterface::new(
                    "default".into(),
                    Some("Default test interface".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    None,
                )]
            }),
            capabilities: self
                .capabilities
                .clone()
                .unwrap_or_else(|| Capabilities::new(false, false)),
            provider: self.provider.clone(),
            version: self.version.clone().unwrap_or_else(|| "0.1.0".into()),
            artifact_locators: self.artifact_locators.clone().unwrap_or_default(),
            skills: self.skills.clone().unwrap_or_default(),
            icon_url: self.icon_url.clone(),
            documentation_url: self.documentation_url.clone(),
            visibility: self.visibility.clone().unwrap_or(Visibility::Private),
        })
    }
}
