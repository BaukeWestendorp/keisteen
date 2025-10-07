use std::{fmt, str};

use eyre::{ContextCompat, bail};

use crate::error::{KeisteenError, KeisteenResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> KeisteenResult<Self> {
        let namespace = namespace.into();
        let value = value.into();

        // Validate namespace
        if !namespace.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-' || c == '_'
        }) {
            bail!("invalid namespace");
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
            bail!("invalid value");
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
    type Err = KeisteenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let namespace = split.next().wrap_err("missing namespace")?;
        let value = split.next().wrap_err("missing separator")?;
        Self::new(namespace.to_string(), value.to_string())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}

impl serde::Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(|err| {
            serde::de::Error::custom(format!("Could not deserialize invalid identifier: {err}"))
        })
    }
}
