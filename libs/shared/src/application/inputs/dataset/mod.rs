pub mod mappers;

#[derive(Clone, Debug)]
pub struct ListDatasetsInput {
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl ListDatasetsInput {
    pub const DEFAULT_LIMIT: u16 = 100;
    pub const MAX_LIMIT: u16 = 100;
    pub const MIN_LIMIT: u16 = 1;

    pub fn new(limit: Option<u16>, cursor: Option<String>, include_count: Option<bool>) -> Self {
        Self {
            limit: limit
                .unwrap_or(Self::DEFAULT_LIMIT)
                .clamp(Self::MIN_LIMIT, Self::MAX_LIMIT),
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

#[derive(Clone, Debug)]
pub struct RegisterDatasetInput {
    pub tags: Vec<String>,
    pub provider: DatasetProviderInput,
    pub items: Vec<DatasetItemInput>,
    pub size: u64,
    pub visibility: VisibilityInput,
}

#[derive(Clone, Debug)]
pub struct DatasetItemInput {
    pub path: String,
    pub size: u64,
}

#[derive(Clone, Debug)]
pub enum DatasetProviderInput {
    HuggingFace(HuggingFaceRepoLocatorInput),
    Tapis(TapisSystemLocatorInput),
}

#[derive(Clone, Debug)]
pub struct HuggingFaceRepoLocatorInput {
    pub id: String,
    pub sha: String,
}

#[derive(Clone, Debug)]
pub struct TapisSystemLocatorInput {
    pub site_id: String,
    pub tenant_id: String,
    pub system_id: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub enum VisibilityInput {
    Public,
    Private,
}
