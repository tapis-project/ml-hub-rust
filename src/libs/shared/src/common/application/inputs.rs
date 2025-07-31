#[derive(Clone, Debug)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Clone, Debug)]
pub struct IngestArtifactInput {
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub platform_artifact_id: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct UploadArtifactInput {
    pub artifact_type: ArtifactType,
}

#[derive(Clone, Debug)]
pub struct DownloadArtifactInput {
    pub artifact_type: ArtifactType,
    pub artifact_id: String,
}