use crate::mc::core::{ClientAsset, Holder};
use crate::mc::registries::Registry;
use crate::mc::sounds::SoundEvent;
use crate::mc::types::Identifier;
use crate::mc::world::entity::variant::SpawnPrioritySelectors;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfSoundVariant {
    pub ambient_sound: Holder<SoundEvent>,
    pub death_sound: Holder<SoundEvent>,
    pub growl_sound: Holder<SoundEvent>,
    pub hurt_sound: Holder<SoundEvent>,
    pub pant_sound: Holder<SoundEvent>,
    pub whine_sound: Holder<SoundEvent>,
}

impl Registry for WolfSoundVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "wolf_sound_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariant {
    pub assets: WolfVariantAssetsInfo,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for WolfVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "wolf_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariantAssetsInfo {
    pub wild: ClientAsset,
    pub tame: ClientAsset,
    pub angry: ClientAsset,
}
