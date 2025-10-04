use crate::protocol::registry::Registry;
use crate::text::text_component::TextComponent;
use crate::types::Identifier;

#[derive(Debug)]
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
