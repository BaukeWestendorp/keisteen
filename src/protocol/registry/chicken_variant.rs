use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChickenVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for ChickenVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chicken_variant").unwrap()
    }
}
