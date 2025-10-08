use crate::mc::registries::{RegItem, Registry};
use std::fmt;

#[derive(Debug)]
pub enum HolderSet<T: Registry> {
    Direct(Vec<RegItem<T>>),
    String(String),
}

impl<T: Registry> serde::Serialize for HolderSet<T>
where
    RegItem<T>: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HolderSet::String(s) => serializer.serialize_str(s),
            HolderSet::Direct(items) => items.serialize(serializer),
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for HolderSet<T>
where
    T: Registry,
    RegItem<T>: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HolderSetVisitor<T: Registry> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for HolderSetVisitor<T>
        where
            T: Registry,
            RegItem<T>: serde::Deserialize<'de>,
        {
            type Value = HolderSet<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or a sequence")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(HolderSet::String(value.to_owned()))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let items = serde::Deserialize::deserialize(
                    serde::de::value::SeqAccessDeserializer::new(seq),
                )?;
                Ok(HolderSet::Direct(items))
            }
        }

        deserializer.deserialize_any(HolderSetVisitor { marker: std::marker::PhantomData })
    }
}
