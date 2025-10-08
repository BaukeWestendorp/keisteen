#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextComponent {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
}

impl TextComponent {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into(), color: None, italic: None }
    }
}
