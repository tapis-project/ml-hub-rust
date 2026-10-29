use std::error::Error;

use thiserror::Error;

use super::format_error_chain;

#[derive(Debug, Error)]
#[error("TLS certificate validation failed")]
struct TlsError;

#[derive(Debug, Error)]
#[error("request could not be sent")]
struct RequestError {
    #[source]
    source: TlsError,
}

#[test]
fn formats_every_error_source() {
    let error = RequestError { source: TlsError };

    let formatted = format_error_chain(&error);

    assert_eq!(
        formatted,
        "request could not be sent; caused by: TLS certificate validation failed"
    );
}

#[derive(Debug)]
struct DuplicateMessageError {
    source: TlsError,
}

impl std::fmt::Display for DuplicateMessageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(formatter)
    }
}

impl Error for DuplicateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[test]
fn omits_adjacent_duplicate_messages_from_transparent_wrappers() {
    let error = DuplicateMessageError { source: TlsError };

    let formatted = format_error_chain(&error);

    assert_eq!(formatted, "TLS certificate validation failed");
}
