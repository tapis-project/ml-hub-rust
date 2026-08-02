use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::workflows::reconciliation::{
    ReconciliationAction, ReconciliationError, ReconciliationOutcome, StartedOutcomePayload,
    StoppedOutcomePayload, UndeployedOutcomePayload, ObeservedOutcomePayload,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, State,
};
use flexserv_deployer::{ Backend, FlexServDeployment, FlexServHPCDeployment, FlexServInstance};
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

    // fn create_deployment_from_entity(
    //     deployment: &ModelDeployment,
    // ) -> Result<FlexServHPCDeployment, ReconciliationError> {
    //     let (tenant_url, tapis_user, tapis_token) = Self::extract_tapis_credentials(deployment)?;
    //     let model_id = format!("{}/{}", deployment.model.author, deployment.model.name);

    //     let server = FlexServInstance::builder()
    //         .tenant_url(normalize_tenant_url(&tenant_url))
    //         .tapis_user(tapis_user.clone())
    //         .model(model_id.clone())
    //         .backend(Self::default_backend())
    //         .build()
    //         .map_err(|e| {
    //             ReconciliationError::Unimplemented(format!("Failed to create FlexServInstance: {}", e))
    //         })?;

    //     if let Some(job_uuid) = Self::extract_job_uuid(deployment) {
    //         let mut existing = FlexServHPCDeployment::from_existing(tapis_token, job_uuid);
    //         existing.tenant_url = Some(normalize_tenant_url(&tenant_url));
    //         Ok(existing)
    //     } else {
    //         let options = Self::extract_hpc_options(deployment)?;
    //         Ok(FlexServHPCDeployment::new(server, tapis_token, options))
    //     }
    // }

    fn result_to_metadata_delta(
        result: &DeploymentResult,
        deployment: &ModelDeployment,
    ) -> ModelDeploymentMetadataDelta {
        match result {
            DeploymentResult::HPCResult {
                job_uuid,
                status,
                job,
                hpc_url,
                flexserv_token,
                ..
            } => {
                if job_uuid.is_empty() {
                    log::warn!("HPC operation returned empty job_uuid, not updating job metadata keys");
                    return ModelDeploymentMetadataDelta::NoChange;
                }
                let mut map = HashMap::new();
                map.insert("job_uuid".to_string(), json!(job_uuid));
                map.insert(
                    "model_id".to_string(),
                    json!(format!(
                        "{}/{}",
                        deployment.model.author, deployment.model.name
                    )),
                );
                if let Some(status) = status {
                    if !status.is_empty() {
                        map.insert("job_status".to_string(), json!(status));
                    }
                }
                if let Some(job) = job {
                    map.insert("job_info".to_string(), json!(job));
                }
                if let Some(url) = hpc_url {
                    if !url.is_empty() {
                        map.insert("hpc_url".to_string(), json!(url));
                    }
                }
                if let Some(token) = flexserv_token {
                    if !token.is_empty() {
                        map.insert("flexserv_token".to_string(), json!(token));
                    }
                }
                if let Some(metadata) = deployment.metadata.as_ref() {
                    if let Some(tapis_user) = metadata.get("tapis_user") {
                        map.insert("tapis_user".to_string(), tapis_user.clone());
                    }
                    if let Some(tapis_tenant) = metadata.get("tapis_tenant") {
                        map.insert("tapis_tenant".to_string(), tapis_tenant.clone());
                    } else if let Some(tapis_tenant_url) = metadata.get("tapis_tenant_url") {
                        map.insert("tapis_tenant_url".to_string(), tapis_tenant_url.clone());
                    }
                }
                ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))
            }
            _ => ModelDeploymentMetadataDelta::NoChange,
        }
    }

    /// Infer [State] from raw TAPIS job status.
    fn state_from_job_status(status: Option<&str>) -> State {
        match status
            .unwrap_or_default()
            .trim()
            .to_ascii_uppercase()
            .as_str()
        {
            "RUNNING" => State::Running,
            "QUEUED"
            | "PENDING"
            | "PROCESSING_INPUTS"
            | "STAGING_INPUTS"
            | "STAGING_JOB"
            | "SUBMITTING_JOB"
            | "ARCHIVING" => State::Unknown,
            "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" => State::Stopped,
            "FAILED" => State::Failed,
            "BLOCKED" | "PAUSED" => State::Blocked,
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
        let canonical_model = match input.model_metadata.canonical {
            Some(c) => Ok(c.model_id),
            None => Err(ReconciliationError::MissingCanonicalModel(input.model_metadata.name.clone(), input.model_metadata.author.clone()))
        }?;
        
        // TODO handle exsiting job with jobuuid from deployment metadata
        let result = FlexServHPCDeployment::new(
            FlexServInstance {
                backend: Backend::Transformers { command: vec![] },
                default_embedding_model: None,
                tapis_user: input.deployment.owner.clone(),
                default_model: canonical_model,
            }
        ).create();


        Ok(ReconciliationOutcome::Started(StartedOutcomePayload {
            message: Some("Deployment started successfully".to_string()),
            state: State::Unknown,
            metadata: Some(Self::result_to_metadata_delta(&result, &input.deployment)),
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
            metadata: Some(Self::result_to_metadata_delta(&result, &input.deployment)),
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
            metadata: Some(Self::result_to_metadata_delta(&result, &input.deployment)),
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
#[path = "tapis_jobs.test.rs"]
mod tapis_jobs_test;
