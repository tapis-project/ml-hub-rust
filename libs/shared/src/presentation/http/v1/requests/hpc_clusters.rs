use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    application::inputs::hpc_cluster::ListHpcClustersInput, domain::entities::hpc_cluster as domain,
};

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct ListHpcClustersPath {
    #[param(inline)]
    pub data_center: DataCenter,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetHpcClusterPath {
    #[param(inline)]
    pub data_center: DataCenter,
    #[param(value_type = String, format = "uuid")]
    pub hpc_cluster_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListHpcClustersQuery {
    /// Maximum number of HPC clusters to return. Values are capped at 100.
    #[param(minimum = 1, maximum = 100)]
    pub limit: Option<u16>,
    /// Opaque cursor returned by the previous page.
    pub cursor: Option<String>,
    /// Include the number of HPC clusters in the selected data center.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub enum DataCenter {
    Tacc,
}

impl TryFrom<&str> for DataCenter {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Tacc" => Ok(Self::Tacc),
            _ => Err(format!("Unsupported data center: {value}")),
        }
    }
}

impl From<DataCenter> for domain::DataCenter {
    fn from(value: DataCenter) -> Self {
        match value {
            DataCenter::Tacc => Self::Tacc,
        }
    }
}

impl ListHpcClustersQuery {
    pub fn into_input(self, data_center: DataCenter) -> ListHpcClustersInput {
        ListHpcClustersInput::new(
            data_center.into(),
            self.limit,
            self.cursor,
            self.include_count,
        )
    }
}

#[cfg(test)]
#[path = "hpc_clusters.test.rs"]
mod hpc_clusters_test;
