use async_trait;
use strum_macros::{EnumString, EnumIter, Display};
use platforms::Platform;

#[derive(Eq, PartialEq, EnumIter, EnumString, Display)]
pub enum Capability {
    ListModels,
    GetModel,
    IngestModel,
    DiscoverModels,
    PublishModel,
    PublishModelMetadata,
    ListDatasets,
    GetDataset,
    IngestDataset,
    DiscoverDatasets,
    PublishDataset,
    PublishDatasetMetadata,
    ConvertModelMetadata
}

#[async_trait::async_trait]
pub trait Client: Send + Sync {
    /// Returns the platform platform this client belongs to
    fn platform(&self) -> Option<Platform>;

    /// Lists the capabilities of the client
    fn capabilities(&self) -> Option<Vec<Capability>>;

    /// Determines if a client as a capability
    fn has_capability(&self, capability: &Capability) -> bool {
        if let Some(capabilities) = self.capabilities() {
            return capabilities.contains(capability)
        }

        return false
    }
}
