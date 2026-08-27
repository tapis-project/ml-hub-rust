#[cfg(test)]
mod endpoint_catalog_service_test {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
    use crate::application::ports::errors::InfrastructureError;
    use crate::application::services::endpoint_catalog_service::{
        EndpointCatalogService, EndpointCatalogServiceError,
    };
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
            Vec::new()
        }

        fn resolve_target_url(&self, _target_name: &str) -> Option<&str> {
            None
        }
    }

    struct RecordingEndpointRepository {
        requested_tenant_id: Mutex<Option<String>>,
        requested_target_resource_urn: Mutex<Option<String>>,
    }

    impl RecordingEndpointRepository {
        fn new() -> Self {
            Self {
                requested_tenant_id: Mutex::new(None),
                requested_target_resource_urn: Mutex::new(None),
            }
        }
    }

    #[async_trait]
    impl EndpointRepository for RecordingEndpointRepository {
        async fn list_by_target_urn(
            &self,
            tenant_id: &str,
            target_resource_urn: &str,
        ) -> Result<Vec<Endpoint>, EndpointRepositoryError> {
            let mut requested_tenant_id = match self.requested_tenant_id.lock() {
                Ok(requested_tenant_id) => requested_tenant_id,
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            };
            *requested_tenant_id = Some(tenant_id.into());

            let mut requested_target_resource_urn = match self.requested_target_resource_urn.lock()
            {
                Ok(requested_target_resource_urn) => requested_target_resource_urn,
                Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
            };
            *requested_target_resource_urn = Some(target_resource_urn.into());

            Ok(Vec::new())
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

    #[tokio::test]
    async fn finds_endpoints_for_the_context_tenant_and_resource(
    ) -> Result<(), EndpointCatalogServiceError> {
        let repository = Arc::new(RecordingEndpointRepository::new());
        let service = EndpointCatalogService::new(repository.clone());
        let endpoints = service
            .find_by_network_addressable_resource(&RequestContext::system(None), &TestResource)
            .await?;

        assert!(endpoints.is_empty());

        let requested_tenant_id = match repository.requested_tenant_id.lock() {
            Ok(requested_tenant_id) => requested_tenant_id.clone(),
            Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
        };
        let requested_target_resource_urn = match repository.requested_target_resource_urn.lock() {
            Ok(requested_target_resource_urn) => requested_target_resource_urn.clone(),
            Err(error) => panic!("Endpoint repository mutex poisoned: {error}"),
        };

        assert_eq!(requested_tenant_id.as_deref(), Some("__GLOBAL__"));
        assert_eq!(
            requested_target_resource_urn.as_deref(),
            Some("urn:mlhub:v1:__GLOBAL__:agent:agent-a")
        );

        Ok(())
    }

    #[tokio::test]
    async fn propagates_repository_errors() {
        let service = EndpointCatalogService::new(Arc::new(FailingEndpointRepository));

        let result = service
            .find_by_network_addressable_resource(&RequestContext::system(None), &TestResource)
            .await;

        assert!(matches!(
            result,
            Err(EndpointCatalogServiceError::Repository(_))
        ));
    }
}
