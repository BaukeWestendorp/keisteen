use std::collections::HashMap;
use std::fs;

pub use res_loc::*;

use crate::mc::types::Identifier;

pub mod res_loc;

const REGISTRIES_PATH: &str = "assets/registries/";

pub struct Registries {
    banner_patterns: HashMap<ResourceLocation, BannerPattern>,
}

impl Registries {
    pub fn load_from_assets() -> Self {
        let banner_patterns =
            load_registry(Identifier::new("minecraft", "banner_pattern").unwrap());

        Self { banner_patterns }
    }

    pub fn banner_patterns(&self) -> &HashMap<ResourceLocation, BannerPattern> {
        &self.banner_patterns
    }
}

fn load_registry<T: serde::de::DeserializeOwned>(
    identifier: Identifier,
) -> HashMap<ResourceLocation, T> {
    let patterns_dir = std::path::Path::new(REGISTRIES_PATH)
        .join(identifier.namespace())
        .join(identifier.value())
        .to_string_lossy()
        .to_string();

    let entries =
        fs::read_dir(&patterns_dir).unwrap_or_else(|_| panic!("failed to read {}", patterns_dir));

    let mut banner_patterns = HashMap::new();
    for entry in entries {
        let entry = entry.expect("failed to read registry file entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file = fs::File::open(&path).expect("failed to open registry file");
            let pattern: T = serde_json::from_reader(file).expect("failed to parse registry file");
            let file_stem = path.file_stem().and_then(|s| s.to_str()).expect("invalid file name");
            let asset_id = format!("{}:{}", identifier.namespace(), file_stem);
            let res_loc: ResourceLocation = asset_id.parse().expect("invalid resource location");
            banner_patterns.insert(res_loc, pattern);
        }
    }

    banner_patterns
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct BannerPattern {
    pub asset_id: String,
    pub translation_key: String,
}
