use async_trait;

use crate::application::errors::ApplicationError;

#[async_trait::async_trait]
pub trait CountRepository {
    // Returns the string identitifer of the resource i.e., of the count
    fn resource(&self) -> &'static str;
    async fn find_by_field_value(&self, input: FindByFieldValueInput) -> Result<Option<u128>, ApplicationError>;
    async fn increment_count(&self, input: IncrementInput) -> Result<u128, ApplicationError>;
    async fn decrement_count(&self, input: DecrementInput) -> Result<u128, ApplicationError>;
}

#[derive(Clone, Debug)]
pub struct FindByFieldValueInput {
    pub tenant_id: String,
    pub resource_owner_id: String,
    pub field: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct IncrementInput {
    pub tenant_id: String,
    pub resource_owner_id: String,
    pub field: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct DecrementInput {
    pub tenant_id: String,
    pub resource_owner_id: String,
    pub field: String,
    pub value: String,
}
