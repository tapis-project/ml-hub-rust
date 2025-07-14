#[derive(Clone)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Clone)]
pub struct IngestArtifactInput {
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub platform_artifact_id: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone)]
pub struct UploadArtifactInput {
    pub artifact_type: ArtifactType,
}