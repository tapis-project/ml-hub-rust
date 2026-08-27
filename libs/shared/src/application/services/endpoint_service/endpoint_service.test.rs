#[cfg(test)]
mod endpoint_service_test {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
    use crate::application::ports::errors::InfrastructureError;
    use crate::application::services::endpoint_service::{EndpointService, EndpointServiceError};
    use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource};
    use crate::domain::entities::tenancy::TenantScopedResource;
    use crate::shared_kernel::context::RequestContext;
    use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

    struct TestResource;

    impl TenantScopedResource for TestResource {
        fn tenant_id(&self) -> String {
            "__GLOBAL__".into()
        }
    }

    impl UrnGenerator for TestResource {
        fn urn(&self) -> Urn {
            Urn::new("urn:mlhub:v1:__GLOBAL__:agent:agent-a".into())
        }
    }

    impl NetworkAddressableResource for TestResource {
        fn network_target_names(&self) -> Vec<String> {
            vec!["rest".into(), "rpc".into()]
        }

        fn resolve_target_url(&self, target_name: &str) -> Option<&str> {
            match target_name {
                "rest" => Some("https://agent.example.test"),
                "rpc" => Some("https://agent-rpc.example.test"),
                _ => None,
            }
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
            slug: &str,
        ) -> Result<Option<Endpoint>, EndpointRepositoryError> {
            let endpoints = match self.endpoints.lock() {
                Ok(endpoints) => endpoints,
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            };

            Ok(endpoints
                .iter()
                .find(|endpoint| endpoint.slug() == slug)
                .cloned())
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
            Err(EndpointRepositoryError::Persistence(
                InfrastructureError::new_internal(),
            ))
        }

        async fn save(&self, _endpoint: &Endpoint) -> Result<(), EndpointRepositoryError> {
            Err(EndpointRepositoryError::Persistence(
                InfrastructureError::new_internal(),
            ))
        }
    }

    #[tokio::test]
    async fn issues_and_reuses_endpoints_for_resource() -> Result<(), EndpointServiceError> {
        let repository = Arc::new(InMemoryEndpointRepository::new());
        let service = EndpointService::new(repository.clone());
        let ctx = RequestContext::system(None);

        let issued = service.issue_for_resource(&ctx, &TestResource).await?;

        assert_eq!(issued.len(), 2);
        assert_eq!(repository.saved_count(), 2);

        let reissued = service.issue_for_resource(&ctx, &TestResource).await?;

        assert_eq!(reissued.len(), 2);
        assert_eq!(repository.saved_count(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn propagates_repository_errors() {
        let service = EndpointService::new(Arc::new(FailingEndpointRepository));
        let ctx = RequestContext::system(None);

        let result = service.issue_for_resource(&ctx, &TestResource).await;

        assert!(matches!(result, Err(EndpointServiceError::Repository(_))));
    }
}
