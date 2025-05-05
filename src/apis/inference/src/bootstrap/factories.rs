//! This module contains factories that wire together infrastructure-level concerns
//! with application level concerns
use mongodb::Database;
use crate::application::repositories::InferenceServerRepository;
use crate::infra::mongo::repositories::InferenceServerRepository as MongoInferenceServerRepository;
use std::sync::Arc;

#[cfg(feature = "mongodb")]
pub fn inference_server_repo_factory(db: Database) -> Arc<dyn InferenceServerRepository> {
    Arc::new(MongoInferenceServerRepository::new(db))
}