use crate::mc::registries::Registry;
use crate::mc::types::Identifier;

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Structure {
    // TODO: Implement
}

impl Registry for Structure {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/structure").unwrap()
    }
}
