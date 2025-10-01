use crate::protocol::registry::Registry;
use crate::text::style::Style;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatType {
    chat: Decoration,
    narration: Decoration,
}

impl Registry for ChatType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chat_type").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Decoration {
    translation_key: String,
    style: Option<Style>,
    parameters: Vec<String>,
}
