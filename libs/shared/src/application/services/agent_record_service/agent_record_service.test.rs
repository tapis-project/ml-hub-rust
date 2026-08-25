use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use super::*;
use crate::application::inputs::agent_record::{
    AgentArtifactTypeInput, AgentInterfaceInput, AgentSkillInput, ArtifactLocatorInput,
    CapabilitiesInput, CreateAgentRecordInput, MessageBindingInput, ProtocolInput, VisibilityInput,
};
use crate::application::ports::agent_record::AgentRecordRepository;
use crate::domain::entities::agent_record::AgentSkill;

struct TestAgentRecordRepository {
    saved: Mutex<Option<AgentRecord>>,
    owner_list_calls: Mutex<Vec<(String, String)>>,
    shared_list_calls: Mutex<Vec<(String, String)>>,
}

#[async_trait]
impl AgentRecordRepository for TestAgentRecordRepository {
    async fn save(&self, agent_record: &AgentRecord) -> Result<(), AgentRecordRepositoryError> {
        let mut saved = match self.saved.lock() {
            Ok(saved) => saved,
            Err(poisoned) => poisoned.into_inner(),
        };
        *saved = Some(agent_record.clone());
        Ok(())
    }

    async fn list_by_owner(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError> {
        let mut calls = match self.owner_list_calls.lock() {
            Ok(calls) => calls,
            Err(poisoned) => poisoned.into_inner(),
        };
        calls.push((tenant_id.into(), owner.into()));
        Ok(Vec::new())
    }

    async fn list_shared_with_user(
        &self,
        tenant_id: &str,
        owner: &str,
    ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError> {
        let mut calls = match self.shared_list_calls.lock() {
            Ok(calls) => calls,
            Err(poisoned) => poisoned.into_inner(),
        };
        calls.push((tenant_id.into(), owner.into()));
        Ok(Vec::new())
    }
}

fn input() -> CreateAgentRecordInput {
    CreateAgentRecordInput {
        name: "Test Agent".into(),
        description: "A test agent.".into(),
        interfaces: vec![AgentInterfaceInput {
            name: "rest".into(),
            description: Some("REST interface".into()),
            protocol: ProtocolInput::RestHttp,
            message_binding: Some(MessageBindingInput::HttpJson),
        }],
        capabilities: CapabilitiesInput {
            streaming: true,
            push_notifications: true,
        },
        provider: None,
        version: "1.0.0".into(),
        artifact_locators: vec![ArtifactLocatorInput {
            artifact_type: AgentArtifactTypeInput::DockerImage,
            url: "tapis://example/agent:1.0.0".into(),
        }],
        skills: vec![AgentSkillInput {
            id: "geospatial-search".into(),
            name: "Geospatial search".into(),
            description: "Searches geospatial data.".into(),
            tags: vec!["geospatial".into()],
            examples: vec!["Find flood zones.".into()],
        }],
        icon_url: Some("https://example.com/icon.svg".into()),
        documentation_url: Some("https://example.com/docs".into()),
        visibility: VisibilityInput::Private,
    }
}

#[tokio::test]
async fn create_agent_record_derives_owner_and_tenant_from_context(
) -> Result<(), AgentRecordServiceError> {
    let repository = Arc::new(TestAgentRecordRepository {
        saved: Mutex::new(None),
        owner_list_calls: Mutex::new(Vec::new()),
        shared_list_calls: Mutex::new(Vec::new()),
    });
    let service = AgentRecordService::new(repository.clone());
    let context = RequestContext::system(None);
    let created = service.create_agent_record(&context, input()).await?;
    let saved = match repository.saved.lock() {
        Ok(saved) => saved,
        Err(poisoned) => poisoned.into_inner(),
    };
    let saved = match saved.as_ref() {
        Some(saved) => saved,
        None => panic!("AgentRecord repository should receive the created record"),
    };
    assert_eq!(created.tenant_id(), context.actor_tenant_id());
    assert_eq!(created.owner(), context.actor_principal_id());
    assert_eq!(saved.id(), created.id());
    assert!(saved.supports_streaming());
    assert!(saved.supports_push_notifications());
    assert_eq!(
        saved.skills().first().map(AgentSkill::id),
        Some("geospatial-search")
    );
    Ok(())
}

#[tokio::test]
async fn list_for_user_uses_context_tenant_and_principal() -> Result<(), AgentRecordServiceError> {
    let repository = Arc::new(TestAgentRecordRepository {
        saved: Mutex::new(None),
        owner_list_calls: Mutex::new(Vec::new()),
        shared_list_calls: Mutex::new(Vec::new()),
    });
    let service = AgentRecordService::new(repository.clone());
    let context = RequestContext::system(None);

    let agent_records = service.list_for_user(&context).await?;

    assert!(agent_records.is_empty());
    let calls = match repository.owner_list_calls.lock() {
        Ok(calls) => calls,
        Err(poisoned) => poisoned.into_inner(),
    };
    assert_eq!(
        calls.as_slice(),
        &[(
            context.actor_tenant_id().clone(),
            context.actor_principal_id().clone()
        )]
    );
    Ok(())
}

#[tokio::test]
async fn list_shared_with_user_uses_context_tenant_and_principal(
) -> Result<(), AgentRecordServiceError> {
    let repository = Arc::new(TestAgentRecordRepository {
        saved: Mutex::new(None),
        owner_list_calls: Mutex::new(Vec::new()),
        shared_list_calls: Mutex::new(Vec::new()),
    });
    let service = AgentRecordService::new(repository.clone());
    let context = RequestContext::system(None);

    let agent_records = service.list_shared_with_user(&context).await?;

    assert!(agent_records.is_empty());
    let calls = match repository.shared_list_calls.lock() {
        Ok(calls) => calls,
        Err(poisoned) => poisoned.into_inner(),
    };
    assert_eq!(
        calls.as_slice(),
        &[(
            context.actor_tenant_id().clone(),
            context.actor_principal_id().clone()
        )]
    );
    Ok(())
}
