#[cfg(test)]
mod agent_service_test {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use crate::application::inputs::agent::{
        AgentDeploymentModalityInput, AgentEndpointInput, MessageBindingInput, ProtocolInput,
        RegisterAgentInput, VisibilityInput,
    };
    use crate::application::ports::agent::{AgentRepository, AgentRepositoryError};
    use crate::application::ports::agent_record::{
        AgentRecordRepository, AgentRecordRepositoryError,
    };
    use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
    use crate::application::ports::errors::InfrastructureError;
    use crate::application::services::agent_service::{AgentService, AgentServiceError};
    use crate::application::services::endpoint_issuance_service::EndpointIssuanceService;
    use crate::domain::entities::agent::Agent;
    use crate::domain::entities::agent_record::AgentRecord;
    use crate::domain::entities::endpoint::Endpoint;
    use crate::shared_kernel::context::RequestContext;

    struct InMemoryAgentRepository {
        agents: Mutex<Vec<Agent>>,
    }

    impl InMemoryAgentRepository {
        fn new() -> Self {
            Self {
                agents: Mutex::new(Vec::new()),
            }
        }

        fn saved_count(&self) -> usize {
            match self.agents.lock() {
                Ok(agents) => agents.len(),
                Err(error) => panic!("Agent repository mutex poisoned: {error}"),
            }
        }
    }

    #[async_trait]
    impl AgentRepository for InMemoryAgentRepository {
        async fn save(&self, agent: &Agent) -> Result<(), AgentRepositoryError> {
            let mut agents = match self.agents.lock() {
                Ok(agents) => agents,
                Err(error) => panic!("Agent repository mutex poisoned: {error}"),
            };

            agents.push(agent.clone());

            Ok(())
        }

        async fn list_by_owner(
            &self,
            _tenant_id: &str,
            _owner: &str,
        ) -> Result<Vec<Agent>, AgentRepositoryError> {
            Ok(Vec::new())
        }

        async fn list_shared_with_user(
            &self,
            _tenant_id: &str,
            _owner: &str,
        ) -> Result<Vec<Agent>, AgentRepositoryError> {
            Ok(Vec::new())
        }
    }

    struct EmptyAgentRecordRepository;

    #[async_trait]
    impl AgentRecordRepository for EmptyAgentRecordRepository {
        async fn save(
            &self,
            _agent_record: &AgentRecord,
        ) -> Result<(), AgentRecordRepositoryError> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _tenant_id: &str,
            _id: uuid::Uuid,
        ) -> Result<Option<AgentRecord>, AgentRecordRepositoryError> {
            Ok(None)
        }

        async fn list_by_owner(
            &self,
            _tenant_id: &str,
            _owner: &str,
        ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError> {
            Ok(Vec::new())
        }

        async fn list_shared_with_user(
            &self,
            _tenant_id: &str,
            _owner: &str,
        ) -> Result<Vec<AgentRecord>, AgentRecordRepositoryError> {
            Ok(Vec::new())
        }
    }

    struct InMemoryEndpointRepository {
        endpoints: Mutex<Vec<Endpoint>>,
    }

    impl InMemoryEndpointRepository {
        fn new() -> Self {
            Self {
                endpoints: Mutex::new(Vec::new()),
            }
        }

        fn saved_count(&self) -> usize {
            match self.endpoints.lock() {
                Ok(endpoints) => endpoints.len(),
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            }
        }
    }

    #[async_trait]
    impl EndpointRepository for InMemoryEndpointRepository {
        async fn list_by_target_urn(
            &self,
            tenant_id: &str,
            target_resource_urn: &str,
        ) -> Result<Vec<Endpoint>, EndpointRepositoryError> {
            let endpoints = match self.endpoints.lock() {
                Ok(endpoints) => endpoints,
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            };

            Ok(endpoints
                .iter()
                .filter(|endpoint| {
                    endpoint.tenant_id() == tenant_id
                        && endpoint.target_resource_urn().as_str() == target_resource_urn
                })
                .cloned()
                .collect())
        }

