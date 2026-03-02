use serde_json::Value;
use crate::domain::entities::timestamp::TimeStamp;

#[derive(Clone, Debug)]
pub struct FederatedIdentity {
    pub idp: IdentityProviderIdentifier,
    pub subject: String,
    pub metadata: Value,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

impl FederatedIdentity {
    pub fn idp(&self) -> &String {
        self.idp.into_inner()
    }
}

#[derive(Clone, Debug)]
pub struct IdentityProviderIdentifier(String);

impl IdentityProviderIdentifier {
    fn new(platform_name: String, issuer: String) -> Self {
        Self(format!("{}:{}", platform_name, issuer))
    }

    fn into_inner(&self) -> &String {
        &self.0
    }

    fn rehydrate(inner: String) -> Self {
        Self(inner)
    }
}
