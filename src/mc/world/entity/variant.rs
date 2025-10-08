use crate::mc::core::{BlockPos, HolderSet};
use crate::mc::registries::{Biome, RegItem, Structure};

pub struct SpawnContext {
    pub pos: BlockPos,
    // TODO: Implement
    // pub level: ServerLevelAccessor,
    pub biome: RegItem<Biome>,
}

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

// TODO: Implement
// impl Predicate<SpawnContext> for StructureCheck {
//     fn test(&self, context: &SpawnContext) -> bool {
//         context
//             .level
//             .getLevel()
//             .structureManager()
//             .getStructureWithPieceAt(context.pos, self.structures)
//             .isValid()
//     }
// }

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BiomeCheck {
    pub biomes: HolderSet<Biome>,
}

// TODO: Implement
// impl Predicate<SpawnContext> for BiomeCheck {
//     fn test(&self, context: &SpawnContext) -> bool {
//         self.biomes.contains(&context.biome)
//     }
// }

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

// TODO: Implement
// impl Predicate<SpawnContext> for MoonBrightnessCheck {
//     fn test(&self, context: &SpawnContext) -> bool {
//         self.range.contains(context.level.getLevel().getMoonBrightness())
//     }
// }

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
