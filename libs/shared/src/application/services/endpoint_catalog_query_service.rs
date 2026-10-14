use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::application::inputs::endpoint::ListEndpointsInput;
use crate::application::outputs::endpoint::{ResolvedEndpointListOutput, ResolvedEndpointOutput};
use crate::application::ports::agent::{AgentRepository, AgentRepositoryError};
use crate::application::ports::deployment::{
    ModelDeploymentRepository, ModelDeploymentRepositoryError,
};
use crate::application::ports::endpoint::{EndpointRepository, EndpointRepositoryError};
use crate::domain::entities::deployment::ModelDeploymentId;
use crate::domain::entities::endpoint::{Endpoint, NetworkAddressableResource, NetworkTargetError};
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::identifiers::urn::{NetworkResourceReference, Urn, UrnParseError};

pub struct EndpointCatalogQueryService {
    endpoint_repository: Arc<dyn EndpointRepository>,
    agent_repository: Arc<dyn AgentRepository>,
    deployment_repository: Arc<dyn ModelDeploymentRepository>,
}

impl EndpointCatalogQueryService {
    pub fn new(
        endpoint_repository: Arc<dyn EndpointRepository>,
        agent_repository: Arc<dyn AgentRepository>,
        deployment_repository: Arc<dyn ModelDeploymentRepository>,
    ) -> Self {
        Self {
            endpoint_repository,
            agent_repository,
            deployment_repository,
        }
    }

    pub async fn list(
        &self,
        ctx: &RequestContext,
        limit: Option<u16>,
        cursor: Option<String>,
        include_count: bool,
    ) -> Result<ResolvedEndpointListOutput, EndpointCatalogQueryServiceError> {
        let input = ListEndpointsInput::new(
            ctx.actor_tenant_id().clone(),
            ctx.actor_principal_id().clone(),
            None,
            limit,
            cursor,
            include_count,
        );

        self.list_with_input(ctx, &input).await
    }

    pub async fn get(
        &self,
        ctx: &RequestContext,
        id: &Uuid,
    ) -> Result<ResolvedEndpointOutput, EndpointCatalogQueryServiceError> {
        let endpoint = self
            .endpoint_repository
            .get_by_id(ctx.actor_tenant_id(), ctx.actor_principal_id(), id)
            .await?
            .ok_or(EndpointCatalogQueryServiceError::NotFound)?;

        self.resolve(ctx, endpoint).await
    }

    pub async fn list_for_resource(
        &self,
        ctx: &RequestContext,
        resource_urn: Urn,
        limit: Option<u16>,
        cursor: Option<String>,
        include_count: bool,
    ) -> Result<ResolvedEndpointListOutput, EndpointCatalogQueryServiceError> {
        self.authorize_resource(ctx, &resource_urn).await?;

        let input = ListEndpointsInput::new(
            ctx.actor_tenant_id().clone(),
            ctx.actor_principal_id().clone(),
            Some(resource_urn),
            limit,
            cursor,
            include_count,
        );

        self.list_with_input(ctx, &input).await
    }

    async fn list_with_input(
        &self,
        ctx: &RequestContext,
        input: &ListEndpointsInput,
    ) -> Result<ResolvedEndpointListOutput, EndpointCatalogQueryServiceError> {
        let output = self.endpoint_repository.list(input).await?;

        let mut references = Vec::with_capacity(output.endpoints.len());
        let mut agent_ids = HashSet::new();
        let mut deployment_ids = HashSet::new();

        for endpoint in output.endpoints {
            let reference = endpoint.target_resource_urn().parse_network_resource()?;

            if reference.tenant_id() != endpoint.tenant_id()
                || endpoint.tenant_id() != ctx.actor_tenant_id()
            {
                return Err(EndpointCatalogQueryServiceError::DataIntegrity(
                    "Endpoint tenant does not match its resource tenant".into(),
                ));
            }

            match &reference {
                NetworkResourceReference::Agent { id, .. } => {
                    agent_ids.insert(*id);
                }
                NetworkResourceReference::ModelDeployment { id, .. } => {
                    deployment_ids.insert(ModelDeploymentId::reconstitute(*id));
                }
            }

            references.push((endpoint, reference));
        }

        let agents = self
            .agent_repository
            .find_by_ids(
                ctx.actor_tenant_id(),
                &agent_ids.into_iter().collect::<Vec<_>>(),
            )
            .await?
            .into_iter()
            .map(|agent| (*agent.id(), agent))
            .collect::<HashMap<_, _>>();

        let deployments = self
            .deployment_repository
            .find_by_ids_and_tenant(
                ctx.actor_tenant_id(),
                &deployment_ids.into_iter().collect::<Vec<_>>(),
            )
            .await?
            .into_iter()
            .map(|deployment| (*deployment.id().as_uuid(), deployment))
            .collect::<HashMap<_, _>>();

        let mut endpoints = Vec::with_capacity(references.len());

        for (endpoint, reference) in references {
            let (owner, target_base_url) = match reference {
                NetworkResourceReference::Agent { id, .. } => {
                    let agent = agents.get(&id).ok_or_else(|| {
                        EndpointCatalogQueryServiceError::DataIntegrity(
                            "Endpoint references a missing Agent".into(),
                        )
                    })?;

                    let target = agent.resolve_network_target(endpoint.target_name())?;

                    (
                        agent.owner().to_owned(),
                        target.map(|target| target.base_url().to_string()),
                    )
                }
                NetworkResourceReference::ModelDeployment { id, .. } => {
                    let deployment = deployments.get(&id).ok_or_else(|| {
                        EndpointCatalogQueryServiceError::DataIntegrity(
                            "Endpoint references a missing ModelDeployment".into(),
                        )
                    })?;

                    let target = deployment.resolve_network_target(endpoint.target_name())?;

                    (
                        deployment.owner().to_owned(),
                        target.map(|target| target.base_url().to_string()),
                    )
                }
            };

            if owner != endpoint.owner() {
                return Err(EndpointCatalogQueryServiceError::DataIntegrity(
                    "Endpoint owner does not match its resource owner".into(),
                ));
            }

            endpoints.push(ResolvedEndpointOutput {
                endpoint,
                target_base_url,
            });
        }

        Ok(ResolvedEndpointListOutput {
            endpoints,
            cursor: output.cursor,
            count: output.count,
        })
    }

