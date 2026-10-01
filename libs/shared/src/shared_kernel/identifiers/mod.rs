pub mod traits;
pub mod urn;

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
