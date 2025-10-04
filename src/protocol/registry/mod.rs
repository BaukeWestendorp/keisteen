use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::protocol::registry::banner_pattern::BannerPattern;
use crate::protocol::registry::cat_variant::CatVariant;
use crate::protocol::registry::chat_type::ChatType;
use crate::protocol::registry::chicken_variant::ChickenVariant;
use crate::protocol::registry::cow_variant::CowVariant;
use crate::protocol::registry::damage_type::DamageType;
use crate::protocol::registry::dialog::Dialog;
use crate::protocol::registry::dimension_type::DimensionType;
use crate::protocol::registry::frog_variant::FrogVariant;
use crate::protocol::registry::painting_variant::PaintingVariant;
use crate::protocol::registry::pig_variant::PigVariant;
use crate::protocol::registry::trim_material::TrimMaterial;
use crate::protocol::registry::trim_pattern::TrimPattern;
use crate::protocol::registry::wolf_sound_variant::WolfSoundVariant;
use crate::protocol::registry::wolf_variant::WolfVariant;
use crate::protocol::registry::worldgen_biome::WorldgenBiome;
use crate::types::Identifier;

pub mod banner_pattern;
pub mod cat_variant;
pub mod chat_type;
pub mod chicken_variant;
pub mod cow_variant;
pub mod damage_type;
pub mod dialog;
pub mod dimension_type;
pub mod frog_variant;
pub mod painting_variant;
pub mod pig_variant;
pub mod trim_material;
pub mod trim_pattern;
pub mod wolf_sound_variant;
pub mod wolf_variant;
pub mod worldgen_biome;

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
    worldgen_biome: BTreeMap<Identifier, WorldgenBiome>,
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

    pub fn worldgen_biome(&self) -> &BTreeMap<Identifier, WorldgenBiome> {
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
        serde_json::from_reader(reader).unwrap()
    }
}