    async fn authorize_resource(
        &self,
        ctx: &RequestContext,
        resource_urn: &Urn,
    ) -> Result<(), EndpointCatalogQueryServiceError> {
        let reference = resource_urn.parse_network_resource()?;

        if reference.tenant_id() != ctx.actor_tenant_id() {
            return Err(EndpointCatalogQueryServiceError::Forbidden);
        }

        let owner = self
            .resource_owner(ctx.actor_tenant_id(), &reference)
            .await?;

        if &owner != ctx.actor_principal_id() {
            return Err(EndpointCatalogQueryServiceError::Forbidden);
        }

        Ok(())
    }

    async fn resolve(
        &self,
        ctx: &RequestContext,
        endpoint: Endpoint,
    ) -> Result<ResolvedEndpointOutput, EndpointCatalogQueryServiceError> {
        let reference = endpoint.target_resource_urn().parse_network_resource()?;

        if reference.tenant_id() != endpoint.tenant_id()
            || endpoint.tenant_id() != ctx.actor_tenant_id()
        {
            return Err(EndpointCatalogQueryServiceError::DataIntegrity(
                "Endpoint tenant does not match its resource tenant".into(),
            ));
        }

        let (owner, target_base_url) = match reference {
            NetworkResourceReference::Agent { id, .. } => {
                let agent = self
                    .agent_repository
                    .find_by_id(ctx.actor_tenant_id(), &id)
                    .await?
                    .ok_or_else(|| {
                        EndpointCatalogQueryServiceError::DataIntegrity(
                            "Endpoint references a missing Agent".into(),
                        )
                    })?;

                let target = agent.resolve_network_target(endpoint.target_name())?;

                (
                    agent.owner().to_owned(),
                    target.map(|target| target.base_url().to_string()),
                )
            }
            NetworkResourceReference::ModelDeployment { id, .. } => {
                let deployment_id = ModelDeploymentId::reconstitute(id);
                let deployment = self
                    .deployment_repository
                    .find_by_id_and_tenant(ctx.actor_tenant_id(), &deployment_id)
                    .await?
                    .ok_or_else(|| {
                        EndpointCatalogQueryServiceError::DataIntegrity(
                            "Endpoint references a missing ModelDeployment".into(),
                        )
                    })?;

                let target = deployment.resolve_network_target(endpoint.target_name())?;

                (
                    deployment.owner().to_owned(),
                    target.map(|target| target.base_url().to_string()),
                )
            }
        };

        if owner != endpoint.owner() {
            return Err(EndpointCatalogQueryServiceError::DataIntegrity(
                "Endpoint owner does not match its resource owner".into(),
            ));
        }

        Ok(ResolvedEndpointOutput {
            endpoint,
            target_base_url,
        })
    }

    async fn resource_owner(
        &self,
        tenant_id: &str,
        reference: &NetworkResourceReference,
    ) -> Result<String, EndpointCatalogQueryServiceError> {
        match reference {
            NetworkResourceReference::Agent { id, .. } => self
                .agent_repository
                .find_by_id(tenant_id, id)
                .await?
                .map(|agent| agent.owner().to_owned())
                .ok_or(EndpointCatalogQueryServiceError::ResourceNotFound),
            NetworkResourceReference::ModelDeployment { id, .. } => {
                let deployment_id = ModelDeploymentId::reconstitute(*id);

                self.deployment_repository
                    .find_by_id_and_tenant(tenant_id, &deployment_id)
                    .await?
                    .map(|deployment| deployment.owner().to_owned())
                    .ok_or(EndpointCatalogQueryServiceError::ResourceNotFound)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum EndpointCatalogQueryServiceError {
    #[error("Endpoint not found")]
    NotFound,

    #[error("Endpoint resource not found")]
    ResourceNotFound,

    #[error("Endpoint access is forbidden")]
    Forbidden,

    #[error("Endpoint data integrity error: {0}")]
    DataIntegrity(String),

    #[error(transparent)]
    EndpointRepository(#[from] EndpointRepositoryError),

    #[error(transparent)]
    AgentRepository(#[from] AgentRepositoryError),

    #[error(transparent)]
    DeploymentRepository(#[from] ModelDeploymentRepositoryError),

    #[error(transparent)]
    InvalidUrn(#[from] UrnParseError),

    #[error(transparent)]
    InvalidTarget(#[from] NetworkTargetError),
}
