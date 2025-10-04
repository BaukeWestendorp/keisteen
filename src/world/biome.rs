use std::collections::BTreeMap;

use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Biome {
    // Climate settings
    has_precipitation: bool,
    temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    temperature_modifier: Option<String>,
    downfall: f32,

    // Biome generation settings
    carvers: Vec<Identifier>,
    features: Vec<Vec<Identifier>>,

    // Special effects
    effects: BiomeSpecialEffects,

    #[serde(default = "default_creature_spawn_probability")]
    creature_spawn_probability: f32,
    spawners: BTreeMap<String, ()>,
    spawn_costs: BTreeMap<Identifier, ()>,
}

fn default_creature_spawn_probability() -> f32 {
    0.1
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BiomeSpecialEffects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // foliage_color: Option<i32>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // dry_foliage_color: Option<i32>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // grass_color: Option<i32>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // grass_color_modifier: Option<GrassColorModifier>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // particle: Option<AmbientParticleSettings>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // ambient_sound: Option<SoundEvent>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // mood_sound: Option<AmbientMoodSettings>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // additions_sound: Option<AmbientAdditionsSettings>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // music: Option<WeightedList<Music>>,
    #[serde(default = "default_music_volume")]
    music_volume: f32,
}

fn default_music_volume() -> f32 {
    1.0
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum GrassColorModifier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dark_forest")]
    DarkForest,
    #[serde(rename = "swamp")]
    Swamp,
}
