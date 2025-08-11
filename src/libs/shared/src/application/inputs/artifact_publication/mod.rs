use uuid::Uuid;

pub struct PublishArtifactInput {
    pub artifact_id: Uuid,
    pub platform: String,
}