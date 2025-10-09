use crate::mc::registries::Registry;
use crate::mc::resources::ResourceLocation;
use crate::mc::types::Identifier;
use crate::mc::util::IntProvider;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DimensionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: ResourceLocation,
    pub ambient_light: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_height: Option<i32>,

    // monster_settings
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: IntProvider,
    pub monster_spawn_block_light_limit: i32,
}

impl Registry for DimensionType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "dimension_type").unwrap()
    }
}