        async fn get_by_slug(
            &self,
            _slug: &str,
        ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
            Ok(None)
        }

        async fn save(&self, endpoint: &Endpoint) -> Result<(), EndpointRepositoryError> {
            let mut endpoints = match self.endpoints.lock() {
                Ok(endpoints) => endpoints,
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            };

            endpoints.push(endpoint.clone());

            Ok(())
        }
    }

    struct FailingEndpointRepository;

    #[async_trait]
    impl EndpointRepository for FailingEndpointRepository {
        async fn list_by_target_urn(
            &self,
            _tenant_id: &str,
            _target_resource_urn: &str,
        ) -> Result<Vec<Endpoint>, EndpointRepositoryError> {
            Err(EndpointRepositoryError::Persistence(
                InfrastructureError::new_internal(),
            ))
        }

        async fn get_by_slug(
            &self,
            _slug: &str,
        ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
            Ok(None)
        }

        async fn save(&self, _endpoint: &Endpoint) -> Result<(), EndpointRepositoryError> {
            Ok(())
        }
    }

    fn input() -> RegisterAgentInput {
        RegisterAgentInput {
            name: "Agent".into(),
            description: "Description".into(),
            deployment_modality: AgentDeploymentModalityInput::Persistent,
            endpoints: vec![AgentEndpointInput {
                name: Some("rest".into()),
                protocol: ProtocolInput::RestHttp,
                message_binding: Some(MessageBindingInput::HttpJson),
                base_url: Some("https://agent.example.test".into()),
                liveness_probe: None,
            }],
            tags: Vec::new(),
            agent_record_id: None,
            visibility: VisibilityInput::Private,
        }
    }

    fn input_without_network_target() -> RegisterAgentInput {
        RegisterAgentInput {
            endpoints: vec![AgentEndpointInput {
                name: None,
                protocol: ProtocolInput::Stdio,
                message_binding: None,
                base_url: None,
                liveness_probe: None,
            }],
            ..input()
        }
    }

    #[tokio::test]
    async fn registers_an_agent_and_issues_its_endpoints() -> Result<(), AgentServiceError> {
        let agent_repository = Arc::new(InMemoryAgentRepository::new());
        let endpoint_repository = Arc::new(InMemoryEndpointRepository::new());
        let service = AgentService::new(
            agent_repository.clone(),
            Arc::new(EmptyAgentRecordRepository),
            EndpointIssuanceService::new(endpoint_repository.clone()),
        );

        service
            .register_agent(&RequestContext::system(None), input())
            .await?;

        assert_eq!(agent_repository.saved_count(), 1);
        assert_eq!(endpoint_repository.saved_count(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn registers_an_agent_without_issuing_endpoints_when_it_has_no_network_targets(
    ) -> Result<(), AgentServiceError> {
        let agent_repository = Arc::new(InMemoryAgentRepository::new());
        let endpoint_repository = Arc::new(InMemoryEndpointRepository::new());
        let service = AgentService::new(
            agent_repository.clone(),
            Arc::new(EmptyAgentRecordRepository),
            EndpointIssuanceService::new(endpoint_repository.clone()),
        );

        service
            .register_agent(
                &RequestContext::system(None),
                input_without_network_target(),
            )
            .await?;

        assert_eq!(agent_repository.saved_count(), 1);
        assert_eq!(endpoint_repository.saved_count(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn returns_endpoint_issuance_failure_after_persisting_agent() {
        let agent_repository = Arc::new(InMemoryAgentRepository::new());
        let service = AgentService::new(
            agent_repository.clone(),
            Arc::new(EmptyAgentRecordRepository),
            EndpointIssuanceService::new(Arc::new(FailingEndpointRepository)),
        );

        let result = service
            .register_agent(&RequestContext::system(None), input())
            .await;

        assert!(matches!(
            result,
            Err(AgentServiceError::EndpointIssuance(_))
        ));
        assert_eq!(agent_repository.saved_count(), 1);
    }
}
