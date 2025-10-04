use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfSoundVariant {
    ambient_sound: Identifier,
    death_sound: Identifier,
    growl_sound: Identifier,
    hurt_sound: Identifier,
    pant_sound: Identifier,
    whine_sound: Identifier,
}

impl Registry for WolfSoundVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "wolf_sound_variant").unwrap()
    }
}
