use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Urn(String);

impl Urn {
    pub fn new(urn: String) -> Self {
        Self(urn)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Urn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}