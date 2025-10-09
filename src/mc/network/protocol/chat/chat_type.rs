use crate::mc::registries::Registry;
use crate::mc::text::TextComponent;
use crate::mc::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatType {
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatTypeDecoration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TextComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_key: Option<String>,
}

impl Registry for ChatType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chat_type").unwrap()
    }
}
