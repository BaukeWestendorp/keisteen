use std::collections::{BTreeMap, HashMap};
use std::fs;

use crate::mc::core::{ClientAsset, Holder};
use crate::mc::resources::ResourceLocation;
use crate::mc::sounds::SoundEvent;
use crate::mc::text::TextComponent;
use crate::mc::types::Identifier;
use crate::mc::util::IntProvider;
use crate::mc::world::entity::SpawnPrioritySelectors;

pub use item::*;

mod item;

const REGISTRIES_PATH: &str = "assets/registries/";

#[derive(Debug)]
pub struct Registries {
    banner_patterns: BTreeMap<ResourceLocation, RegItem<BannerPattern>>,
    cat_variants: BTreeMap<ResourceLocation, RegItem<CatVariant>>,
    chat_types: BTreeMap<ResourceLocation, RegItem<ChatType>>,
    chicken_variants: BTreeMap<ResourceLocation, RegItem<ChickenVariant>>,
    cow_variants: BTreeMap<ResourceLocation, RegItem<CowVariant>>,
    damage_type: BTreeMap<ResourceLocation, RegItem<DamageType>>,
    // TODO: dialogs: BTreeMap<ResourceLocation, RegItem<Dialog>>,
    dimension_types: BTreeMap<ResourceLocation, RegItem<DimensionType>>,
    frog_variants: BTreeMap<ResourceLocation, RegItem<FrogVariant>>,
    painting_variants: BTreeMap<ResourceLocation, RegItem<PaintingVariant>>,
    pig_variants: BTreeMap<ResourceLocation, RegItem<PigVariant>>,
    trim_materials: BTreeMap<ResourceLocation, RegItem<TrimMaterial>>,
    trim_paterns: BTreeMap<ResourceLocation, RegItem<TrimPatern>>,
    wolf_sound_variant: BTreeMap<ResourceLocation, RegItem<WolfSoundVariant>>,
    wolf_variant: BTreeMap<ResourceLocation, RegItem<WolfVariant>>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let banner_patterns = BannerPattern::load_from_file();
        let cat_variants = CatVariant::load_from_file();
        let chat_types = ChatType::load_from_file();
        let chicken_variants = ChickenVariant::load_from_file();
        let cow_variants = CowVariant::load_from_file();
        let damage_type = DamageType::load_from_file();
        // TODO: let dialogs = Dialog::load_from_file();
        let dimension_types = DimensionType::load_from_file();
        let frog_variants = FrogVariant::load_from_file();
        let painting_variants = PaintingVariant::load_from_file();
        let pig_variants = PigVariant::load_from_file();
        let trim_materials = TrimMaterial::load_from_file();
        let trim_paterns = TrimPatern::load_from_file();
        let wolf_sound_variant = WolfSoundVariant::load_from_file();
        let wolf_variant = WolfVariant::load_from_file();

        Self {
            banner_patterns,
            cat_variants,
            chat_types,
            chicken_variants,
            cow_variants,
            damage_type,
            // TODO: dialogs,
            dimension_types,
            frog_variants,
            painting_variants,
            pig_variants,
            trim_materials,
            trim_paterns,
            wolf_sound_variant,
            wolf_variant,
        }
    }

    pub fn banner_patterns(&self) -> &BTreeMap<ResourceLocation, RegItem<BannerPattern>> {
        &self.banner_patterns
    }

    pub fn cat_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<CatVariant>> {
        &self.cat_variants
    }

    pub fn chat_types(&self) -> &BTreeMap<ResourceLocation, RegItem<ChatType>> {
        &self.chat_types
    }

    pub fn chicken_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<ChickenVariant>> {
        &self.chicken_variants
    }

    pub fn cow_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<CowVariant>> {
        &self.cow_variants
    }

    pub fn damage_types(&self) -> &BTreeMap<ResourceLocation, RegItem<DamageType>> {
        &self.damage_type
    }

    // TODO:
    // pub fn dialogs(&self) -> &BTreeMap<ResourceLocation, RegItem<Dialog>> {
    //     &self.dialogs
    // }

    pub fn dimension_types(&self) -> &BTreeMap<ResourceLocation, RegItem<DimensionType>> {
        &self.dimension_types
    }

    pub fn frog_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<FrogVariant>> {
        &self.frog_variants
    }

    pub fn painting_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<PaintingVariant>> {
        &self.painting_variants
    }

    pub fn pig_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<PigVariant>> {
        &self.pig_variants
    }

    pub fn trim_materials(&self) -> &BTreeMap<ResourceLocation, RegItem<TrimMaterial>> {
        &self.trim_materials
    }

    pub fn trim_paterns(&self) -> &BTreeMap<ResourceLocation, RegItem<TrimPatern>> {
        &self.trim_paterns
    }

    pub fn wolf_sound_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<WolfSoundVariant>> {
        &self.wolf_sound_variant
    }

    pub fn wolf_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<WolfVariant>> {
        &self.wolf_variant
    }
}

