use std::collections::BTreeMap;
use std::fs;

use crate::mc::resources::ResourceLocation;
use crate::mc::text::TextComponent;
use crate::mc::types::Identifier;
use crate::mc::world::entity::SpawnPrioritySelectors;

pub use item::*;

mod item;

const REGISTRIES_PATH: &str = "assets/registries/";

pub struct Registries {
    banner_patterns: BTreeMap<ResourceLocation, RegItem<BannerPattern>>,
    cat_variants: BTreeMap<ResourceLocation, RegItem<CatVariant>>,
    chat_types: BTreeMap<ResourceLocation, RegItem<ChatType>>,
    chicken_variants: BTreeMap<ResourceLocation, RegItem<ChickenVariant>>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let banner_patterns = BannerPattern::load_from_file();
        let cat_variants = CatVariant::load_from_file();
        let chat_types = ChatType::load_from_file();
        let chicken_variants = ChickenVariant::load_from_file();

        Self { banner_patterns, cat_variants, chat_types, chicken_variants }
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
    pub asset_id: String,
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
    pub asset_id: String,
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

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ChickenVariant {
    pub asset_id: String,
    pub model: String,
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
