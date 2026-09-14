use uuid::Uuid;

use crate::shared_kernel::{enums::Visibility, identifiers::ExternalModelId};

#[derive(Debug, Clone)]
pub struct CreateModelInput {
    pub name: String,
    pub description: Option<String>,
    pub external_model_id: ExternalModelId,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct AssociateModelWithArtifactInput {
    pub artifact_id: Uuid,
    pub model_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ListModelsInput {
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl ListModelsInput {
    pub const DEFAULT_LIMIT: u16 = 100;
    pub const MAX_LIMIT: u16 = 100;

    pub fn new(limit: Option<u16>, cursor: Option<String>, include_count: Option<bool>) -> Self {
        Self {
            limit: limit
                .unwrap_or(Self::DEFAULT_LIMIT)
                .clamp(1, Self::MAX_LIMIT),
            cursor,
            include_count: include_count.unwrap_or(false),
        }
    }

    pub fn limit(&self) -> u16 {
        self.limit
    }

    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    pub fn include_count(&self) -> bool {
        self.include_count
    }
}
