use crate::mc::core::ClientAsset;
use crate::mc::registries::Registry;
use crate::mc::types::Identifier;
use crate::mc::world::entity::variant::SpawnPrioritySelectors;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FrogVariant {
    pub asset_id: ClientAsset,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for FrogVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "frog_variant").unwrap()
    }
}
