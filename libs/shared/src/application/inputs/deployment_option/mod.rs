#[derive(Clone, Debug)]
pub struct ListDeploymentOptionsInput {
    limit: u16,
    cursor: Option<String>,
    include_count: bool,
}

impl ListDeploymentOptionsInput {
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

#[cfg(test)]
#[path = "deployment_option.test.rs"]
mod deployment_option_test;
