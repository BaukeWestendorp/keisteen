use std::{fmt, str};

use crate::error::CraftError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> Result<Self, CraftError> {
        let namespace = namespace.into();
        let value = value.into();

        // Validate namespace
        if !namespace.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-' || c == '_'
        }) {
            return Err(CraftError::InvalidIdentifierNamespace(namespace));
        }

        // Validate value
        if !value.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '.'
                || c == '-'
                || c == '_'
                || c == '/'
        }) {
            return Err(CraftError::InvalidIdentifierValue(value));
        }

        Ok(Self { namespace, value })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl str::FromStr for Identifier {
    type Err = CraftError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let Some(namespace) = split.next() else {
            return Err(CraftError::InvalidNamespace);
        };
        let Some(value) = split.next() else {
            return Err(CraftError::InvalidNamespace);
        };
        Self::new(namespace.to_string(), value.to_string())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}
