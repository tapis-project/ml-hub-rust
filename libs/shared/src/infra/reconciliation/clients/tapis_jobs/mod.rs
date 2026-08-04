use crate::application::ports::deployment::ModelDeploymentPlatformReconciliationClient;
use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::services::deployment_argument_service::DecryptedArgument;
use crate::application::workflows::reconciliation::{
    ReconciliationAction, ReconciliationError, ReconciliationOutcome, StartedOutcomePayload,
    StoppedOutcomePayload, UndeployedOutcomePayload, ObeservedOutcomePayload,
};
use crate::domain::entities::deployment::{
    ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, State,
};
use crate::domain::entities::model_metadata::ModelMetadata;

use serde_json::json;
use log::error;

use std::collections::HashMap;
use std::env;

pub struct TapisJobsModelDeploymentReconciliationClient {}

impl TapisJobsModelDeploymentReconciliationClient {
    pub fn new() -> Self {
        match env::var("MLHUB_SERVICE_PASSWORD") {
            Ok(value) => println!("Database URL is: {value}"),
            Err(e) => println!("Couldn't read DATABASE_URL: {e}"),
        }

        Self {}
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
        let canonical_model = match model.canonical.clone() {
            Some(c) => Ok(c.model_id),
            None => Err(ReconciliationError::MissingCanonicalModel(model.name.clone(), model.author.clone()))
        }?;

        // TODO Start

        let mut map = HashMap::new();
        // map.insert("job_uuid".to_string(), json!(job_uuid));
        // map.insert("hpc_url".to_string(), json!(url));

        Ok(ReconciliationOutcome::Started(StartedOutcomePayload {
            message: Some("Deployment started successfully".to_string()),
            state: State::Failed,
            metadata: Some(ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))),
            replicas: None,
            interface: None,
        }))

        
        // Ok(ReconciliationOutcome::Started(StartedOutcomePayload {
        //     message: Some("Deployment started successfully".to_string()),
        //     state: State::Unknown,
        //     metadata: Some(ModelDeploymentMetadataDelta::Merge(ModelDeploymentMetadata(map))),
        //     replicas: None,
        //     interface: None,
        // }))
    }

    async fn handle_stop(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {

        Ok(ReconciliationOutcome::Stopped(StoppedOutcomePayload {
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

        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcomePayload {
            message: Some("Deployment canceled successfully".to_string()),
            metadata: Some(ModelDeploymentMetadataDelta::Delete),
        }))
    }

    async fn handle_observe(
        &self,
        input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        
        Ok(ReconciliationOutcome::Observed(ObeservedOutcomePayload {
            message: Some("Observing Tapis Job".into()),
            state: State::Unknown, // TODO Put the actual observed state
            metadata: None,
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
            ReconciliationAction::Start { payload } => self.handle_start(
                &input.deployment,
                &input.model_metadata,
                &payload,
            ).await,
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
            ReconciliationAction::Observe => self.handle_observe(&input).await,
        }
    }
}
