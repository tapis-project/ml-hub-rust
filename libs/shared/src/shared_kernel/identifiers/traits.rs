use super::urn::Urn;

/// A trait defining the capability to generate a unique, uniform identifier
/// for domain entities within the `mlhub` ecosystem.
pub trait UrnGenerator {
    /// Generates the standard URN identifier for the specific entity instance.
    fn urn(&self) -> Urn;
}