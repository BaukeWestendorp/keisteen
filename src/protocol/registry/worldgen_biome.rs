use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WorldgenBiome {
    // TODO: Implement WorldgenBiome Registry
}

impl Registry for WorldgenBiome {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/biome").unwrap()
    }
}
