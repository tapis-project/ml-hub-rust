use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::workflows::reconciliation::{
    FailedOutcome, ObeservedOutcome, ReconciliationAction, ReconciliationOutcome, StartedOutcome,
    StoppedOutcome, UndeployedOutcome,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, State,
};
use crate::domain::entities::model::external_model::{ExternalModel, ModelLocator, ModelProvider};
use crate::domain::entities::site::SiteContext;
use flexserv_deployer::deployment::{DeploymentError as FlexServDeploymentError, DeploymentResult};
use flexserv_deployer::{
    normalize_tenant_url, Backend, FlexServDeployment, FlexServInstance, FlexServPodDeployment,
    PodDeploymentOptions,
};
use log::info;
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
enum ReconciliationError {
    #[error("{0}")]
    Fatal(String),
}

pub struct TapisPodsModelDeploymentReconciliationClient {
    site_context: SiteContext,
}

impl TapisPodsModelDeploymentReconciliationClient {
    pub fn new(site_context: &SiteContext) -> Self {
        Self {
            site_context: site_context.clone(),
        }
    }

    /// Extract Tapis credentials from deployment metadata.
    ///
    /// Credentials must be provided by the UI/API when creating or updating the deployment
    /// and stored in `deployment.metadata`. Required keys: `tapis_tenant_url`, `tapis_user`, `tapis_token`.
    /// No env var fallback—parameters flow from the request through to the reconciler.
    fn extract_tapis_credentials(
        deployment: &ModelDeployment,
    ) -> Result<(String, String, String), ReconciliationError> {
        let metadata = deployment.metadata.as_ref().ok_or_else(|| {
            ReconciliationError::Fatal(
                "Deployment metadata is required for TapisPods. Include tapis_tenant_url, tapis_user, tapis_token (e.g. from the deploy request).".into()
            )
        })?;

        let tenant_url = metadata
            .get("tapis_tenant_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Fatal(
                    "deployment.metadata must include tapis_tenant_url".into(),
                )
            })?
            .to_string();

        let tapis_user = metadata
            .get("tapis_user")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Fatal("deployment.metadata must include tapis_user".into())
            })?
            .to_string();

        let tapis_token = metadata
            .get("tapis_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Fatal("deployment.metadata must include tapis_token".into())
            })?
            .to_string();

        Ok((tenant_url, tapis_user, tapis_token))
    }

    /// Extract pod_id and volume_id from deployment metadata. volume_id is optional (some pods have no volume).
    fn extract_pod_info(deployment: &ModelDeployment) -> Option<(String, String)> {
        deployment.metadata.as_ref().and_then(|m| {
            let pod_id = m.get("pod_id")?.as_str()?.to_string();

            let volume_id = m
                .get("volume_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Some((pod_id, volume_id))
        })
    }

    /// Default FlexServ backend.
    fn default_backend() -> Backend {
        // TODO: Support additional FlexServ backends (VLlm, SGLang, TrtLlm) once
        //       their deployment options and tuning parameters are exposed through ML Hub.
        //
        // For now we rely on the FlexServ-Deployer defaults for the Transformers backend
        // (it wires the correct Python entrypoint inside the pod), so we leave the
        // command list empty here.
        Backend::Transformers { command: vec![] }
    }

    /// Create a FlexServPodDeployment from deployment entity
    fn create_deployment_from_entity(
        deployment: &ModelDeployment,
        external_model: &ExternalModel,
    ) -> Result<FlexServPodDeployment, ReconciliationError> {
        let (tenant_url, tapis_user, tapis_token) = Self::extract_tapis_credentials(deployment)?;

        let locator = match (external_model.provider(), external_model.locator()) {
            (ModelProvider::HuggingFace, ModelLocator::HuggingFace(locator)) => locator,
            (provider, _) => {
                return Err(ReconciliationError::Fatal(format!(
                    "Unsupported model provider for Tapis Pods: {provider}"
                )))
            }
        };

        // Check if pod_id and volume_id exist in metadata (for existing deployments)
        let (pod_id, volume_id) = Self::extract_pod_info(deployment).unwrap_or_default();

        // TODO: Wire through model_revision, hf_token, and default_embedding_model from ML Hub
        //       once the API plumbs these fields. For now we rely on FlexServ-Deployer defaults:
        //       - model_revision: None  -> Hugging Face repo default revision is used.
        //       - hf_token: None       -> pod falls back to HF_TOKEN env inside the container.
        //       - default_embedding_model: None -> backend uses its own internal default.
        let server = FlexServInstance::builder()
            .tenant_url(normalize_tenant_url(&tenant_url))
            .tapis_user(tapis_user.clone())
            .model(locator.id().to_owned())
            .model_revision(Some(locator.sha().to_owned()))
            .backend(Self::default_backend())
            .build()
            .map_err(|e| {
                ReconciliationError::Fatal(format!("Failed to create FlexServInstance: {}", e))
            })?;

        if !pod_id.is_empty() {
            Ok(FlexServPodDeployment::from_existing(
                server,
                tapis_token,
                pod_id,
                volume_id,
            ))
        } else {
            let mut options = PodDeploymentOptions::default();
            // TODO: Expose PodDeploymentOptions on the ML Hub side instead of relying purely on defaults.
            //       Current defaults in flexserv-deployer:
            //       - volume_size_mb: 10 GiB
            //       - image: "tapis/flexserv:1.0"
            //       - cpu_request / cpu_limit: 1000 / 2000 millicores
            //       - mem_request_mb / mem_limit_mb: 4096 / 8192 MB
            //       - gpus: 0
            //       - flexserv_secret: from FLEXSERV_SECRET env (may be empty)
            //
            // Use the deployment UUID as deployment_id so FlexServ can derive pod_id and volume_id from it.
            options.deployment_id = Some(deployment.id.to_string());

            Ok(FlexServPodDeployment::with_options(
                server,
                tapis_token,
                options,
            ))
        }
    }

    /// Build metadata delta from FlexServ DeploymentResult::PodResult (pod_id, volume_id, pod_url, tapis_user, tapis_tenant, model_id, etc.).
    fn result_to_metadata_delta(result: &DeploymentResult) -> ModelDeploymentMetadataDelta {
        match result {
            DeploymentResult::PodResult {
                pod_id,
                volume_id,
                pod_url,
                status,
                pod_info,
                volume_info,
                tapis_user,
                tapis_tenant,
                model_id,
                ..
            } => {
                // Validate critical fields - pod_id must not be empty
                if pod_id.is_empty() {
                    log::warn!(
                        "Pod creation/operation returned empty pod_id, not updating metadata"
                    );
                    return ModelDeploymentMetadataDelta::NoChange;
                }

                let mut map = HashMap::new();

                map.insert("pod_id".to_string(), json!(pod_id));
                map.insert("volume_id".to_string(), json!(volume_id));
                map.insert("tapis_user".to_string(), json!(tapis_user));
                map.insert("tapis_tenant".to_string(), json!(tapis_tenant));
                map.insert("model_id".to_string(), json!(model_id));

                if let Some(url) = pod_url {
                    if !url.is_empty() {
                        map.insert("pod_url".to_string(), json!(url));
                    }
                }
                if let Some(status) = status {
                    if !status.is_empty() {
                        map.insert("pod_status".to_string(), json!(status));
                    }
                }
                map.insert("pod_info".to_string(), json!(pod_info));
                map.insert("volume_info".to_string(), json!(volume_info));

                ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))
            }
            _ => ModelDeploymentMetadataDelta::NoChange,
        }
    }

    /// Map canonical TAPIS pod status strings to our [State].
    fn state_from_status(status: Option<&str>) -> State {
        match status
            .unwrap_or_default()
            .trim()
            .to_ascii_uppercase()
            .as_str()
        {
            "AVAILABLE" | "RUNNING" => State::Running,
            "STOPPED" => State::Stopped,
            "FAILED" => State::Failed,
            "PENDING" | "CREATING" => State::Unknown,
            _ => State::Unknown,
        }
    }

    /// Map FlexServ DeploymentError to ReconciliationError
    fn map_deployment_error(err: FlexServDeploymentError) -> ReconciliationError {
        match err {
            FlexServDeploymentError::TapisAuthFailed(msg) => {
                ReconciliationError::Fatal(format!("TAPIS authentication failed: {}", msg))
            }
            FlexServDeploymentError::TapisAPIUnreachable(msg) => {
                ReconciliationError::Fatal(format!("TAPIS API unreachable: {}", msg))
            }
            FlexServDeploymentError::TapisBadRequest(msg) => {
                ReconciliationError::Fatal(format!("TAPIS bad request: {}", msg))
            }
            FlexServDeploymentError::TapisTimeout(msg) => {
                ReconciliationError::Fatal(format!("TAPIS timeout: {}", msg))
            }
            FlexServDeploymentError::TapisInternalServerError(msg) => {
                ReconciliationError::Fatal(format!("TAPIS internal server error: {}", msg))
            }
            FlexServDeploymentError::ModelUploadingFailed(msg) => {
                ReconciliationError::Fatal(format!("Model upload failed: {}", msg))
            }
            FlexServDeploymentError::PodCreationFailed(msg) => {
                ReconciliationError::Fatal(format!("Pod creation failed: {}", msg))
            }
            FlexServDeploymentError::JobCreationFailed(msg) => {
                ReconciliationError::Fatal(format!("Job creation failed: {}", msg))
            }
            FlexServDeploymentError::UnknownError(msg) => {
                ReconciliationError::Fatal(format!("Unknown error: {}", msg))
            }
            FlexServDeploymentError::InvalidConfiguration(msg) => ReconciliationError::Fatal(msg),
        }
    }

    /// Handle Start action
    async fn handle_start(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Starting deployment {}", input.deployment.id);

        // Check if pod already exists
        let has_existing_pod = Self::extract_pod_info(&input.deployment)
            .map(|(pod_id, _)| !pod_id.is_empty())
            .unwrap_or(false);

        let mut deployment =
            Self::create_deployment_from_entity(&input.deployment, &input.external_model)?;

        // If pod already exists (has pod_id), use start(). Otherwise, create new pod.
        let result = if has_existing_pod {
            deployment.start().await
        } else {
            deployment.create().await
        }
        .map_err(Self::map_deployment_error)?;

        // Log success
        if !has_existing_pod {
            if let DeploymentResult::PodResult {
                pod_id,
                volume_id,
                pod_url,
                ..
            } = &result
            {
                info!(
                    "Deployment created successfully: pod_id={}, volume_id={}, pod_url={:?}",
                    pod_id, volume_id, pod_url
                );
            }
        } else {
            info!("Deployment started successfully");
        }

        Ok(ReconciliationOutcome::Started(StartedOutcome {
            message: Some(format!("Deployment started successfully")),
            state: State::Unknown,
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }

    /// Handle Stop action
    async fn handle_stop(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Stopping deployment {}", input.deployment.id);

        let deployment =
            Self::create_deployment_from_entity(&input.deployment, &input.external_model)?;

        let result = deployment
            .stop()
            .await
            .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Stopped(StoppedOutcome {
            message: Some(format!("Deployment stopped successfully")),
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }

    /// Handle Undeploy action
    async fn handle_undeploy(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Undeploying deployment {}", input.deployment.id);

        let deployment =
            Self::create_deployment_from_entity(&input.deployment, &input.external_model)?;

        deployment
            .terminate()
            .await
            .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcome {
            message: Some(format!("Deployment terminated successfully")),
            metadata: Some(ModelDeploymentMetadataDelta::Delete),
        }))
    }

    /// Handle Observe action
    async fn handle_observe(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Observing deployment {}", input.deployment.id);

        let deployment =
            Self::create_deployment_from_entity(&input.deployment, &input.external_model)?;

        let result = deployment
            .monitor()
            .await
            .map_err(Self::map_deployment_error)?;

        let observed_state = match &result {
            DeploymentResult::PodResult { status, .. } => {
                Self::state_from_status(status.as_deref())
            }
            _ => State::Unknown,
        };

        info!(
            "Observed state for deployment {}: {:?}",
            input.deployment.id, observed_state
        );

        Ok(ReconciliationOutcome::Observed(ObeservedOutcome {
            message: Some(format!("Deployment observed: {:?}", result)),
            state: observed_state,
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconciliationClient for TapisPodsModelDeploymentReconciliationClient {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> ReconciliationOutcome {
        let outcome = match input.action {
            ReconciliationAction::Start { .. } => self.handle_start(&input).await,
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
            ReconciliationAction::Observe => self.handle_observe(&input).await,
        };

        outcome.unwrap_or_else(|error| {
            ReconciliationOutcome::Failed(FailedOutcome {
                message: Some(error.to_string()),
                metadata: None,
                replicas: None,
                interface: None,
            })
        })
    }

    fn get_site_context(&self) -> &SiteContext {
        &self.site_context
    }
}

#[cfg(test)]
#[path = "tapis_pods.test.rs"]
mod tapis_pods_test;
