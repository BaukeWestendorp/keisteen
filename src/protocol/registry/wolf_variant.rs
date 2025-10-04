use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariant {
    assets: WolfVariantAssets,
    // TODO: spawn_conditions
}

impl Registry for WolfVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "wolf_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariantAssets {
    wild: Identifier,
    tame: Identifier,
    angry: Identifier,
}
