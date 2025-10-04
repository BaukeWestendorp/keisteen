use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FrogVariant {
    asset_id: Identifier,
    // TODO: spawn_conditions
}

impl Registry for FrogVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "frog_variant").unwrap()
    }
}
