use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatVariant {
    asset_id: Identifier,
    // TODO: spawn_conditions
}

impl Registry for CatVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cat_variant").unwrap()
    }
}
