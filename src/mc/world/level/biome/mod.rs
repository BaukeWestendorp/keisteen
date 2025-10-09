use crate::mc::registries::Registry;
use crate::mc::types::Identifier;

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Biome {
    // TODO: Implement
}

impl Registry for Biome {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/biome").unwrap()
    }
}
