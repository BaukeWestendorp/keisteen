use crate::protocol::registry::Registry;
use crate::types::Identifier;
use crate::worldgen::IntProvider;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DimensionType {
    fixed_time: Option<i64>,
    has_skylight: bool,
    has_ceiling: bool,
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64,
    bed_works: bool,
    respawn_anchor_works: bool,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: bool,
    has_raids: bool,
    monster_spawn_light_level: MonsterSpawnLightLevel,
    monster_spawn_block_light_limit: i32,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Level(i32),
    IntProvider(IntProvider),
}

impl Registry for DimensionType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "dimension_type").unwrap()
    }
}
