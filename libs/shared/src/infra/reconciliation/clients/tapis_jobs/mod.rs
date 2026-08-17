use platforms::Platform;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::json;
use tapis_jobs::models::{JobArgSpec, ReqSubmitJob};
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
use crate::domain::entities::deployment_strategy::strategy::Strategy;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::site::SiteContext;

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
    site_context: SiteContext,
    client: Client,
    base_job_definition: ReqSubmitJob,
}

impl TapisJobsModelDeploymentReconciliationClient {
    const FLEXSERV_APP_ID: &'static str = "FlexServ-1.4.0";
    const FLEXSERV_APP_VERSION: &'static str = "1.4.0";
    const FLEXSERV_JOB_DEF_URL: &'static str = "https://raw.githubusercontent.com/tapis-project/FlexServ-Deployer/refs/heads/main/tapis_def/1.4.0/job.json";
    const TAPIS_JOB_UUID_KEY: &'static str = "tapis_job_uuid";

    pub async fn new(site_context: &SiteContext) -> Result<Self, ReconcilerError> {
        let mlhub_service_password = match env::var("MLHUB_SERVICE_PASSWORD") {
            Ok(p) => p,
            Err(_) => {
                let msg = "Could not initialize Tapis Jobs deployment reconciler";
                log::error!("{}", msg);
                return Err(ReconcilerError::InitializationFailed(msg.into()))
            }
        };

        // Initialize http client
        let client = Client::new();
        
        // Fetch the base tapis job def
        let base_job_definition = client.get(Self::FLEXSERV_JOB_DEF_URL)
            .send()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Error fetching FlexServ job definition: {}", error.error_id(), e.to_string());
                ReconcilerError::InitializationFailed(error.to_string())
            })?
            .json::<ReqSubmitJob>()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Error deserializing FlexServ job definition: {}", error.error_id(), e.to_string());
                ReconcilerError::InitializationFailed(error.to_string())
            })?;

        Ok(Self {
            client,
            base_job_definition,
            mlhub_service_password,
            site_context: site_context.clone(),
        })
    }

    async fn handle_start(
        &self,
        deployment: &ModelDeployment,
        model: &ModelMetadata,
        strategy: Option<Strategy>,
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

        let jobs_client = TapisJobs::new(&self.get_target_base_url(deployment), Some(&service_jwt))
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

        let strat = match strategy {
            Some(s) => s,
            None => {
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Missing strategy", error.error_id());
                return Err(ReconciliationError::Fatal(error))
            }
        };

        let job_def = self.build_job_request(
            &model_id,
            deployment,
            &strat,
            arguments
        )?;

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
        map.insert(Self::TAPIS_JOB_UUID_KEY.to_string(), json!(job_uuid));

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

        let resp = self.client.post(format!("{}/v3/tokens", self.get_site_context().base_url))
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

    fn state_from_job_status(status: &str) -> State {
        match status {
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

    fn get_target_base_url(&self, deployment: &ModelDeployment) -> String {
        let target_base_url = self.get_site_context()
            .base_url
            .clone()
            .replace("admin", &deployment.tenant_id.clone());

        target_base_url
    }

    fn build_job_request(&self, model_id: &str, deployment: &ModelDeployment, strategy: &Strategy, args: &[DecryptedArgument]) -> Result<ReqSubmitJob, ReconciliationError> {
        let mut job_def = self.base_job_definition.clone();
        
        job_def.name = deployment.name.clone();
        job_def.app_id = Self::FLEXSERV_APP_ID.into();
        job_def.app_version = Self::FLEXSERV_APP_VERSION.into();
        job_def.tenant = Some(deployment.tenant_id.clone());
        job_def.owner = Some(deployment.owner.clone());
        
        // Set the model id
        let target_spec = job_def.parameter_set
            .as_mut()
            .and_then(|params| params.app_args.as_mut())
            .and_then(|args| {
                args.iter_mut()
                    .find(|spec| spec.name.as_deref() == Some("modelName"))
        });

        if let Some(spec) = target_spec {
            spec.arg = Some(format!("--model-name {}", model_id));
        }

        let target_host_name = args.iter()
            .find(|a| a.parameter_name == "HPC System")
            .map(|arg| &arg.value)
            .ok_or_else(|| {
                let message: String = "'HPC System' not provided in arguments".into();
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Missing argument: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;


        let strategy_data = strategy
            .data()
            .clone()
            .unwrap_or_default();
        
        // The key to retrieve the exec_system_id from the data on the Strategy
        let exec_system_data_strategy_key = format!("{}_tapis_system_id", &target_host_name.to_ascii_lowercase());

        let exec_system_id = strategy_data
            .get(&exec_system_data_strategy_key)
            .ok_or_else(|| {
                let message = format!("Could not find data on strategy at key '{}'", &exec_system_data_strategy_key);
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Missing strategy data: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;

        job_def.exec_system_id = Some(exec_system_id.clone());
        
        // Set the logical queue
        let exec_system_logical_queue = args.iter()
            .find(|a| a.parameter_name == "Slurm Partition")
            .map(|arg| &arg.value)
            .ok_or_else(|| {
                let message: String = "'Slurm Partition' not provided in arguments".into();
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Missing argument: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;

        job_def.exec_system_logical_queue = Some(exec_system_logical_queue.clone());
        
        // Set the slurm allocation
        let slurm_allocation = args.iter()
            .find(|a| a.parameter_name == "Slurm Allocation")
            .map(|arg| &arg.value)
            .ok_or_else(|| {
                let message: String = "'Slurm Allocation' not provided in arguments".into();
                let error = InfrastructureError::new_internal();
                log::error!("[{}] Missing argument: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;

        let target_spec = job_def.parameter_set
            .as_mut()
            .and_then(|params| params.scheduler_options.as_mut())
            .and_then(|opts| {
                opts.iter_mut()
                    .find(|spec| spec.name.as_deref() == Some("TACC Resource Allocation"))
            });

        if let Some(spec) = target_spec {
            spec.arg = Some(format!("-A {}", slurm_allocation));
        }

        let maybe_slurm_reservation = args.iter()
            .find(|a| a.parameter_name == "Slurm Reservation")
            .map(|arg| &arg.value);

        if let Some(slurm_reservation) = maybe_slurm_reservation {
            let scheduler_opts = job_def.parameter_set.as_mut()
                .and_then(|params| params.scheduler_options.as_mut());

            if let Some(opts) = scheduler_opts {
                opts.push(JobArgSpec { 
                    name: Some("Slurm Reservation".to_string()), 
                    arg: Some(format!("-R {}", slurm_reservation)), 
                    description: Some("The Slurm Reservation (set by MLHub)".into()), 
                    include: Some(true), 
                    notes: None 
                });
            }   
        }
       
        Ok(job_def)
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
                input.strategy,
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

    fn get_site_context(&self) -> &SiteContext {
        return &self.site_context;
    }
}
