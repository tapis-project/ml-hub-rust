pub mod urn;
pub mod traits;

/// A macro to automatically implement `UrnGenerator` for any domain entity struct.
/// 
/// Accepts a tenant identifier field, a resource type string, and one or more 
/// identity fields that form a composite resource identifier delimited by `/`.
///
/// Format produced: `urn:mlhub:<tenant_id>:<resource>:<id_part1>/.../<id_partn>`
#[macro_export]
macro_rules! impl_urn_generator {
    ($struct_name:ty, $tenant_field:ident, $resource:expr, $($id_field:ident),+ $(,)?) => {
        impl $crate::shared_kernel::identifiers::traits::UrnGenerator for $struct_name {
            fn urn(&self) -> $crate::shared_kernel::identifiers::urn::Urn {
                // Format each identifier field to a String and join them with "/"
                let resource_identifier = [
                    $(format!("{}", self.$id_field)),+
                ].join("/");

                $crate::shared_kernel::identifiers::urn::Urn::new(format!(
                    "urn:mlhub:{}:{}:{}",
                    self.$tenant_field, $resource, resource_identifier
                ))
            }
        }
    };
}