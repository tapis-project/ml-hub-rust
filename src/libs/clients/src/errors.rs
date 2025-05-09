use std::fmt::{Display, Formatter, Result as FormatResult};

/// Represents an error that occurs within the provider. This error is returned
/// from provider method calls as the `Err` variant of the `Result` enum
/// 
/// # Fields
/// 
/// - `message`: A `String` of human readable error message
#[derive(Debug)]
pub struct ClientProviderError {
    message: String
}

/// Inherent implementation for ClientProviderError
impl ClientProviderError {
    /// Creates a new `ClientProviderError` with the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string describing the error.
    ///
    /// # Returns
    ///
    /// A new instance of `ClientProviderError`.
    pub fn new(message: String) -> Self {
        ClientProviderError {
            message,
        }
    }
}

/// Implementation of std::fmt::Display for RegisrarError
impl Display for ClientProviderError {
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