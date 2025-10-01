use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::protocol::registry::banner_pattern::BannerPattern;
use crate::protocol::registry::chat_type::ChatType;
use crate::protocol::registry::damage_type::DamageType;
use crate::protocol::registry::dimension_type::DimensionType;
use crate::types::Identifier;

pub mod banner_pattern;
pub mod chat_type;
pub mod damage_type;
pub mod dimension_type;

#[derive(Debug)]
pub struct Registries {
    banner_pattern: BTreeMap<Identifier, BannerPattern>,
    chat_type: BTreeMap<Identifier, ChatType>,
    damage_type: BTreeMap<Identifier, DamageType>,
    // TODO: dialog: BTreeMap<Identifier, Dialog>,
    dimension_type: BTreeMap<Identifier, DimensionType>,
    // TODO: painting_variant: BTreeMap<Identifier, PaintingVariant>,
    // TODO: trim_material: BTreeMap<Identifier, TrimMaterial>,
    // TODO: trim_pattern: BTreeMap<Identifier, TrimPattern>,
    // TODO: wolf_variant: BTreeMap<Identifier, WolfVariant>,
    // TODO: biome: BTreeMap<Identifier, Biome>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let assets_path = PathBuf::from("assets/registries");
        let registries = Self {
            banner_pattern: load_entries(&assets_path),
            chat_type: load_entries(&assets_path),
            damage_type: load_entries(&assets_path),
            dimension_type: load_entries(&assets_path),
        };
        tracing::debug!("loaded registries from assets");
        registries
    }

    pub fn banner_pattern(&self) -> &BTreeMap<Identifier, BannerPattern> {
        &self.banner_pattern
    }

    pub fn chat_type(&self) -> &BTreeMap<Identifier, ChatType> {
        &self.chat_type
    }

    pub fn damage_type(&self) -> &BTreeMap<Identifier, DamageType> {
        &self.damage_type
    }

    pub fn dimension_type(&self) -> &BTreeMap<Identifier, DimensionType> {
        &self.dimension_type
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
        let entry_name = file_name.to_str().unwrap().strip_suffix(".json").unwrap().to_string();

        map.insert(
            Identifier::new(registry_namespace.to_string(), entry_name).unwrap(),
            T::load_from_asset(&entry_path),
        );
    }
    map
}

trait Registry {
    fn identifier() -> Identifier;

    fn load_from_asset(path: &PathBuf) -> Self
    where
        Self: Sized + for<'de> serde::Deserialize<'de>,
    {
        let reader = fs::File::open(path).unwrap();
        serde_json::from_reader(reader).unwrap()
    }
}
