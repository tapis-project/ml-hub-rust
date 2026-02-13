use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::workflows::reconciliation::{
    ReconciliationError, ReconciliationOutcome, ReconciliationAction,
    StartedOutcomePayload, StoppedOutcomePayload, UndeployedOutcomePayload, ObeservedOutcomePayload
};
use crate::domain::entities::deployment::State;
use flexserv_deployer::{FlexServPodDeployment, PodDeploymentOptions, FlexServInstance, Backend, normalize_tenant_url};
use flexserv_deployer::deployment::{DeploymentError as FlexServDeploymentError, DeploymentResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn};

pub struct TapisPodsModelDeploymentReconciliationClient {
    /// Cache of active deployments by deployment ID (stored as serializable data)
    deployments: Arc<Mutex<HashMap<uuid::Uuid, DeploymentInfo>>>,
}

/// Information needed to recreate a FlexServ deployment
#[derive(Clone)]
struct DeploymentInfo {
    pod_id: String,
    volume_id: String,
    tenant_url: String,
    tapis_user: String,
    tapis_token: String,
    model_id: String,
    backend: Backend,
}

impl TapisPodsModelDeploymentReconciliationClient {
    pub fn new() -> Self {
        Self {
            deployments: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Extract Tapis credentials from deployment metadata or environment variables
    fn extract_tapis_credentials(deployment: &crate::domain::entities::deployment::ModelDeployment) -> Result<(String, String, String), ReconciliationError> {
        // Try to get from metadata first
        if let Some(metadata) = &deployment.metadata {
            if let Some(tenant_url) = metadata.get("tapis_tenant_url").and_then(|v| v.as_str()) {
                if let Some(tapis_user) = metadata.get("tapis_user").and_then(|v| v.as_str()) {
                    if let Some(tapis_token) = metadata.get("tapis_token").and_then(|v| v.as_str()) {
                        return Ok((tenant_url.to_string(), tapis_user.to_string(), tapis_token.to_string()));
                    }
                }
            }
        }

        // Fall back to environment variables
        let tenant_url = std::env::var("TAPIS_TENANT_URL")
            .map_err(|_| ReconciliationError::Unimplemented("TAPIS_TENANT_URL not found in environment or deployment metadata".into()))?;
        let tapis_user = std::env::var("TAPIS_USER")
            .or_else(|_| Ok::<String, _>(deployment.owner.clone()))
            .map_err(|_| ReconciliationError::Unimplemented("TAPIS_USER not found in environment or deployment metadata".into()))?;
        let tapis_token = std::env::var("TAPIS_TOKEN")
            .map_err(|_| ReconciliationError::Unimplemented("TAPIS_TOKEN not found in environment or deployment metadata".into()))?;

        Ok((tenant_url, tapis_user, tapis_token))
    }

    /// Extract pod_id and volume_id from deployment metadata
    fn extract_pod_info(deployment: &crate::domain::entities::deployment::ModelDeployment) -> Option<(String, String)> {
        deployment.metadata.as_ref().and_then(|m| {
            let pod_id = m.get("pod_id")?.as_str()?.to_string();
            let volume_id = m.get("volume_id")?.as_str()?.to_string();
            Some((pod_id, volume_id))
        })
    }

    /// Determine backend from model metadata or use default
    fn determine_backend(_model_metadata: &crate::domain::entities::model_metadata::ModelMetadata) -> Backend {
        // Default to Transformers backend
        // TODO: Extract backend from model metadata if available
        Backend::Transformers {
            command: vec!["python".to_string()],
        }
    }

    /// Get or create deployment info
    async fn get_or_create_deployment_info(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<DeploymentInfo, ReconciliationError> {
        let mut deployments = self.deployments.lock().await;

        // Check if we have an existing deployment info
        if let Some(existing) = deployments.get(&input.deployment.id) {
            return Ok(existing.clone());
        }

        let (tenant_url, tapis_user, tapis_token) = Self::extract_tapis_credentials(&input.deployment)?;
        let model_id = format!("{}/{}", input.deployment.model.author, input.deployment.model.name);
        let backend = Self::determine_backend(&input.model_metadata);

        // Check if pod_id and volume_id exist in metadata (for existing deployments)
        let (pod_id, volume_id) = if let Some((pid, vid)) = Self::extract_pod_info(&input.deployment) {
            (pid, vid)
        } else {
            // Will be set after creation
            (String::new(), String::new())
        };

        let info = DeploymentInfo {
            pod_id,
            volume_id,
            tenant_url: tenant_url.clone(),
            tapis_user: tapis_user.clone(),
            tapis_token: tapis_token.clone(),
            model_id,
            backend,
        };

        deployments.insert(input.deployment.id, info.clone());
        Ok(info)
    }

    /// Create a FlexServPodDeployment from DeploymentInfo
    fn create_deployment_from_info(&self, info: &DeploymentInfo, deployment_id: uuid::Uuid) -> Result<FlexServPodDeployment, ReconciliationError> {
        let server = FlexServInstance::builder()
            .tenant_url(normalize_tenant_url(&info.tenant_url))
            .tapis_user(info.tapis_user.clone())
            .model(info.model_id.clone())
            .backend(info.backend.clone())
            .build()
            .map_err(|e| ReconciliationError::Unimplemented(format!("Failed to create FlexServInstance: {}", e)))?;

        if !info.pod_id.is_empty() && !info.volume_id.is_empty() {
            Ok(FlexServPodDeployment::from_existing(
                server,
                info.tapis_token.clone(),
                info.pod_id.clone(),
                info.volume_id.clone(),
            ))
        } else {
            let mut options = PodDeploymentOptions::default();
            // Use the deployment UUID as deployment_id so FlexServ can derive pod_id and volume_id from it
            options.deployment_id = Some(deployment_id.to_string());
            Ok(FlexServPodDeployment::with_options(
                server,
                info.tapis_token.clone(),
                options,
            ))
        }
    }

    /// Map FlexServ DeploymentError to ReconciliationError
    fn map_deployment_error(err: FlexServDeploymentError) -> ReconciliationError {
        match err {
            FlexServDeploymentError::TapisAuthFailed(msg) => {
                ReconciliationError::Unimplemented(format!("TAPIS authentication failed: {}", msg))
            }
            FlexServDeploymentError::TapisAPIUnreachable(msg) => {
                ReconciliationError::Unimplemented(format!("TAPIS API unreachable: {}", msg))
            }
            FlexServDeploymentError::TapisBadRequest(msg) => {
                ReconciliationError::Unimplemented(format!("TAPIS bad request: {}", msg))
            }
            FlexServDeploymentError::TapisTimeout(msg) => {
                ReconciliationError::Unimplemented(format!("TAPIS timeout: {}", msg))
            }
            FlexServDeploymentError::TapisInternalServerError(msg) => {
                ReconciliationError::Unimplemented(format!("TAPIS internal server error: {}", msg))
            }
            FlexServDeploymentError::ModelUploadingFailed(msg) => {
                ReconciliationError::Unimplemented(format!("Model upload failed: {}", msg))
            }
            FlexServDeploymentError::PodCreationFailed(msg) => {
                ReconciliationError::Unimplemented(format!("Pod creation failed: {}", msg))
            }
            FlexServDeploymentError::JobCreationFailed(msg) => {
                ReconciliationError::Unimplemented(format!("Job creation failed: {}", msg))
            }
            FlexServDeploymentError::UnknownError(msg) => {
                ReconciliationError::Unimplemented(format!("Unknown error: {}", msg))
            }
        }
    }

    /// Handle Start action
    async fn handle_start(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Starting deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let mut deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        // Create the deployment (this also starts it)
        // Run in a blocking task since FlexServ methods are synchronous
        let result = tokio::task::spawn_blocking(move || {
            deployment.create()
        })
        .await
        .map_err(|e| ReconciliationError::Unimplemented(format!("Task join error: {}", e)))?
        .map_err(Self::map_deployment_error)?;

        // Store pod_id and volume_id in metadata for future operations
        if let DeploymentResult::PodResult { pod_id, volume_id, pod_url, .. } = &result {
            info!("Deployment created successfully: pod_id={}, volume_id={}, pod_url={:?}", pod_id, volume_id, pod_url);
            
            // Update cached deployment info
            let mut deployments = self.deployments.lock().await;
            if let Some(existing_info) = deployments.get_mut(&input.deployment.id) {
                existing_info.pod_id = pod_id.clone();
                existing_info.volume_id = volume_id.clone();
            }
        }

        Ok(ReconciliationOutcome::Started(StartedOutcomePayload {
            message: Some(format!("Deployment started successfully")),
        }))
    }

    /// Handle Stop action
    async fn handle_stop(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Stopping deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        tokio::task::spawn_blocking(move || {
            deployment.stop()
        })
        .await
        .map_err(|e| ReconciliationError::Unimplemented(format!("Task join error: {}", e)))?
        .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Stopped(StoppedOutcomePayload {
            message: Some(format!("Deployment stopped successfully")),
        }))
    }

    /// Handle Undeploy action
    async fn handle_undeploy(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Undeploying deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        tokio::task::spawn_blocking(move || {
            deployment.terminate()
        })
        .await
        .map_err(|e| ReconciliationError::Unimplemented(format!("Task join error: {}", e)))?
        .map_err(Self::map_deployment_error)?;

        // Remove from cache
        let mut deployments = self.deployments.lock().await;
        deployments.remove(&input.deployment.id);

        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcomePayload {
            message: Some(format!("Deployment terminated successfully")),
        }))
    }

