use uuid::Uuid;

pub struct PublishArtifactInput {
    pub artifact_id: Uuid,
    pub target_platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

pub struct ListModelPublicationsInput {}

pub struct GetModelPublicationInput {
    pub publication_id: Uuid
}

pub struct ListPublicationsByArtifactIdInput {
    pub artifact_id: Uuid,
}