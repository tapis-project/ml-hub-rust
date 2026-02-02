use crate::application::ports::events::Event;
use crate::application::ports::commands::Command;

pub const ARTIFACT_INGESTION_ROUTING_KEY: &'static str = "artifact.ingest.queue";
pub const ARTIFACT_PUBLICATION_ROUTING_KEY: &'static str = "artifact.publish.queue";
pub const MODEL_DEPLOYMENT_ROUTING_KEY: &'static str = "model.deploy.queue";

pub fn get_routing_key_for_command(command: &Command) -> &'static str {
    match command {
        Command::IngestArtifactCommand(_) => ARTIFACT_INGESTION_ROUTING_KEY,
        Command::PublishArtifactCommand(_) => ARTIFACT_PUBLICATION_ROUTING_KEY,
    }
}

pub fn get_routing_key_for_event(event: &Event) -> &'static str {
    match event {
        _ => MODEL_DEPLOYMENT_ROUTING_KEY,
    }
}