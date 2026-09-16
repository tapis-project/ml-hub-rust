pub mod artifact_ingestions;
pub mod artifact_publications;
pub mod artifacts;
pub mod common;
pub mod datasets;
pub mod deployment;
pub mod hpc_clusters;
pub mod models;

pub mod associate_model;
pub mod create_agent;
pub mod create_agent_record;
pub mod create_model;
pub mod discover_models;
pub mod download_dataset;
pub mod download_model;
pub mod get_dataset_by_platform;
pub mod get_model_by_platform;
pub mod ingest_canonical_model;
pub mod ingest_dataset;
pub mod ingest_model;
pub mod list_agent_records;
pub mod list_agents;
pub mod list_datasets_by_platform;
pub mod list_models_by_platform;
pub mod publish_dataset;
pub mod upload_model;

// pub mod skills;
// pub mod domains;
pub mod errors;

#[cfg(test)]
#[path = "dataset_client_contracts.test.rs"]
mod dataset_client_contracts_test;
