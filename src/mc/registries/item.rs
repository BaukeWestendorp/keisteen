use std::ops::Deref;
use std::sync::Arc;

use crate::mc::registries::Registry;

#[derive(Debug, PartialEq, Eq)]
pub struct RegItem<T: Registry> {
    inner: Arc<T>,
}

impl<T: Registry> RegItem<T> {
    pub fn new(inner: T) -> Self {
        Self { inner: Arc::new(inner) }
    }
}

impl<T: Registry> Clone for RegItem<T> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl<T: Registry> Deref for RegItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Registry> From<T> for RegItem<T> {
    fn from(value: T) -> Self {
        Self { inner: Arc::new(value) }
    }
}

impl<T: Registry> serde::Serialize for RegItem<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.deref().serialize(serializer)
    }
}

impl<'de, T: Registry> serde::Deserialize<'de> for RegItem<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = T::deserialize(deserializer)?;
        Ok(Self { inner: Arc::new(inner) })
    }
}
