use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimMaterial {
    asset_name: String,
    // TODO: override_armor_assets
    // TODO: description
}

impl Registry for TrimMaterial {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_material").unwrap()
    }
}
