use std::fmt;

use uuid::Uuid;

pub mod traits;
pub mod urn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalModelId(Uuid);

impl ExternalModelId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn reconstitute(value: Uuid) -> Self {
        Self(value)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for ExternalModelId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ExternalModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A macro to automatically implement `UrnGenerator` for any domain entity struct.
#[macro_export]
macro_rules! impl_urn_generator {
    ($struct_name:ty, $tenant_field:ident, $resource:expr, $($id_field:ident),+ $(,)?) => {
        impl $crate::shared_kernel::identifiers::traits::UrnGenerator for $struct_name {
            fn urn(&self) -> $crate::shared_kernel::identifiers::urn::Urn {
                let resource_identifier = [
                    $(format!("{}", self.$id_field)),+
                ].join("/");

                $crate::shared_kernel::identifiers::urn::Urn::new(format!(
                    "urn:mlhub:v1:{}:{}:{}",
                    self.$tenant_field, $resource, resource_identifier
                ))
            }
        }
    };
}
