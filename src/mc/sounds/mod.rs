use crate::mc::registries::Registry;
use crate::mc::resources::ResourceLocation;

use super::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SoundEvent {
    pub sound_id: ResourceLocation,
    pub range: Option<f32>,
}

impl Registry for SoundEvent {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "sound_event").unwrap()
    }
}
