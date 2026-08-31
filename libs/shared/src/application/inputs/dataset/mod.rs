pub mod mappers;

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
