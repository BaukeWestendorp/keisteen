use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::mc::text::style::Style;
use crate::mc::text::text_component::TextComponent;
use crate::mc::types::Identifier;
use crate::mc::util::IntProvider;
use crate::mc::world::biome::Biome;

#[derive(Debug)]
pub struct Registries {
    banner_pattern: BTreeMap<Identifier, BannerPattern>,
    cat_variant: BTreeMap<Identifier, CatVariant>,
    chat_type: BTreeMap<Identifier, ChatType>,
    chicken_variant: BTreeMap<Identifier, ChickenVariant>,
    cow_variant: BTreeMap<Identifier, CowVariant>,
    damage_type: BTreeMap<Identifier, DamageType>,
    dialog: BTreeMap<Identifier, Dialog>,
    dimension_type: BTreeMap<Identifier, DimensionType>,
    frog_variant: BTreeMap<Identifier, FrogVariant>,
    painting_variant: BTreeMap<Identifier, PaintingVariant>,
    pig_variant: BTreeMap<Identifier, PigVariant>,
    trim_material: BTreeMap<Identifier, TrimMaterial>,
    trim_pattern: BTreeMap<Identifier, TrimPattern>,
    wolf_sound_variant: BTreeMap<Identifier, WolfSoundVariant>,
    wolf_variant: BTreeMap<Identifier, WolfVariant>,
    worldgen_biome: BTreeMap<Identifier, Biome>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let assets_path = PathBuf::from("assets/registries");
        let registries = Self {
            banner_pattern: load_entries(&assets_path),
            cat_variant: load_entries(&assets_path),
            chat_type: load_entries(&assets_path),
            chicken_variant: load_entries(&assets_path),
            cow_variant: load_entries(&assets_path),
            damage_type: load_entries(&assets_path),
            dialog: load_entries(&assets_path),
            dimension_type: load_entries(&assets_path),
            frog_variant: load_entries(&assets_path),
            painting_variant: load_entries(&assets_path),
            pig_variant: load_entries(&assets_path),
            trim_material: load_entries(&assets_path),
            trim_pattern: load_entries(&assets_path),
            wolf_sound_variant: load_entries(&assets_path),
            wolf_variant: load_entries(&assets_path),
            worldgen_biome: load_entries(&assets_path),
        };
        log::debug!("loaded registries from assets");
        registries
    }

    pub fn banner_pattern(&self) -> &BTreeMap<Identifier, BannerPattern> {
        &self.banner_pattern
    }

    pub fn cat_variant(&self) -> &BTreeMap<Identifier, CatVariant> {
        &self.cat_variant
    }

    pub fn chat_type(&self) -> &BTreeMap<Identifier, ChatType> {
        &self.chat_type
    }

    pub fn chicken_variant(&self) -> &BTreeMap<Identifier, ChickenVariant> {
        &self.chicken_variant
    }

    pub fn cow_variant(&self) -> &BTreeMap<Identifier, CowVariant> {
        &self.cow_variant
    }

    pub fn damage_type(&self) -> &BTreeMap<Identifier, DamageType> {
        &self.damage_type
    }

    pub fn dialog(&self) -> &BTreeMap<Identifier, Dialog> {
        &self.dialog
    }

    pub fn dimension_type(&self) -> &BTreeMap<Identifier, DimensionType> {
        &self.dimension_type
    }

    pub fn frog_variant(&self) -> &BTreeMap<Identifier, FrogVariant> {
        &self.frog_variant
    }

    pub fn painting_variant(&self) -> &BTreeMap<Identifier, PaintingVariant> {
        &self.painting_variant
    }

    pub fn pig_variant(&self) -> &BTreeMap<Identifier, PigVariant> {
        &self.pig_variant
    }

    pub fn trim_material(&self) -> &BTreeMap<Identifier, TrimMaterial> {
        &self.trim_material
    }

    pub fn trim_pattern(&self) -> &BTreeMap<Identifier, TrimPattern> {
        &self.trim_pattern
    }

    pub fn wolf_sound_variant(&self) -> &BTreeMap<Identifier, WolfSoundVariant> {
        &self.wolf_sound_variant
    }

    pub fn wolf_variant(&self) -> &BTreeMap<Identifier, WolfVariant> {
        &self.wolf_variant
    }

    pub fn worldgen_biome(&self) -> &BTreeMap<Identifier, Biome> {
        &self.worldgen_biome
    }
}

fn load_entries<T>(assets_path: &PathBuf) -> BTreeMap<Identifier, T>
where
    T: Registry + for<'de> serde::Deserialize<'de>,
{
    let registry_namespace = T::identifier().namespace().to_string();
    let registry_name = T::identifier().value().to_string();
    let entries_path = assets_path.join(registry_name);

    let mut map = BTreeMap::new();
    for entry_file in entries_path.read_dir().unwrap() {
        let entry_file = entry_file.unwrap();
        let file_name = entry_file.file_name();
        let entry_path = entries_path.join(entry_file.file_name());
        if let Some(entry_name) = file_name.to_str().unwrap().strip_suffix(".json") {
            map.insert(
                Identifier::new(registry_namespace.clone(), entry_name).unwrap(),
                T::load_from_asset(&entry_path),
            );
        }
    }
    map
}

pub trait Registry: serde::Serialize {
    fn identifier() -> Identifier;

    fn load_from_asset(path: &PathBuf) -> Self
    where
        Self: Sized + for<'de> serde::Deserialize<'de>,
    {
        let reader = fs::File::open(path).unwrap();
        serde_json::from_reader(reader)
            .expect(format!("Failed to load asset from {}", path.display()).as_str())
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BannerPattern {
    // TODO: To make this more like the vanilla implementation in java, this should be an enum with more possible asset loaders.
    // TODO: This should really be a `ResourceLocation`.
    asset_id: Identifier,
    translation_key: String,
}

impl Registry for BannerPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "banner_pattern").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatVariant {
    asset_id: Identifier,
    // TODO: spawn_conditions
}

impl Registry for CatVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cat_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatType {
    chat: Decoration,
    narration: Decoration,
}

impl Registry for ChatType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chat_type").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Decoration {
    translation_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    parameters: Vec<String>,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChickenVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for ChickenVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chicken_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CowVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    // TODO: To make this more like the vanilla implementation in java, this should be an enum with more possible model loaders.
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for CowVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cow_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effects: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    death_message_type: Option<String>,
}

impl Registry for DamageType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "damage_type").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dialog {
    // TODO: Implement Dialog Registry
}

impl Registry for Dialog {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "dialog").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DimensionType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FrogVariant {
    asset_id: Identifier,
    // TODO: spawn_conditions
}

impl Registry for FrogVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "frog_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaintingVariant {
    asset_id: Identifier,
    width: i32,
    height: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<TextComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    author: Option<TextComponent>,
}

impl Registry for PaintingVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "painting_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PigVariant {
    asset_id: Identifier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    // TODO: spawn_conditions
}

impl Registry for PigVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "pig_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimMaterial {
    asset_name: String,
    // TODO: override_armor_assets
    // TODO: description
}

impl Registry for TrimMaterial {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_material").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimPattern {
    asset_id: Identifier,
    decal: bool,
    // TODO: description
}

impl Registry for TrimPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_pattern").unwrap()
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariant {
    assets: WolfVariantAssets,
    // TODO: spawn_conditions
}

impl Registry for WolfVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "wolf_variant").unwrap()
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WolfVariantAssets {
    wild: Identifier,
    tame: Identifier,
    angry: Identifier,
}

impl Registry for Biome {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/biome").unwrap()
    }
}
