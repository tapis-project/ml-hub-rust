use platforms::Platform;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::{from_str, json};
use tapis_jobs::models::ReqSubmitJob;
use tapis_jobs::{with_headers, TapisJobs};
use tapis_tokens::models::NewTokenResponse;
use thiserror::Error;

use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::ports::errors::InfrastructureError;
use crate::application::services::deployment_argument_service::DecryptedArgument;
use crate::application::workflows::reconciliation::{
    FailedOutcome, ObeservedOutcome, ReconcilerError, ReconciliationAction, ReconciliationOutcome, StartedOutcome, StoppedOutcome, UndeployedOutcome
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, State,
};
use crate::domain::entities::model_metadata::ModelMetadata;

use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Error)]
enum ReconciliationError {
    #[error(transparent)]
    Fatal(#[from] InfrastructureError),

    #[error("{0}")]
    Recoverable(String),
}

pub struct TapisJobsModelDeploymentReconciliationClient {
    mlhub_service_password: String,
    base_url: String,
    client: Client
}

impl TapisJobsModelDeploymentReconciliationClient {
    pub fn new() -> Result<Self, ReconcilerError> {
        let mlhub_service_password = match env::var("MLHUB_SERVICE_PASSWORD") {
            Ok(p) => p,
            Err(_) => {
                let msg = "Could not initialize Tapis Jobs deployment reconciler";
                log::error!("{}", msg);
                return Err(ReconcilerError::InitializationFailed(msg.into()))
            }
        };



        Ok(Self {
            client: Client::new(),
            mlhub_service_password,
            base_url: "".into() // TODO
        })
    }

    /// `job_uuid` from metadata when the deployment already submitted a Tapis job.
    fn extract_job_uuid(deployment: &ModelDeployment) -> Option<String> {
        deployment.metadata.as_ref().and_then(|m| {
            m.get("tapis_job_uuid")?
                .as_str()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
        })
    }

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

    async fn handle_start(
        &self,
        deployment: &ModelDeployment,
        model: &ModelMetadata,
        arguments: &[DecryptedArgument],
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        let (model_id, platform) = match model.canonical.clone() {
            Some(c) => (c.model_id, c.platform),
            None => {
                let message = "Missing canonical model data";
                let error = InfrastructureError::new_internal();
                log::error!("[{}] {}", error.error_id(), &message);
                return Err(ReconciliationError::Fatal(error))
            }
        };

        if platform != Platform::HuggingFace {
            let message = format!("Unsupported platform: {}", &deployment.platform);
            let error = InfrastructureError::new_internal();
            log::error!("[{}] {}", error.error_id(), &message);
            return Err(ReconciliationError::Fatal(error))
        }

        // TODO Check if Job UUID is in the metadata. If so, resubmit instead of submit

        let service_jwt = self.generate_service_token().await?;

        let jobs_client = TapisJobs::new(&self.base_url, Some(&service_jwt))
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Failed to initialize TapisJobs client: {}", error.error_id(), e.to_string());
                error
            })?;
        
        let mut custom_headers = HeaderMap::new();
        
        // OBO Tapis tenant
        let obo_tenant_id = deployment.tenant_id.as_str();
        custom_headers.insert(
            "X-Tapis-Tenant",
            HeaderValue::from_str(&obo_tenant_id)
                .map_err(|e| {
                    let error = InfrastructureError::new_internal();
                    log::error!("[{}] Invalid header value for X-Tapis-Tenant: {}", error.error_id(), e.to_string());
                    error
                })?
        );

        // OBO Tapis username
        let obo_username = deployment.owner.as_str();
        custom_headers.insert(
            "X-Tapis-User",
            HeaderValue::from_str(obo_username)
                .map_err(|e| {
                    let error = InfrastructureError::new_internal();
                    log::error!("[{}] Invalid header value for X-Tapis-User: {}", error.error_id(), e.to_string());
                    error
                })?
        );

        // NOTE We assume the system and FlexServ app exists in the target tenant
        // for the host specificed in the strategy
        let job_def_config = "";

        let mut job_def: ReqSubmitJob = from_str(job_def_config)
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Error deserializing FlexServ job definition: {}", error.error_id(), e.to_string());
                error
            })?;

        // TODO Update all the fields on the job definition with 

        let job_uuid = with_headers(
            custom_headers,
            async { jobs_client.jobs.submit_job(job_def).await }
        )
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Error when submitting FlexServ job to Tapis: {}", error.error_id(), e.to_string());
                error
            })?
            .result
            .ok_or_else(|| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Job is None", error.error_id());
                error
            })?
            .uuid
            .ok_or_else(|| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Uuid is None", error.error_id());
                error
            })?;

        let mut map = HashMap::new();
        map.insert("tapis_job_uuid".to_string(), json!(job_uuid));

        Ok(ReconciliationOutcome::Started(StartedOutcome {
            message: Some("Deployment started successfully".to_string()),
            state: State::Failed,
            metadata: Some(ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))),
            replicas: None,
            interface: None,
        }))
    }

    async fn resubmit_job() -> Result<ReconciliationOutcome, ReconciliationError> {
        todo!();
    }

    async fn handle_stop(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {

        Ok(ReconciliationOutcome::Stopped(StoppedOutcome {
            message: Some("Deployment stopped successfully".to_string()),
            metadata: None,
            replicas: None,
            interface: None,
        }))
    }

    async fn handle_undeploy(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {

        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcome {
            message: Some("Deployment canceled successfully".to_string()),
            metadata: Some(ModelDeploymentMetadataDelta::Delete),
        }))
    }

    async fn handle_observe(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        
        Ok(ReconciliationOutcome::Observed(ObeservedOutcome {
            message: Some("Observing Tapis Job".into()),
            state: State::Unknown, // TODO Put the actual observed state
            metadata: None,
            replicas: None,
            interface: None,
        }))
    }

    async fn generate_service_token(&self) -> Result<String, ReconciliationError> {    
        let payload = json!({
            "account_type": "service",
            "token_tenant_id": "admin",
            "token_username": "mlhub"
        });

        let resp = self.client.post(format!("{}/v3/tokens", self.base_url))
            .basic_auth("mlhub", Some(&self.mlhub_service_password))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] External service call error: {}", error.error_id(), e.to_string());
                error
            })?
            .json::<NewTokenResponse>()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Deserialization error: {}", error.error_id(), e.to_string());
                error
            })?
            .access_token
            .ok_or_else(|| InfrastructureError::new_internal())
            .map_err(|e| {
                log::error!("[{}] Missing access token response: {}", e.error_id(), e.to_string());
                e
            })?
            .access_token
            .ok_or_else(|| InfrastructureError::new_internal())
            .map_err(|e| {
                log::error!("[{}] Missing access token: {}", e.error_id(), e.to_string());
                e
            })?;

        Ok(resp)
    } 
}

#[async_trait::async_trait]
impl ModelDeploymentPlatformReconciliationClient for TapisJobsModelDeploymentReconciliationClient {
    async fn reconcile(
        &self,
        input: ReconcileModelDeploymentInput,
    ) -> ReconciliationOutcome {
        let outcome = match input.action {
            ReconciliationAction::Start { payload } => self.handle_start(
                &input.deployment,
                &input.model_metadata,
                &payload,
            ).await,
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
            ReconciliationAction::Observe => self.handle_observe(&input).await,
        };

        match outcome {
            Ok(o) => o,
            Err(e) => {
                ReconciliationOutcome::Failed(FailedOutcome {
                    message: Some(e.to_string().clone()),
                    metadata: None,
                    replicas: None,
                    interface: None,
                })
            }
        }
    }
}
