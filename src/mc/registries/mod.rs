use std::collections::BTreeMap;
use std::fs;

use crate::mc::network::protocol::chat::ChatType;
use crate::mc::resources::ResourceLocation;
use crate::mc::types::Identifier;
use crate::mc::world::damagesource::DamageType;
use crate::mc::world::entity::animal::{
    CatVariant, ChickenVariant, CowVariant, FrogVariant, PigVariant, WolfSoundVariant, WolfVariant,
};
use crate::mc::world::entity::decoration::PaintingVariant;
use crate::mc::world::item::equipment::trim::{TrimMaterial, TrimPatern};
use crate::mc::world::level::biome::Biome;
use crate::mc::world::level::block::BannerPattern;
use crate::mc::world::level::dimension::DimensionType;

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
    trim_patterns: BTreeMap<ResourceLocation, RegItem<TrimPatern>>,
    wolf_sound_variants: BTreeMap<ResourceLocation, RegItem<WolfSoundVariant>>,
    wolf_variants: BTreeMap<ResourceLocation, RegItem<WolfVariant>>,
    biomes: BTreeMap<ResourceLocation, RegItem<Biome>>,
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
        let trim_patterns = TrimPatern::load_from_file();
        let wolf_sound_variants = WolfSoundVariant::load_from_file();
        let wolf_variants = WolfVariant::load_from_file();
        let biomes = Biome::load_from_file();

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
            trim_patterns,
            wolf_sound_variants,
            wolf_variants,
            biomes,
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

    pub fn trim_patterns(&self) -> &BTreeMap<ResourceLocation, RegItem<TrimPatern>> {
        &self.trim_patterns
    }

    pub fn wolf_sound_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<WolfSoundVariant>> {
        &self.wolf_sound_variants
    }

    pub fn wolf_variants(&self) -> &BTreeMap<ResourceLocation, RegItem<WolfVariant>> {
        &self.wolf_variants
    }

    pub fn biomes(&self) -> &BTreeMap<ResourceLocation, RegItem<Biome>> {
        &self.biomes
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
