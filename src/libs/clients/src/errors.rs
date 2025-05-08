use std::fmt::{Display, Formatter, Result as FormatResult};

/// Represents an error that occurs within the registrar. This error is returned
/// from registrar method calls as the `Err` variant of the `Result` enum
/// 
/// # Fields
/// 
/// - `message`: A `String` of human readable error message
#[derive(Debug)]
pub struct RegistrarError {
    message: String
}

/// Inherent implementation for RegistrarError
impl RegistrarError {
    /// Creates a new `RegistrarError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string describing the error.
    ///
    /// # Returns
    ///
    /// A new instance of `RegistrarError`.
    pub fn new(message: String) -> Self {
        RegistrarError {
            message,
        }
    }
}

/// Implementation of std::fmt::Display for RegisrarError
impl Display for RegistrarError {
    /// Formats the error message for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to display the error message.
    ///
    /// # Returns
    ///
    /// A FormatResutl which == Result<(), std::fmt::Error>
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}