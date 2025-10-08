use crate::mc::core::HolderSet;
use crate::mc::registries::{Biome, Structure};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SpawnCondition {
    #[serde(rename = "minecraft:structure")]
    Structure(StructureCheck),
    #[serde(rename = "minecraft:biome")]
    Biome(BiomeCheck),
    #[serde(rename = "minecraft:moon_brightness")]
    MoonBrightness(MoonBrightnessCheck),
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StructureCheck {
    pub structures: HolderSet<Structure>,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BiomeCheck {
    pub biomes: HolderSet<Biome>,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MoonBrightnessCheck {
    pub min: f64,
    pub max: f64,
}

impl Default for MoonBrightnessCheck {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Selector {
    pub condition: Option<SpawnCondition>,
    pub priority: i32,
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SpawnPrioritySelectors {
    pub selectors: Vec<Selector>,
}
