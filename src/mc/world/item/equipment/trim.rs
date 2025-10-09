use std::collections::HashMap;

use crate::mc::core::ClientAsset;
use crate::mc::registries::Registry;
use crate::mc::resources::ResourceLocation;
use crate::mc::text::TextComponent;
use crate::mc::types::Identifier;

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
