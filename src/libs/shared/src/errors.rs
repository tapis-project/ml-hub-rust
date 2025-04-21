use std::error::Error as BaseError;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message: String) -> Self {
        Error {
            message,
        }
    }

    pub fn from_str(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}

impl BaseError for Error {}

#[derive(Debug)]
pub struct ClientError {
    message: String
}

impl ClientError {
    pub fn new(message: String) -> Self {
        ClientError {
            message,
        }
    }

    pub fn from_str(message: &str) -> Self {
        ClientError {
            message: message.to_string(),
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}

impl BaseError for ClientError {}