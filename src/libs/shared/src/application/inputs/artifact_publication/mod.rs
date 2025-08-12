use uuid::Uuid;

pub struct PublishArtifactInput {
    pub artifact_id: Uuid,
    pub target_platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}