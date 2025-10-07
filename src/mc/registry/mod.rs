use std::collections::BTreeMap;
use std::fs;

pub use res_loc::*;

use crate::mc::types::Identifier;

pub mod res_loc;

const REGISTRIES_PATH: &str = "assets/registries/";

pub struct Registries {
    banner_patterns: BTreeMap<ResourceLocation, BannerPattern>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let banner_patterns = BannerPattern::load_from_file();

        Self { banner_patterns }
    }

    pub fn banner_patterns(&self) -> &BTreeMap<ResourceLocation, BannerPattern> {
        &self.banner_patterns
    }
}

pub trait Registry: Sized + serde::de::DeserializeOwned {
    fn identifier() -> Identifier;

    fn load_from_file() -> BTreeMap<ResourceLocation, Self> {
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
                map.insert(res_loc, value);
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
