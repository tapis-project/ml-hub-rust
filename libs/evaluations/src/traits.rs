use thiserror::Error;

use crate::{Arguments, FieldPath, FieldValue};

#[derive(Debug, Clone, Error)]
pub enum ValueResolutionError {
    #[error("Missing or Invalid field path: {0}")]
    InvalidFieldPath(String),

    #[error("Failed to convert value at field path {0} into a Value")]
    FieldPathValueConversionError(FieldPath),
}

pub trait ResolveValue {
    /// Resolves a supported field path to its value.
    ///
    /// Implementations return [`FieldValue::Undefined`] when the path is valid but no value is
    /// present. Unsupported or malformed paths return [`ValueResolutionError::InvalidFieldPath`].
    fn resolve_value(
        &self,
        field_path: Option<FieldPath>,
    ) -> Result<FieldValue, ValueResolutionError>;
}

pub trait Evaluate {
    type Error;

    fn evaluate(&self, arguments: &Arguments) -> Result<bool, Self::Error>;
}
