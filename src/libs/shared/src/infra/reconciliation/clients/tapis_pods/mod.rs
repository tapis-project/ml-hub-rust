use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::workflows::reconciliation::{
    ReconciliationError, ReconciliationOutcome, ReconciliationAction,
    StartedOutcomePayload, StoppedOutcomePayload, UndeployedOutcomePayload, ObeservedOutcomePayload,
    PodResultInfo,
};
use crate::domain::entities::deployment::State;
use flexserv_deployer::{FlexServDeployment, FlexServPodDeployment, PodDeploymentOptions, FlexServInstance, Backend, normalize_tenant_url};
use flexserv_deployer::deployment::{DeploymentError as FlexServDeploymentError, DeploymentResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;

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

    /// Extract Tapis credentials from deployment metadata.
    ///
    /// Credentials must be provided by the UI/API when creating or updating the deployment
    /// and stored in `deployment.metadata`. Required keys: `tapis_tenant_url`, `tapis_user`, `tapis_token`.
    /// No env var fallback—parameters flow from the request through to the reconciler.
    fn extract_tapis_credentials(deployment: &crate::domain::entities::deployment::ModelDeployment) -> Result<(String, String, String), ReconciliationError> {
        let metadata = deployment.metadata.as_ref().ok_or_else(|| {
            ReconciliationError::Unimplemented(
                "Deployment metadata is required for TapisPods. Include tapis_tenant_url, tapis_user, tapis_token (e.g. from the deploy request).".into()
            )
        })?;

        let tenant_url = metadata.get("tapis_tenant_url").and_then(|v| v.as_str()).ok_or_else(|| {
            ReconciliationError::Unimplemented("deployment.metadata must include tapis_tenant_url".into())
        })?.to_string();
        let tapis_user = metadata.get("tapis_user").and_then(|v| v.as_str()).ok_or_else(|| {
            ReconciliationError::Unimplemented("deployment.metadata must include tapis_user".into())
        })?.to_string();
        let tapis_token = metadata.get("tapis_token").and_then(|v| v.as_str()).ok_or_else(|| {
            ReconciliationError::Unimplemented("deployment.metadata must include tapis_token".into())
        })?.to_string();

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

    /// Build PodResultInfo from FlexServ DeploymentResult::PodResult for outcome payloads.
    fn pod_result_info(result: &DeploymentResult) -> Option<PodResultInfo> {
        match result {
            DeploymentResult::PodResult {
                pod_id,
                volume_id,
                pod_url,
                pod_info,
                volume_info,
                ..
            } => Some(PodResultInfo {
                pod_id: Some(pod_id.clone()),
                volume_id: Some(volume_id.clone()),
                pod_url: pod_url.clone(),
                pod_info: Some(pod_info.clone()),
                volume_info: Some(volume_info.clone()),
            }),
            _ => None,
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
            result: Self::pod_result_info(&result),
        }))
    }

    /// Handle Stop action
    async fn handle_stop(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Stopping deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        let result = tokio::task::spawn_blocking(move || {
            deployment.stop()
        })
        .await
        .map_err(|e| ReconciliationError::Unimplemented(format!("Task join error: {}", e)))?
        .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Stopped(StoppedOutcomePayload {
            message: Some(format!("Deployment stopped successfully")),
            result: Self::pod_result_info(&result),
        }))
    }

    /// Handle Undeploy action
    async fn handle_undeploy(&self, input: &ReconcileModelDeploymentInput) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Undeploying deployment {}", input.deployment.id);
        let info = self.get_or_create_deployment_info(input).await?;
        let deployment = self.create_deployment_from_info(&info, input.deployment.id)?;

        let result = tokio::task::spawn_blocking(move || {
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
            result: Self::pod_result_info(&result),
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
            result: Self::pod_result_info(&result),
        }))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::workflows::reconciliation::PodResultInfo;
    use crate::domain::entities::deployment::{
        ModelDeployment, ModelReference, State, DesiredState, RehydrateModelDeploymentProps,
        DeploymentStrategyReference,
    };
    use crate::domain::entities::visibility::Visibility;
    use crate::domain::entities::model_metadata::{ModelMetadata, fixtures::full_model_metadata};
    use crate::domain::entities::timestamp::TimeStamp;
    use platforms::Platform;
    use uuid::Uuid;
    use serde_json::json;

    fn ts() -> TimeStamp {
        TimeStamp::now()
    }

    fn deployment_with_metadata(metadata: HashMap<String, serde_json::Value>) -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisPods,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some(DeploymentStrategyReference {
                client: "tapis-pods".into(),
                name: "default".into(),
            }),
            visibility: Visibility::Private,
            deployment_interface: None,
            parallelism: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(metadata),
        })
    }

    fn deployment_without_metadata() -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisPods,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: None,
            visibility: Visibility::Private,
            deployment_interface: None,
            parallelism: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: None,
        })
    }

    fn minimal_model_metadata() -> ModelMetadata {
        let mut m = full_model_metadata();
        m.name = Some("gpt2".into());
        m.author = Some("openai-community".into());
        m
    }

    // ---- Unit tests: credential extraction ----

    #[test]
    fn extract_tapis_credentials_ok() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt-token"));
        let deployment = deployment_with_metadata(meta);
        let (url, user, token) =
            TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
                .unwrap();
        assert_eq!(url, "https://tacc.tapis.io");
        assert_eq!(user, "user1");
        assert_eq!(token, "jwt-token");
    }

    #[test]
    fn extract_tapis_credentials_err_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("Deployment metadata is required"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_tenant_url_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_tenant_url"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_user_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_user"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_token_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_token"));
    }

    // ---- Unit tests: pod info extraction ----

    #[test]
    fn extract_pod_info_some() {
        let mut meta = HashMap::new();
        meta.insert("pod_id".into(), json!("pabc123"));
        meta.insert("volume_id".into(), json!("vabc123"));
        let deployment = deployment_with_metadata(meta);
        let (pod_id, volume_id) =
            TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).unwrap();
        assert_eq!(pod_id, "pabc123");
        assert_eq!(volume_id, "vabc123");
    }

    #[test]
    fn extract_pod_info_none_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        assert!(TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).is_none());
    }

    #[test]
    fn extract_pod_info_none_when_keys_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        let deployment = deployment_with_metadata(meta);
        assert!(TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).is_none());
    }

    // ---- Unit tests: error mapping ----

    #[test]
    fn map_deployment_error_tapis_auth() {
        let e = FlexServDeploymentError::TapisAuthFailed("bad token".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("TAPIS authentication failed"));
        assert!(r.to_string().contains("bad token"));
    }

    #[test]
    fn map_deployment_error_pod_creation() {
        let e = FlexServDeploymentError::PodCreationFailed("quota exceeded".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Pod creation failed"));
    }

    #[test]
    fn map_deployment_error_unknown() {
        let e = FlexServDeploymentError::UnknownError("something broke".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Unknown error"));
    }

    // ---- Async unit tests: reconcile returns error when metadata missing ----

    #[tokio::test]
    async fn reconcile_start_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("metadata"));
    }

    #[tokio::test]
    async fn reconcile_stop_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reconcile_undeploy_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reconcile_observe_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    // ---- Integration tests: real Tapis (one test per action) ----
    // Run with: cargo test -p shared integration_reconcile_<create|start|stop|terminate|monitor> -- --ignored
    // Create: needs only TAPIS_* env. Start/Stop/Terminate/Monitor: need deployment.metadata with pod_id and volume_id.

    fn has_tapis_credentials() -> bool {
        std::env::var("TAPIS_TENANT_URL").is_ok()
            && std::env::var("TAPIS_USER").is_ok()
            && std::env::var("TAPIS_TOKEN").is_ok()
    }

    fn deployment_with_tapis_meta(
        deployment_id: Uuid,
        tenant_url: &str,
        tapis_user: &str,
        tapis_token: &str,
        pod_id: Option<&str>,
        volume_id: Option<&str>,
    ) -> ModelDeployment {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!(tenant_url));
        meta.insert("tapis_user".into(), json!(tapis_user));
        meta.insert("tapis_token".into(), json!(tapis_token));
        if let Some(p) = pod_id {
            meta.insert("pod_id".into(), json!(p));
        }
        if let Some(v) = volume_id {
            meta.insert("volume_id".into(), json!(v));
        }
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: deployment_id,
            platform: Platform::TapisPods,
            owner: tapis_user.to_string(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some(DeploymentStrategyReference {
                client: "tapis-pods".into(),
                name: "default".into(),
            }),
            visibility: Visibility::Private,
            deployment_interface: None,
            parallelism: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(meta),
        })
    }

    fn assert_and_print_result(label: &str, result: &Option<PodResultInfo>) {
        let r = result.as_ref().expect("outcome should have result from library");
        eprintln!("{} response: pod_id={:?} volume_id={:?} pod_url={:?}", label, r.pod_id, r.volume_id, r.pod_url);
        assert!(r.pod_id.is_some(), "result should have pod_id");
        assert!(r.volume_id.is_some(), "result should have volume_id");
    }

    /// Create pod only. No pod_id/volume_id needed. Response includes pod_id, volume_id, pod_url.
    #[tokio::test]
    #[ignore = "requires TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN and real Tapis Pods API"]
    async fn integration_reconcile_create_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            None,
            None,
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile create");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_and_print_result("create", &p.result);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Start an existing pod. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_start_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for start test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for start test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile start");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_and_print_result("start", &p.result);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Stop a running pod. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_stop_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for stop test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for stop test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile stop");
        match &outcome {
            ReconciliationOutcome::Stopped(p) => {
                assert!(p.message.is_some());
                assert_and_print_result("stop", &p.result);
            }
            other => panic!("expected Stopped, got {:?}", other),
        }
    }

    /// Terminate (delete) pod and volume. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_terminate_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for terminate test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for terminate test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile terminate");
        match &outcome {
            ReconciliationOutcome::Undeployed(p) => {
                assert!(p.message.is_some());
                assert_and_print_result("terminate", &p.result);
            }
            other => panic!("expected Undeployed, got {:?}", other),
        }
    }

    /// Monitor (observe) an existing pod. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_monitor_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for monitor test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for monitor test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile monitor");
        match &outcome {
            ReconciliationOutcome::Observed(p) => {
                assert!(p.message.is_some());
                assert_and_print_result("monitor", &p.result);
            }
            other => panic!("expected Observed, got {:?}", other),
        }
    }
}