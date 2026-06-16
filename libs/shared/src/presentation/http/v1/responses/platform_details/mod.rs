use serde::Serialize;
use platforms::Platform;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PlatformDetails {
    pub name: Platform,
    pub capabilities: Vec<String>,
}