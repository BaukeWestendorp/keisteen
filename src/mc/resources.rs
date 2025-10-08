use core::fmt;
use std::str;

use eyre::{ContextCompat, bail};

use crate::error::{KeisteenError, KeisteenResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

impl ResourceLocation {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> KeisteenResult<Self> {
        let namespace = namespace.into();
        let path = path.into();

        // Validate namespace
        if !namespace.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-' || c == '_'
        }) {
            bail!("invalid namespace");
        }

        // Validate path
        if !path.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '.'
                || c == '-'
                || c == '_'
                || c == '/'
        }) {
            bail!("invalid path");
        }

        Ok(Self { namespace, path })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl str::FromStr for ResourceLocation {
    type Err = KeisteenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let namespace = split.next().wrap_err("missing namespace")?;
        let path = split.next().wrap_err("missing separator")?;
        Self::new(namespace.to_string(), path.to_string())
    }
}

impl fmt::Display for ResourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl serde::Serialize for ResourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ResourceLocation {
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
