use crate::shared_kernel::identifiers::urn::Urn;

const DEFAULT_LIMIT: u16 = 100;
const MAX_LIMIT: u16 = 100;

#[derive(Clone, Debug)]
pub struct ListEndpointsInput {
    tenant_id: String,
    owner: String,
    target_resource_urn: Option<Urn>,
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl ListEndpointsInput {
    pub fn new(
        tenant_id: String,
        owner: String,
        target_resource_urn: Option<Urn>,
        limit: Option<u16>,
        cursor: Option<String>,
        include_count: bool,
    ) -> Self {
        Self {
            tenant_id,
            owner,
            target_resource_urn,
            limit: limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT),
            cursor,
            include_count,
        }
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn target_resource_urn(&self) -> Option<&Urn> {
        self.target_resource_urn.as_ref()
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
