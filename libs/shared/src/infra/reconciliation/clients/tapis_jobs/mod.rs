use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::workflows::reconciliation::{
    ReconciliationAction, ReconciliationError, ReconciliationOutcome, StartedOutcomePayload,
    StoppedOutcomePayload, UndeployedOutcomePayload, ObeservedOutcomePayload,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, State,
};
use flexserv_deployer::{
    Backend, FlexServDeployment, FlexServHPCDeployment, FlexServInstance, HpcDeploymentOptions,
    normalize_tenant_url,
};
use flexserv_deployer::deployment::{DeploymentError as FlexServDeploymentError, DeploymentResult};
use log::info;
use serde_json::json;
use std::collections::HashMap;

pub struct TapisJobsModelDeploymentReconciliationClient {}

impl TapisJobsModelDeploymentReconciliationClient {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_tapis_credentials(
        deployment: &ModelDeployment,
    ) -> Result<(String, String, String), ReconciliationError> {
        let metadata = deployment.metadata.as_ref().ok_or_else(|| {
            ReconciliationError::Unimplemented(
                "Deployment metadata is required for TapisJobs. Include tapis_tenant_url, tapis_user, tapis_token (e.g. from the deploy request)."
                    .into(),
            )
        })?;

        let tenant_url = metadata
            .get("tapis_tenant_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Unimplemented(
                    "deployment.metadata must include tapis_tenant_url".into(),
                )
            })?
            .to_string();
        let tapis_user = metadata
            .get("tapis_user")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Unimplemented("deployment.metadata must include tapis_user".into())
            })?
            .to_string();
        let tapis_token = metadata
            .get("tapis_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ReconciliationError::Unimplemented(
                    "deployment.metadata must include tapis_token".into(),
                )
            })?
            .to_string();

        Ok((tenant_url, tapis_user, tapis_token))
    }

    /// `job_uuid` from metadata when the deployment already submitted a Tapis job.
    fn extract_job_uuid(deployment: &ModelDeployment) -> Option<String> {
        deployment.metadata.as_ref().and_then(|m| {
            m.get("job_uuid")?
                .as_str()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
        })
    }

    /// HPC submission options from metadata (required for first-time `create` / Start with no `job_uuid`).
    ///
    /// Keys: `tapis_hpc_app_id`, `tapis_hpc_app_version`, `tapis_hpc_exec_system_id`,
    /// `tapis_hpc_exec_system_logical_queue`, `tapis_hpc_max_minutes` (number or string),
    /// `tapis_hpc_allocation`.
    fn extract_hpc_options(
        deployment: &ModelDeployment,
    ) -> Result<HpcDeploymentOptions, ReconciliationError> {
        let metadata = deployment.metadata.as_ref().ok_or_else(|| {
            ReconciliationError::Unimplemented(
                "Deployment metadata is required to build HPC job options.".into(),
            )
        })?;

        let req_str = |key: &str| -> Result<String, ReconciliationError> {
            metadata
                .get(key)
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .ok_or_else(|| {
                    ReconciliationError::Unimplemented(format!(
                        "deployment.metadata must include non-empty {}",
                        key
                    ))
                })
        };

        let max_minutes: i32 = metadata
            .get("tapis_hpc_max_minutes")
            .and_then(|v| {
                v.as_i64()
                    .map(|n| n as i32)
                    .or_else(|| v.as_u64().map(|n| n as i32))
                    .or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))
            })
            .ok_or_else(|| {
                ReconciliationError::Unimplemented(
                    "deployment.metadata must include tapis_hpc_max_minutes (integer)".into(),
                )
            })?;

        Ok(HpcDeploymentOptions::new(
            req_str("tapis_hpc_app_id")?,
            req_str("tapis_hpc_app_version")?,
            req_str("tapis_hpc_exec_system_id")?,
            req_str("tapis_hpc_exec_system_logical_queue")?,
            max_minutes,
            req_str("tapis_hpc_allocation")?,
        ))
    }

    fn default_backend() -> Backend {
        Backend::Transformers { command: vec![] }
    }

    fn create_deployment_from_entity(
        deployment: &ModelDeployment,
    ) -> Result<FlexServHPCDeployment, ReconciliationError> {
        let (tenant_url, tapis_user, tapis_token) = Self::extract_tapis_credentials(deployment)?;
        let model_id = format!("{}/{}", deployment.model.author, deployment.model.name);

        let server = FlexServInstance::builder()
            .tenant_url(normalize_tenant_url(&tenant_url))
            .tapis_user(tapis_user.clone())
            .model(model_id.clone())
            .backend(Self::default_backend())
            .build()
            .map_err(|e| {
                ReconciliationError::Unimplemented(format!("Failed to create FlexServInstance: {}", e))
            })?;

        if let Some(job_uuid) = Self::extract_job_uuid(deployment) {
            Ok(FlexServHPCDeployment::from_existing(
                server,
                tapis_token,
                job_uuid,
            ))
        } else {
            let options = Self::extract_hpc_options(deployment)?;
            Ok(FlexServHPCDeployment::new(server, tapis_token, options))
        }
    }

    fn result_to_metadata_delta(result: &DeploymentResult) -> ModelDeploymentMetadataDelta {
        match result {
            DeploymentResult::HPCResult {
                job_uuid,
                status,
                job_info,
                tapis_user,
                tapis_tenant,
                model_id,
                ..
            } => {
                if job_uuid.is_empty() {
                    log::warn!("HPC operation returned empty job_uuid, not updating job metadata keys");
                    return ModelDeploymentMetadataDelta::NoChange;
                }
                let mut map = HashMap::new();
                map.insert("job_uuid".to_string(), json!(job_uuid));
                map.insert("tapis_user".to_string(), json!(tapis_user));
                map.insert("tapis_tenant".to_string(), json!(tapis_tenant));
                map.insert("model_id".to_string(), json!(model_id));
                if let Some(status) = status {
                    if !status.is_empty() {
                        map.insert("job_status".to_string(), json!(status));
                    }
                }
                map.insert("job_info".to_string(), json!(job_info));
                ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))
            }
            _ => ModelDeploymentMetadataDelta::NoChange,
        }
    }

    /// Infer [State] from raw TAPIS job status.
    fn state_from_job_status(status: Option<&str>) -> State {
        match status.unwrap_or_default().trim().to_ascii_uppercase().as_str() {
            "RUNNING" => State::Running,
            "QUEUED" | "PENDING" | "SUBMITTING" | "STAGING" => State::Unknown,
            "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" => State::Stopped,
            "FAILED" => State::Failed,
            _ => State::Unknown,
        }
    }

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

    async fn handle_start(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Starting Tapis Jobs deployment {}", input.deployment.id);

        let has_existing_job = Self::extract_job_uuid(&input.deployment).is_some();
        let mut deployment = Self::create_deployment_from_entity(&input.deployment)?;

        let result = if has_existing_job {
            deployment.start().await
        } else {
            deployment.create().await
        }
        .map_err(Self::map_deployment_error)?;

        if !has_existing_job {
            if let DeploymentResult::HPCResult { job_uuid, .. } = &result {
                info!("HPC job submitted: job_uuid={}", job_uuid);
            }
        } else {
            info!("HPC job resubmitted successfully");
        }

        Ok(ReconciliationOutcome::Started(StartedOutcomePayload {
            message: Some("Deployment started successfully".to_string()),
            state: State::Unknown,
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }

    async fn handle_stop(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Stopping Tapis Jobs deployment {}", input.deployment.id);
        let deployment = Self::create_deployment_from_entity(&input.deployment)?;

        let result = deployment
            .stop()
            .await
            .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Stopped(StoppedOutcomePayload {
            message: Some("Deployment stopped successfully".to_string()),
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }

    async fn handle_undeploy(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Undeploying Tapis Jobs deployment {}", input.deployment.id);
        let deployment = Self::create_deployment_from_entity(&input.deployment)?;

        deployment
            .terminate()
            .await
            .map_err(Self::map_deployment_error)?;

        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcomePayload {
            message: Some("Deployment canceled successfully".to_string()),
            metadata: Some(ModelDeploymentMetadataDelta::Delete),
        }))
    }

    async fn handle_observe(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        info!("Observing Tapis Jobs deployment {}", input.deployment.id);
        let deployment = Self::create_deployment_from_entity(&input.deployment)?;

        let result = deployment
            .monitor()
            .await
            .map_err(Self::map_deployment_error)?;

        let observed_state = Self::state_from_job_status(match &result {
            DeploymentResult::HPCResult { status, .. } => status.as_deref(),
            _ => None,
        });

        info!(
            "Observed state for deployment {}: {:?}",
            input.deployment.id, observed_state
        );

        Ok(ReconciliationOutcome::Observed(ObeservedOutcomePayload {
            message: Some(format!("Deployment observed: {:?}", result)),
            state: observed_state,
            metadata: Some(Self::result_to_metadata_delta(&result)),
            replicas: None,
            interface: None,
        }))
    }
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconciliationClient for TapisJobsModelDeploymentReconciliationClient {
    async fn reconcile(
        &self,
        input: ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        match input.action {
            ReconciliationAction::Start { .. } => self.handle_start(&input).await,
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
            ReconciliationAction::Observe => self.handle_observe(&input).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
    use crate::application::workflows::reconciliation::{
        ReconciliationAction, ReconciliationOutcome,
    };
    use crate::domain::entities::deployment::{
        DesiredState, ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta,
        ModelReference, RehydrateModelDeploymentProps, State,
    };
    use crate::domain::entities::model_metadata::{fixtures::full_model_metadata, ModelMetadata};
    use crate::domain::entities::timestamp::TimeStamp;
    use crate::domain::entities::visibility::Visibility;
    use platforms::Platform;
    use serde_json::json;
    use uuid::Uuid;

    fn ts() -> TimeStamp {
        TimeStamp::now()
    }

    fn deployment_with_metadata(metadata: HashMap<String, serde_json::Value>) -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisJobs,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "Qwen3.5-0.8B".into(),
                author: "Qwen".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-jobs:default".into()),
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(ModelDeploymentMetadata(metadata)),
        })
    }

    fn deployment_without_metadata() -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisJobs,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "Qwen3.5-0.8B".into(),
                author: "Qwen".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: None,
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
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
        m.name = Some("Qwen3.5-0.8B".into());
        m.author = Some("Qwen".into());
        m
    }

    fn base_tapis_meta() -> HashMap<String, serde_json::Value> {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt-token"));
        meta
    }

    // ---- Unit tests: credential extraction ----

    #[test]
    fn extract_tapis_credentials_ok() {
        let deployment = deployment_with_metadata(base_tapis_meta());
        let (url, user, token) =
            TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
                .unwrap();
        assert_eq!(url, "https://tacc.tapis.io");
        assert_eq!(user, "user1");
        assert_eq!(token, "jwt-token");
    }

    #[test]
    fn extract_tapis_credentials_err_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("Deployment metadata is required"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_tenant_url_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_tenant_url"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_user_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_user"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_token_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_token"));
    }

    // ---- Unit tests: job UUID and HPC metadata ----

    #[test]
    fn extract_job_uuid_some() {
        let mut meta = base_tapis_meta();
        meta.insert("job_uuid".into(), json!("abc-123"));
        let d = deployment_with_metadata(meta);
        assert_eq!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&d).as_deref(),
            Some("abc-123")
        );
    }

    #[test]
    fn extract_hpc_options_ok() {
        let mut meta = base_tapis_meta();
        meta.insert("tapis_hpc_app_id".into(), json!("FlexServ-1.4.0"));
        meta.insert("tapis_hpc_app_version".into(), json!("1.4.0"));
        meta.insert("tapis_hpc_exec_system_id".into(), json!("vista-tapis"));
        meta.insert("tapis_hpc_exec_system_logical_queue".into(), json!("gh"));
        meta.insert("tapis_hpc_max_minutes".into(), json!(60));
        meta.insert("tapis_hpc_allocation".into(), json!("TACC-ACI-CIC"));
        let d = deployment_with_metadata(meta);
        let o = TapisJobsModelDeploymentReconciliationClient::extract_hpc_options(&d).unwrap();
        assert_eq!(o.app_id, "FlexServ-1.4.0");
        assert_eq!(o.exec_system_id, "vista-tapis");
        assert_eq!(o.max_minutes, 60);
    }

    #[test]
    fn extract_job_uuid_none_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    #[test]
    fn extract_job_uuid_none_when_key_missing() {
        let deployment = deployment_with_metadata(base_tapis_meta());
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    #[test]
    fn extract_job_uuid_none_when_empty_string() {
        let mut meta = base_tapis_meta();
        meta.insert("job_uuid".into(), json!("  "));
        let deployment = deployment_with_metadata(meta);
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    // ---- Unit tests: error mapping ----

    #[test]
    fn map_deployment_error_tapis_auth() {
        let e = FlexServDeploymentError::TapisAuthFailed("bad token".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("TAPIS authentication failed"));
        assert!(r.to_string().contains("bad token"));
    }

    #[test]
    fn map_deployment_error_job_creation() {
        let e = FlexServDeploymentError::JobCreationFailed("queue closed".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Job creation failed"));
    }

    #[test]
    fn map_deployment_error_unknown() {
        let e = FlexServDeploymentError::UnknownError("something broke".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Unknown error"));
    }

    // ---- Unit tests: metadata delta ----

    #[test]
    fn result_to_metadata_delta_no_change_when_job_uuid_empty() {
        let r = DeploymentResult::HPCResult {
            job_uuid: String::new(),
            status: None,
            job_info: "x".into(),
            tapis_user: "u".into(),
            tapis_tenant: "t".into(),
            model_id: "m".into(),
        };
        assert!(matches!(
            TapisJobsModelDeploymentReconciliationClient::result_to_metadata_delta(&r),
            ModelDeploymentMetadataDelta::NoChange
        ));
    }

    #[test]
    fn result_to_metadata_delta_merge_when_job_uuid_set() {
        let r = DeploymentResult::HPCResult {
            job_uuid: "550e8400-e29b-41d4-a716-446655440000".into(),
            status: Some("RUNNING".into()),
            job_info: "status=RUNNING".into(),
            tapis_user: "u".into(),
            tapis_tenant: "t".into(),
            model_id: "Qwen/Qwen3.5-0.8B".into(),
        };
        match TapisJobsModelDeploymentReconciliationClient::result_to_metadata_delta(&r) {
            ModelDeploymentMetadataDelta::Merge(m) => {
                assert_eq!(
                    m.get("job_uuid").and_then(|v| v.as_str()),
                    Some("550e8400-e29b-41d4-a716-446655440000")
                );
                assert_eq!(
                    m.get("model_id").and_then(|v| v.as_str()),
                    Some("Qwen/Qwen3.5-0.8B")
                );
            }
            other => panic!("expected Merge, got {:?}", other),
        }
    }

    // ---- Unit tests: state_from_job_status ----

    #[test]
    fn state_from_job_status_running() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("RUNNING"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_job_status_finished_stopped() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("FINISHED"));
        assert_eq!(s, State::Stopped);
    }

    #[test]
    fn state_from_job_status_failed() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("FAILED"));
        assert_eq!(s, State::Failed);
    }

    #[test]
    fn state_from_job_status_pending_unknown() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("PENDING"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_empty_unknown() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some(""));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_running_is_case_insensitive() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("rUnNiNg"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_job_status_queued_not_failed() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("QUEUED"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_queued_case_insensitive() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("Queued"));
        assert_eq!(s, State::Unknown);
    }

    // ---- Async unit tests: reconcile returns error when metadata missing ----

    #[tokio::test]
    async fn reconcile_start_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("metadata"));
    }

    #[tokio::test]
    async fn reconcile_start_fails_without_hpc_options_when_no_job_uuid() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_with_metadata(base_tapis_meta());
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("tapis_hpc_app_id"));
    }

    #[tokio::test]
    async fn reconcile_stop_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
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
        let client = TapisJobsModelDeploymentReconciliationClient::new();
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
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    // ---- Integration tests: real Tapis Jobs API ----
    // Run with: cargo test -p shared integration_reconcile_hpc_ -- --ignored --nocapture
    // Submit: TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN + TAPIS_HPC_* (see has_tapis_hpc_submit_env).
    // Observe/stop/undeploy with existing job: TEST_JOB_UUID + TAPIS_*.

    fn has_tapis_credentials() -> bool {
        std::env::var("TAPIS_TENANT_URL").is_ok()
            && std::env::var("TAPIS_USER").is_ok()
            && std::env::var("TAPIS_TOKEN").is_ok()
    }

    fn has_tapis_hpc_submit_env() -> bool {
        has_tapis_credentials()
            && std::env::var("TAPIS_HPC_APP_ID").is_ok()
            && std::env::var("TAPIS_HPC_APP_VERSION").is_ok()
            && std::env::var("TAPIS_HPC_EXEC_SYSTEM_ID").is_ok()
            && std::env::var("TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE").is_ok()
            && std::env::var("TAPIS_HPC_MAX_MINUTES").is_ok()
            && std::env::var("TAPIS_HPC_ALLOCATION").is_ok()
    }

    fn has_test_job_uuid() -> bool {
        std::env::var("TEST_JOB_UUID")
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
    }

    fn deployment_with_tapis_jobs_meta(
        deployment_id: Uuid,
        tenant_url: &str,
        tapis_user: &str,
        tapis_token: &str,
        job_uuid: Option<&str>,
        hpc_submit: Option<(
            String,
            String,
            String,
            String,
            i32,
            String,
        )>,
    ) -> ModelDeployment {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!(tenant_url));
        meta.insert("tapis_user".into(), json!(tapis_user));
        meta.insert("tapis_token".into(), json!(tapis_token));
        if let Some(j) = job_uuid {
            meta.insert("job_uuid".into(), json!(j));
        }
        if let Some((app_id, app_ver, exec_id, queue, max_min, alloc)) = hpc_submit {
            meta.insert("tapis_hpc_app_id".into(), json!(app_id));
            meta.insert("tapis_hpc_app_version".into(), json!(app_ver));
            meta.insert("tapis_hpc_exec_system_id".into(), json!(exec_id));
            meta.insert("tapis_hpc_exec_system_logical_queue".into(), json!(queue));
            meta.insert("tapis_hpc_max_minutes".into(), json!(max_min));
            meta.insert("tapis_hpc_allocation".into(), json!(alloc));
        }
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: deployment_id,
            platform: Platform::TapisJobs,
            owner: tapis_user.to_string(),
            model: ModelReference {
                name: std::env::var("FLEXSERV_MODEL_NAME")
                    .unwrap_or_else(|_| "Qwen3.5-0.8B".into()),
                author: std::env::var("FLEXSERV_MODEL_AUTHOR").unwrap_or_else(|_| "Qwen".into()),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-jobs:default".into()),
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(ModelDeploymentMetadata(meta)),
        })
    }

    fn assert_and_print_metadata_delta_hpc(
        label: &str,
        metadata_delta: &Option<ModelDeploymentMetadataDelta>,
    ) {
        match metadata_delta {
            Some(ModelDeploymentMetadataDelta::Merge(m)) => {
                let job_uuid = m.get("job_uuid").and_then(|v| v.as_str());
                let model_id = m.get("model_id").and_then(|v| v.as_str());
                let job_info = m
                    .get("job_info")
                    .and_then(|v| v.as_str())
                    .map(|s| {
                        const PREVIEW: usize = 400;
                        let mut out: String = s.chars().take(PREVIEW).collect();
                        if s.chars().count() > PREVIEW {
                            out.push_str(" ...<truncated>");
                        }
                        out
                    });
                eprintln!(
                    "{} response: job_uuid={:?} model_id={:?} job_info_preview={:?}",
                    label, job_uuid, model_id, job_info
                );
                assert!(m.get("job_uuid").is_some(), "metadata should have job_uuid");
                assert!(m.get("model_id").is_some(), "metadata should have model_id");
                assert!(m.get("tapis_user").is_some(), "metadata should have tapis_user");
                assert!(m.get("tapis_tenant").is_some(), "metadata should have tapis_tenant");
            }
            Some(ModelDeploymentMetadataDelta::Delete) => {
                eprintln!("{} response: metadata marked for deletion", label);
            }
            Some(ModelDeploymentMetadataDelta::NoChange) => {
                eprintln!("{} response: no metadata changes", label);
            }
            None => panic!("Expected metadata delta in outcome"),
        }
    }

    /// First submission: no `job_uuid`; requires full TAPIS_HPC_* env set.
    #[tokio::test]
    #[ignore = "requires TAPIS_* + TAPIS_HPC_* env and real Tapis Jobs API"]
    async fn integration_reconcile_hpc_submit_only() {
        if !has_tapis_hpc_submit_env() {
            eprintln!(
                "Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN and TAPIS_HPC_APP_ID, TAPIS_HPC_APP_VERSION, TAPIS_HPC_EXEC_SYSTEM_ID, TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE, TAPIS_HPC_MAX_MINUTES, TAPIS_HPC_ALLOCATION"
            );
            return;
        }
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();
        let hpc = (
            std::env::var("TAPIS_HPC_APP_ID").unwrap(),
            std::env::var("TAPIS_HPC_APP_VERSION").unwrap(),
            std::env::var("TAPIS_HPC_EXEC_SYSTEM_ID").unwrap(),
            std::env::var("TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE").unwrap(),
            std::env::var("TAPIS_HPC_MAX_MINUTES")
                .unwrap()
                .parse::<i32>()
                .expect("TAPIS_HPC_MAX_MINUTES must be i32"),
            std::env::var("TAPIS_HPC_ALLOCATION").unwrap(),
        );

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            None,
            Some(hpc),
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile submit");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_eq!(p.state, State::Unknown);
                assert_and_print_metadata_delta_hpc("submit", &p.metadata);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Observe an existing job. Set `TEST_JOB_UUID` to a valid Tapis job id.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_observe_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID (e.g. from a successful submit)");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile observe");
        match &outcome {
            ReconciliationOutcome::Observed(p) => {
                assert!(p.message.is_some());
                eprintln!("observed state: {:?}", p.state);
                assert_and_print_metadata_delta_hpc("observe", &p.metadata);
            }
            other => panic!("expected Observed, got {:?}", other),
        }
    }

    /// Cancel/stop an existing job (`TEST_JOB_UUID`).
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_stop_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile stop");
        match &outcome {
            ReconciliationOutcome::Stopped(p) => {
                assert!(p.message.is_some());
                assert_and_print_metadata_delta_hpc("stop", &p.metadata);
            }
            other => panic!("expected Stopped, got {:?}", other),
        }
    }

    /// Undeploy maps to cancel (same as stop for HPC).
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_undeploy_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile undeploy");
        match &outcome {
            ReconciliationOutcome::Undeployed(p) => {
                assert!(p.message.is_some());
            }
            other => panic!("expected Undeployed, got {:?}", other),
        }
    }
}