pub trait Registry: Sized + serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn identifier() -> Identifier;

    fn load_from_file() -> BTreeMap<ResourceLocation, RegItem<Self>> {
        log::trace!("loading registry {}", Self::identifier());

        let registry_dir = std::path::Path::new(REGISTRIES_PATH)
            .join(Self::identifier().namespace())
            .join(Self::identifier().value())
            .to_string_lossy()
            .to_string();

        let entries = fs::read_dir(&registry_dir)
            .unwrap_or_else(|_| panic!("failed to read {}", registry_dir));

        let mut map = BTreeMap::new();
        for entry in entries {
            let entry = entry.expect("failed to read registry file entry");
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file = fs::File::open(&path).expect("failed to open registry file");
                let value: Self =
                    serde_json::from_reader(file).expect("failed to parse registry file");
                let file_stem =
                    path.file_stem().and_then(|s| s.to_str()).expect("invalid file name");
                let asset_id = format!("{}:{}", Self::identifier().namespace(), file_stem);
                let res_loc: ResourceLocation =
                    asset_id.parse().expect("invalid resource location");
                map.insert(res_loc, RegItem::new(value));
            }
        }

        map
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BannerPattern {
    pub asset_id: ClientAsset,
    pub translation_key: String,
}

impl Registry for BannerPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "banner_pattern").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatVariant {
    pub asset_id: ClientAsset,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for CatVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cat_variant").unwrap()
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChatType {
    pub parameters: Vec<String>,
    pub style: Option<TextComponent>,
    pub translation_key: Option<String>,
}

impl Registry for ChatType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chat_type").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChickenVariant {
    pub asset_id: ClientAsset,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for ChickenVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chicken_variant").unwrap()
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChickenModel {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CowVariant {
    pub asset_id: ClientAsset,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for CowVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cow_variant").unwrap()
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CowModel {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "warm")]
    Warm,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamageType {
    pub message_id: String,
    pub scaling: DamageScaling,
    pub exhaustion: f32,
    pub effects: Option<DamageEffects>,
    pub death_message_type: Option<DeathMessageType>,
}

impl Registry for DamageType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "damage_type").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DamageScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenCausedByLivingNonPlayer,
    #[serde(rename = "always")]
    Always,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DeathMessageType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DamageEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing,
}

// TODO: Dialog

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
    #[serde(flatten)]
    pub monster_settings: Option<MonsterSettings>,
}

impl Registry for DimensionType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "dimension_type").unwrap()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MonsterSettings {
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: IntProvider,
    pub monster_spawn_block_light_limit: i32,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FrogVariant {
    pub asset_id: ClientAsset,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for FrogVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "frog_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaintingVariant {
    pub width: u32,
    pub height: u32,
    pub asset_id: ResourceLocation,
    pub title: Option<TextComponent>,
    pub author: Option<TextComponent>,
}

impl Registry for PaintingVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "painting_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PigVariant {
    pub model: Option<PigVariantModelType>,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for PigVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "pig_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PigVariantModelType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimMaterial {
    pub asset_name: MaterialAssetGroup,
    pub description: TextComponent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_armor_assets: Option<HashMap<ResourceLocation, String>>,
}

impl Registry for TrimMaterial {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_material").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MaterialAssetGroup {
    pub base: String,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrimPatern {
    pub asset_id: ClientAsset,
    pub description: TextComponent,
    pub decal: bool,
}

impl Registry for TrimPatern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "trim_pattern").unwrap()
    }
}

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

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Biome {
    // TODO: Implement
}

impl Registry for Biome {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/biome").unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Structure {
    // TODO: Implement
}

impl Registry for Structure {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "worldgen/structure").unwrap()
    }
}
