use std::error::Error;

#[cfg(test)]
#[path = "error_chain.test.rs"]
mod error_chain_test;

/// Formats every available error source instead of discarding nested transport details.
///
/// Adapter errors frequently wrap the actionable DNS, connection, or TLS failure. This
/// formatter is intended for trusted server logs and infrastructure error messages. Callers
/// must not pass errors that may contain credentials or other sensitive request data.
pub fn format_error_chain(error: &(dyn Error + 'static)) -> String {
    let mut messages = Vec::new();
    let mut current = Some(error);

    while let Some(cause) = current {
        let message = cause.to_string();

        if messages.last() != Some(&message) {
            messages.push(message);
        }

        current = cause.source();
    }

    messages.join("; caused by: ")
}
