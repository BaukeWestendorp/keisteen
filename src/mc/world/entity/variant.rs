use crate::mc::core::HolderSet;
use crate::mc::registries::{Biome, Structure};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum SpawnCondition {
    #[serde(rename = "minecraft:structure")]
    Structure { structures: HolderSet<Structure> },
    #[serde(rename = "minecraft:biome")]
    Biome { biomes: HolderSet<Biome> },
    #[serde(rename = "minecraft:moon_brightness")]
    MoonBrightness { range: MoonBrightnessCheckRange },
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MoonBrightnessCheckRange {
    pub min: f32,
    pub max: f32,
}

impl Default for MoonBrightnessCheckRange {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Selector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<SpawnCondition>,
    pub priority: i32,
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SpawnPrioritySelectors {
    pub selectors: Vec<Selector>,
}
