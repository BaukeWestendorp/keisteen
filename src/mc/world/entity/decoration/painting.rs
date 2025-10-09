use crate::mc::registries::Registry;
use crate::mc::resources::ResourceLocation;
use crate::mc::text::TextComponent;
use crate::mc::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaintingVariant {
    pub width: i32,
    pub height: i32,
    pub asset_id: ResourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<TextComponent>,
}

impl Registry for PaintingVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "painting_variant").unwrap()
    }
}
