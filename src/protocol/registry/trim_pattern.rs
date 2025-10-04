use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimPattern {
    asset_id: Identifier,
    decal: bool,
    // TODO: description
}

impl Registry for TrimPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_pattern").unwrap()
    }
}
