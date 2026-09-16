use crate::domain::entities::hpc_cluster::DataCenter;

#[derive(Clone, Debug)]
pub struct ListHpcClustersInput {
    data_center: DataCenter,
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl ListHpcClustersInput {
    pub const DEFAULT_LIMIT: u16 = 100;
    pub const MAX_LIMIT: u16 = 100;
    pub const MIN_LIMIT: u16 = 1;

    pub fn new(
        data_center: DataCenter,
        limit: Option<u16>,
        cursor: Option<String>,
        include_count: Option<bool>,
    ) -> Self {
        Self {
            data_center,
            limit: limit
                .unwrap_or(Self::DEFAULT_LIMIT)
                .clamp(Self::MIN_LIMIT, Self::MAX_LIMIT),
            cursor,
            include_count: include_count.unwrap_or(false),
        }
    }

    pub fn data_center(&self) -> &DataCenter {
        &self.data_center
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
