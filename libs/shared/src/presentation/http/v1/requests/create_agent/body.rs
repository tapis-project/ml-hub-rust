use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAgentBody {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
}

#[cfg(test)]
#[path = "body.test.rs"]
mod body_test;
