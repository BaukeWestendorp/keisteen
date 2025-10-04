use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PigVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for PigVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "pig_variant").unwrap()
    }
}
