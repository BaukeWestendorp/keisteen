use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaintingVariant {
    asset_id: Identifier,
    width: i32,
    height: i32,
    // TODO: We need to use text components instead of Title and Author, but we do not support TextComponents yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    author: Option<Author>,
}

impl Registry for PaintingVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "painting_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Title {
    color: String,
    translate: String,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Author {
    color: String,
    translate: String,
}
