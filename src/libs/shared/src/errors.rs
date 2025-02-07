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
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.message)
    }
}


impl BaseError for Error {}