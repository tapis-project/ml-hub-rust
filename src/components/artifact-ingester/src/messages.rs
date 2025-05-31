use shared::common::presentation::http::v1::dto::{Archive, Compression, Parameters};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Deserialize, Debug)]
pub struct StageArtifactJobRequest {
    pub r#type: ArtifactType,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub archive: Option<Archive>,
    pub compression: Option<Compression>,
    pub filename: Option<String>,
    pub params: Option<Parameters>,
    pub platform: String,
    pub artifact_id: String,
}