    /// Handle Observe action
    async fn handle_observe(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Observing deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        let result = tokio::task::spawn_blocking(move || {
            deployment.monitor()
        })
        .await
        .map_err(|e| ReconciliationError::Unimplemented(format!("Task join error: {}", e)))?
        .map_err(Self::map_deployment_error)?;

        // Determine state from monitoring result
        // TODO: Parse pod status from result to determine actual state
        let observed_state = State::Running; // Default assumption

        Ok(ReconciliationOutcome::Observed(ObeservedOutcomePayload {
            message: Some(format!("Deployment observed: {:?}", result)),
            state: observed_state,
        }))
    }
}

impl Clone for Backend {
    fn clone(&self) -> Self {
        match self {
            Backend::Transformers { command } => Backend::Transformers { command: command.clone() },
            Backend::VLlm { command } => Backend::VLlm { command: command.clone() },
            Backend::SGLang { command } => Backend::SGLang { command: command.clone() },
            Backend::TrtLlm { command } => Backend::TrtLlm { command: command.clone() },
        }
    }
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconciliationClient for TapisPodsModelDeploymentReconciliationClient {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        match input.action {
            ReconciliationAction::Start => {
                self.handle_start(&input).await
            }
            ReconciliationAction::Stop => {
                self.handle_stop(&input).await
            }
            ReconciliationAction::Undeploy => {
                self.handle_undeploy(&input).await
            }
            ReconciliationAction::Observe => {
                self.handle_observe(&input).await
            }
        }
    }
}