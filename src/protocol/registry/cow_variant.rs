use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CowVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    // TODO: To make this more like the vanilla implementation in java, this should be an enum with more possible model loaders.
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for CowVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cow_variant").unwrap()
    }
}
